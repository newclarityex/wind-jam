//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNEntityType {
        CNEntityTypeContacts = 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNAuthorizationStatus {
        CNAuthorizationStatusNotDetermined = 0,
        CNAuthorizationStatusRestricted = 1,
        CNAuthorizationStatusDenied = 2,
        CNAuthorizationStatusAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNContactStore")]
    pub struct CNContactStore;

    #[cfg(feature = "Contacts_CNContactStore")]
    unsafe impl ClassType for CNContactStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNContactStore")]
unsafe impl NSObjectProtocol for CNContactStore {}

extern_methods!(
    #[cfg(feature = "Contacts_CNContactStore")]
    unsafe impl CNContactStore {
        #[method(authorizationStatusForEntityType:)]
        pub unsafe fn authorizationStatusForEntityType(
            entity_type: CNEntityType,
        ) -> CNAuthorizationStatus;

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestAccessForEntityType:completionHandler:)]
        pub unsafe fn requestAccessForEntityType_completionHandler(
            &self,
            entity_type: CNEntityType,
            completion_handler: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other unifiedContactsMatchingPredicate:keysToFetch:error:_)]
        pub unsafe fn unifiedContactsMatchingPredicate_keysToFetch_error(
            &self,
            predicate: &NSPredicate,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Id<NSArray<CNContact>>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other unifiedContactWithIdentifier:keysToFetch:error:_)]
        pub unsafe fn unifiedContactWithIdentifier_keysToFetch_error(
            &self,
            identifier: &NSString,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Id<CNContact>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other unifiedMeContactWithKeysToFetch:error:_)]
        pub unsafe fn unifiedMeContactWithKeysToFetch_error(
            &self,
            keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Result<Id<CNContact>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Contacts_CNContactFetchRequest",
            feature = "Contacts_CNFetchResult",
            feature = "Foundation_NSEnumerator",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForContactFetchRequest:error:_)]
        pub unsafe fn enumeratorForContactFetchRequest_error(
            &self,
            request: &CNContactFetchRequest,
        ) -> Result<Id<CNFetchResult<NSEnumerator<CNContact>>>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNChangeHistoryEvent",
            feature = "Contacts_CNChangeHistoryFetchRequest",
            feature = "Contacts_CNFetchResult",
            feature = "Foundation_NSEnumerator",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForChangeHistoryFetchRequest:error:_)]
        pub unsafe fn enumeratorForChangeHistoryFetchRequest_error(
            &self,
            request: &CNChangeHistoryFetchRequest,
        ) -> Result<Id<CNFetchResult<NSEnumerator<CNChangeHistoryEvent>>>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Contacts_CNContactFetchRequest",
            feature = "Foundation_NSError"
        ))]
        #[method(enumerateContactsWithFetchRequest:error:usingBlock:)]
        pub unsafe fn enumerateContactsWithFetchRequest_error_usingBlock(
            &self,
            fetch_request: &CNContactFetchRequest,
            error: Option<&mut Option<Id<NSError>>>,
            block: &Block<(NonNull<CNContact>, NonNull<Bool>), ()>,
        ) -> bool;

        #[cfg(all(
            feature = "Contacts_CNGroup",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other groupsMatchingPredicate:error:_)]
        pub unsafe fn groupsMatchingPredicate_error(
            &self,
            predicate: Option<&NSPredicate>,
        ) -> Result<Id<NSArray<CNGroup>>, Id<NSError>>;

        #[cfg(all(
            feature = "Contacts_CNContainer",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other containersMatchingPredicate:error:_)]
        pub unsafe fn containersMatchingPredicate_error(
            &self,
            predicate: Option<&NSPredicate>,
        ) -> Result<Id<NSArray<CNContainer>>, Id<NSError>>;

        #[cfg(all(feature = "Contacts_CNSaveRequest", feature = "Foundation_NSError"))]
        #[method(executeSaveRequest:error:_)]
        pub unsafe fn executeSaveRequest_error(
            &self,
            save_request: &CNSaveRequest,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other currentHistoryToken)]
        pub unsafe fn currentHistoryToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultContainerIdentifier)]
        pub unsafe fn defaultContainerIdentifier(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNContactStore")]
    unsafe impl CNContactStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(CNContactStoreDidChangeNotification: &'static NSString);
