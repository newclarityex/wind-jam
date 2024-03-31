/* automatically generated by rust-bindgen 0.69.4 */

pub const eRENDERDOC_Option_AllowVSync: RENDERDOC_CaptureOption = 0;
pub const eRENDERDOC_Option_AllowFullscreen: RENDERDOC_CaptureOption = 1;
pub const eRENDERDOC_Option_APIValidation: RENDERDOC_CaptureOption = 2;
pub const eRENDERDOC_Option_DebugDeviceMode: RENDERDOC_CaptureOption = 2;
pub const eRENDERDOC_Option_CaptureCallstacks: RENDERDOC_CaptureOption = 3;
pub const eRENDERDOC_Option_CaptureCallstacksOnlyDraws: RENDERDOC_CaptureOption = 4;
pub const eRENDERDOC_Option_CaptureCallstacksOnlyActions: RENDERDOC_CaptureOption = 4;
pub const eRENDERDOC_Option_DelayForDebugger: RENDERDOC_CaptureOption = 5;
pub const eRENDERDOC_Option_VerifyBufferAccess: RENDERDOC_CaptureOption = 6;
pub const eRENDERDOC_Option_VerifyMapWrites: RENDERDOC_CaptureOption = 6;
pub const eRENDERDOC_Option_HookIntoChildren: RENDERDOC_CaptureOption = 7;
pub const eRENDERDOC_Option_RefAllResources: RENDERDOC_CaptureOption = 8;
pub const eRENDERDOC_Option_SaveAllInitials: RENDERDOC_CaptureOption = 9;
pub const eRENDERDOC_Option_CaptureAllCmdLists: RENDERDOC_CaptureOption = 10;
pub const eRENDERDOC_Option_DebugOutputMute: RENDERDOC_CaptureOption = 11;
pub const eRENDERDOC_Option_AllowUnsupportedVendorExtensions: RENDERDOC_CaptureOption = 12;
pub const eRENDERDOC_Option_SoftMemoryLimit: RENDERDOC_CaptureOption = 13;
pub type RENDERDOC_CaptureOption = ::std::os::raw::c_uint;
pub type pRENDERDOC_SetCaptureOptionU32 = ::std::option::Option<
    unsafe extern "C" fn(opt: RENDERDOC_CaptureOption, val: u32) -> ::std::os::raw::c_int,
>;
pub type pRENDERDOC_SetCaptureOptionF32 = ::std::option::Option<
    unsafe extern "C" fn(opt: RENDERDOC_CaptureOption, val: f32) -> ::std::os::raw::c_int,
>;
pub type pRENDERDOC_GetCaptureOptionU32 =
    ::std::option::Option<unsafe extern "C" fn(opt: RENDERDOC_CaptureOption) -> u32>;
pub type pRENDERDOC_GetCaptureOptionF32 =
    ::std::option::Option<unsafe extern "C" fn(opt: RENDERDOC_CaptureOption) -> f32>;
