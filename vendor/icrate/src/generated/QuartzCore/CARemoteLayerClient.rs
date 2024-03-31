//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
    pub struct CARemoteLayerClient;

    #[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
    unsafe impl ClassType for CARemoteLayerClient {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
unsafe impl NSObjectProtocol for CARemoteLayerClient {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
    unsafe impl CARemoteLayerClient {
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(clientId)]
        pub unsafe fn clientId(&self) -> u32;

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer(&self) -> Option<Id<CALayer>>;

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[method(setLayer:)]
        pub unsafe fn setLayer(&self, layer: Option<&CALayer>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
    unsafe impl CARemoteLayerClient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
