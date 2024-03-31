//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCKeyboardValueChangedHandler = *mut Block<
    (
        NonNull<GCKeyboardInput>,
        NonNull<GCControllerButtonInput>,
        GCKeyCode,
        Bool,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCKeyboardInput")]
    pub struct GCKeyboardInput;

    #[cfg(feature = "GameController_GCKeyboardInput")]
    unsafe impl ClassType for GCKeyboardInput {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCKeyboardInput")]
unsafe impl NSObjectProtocol for GCKeyboardInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCKeyboardInput")]
    unsafe impl GCKeyboardInput {
        #[method(keyChangedHandler)]
        pub unsafe fn keyChangedHandler(&self) -> GCKeyboardValueChangedHandler;

        #[method(setKeyChangedHandler:)]
        pub unsafe fn setKeyChangedHandler(
            &self,
            key_changed_handler: GCKeyboardValueChangedHandler,
        );

        #[method(isAnyKeyPressed)]
        pub unsafe fn isAnyKeyPressed(&self) -> bool;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonForKeyCode:)]
        pub unsafe fn buttonForKeyCode(
            &self,
            code: GCKeyCode,
        ) -> Option<Id<GCControllerButtonInput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCKeyboardInput")]
    unsafe impl GCKeyboardInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
