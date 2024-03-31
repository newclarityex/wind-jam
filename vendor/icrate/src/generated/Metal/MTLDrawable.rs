//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

pub type MTLDrawablePresentedHandler = *mut Block<(NonNull<ProtocolObject<dyn MTLDrawable>>,), ()>;

extern_protocol!(
    pub unsafe trait MTLDrawable: NSObjectProtocol {
        #[method(present)]
        fn present(&self);

        #[method(presentAtTime:)]
        unsafe fn presentAtTime(&self, presentation_time: CFTimeInterval);

        #[method(presentAfterMinimumDuration:)]
        unsafe fn presentAfterMinimumDuration(&self, duration: CFTimeInterval);

        #[method(addPresentedHandler:)]
        unsafe fn addPresentedHandler(&self, block: MTLDrawablePresentedHandler);

        #[method(presentedTime)]
        unsafe fn presentedTime(&self) -> CFTimeInterval;

        #[method(drawableID)]
        fn drawableID(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLDrawable {}
);
