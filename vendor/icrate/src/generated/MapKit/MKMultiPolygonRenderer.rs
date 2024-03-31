//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
    pub struct MKMultiPolygonRenderer;

    #[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
    unsafe impl ClassType for MKMultiPolygonRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
unsafe impl NSObjectProtocol for MKMultiPolygonRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
    unsafe impl MKMultiPolygonRenderer {
        #[cfg(feature = "MapKit_MKMultiPolygon")]
        #[method_id(@__retain_semantics Init initWithMultiPolygon:)]
        pub unsafe fn initWithMultiPolygon(
            this: Option<Allocated<Self>>,
            multi_polygon: &MKMultiPolygon,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMultiPolygon")]
        #[method_id(@__retain_semantics Other multiPolygon)]
        pub unsafe fn multiPolygon(&self) -> Id<MKMultiPolygon>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
    unsafe impl MKMultiPolygonRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
    unsafe impl MKMultiPolygonRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
