use effy_base::error::EffyError;

#[derive(Debug)]
pub enum TestEvent {
    TestStarted {
        test_name: String,
    },

    TestSuccess {
        test_name: String,
    },

    TestFailed {
        test_name: String,
        error: EffyError,
        // TODO: error diagnostics
    },
}
