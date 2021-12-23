#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommand {
    type Vtable = IVoiceCommandVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Media_SpeechRecognition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc85e675d_fe42_432c_9907_09df9fcf64e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandCompletionReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandConfirmationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa022593e_8221_4526_b083_840972262247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandContentTile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandContentTileVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandContentTileType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VoiceCommandContentTileType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7972aad0_0974_4979_984b_cb8959cd61ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDefinitionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandDefinitionManagerStatics {
    type Vtable = IVoiceCommandDefinitionManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fe7a69e_067e_4f16_a18c_5b17e9499940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDisambiguationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandResponse {
    type Vtable = IVoiceCommandResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandResponseStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandResponseStatics {
    type Vtable = IVoiceCommandResponseStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2932f813_0d3b_49f2_96dd_625019bd3b5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usermessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandServiceConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandServiceConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandServiceConnectionStatics {
    type Vtable = IVoiceCommandServiceConnectionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x370ebffb_2d34_42df_8770_074d0f334697);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandUserMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674eb3c0_44f6_4f07_b979_4c723fc08597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommand(::windows::core::IUnknown);
impl VoiceCommand {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn CommandName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Media_SpeechRecognition'*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn SpeechRecognitionResult(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommand {}
impl ::core::fmt::Debug for VoiceCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommand;{936f5273-ec82-42a6-a55c-d2d79ec6f920})");
}
unsafe impl ::windows::core::Interface for VoiceCommand {
    type Vtable = IVoiceCommandVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
}
impl ::windows::core::RuntimeName for VoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommand";
}
impl ::core::convert::From<VoiceCommand> for ::windows::core::IUnknown {
    fn from(value: VoiceCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommand> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommand> for ::windows::core::IInspectable {
    fn from(value: VoiceCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommand> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommand {}
unsafe impl ::core::marker::Sync for VoiceCommand {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandCompletedEventArgs(::windows::core::IUnknown);
impl VoiceCommandCompletedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Reason(&self) -> ::windows::core::Result<VoiceCommandCompletionReason> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandCompletionReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandCompletionReason>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandCompletedEventArgs {}
impl ::core::fmt::Debug for VoiceCommandCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs;{c85e675d-fe42-432c-9907-09df9fcf64e8})");
}
unsafe impl ::windows::core::Interface for VoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc85e675d_fe42_432c_9907_09df9fcf64e8);
}
impl ::windows::core::RuntimeName for VoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs";
}
impl ::core::convert::From<VoiceCommandCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandCompletedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandCompletedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: Self = Self(0i32);
    pub const CommunicationFailed: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const TimeoutExceeded: Self = Self(4i32);
    pub const AppLaunched: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for VoiceCommandCompletionReason {}
impl ::core::clone::Clone for VoiceCommandCompletionReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VoiceCommandCompletionReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VoiceCommandCompletionReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandCompletionReason {}
impl ::core::fmt::Debug for VoiceCommandCompletionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandCompletionReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandCompletionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletionReason;i4)");
}
impl ::windows::core::DefaultType for VoiceCommandCompletionReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandConfirmationResult(::windows::core::IUnknown);
impl VoiceCommandConfirmationResult {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Confirmed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandConfirmationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandConfirmationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandConfirmationResult {}
impl ::core::fmt::Debug for VoiceCommandConfirmationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandConfirmationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandConfirmationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult;{a022593e-8221-4526-b083-840972262247})");
}
unsafe impl ::windows::core::Interface for VoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa022593e_8221_4526_b083_840972262247);
}
impl ::windows::core::RuntimeName for VoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult";
}
impl ::core::convert::From<VoiceCommandConfirmationResult> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandConfirmationResult> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandConfirmationResult> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandConfirmationResult> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandConfirmationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandConfirmationResult {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandContentTile(::windows::core::IUnknown);
impl VoiceCommandContentTile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandContentTile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn TextLine1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetTextLine1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn TextLine2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetTextLine2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn TextLine3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetTextLine3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn Image(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn AppContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetAppContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetAppLaunchArgument<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn ContentTileType(&self) -> ::windows::core::Result<VoiceCommandContentTileType> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandContentTileType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandContentTileType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetContentTileType(&self, value: VoiceCommandContentTileType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for VoiceCommandContentTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandContentTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandContentTile {}
impl ::core::fmt::Debug for VoiceCommandContentTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandContentTile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandContentTile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile;{3eefe9f0-b8c7-4c76-a0de-1607895ee327})");
}
unsafe impl ::windows::core::Interface for VoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
}
impl ::windows::core::RuntimeName for VoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile";
}
impl ::core::convert::From<VoiceCommandContentTile> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandContentTile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandContentTile> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandContentTile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandContentTile> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandContentTile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandContentTile> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandContentTile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandContentTile {}
unsafe impl ::core::marker::Sync for VoiceCommandContentTile {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: Self = Self(0i32);
    pub const TitleWithText: Self = Self(1i32);
    pub const TitleWith68x68Icon: Self = Self(2i32);
    pub const TitleWith68x68IconAndText: Self = Self(3i32);
    pub const TitleWith68x92Icon: Self = Self(4i32);
    pub const TitleWith68x92IconAndText: Self = Self(5i32);
    pub const TitleWith280x140Icon: Self = Self(6i32);
    pub const TitleWith280x140IconAndText: Self = Self(7i32);
}
impl ::core::marker::Copy for VoiceCommandContentTileType {}
impl ::core::clone::Clone for VoiceCommandContentTileType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VoiceCommandContentTileType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VoiceCommandContentTileType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandContentTileType {}
impl ::core::fmt::Debug for VoiceCommandContentTileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandContentTileType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandContentTileType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTileType;i4)");
}
impl ::windows::core::DefaultType for VoiceCommandContentTileType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandDefinition(::windows::core::IUnknown);
impl VoiceCommandDefinition {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetPhraseListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, phraselistname: Param0, phraselist: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), phraselistname.into_param().abi(), phraselist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandDefinition {}
impl ::core::fmt::Debug for VoiceCommandDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition;{7972aad0-0974-4979-984b-cb8959cd61ae})");
}
unsafe impl ::windows::core::Interface for VoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7972aad0_0974_4979_984b_cb8959cd61ae);
}
impl ::windows::core::RuntimeName for VoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition";
}
impl ::core::convert::From<VoiceCommandDefinition> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandDefinition> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandDefinition> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandDefinition> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandDefinition {}
unsafe impl ::core::marker::Sync for VoiceCommandDefinition {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
pub struct VoiceCommandDefinitionManager {}
impl VoiceCommandDefinitionManager {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandDefinitionsFromStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandDefinitions() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandDefinitionManagerStatics<R, F: FnOnce(&IVoiceCommandDefinitionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandDefinitionManager, IVoiceCommandDefinitionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for VoiceCommandDefinitionManager {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager";
}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandDisambiguationResult(::windows::core::IUnknown);
impl VoiceCommandDisambiguationResult {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SelectedItem(&self) -> ::windows::core::Result<VoiceCommandContentTile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandContentTile>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandDisambiguationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandDisambiguationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandDisambiguationResult {}
impl ::core::fmt::Debug for VoiceCommandDisambiguationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandDisambiguationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandDisambiguationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult;{ecc68cfe-c9ac-45df-a8ea-feea08ef9c5e})");
}
unsafe impl ::windows::core::Interface for VoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
}
impl ::windows::core::RuntimeName for VoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult";
}
impl ::core::convert::From<VoiceCommandDisambiguationResult> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandDisambiguationResult> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandDisambiguationResult> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandDisambiguationResult> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandDisambiguationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandDisambiguationResult {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandResponse(::windows::core::IUnknown);
impl VoiceCommandResponse {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn Message(&self) -> ::windows::core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetMessage<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn RepeatMessage(&self) -> ::windows::core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetRepeatMessage<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetAppLaunchArgument<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VoiceCommandContentTiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn MaxSupportedVoiceCommandContentTiles() -> ::windows::core::Result<u32> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn CreateResponse<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(usermessage: Param0) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), usermessage.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseWithTiles<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>>(message: Param0, contenttiles: Param1) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), message.into_param().abi(), contenttiles.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn CreateResponseForPrompt<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(message: Param0, repeatmessage: Param1) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), message.into_param().abi(), repeatmessage.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseForPromptWithTiles<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>>(message: Param0, repeatmessage: Param1, contenttiles: Param2) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), message.into_param().abi(), repeatmessage.into_param().abi(), contenttiles.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandResponseStatics<R, F: FnOnce(&IVoiceCommandResponseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandResponse, IVoiceCommandResponseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VoiceCommandResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandResponse {}
impl ::core::fmt::Debug for VoiceCommandResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse;{0284b30e-8a3b-4cc4-a6a1-cad5be2716b5})");
}
unsafe impl ::windows::core::Interface for VoiceCommandResponse {
    type Vtable = IVoiceCommandResponseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
}
impl ::windows::core::RuntimeName for VoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse";
}
impl ::core::convert::From<VoiceCommandResponse> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandResponse> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandResponse> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandResponse> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandResponse {}
unsafe impl ::core::marker::Sync for VoiceCommandResponse {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandServiceConnection(::windows::core::IUnknown);
impl VoiceCommandServiceConnection {
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCommandAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommand>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestConfirmationAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDisambiguationAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportProgressAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportSuccessAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailureAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAppLaunchAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn Language(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn VoiceCommandCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVoiceCommandCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands', 'ApplicationModel_AppService'*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn FromAppServiceTriggerDetails<'a, Param0: ::windows::core::IntoParam<'a, super::AppService::AppServiceTriggerDetails>>(triggerdetails: Param0) -> ::windows::core::Result<VoiceCommandServiceConnection> {
        Self::IVoiceCommandServiceConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerdetails.into_param().abi(), &mut result__).from_abi::<VoiceCommandServiceConnection>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandServiceConnectionStatics<R, F: FnOnce(&IVoiceCommandServiceConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandServiceConnection, IVoiceCommandServiceConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VoiceCommandServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandServiceConnection {}
impl ::core::fmt::Debug for VoiceCommandServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandServiceConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandServiceConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection;{d894bb9f-21da-44a4-98a2-fb131920a9cc})");
}
unsafe impl ::windows::core::Interface for VoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
}
impl ::windows::core::RuntimeName for VoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection";
}
impl ::core::convert::From<VoiceCommandServiceConnection> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandServiceConnection> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandServiceConnection> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandServiceConnection> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandServiceConnection {}
unsafe impl ::core::marker::Sync for VoiceCommandServiceConnection {}
#[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
#[repr(transparent)]
pub struct VoiceCommandUserMessage(::windows::core::IUnknown);
impl VoiceCommandUserMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandUserMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetDisplayMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SpokenMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_VoiceCommands'*"]
    pub fn SetSpokenMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for VoiceCommandUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandUserMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandUserMessage {}
impl ::core::fmt::Debug for VoiceCommandUserMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandUserMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandUserMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage;{674eb3c0-44f6-4f07-b979-4c723fc08597})");
}
unsafe impl ::windows::core::Interface for VoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674eb3c0_44f6_4f07_b979_4c723fc08597);
}
impl ::windows::core::RuntimeName for VoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage";
}
impl ::core::convert::From<VoiceCommandUserMessage> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandUserMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandUserMessage> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandUserMessage> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandUserMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandUserMessage> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandUserMessage {}
unsafe impl ::core::marker::Sync for VoiceCommandUserMessage {}
