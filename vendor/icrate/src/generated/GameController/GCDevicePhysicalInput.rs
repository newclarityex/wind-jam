//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCDevicePhysicalInput: GCDevicePhysicalInputState {
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Option<Id<ProtocolObject<dyn GCDevice>>>;

        #[method(elementValueDidChangeHandler)]
        unsafe fn elementValueDidChangeHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<ProtocolObject<dyn GCDevicePhysicalInput>>,
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
            ),
            (),
        >;

        #[method(setElementValueDidChangeHandler:)]
        unsafe fn setElementValueDidChangeHandler(
            &self,
            element_value_did_change_handler: Option<
                &Block<
                    (
                        NonNull<ProtocolObject<dyn GCDevicePhysicalInput>>,
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                    ),
                    (),
                >,
            >,
        );

        #[method_id(@__retain_semantics Other capture)]
        unsafe fn capture(&self) -> Id<ProtocolObject<dyn GCDevicePhysicalInputState>>;

        #[method(inputStateAvailableHandler)]
        unsafe fn inputStateAvailableHandler(
            &self,
        ) -> *mut Block<(NonNull<ProtocolObject<dyn GCDevicePhysicalInput>>,), ()>;

        #[method(setInputStateAvailableHandler:)]
        unsafe fn setInputStateAvailableHandler(
            &self,
            input_state_available_handler: Option<
                &Block<(NonNull<ProtocolObject<dyn GCDevicePhysicalInput>>,), ()>,
            >,
        );

        #[method(inputStateQueueDepth)]
        unsafe fn inputStateQueueDepth(&self) -> NSInteger;

        #[method(setInputStateQueueDepth:)]
        unsafe fn setInputStateQueueDepth(&self, input_state_queue_depth: NSInteger);

        #[method_id(@__retain_semantics Other nextInputState)]
        unsafe fn nextInputState(&self) -> Option<Id<TodoProtocols>>;
    }

    unsafe impl ProtocolType for dyn GCDevicePhysicalInput {}
);
