//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum UNNotificationActionOptions {
        UNNotificationActionOptionAuthenticationRequired = 1 << 0,
        UNNotificationActionOptionDestructive = 1 << 1,
        UNNotificationActionOptionForeground = 1 << 2,
    }
);

extern_static!(UNNotificationActionOptionNone: UNNotificationActionOptions = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    pub struct UNNotificationAction;

    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    unsafe impl ClassType for UNNotificationAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationAction")]
unsafe impl NSCoding for UNNotificationAction {}

#[cfg(feature = "UserNotifications_UNNotificationAction")]
unsafe impl NSCopying for UNNotificationAction {}

#[cfg(feature = "UserNotifications_UNNotificationAction")]
unsafe impl NSObjectProtocol for UNNotificationAction {}

#[cfg(feature = "UserNotifications_UNNotificationAction")]
unsafe impl NSSecureCoding for UNNotificationAction {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    unsafe impl UNNotificationAction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method(options)]
        pub unsafe fn options(&self) -> UNNotificationActionOptions;

        #[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<UNNotificationActionIcon>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    unsafe impl UNNotificationAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    pub struct UNTextInputNotificationAction;

    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    unsafe impl ClassType for UNTextInputNotificationAction {
        #[inherits(NSObject)]
        type Super = UNNotificationAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
unsafe impl NSCoding for UNTextInputNotificationAction {}

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
unsafe impl NSCopying for UNTextInputNotificationAction {}

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
unsafe impl NSObjectProtocol for UNTextInputNotificationAction {}

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
unsafe impl NSSecureCoding for UNTextInputNotificationAction {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    unsafe impl UNTextInputNotificationAction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:textInputButtonTitle:textInputPlaceholder:)]
        pub unsafe fn actionWithIdentifier_title_options_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:textInputButtonTitle:textInputPlaceholder:)]
        pub unsafe fn actionWithIdentifier_title_options_icon_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textInputButtonTitle)]
        pub unsafe fn textInputButtonTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textInputPlaceholder)]
        pub unsafe fn textInputPlaceholder(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationAction`
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    unsafe impl UNTextInputNotificationAction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    unsafe impl UNTextInputNotificationAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
