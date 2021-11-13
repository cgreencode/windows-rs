#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INamedPolicyData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INamedPolicyData {}
impl ::core::clone::Clone for INamedPolicyData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INamedPolicyStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INamedPolicyStatics {}
impl ::core::clone::Clone for INamedPolicyStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NamedPolicyData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NamedPolicyData {}
impl ::core::clone::Clone for NamedPolicyData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NamedPolicyKind(pub i32);
impl NamedPolicyKind {
    pub const Invalid: Self = Self(0i32);
    pub const Binary: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Int32: Self = Self(3i32);
    pub const Int64: Self = Self(4i32);
    pub const String: Self = Self(5i32);
}
impl ::core::marker::Copy for NamedPolicyKind {}
impl ::core::clone::Clone for NamedPolicyKind {
    fn clone(&self) -> Self {
        *self
    }
}