pub const eRENDERDOC_Key_0: RENDERDOC_InputButton = 48;
pub const eRENDERDOC_Key_1: RENDERDOC_InputButton = 49;
pub const eRENDERDOC_Key_2: RENDERDOC_InputButton = 50;
pub const eRENDERDOC_Key_3: RENDERDOC_InputButton = 51;
pub const eRENDERDOC_Key_4: RENDERDOC_InputButton = 52;
pub const eRENDERDOC_Key_5: RENDERDOC_InputButton = 53;
pub const eRENDERDOC_Key_6: RENDERDOC_InputButton = 54;
pub const eRENDERDOC_Key_7: RENDERDOC_InputButton = 55;
pub const eRENDERDOC_Key_8: RENDERDOC_InputButton = 56;
pub const eRENDERDOC_Key_9: RENDERDOC_InputButton = 57;
pub const eRENDERDOC_Key_A: RENDERDOC_InputButton = 65;
pub const eRENDERDOC_Key_B: RENDERDOC_InputButton = 66;
pub const eRENDERDOC_Key_C: RENDERDOC_InputButton = 67;
pub const eRENDERDOC_Key_D: RENDERDOC_InputButton = 68;
pub const eRENDERDOC_Key_E: RENDERDOC_InputButton = 69;
pub const eRENDERDOC_Key_F: RENDERDOC_InputButton = 70;
pub const eRENDERDOC_Key_G: RENDERDOC_InputButton = 71;
pub const eRENDERDOC_Key_H: RENDERDOC_InputButton = 72;
pub const eRENDERDOC_Key_I: RENDERDOC_InputButton = 73;
pub const eRENDERDOC_Key_J: RENDERDOC_InputButton = 74;
pub const eRENDERDOC_Key_K: RENDERDOC_InputButton = 75;
pub const eRENDERDOC_Key_L: RENDERDOC_InputButton = 76;
pub const eRENDERDOC_Key_M: RENDERDOC_InputButton = 77;
pub const eRENDERDOC_Key_N: RENDERDOC_InputButton = 78;
pub const eRENDERDOC_Key_O: RENDERDOC_InputButton = 79;
pub const eRENDERDOC_Key_P: RENDERDOC_InputButton = 80;
pub const eRENDERDOC_Key_Q: RENDERDOC_InputButton = 81;
pub const eRENDERDOC_Key_R: RENDERDOC_InputButton = 82;
pub const eRENDERDOC_Key_S: RENDERDOC_InputButton = 83;
pub const eRENDERDOC_Key_T: RENDERDOC_InputButton = 84;
pub const eRENDERDOC_Key_U: RENDERDOC_InputButton = 85;
pub const eRENDERDOC_Key_V: RENDERDOC_InputButton = 86;
pub const eRENDERDOC_Key_W: RENDERDOC_InputButton = 87;
pub const eRENDERDOC_Key_X: RENDERDOC_InputButton = 88;
pub const eRENDERDOC_Key_Y: RENDERDOC_InputButton = 89;
pub const eRENDERDOC_Key_Z: RENDERDOC_InputButton = 90;
pub const eRENDERDOC_Key_NonPrintable: RENDERDOC_InputButton = 256;
pub const eRENDERDOC_Key_Divide: RENDERDOC_InputButton = 257;
pub const eRENDERDOC_Key_Multiply: RENDERDOC_InputButton = 258;
pub const eRENDERDOC_Key_Subtract: RENDERDOC_InputButton = 259;
pub const eRENDERDOC_Key_Plus: RENDERDOC_InputButton = 260;
pub const eRENDERDOC_Key_F1: RENDERDOC_InputButton = 261;
pub const eRENDERDOC_Key_F2: RENDERDOC_InputButton = 262;
pub const eRENDERDOC_Key_F3: RENDERDOC_InputButton = 263;
pub const eRENDERDOC_Key_F4: RENDERDOC_InputButton = 264;
pub const eRENDERDOC_Key_F5: RENDERDOC_InputButton = 265;
pub const eRENDERDOC_Key_F6: RENDERDOC_InputButton = 266;
pub const eRENDERDOC_Key_F7: RENDERDOC_InputButton = 267;
pub const eRENDERDOC_Key_F8: RENDERDOC_InputButton = 268;
pub const eRENDERDOC_Key_F9: RENDERDOC_InputButton = 269;
pub const eRENDERDOC_Key_F10: RENDERDOC_InputButton = 270;
pub const eRENDERDOC_Key_F11: RENDERDOC_InputButton = 271;
pub const eRENDERDOC_Key_F12: RENDERDOC_InputButton = 272;
pub const eRENDERDOC_Key_Home: RENDERDOC_InputButton = 273;
pub const eRENDERDOC_Key_End: RENDERDOC_InputButton = 274;
pub const eRENDERDOC_Key_Insert: RENDERDOC_InputButton = 275;
pub const eRENDERDOC_Key_Delete: RENDERDOC_InputButton = 276;
pub const eRENDERDOC_Key_PageUp: RENDERDOC_InputButton = 277;
pub const eRENDERDOC_Key_PageDn: RENDERDOC_InputButton = 278;
pub const eRENDERDOC_Key_Backspace: RENDERDOC_InputButton = 279;
pub const eRENDERDOC_Key_Tab: RENDERDOC_InputButton = 280;
pub const eRENDERDOC_Key_PrtScrn: RENDERDOC_InputButton = 281;
pub const eRENDERDOC_Key_Pause: RENDERDOC_InputButton = 282;
pub const eRENDERDOC_Key_Max: RENDERDOC_InputButton = 283;
pub type RENDERDOC_InputButton = ::std::os::raw::c_uint;
pub type pRENDERDOC_SetFocusToggleKeys = ::std::option::Option<
    unsafe extern "C" fn(keys: *mut RENDERDOC_InputButton, num: ::std::os::raw::c_int),
