//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPContentItem")]
    pub struct MPContentItem;

    #[cfg(feature = "MediaPlayer_MPContentItem")]
    unsafe impl ClassType for MPContentItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPContentItem")]
unsafe impl NSObjectProtocol for MPContentItem {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPContentItem")]
    unsafe impl MPContentItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
        #[method_id(@__retain_semantics Other artwork)]
        pub unsafe fn artwork(&self) -> Option<Id<MPMediaItemArtwork>>;

        #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
        #[method(setArtwork:)]
        pub unsafe fn setArtwork(&self, artwork: Option<&MPMediaItemArtwork>);

        #[method(playbackProgress)]
        pub unsafe fn playbackProgress(&self) -> c_float;

        #[method(setPlaybackProgress:)]
        pub unsafe fn setPlaybackProgress(&self, playback_progress: c_float);

        #[method(isStreamingContent)]
        pub unsafe fn isStreamingContent(&self) -> bool;

        #[method(setStreamingContent:)]
        pub unsafe fn setStreamingContent(&self, streaming_content: bool);

        #[method(isExplicitContent)]
        pub unsafe fn isExplicitContent(&self) -> bool;

        #[method(setExplicitContent:)]
        pub unsafe fn setExplicitContent(&self, explicit_content: bool);

        #[method(isContainer)]
        pub unsafe fn isContainer(&self) -> bool;

        #[method(setContainer:)]
        pub unsafe fn setContainer(&self, container: bool);

        #[method(isPlayable)]
        pub unsafe fn isPlayable(&self) -> bool;

        #[method(setPlayable:)]
        pub unsafe fn setPlayable(&self, playable: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPContentItem")]
    unsafe impl MPContentItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
