//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileVersionAddingOptions {
        NSFileVersionAddingByMoving = 1 << 0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileVersionReplacingOptions {
        NSFileVersionReplacingByMoving = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFileVersion")]
    pub struct NSFileVersion;

    #[cfg(feature = "Foundation_NSFileVersion")]
    unsafe impl ClassType for NSFileVersion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSFileVersion")]
unsafe impl NSObjectProtocol for NSFileVersion {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFileVersion")]
    unsafe impl NSFileVersion {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other currentVersionOfItemAtURL:)]
        pub unsafe fn currentVersionOfItemAtURL(url: &NSURL) -> Option<Id<NSFileVersion>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other otherVersionsOfItemAtURL:)]
        pub unsafe fn otherVersionsOfItemAtURL(url: &NSURL) -> Option<Id<NSArray<NSFileVersion>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other unresolvedConflictVersionsOfItemAtURL:)]
        pub unsafe fn unresolvedConflictVersionsOfItemAtURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSFileVersion>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(getNonlocalVersionsOfItemAtURL:completionHandler:)]
        pub unsafe fn getNonlocalVersionsOfItemAtURL_completionHandler(
            url: &NSURL,
            completion_handler: &Block<(*mut NSArray<NSFileVersion>, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other versionOfItemAtURL:forPersistentIdentifier:)]
        pub unsafe fn versionOfItemAtURL_forPersistentIdentifier(
            url: &NSURL,
            persistent_identifier: &AnyObject,
        ) -> Option<Id<NSFileVersion>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other addVersionOfItemAtURL:withContentsOfURL:options:error:_)]
        pub unsafe fn addVersionOfItemAtURL_withContentsOfURL_options_error(
            url: &NSURL,
            contents_url: &NSURL,
            options: NSFileVersionAddingOptions,
        ) -> Result<Id<NSFileVersion>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other temporaryDirectoryURLForNewVersionOfItemAtURL:)]
        pub unsafe fn temporaryDirectoryURLForNewVersionOfItemAtURL(url: &NSURL) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameOfSavingComputer)]
        pub unsafe fn localizedNameOfSavingComputer(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other originatorNameComponents)]
        pub unsafe fn originatorNameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

        #[method_id(@__retain_semantics Other persistentIdentifier)]
        pub unsafe fn persistentIdentifier(&self) -> Id<ProtocolObject<dyn NSCoding>>;

        #[method(isConflict)]
        pub unsafe fn isConflict(&self) -> bool;

        #[method(isResolved)]
        pub unsafe fn isResolved(&self) -> bool;

        #[method(setResolved:)]
        pub unsafe fn setResolved(&self, resolved: bool);

        #[method(isDiscardable)]
        pub unsafe fn isDiscardable(&self) -> bool;

        #[method(setDiscardable:)]
        pub unsafe fn setDiscardable(&self, discardable: bool);

        #[method(hasLocalContents)]
        pub unsafe fn hasLocalContents(&self) -> bool;

        #[method(hasThumbnail)]
        pub unsafe fn hasThumbnail(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other replaceItemAtURL:options:error:_)]
        pub unsafe fn replaceItemAtURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileVersionReplacingOptions,
        ) -> Result<Id<NSURL>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAndReturnError:_)]
        pub unsafe fn removeAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(removeOtherVersionsOfItemAtURL:error:_)]
        pub unsafe fn removeOtherVersionsOfItemAtURL_error(url: &NSURL) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSFileVersion")]
    unsafe impl NSFileVersion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
