#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(635342800, 7259, 4577, [189, 219, 8, 0, 32, 12, 154, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PowerSavingMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changehandler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1499608783, 6424, 17745, [164, 102, 197, 26, 174, 55, 59, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Phone_System_Power`*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: `Phone_System_Power`*"]
    pub fn PowerSavingMode() -> ::windows::runtime::Result<PowerSavingMode> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSavingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PowerSavingMode>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_Power`, `Foundation`*"]
    pub fn PowerSavingModeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(changehandler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), changehandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_System_Power`, `Foundation`*"]
    pub fn RemovePowerSavingModeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Phone_System_Power`*"]
    pub fn PowerSavingModeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PowerManager, IPowerManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPowerManagerStatics2<R, F: FnOnce(&IPowerManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PowerManager, IPowerManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[doc = "*Required features: `Phone_System_Power`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: PowerSavingMode = PowerSavingMode(0i32);
    pub const On: PowerSavingMode = PowerSavingMode(1i32);
}
impl ::core::convert::From<i32> for PowerSavingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PowerSavingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PowerSavingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
}
impl ::windows::runtime::DefaultType for PowerSavingMode {
    type DefaultType = Self;
}
