use crate::ast::expr::LitExprKind;

use super::CommonPatData;

/// A literal expression inside a pattern.
///
/// ```
/// # let string = "marker remix";
/// match string {
///     "example" => true,
/// //  ^^^^^^^^^ A string literal used as a pattern
///     _ => false,
/// };
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct LitPat<'ast> {
    data: CommonPatData<'ast>,
    lit: LitExprKind<'ast>,
}

impl<'ast> LitPat<'ast> {
    /// The literal expression used as a pattern.
    pub fn lit_expr(&self) -> LitExprKind<'ast> {
        self.lit
    }
}

super::impl_pat_data!(LitPat<'ast>, Lit);

#[cfg(feature = "driver-api")]
impl<'ast> LitPat<'ast> {
    pub fn new(data: CommonPatData<'ast>, lit: LitExprKind<'ast>) -> Self {
        Self { data, lit }
    }
}
