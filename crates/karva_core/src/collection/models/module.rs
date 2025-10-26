use pyo3::prelude::*;

use crate::{
    collection::TestCase,
    diagnostic::{Diagnostic, reporter::Reporter},
    extensions::fixtures::Finalizers,
    runner::TestRunResult,
};

/// A collected module represents a single Python module with its test cases and finalizers.
#[derive(Default, Debug)]
pub(crate) struct CollectedModule<'proj> {
    /// The test cases in the module.
    test_cases: Vec<(TestCase<'proj>, Option<Diagnostic>)>,

    /// Finalizers to run after the module is executed.
    finalizers: Finalizers,
}

impl<'proj> CollectedModule<'proj> {
    #[must_use]
    pub(crate) fn total_test_cases(&self) -> usize {
        self.test_cases.len()
    }

    pub(crate) fn add_test_cases(
        &mut self,
        test_cases: Vec<(TestCase<'proj>, Option<Diagnostic>)>,
    ) {
        self.test_cases.extend(test_cases);
    }

    #[must_use]
    pub(crate) const fn finalizers(&self) -> &Finalizers {
        &self.finalizers
    }

    pub(crate) fn add_finalizers(&mut self, finalizers: Finalizers) {
        self.finalizers.update(finalizers);
    }

    pub(crate) fn run_with_reporter(
        &self,
        py: Python<'_>,
        reporter: &dyn Reporter,
    ) -> TestRunResult {
        let mut diagnostics = TestRunResult::default();

        self.test_cases.iter().for_each(|(test_case, diagnostic)| {
            let mut result = test_case.run(py, diagnostic.clone(), reporter);
            result.add_diagnostics(test_case.finalizers().run(py));
            diagnostics.update(&result);
        });

        diagnostics.add_diagnostics(self.finalizers().run(py));

        diagnostics
    }
}
