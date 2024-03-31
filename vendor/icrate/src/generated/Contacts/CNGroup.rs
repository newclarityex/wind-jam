//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNGroup")]
    pub struct CNGroup;

    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl ClassType for CNGroup {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSCoding for CNGroup {}

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSCopying for CNGroup {}

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSMutableCopying for CNGroup {}

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSObjectProtocol for CNGroup {}

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSSecureCoding for CNGroup {}

extern_methods!(
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNGroup {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(CNGroupIdentifierKey: &'static NSString);

extern_static!(CNGroupNameKey: &'static NSString);
