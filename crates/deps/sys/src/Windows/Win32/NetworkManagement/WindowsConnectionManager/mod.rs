#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInterfaceContextTableForHostName(hostname: super::super::Foundation::PWSTR, proxyname: super::super::Foundation::PWSTR, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32, interfacecontexttable: *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandGetRoutingHint(destinationhostname: super::super::Foundation::PWSTR, interfaceindex: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void, registrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void);
    pub fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmQueryProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcmSetProperty(pinterface: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, dwdatasize: u32, pbdata: *const u8) -> u32;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_INTERFACE_CONTEXT {
    pub InterfaceIndex: u32,
    pub ConfigurationName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_INTERFACE_CONTEXT_TABLE {
    pub InterfaceContextHandle: super::super::Foundation::HANDLE,
    pub NumberOfEntries: u32,
    pub InterfaceContextArray: *mut NET_INTERFACE_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
pub type ONDEMAND_NOTIFICATION_CALLBACK = unsafe extern "system" fn(param0: *const ::core::ffi::c_void);
pub const WCM_API_VERSION: u32 = 1u32;
pub const WCM_API_VERSION_1_0: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::super::Foundation::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_BILLING_CYCLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_BILLING_CYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WCM_CONNECTION_COST(pub i32);
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST {}
impl ::core::clone::Clone for WCM_CONNECTION_COST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
impl ::core::marker::Copy for WCM_CONNECTION_COST_DATA {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST_SOURCE {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
#[repr(transparent)]
pub struct WCM_MEDIA_TYPE(pub i32);
pub const wcm_media_unknown: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(0i32);
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(1i32);
pub const wcm_media_wlan: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(2i32);
pub const wcm_media_mbn: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(3i32);
pub const wcm_media_invalid: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(4i32);
pub const wcm_media_max: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(5i32);
impl ::core::marker::Copy for WCM_MEDIA_TYPE {}
impl ::core::clone::Clone for WCM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_POLICY_VALUE {
    pub fValue: super::super::Foundation::BOOL,
    pub fIsGroupPolicy: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_POLICY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_POLICY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: ::windows_sys::core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl ::core::marker::Copy for WCM_PROFILE_INFO {}
impl ::core::clone::Clone for WCM_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl ::core::marker::Copy for WCM_PROFILE_INFO_LIST {}
impl ::core::clone::Clone for WCM_PROFILE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WCM_PROPERTY(pub i32);
pub const wcm_global_property_domain_policy: WCM_PROPERTY = WCM_PROPERTY(0i32);
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = WCM_PROPERTY(1i32);
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = WCM_PROPERTY(2i32);
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = WCM_PROPERTY(3i32);
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = WCM_PROPERTY(4i32);
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = WCM_PROPERTY(5i32);
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = WCM_PROPERTY(6i32);
impl ::core::marker::Copy for WCM_PROPERTY {}
impl ::core::clone::Clone for WCM_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WCM_TIME_INTERVAL {}
impl ::core::clone::Clone for WCM_TIME_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
