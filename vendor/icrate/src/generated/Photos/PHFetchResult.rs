//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHFetchResult")]
    pub struct PHFetchResult<ObjectType: Message = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "PhotoKit_PHFetchResult")]
    unsafe impl<ObjectType: Message> ClassType for PHFetchResult<ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "PhotoKit_PHFetchResult")]
unsafe impl<ObjectType: IsIdCloneable> NSCopying for PHFetchResult<ObjectType> {}

#[cfg(feature = "PhotoKit_PHFetchResult")]
unsafe impl<ObjectType: Message> NSFastEnumeration for PHFetchResult<ObjectType> {}

#[cfg(feature = "PhotoKit_PHFetchResult")]
unsafe impl<ObjectType: Message> NSObjectProtocol for PHFetchResult<ObjectType> {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHFetchResult")]
    unsafe impl<ObjectType: Message> PHFetchResult<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, index: NSUInteger) -> Id<ObjectType>;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Id<ObjectType>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, an_object: &ObjectType) -> bool;

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, an_object: &ObjectType) -> NSUInteger;

        #[method(indexOfObject:inRange:)]
        pub unsafe fn indexOfObject_inRange(
            &self,
            an_object: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method_id(@__retain_semantics Other firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Id<ObjectType>>;

        #[method_id(@__retain_semantics Other lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Id<ObjectType>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
        #[method_id(@__retain_semantics Other objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(&self, indexes: &NSIndexSet) -> Id<NSArray<ObjectType>>;

        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(countOfAssetsWithMediaType:)]
        pub unsafe fn countOfAssetsWithMediaType(&self, media_type: PHAssetMediaType)
            -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHFetchResult")]
    unsafe impl<ObjectType: Message> PHFetchResult<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
