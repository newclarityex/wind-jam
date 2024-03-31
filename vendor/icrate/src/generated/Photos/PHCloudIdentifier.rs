//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
    pub struct PHCloudIdentifier;

    #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
    unsafe impl ClassType for PHCloudIdentifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHCloudIdentifier")]
unsafe impl NSCoding for PHCloudIdentifier {}

#[cfg(feature = "PhotoKit_PHCloudIdentifier")]
unsafe impl NSObjectProtocol for PHCloudIdentifier {}

#[cfg(feature = "PhotoKit_PHCloudIdentifier")]
unsafe impl NSSecureCoding for PHCloudIdentifier {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
    unsafe impl PHCloudIdentifier {
        #[deprecated]
        #[method_id(@__retain_semantics Other notFoundIdentifier)]
        pub unsafe fn notFoundIdentifier() -> Id<PHCloudIdentifier>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithStringValue:)]
        pub unsafe fn initWithStringValue(
            this: Option<Allocated<Self>>,
            string_value: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
    unsafe impl PHCloudIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
    pub struct PHCloudIdentifierMapping;

    #[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
    unsafe impl ClassType for PHCloudIdentifierMapping {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
unsafe impl NSObjectProtocol for PHCloudIdentifierMapping {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
    unsafe impl PHCloudIdentifierMapping {
        #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
        #[method_id(@__retain_semantics Other cloudIdentifier)]
        pub unsafe fn cloudIdentifier(&self) -> Option<Id<PHCloudIdentifier>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
    unsafe impl PHCloudIdentifierMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
    pub struct PHLocalIdentifierMapping;

    #[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
    unsafe impl ClassType for PHLocalIdentifierMapping {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
unsafe impl NSObjectProtocol for PHLocalIdentifierMapping {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
    unsafe impl PHLocalIdentifierMapping {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localIdentifier)]
        pub unsafe fn localIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
    unsafe impl PHLocalIdentifierMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "PhotoKit_PHCloudIdentifier")]
    unsafe impl PHCloudIdentifier {}
);

#[cfg(feature = "PhotoKit_PHCloudIdentifier")]
unsafe impl NSCopying for PHCloudIdentifier {}

extern_methods!(
    /// CloudIdentifiers
    #[cfg(feature = "PhotoKit_PHPhotoLibrary")]
    unsafe impl PHPhotoLibrary {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "PhotoKit_PHCloudIdentifier",
            feature = "PhotoKit_PHLocalIdentifierMapping"
        ))]
        #[method_id(@__retain_semantics Other localIdentifierMappingsForCloudIdentifiers:)]
        pub unsafe fn localIdentifierMappingsForCloudIdentifiers(
            &self,
            cloud_identifiers: &NSArray<PHCloudIdentifier>,
        ) -> Id<NSDictionary<PHCloudIdentifier, PHLocalIdentifierMapping>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHCloudIdentifierMapping"
        ))]
        #[method_id(@__retain_semantics Other cloudIdentifierMappingsForLocalIdentifiers:)]
        pub unsafe fn cloudIdentifierMappingsForLocalIdentifiers(
            &self,
            local_identifiers: &NSArray<NSString>,
        ) -> Id<NSDictionary<NSString, PHCloudIdentifierMapping>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHCloudIdentifier"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other localIdentifiersForCloudIdentifiers:)]
        pub unsafe fn localIdentifiersForCloudIdentifiers(
            &self,
            cloud_identifiers: &NSArray<PHCloudIdentifier>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHCloudIdentifier"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other cloudIdentifiersForLocalIdentifiers:)]
        pub unsafe fn cloudIdentifiersForLocalIdentifiers(
            &self,
            local_identifiers: &NSArray<NSString>,
        ) -> Id<NSArray<PHCloudIdentifier>>;
    }
);

extern_static!(PHLocalIdentifierNotFound: &'static NSString);
