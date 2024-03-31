//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_methods!(
    /// Predicates
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNGroup {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other predicateForGroupsWithIdentifiers:)]
        pub unsafe fn predicateForGroupsWithIdentifiers(
            identifiers: &NSArray<NSString>,
        ) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateForSubgroupsInGroupWithIdentifier:)]
        pub unsafe fn predicateForSubgroupsInGroupWithIdentifier(
            parent_group_identifier: &NSString,
        ) -> Id<NSPredicate>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateForGroupsInContainerWithIdentifier:)]
        pub unsafe fn predicateForGroupsInContainerWithIdentifier(
            container_identifier: &NSString,
        ) -> Id<NSPredicate>;
    }
);