>;
pub type pRENDERDOC_SetCaptureKeys = ::std::option::Option<
    unsafe extern "C" fn(keys: *mut RENDERDOC_InputButton, num: ::std::os::raw::c_int),
>;
pub const eRENDERDOC_Overlay_Enabled: RENDERDOC_OverlayBits = 1;
pub const eRENDERDOC_Overlay_FrameRate: RENDERDOC_OverlayBits = 2;
pub const eRENDERDOC_Overlay_FrameNumber: RENDERDOC_OverlayBits = 4;
pub const eRENDERDOC_Overlay_CaptureList: RENDERDOC_OverlayBits = 8;
pub const eRENDERDOC_Overlay_Default: RENDERDOC_OverlayBits = 15;
pub const eRENDERDOC_Overlay_All: RENDERDOC_OverlayBits = 4294967295;
pub const eRENDERDOC_Overlay_None: RENDERDOC_OverlayBits = 0;
pub type RENDERDOC_OverlayBits = ::std::os::raw::c_uint;
pub type pRENDERDOC_GetOverlayBits = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type pRENDERDOC_MaskOverlayBits =
    ::std::option::Option<unsafe extern "C" fn(And: u32, Or: u32)>;
pub type pRENDERDOC_RemoveHooks = ::std::option::Option<unsafe extern "C" fn()>;
pub type pRENDERDOC_Shutdown = pRENDERDOC_RemoveHooks;
pub type pRENDERDOC_UnloadCrashHandler = ::std::option::Option<unsafe extern "C" fn()>;
pub type pRENDERDOC_SetCaptureFilePathTemplate =
    ::std::option::Option<unsafe extern "C" fn(pathtemplate: *const ::std::os::raw::c_char)>;
pub type pRENDERDOC_GetCaptureFilePathTemplate =
    ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>;
pub type pRENDERDOC_SetLogFilePathTemplate = pRENDERDOC_SetCaptureFilePathTemplate;
pub type pRENDERDOC_GetLogFilePathTemplate = pRENDERDOC_GetCaptureFilePathTemplate;
pub type pRENDERDOC_GetNumCaptures = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type pRENDERDOC_GetCapture = ::std::option::Option<
    unsafe extern "C" fn(
        idx: u32,
        filename: *mut ::std::os::raw::c_char,
        pathlength: *mut u32,
        timestamp: *mut u64,
    ) -> u32,
>;
pub type pRENDERDOC_SetCaptureFileComments = ::std::option::Option<
    unsafe extern "C" fn(
        filePath: *const ::std::os::raw::c_char,
        comments: *const ::std::os::raw::c_char,
    ),
>;
pub type pRENDERDOC_IsTargetControlConnected = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type pRENDERDOC_IsRemoteAccessConnected = pRENDERDOC_IsTargetControlConnected;
pub type pRENDERDOC_LaunchReplayUI = ::std::option::Option<
    unsafe extern "C" fn(connectTargetControl: u32, cmdline: *const ::std::os::raw::c_char) -> u32,
>;
pub type pRENDERDOC_GetAPIVersion = ::std::option::Option<
    unsafe extern "C" fn(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        patch: *mut ::std::os::raw::c_int,
    ),
>;
pub type pRENDERDOC_ShowReplayUI = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type RENDERDOC_DevicePointer = *mut ::std::os::raw::c_void;
pub type RENDERDOC_WindowHandle = *mut ::std::os::raw::c_void;
pub type pRENDERDOC_SetActiveWindow = ::std::option::Option<
    unsafe extern "C" fn(device: RENDERDOC_DevicePointer, wndHandle: RENDERDOC_WindowHandle),
>;
pub type pRENDERDOC_TriggerCapture = ::std::option::Option<unsafe extern "C" fn()>;
pub type pRENDERDOC_TriggerMultiFrameCapture =
    ::std::option::Option<unsafe extern "C" fn(numFrames: u32)>;
pub type pRENDERDOC_StartFrameCapture = ::std::option::Option<
    unsafe extern "C" fn(device: RENDERDOC_DevicePointer, wndHandle: RENDERDOC_WindowHandle),
