//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEComposeSession")]
    pub struct MEComposeSession;

    #[cfg(feature = "MailKit_MEComposeSession")]
    unsafe impl ClassType for MEComposeSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MailKit_MEComposeSession")]
unsafe impl NSCoding for MEComposeSession {}

#[cfg(feature = "MailKit_MEComposeSession")]
unsafe impl NSObjectProtocol for MEComposeSession {}

#[cfg(feature = "MailKit_MEComposeSession")]
unsafe impl NSSecureCoding for MEComposeSession {}

extern_methods!(
    #[cfg(feature = "MailKit_MEComposeSession")]
    unsafe impl MEComposeSession {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other sessionID)]
        pub unsafe fn sessionID(&self) -> Id<NSUUID>;

        #[cfg(feature = "MailKit_MEMessage")]
        #[method_id(@__retain_semantics Other mailMessage)]
        pub unsafe fn mailMessage(&self) -> Id<MEMessage>;

        #[cfg(feature = "MailKit_MEComposeContext")]
        #[method_id(@__retain_semantics Other composeContext)]
        pub unsafe fn composeContext(&self) -> Id<MEComposeContext>;

        #[method(reloadSession)]
        pub unsafe fn reloadSession(&self);
    }
);

extern_static!(MEComposeSessionErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum MEComposeSessionErrorCode {
        MEComposeSessionErrorCodeInvalidRecipients = 0,
        MEComposeSessionErrorCodeInvalidHeaders = 1,
        MEComposeSessionErrorCodeInvalidBody = 2,
    }
);

extern_protocol!(
    pub unsafe trait MEComposeSessionHandler: NSObjectProtocol {
        #[cfg(feature = "MailKit_MEComposeSession")]
        #[method(mailComposeSessionDidBegin:)]
        unsafe fn mailComposeSessionDidBegin(&self, session: &MEComposeSession);

        #[cfg(feature = "MailKit_MEComposeSession")]
        #[method(mailComposeSessionDidEnd:)]
        unsafe fn mailComposeSessionDidEnd(&self, session: &MEComposeSession);

        #[cfg(all(
            feature = "MailKit_MEComposeSession",
            feature = "MailKit_MEExtensionViewController"
        ))]
        #[method_id(@__retain_semantics Other viewControllerForSession:)]
        unsafe fn viewControllerForSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<MEExtensionViewController>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "MailKit_MEAddressAnnotation",
            feature = "MailKit_MEComposeSession",
            feature = "MailKit_MEEmailAddress"
        ))]
        #[optional]
        #[method(session:annotateAddressesWithCompletionHandler:)]
        unsafe fn session_annotateAddressesWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion_handler: &Block<
                (NonNull<NSDictionary<MEEmailAddress, MEAddressAnnotation>>,),
                (),
            >,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "MailKit_MEComposeSession"))]
        #[optional]
        #[method(session:canSendMessageWithCompletionHandler:)]
        unsafe fn session_canSendMessageWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "MailKit_MEComposeSession"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other additionalHeadersForSession:)]
        unsafe fn additionalHeadersForSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<NSDictionary<NSString, NSArray<NSString>>>;
    }

    unsafe impl ProtocolType for dyn MEComposeSessionHandler {}
);
