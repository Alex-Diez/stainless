use syntax::ptr::P;
use syntax::ast;

/// A test as a description and associated block.
#[derive(Clone)]
pub struct Test {
    pub description: String,
    pub block: P<ast::Block>,
    pub failing: bool,
    pub ignored: bool
}

pub struct TestConfig {
    pub ignored: bool,
    pub failing: bool
}

impl TestConfig {

    pub fn failing_test() -> TestConfig {
        TestConfig {
            failing: true,
            ignored: false
        }
    }

    pub fn ignored_test() -> TestConfig {
        TestConfig {
            failing: false,
            ignored: true
        }
    }

    pub fn test() -> TestConfig {
        TestConfig {
            failing: false,
            ignored: false
        }
    }
}