>;
pub type pRENDERDOC_IsFrameCapturing = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type pRENDERDOC_EndFrameCapture = ::std::option::Option<
    unsafe extern "C" fn(device: RENDERDOC_DevicePointer, wndHandle: RENDERDOC_WindowHandle) -> u32,
>;
pub type pRENDERDOC_DiscardFrameCapture = ::std::option::Option<
    unsafe extern "C" fn(device: RENDERDOC_DevicePointer, wndHandle: RENDERDOC_WindowHandle) -> u32,
>;
pub type pRENDERDOC_SetCaptureTitle =
    ::std::option::Option<unsafe extern "C" fn(title: *const ::std::os::raw::c_char)>;
pub const eRENDERDOC_API_Version_1_0_0: RENDERDOC_Version = 10000;
pub const eRENDERDOC_API_Version_1_0_1: RENDERDOC_Version = 10001;
pub const eRENDERDOC_API_Version_1_0_2: RENDERDOC_Version = 10002;
pub const eRENDERDOC_API_Version_1_1_0: RENDERDOC_Version = 10100;
pub const eRENDERDOC_API_Version_1_1_1: RENDERDOC_Version = 10101;
pub const eRENDERDOC_API_Version_1_1_2: RENDERDOC_Version = 10102;
pub const eRENDERDOC_API_Version_1_2_0: RENDERDOC_Version = 10200;
pub const eRENDERDOC_API_Version_1_3_0: RENDERDOC_Version = 10300;
pub const eRENDERDOC_API_Version_1_4_0: RENDERDOC_Version = 10400;
pub const eRENDERDOC_API_Version_1_4_1: RENDERDOC_Version = 10401;
pub const eRENDERDOC_API_Version_1_4_2: RENDERDOC_Version = 10402;
pub const eRENDERDOC_API_Version_1_5_0: RENDERDOC_Version = 10500;
pub const eRENDERDOC_API_Version_1_6_0: RENDERDOC_Version = 10600;
pub type RENDERDOC_Version = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RENDERDOC_API_1_6_0 {
    pub GetAPIVersion: pRENDERDOC_GetAPIVersion,
    pub SetCaptureOptionU32: pRENDERDOC_SetCaptureOptionU32,
    pub SetCaptureOptionF32: pRENDERDOC_SetCaptureOptionF32,
    pub GetCaptureOptionU32: pRENDERDOC_GetCaptureOptionU32,
    pub GetCaptureOptionF32: pRENDERDOC_GetCaptureOptionF32,
    pub SetFocusToggleKeys: pRENDERDOC_SetFocusToggleKeys,
    pub SetCaptureKeys: pRENDERDOC_SetCaptureKeys,
    pub GetOverlayBits: pRENDERDOC_GetOverlayBits,
    pub MaskOverlayBits: pRENDERDOC_MaskOverlayBits,
    pub __bindgen_anon_1: RENDERDOC_API_1_6_0__bindgen_ty_1,
    pub UnloadCrashHandler: pRENDERDOC_UnloadCrashHandler,
    pub __bindgen_anon_2: RENDERDOC_API_1_6_0__bindgen_ty_2,
    pub __bindgen_anon_3: RENDERDOC_API_1_6_0__bindgen_ty_3,
    pub GetNumCaptures: pRENDERDOC_GetNumCaptures,
    pub GetCapture: pRENDERDOC_GetCapture,
    pub TriggerCapture: pRENDERDOC_TriggerCapture,
    pub __bindgen_anon_4: RENDERDOC_API_1_6_0__bindgen_ty_4,
    pub LaunchReplayUI: pRENDERDOC_LaunchReplayUI,
    pub SetActiveWindow: pRENDERDOC_SetActiveWindow,
    pub StartFrameCapture: pRENDERDOC_StartFrameCapture,
    pub IsFrameCapturing: pRENDERDOC_IsFrameCapturing,
    pub EndFrameCapture: pRENDERDOC_EndFrameCapture,
    pub TriggerMultiFrameCapture: pRENDERDOC_TriggerMultiFrameCapture,
    pub SetCaptureFileComments: pRENDERDOC_SetCaptureFileComments,
    pub DiscardFrameCapture: pRENDERDOC_DiscardFrameCapture,
    pub ShowReplayUI: pRENDERDOC_ShowReplayUI,
    pub SetCaptureTitle: pRENDERDOC_SetCaptureTitle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RENDERDOC_API_1_6_0__bindgen_ty_1 {
    pub Shutdown: pRENDERDOC_Shutdown,
    pub RemoveHooks: pRENDERDOC_RemoveHooks,
}
#[test]
fn bindgen_test_layout_RENDERDOC_API_1_6_0__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<RENDERDOC_API_1_6_0__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RENDERDOC_API_1_6_0__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(RENDERDOC_API_1_6_0__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<RENDERDOC_API_1_6_0__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Shutdown) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_1),
            "::",
            stringify!(Shutdown)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RemoveHooks) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_1),
            "::",
            stringify!(RemoveHooks)
        )
    );
}
impl ::std::fmt::Debug for RENDERDOC_API_1_6_0__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "RENDERDOC_API_1_6_0__bindgen_ty_1 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RENDERDOC_API_1_6_0__bindgen_ty_2 {
    pub SetLogFilePathTemplate: pRENDERDOC_SetLogFilePathTemplate,
    pub SetCaptureFilePathTemplate: pRENDERDOC_SetCaptureFilePathTemplate,
}
#[test]
fn bindgen_test_layout_RENDERDOC_API_1_6_0__bindgen_ty_2() {
    const UNINIT: ::std::mem::MaybeUninit<RENDERDOC_API_1_6_0__bindgen_ty_2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RENDERDOC_API_1_6_0__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(RENDERDOC_API_1_6_0__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<RENDERDOC_API_1_6_0__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetLogFilePathTemplate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_2),
            "::",
            stringify!(SetLogFilePathTemplate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureFilePathTemplate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_2),
            "::",
            stringify!(SetCaptureFilePathTemplate)
        )
    );
}
impl ::std::fmt::Debug for RENDERDOC_API_1_6_0__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "RENDERDOC_API_1_6_0__bindgen_ty_2 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RENDERDOC_API_1_6_0__bindgen_ty_3 {
    pub GetLogFilePathTemplate: pRENDERDOC_GetLogFilePathTemplate,
    pub GetCaptureFilePathTemplate: pRENDERDOC_GetCaptureFilePathTemplate,
}
#[test]
fn bindgen_test_layout_RENDERDOC_API_1_6_0__bindgen_ty_3() {
    const UNINIT: ::std::mem::MaybeUninit<RENDERDOC_API_1_6_0__bindgen_ty_3> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RENDERDOC_API_1_6_0__bindgen_ty_3>(),
        8usize,
        concat!("Size of: ", stringify!(RENDERDOC_API_1_6_0__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<RENDERDOC_API_1_6_0__bindgen_ty_3>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetLogFilePathTemplate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_3),
            "::",
            stringify!(GetLogFilePathTemplate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetCaptureFilePathTemplate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_3),
            "::",
            stringify!(GetCaptureFilePathTemplate)
        )
    );
}
impl ::std::fmt::Debug for RENDERDOC_API_1_6_0__bindgen_ty_3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "RENDERDOC_API_1_6_0__bindgen_ty_3 {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RENDERDOC_API_1_6_0__bindgen_ty_4 {
    pub IsRemoteAccessConnected: pRENDERDOC_IsRemoteAccessConnected,
    pub IsTargetControlConnected: pRENDERDOC_IsTargetControlConnected,
}
#[test]
fn bindgen_test_layout_RENDERDOC_API_1_6_0__bindgen_ty_4() {
    const UNINIT: ::std::mem::MaybeUninit<RENDERDOC_API_1_6_0__bindgen_ty_4> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RENDERDOC_API_1_6_0__bindgen_ty_4>(),
        8usize,
        concat!("Size of: ", stringify!(RENDERDOC_API_1_6_0__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<RENDERDOC_API_1_6_0__bindgen_ty_4>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsRemoteAccessConnected) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_4),
            "::",
            stringify!(IsRemoteAccessConnected)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsTargetControlConnected) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0__bindgen_ty_4),
            "::",
            stringify!(IsTargetControlConnected)
        )
    );
}
impl ::std::fmt::Debug for RENDERDOC_API_1_6_0__bindgen_ty_4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "RENDERDOC_API_1_6_0__bindgen_ty_4 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_RENDERDOC_API_1_6_0() {
    const UNINIT: ::std::mem::MaybeUninit<RENDERDOC_API_1_6_0> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<RENDERDOC_API_1_6_0>(),
        216usize,
        concat!("Size of: ", stringify!(RENDERDOC_API_1_6_0))
    );
    assert_eq!(
        ::std::mem::align_of::<RENDERDOC_API_1_6_0>(),
        8usize,
        concat!("Alignment of ", stringify!(RENDERDOC_API_1_6_0))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetAPIVersion) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetAPIVersion)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureOptionU32) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetCaptureOptionU32)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureOptionF32) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetCaptureOptionF32)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetCaptureOptionU32) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetCaptureOptionU32)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetCaptureOptionF32) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetCaptureOptionF32)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetFocusToggleKeys) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetFocusToggleKeys)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureKeys) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetCaptureKeys)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetOverlayBits) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetOverlayBits)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaskOverlayBits) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(MaskOverlayBits)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).UnloadCrashHandler) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(UnloadCrashHandler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetNumCaptures) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetNumCaptures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GetCapture) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(GetCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TriggerCapture) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(TriggerCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).LaunchReplayUI) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(LaunchReplayUI)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetActiveWindow) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetActiveWindow)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).StartFrameCapture) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(StartFrameCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsFrameCapturing) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(IsFrameCapturing)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).EndFrameCapture) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(EndFrameCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TriggerMultiFrameCapture) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(TriggerMultiFrameCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureFileComments) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetCaptureFileComments)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DiscardFrameCapture) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(DiscardFrameCapture)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ShowReplayUI) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(ShowReplayUI)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SetCaptureTitle) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(RENDERDOC_API_1_6_0),
            "::",
            stringify!(SetCaptureTitle)
        )
    );
}
impl ::std::fmt::Debug for RENDERDOC_API_1_6_0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "RENDERDOC_API_1_6_0 {{ GetAPIVersion: {:?}, SetCaptureOptionU32: {:?}, SetCaptureOptionF32: {:?}, GetCaptureOptionU32: {:?}, GetCaptureOptionF32: {:?}, SetFocusToggleKeys: {:?}, SetCaptureKeys: {:?}, GetOverlayBits: {:?}, MaskOverlayBits: {:?}, __bindgen_anon_1: {:?}, UnloadCrashHandler: {:?}, __bindgen_anon_2: {:?}, __bindgen_anon_3: {:?}, GetNumCaptures: {:?}, GetCapture: {:?}, TriggerCapture: {:?}, __bindgen_anon_4: {:?}, LaunchReplayUI: {:?}, SetActiveWindow: {:?}, StartFrameCapture: {:?}, IsFrameCapturing: {:?}, EndFrameCapture: {:?}, TriggerMultiFrameCapture: {:?}, SetCaptureFileComments: {:?}, DiscardFrameCapture: {:?}, ShowReplayUI: {:?}, SetCaptureTitle: {:?} }}" , self . GetAPIVersion , self . SetCaptureOptionU32 , self . SetCaptureOptionF32 , self . GetCaptureOptionU32 , self . GetCaptureOptionF32 , self . SetFocusToggleKeys , self . SetCaptureKeys , self . GetOverlayBits , self . MaskOverlayBits , self . __bindgen_anon_1 , self . UnloadCrashHandler , self . __bindgen_anon_2 , self . __bindgen_anon_3 , self . GetNumCaptures , self . GetCapture , self . TriggerCapture , self . __bindgen_anon_4 , self . LaunchReplayUI , self . SetActiveWindow , self . StartFrameCapture , self . IsFrameCapturing , self . EndFrameCapture , self . TriggerMultiFrameCapture , self . SetCaptureFileComments , self . DiscardFrameCapture , self . ShowReplayUI , self . SetCaptureTitle)
    }
}
pub type RENDERDOC_API_1_0_0 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_0_1 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_0_2 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_1_0 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_1_1 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_1_2 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_2_0 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_3_0 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_4_0 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_4_1 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_4_2 = RENDERDOC_API_1_6_0;
pub type RENDERDOC_API_1_5_0 = RENDERDOC_API_1_6_0;
pub type pRENDERDOC_GetAPI = ::std::option::Option<
    unsafe extern "C" fn(
        version: RENDERDOC_Version,
        outAPIPointers: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
