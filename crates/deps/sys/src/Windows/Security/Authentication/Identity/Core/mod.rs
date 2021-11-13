#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorAuthenticationManager {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorAuthenticationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorAuthenticatorStatics {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorGetSessionsResult {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorGetSessionsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorOneTimeCodedInfo {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorSessionInfo {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
impl ::core::clone::Clone for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MicrosoftAccountMultiFactorAuthenticationManager {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: Self = Self(0i32);
    pub const Device: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorAuthenticationType {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MicrosoftAccountMultiFactorGetSessionsResult {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorGetSessionsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const NoNetworkConnection: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const TotpSetupDenied: Self = Self(4i32);
    pub const NgcNotSetup: Self = Self(5i32);
    pub const SessionAlreadyDenied: Self = Self(6i32);
    pub const SessionAlreadyApproved: Self = Self(7i32);
    pub const SessionExpired: Self = Self(8i32);
    pub const NgcNonceExpired: Self = Self(9i32);
    pub const InvalidSessionId: Self = Self(10i32);
    pub const InvalidSessionType: Self = Self(11i32);
    pub const InvalidOperation: Self = Self(12i32);
    pub const InvalidStateTransition: Self = Self(13i32);
    pub const DeviceNotFound: Self = Self(14i32);
    pub const FlowDisabled: Self = Self(15i32);
    pub const SessionNotApproved: Self = Self(16i32);
    pub const OperationCanceledByUser: Self = Self(17i32);
    pub const NgcDisabledByServer: Self = Self(18i32);
    pub const NgcKeyNotFoundOnServer: Self = Self(19i32);
    pub const UIRequired: Self = Self(20i32);
    pub const DeviceIdChanged: Self = Self(21i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorServiceResponse {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionApprovalStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionAuthenticationStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionInfo {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
