#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EasClientDeviceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasClientDeviceInformation {}
impl ::core::clone::Clone for EasClientDeviceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasClientSecurityPolicy {}
impl ::core::clone::Clone for EasClientSecurityPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasComplianceResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EasComplianceResults {}
impl ::core::clone::Clone for EasComplianceResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
}
impl ::core::marker::Copy for EasDisallowConvenienceLogonResult {}
impl ::core::clone::Clone for EasDisallowConvenienceLogonResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: Self = Self(0i32);
    pub const WindowsEncryption: Self = Self(1i32);
    pub const OtherEncryption: Self = Self(2i32);
}
impl ::core::marker::Copy for EasEncryptionProviderType {}
impl ::core::clone::Clone for EasEncryptionProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxInactivityTimeLockResult {}
impl ::core::clone::Clone for EasMaxInactivityTimeLockResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxPasswordFailedAttemptsResult {}
impl ::core::clone::Clone for EasMaxPasswordFailedAttemptsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordComplexCharactersResult {}
impl ::core::clone::Clone for EasMinPasswordComplexCharactersResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordLengthResult {}
impl ::core::clone::Clone for EasMinPasswordLengthResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedExpirationIncompatible: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const UserCannotChangePassword: Self = Self(6i32);
    pub const AdminsCannotChangePassword: Self = Self(7i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(8i32);
}
impl ::core::marker::Copy for EasPasswordExpirationResult {}
impl ::core::clone::Clone for EasPasswordExpirationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasPasswordHistoryResult {}
impl ::core::clone::Clone for EasPasswordHistoryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const NotProvisionedOnAllVolumes: Self = Self(3i32);
    pub const DeFixedDataNotSupported: Self = Self(4i32);
    pub const FixedDataNotSupported: Self = Self(4i32);
    pub const DeHardwareNotCompliant: Self = Self(5i32);
    pub const HardwareNotCompliant: Self = Self(5i32);
    pub const DeWinReNotConfigured: Self = Self(6i32);
    pub const LockNotConfigured: Self = Self(6i32);
    pub const DeProtectionSuspended: Self = Self(7i32);
    pub const ProtectionSuspended: Self = Self(7i32);
    pub const DeOsVolumeNotProtected: Self = Self(8i32);
    pub const OsVolumeNotProtected: Self = Self(8i32);
    pub const DeProtectionNotYetEnabled: Self = Self(9i32);
    pub const ProtectionNotYetEnabled: Self = Self(9i32);
    pub const NoFeatureLicense: Self = Self(10i32);
    pub const OsNotProtected: Self = Self(11i32);
    pub const UnexpectedFailure: Self = Self(12i32);
}
impl ::core::marker::Copy for EasRequireEncryptionResult {}
impl ::core::clone::Clone for EasRequireEncryptionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasClientDeviceInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasClientDeviceInformation {}
impl ::core::clone::Clone for IEasClientDeviceInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasClientDeviceInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasClientDeviceInformation2 {}
impl ::core::clone::Clone for IEasClientDeviceInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasClientSecurityPolicy {}
impl ::core::clone::Clone for IEasClientSecurityPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasComplianceResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasComplianceResults {}
impl ::core::clone::Clone for IEasComplianceResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEasComplianceResults2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEasComplianceResults2 {}
impl ::core::clone::Clone for IEasComplianceResults2 {
    fn clone(&self) -> Self {
        *self
    }
}
