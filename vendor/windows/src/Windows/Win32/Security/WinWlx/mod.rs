pub const STATUSMSG_OPTION_NOANIMATION: u32 = 1u32;
pub const STATUSMSG_OPTION_SETFOREGROUND: u32 = 2u32;
pub const WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREATE_INSTANCE_ONLY: u32 = 1u32;
pub const WLX_CREATE_USER: u32 = 2u32;
pub const WLX_CREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREDENTIAL_TYPE_V2_0: u32 = 2u32;
pub const WLX_CURRENT_VERSION: u32 = 65540u32;
pub const WLX_DESKTOP_HANDLE: u32 = 2u32;
pub const WLX_DESKTOP_NAME: u32 = 1u32;
pub const WLX_DIRECTORY_LENGTH: u32 = 256u32;
pub const WLX_DLG_INPUT_TIMEOUT: u32 = 102u32;
pub const WLX_DLG_SAS: u32 = 101u32;
pub const WLX_DLG_SCREEN_SAVER_TIMEOUT: u32 = 103u32;
pub const WLX_DLG_USER_LOGOFF: u32 = 104u32;
pub const WLX_LOGON_OPT_NO_PROFILE: u32 = 1u32;
pub const WLX_OPTION_CONTEXT_POINTER: u32 = 2u32;
pub const WLX_OPTION_DISPATCH_TABLE_SIZE: u32 = 65539u32;
pub const WLX_OPTION_FORCE_LOGOFF_TIME: u32 = 4u32;
pub const WLX_OPTION_IGNORE_AUTO_LOGON: u32 = 8u32;
pub const WLX_OPTION_NO_SWITCH_ON_SAS: u32 = 9u32;
pub const WLX_OPTION_SMART_CARD_INFO: u32 = 65538u32;
pub const WLX_OPTION_SMART_CARD_PRESENT: u32 = 65537u32;
pub const WLX_OPTION_USE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_OPTION_USE_SMART_CARD: u32 = 3u32;
pub const WLX_PROFILE_TYPE_V1_0: u32 = 1u32;
pub const WLX_PROFILE_TYPE_V2_0: u32 = 2u32;
pub const WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF: u32 = 16u32;
pub const WLX_SAS_ACTION_FORCE_LOGOFF: u32 = 9u32;
pub const WLX_SAS_ACTION_LOCK_WKSTA: u32 = 3u32;
pub const WLX_SAS_ACTION_LOGOFF: u32 = 4u32;
pub const WLX_SAS_ACTION_LOGON: u32 = 1u32;
pub const WLX_SAS_ACTION_NONE: u32 = 2u32;
pub const WLX_SAS_ACTION_PWD_CHANGED: u32 = 6u32;
pub const WLX_SAS_ACTION_RECONNECTED: u32 = 15u32;
pub const WLX_SAS_ACTION_SHUTDOWN: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(5u32);
pub const WLX_SAS_ACTION_SHUTDOWN_HIBERNATE: u32 = 14u32;
pub const WLX_SAS_ACTION_SHUTDOWN_POWER_OFF: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(10u32);
pub const WLX_SAS_ACTION_SHUTDOWN_REBOOT: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(11u32);
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP: u32 = 12u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP2: u32 = 13u32;
pub const WLX_SAS_ACTION_SWITCH_CONSOLE: u32 = 17u32;
pub const WLX_SAS_ACTION_TASKLIST: u32 = 7u32;
pub const WLX_SAS_ACTION_UNLOCK_WKSTA: u32 = 8u32;
pub const WLX_SAS_TYPE_AUTHENTICATED: u32 = 7u32;
pub const WLX_SAS_TYPE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_SAS_TYPE_MAX_MSFT_VALUE: u32 = 127u32;
pub const WLX_SAS_TYPE_SCRNSVR_ACTIVITY: u32 = 3u32;
pub const WLX_SAS_TYPE_SCRNSVR_TIMEOUT: u32 = 2u32;
pub const WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED: u32 = 8u32;
pub const WLX_SAS_TYPE_SC_INSERT: u32 = 5u32;
pub const WLX_SAS_TYPE_SC_LAST_READER_REMOVED: u32 = 9u32;
pub const WLX_SAS_TYPE_SC_REMOVE: u32 = 6u32;
pub const WLX_SAS_TYPE_SWITCHUSER: u32 = 10u32;
pub const WLX_SAS_TYPE_TIMEOUT: u32 = 0u32;
pub const WLX_SAS_TYPE_USER_LOGOFF: u32 = 4u32;
pub const WLX_VERSION_1_0: u32 = 65536u32;
pub const WLX_VERSION_1_1: u32 = 65537u32;
pub const WLX_VERSION_1_2: u32 = 65538u32;
pub const WLX_VERSION_1_3: u32 = 65539u32;
pub const WLX_VERSION_1_4: u32 = 65540u32;
pub const WLX_WM_SAS: u32 = 1625u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct WLX_SHUTDOWN_TYPE(pub u32);
impl ::windows_core::TypeKind for WLX_SHUTDOWN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WLX_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLX_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub pszUserName: ::windows_core::PWSTR,
    pub pszDomain: ::windows_core::PWSTR,
    pub pszPassword: ::windows_core::PWSTR,
    pub fPromptForPassword: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {}
