#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserNotificationListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserNotificationListener {}
impl ::core::clone::Clone for IUserNotificationListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserNotificationListenerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserNotificationListenerStatics {}
impl ::core::clone::Clone for IUserNotificationListenerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotificationListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserNotificationListener {}
impl ::core::clone::Clone for UserNotificationListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotificationListenerAccessStatus(pub i32);
impl UserNotificationListenerAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for UserNotificationListenerAccessStatus {}
impl ::core::clone::Clone for UserNotificationListenerAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
