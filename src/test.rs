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
