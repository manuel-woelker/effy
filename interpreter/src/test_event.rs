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
        // TODO: error diagnostics
    },
}
