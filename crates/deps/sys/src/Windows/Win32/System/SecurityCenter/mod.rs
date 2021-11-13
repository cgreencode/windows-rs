#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscGetAntiMalwareUri(ppszuri: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_sys::core::HRESULT;
    pub fn WscQueryAntiMalwareUri() -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WscRegisterForUserNotifications() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IWSCDefaultProduct(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWSCDefaultProduct {}
impl ::core::clone::Clone for IWSCDefaultProduct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWSCProductList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWSCProductList {}
impl ::core::clone::Clone for IWSCProductList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct {}
impl ::core::clone::Clone for IWscProduct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct2 {}
impl ::core::clone::Clone for IWscProduct2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct3 {}
impl ::core::clone::Clone for IWscProduct3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
impl ::core::marker::Copy for SECURITY_PRODUCT_TYPE {}
impl ::core::clone::Clone for SECURITY_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSCDefaultProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 696361838, data2: 61997, data3: 4581, data4: [156, 233, 94, 85, 23, 80, 124, 102] };
pub const WSCProductList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 386346875, data2: 39614, data3: 19060, data4: [162, 97, 30, 183, 107, 85, 16, 122] };
#[repr(transparent)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_STATE {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_SUBSTATUS {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER_HEALTH {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER_HEALTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
impl ::core::marker::Copy for WSC_SECURITY_SIGNATURE_STATUS {}
impl ::core::clone::Clone for WSC_SECURITY_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
