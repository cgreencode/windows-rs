#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const BlockRange: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186599, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
pub const BlockRangeList: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186600, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
pub const BootOptions: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904974, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3171631974, 36355, 4562, [148, 246, 0, 192, 79, 121, 241, 214]);
pub const CATID_SMTP_DSN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582309681, 62968, 19747, [189, 143, 135, 181, 35, 113, 167, 58]);
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231318154, 64179, 17367, [188, 223, 105, 44, 91, 70, 230, 177]);
pub const CATID_SMTP_LOG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2479924536, 11294, 19304, [167, 201, 215, 58, 138, 166, 238, 151]);
pub const CATID_SMTP_MAXMSGSIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3958462942, 42622, 4562, [148, 247, 0, 192, 79, 121, 241, 214]);
pub const CATID_SMTP_MSGTRACKLOG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336524458, 32176, 4562, [148, 244, 0, 192, 79, 121, 241, 214]);
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653650, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653645, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_MESSAGE_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653648, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653649, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653646, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_SESSION_END: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653651, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_ON_SESSION_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133653647, 3422, 4562, [170, 104, 0, 192, 79, 163, 91, 130]);
pub const CATID_SMTP_STORE_DRIVER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1494702160, 58675, 4561, [170, 103, 0, 192, 79, 163, 69, 246]);
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2516734627, 2618, 4562, [158, 0, 0, 192, 79, 163, 34, 186]);
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1987155540, 1446, 4562, [157, 253, 0, 192, 79, 163, 34, 186]);
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2746022669, 33791, 4562, [158, 20, 0, 192, 79, 163, 34, 186]);
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(674509001, 6224, 4562, [158, 3, 0, 192, 79, 163, 34, 186]);
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4282165795, 185, 4562, [157, 251, 0, 192, 79, 163, 34, 186]);
pub const CLSID_SmtpCat: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2990290359, 37401, 4562, [158, 23, 0, 192, 79, 163, 34, 186]);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[inline]
pub unsafe fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
        }
        ::core::mem::transmute(CloseIMsgSession(::core::mem::transmute(lpmsgsess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DDiscFormat2DataEvents(pub ::windows::runtime::IUnknown);
impl DDiscFormat2DataEvents {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, object: Param0, progress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DDiscFormat2DataEvents {
    type Vtable = DDiscFormat2DataEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801532, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DDiscFormat2DataEvents> for ::windows::runtime::IUnknown {
    fn from(value: DDiscFormat2DataEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DDiscFormat2DataEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DDiscFormat2DataEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DDiscFormat2DataEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DDiscFormat2DataEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2DataEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2DataEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2DataEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2DataEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DDiscFormat2DataEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DDiscFormat2DataEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, progress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DDiscFormat2EraseEvents(pub ::windows::runtime::IUnknown);
impl DDiscFormat2EraseEvents {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, object: Param0, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), ::core::mem::transmute(elapsedseconds), ::core::mem::transmute(estimatedtotalseconds)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DDiscFormat2EraseEvents {
    type Vtable = DDiscFormat2EraseEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801530, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DDiscFormat2EraseEvents> for ::windows::runtime::IUnknown {
    fn from(value: DDiscFormat2EraseEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DDiscFormat2EraseEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DDiscFormat2EraseEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DDiscFormat2EraseEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DDiscFormat2EraseEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2EraseEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2EraseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2EraseEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2EraseEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DDiscFormat2EraseEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DDiscFormat2EraseEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DDiscFormat2RawCDEvents(pub ::windows::runtime::IUnknown);
impl DDiscFormat2RawCDEvents {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, object: Param0, progress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DDiscFormat2RawCDEvents {
    type Vtable = DDiscFormat2RawCDEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801538, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DDiscFormat2RawCDEvents> for ::windows::runtime::IUnknown {
    fn from(value: DDiscFormat2RawCDEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DDiscFormat2RawCDEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DDiscFormat2RawCDEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DDiscFormat2RawCDEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DDiscFormat2RawCDEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2RawCDEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2RawCDEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2RawCDEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2RawCDEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DDiscFormat2RawCDEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DDiscFormat2RawCDEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, progress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DDiscFormat2TrackAtOnceEvents(pub ::windows::runtime::IUnknown);
impl DDiscFormat2TrackAtOnceEvents {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, object: Param0, progress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DDiscFormat2TrackAtOnceEvents {
    type Vtable = DDiscFormat2TrackAtOnceEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801535, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DDiscFormat2TrackAtOnceEvents> for ::windows::runtime::IUnknown {
    fn from(value: DDiscFormat2TrackAtOnceEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DDiscFormat2TrackAtOnceEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DDiscFormat2TrackAtOnceEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DDiscFormat2TrackAtOnceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DDiscFormat2TrackAtOnceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2TrackAtOnceEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2TrackAtOnceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2TrackAtOnceEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2TrackAtOnceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DDiscFormat2TrackAtOnceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DDiscFormat2TrackAtOnceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, progress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DDiscMaster2Events(pub ::windows::runtime::IUnknown);
impl DDiscMaster2Events {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn NotifyDeviceAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, object: Param0, uniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), uniqueid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn NotifyDeviceRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, object: Param0, uniqueid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), object.into_param().abi(), uniqueid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DDiscMaster2Events {
    type Vtable = DDiscMaster2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801521, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DDiscMaster2Events> for ::windows::runtime::IUnknown {
    fn from(value: DDiscMaster2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DDiscMaster2Events> for ::windows::runtime::IUnknown {
    fn from(value: &DDiscMaster2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DDiscMaster2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DDiscMaster2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscMaster2Events> for super::super::System::Com::IDispatch {
    fn from(value: DDiscMaster2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscMaster2Events> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscMaster2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DDiscMaster2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DDiscMaster2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DFileSystemImageEvents(pub ::windows::runtime::IUnknown);
impl DFileSystemImageEvents {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, object: Param0, currentfile: Param1, copiedsectors: i32, totalsectors: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), currentfile.into_param().abi(), ::core::mem::transmute(copiedsectors), ::core::mem::transmute(totalsectors)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DFileSystemImageEvents {
    type Vtable = DFileSystemImageEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904991, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<DFileSystemImageEvents> for ::windows::runtime::IUnknown {
    fn from(value: DFileSystemImageEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DFileSystemImageEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DFileSystemImageEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DFileSystemImageEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DFileSystemImageEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageEvents> for super::super::System::Com::IDispatch {
    fn from(value: DFileSystemImageEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DFileSystemImageEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DFileSystemImageEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DFileSystemImageEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DFileSystemImageImportEvents(pub ::windows::runtime::IUnknown);
impl DFileSystemImageImportEvents {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn UpdateImport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, object: Param0, filesystem: FsiFileSystems, currentitem: Param2, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            object.into_param().abi(),
            ::core::mem::transmute(filesystem),
            currentitem.into_param().abi(),
            ::core::mem::transmute(importeddirectoryitems),
            ::core::mem::transmute(totaldirectoryitems),
            ::core::mem::transmute(importedfileitems),
            ::core::mem::transmute(totalfileitems),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for DFileSystemImageImportEvents {
    type Vtable = DFileSystemImageImportEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3529257209, 16519, 17254, [158, 36, 229, 91, 226, 134, 66, 75]);
}
impl ::core::convert::From<DFileSystemImageImportEvents> for ::windows::runtime::IUnknown {
    fn from(value: DFileSystemImageImportEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DFileSystemImageImportEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DFileSystemImageImportEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DFileSystemImageImportEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DFileSystemImageImportEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageImportEvents> for super::super::System::Com::IDispatch {
    fn from(value: DFileSystemImageImportEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageImportEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DFileSystemImageImportEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DFileSystemImageImportEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DFileSystemImageImportEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
impl ::core::convert::From<u32> for DISC_RECORDER_STATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISC_RECORDER_STATE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for DISC_RECORDER_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DISC_RECORDER_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DISC_RECORDER_STATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DISC_RECORDER_STATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DISC_RECORDER_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DWriteEngine2Events(pub ::windows::runtime::IUnknown);
impl DWriteEngine2Events {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, object: Param0, progress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), object.into_param().abi(), progress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for DWriteEngine2Events {
    type Vtable = DWriteEngine2Events_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801527, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<DWriteEngine2Events> for ::windows::runtime::IUnknown {
    fn from(value: DWriteEngine2Events) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DWriteEngine2Events> for ::windows::runtime::IUnknown {
    fn from(value: &DWriteEngine2Events) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DWriteEngine2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DWriteEngine2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DWriteEngine2Events> for super::super::System::Com::IDispatch {
    fn from(value: DWriteEngine2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DWriteEngine2Events> for super::super::System::Com::IDispatch {
    fn from(value: &DWriteEngine2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for DWriteEngine2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &DWriteEngine2Events {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, progress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EmulationType(pub i32);
pub const EmulationNone: EmulationType = EmulationType(0i32);
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
impl ::core::convert::From<i32> for EmulationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EmulationType {
    type Abi = Self;
}
pub const EnumFsiItems: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904966, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const EnumProgressItems: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904970, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const FileSystemImageResult: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904972, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const FsiDirectoryItem: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904968, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const FsiFileItem: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904967, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsiFileSystems(pub i32);
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
impl ::core::convert::From<i32> for FsiFileSystems {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsiFileSystems {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsiItemType(pub i32);
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
impl ::core::convert::From<i32> for FsiItemType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsiItemType {
    type Abi = Self;
}
pub const FsiNamedStreams: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3333880045, 27929, 17588, [181, 57, 177, 89, 183, 147, 163, 45]);
pub const FsiStream: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904973, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const GUID_SMTPSVC_SOURCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(456918630, 58480, 4561, [170, 103, 0, 192, 79, 163, 69, 246]);
pub const GUID_SMTP_SOURCE_TYPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4217750748, 58472, 4561, [170, 103, 0, 192, 79, 163, 69, 246]);
#[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows::runtime::HRESULT;
        }
        GetAttribIMsgOnIStg(::core::mem::transmute(lpobject), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lpppropattrarray)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBlockRange(pub ::windows::runtime::IUnknown);
impl IBlockRange {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EndLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IBlockRange {
    type Vtable = IBlockRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186597, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IBlockRange> for ::windows::runtime::IUnknown {
    fn from(value: IBlockRange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBlockRange> for ::windows::runtime::IUnknown {
    fn from(value: &IBlockRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBlockRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBlockRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRange> for super::super::System::Com::IDispatch {
    fn from(value: IBlockRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRange> for super::super::System::Com::IDispatch {
    fn from(value: &IBlockRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IBlockRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IBlockRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBlockRangeList(pub ::windows::runtime::IUnknown);
impl IBlockRangeList {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn BlockRanges(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IBlockRangeList {
    type Vtable = IBlockRangeList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186598, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IBlockRangeList> for ::windows::runtime::IUnknown {
    fn from(value: IBlockRangeList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBlockRangeList> for ::windows::runtime::IUnknown {
    fn from(value: &IBlockRangeList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBlockRangeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBlockRangeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRangeList> for super::super::System::Com::IDispatch {
    fn from(value: IBlockRangeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRangeList> for super::super::System::Com::IDispatch {
    fn from(value: &IBlockRangeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IBlockRangeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IBlockRangeList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBootOptions(pub ::windows::runtime::IUnknown);
impl IBootOptions {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn BootImage(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Manufacturer(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetManufacturer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn PlatformId(&self) -> ::windows::runtime::Result<PlatformId> {
        let mut result__: <PlatformId as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PlatformId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Emulation(&self) -> ::windows::runtime::Result<EmulationType> {
        let mut result__: <EmulationType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<EmulationType>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImageSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn AssignBootImage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBootOptions {
    type Vtable = IBootOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904980, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IBootOptions> for ::windows::runtime::IUnknown {
    fn from(value: IBootOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBootOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IBootOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBootOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBootOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootOptions> for super::super::System::Com::IDispatch {
    fn from(value: IBootOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootOptions> for super::super::System::Com::IDispatch {
    fn from(value: &IBootOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IBootOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IBootOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut PlatformId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: PlatformId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut EmulationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: EmulationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBurnVerification(pub ::windows::runtime::IUnknown);
impl IBurnVerification {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BurnVerificationLevel(&self) -> ::windows::runtime::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        let mut result__: <IMAPI_BURN_VERIFICATION_LEVEL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_BURN_VERIFICATION_LEVEL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IBurnVerification {
    type Vtable = IBurnVerification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3539982388, 38283, 17005, [132, 112, 42, 19, 135, 156, 106, 145]);
}
impl ::core::convert::From<IBurnVerification> for ::windows::runtime::IUnknown {
    fn from(value: IBurnVerification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBurnVerification> for ::windows::runtime::IUnknown {
    fn from(value: &IBurnVerification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBurnVerification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBurnVerification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2(pub ::windows::runtime::IUnknown);
impl IDiscFormat2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRecorderSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsCurrentMediaSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2 {
    type Vtable = IDiscFormat2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801554, 36708, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2Data(pub ::windows::runtime::IUnknown);
impl IDiscFormat2Data {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRecorderSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsCurrentMediaSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Recorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetPostgapAlreadyInImage(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn PostgapAlreadyInImage(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentMediaStatus(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        let mut result__: <IMAPI_FORMAT2_DATA_MEDIA_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_DATA_MEDIA_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn WriteProtectStatus(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        let mut result__: <IMAPI_MEDIA_WRITE_PROTECT_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_WRITE_PROTECT_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NextWritableAddress(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetForceMediaToBeClosed(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ForceMediaToBeClosed(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__: <IMAPI_MEDIA_PHYSICAL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetClientName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ClientName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetForceOverwrite(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ForceOverwrite(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Write<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CancelWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedsectorspersecond), ::core::mem::transmute(rotationtypeispurecav)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2Data {
    type Vtable = IDiscFormat2Data_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801555, 40804, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2Data> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2Data) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2Data> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2Data) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2Data> for IDiscFormat2 {
    fn from(value: IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2Data> for IDiscFormat2 {
    fn from(value: &IDiscFormat2Data) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for &IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Data> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Data> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2Data) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2Data {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2DataEventArgs(pub ::windows::runtime::IUnknown);
impl IDiscFormat2DataEventArgs {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastReadLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ElapsedTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RemainingTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentAction(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        let mut result__: <IMAPI_FORMAT2_DATA_WRITE_ACTION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_DATA_WRITE_ACTION>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2DataEventArgs {
    type Vtable = IDiscFormat2DataEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801533, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2DataEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2DataEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for &IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2DataEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2DataEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2Erase(pub ::windows::runtime::IUnknown);
impl IDiscFormat2Erase {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRecorderSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsCurrentMediaSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Recorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFullErase(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FullErase(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__: <IMAPI_MEDIA_PHYSICAL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetClientName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ClientName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EraseMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2Erase {
    type Vtable = IDiscFormat2Erase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801558, 36708, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2Erase> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2Erase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2Erase> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2Erase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2Erase> for IDiscFormat2 {
    fn from(value: IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2Erase> for IDiscFormat2 {
    fn from(value: &IDiscFormat2Erase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for &IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Erase> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Erase> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2Erase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2Erase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Erase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2RawCD(pub ::windows::runtime::IUnknown);
impl IDiscFormat2RawCD {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRecorderSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsCurrentMediaSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn PrepareMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn WriteMedia<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn WriteMedia2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0, streamleadinsectors: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), data.into_param().abi(), ::core::mem::transmute(streamleadinsectors)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CancelWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ReleaseMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedsectorspersecond), ::core::mem::transmute(rotationtypeispurecav)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Recorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartOfNextSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__: <IMAPI_MEDIA_PHYSICAL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedSectorTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedSectorType(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__: <IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetClientName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ClientName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2RawCD {
    type Vtable = IDiscFormat2RawCD_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801557, 36708, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2RawCD> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2RawCD) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2RawCD> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2RawCD> for IDiscFormat2 {
    fn from(value: IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2RawCD> for IDiscFormat2 {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for &IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCD> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCD> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2RawCD {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCD_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, streamleadinsectors: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2RawCDEventArgs(pub ::windows::runtime::IUnknown);
impl IDiscFormat2RawCDEventArgs {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastReadLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentAction(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        let mut result__: <IMAPI_FORMAT2_RAW_CD_WRITE_ACTION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ElapsedTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RemainingTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2RawCDEventArgs {
    type Vtable = IDiscFormat2RawCDEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801539, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for &IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2RawCDEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2TrackAtOnce(pub ::windows::runtime::IUnknown);
impl IDiscFormat2TrackAtOnce {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRecorderSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsCurrentMediaSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, recorder: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), recorder.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn PrepareMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn AddAudioTrack<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CancelAddTrack(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ReleaseMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedsectorspersecond), ::core::mem::transmute(rotationtypeispurecav)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Recorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetDoNotFinalizeMedia(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DoNotFinalizeMedia(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__: <IMAPI_MEDIA_PHYSICAL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetClientName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ClientName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2TrackAtOnce {
    type Vtable = IDiscFormat2TrackAtOnce_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801556, 36708, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for IDiscFormat2 {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for IDiscFormat2 {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDiscFormat2> for &IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDiscFormat2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2TrackAtOnce {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnce_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorder: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscFormat2TrackAtOnceEventArgs(pub ::windows::runtime::IUnknown);
impl IDiscFormat2TrackAtOnceEventArgs {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastReadLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentTrackNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CurrentAction(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        let mut result__: <IMAPI_FORMAT2_TAO_WRITE_ACTION as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_TAO_WRITE_ACTION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ElapsedTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RemainingTime(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscFormat2TrackAtOnceEventArgs {
    type Vtable = IDiscFormat2TrackAtOnceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801536, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWriteEngine2EventArgs> for &IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWriteEngine2EventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscFormat2TrackAtOnceEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscMaster(pub ::windows::runtime::IUnknown);
impl IDiscMaster {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Open(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumDiscMasterFormats(&self) -> ::windows::runtime::Result<IEnumDiscMasterFormats> {
        let mut result__: <IEnumDiscMasterFormats as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDiscMasterFormats>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetActiveDiscMasterFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetActiveDiscMasterFormat(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumDiscRecorders(&self) -> ::windows::runtime::Result<IEnumDiscRecorders> {
        let mut result__: <IEnumDiscRecorders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDiscRecorders>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetActiveDiscRecorder(&self) -> ::windows::runtime::Result<IDiscRecorder> {
        let mut result__: <IDiscRecorder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetActiveDiscRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder>>(&self, precorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), precorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ClearFormatContent(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProgressAdvise<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscMasterProgressEvents>>(&self, pevents: Param0) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pevents.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProgressUnadvise(&self, vcookie: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcookie)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsimulate), ::core::mem::transmute(bejectafterburn)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiscMaster {
    type Vtable = IDiscMaster_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376569954, 20901, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: IDiscMaster) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscMaster) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpiid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprecorder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pevents: ::windows::runtime::RawPtr, pvcookie: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcookie: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsimulate: u8, bejectafterburn: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscMaster2(pub ::windows::runtime::IUnknown);
impl IDiscMaster2 {
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsSupportedEnvironment(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscMaster2 {
    type Vtable = IDiscMaster2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801520, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscMaster2> for ::windows::runtime::IUnknown {
    fn from(value: IDiscMaster2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscMaster2> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscMaster2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscMaster2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscMaster2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscMaster2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscMaster2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscMaster2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscMaster2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscMaster2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscMaster2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscMasterProgressEvents(pub ::windows::runtime::IUnknown);
impl IDiscMasterProgressEvents {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn QueryCancel(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyPnPActivity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncompletedsteps), ::core::mem::transmute(ntotalsteps)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncompleted), ::core::mem::transmute(ntotal)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ncurrenttrack), ::core::mem::transmute(ntotaltracks)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(nestimatedseconds)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(nestimatedseconds)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyBurnComplete(&self, status: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NotifyEraseComplete(&self, status: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiscMasterProgressEvents {
    type Vtable = IDiscMasterProgressEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3969798593, 20061, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IDiscMasterProgressEvents> for ::windows::runtime::IUnknown {
    fn from(value: IDiscMasterProgressEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscMasterProgressEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscMasterProgressEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscMasterProgressEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscMasterProgressEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMasterProgressEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcancel: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncompleted: i32, ntotal: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nestimatedseconds: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nestimatedseconds: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscRecorder(pub ::windows::runtime::IUnknown);
impl IDiscRecorder {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Init(&self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbyuniqueid), ::core::mem::transmute(nulidsize), ::core::mem::transmute(nuldrivenumber)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetRecorderGUID(&self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbyuniqueid), ::core::mem::transmute(ulbuffersize), ::core::mem::transmute(pulreturnsizerequired)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetRecorderType(&self) -> ::windows::runtime::Result<RECORDER_TYPES> {
        let mut result__: <RECORDER_TYPES as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RECORDER_TYPES>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayNames(&self, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrvendorid), ::core::mem::transmute(pbstrproductid), ::core::mem::transmute(pbstrrevision)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetBasePnPID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetRecorderProperties(&self) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__: <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::IPropertyStorage>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetRecorderProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IPropertyStorage>>(&self, ppropstg: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ppropstg.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetRecorderState(&self) -> ::windows::runtime::Result<DISC_RECORDER_STATE_FLAGS> {
        let mut result__: <DISC_RECORDER_STATE_FLAGS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DISC_RECORDER_STATE_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn OpenExclusive(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(fmediatype), ::core::mem::transmute(fmediaflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsessions), ::core::mem::transmute(pblasttrack), ::core::mem::transmute(ulstartaddress), ::core::mem::transmute(ulnextwritable), ::core::mem::transmute(ulfreeblocks)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Eject(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Erase(&self, bfullerase: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bfullerase)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDiscRecorder {
    type Vtable = IDiscRecorder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2242680694, 51848, 19698, [137, 78, 9, 89, 140, 7, 138, 65]);
}
impl ::core::convert::From<IDiscRecorder> for ::windows::runtime::IUnknown {
    fn from(value: IDiscRecorder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscRecorder> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscRecorder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ftypecode: *mut RECORDER_TYPES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrvendorid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrproductid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrrevision: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbasepnpid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bfullerase: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscRecorder2(pub ::windows::runtime::IUnknown);
impl IDiscRecorder2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EjectMedia(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CloseTray(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AcquireExclusiveAccess<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, force: i16, __midl__idiscrecorder20000: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(force), __midl__idiscrecorder20000.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ReleaseExclusiveAccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DisableMcn(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnableMcn(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn InitializeDiscRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, recorderuniqueid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), recorderuniqueid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ActiveDiscRecorder(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VendorId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ProductId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ProductRevision(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn VolumePathNames(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DeviceCanLoadMedia(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LegacyDeviceNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedFeaturePages(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CurrentFeaturePages(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedProfiles(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CurrentProfiles(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SupportedModePages(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ExclusiveAccessOwner(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscRecorder2 {
    type Vtable = IDiscRecorder2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801523, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscRecorder2> for ::windows::runtime::IUnknown {
    fn from(value: IDiscRecorder2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscRecorder2> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscRecorder2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscRecorder2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscRecorder2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscRecorder2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscRecorder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscRecorder2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscRecorder2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IDiscRecorder2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IDiscRecorder2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, legacydevicenumber: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDiscRecorder2Ex(pub ::windows::runtime::IUnknown);
impl IDiscRecorder2Ex {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cdb), ::core::mem::transmute(cdbsize), ::core::mem::transmute(sensebuffer), ::core::mem::transmute(timeout)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cdb), ::core::mem::transmute(cdbsize), ::core::mem::transmute(sensebuffer), ::core::mem::transmute(timeout), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cdb), ::core::mem::transmute(cdbsize), ::core::mem::transmute(sensebuffer), ::core::mem::transmute(timeout), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(bufferfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(address), ::core::mem::transmute(layer), ::core::mem::transmute(agid), ::core::mem::transmute(data), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SendDvdStructure(&self, format: u32, data: *const u8, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(data), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(discinformation), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(address), ::core::mem::transmute(addresstype), ::core::mem::transmute(trackinformation), ::core::mem::transmute(bytesize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetFeaturePage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: Param1, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedfeature), currentfeatureonly.into_param().abi(), ::core::mem::transmute(featuredata), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedmodepage), ::core::mem::transmute(requesttype), ::core::mem::transmute(modepagedata), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(requesttype), ::core::mem::transmute(data), ::core::mem::transmute(bytesize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedFeaturePages<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, currentfeatureonly: Param0, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), currentfeatureonly.into_param().abi(), ::core::mem::transmute(featuredata), ::core::mem::transmute(bytesize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedProfiles<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, currentonly: Param0, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), currentonly.into_param().abi(), ::core::mem::transmute(profiletypes), ::core::mem::transmute(validprofiles)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(requesttype), ::core::mem::transmute(modepagetypes), ::core::mem::transmute(validpages)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetByteAlignmentMask(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDiscRecorder2Ex {
    type Vtable = IDiscRecorder2Ex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801522, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IDiscRecorder2Ex> for ::windows::runtime::IUnknown {
    fn from(value: IDiscRecorder2Ex) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDiscRecorder2Ex> for ::windows::runtime::IUnknown {
    fn from(value: &IDiscRecorder2Ex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDiscRecorder2Ex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDiscRecorder2Ex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2Ex_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: u32, data: *const u8, count: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumDiscMasterFormats(pub ::windows::runtime::IUnknown);
impl IEnumDiscMasterFormats {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Next(&self, cformats: u32, lpiidformatid: *mut ::windows::runtime::GUID, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats), ::core::mem::transmute(lpiidformatid), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Skip(&self, cformats: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cformats)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumDiscMasterFormats> {
        let mut result__: <IEnumDiscMasterFormats as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDiscMasterFormats>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumDiscMasterFormats {
    type Vtable = IEnumDiscMasterFormats_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723773409, 21690, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IEnumDiscMasterFormats> for ::windows::runtime::IUnknown {
    fn from(value: IEnumDiscMasterFormats) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumDiscMasterFormats> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumDiscMasterFormats) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumDiscMasterFormats {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumDiscMasterFormats {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscMasterFormats_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cformats: u32, lpiidformatid: *mut ::windows::runtime::GUID, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cformats: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumDiscRecorders(pub ::windows::runtime::IUnknown);
impl IEnumDiscRecorders {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Next(&self, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crecorders), ::core::mem::transmute(pprecorder), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Skip(&self, crecorders: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(crecorders)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumDiscRecorders> {
        let mut result__: <IEnumDiscRecorders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDiscRecorders>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumDiscRecorders {
    type Vtable = IEnumDiscRecorders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602115553, 21676, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IEnumDiscRecorders> for ::windows::runtime::IUnknown {
    fn from(value: IEnumDiscRecorders) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumDiscRecorders> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumDiscRecorders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumDiscRecorders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumDiscRecorders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscRecorders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crecorders: u32, pprecorder: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crecorders: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumFsiItems(pub ::windows::runtime::IUnknown);
impl IEnumFsiItems {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IFsiItem>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumFsiItems> {
        let mut result__: <IEnumFsiItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFsiItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumFsiItems {
    type Vtable = IEnumFsiItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904986, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IEnumFsiItems> for ::windows::runtime::IUnknown {
    fn from(value: IEnumFsiItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumFsiItems> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumFsiItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumFsiItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumFsiItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumProgressItems(pub ::windows::runtime::IUnknown);
impl IEnumProgressItems {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IProgressItem>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumProgressItems> {
        let mut result__: <IEnumProgressItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumProgressItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumProgressItems {
    type Vtable = IEnumProgressItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904982, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IEnumProgressItems> for ::windows::runtime::IUnknown {
    fn from(value: IEnumProgressItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumProgressItems> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumProgressItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFileSystemImage(pub ::windows::runtime::IUnknown);
impl IFileSystemImage {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Root(&self) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SessionStartBlock(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetVolumeName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BootImageOptions(&self) -> ::windows::runtime::Result<IBootOptions> {
        let mut result__: <IBootOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBootImageOptions<'a, Param0: ::windows::runtime::IntoParam<'a, IBootOptions>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DirectoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChangePoint(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UDFRevision(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaults<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreateResultImage(&self) -> ::windows::runtime::Result<IFileSystemImageResult> {
        let mut result__: <IFileSystemImageResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFileSystemImageResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Exists<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fullpath: Param0) -> ::windows::runtime::Result<FsiItemType> {
        let mut result__: <FsiItemType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), fullpath.into_param().abi(), &mut result__).from_abi::<FsiItemType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), discrecorder.into_param().abi(), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystems), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportFileSystem(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystemtouse)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(changepoint)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockInChangePoint(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateDirectoryItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateFileItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiFileItem> {
        let mut result__: <IFsiFileItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiFileItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StageFiles(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFileSystemImage {
    type Vtable = IFileSystemImage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904993, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IFileSystemImage> for ::windows::runtime::IUnknown {
    fn from(value: IFileSystemImage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSystemImage> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSystemImage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSystemImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSystemImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFileSystemImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFileSystemImage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resultstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discidentifier: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, importedfilesystem: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystemtouse: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changepoint: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFileSystemImage2(pub ::windows::runtime::IUnknown);
impl IFileSystemImage2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Root(&self) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SessionStartBlock(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetVolumeName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BootImageOptions(&self) -> ::windows::runtime::Result<IBootOptions> {
        let mut result__: <IBootOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBootImageOptions<'a, Param0: ::windows::runtime::IntoParam<'a, IBootOptions>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DirectoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChangePoint(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UDFRevision(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaults<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreateResultImage(&self) -> ::windows::runtime::Result<IFileSystemImageResult> {
        let mut result__: <IFileSystemImageResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFileSystemImageResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Exists<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fullpath: Param0) -> ::windows::runtime::Result<FsiItemType> {
        let mut result__: <FsiItemType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), fullpath.into_param().abi(), &mut result__).from_abi::<FsiItemType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), discrecorder.into_param().abi(), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystems), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportFileSystem(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystemtouse)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(changepoint)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockInChangePoint(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateDirectoryItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateFileItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiFileItem> {
        let mut result__: <IFsiFileItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiFileItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StageFiles(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFileSystemImage2 {
    type Vtable = IFileSystemImage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3613674284, 5431, 18279, [182, 47, 241, 56, 123, 2, 221, 253]);
}
impl ::core::convert::From<IFileSystemImage2> for ::windows::runtime::IUnknown {
    fn from(value: IFileSystemImage2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSystemImage2> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSystemImage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFileSystemImage2> for IFileSystemImage {
    fn from(value: IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSystemImage2> for IFileSystemImage {
    fn from(value: &IFileSystemImage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage> for IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage> for &IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage2> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage2> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFileSystemImage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resultstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discidentifier: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, importedfilesystem: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystemtouse: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changepoint: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFileSystemImage3(pub ::windows::runtime::IUnknown);
impl IFileSystemImage3 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Root(&self) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SessionStartBlock(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetVolumeName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BootImageOptions(&self) -> ::windows::runtime::Result<IBootOptions> {
        let mut result__: <IBootOptions as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBootImageOptions<'a, Param0: ::windows::runtime::IntoParam<'a, IBootOptions>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DirectoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChangePoint(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UDFRevision(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaults<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), discrecorder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreateResultImage(&self) -> ::windows::runtime::Result<IFileSystemImageResult> {
        let mut result__: <IFileSystemImageResult as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFileSystemImageResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Exists<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fullpath: Param0) -> ::windows::runtime::Result<FsiItemType> {
        let mut result__: <FsiItemType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), fullpath.into_param().abi(), &mut result__).from_abi::<FsiItemType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2>>(&self, discrecorder: Param0) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), discrecorder.into_param().abi(), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystems), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportFileSystem(&self) -> ::windows::runtime::Result<FsiFileSystems> {
        let mut result__: <FsiFileSystems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FsiFileSystems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystemtouse)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(changepoint)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockInChangePoint(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateDirectoryItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiDirectoryItem> {
        let mut result__: <IFsiDirectoryItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn CreateFileItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsiFileItem> {
        let mut result__: <IFsiFileItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsiFileItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StageFiles(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreateRedundantUdfMetadataFiles(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystemtoprobe), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFileSystemImage3 {
    type Vtable = IFileSystemImage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2097120300, 32407, 18439, [131, 4, 145, 13, 216, 247, 192, 81]);
}
impl ::core::convert::From<IFileSystemImage3> for ::windows::runtime::IUnknown {
    fn from(value: IFileSystemImage3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSystemImage3> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSystemImage3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFileSystemImage3> for IFileSystemImage2 {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSystemImage3> for IFileSystemImage2 {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage2> for IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage2> for &IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSystemImage3> for IFileSystemImage {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSystemImage3> for IFileSystemImage {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage> for IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImage> for &IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage3> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage3> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFileSystemImage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resultstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discidentifier: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, discrecorder: ::windows::runtime::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, importedfilesystem: *mut FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystemtouse: FsiFileSystems) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changepoint: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFileSystemImageResult(pub ::windows::runtime::IUnknown);
impl IFileSystemImageResult {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ImageStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProgressItems(&self) -> ::windows::runtime::Result<IProgressItems> {
        let mut result__: <IProgressItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IProgressItems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BlockSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn DiscId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFileSystemImageResult {
    type Vtable = IFileSystemImageResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904984, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IFileSystemImageResult> for ::windows::runtime::IUnknown {
    fn from(value: IFileSystemImageResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSystemImageResult> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSystemImageResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSystemImageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSystemImageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFileSystemImageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFileSystemImageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFileSystemImageResult2(pub ::windows::runtime::IUnknown);
impl IFileSystemImageResult2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ImageStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProgressItems(&self) -> ::windows::runtime::Result<IProgressItems> {
        let mut result__: <IProgressItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IProgressItems>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BlockSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn DiscId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ModifiedBlocks(&self) -> ::windows::runtime::Result<IBlockRangeList> {
        let mut result__: <IBlockRangeList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBlockRangeList>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFileSystemImageResult2 {
    type Vtable = IFileSystemImageResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186601, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IFileSystemImageResult2> for ::windows::runtime::IUnknown {
    fn from(value: IFileSystemImageResult2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSystemImageResult2> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSystemImageResult2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFileSystemImageResult2> for IFileSystemImageResult {
    fn from(value: IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSystemImageResult2> for IFileSystemImageResult {
    fn from(value: &IFileSystemImageResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImageResult> for IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImageResult> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSystemImageResult> for &IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSystemImageResult> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult2> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult2> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImageResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFileSystemImageResult2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiDirectoryItem(pub ::windows::runtime::IUnknown);
impl IFsiDirectoryItem {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FullPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastAccessedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsHidden(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsiItem> {
        let mut result__: <IFsiItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsiItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumFsiItems(&self) -> ::windows::runtime::Result<IEnumFsiItems> {
        let mut result__: <IEnumFsiItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFsiItems>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AddDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, path: Param0, filedata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), path.into_param().abi(), filedata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AddTree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sourcedirectory: Param0, includebasedirectory: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), sourcedirectory.into_param().abi(), ::core::mem::transmute(includebasedirectory)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IFsiItem>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn RemoveTree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsiDirectoryItem {
    type Vtable = IFsiDirectoryItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904988, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IFsiDirectoryItem> for ::windows::runtime::IUnknown {
    fn from(value: IFsiDirectoryItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiDirectoryItem> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiDirectoryItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFsiDirectoryItem> for IFsiItem {
    fn from(value: IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiDirectoryItem> for IFsiItem {
    fn from(value: &IFsiDirectoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for &IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiDirectoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiDirectoryItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiDirectoryItem2(pub ::windows::runtime::IUnknown);
impl IFsiDirectoryItem2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FullPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastAccessedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsHidden(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsiItem> {
        let mut result__: <IFsiItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsiItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumFsiItems(&self) -> ::windows::runtime::Result<IEnumFsiItems> {
        let mut result__: <IEnumFsiItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFsiItems>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AddDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, path: Param0, filedata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), path.into_param().abi(), filedata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AddTree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sourcedirectory: Param0, includebasedirectory: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), sourcedirectory.into_param().abi(), ::core::mem::transmute(includebasedirectory)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IFsiItem>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn RemoveTree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn AddTreeWithNamedStreams<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sourcedirectory: Param0, includebasedirectory: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), sourcedirectory.into_param().abi(), ::core::mem::transmute(includebasedirectory)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsiDirectoryItem2 {
    type Vtable = IFsiDirectoryItem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4160441243, 28054, 19835, [145, 21, 32, 27, 20, 72, 17, 239]);
}
impl ::core::convert::From<IFsiDirectoryItem2> for ::windows::runtime::IUnknown {
    fn from(value: IFsiDirectoryItem2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiDirectoryItem2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFsiDirectoryItem2> for IFsiDirectoryItem {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiDirectoryItem2> for IFsiDirectoryItem {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiDirectoryItem> for IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiDirectoryItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiDirectoryItem> for &IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiDirectoryItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsiDirectoryItem2> for IFsiItem {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiDirectoryItem2> for IFsiItem {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for &IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem2> for super::super::System::Com::IDispatch {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiDirectoryItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiFileItem(pub ::windows::runtime::IUnknown);
impl IFsiFileItem {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FullPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastAccessedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsHidden(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize32BitLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Data(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsiFileItem {
    type Vtable = IFsiFileItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904987, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IFsiFileItem> for ::windows::runtime::IUnknown {
    fn from(value: IFsiFileItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiFileItem> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiFileItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFsiFileItem> for IFsiItem {
    fn from(value: IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiFileItem> for IFsiItem {
    fn from(value: &IFsiFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for &IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiFileItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiFileItem2(pub ::windows::runtime::IUnknown);
impl IFsiFileItem2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FullPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastAccessedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsHidden(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize32BitLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Data(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FsiNamedStreams(&self) -> ::windows::runtime::Result<IFsiNamedStreams> {
        let mut result__: <IFsiNamedStreams as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFsiNamedStreams>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsNamedStream(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, name: Param0, streamdata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), name.into_param().abi(), streamdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsRealTime(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsiFileItem2 {
    type Vtable = IFsiFileItem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(429722649, 4577, 16619, [142, 194, 200, 200, 34, 160, 119, 146]);
}
impl ::core::convert::From<IFsiFileItem2> for ::windows::runtime::IUnknown {
    fn from(value: IFsiFileItem2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiFileItem2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiFileItem2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IFsiFileItem2> for IFsiFileItem {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiFileItem2> for IFsiFileItem {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiFileItem> for IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiFileItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiFileItem> for &IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiFileItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsiFileItem2> for IFsiItem {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsiFileItem2> for IFsiItem {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsiItem> for &IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsiItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem2> for super::super::System::Com::IDispatch {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiFileItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiItem(pub ::windows::runtime::IUnknown);
impl IFsiItem {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FullPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastAccessedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastModifiedTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsHidden(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesystem), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsiItem {
    type Vtable = IFsiItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904985, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IFsiItem> for ::windows::runtime::IUnknown {
    fn from(value: IFsiItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiItem> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesystem: FsiFileSystems, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFsiNamedStreams(pub ::windows::runtime::IUnknown);
impl IFsiNamedStreams {
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IFsiFileItem2> {
        let mut result__: <IFsiFileItem2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IFsiFileItem2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumNamedStreams(&self) -> ::windows::runtime::Result<IEnumFsiItems> {
        let mut result__: <IEnumFsiItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFsiItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsiNamedStreams {
    type Vtable = IFsiNamedStreams_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3984177750, 21140, 16976, [141, 70, 249, 174, 206, 226, 52, 89]);
}
impl ::core::convert::From<IFsiNamedStreams> for ::windows::runtime::IUnknown {
    fn from(value: IFsiNamedStreams) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFsiNamedStreams> for ::windows::runtime::IUnknown {
    fn from(value: &IFsiNamedStreams) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsiNamedStreams {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFsiNamedStreams {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiNamedStreams> for super::super::System::Com::IDispatch {
    fn from(value: IFsiNamedStreams) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiNamedStreams> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiNamedStreams) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IFsiNamedStreams {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsiNamedStreams {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIsoImageManager(pub ::windows::runtime::IUnknown);
impl IIsoImageManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Stream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, val: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn SetStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Validate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIsoImageManager {
    type Vtable = IIsoImageManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1822657509, 64443, 18432, [149, 161, 164, 56, 134, 94, 176, 212]);
}
impl ::core::convert::From<IIsoImageManager> for ::windows::runtime::IUnknown {
    fn from(value: IIsoImageManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIsoImageManager> for ::windows::runtime::IUnknown {
    fn from(value: &IIsoImageManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIsoImageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IIsoImageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIsoImageManager> for super::super::System::Com::IDispatch {
    fn from(value: IIsoImageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIsoImageManager> for super::super::System::Com::IDispatch {
    fn from(value: &IIsoImageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IIsoImageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IIsoImageManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IJolietDiscMaster(pub ::windows::runtime::IUnknown);
impl IJolietDiscMaster {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTotalDataBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetUsedDataBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetDataBlockSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn AddData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>>(&self, pstorage: Param0, lfileoverwrite: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstorage.into_param().abi(), ::core::mem::transmute(lfileoverwrite)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetJolietProperties(&self) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__: <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::IPropertyStorage>(result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetJolietProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IPropertyStorage>>(&self, ppropstg: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ppropstg.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IJolietDiscMaster {
    type Vtable = IJolietDiscMaster_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820765902, 20060, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IJolietDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: IJolietDiscMaster) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IJolietDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: &IJolietDiscMaster) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IJolietDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IJolietDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJolietDiscMaster_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblocks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblocks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblockbytes: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstorage: ::windows::runtime::RawPtr, lfileoverwrite: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPILib2_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPILib2_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
impl ::core::convert::From<i32> for IMAPI_BURN_VERIFICATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_BURN_VERIFICATION_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_CD_SECTOR_TYPE(pub i32);
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(0i32);
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(1i32);
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(2i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(3i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(4i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(5i32);
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(6i32);
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(7i32);
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(8i32);
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(9i32);
impl ::core::convert::From<i32> for IMAPI_CD_SECTOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_CD_SECTOR_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
impl ::core::convert::From<i32> for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_ALREADYOPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220958i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_BADJOLIETNAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220963i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220946i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220948i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_COMPRESSEDSTASH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220952i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220972i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220975i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220974i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220973i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220954i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DISCFULL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220964i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_DISCINFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220967i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220951i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILEACCESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220968i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILEEXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220956i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_FILESYSTEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220969i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_GENERIC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220978i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220970i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INITIALIZE_WRITE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220971i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_INVALIDIMAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220962i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220953i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220976i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220977i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOACTIVEFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220961i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOACTIVERECORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220960i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220950i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTINITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220980i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_NOTOPENED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220981i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_REMOVABLESTASH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220949i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_STASHINUSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220955i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACKNOTOPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220966i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACKOPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220965i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220947i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_USERABORT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220979i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_WRONGDISC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220957i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_E_WRONGFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220959i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FEATURE_PAGE_TYPE(pub i32);
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(0i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(1i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(2i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(3i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(4i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(16i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(29i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(30i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(31i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(32i32);
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(33i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(34i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(35i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(36i32);
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(37i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(38i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(39i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(40i32);
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(41i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(42i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(43i32);
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(44i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(45i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(46i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(47i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(48i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(49i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(50i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(51i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(55i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(56i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(59i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(64i32);
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(65i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(80i32);
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(81i32);
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(256i32);
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(257i32);
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(258i32);
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(259i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(260i32);
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(261i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(262i32);
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(263i32);
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(264i32);
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(265i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(266i32);
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(267i32);
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(268i32);
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(269i32);
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(272i32);
impl ::core::convert::From<i32> for IMAPI_FEATURE_PAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FEATURE_PAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(pub i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(0i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(15i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(64512i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1024i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2048i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4096i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8192i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(16384i32);
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(32768i32);
impl ::core::convert::From<i32> for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(4i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(5i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(6i32);
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(7i32);
impl ::core::convert::From<i32> for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
impl ::core::convert::From<i32> for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
impl ::core::convert::From<i32> for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
impl ::core::convert::From<i32> for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(pub i32);
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(0i32);
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(1i32);
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(2i32);
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(3i32);
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(4i32);
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(5i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(6i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(7i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(8i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(9i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(10i32);
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(11i32);
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(12i32);
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(13i32);
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(14i32);
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(15i32);
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(16i32);
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(17i32);
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(18i32);
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
impl ::core::convert::From<i32> for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_MEDIA_PHYSICAL_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
impl ::core::convert::From<i32> for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
impl ::core::convert::From<i32> for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_MODE_PAGE_REQUEST_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_MODE_PAGE_TYPE(pub i32);
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(1i32);
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(3i32);
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(5i32);
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(8i32);
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(28i32);
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(29i32);
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(26i32);
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(42i32);
impl ::core::convert::From<i32> for IMAPI_MODE_PAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_MODE_PAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_PROFILE_TYPE(pub i32);
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(0i32);
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(1i32);
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(2i32);
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(3i32);
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(4i32);
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(5i32);
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(8i32);
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(9i32);
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(10i32);
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(16i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(17i32);
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(18i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(19i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(20i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(21i32);
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(22i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(26i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(27i32);
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(32i32);
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(33i32);
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(34i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(42i32);
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(43i32);
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(64i32);
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65i32);
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(66i32);
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(67i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(80i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(81i32);
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(82i32);
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65535i32);
impl ::core::convert::From<i32> for IMAPI_PROFILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_PROFILE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
impl ::core::convert::From<i32> for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAPI_READ_TRACK_ADDRESS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262657i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const IMAPI_S_PROPERTIESIGNORED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262656i32 as _);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_CPV_ENUM(pub i32);
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
impl ::core::convert::From<i32> for IMMPID_CPV_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_CPV_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_MPV_ENUM(pub i32);
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12287i32);
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12288i32);
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12289i32);
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12290i32);
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12291i32);
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12292i32);
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12293i32);
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12294i32);
impl ::core::convert::From<i32> for IMMPID_MPV_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_MPV_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_MP_ENUM(pub i32);
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4095i32);
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4096i32);
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4097i32);
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4098i32);
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4099i32);
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4100i32);
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4101i32);
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4102i32);
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4103i32);
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4104i32);
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4105i32);
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4106i32);
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4107i32);
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4108i32);
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4109i32);
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4110i32);
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4111i32);
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4112i32);
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4113i32);
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4114i32);
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4115i32);
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4116i32);
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4117i32);
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4118i32);
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4119i32);
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4120i32);
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4121i32);
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4122i32);
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4123i32);
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4124i32);
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4125i32);
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4126i32);
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4127i32);
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4128i32);
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4129i32);
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4130i32);
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4131i32);
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4132i32);
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4133i32);
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4134i32);
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4135i32);
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4136i32);
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4137i32);
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4138i32);
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4139i32);
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4140i32);
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4141i32);
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4142i32);
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4143i32);
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4144i32);
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4145i32);
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4146i32);
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4147i32);
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4148i32);
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4149i32);
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4150i32);
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4151i32);
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4152i32);
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4153i32);
impl ::core::convert::From<i32> for IMMPID_MP_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_MP_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_NMP_ENUM(pub i32);
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24575i32);
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24576i32);
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24577i32);
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24578i32);
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24579i32);
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24580i32);
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24581i32);
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24582i32);
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24583i32);
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24584i32);
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24585i32);
impl ::core::convert::From<i32> for IMMPID_NMP_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_NMP_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_RPV_ENUM(pub i32);
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
impl ::core::convert::From<i32> for IMMPID_RPV_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_RPV_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMMPID_RP_ENUM(pub i32);
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8191i32);
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8192i32);
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8193i32);
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8194i32);
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8195i32);
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8196i32);
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8197i32);
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8198i32);
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8199i32);
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8200i32);
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8201i32);
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8202i32);
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8203i32);
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8204i32);
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8205i32);
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8206i32);
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8207i32);
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8208i32);
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8209i32);
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8210i32);
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8211i32);
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8212i32);
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8213i32);
impl ::core::convert::From<i32> for IMMPID_RP_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMMPID_RP_ENUM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: ::windows::runtime::GUID,
}
impl IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::default::Default for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IMMP_MPV_STORE_DRIVER_HANDLE").field("guidSignature", &self.guidSignature).finish()
    }
}
impl ::core::cmp::PartialEq for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.guidSignature == other.guidSignature
    }
}
impl ::core::cmp::Eq for IMMP_MPV_STORE_DRIVER_HANDLE {}
unsafe impl ::windows::runtime::Abi for IMMP_MPV_STORE_DRIVER_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultisession(pub ::windows::runtime::IUnknown);
impl IMultisession {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn InUse(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportRecorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMultisession {
    type Vtable = IMultisession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801552, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IMultisession> for ::windows::runtime::IUnknown {
    fn from(value: IMultisession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultisession> for ::windows::runtime::IUnknown {
    fn from(value: &IMultisession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultisession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMultisession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisession> for super::super::System::Com::IDispatch {
    fn from(value: IMultisession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisession> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IMultisession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IMultisession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultisessionRandomWrite(pub ::windows::runtime::IUnknown);
impl IMultisessionRandomWrite {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn InUse(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportRecorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn WriteUnitSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenAddress(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMultisessionRandomWrite {
    type Vtable = IMultisessionRandomWrite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186595, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IMultisessionRandomWrite> for ::windows::runtime::IUnknown {
    fn from(value: IMultisessionRandomWrite) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultisessionRandomWrite> for ::windows::runtime::IUnknown {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMultisessionRandomWrite> for IMultisession {
    fn from(value: IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultisessionRandomWrite> for IMultisession {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for &IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionRandomWrite> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionRandomWrite> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IMultisessionRandomWrite {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultisessionSequential(pub ::windows::runtime::IUnknown);
impl IMultisessionSequential {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn InUse(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportRecorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsFirstDataSession(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NextWritableAddress(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMultisessionSequential {
    type Vtable = IMultisessionSequential_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801553, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IMultisessionSequential> for ::windows::runtime::IUnknown {
    fn from(value: IMultisessionSequential) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultisessionSequential> for ::windows::runtime::IUnknown {
    fn from(value: &IMultisessionSequential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMultisessionSequential> for IMultisession {
    fn from(value: IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultisessionSequential> for IMultisession {
    fn from(value: &IMultisessionSequential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for &IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionSequential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IMultisessionSequential {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultisessionSequential2(pub ::windows::runtime::IUnknown);
impl IMultisessionSequential2 {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn InUse(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ImportRecorder(&self) -> ::windows::runtime::Result<IDiscRecorder2> {
        let mut result__: <IDiscRecorder2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn IsFirstDataSession(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NextWritableAddress(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn WriteUnitSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMultisessionSequential2 {
    type Vtable = IMultisessionSequential2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186594, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IMultisessionSequential2> for ::windows::runtime::IUnknown {
    fn from(value: IMultisessionSequential2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultisessionSequential2> for ::windows::runtime::IUnknown {
    fn from(value: &IMultisessionSequential2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMultisessionSequential2> for IMultisessionSequential {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultisessionSequential2> for IMultisessionSequential {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisessionSequential> for IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisessionSequential> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisessionSequential> for &IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisessionSequential> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMultisessionSequential2> for IMultisession {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultisessionSequential2> for IMultisession {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMultisession> for &IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMultisession> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential2> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential2> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IMultisessionSequential2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProgressItem(pub ::windows::runtime::IUnknown);
impl IProgressItem {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FirstBlock(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastBlock(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BlockCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProgressItem {
    type Vtable = IProgressItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904981, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IProgressItem> for ::windows::runtime::IUnknown {
    fn from(value: IProgressItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProgressItem> for ::windows::runtime::IUnknown {
    fn from(value: &IProgressItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProgressItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProgressItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItem> for super::super::System::Com::IDispatch {
    fn from(value: IProgressItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItem> for super::super::System::Com::IDispatch {
    fn from(value: &IProgressItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IProgressItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IProgressItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, block: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, block: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocks: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProgressItems(pub ::windows::runtime::IUnknown);
impl IProgressItems {
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Ole`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__: <super::super::System::Ole::IEnumVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IProgressItem> {
        let mut result__: <IProgressItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IProgressItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> ::windows::runtime::Result<IProgressItem> {
        let mut result__: <IProgressItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(block), &mut result__).from_abi::<IProgressItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ProgressItemFromDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<IProgressItem> {
        let mut result__: <IProgressItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi(), &mut result__).from_abi::<IProgressItem>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EnumProgressItems(&self) -> ::windows::runtime::Result<IEnumProgressItems> {
        let mut result__: <IEnumProgressItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumProgressItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProgressItems {
    type Vtable = IProgressItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904983, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
}
impl ::core::convert::From<IProgressItems> for ::windows::runtime::IUnknown {
    fn from(value: IProgressItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProgressItems> for ::windows::runtime::IUnknown {
    fn from(value: &IProgressItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItems> for super::super::System::Com::IDispatch {
    fn from(value: IProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItems> for super::super::System::Com::IDispatch {
    fn from(value: &IProgressItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IProgressItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, block: u32, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRawCDImageCreator(pub ::windows::runtime::IUnknown);
impl IRawCDImageCreator {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CreateResultImage(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn AddTrack<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(datatype), data.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn AddSpecialPregap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn AddSubcodeRWGenerator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, subcode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), subcode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ResultingImageType(&self) -> ::windows::runtime::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__: <IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartOfLeadout(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartOfLeadoutLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetDisableGaplessAudio(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DisableGaplessAudio(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetMediaCatalogNumber<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn MediaCatalogNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartingTrackNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TrackInfo(&self, trackindex: i32) -> ::windows::runtime::Result<IRawCDImageTrackInfo> {
        let mut result__: <IRawCDImageTrackInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(trackindex), &mut result__).from_abi::<IRawCDImageTrackInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastUsedUserSectorInImage(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRawCDImageCreator {
    type Vtable = IRawCDImageCreator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(630732112, 40293, 18894, [179, 53, 64, 99, 13, 144, 18, 39]);
}
impl ::core::convert::From<IRawCDImageCreator> for ::windows::runtime::IUnknown {
    fn from(value: IRawCDImageCreator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRawCDImageCreator> for ::windows::runtime::IUnknown {
    fn from(value: &IRawCDImageCreator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRawCDImageCreator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRawCDImageCreator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageCreator> for super::super::System::Com::IDispatch {
    fn from(value: IRawCDImageCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageCreator> for super::super::System::Com::IDispatch {
    fn from(value: &IRawCDImageCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IRawCDImageCreator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IRawCDImageCreator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resultstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: IMAPI_CD_SECTOR_TYPE, data: ::windows::runtime::RawPtr, trackindex: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subcode: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, trackindex: i32, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRawCDImageTrackInfo(pub ::windows::runtime::IUnknown);
impl IRawCDImageTrackInfo {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartingLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TrackNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorType(&self) -> ::windows::runtime::Result<IMAPI_CD_SECTOR_TYPE> {
        let mut result__: <IMAPI_CD_SECTOR_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_CD_SECTOR_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn ISRC(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`*"]
    pub unsafe fn SetISRC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn DigitalAudioCopySetting(&self) -> ::windows::runtime::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        let mut result__: <IMAPI_CD_TRACK_DIGITAL_COPY_SETTING as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn AudioHasPreemphasis(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetAudioHasPreemphasis(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn TrackIndexes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbaoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbaoffset)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRawCDImageTrackInfo {
    type Vtable = IRawCDImageTrackInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(630732113, 40293, 18894, [179, 53, 64, 99, 13, 144, 18, 39]);
}
impl ::core::convert::From<IRawCDImageTrackInfo> for ::windows::runtime::IUnknown {
    fn from(value: IRawCDImageTrackInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRawCDImageTrackInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IRawCDImageTrackInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRawCDImageTrackInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRawCDImageTrackInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageTrackInfo> for super::super::System::Com::IDispatch {
    fn from(value: IRawCDImageTrackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageTrackInfo> for super::super::System::Com::IDispatch {
    fn from(value: &IRawCDImageTrackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IRawCDImageTrackInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IRawCDImageTrackInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lbaoffset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lbaoffset: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRedbookDiscMaster(pub ::windows::runtime::IUnknown);
impl IRedbookDiscMaster {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTotalAudioTracks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetTotalAudioBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetUsedAudioBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetAvailableAudioTrackBlocks(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn GetAudioBlockSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CreateAudioTrack(&self, nblocks: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(nblocks)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn AddAudioTrackBlocks(&self, pby: *const u8, cb: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pby), ::core::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CloseAudioTrack(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRedbookDiscMaster {
    type Vtable = IRedbookDiscMaster_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820765901, 20060, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
}
impl ::core::convert::From<IRedbookDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: IRedbookDiscMaster) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRedbookDiscMaster> for ::windows::runtime::IUnknown {
    fn from(value: &IRedbookDiscMaster) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRedbookDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRedbookDiscMaster {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedbookDiscMaster_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pntracks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblocks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblocks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblocks: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnblockbytes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nblocks: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pby: *const u8, cb: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStreamConcatenate(pub ::windows::runtime::IUnknown);
impl IStreamConcatenate {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream1: Param0, stream2: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), stream1.into_param().abi(), stream2.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Initialize2(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(streams), ::core::mem::transmute(streamcount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), stream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Append2(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(streams), ::core::mem::transmute(streamcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStreamConcatenate {
    type Vtable = IStreamConcatenate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801542, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IStreamConcatenate> for ::windows::runtime::IUnknown {
    fn from(value: IStreamConcatenate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStreamConcatenate> for ::windows::runtime::IUnknown {
    fn from(value: &IStreamConcatenate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamConcatenate> for super::super::System::Com::IStream {
    fn from(value: IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamConcatenate> for super::super::System::Com::IStream {
    fn from(value: &IStreamConcatenate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for &IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamConcatenate> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamConcatenate> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamConcatenate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IStreamConcatenate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream1: ::windows::runtime::RawPtr, stream2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streams: *const ::windows::runtime::RawPtr, streamcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streams: *const ::windows::runtime::RawPtr, streamcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStreamInterleave(pub ::windows::runtime::IUnknown);
impl IStreamInterleave {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Initialize(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(streams), ::core::mem::transmute(interleavesizes), ::core::mem::transmute(streamcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStreamInterleave {
    type Vtable = IStreamInterleave_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801543, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IStreamInterleave> for ::windows::runtime::IUnknown {
    fn from(value: IStreamInterleave) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStreamInterleave> for ::windows::runtime::IUnknown {
    fn from(value: &IStreamInterleave) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamInterleave> for super::super::System::Com::IStream {
    fn from(value: IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamInterleave> for super::super::System::Com::IStream {
    fn from(value: &IStreamInterleave) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for &IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamInterleave> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamInterleave> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamInterleave) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IStreamInterleave {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streams: *const ::windows::runtime::RawPtr, interleavesizes: *const u32, streamcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStreamPseudoRandomBased(pub ::windows::runtime::IUnknown);
impl IStreamPseudoRandomBased {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetSeed(&self, value: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Seed(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetExtendedSeed(&self, values: *const u32, ecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(values), ::core::mem::transmute(ecount)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(values), ::core::mem::transmute(ecount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStreamPseudoRandomBased {
    type Vtable = IStreamPseudoRandomBased_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801541, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IStreamPseudoRandomBased> for ::windows::runtime::IUnknown {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStreamPseudoRandomBased> for ::windows::runtime::IUnknown {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamPseudoRandomBased> for super::super::System::Com::IStream {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamPseudoRandomBased> for super::super::System::Com::IStream {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for &IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamPseudoRandomBased> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamPseudoRandomBased> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IStreamPseudoRandomBased {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, values: *const u32, ecount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, values: *mut *mut u32, ecount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWriteEngine2(pub ::windows::runtime::IUnknown);
impl IWriteEngine2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    pub unsafe fn WriteSection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, data: Param0, startingblockaddress: i32, numberofblocks: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), data.into_param().abi(), ::core::mem::transmute(startingblockaddress), ::core::mem::transmute(numberofblocks)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn CancelWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetRecorder<'a, Param0: ::windows::runtime::IntoParam<'a, IDiscRecorder2Ex>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn Recorder(&self) -> ::windows::runtime::Result<IDiscRecorder2Ex> {
        let mut result__: <IDiscRecorder2Ex as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDiscRecorder2Ex>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetUseStreamingWrite12(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UseStreamingWrite12(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartingSectorsPerSecond(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn EndingSectorsPerSecond(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn BytesPerSector(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn WriteInProgress(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWriteEngine2 {
    type Vtable = IWriteEngine2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801525, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IWriteEngine2> for ::windows::runtime::IUnknown {
    fn from(value: IWriteEngine2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWriteEngine2> for ::windows::runtime::IUnknown {
    fn from(value: &IWriteEngine2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWriteEngine2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWriteEngine2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2> for super::super::System::Com::IDispatch {
    fn from(value: IWriteEngine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteEngine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IWriteEngine2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IWriteEngine2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, startingblockaddress: i32, numberofblocks: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWriteEngine2EventArgs(pub ::windows::runtime::IUnknown);
impl IWriteEngine2EventArgs {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn StartLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn SectorCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastReadLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn LastWrittenLba(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWriteEngine2EventArgs {
    type Vtable = IWriteEngine2EventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801526, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IWriteEngine2EventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IWriteEngine2EventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWriteEngine2EventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IWriteEngine2EventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWriteEngine2EventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWriteEngine2EventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2EventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IWriteEngine2EventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2EventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteEngine2EventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IWriteEngine2EventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IWriteEngine2EventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWriteSpeedDescriptor(pub ::windows::runtime::IUnknown);
impl IWriteSpeedDescriptor {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn MediaType(&self) -> ::windows::runtime::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__: <IMAPI_MEDIA_PHYSICAL_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn RotationTypeIsPureCAV(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub unsafe fn WriteSpeed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWriteSpeedDescriptor {
    type Vtable = IWriteSpeedDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801540, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
}
impl ::core::convert::From<IWriteSpeedDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: IWriteSpeedDescriptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWriteSpeedDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &IWriteSpeedDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWriteSpeedDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWriteSpeedDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteSpeedDescriptor> for super::super::System::Com::IDispatch {
    fn from(value: IWriteSpeedDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteSpeedDescriptor> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteSpeedDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IWriteSpeedDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IWriteSpeedDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MEDIA_FLAGS(pub i32);
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
impl ::core::convert::From<i32> for MEDIA_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEDIA_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MEDIA_TYPES(pub i32);
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
impl ::core::convert::From<i32> for MEDIA_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEDIA_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MPV_WRITE_CONTENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_RETRY: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376569955, 20901, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
pub const MSDiscRecorderObj: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376569953, 20901, 4563, [145, 68, 0, 16, 75, 161, 28, 94]);
pub const MSEnumDiscRecordersObj: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2315474554, 25547, 19368, [186, 246, 82, 17, 152, 22, 209, 239]);
#[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = unsafe extern "system" fn(ulcallerdata: u32, lpmessage: ::windows::runtime::RawPtr);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[inline]
pub unsafe fn MapStorageSCode(stgscode: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapStorageSCode(stgscode: i32) -> i32;
        }
        ::core::mem::transmute(MapStorageSCode(::core::mem::transmute(stgscode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MsftDiscFormat2Data: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801514, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftDiscFormat2Erase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801515, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftDiscFormat2RawCD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801512, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftDiscFormat2TrackAtOnce: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801513, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftDiscMaster2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801518, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftDiscRecorder2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801517, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftFileSystemImage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904965, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const MsftIsoImageManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3471719266, 36694, 16470, [134, 155, 239, 22, 145, 126, 62, 252]);
pub const MsftMultisessionRandomWrite: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037186596, 8708, 4573, [150, 106, 0, 26, 160, 27, 188, 88]);
pub const MsftMultisessionSequential: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801506, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftRawCDImageCreator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(630732129, 40293, 18894, [179, 53, 64, 99, 13, 144, 18, 39]);
pub const MsftStreamConcatenate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801509, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftStreamInterleave: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801508, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftStreamPrng001: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801510, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftStreamZero: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801511, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftWriteEngine2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801516, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
pub const MsftWriteSpeedDescriptor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657801507, 32612, 23311, [143, 0, 93, 119, 175, 190, 38, 30]);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const NMP_PROCESS_POST: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OpenIMsgOnIStg<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>, Param6: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>>(
    lpmsgsess: *mut _MSGSESS,
    lpallocatebuffer: ::core::option::Option<super::super::System::AddressBook::LPALLOCATEBUFFER>,
    lpallocatemore: ::core::option::Option<super::super::System::AddressBook::LPALLOCATEMORE>,
    lpfreebuffer: ::core::option::Option<super::super::System::AddressBook::LPFREEBUFFER>,
    lpmalloc: Param4,
    lpmapisup: *mut ::core::ffi::c_void,
    lpstg: Param6,
    lpfmsgcallrelease: *mut ::core::option::Option<MSGCALLRELEASE>,
    ulcallerdata: u32,
    ulflags: u32,
    lppmsg: *mut ::core::option::Option<super::super::System::AddressBook::IMessage>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenIMsgOnIStg(lpmsgsess: *mut _MSGSESS, lpallocatebuffer: ::windows::runtime::RawPtr, lpallocatemore: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpmalloc: ::windows::runtime::RawPtr, lpmapisup: *mut ::core::ffi::c_void, lpstg: ::windows::runtime::RawPtr, lpfmsgcallrelease: *mut ::windows::runtime::RawPtr, ulcallerdata: u32, ulflags: u32, lppmsg: *mut ::windows::runtime::RawPtr) -> i32;
        }
        ::core::mem::transmute(OpenIMsgOnIStg(
            ::core::mem::transmute(lpmsgsess),
            ::core::mem::transmute(lpallocatebuffer),
            ::core::mem::transmute(lpallocatemore),
            ::core::mem::transmute(lpfreebuffer),
            lpmalloc.into_param().abi(),
            ::core::mem::transmute(lpmapisup),
            lpstg.into_param().abi(),
            ::core::mem::transmute(lpfmsgcallrelease),
            ::core::mem::transmute(ulcallerdata),
            ::core::mem::transmute(ulflags),
            ::core::mem::transmute(lppmsg),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenIMsgSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>>(lpmalloc: Param0, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenIMsgSession(lpmalloc: ::windows::runtime::RawPtr, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
        }
        ::core::mem::transmute(OpenIMsgSession(lpmalloc.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(lppmsgsess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformId(pub i32);
pub const PlatformX86: PlatformId = PlatformId(0i32);
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
pub const PlatformMac: PlatformId = PlatformId(2i32);
pub const PlatformEFI: PlatformId = PlatformId(239i32);
impl ::core::convert::From<i32> for PlatformId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformId {
    type Abi = Self;
}
pub const ProgressItem: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904971, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
pub const ProgressItems: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747904969, 38747, 22974, [169, 96, 154, 42, 38, 40, 83, 165]);
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RECORDER_TYPES(pub i32);
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
impl ::core::convert::From<i32> for RECORDER_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RECORDER_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DELIVERED: u32 = 272u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_HANDLED: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ENPANDED: u32 = 8208u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_EXPANDED: u32 = 8208u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_FAILED: u32 = 2096u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_GENERAL_FAILURE: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_HANDLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_UNRESOLVED: u32 = 4144u32;
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl SPropAttrArray {}
impl ::core::default::Default for SPropAttrArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SPropAttrArray {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SPropAttrArray").field("cValues", &self.cValues).field("aPropAttr", &self.aPropAttr).finish()
    }
}
impl ::core::cmp::PartialEq for SPropAttrArray {
    fn eq(&self, other: &Self) -> bool {
        self.cValues == other.cValues && self.aPropAttr == other.aPropAttr
    }
}
impl ::core::cmp::Eq for SPropAttrArray {}
unsafe impl ::windows::runtime::Abi for SPropAttrArray {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows::runtime::HRESULT;
        }
        SetAttribIMsgOnIStg(::core::mem::transmute(lpobject), ::core::mem::transmute(lpproptags), ::core::mem::transmute(lppropattrs), ::core::mem::transmute(lpppropproblems)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct _MSGSESS(pub u8);
pub const tagIMMPID_CPV_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2728880938, 58669, 4561, [170, 100, 0, 192, 79, 163, 91, 130]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Imapi`*"]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *mut ::windows::runtime::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl tagIMMPID_GUIDLIST_ITEM {}
impl ::core::default::Default for tagIMMPID_GUIDLIST_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for tagIMMPID_GUIDLIST_ITEM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("tagIMMPID_GUIDLIST_ITEM").field("pguid", &self.pguid).field("dwStart", &self.dwStart).field("dwLast", &self.dwLast).finish()
    }
}
impl ::core::cmp::PartialEq for tagIMMPID_GUIDLIST_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwStart == other.dwStart && self.dwLast == other.dwLast
    }
}
impl ::core::cmp::Eq for tagIMMPID_GUIDLIST_ITEM {}
unsafe impl ::windows::runtime::Abi for tagIMMPID_GUIDLIST_ITEM {
    type Abi = Self;
}
pub const tagIMMPID_MPV_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3420886790, 51645, 4561, [159, 242, 0, 192, 79, 163, 115, 72]);
pub const tagIMMPID_MP_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(322456816, 46020, 4561, [170, 146, 0, 170, 0, 107, 200, 11]);
pub const tagIMMPID_NMP_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1949542826, 8418, 4562, [148, 214, 0, 192, 79, 163, 121, 241]);
pub const tagIMMPID_RPV_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045255753, 54048, 4561, [159, 244, 0, 192, 79, 163, 115, 72]);
pub const tagIMMPID_RP_STRUCT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045255752, 54048, 4561, [159, 244, 0, 192, 79, 163, 115, 72]);
