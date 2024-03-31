//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::LinkPresentation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LinkPresentation_LPLinkMetadata")]
    pub struct LPLinkMetadata;

    #[cfg(feature = "LinkPresentation_LPLinkMetadata")]
    unsafe impl ClassType for LPLinkMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "LinkPresentation_LPLinkMetadata")]
unsafe impl NSCoding for LPLinkMetadata {}

#[cfg(feature = "LinkPresentation_LPLinkMetadata")]
unsafe impl NSCopying for LPLinkMetadata {}

#[cfg(feature = "LinkPresentation_LPLinkMetadata")]
unsafe impl NSObjectProtocol for LPLinkMetadata {}

#[cfg(feature = "LinkPresentation_LPLinkMetadata")]
unsafe impl NSSecureCoding for LPLinkMetadata {}

extern_methods!(
    #[cfg(feature = "LinkPresentation_LPLinkMetadata")]
    unsafe impl LPLinkMetadata {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other originalURL)]
        pub unsafe fn originalURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setOriginalURL:)]
        pub unsafe fn setOriginalURL(&self, original_url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method_id(@__retain_semantics Other iconProvider)]
        pub unsafe fn iconProvider(&self) -> Option<Id<NSItemProvider>>;

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method(setIconProvider:)]
        pub unsafe fn setIconProvider(&self, icon_provider: Option<&NSItemProvider>);

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method_id(@__retain_semantics Other imageProvider)]
        pub unsafe fn imageProvider(&self) -> Option<Id<NSItemProvider>>;

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method(setImageProvider:)]
        pub unsafe fn setImageProvider(&self, image_provider: Option<&NSItemProvider>);

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method_id(@__retain_semantics Other videoProvider)]
        pub unsafe fn videoProvider(&self) -> Option<Id<NSItemProvider>>;

        #[cfg(feature = "Foundation_NSItemProvider")]
        #[method(setVideoProvider:)]
        pub unsafe fn setVideoProvider(&self, video_provider: Option<&NSItemProvider>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other remoteVideoURL)]
        pub unsafe fn remoteVideoURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setRemoteVideoURL:)]
        pub unsafe fn setRemoteVideoURL(&self, remote_video_url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "LinkPresentation_LPLinkMetadata")]
    unsafe impl LPLinkMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
