#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdcChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdcChannel {}
impl ::core::clone::Clone for AdcChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdcChannelMode(pub i32);
impl AdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for AdcChannelMode {}
impl ::core::clone::Clone for AdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdcController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdcController {}
impl ::core::clone::Clone for AdcController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdcChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdcChannel {}
impl ::core::clone::Clone for IAdcChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdcController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdcController {}
impl ::core::clone::Clone for IAdcController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdcControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdcControllerStatics {}
impl ::core::clone::Clone for IAdcControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdcControllerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdcControllerStatics2 {}
impl ::core::clone::Clone for IAdcControllerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
