#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AUTHNEXTSTEP(pub i32);
pub const DefaultBehavior: AUTHNEXTSTEP = AUTHNEXTSTEP(0i32);
pub const RetryRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(1i32);
pub const CancelRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(2i32);
impl ::std::convert::From<i32> for AUTHNEXTSTEP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUTHNEXTSTEP {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DAV_CALLBACK_AUTH_BLOB {
    pub pBuffer: *mut ::std::ffi::c_void,
    pub ulSize: u32,
    pub ulType: u32,
}
impl DAV_CALLBACK_AUTH_BLOB {}
impl ::std::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DAV_CALLBACK_AUTH_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_AUTH_BLOB")
            .field("pBuffer", &self.pBuffer)
            .field("ulSize", &self.ulSize)
            .field("ulType", &self.ulType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.ulSize == other.ulSize && self.ulType == other.ulType
    }
}
impl ::std::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_AUTH_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: super::super::Foundation::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: super::super::Foundation::PWSTR,
    pub ulPasswordLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DAV_CALLBACK_AUTH_UNP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_AUTH_UNP")
            .field("pszUserName", &self.pszUserName)
            .field("ulUserNameLength", &self.ulUserNameLength)
            .field("pszPassword", &self.pszPassword)
            .field("ulPasswordLength", &self.ulPasswordLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName
            && self.ulUserNameLength == other.ulUserNameLength
            && self.pszPassword == other.pszPassword
            && self.ulPasswordLength == other.ulPasswordLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_AUTH_UNP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: super::super::Foundation::BOOL,
    pub bSave: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DAV_CALLBACK_CRED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_CRED")
            .field("AuthBlob", &self.AuthBlob)
            .field("UNPBlob", &self.UNPBlob)
            .field("bAuthBlobValid", &self.bAuthBlobValid)
            .field("bSave", &self.bSave)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DAV_CALLBACK_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBlob == other.AuthBlob
            && self.UNPBlob == other.UNPBlob
            && self.bAuthBlobValid == other.bAuthBlobValid
            && self.bSave == other.bSave
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_CRED {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavAddConnection<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    connectionhandle: *mut super::super::Foundation::HANDLE,
    remotename: Param1,
    username: Param2,
    password: Param3,
    clientcert: *const u8,
    certsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavAddConnection(
                connectionhandle: *mut super::super::Foundation::HANDLE,
                remotename: super::super::Foundation::PWSTR,
                username: super::super::Foundation::PWSTR,
                password: super::super::Foundation::PWSTR,
                clientcert: *const u8,
                certsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(DavAddConnection(
            ::std::mem::transmute(connectionhandle),
            remotename.into_param().abi(),
            username.into_param().abi(),
            password.into_param().abi(),
            ::std::mem::transmute(clientcert),
            ::std::mem::transmute(certsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavCancelConnectionsToServer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    fforce: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "davclnt")]
        extern "system" {
            fn DavCancelConnectionsToServer(
                lpname: super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(DavCancelConnectionsToServer(
            lpname.into_param().abi(),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavDeleteConnection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    connectionhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(DavDeleteConnection(connectionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavFlushFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(DavFlushFile(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavGetExtendedError<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    exterror: *mut u32,
    exterrorstring: super::super::Foundation::PWSTR,
    cchsize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavGetExtendedError(
                hfile: super::super::Foundation::HANDLE,
                exterror: *mut u32,
                exterrorstring: super::super::Foundation::PWSTR,
                cchsize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(DavGetExtendedError(
            hfile.into_param().abi(),
            ::std::mem::transmute(exterror),
            ::std::mem::transmute(exterrorstring),
            ::std::mem::transmute(cchsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavGetHTTPFromUNCPath<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    uncpath: Param0,
    url: super::super::Foundation::PWSTR,
    lpsize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavGetHTTPFromUNCPath(
                uncpath: super::super::Foundation::PWSTR,
                url: super::super::Foundation::PWSTR,
                lpsize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(DavGetHTTPFromUNCPath(
            uncpath.into_param().abi(),
            ::std::mem::transmute(url),
            ::std::mem::transmute(lpsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavGetTheLockOwnerOfTheFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filename: Param0,
    lockownername: super::super::Foundation::PWSTR,
    lockownernamelengthinbytes: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "davclnt")]
        extern "system" {
            fn DavGetTheLockOwnerOfTheFile(
                filename: super::super::Foundation::PWSTR,
                lockownername: super::super::Foundation::PWSTR,
                lockownernamelengthinbytes: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(DavGetTheLockOwnerOfTheFile(
            filename.into_param().abi(),
            ::std::mem::transmute(lockownername),
            ::std::mem::transmute(lockownernamelengthinbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavGetUNCFromHTTPPath<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    url: Param0,
    uncpath: super::super::Foundation::PWSTR,
    lpsize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "netapi32")]
        extern "system" {
            fn DavGetUNCFromHTTPPath(
                url: super::super::Foundation::PWSTR,
                uncpath: super::super::Foundation::PWSTR,
                lpsize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(DavGetUNCFromHTTPPath(
            url.into_param().abi(),
            ::std::mem::transmute(uncpath),
            ::std::mem::transmute(lpsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavInvalidateCache<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    urlname: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "davclnt")]
        extern "system" {
            fn DavInvalidateCache(urlname: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(DavInvalidateCache(urlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DavRegisterAuthCallback(
    callback: ::std::option::Option<PFNDAVAUTHCALLBACK>,
    version: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "davclnt")]
        extern "system" {
            fn DavRegisterAuthCallback(callback: ::windows::runtime::RawPtr, version: u32) -> u32;
        }
        ::std::mem::transmute(DavRegisterAuthCallback(
            ::std::mem::transmute(callback),
            ::std::mem::transmute(version),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    #[cfg(windows)]
    {
        #[link(name = "davclnt")]
        extern "system" {
            fn DavUnregisterAuthCallback(hcallback: u32);
        }
        ::std::mem::transmute(DavUnregisterAuthCallback(::std::mem::transmute(hcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = unsafe extern "system" fn(
    lpwzservername: super::super::Foundation::PWSTR,
    lpwzremotename: super::super::Foundation::PWSTR,
    dwauthscheme: u32,
    dwflags: u32,
    pcallbackcred: *mut DAV_CALLBACK_CRED,
    nextstep: *mut AUTHNEXTSTEP,
    pfreecred: *mut ::windows::runtime::RawPtr,
) -> u32;
pub type PFNDAVAUTHCALLBACK_FREECRED =
    unsafe extern "system" fn(pbuffer: *const ::std::ffi::c_void) -> u32;
