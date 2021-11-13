#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Management_Core")]
pub mod Core;
#[cfg(feature = "Management_Deployment")]
pub mod Deployment;
#[cfg(feature = "Management_Policies")]
pub mod Policies;
#[cfg(feature = "Management_Update")]
pub mod Update;
#[cfg(feature = "Management_Workplace")]
pub mod Workplace;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMdmAlert(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMdmAlert {}
impl ::core::clone::Clone for IMdmAlert {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMdmSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMdmSession {}
impl ::core::clone::Clone for IMdmSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMdmSessionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMdmSessionManagerStatics {}
impl ::core::clone::Clone for IMdmSessionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MdmAlert(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MdmAlert {}
impl ::core::clone::Clone for MdmAlert {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MdmAlertDataType(pub i32);
impl MdmAlertDataType {
    pub const String: Self = Self(0i32);
    pub const Base64: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
}
impl ::core::marker::Copy for MdmAlertDataType {}
impl ::core::clone::Clone for MdmAlertDataType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MdmAlertMark(pub i32);
impl MdmAlertMark {
    pub const None: Self = Self(0i32);
    pub const Fatal: Self = Self(1i32);
    pub const Critical: Self = Self(2i32);
    pub const Warning: Self = Self(3i32);
    pub const Informational: Self = Self(4i32);
}
impl ::core::marker::Copy for MdmAlertMark {}
impl ::core::clone::Clone for MdmAlertMark {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MdmSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MdmSession {}
impl ::core::clone::Clone for MdmSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MdmSessionState(pub i32);
impl MdmSessionState {
    pub const NotStarted: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Communicating: Self = Self(3i32);
    pub const AlertStatusAvailable: Self = Self(4i32);
    pub const Retrying: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for MdmSessionState {}
impl ::core::clone::Clone for MdmSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
