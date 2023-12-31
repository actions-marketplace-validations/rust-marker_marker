use crate::{
    common::ItemId,
    sem::{ty::TyKind, ConstValue},
};

/// A semantic generic bound in the form `<identifier=type>`. For example,
/// `Item=i32` would be the generic binding here:
///
/// ```ignore
/// let _baz: &dyn Iterator<Item=i32> = todo!();
/// //                      ^^^^^^^^
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct BindingArg<'ast> {
    binding_target: ItemId,
    ty: TyKind<'ast>,
}

impl<'ast> BindingArg<'ast> {
    /// This returns the `ItemId` of the binding target.
    pub fn binding_target(&self) -> ItemId {
        self.binding_target
    }

    /// The type that the binding is set to. For example:
    ///
    /// ```ignore
    /// let _baz: &dyn Iterator<Item=i32> = todo!();
    /// //                           ^^^
    /// ```
    ///
    /// Would return `i32` as the type.
    pub fn ty(&self) -> TyKind<'ast> {
        self.ty
    }
}

#[cfg(feature = "driver-api")]
impl<'ast> BindingArg<'ast> {
    pub fn new(binding_target: ItemId, ty: TyKind<'ast>) -> Self {
        Self { binding_target, ty }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConstArg<'ast> {
    value: ConstValue<'ast>,
}

impl<'ast> ConstArg<'ast> {
    /// The value that is used as an argument.
    pub fn value(&self) -> &ConstValue<'ast> {
        &self.value
    }
}

#[cfg(feature = "driver-api")]
impl<'ast> ConstArg<'ast> {
    pub fn new(value: ConstValue<'ast>) -> Self {
        Self { value }
    }
}