impl ::core::clone::Clone for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V1_0").field("dwType", &self.dwType).field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("fPromptForPassword", &self.fPromptForPassword).finish()
    }
}
impl ::windows_core::TypeKind for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.fPromptForPassword == other.fPromptForPassword
    }
}
impl ::core::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {}
impl ::core::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    pub dwType: u32,
    pub pszUserName: ::windows_core::PWSTR,
    pub pszDomain: ::windows_core::PWSTR,
    pub pszPassword: ::windows_core::PWSTR,
    pub fPromptForPassword: super::super::Foundation::BOOL,
    pub fDisconnectOnLogonFailure: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {}
impl ::core::clone::Clone for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V2_0").field("dwType", &self.dwType).field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("fPromptForPassword", &self.fPromptForPassword).field("fDisconnectOnLogonFailure", &self.fDisconnectOnLogonFailure).finish()
    }
}
impl ::windows_core::TypeKind for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.fPromptForPassword == other.fPromptForPassword && self.fDisconnectOnLogonFailure == other.fDisconnectOnLogonFailure
    }
}
impl ::core::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {}
impl ::core::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub UserToken: super::super::Foundation::HANDLE,
    pub LogonId: super::super::Foundation::LUID,
    pub Quotas: super::QUOTA_LIMITS,
    pub UserName: ::windows_core::PWSTR,
    pub Domain: ::windows_core::PWSTR,
    pub LogonTime: i64,
    pub SmartCardLogon: super::super::Foundation::BOOL,
    pub ProfileLength: u32,
    pub MessageType: u32,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub ProfileLogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: ::windows_core::PWSTR,
    pub HomeDirectory: ::windows_core::PWSTR,
    pub FullName: ::windows_core::PWSTR,
    pub ProfilePath: ::windows_core::PWSTR,
    pub HomeDirectoryDrive: ::windows_core::PWSTR,
    pub LogonServer: ::windows_core::PWSTR,
    pub UserFlags: u32,
    pub PrivateDataLen: u32,
    pub PrivateData: *mut u8,
}
impl ::core::marker::Copy for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {}
impl ::core::clone::Clone for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0")
            .field("dwType", &self.dwType)
            .field("UserToken", &self.UserToken)
            .field("LogonId", &self.LogonId)
            .field("Quotas", &self.Quotas)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .field("LogonTime", &self.LogonTime)
            .field("SmartCardLogon", &self.SmartCardLogon)
            .field("ProfileLength", &self.ProfileLength)
            .field("MessageType", &self.MessageType)
            .field("LogonCount", &self.LogonCount)
            .field("BadPasswordCount", &self.BadPasswordCount)
            .field("ProfileLogonTime", &self.ProfileLogonTime)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .field("LogonScript", &self.LogonScript)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("FullName", &self.FullName)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogonServer", &self.LogonServer)
            .field("UserFlags", &self.UserFlags)
            .field("PrivateDataLen", &self.PrivateDataLen)
            .field("PrivateData", &self.PrivateData)
            .finish()
    }
}
impl ::windows_core::TypeKind for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.UserToken == other.UserToken
            && self.LogonId == other.LogonId
            && self.Quotas == other.Quotas
            && self.UserName == other.UserName
            && self.Domain == other.Domain
            && self.LogonTime == other.LogonTime
            && self.SmartCardLogon == other.SmartCardLogon
            && self.ProfileLength == other.ProfileLength
            && self.MessageType == other.MessageType
            && self.LogonCount == other.LogonCount
            && self.BadPasswordCount == other.BadPasswordCount
            && self.ProfileLogonTime == other.ProfileLogonTime
            && self.LogoffTime == other.LogoffTime
            && self.KickOffTime == other.KickOffTime
            && self.PasswordLastSet == other.PasswordLastSet
            && self.PasswordCanChange == other.PasswordCanChange
            && self.PasswordMustChange == other.PasswordMustChange
            && self.LogonScript == other.LogonScript
            && self.HomeDirectory == other.HomeDirectory
            && self.FullName == other.FullName
            && self.ProfilePath == other.ProfilePath
            && self.HomeDirectoryDrive == other.HomeDirectoryDrive
            && self.LogonServer == other.LogonServer
            && self.UserFlags == other.UserFlags
            && self.PrivateDataLen == other.PrivateDataLen
            && self.PrivateData == other.PrivateData
    }
}
impl ::core::cmp::Eq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {}
impl ::core::default::Default for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub struct WLX_DESKTOP {
    pub Size: u32,
    pub Flags: u32,
    pub hDesktop: super::super::System::StationsAndDesktops::HDESK,
    pub pszDesktopName: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::marker::Copy for WLX_DESKTOP {}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::clone::Clone for WLX_DESKTOP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::fmt::Debug for WLX_DESKTOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DESKTOP").field("Size", &self.Size).field("Flags", &self.Flags).field("hDesktop", &self.hDesktop).field("pszDesktopName", &self.pszDesktopName).finish()
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::windows_core::TypeKind for WLX_DESKTOP {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::cmp::PartialEq for WLX_DESKTOP {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.hDesktop == other.hDesktop && self.pszDesktopName == other.pszDesktopName
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::cmp::Eq for WLX_DESKTOP {}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::default::Default for WLX_DESKTOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct WLX_DISPATCH_VERSION_1_0 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for WLX_DISPATCH_VERSION_1_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for WLX_DISPATCH_VERSION_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for WLX_DISPATCH_VERSION_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DISPATCH_VERSION_1_0").finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::TypeKind for WLX_DISPATCH_VERSION_1_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_1 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for WLX_DISPATCH_VERSION_1_1 {}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for WLX_DISPATCH_VERSION_1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for WLX_DISPATCH_VERSION_1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DISPATCH_VERSION_1_1").finish()
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for WLX_DISPATCH_VERSION_1_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_2 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for WLX_DISPATCH_VERSION_1_2 {}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for WLX_DISPATCH_VERSION_1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for WLX_DISPATCH_VERSION_1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DISPATCH_VERSION_1_2").finish()
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for WLX_DISPATCH_VERSION_1_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_3 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
    pub WlxSetOption: PWLX_SET_OPTION,
    pub WlxGetOption: PWLX_GET_OPTION,
    pub WlxWin31Migrate: PWLX_WIN31_MIGRATE,
    pub WlxQueryClientCredentials: PWLX_QUERY_CLIENT_CREDENTIALS,
    pub WlxQueryInetConnectorCredentials: PWLX_QUERY_IC_CREDENTIALS,
    pub WlxDisconnect: PWLX_DISCONNECT,
    pub WlxQueryTerminalServicesData: PWLX_QUERY_TERMINAL_SERVICES_DATA,
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for WLX_DISPATCH_VERSION_1_3 {}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for WLX_DISPATCH_VERSION_1_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for WLX_DISPATCH_VERSION_1_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DISPATCH_VERSION_1_3").finish()
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for WLX_DISPATCH_VERSION_1_3 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_4 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
    pub WlxSetOption: PWLX_SET_OPTION,
    pub WlxGetOption: PWLX_GET_OPTION,
    pub WlxWin31Migrate: PWLX_WIN31_MIGRATE,
    pub WlxQueryClientCredentials: PWLX_QUERY_CLIENT_CREDENTIALS,
    pub WlxQueryInetConnectorCredentials: PWLX_QUERY_IC_CREDENTIALS,
    pub WlxDisconnect: PWLX_DISCONNECT,
    pub WlxQueryTerminalServicesData: PWLX_QUERY_TERMINAL_SERVICES_DATA,
    pub WlxQueryConsoleSwitchCredentials: PWLX_QUERY_CONSOLESWITCH_CREDENTIALS,
    pub WlxQueryTsLogonCredentials: PWLX_QUERY_TS_LOGON_CREDENTIALS,
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for WLX_DISPATCH_VERSION_1_4 {}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for WLX_DISPATCH_VERSION_1_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for WLX_DISPATCH_VERSION_1_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DISPATCH_VERSION_1_4").finish()
    }
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for WLX_DISPATCH_VERSION_1_4 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_MPR_NOTIFY_INFO {
    pub pszUserName: ::windows_core::PWSTR,
    pub pszDomain: ::windows_core::PWSTR,
    pub pszPassword: ::windows_core::PWSTR,
    pub pszOldPassword: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WLX_MPR_NOTIFY_INFO {}
impl ::core::clone::Clone for WLX_MPR_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_MPR_NOTIFY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_MPR_NOTIFY_INFO").field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("pszOldPassword", &self.pszOldPassword).finish()
    }
}
impl ::windows_core::TypeKind for WLX_MPR_NOTIFY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_MPR_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.pszOldPassword == other.pszOldPassword
    }
}
impl ::core::cmp::Eq for WLX_MPR_NOTIFY_INFO {}
impl ::core::default::Default for WLX_MPR_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub struct WLX_NOTIFICATION_INFO {
    pub Size: u32,
    pub Flags: u32,
    pub UserName: ::windows_core::PWSTR,
    pub Domain: ::windows_core::PWSTR,
    pub WindowStation: ::windows_core::PWSTR,
    pub hToken: super::super::Foundation::HANDLE,
    pub hDesktop: super::super::System::StationsAndDesktops::HDESK,
    pub pStatusCallback: PFNMSGECALLBACK,
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::marker::Copy for WLX_NOTIFICATION_INFO {}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::clone::Clone for WLX_NOTIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::fmt::Debug for WLX_NOTIFICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_NOTIFICATION_INFO").field("Size", &self.Size).field("Flags", &self.Flags).field("UserName", &self.UserName).field("Domain", &self.Domain).field("WindowStation", &self.WindowStation).field("hToken", &self.hToken).field("hDesktop", &self.hDesktop).finish()
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::windows_core::TypeKind for WLX_NOTIFICATION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::default::Default for WLX_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_PROFILE_V1_0 {
    pub dwType: u32,
    pub pszProfile: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WLX_PROFILE_V1_0 {}
impl ::core::clone::Clone for WLX_PROFILE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_PROFILE_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_PROFILE_V1_0").field("dwType", &self.dwType).field("pszProfile", &self.pszProfile).finish()
    }
}
impl ::windows_core::TypeKind for WLX_PROFILE_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_PROFILE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszProfile == other.pszProfile
    }
}
impl ::core::cmp::Eq for WLX_PROFILE_V1_0 {}
impl ::core::default::Default for WLX_PROFILE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_PROFILE_V2_0 {
    pub dwType: u32,
    pub pszProfile: ::windows_core::PWSTR,
    pub pszPolicy: ::windows_core::PWSTR,
    pub pszNetworkDefaultUserProfile: ::windows_core::PWSTR,
    pub pszServerName: ::windows_core::PWSTR,
    pub pszEnvironment: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WLX_PROFILE_V2_0 {}
impl ::core::clone::Clone for WLX_PROFILE_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_PROFILE_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_PROFILE_V2_0").field("dwType", &self.dwType).field("pszProfile", &self.pszProfile).field("pszPolicy", &self.pszPolicy).field("pszNetworkDefaultUserProfile", &self.pszNetworkDefaultUserProfile).field("pszServerName", &self.pszServerName).field("pszEnvironment", &self.pszEnvironment).finish()
    }
}
impl ::windows_core::TypeKind for WLX_PROFILE_V2_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_PROFILE_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszProfile == other.pszProfile && self.pszPolicy == other.pszPolicy && self.pszNetworkDefaultUserProfile == other.pszNetworkDefaultUserProfile && self.pszServerName == other.pszServerName && self.pszEnvironment == other.pszEnvironment
    }
}
impl ::core::cmp::Eq for WLX_PROFILE_V2_0 {}
impl ::core::default::Default for WLX_PROFILE_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_SC_NOTIFICATION_INFO {
    pub pszCard: ::windows_core::PWSTR,
    pub pszReader: ::windows_core::PWSTR,
    pub pszContainer: ::windows_core::PWSTR,
    pub pszCryptoProvider: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WLX_SC_NOTIFICATION_INFO {}
impl ::core::clone::Clone for WLX_SC_NOTIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_SC_NOTIFICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_SC_NOTIFICATION_INFO").field("pszCard", &self.pszCard).field("pszReader", &self.pszReader).field("pszContainer", &self.pszContainer).field("pszCryptoProvider", &self.pszCryptoProvider).finish()
    }
}
impl ::windows_core::TypeKind for WLX_SC_NOTIFICATION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_SC_NOTIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszCard == other.pszCard && self.pszReader == other.pszReader && self.pszContainer == other.pszContainer && self.pszCryptoProvider == other.pszCryptoProvider
    }
}
impl ::core::cmp::Eq for WLX_SC_NOTIFICATION_INFO {}
impl ::core::default::Default for WLX_SC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLX_TERMINAL_SERVICES_DATA {
    pub ProfilePath: [u16; 257],
    pub HomeDir: [u16; 257],
    pub HomeDirDrive: [u16; 4],
}
impl ::core::marker::Copy for WLX_TERMINAL_SERVICES_DATA {}
impl ::core::clone::Clone for WLX_TERMINAL_SERVICES_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLX_TERMINAL_SERVICES_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_TERMINAL_SERVICES_DATA").field("ProfilePath", &self.ProfilePath).field("HomeDir", &self.HomeDir).field("HomeDirDrive", &self.HomeDirDrive).finish()
    }
}
impl ::windows_core::TypeKind for WLX_TERMINAL_SERVICES_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WLX_TERMINAL_SERVICES_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ProfilePath == other.ProfilePath && self.HomeDir == other.HomeDir && self.HomeDirDrive == other.HomeDirDrive
    }
}
impl ::core::cmp::Eq for WLX_TERMINAL_SERVICES_DATA {}
impl ::core::default::Default for WLX_TERMINAL_SERVICES_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PFNMSGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: ::windows_core::PCWSTR) -> u32>;
pub type PWLX_ASSIGN_SHELL_PROTECTION = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_CHANGE_PASSWORD_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32) -> i32>;
pub type PWLX_CHANGE_PASSWORD_NOTIFY_EX = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32, providername: ::windows_core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> i32>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_CLOSE_USER_DESKTOP = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP, htoken: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_CREATE_USER_DESKTOP = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, flags: u32, pszdesktopname: ::windows_core::PCWSTR, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: ::windows_core::PCWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_INDIRECT = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_INDIRECT_PARAM = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_PARAM = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: ::windows_core::PCWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32>;
pub type PWLX_DISCONNECT = ::core::option::Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type PWLX_GET_OPTION = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: *mut usize) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_GET_SOURCE_DESKTOP = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
pub type PWLX_MESSAGE_BOX = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hwndowner: super::super::Foundation::HWND, lpsztext: ::windows_core::PCWSTR, lpsztitle: ::windows_core::PCWSTR, fustyle: u32) -> i32>;
pub type PWLX_QUERY_CLIENT_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL>;
pub type PWLX_QUERY_CONSOLESWITCH_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(pcred: *mut WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0) -> u32>;
pub type PWLX_QUERY_IC_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL>;
pub type PWLX_QUERY_TERMINAL_SERVICES_DATA = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ptsdata: *mut WLX_TERMINAL_SERVICES_DATA, username: ::windows_core::PCWSTR, domain: ::windows_core::PCWSTR) -> u32>;
pub type PWLX_QUERY_TS_LOGON_CREDENTIALS = ::core::option::Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V2_0) -> super::super::Foundation::BOOL>;
pub type PWLX_SAS_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, dwsastype: u32)>;
pub type PWLX_SET_CONTEXT_POINTER = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pwlxcontext: *mut ::core::ffi::c_void)>;
pub type PWLX_SET_OPTION = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: usize, oldvalue: *mut usize) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_SET_RETURN_DESKTOP = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
pub type PWLX_SET_TIMEOUT = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, timeout: u32) -> super::super::Foundation::BOOL>;
pub type PWLX_SWITCH_DESKTOP_TO_USER = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_SWITCH_DESKTOP_TO_WINLOGON = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_USE_CTRL_ALT_DEL = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE)>;
pub type PWLX_WIN31_MIGRATE = ::core::option::Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE)>;
