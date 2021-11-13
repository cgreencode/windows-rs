#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: Self = Self(0i32);
    pub const SavingData: Self = Self(1i32);
    pub const BackgroundAudio: Self = Self(2i32);
    pub const Unconstrained: Self = Self(3i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundResult {}
impl ::core::clone::Clone for ExtendedExecutionForegroundResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExtendedExecutionForegroundRevokedEventArgs {}
impl ::core::clone::Clone for ExtendedExecutionForegroundRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionForegroundRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionForegroundRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExtendedExecutionForegroundSession {}
impl ::core::clone::Clone for ExtendedExecutionForegroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendedExecutionForegroundRevokedEventArgs {}
impl ::core::clone::Clone for IExtendedExecutionForegroundRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendedExecutionForegroundSession {}
impl ::core::clone::Clone for IExtendedExecutionForegroundSession {
    fn clone(&self) -> Self {
        *self
    }
}
