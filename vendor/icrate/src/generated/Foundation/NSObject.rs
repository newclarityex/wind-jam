//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSCoding {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method(encodeWithCoder:)]
        unsafe fn encodeWithCoder(&self, coder: &NSCoder);

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder)
            -> Option<Id<Self>>;
    }

    unsafe impl ProtocolType for dyn NSCoding {}
);

extern_protocol!(
    pub unsafe trait NSSecureCoding: NSCoding {
        #[method(supportsSecureCoding)]
        unsafe fn supportsSecureCoding() -> bool;
    }

    unsafe impl ProtocolType for dyn NSSecureCoding {}
);

extern_protocol!(
    pub unsafe trait NSDiscardableContent {
        #[method(beginContentAccess)]
        unsafe fn beginContentAccess(&self) -> bool;

        #[method(endContentAccess)]
        unsafe fn endContentAccess(&self);

        #[method(discardContentIfPossible)]
        unsafe fn discardContentIfPossible(&self);

        #[method(isContentDiscarded)]
        unsafe fn isContentDiscarded(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn NSDiscardableContent {}
);

extern_fn!(
    pub unsafe fn NSAllocateObject(
        a_class: &AnyClass,
        extra_bytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<AnyObject>;
);

extern_fn!(
    pub unsafe fn NSDeallocateObject(object: &AnyObject);
);

extern_fn!(
    #[deprecated = "Not supported"]
    pub unsafe fn NSCopyObject(
        object: &AnyObject,
        extra_bytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<AnyObject>;
);

extern_fn!(
    pub unsafe fn NSShouldRetainWithZone(
        an_object: &AnyObject,
        requested_zone: *mut NSZone,
    ) -> Bool;
);

extern_fn!(
    pub unsafe fn NSIncrementExtraRefCount(object: &AnyObject);
);

extern_fn!(
    pub unsafe fn NSDecrementExtraRefCountWasZero(object: &AnyObject) -> Bool;
);

extern_fn!(
    pub unsafe fn NSExtraRefCount(object: &AnyObject) -> NSUInteger;
);
