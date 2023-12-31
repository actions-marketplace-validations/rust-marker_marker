use std::marker::PhantomData;

/// The placeholder type, signalling that the semantic type is still unstable
/// and therefor not represented as part of the API.
#[repr(C)]
#[derive(Debug)]
pub struct UnstableTy<'ast> {
    _lt: PhantomData<&'ast ()>,
}

#[cfg(feature = "driver-api")]
impl<'ast> UnstableTy<'ast> {
    pub fn new() -> Self {
        Self { _lt: PhantomData }
    }
}
