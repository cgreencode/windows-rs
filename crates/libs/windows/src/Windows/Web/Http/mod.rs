#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpBufferContent(::windows::core::IUnknown);
impl HttpBufferContent {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(content: Param0) -> ::windows::core::Result<HttpBufferContent> {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<HttpBufferContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBufferWithOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(content: Param0, offset: u32, count: u32) -> ::windows::core::Result<HttpBufferContent> {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), content.into_param().abi(), offset, count, &mut result__).from_abi::<HttpBufferContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpBufferContentFactory<R, F: FnOnce(&IHttpBufferContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpBufferContent, IHttpBufferContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpBufferContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpBufferContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpBufferContent {}
unsafe impl ::windows::core::RuntimeType for HttpBufferContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpBufferContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpBufferContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpBufferContent {
    const NAME: &'static str = "Windows.Web.Http.HttpBufferContent";
}
impl ::core::convert::From<HttpBufferContent> for ::windows::core::IUnknown {
    fn from(value: HttpBufferContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBufferContent> for ::windows::core::IUnknown {
    fn from(value: &HttpBufferContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpBufferContent> for ::windows::core::IInspectable {
    fn from(value: HttpBufferContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBufferContent> for ::windows::core::IInspectable {
    fn from(value: &HttpBufferContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpBufferContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpBufferContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpBufferContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpBufferContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpBufferContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpBufferContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpBufferContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpBufferContent {}
unsafe impl ::core::marker::Sync for HttpBufferContent {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpClient(::windows::core::IUnknown);
impl HttpClient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpClient, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetWithOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), completionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetInputStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStringAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PostAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), uri.into_param().abi(), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PutAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), uri.into_param().abi(), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendRequestWithOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), request.into_param().abi(), completionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn DefaultRequestHeaders(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpRequestHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetAsync2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), completionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetInputStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetStringAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPostAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), uri.into_param().abi(), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPutAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), uri.into_param().abi(), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySendRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySendRequestAsync2<'a, Param0: ::windows::core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows::core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), request.into_param().abi(), completionoption, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Filters'*"]
    #[cfg(feature = "Web_Http_Filters")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, Filters::IHttpFilter>>(filter: Param0) -> ::windows::core::Result<HttpClient> {
        Self::IHttpClientFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), filter.into_param().abi(), &mut result__).from_abi::<HttpClient>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpClientFactory<R, F: FnOnce(&IHttpClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpClient, IHttpClientFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpClient {}
unsafe impl ::windows::core::RuntimeType for HttpClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpClient;{7fda1151-3574-4880-a8ba-e6b1e0061f3d})");
}
unsafe impl ::windows::core::Interface for HttpClient {
    type Vtable = IHttpClientVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fda1151_3574_4880_a8ba_e6b1e0061f3d);
}
impl ::windows::core::RuntimeName for HttpClient {
    const NAME: &'static str = "Windows.Web.Http.HttpClient";
}
impl ::core::convert::From<HttpClient> for ::windows::core::IUnknown {
    fn from(value: HttpClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpClient> for ::windows::core::IUnknown {
    fn from(value: &HttpClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpClient> for ::windows::core::IInspectable {
    fn from(value: HttpClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpClient> for ::windows::core::IInspectable {
    fn from(value: &HttpClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpClient> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpClient) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpClient> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpClient) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpClient> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpClient) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpClient> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpClient) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpClient {}
unsafe impl ::core::marker::Sync for HttpClient {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: Self = Self(0i32);
    pub const ResponseHeadersRead: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCompletionOption {}
impl ::core::clone::Clone for HttpCompletionOption {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HttpCompletionOption {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpCompletionOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCompletionOption {}
unsafe impl ::windows::core::RuntimeType for HttpCompletionOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpCompletionOption;i4)");
}
impl ::windows::core::DefaultType for HttpCompletionOption {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpCookie(::windows::core::IUnknown);
impl HttpCookie {
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Expires(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpires<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn HttpOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetHttpOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Secure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetSecure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, domain: Param1, path: Param2) -> ::windows::core::Result<HttpCookie> {
        Self::IHttpCookieFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), domain.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<HttpCookie>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCookieFactory<R, F: FnOnce(&IHttpCookieFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpCookie, IHttpCookieFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookie {}
unsafe impl ::windows::core::RuntimeType for HttpCookie {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})");
}
unsafe impl ::windows::core::Interface for HttpCookie {
    type Vtable = IHttpCookieVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f5488e2_cc2d_4779_86a7_88f10687d249);
}
impl ::windows::core::RuntimeName for HttpCookie {
    const NAME: &'static str = "Windows.Web.Http.HttpCookie";
}
impl ::core::convert::From<HttpCookie> for ::windows::core::IUnknown {
    fn from(value: HttpCookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookie> for ::windows::core::IUnknown {
    fn from(value: &HttpCookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookie> for ::windows::core::IInspectable {
    fn from(value: HttpCookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookie> for ::windows::core::IInspectable {
    fn from(value: &HttpCookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCookie> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookie) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookie> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookie) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpCookie {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookie {}
unsafe impl ::core::marker::Sync for HttpCookie {}
#[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct HttpCookieCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HttpCookieCollection {
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<HttpCookie>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<HttpCookie>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<HttpCookie>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpCookie> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<HttpCookie>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, HttpCookie>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<HttpCookie as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for HttpCookieCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for HttpCookieCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for HttpCookieCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for HttpCookieCollection {
    type Vtable = super::super::Foundation::Collections::IVectorViewVtbl<HttpCookie>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for HttpCookieCollection {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<HttpCookieCollection> for ::windows::core::IUnknown {
    fn from(value: HttpCookieCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&HttpCookieCollection> for ::windows::core::IUnknown {
    fn from(value: &HttpCookieCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<HttpCookieCollection> for ::windows::core::IInspectable {
    fn from(value: HttpCookieCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&HttpCookieCollection> for ::windows::core::IInspectable {
    fn from(value: &HttpCookieCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookieCollection> for super::super::Foundation::Collections::IIterable<HttpCookie> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookieCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookieCollection> for super::super::Foundation::Collections::IIterable<HttpCookie> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookieCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<HttpCookie>> for HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<HttpCookie>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<HttpCookie>> for &HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<HttpCookie>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<HttpCookie>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookieCollection> for super::super::Foundation::Collections::IVectorView<HttpCookie> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookieCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookieCollection> for super::super::Foundation::Collections::IVectorView<HttpCookie> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookieCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<HttpCookie>> for HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<HttpCookie>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<HttpCookie>> for &HttpCookieCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<HttpCookie>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IVectorView<HttpCookie>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for HttpCookieCollection {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpCookieManager(::windows::core::IUnknown);
impl HttpCookieManager {
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetCookie<'a, Param0: ::windows::core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), cookie.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetCookieWithThirdParty<'a, Param0: ::windows::core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0, thirdparty: bool) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi(), thirdparty, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn DeleteCookie<'a, Param0: ::windows::core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCookies<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<HttpCookieCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<HttpCookieCollection>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpCookieManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookieManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookieManager {}
unsafe impl ::windows::core::RuntimeType for HttpCookieManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieManager;{7a431780-cd4f-4e57-a84a-5b0a53d6bb96})");
}
unsafe impl ::windows::core::Interface for HttpCookieManager {
    type Vtable = IHttpCookieManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a431780_cd4f_4e57_a84a_5b0a53d6bb96);
}
impl ::windows::core::RuntimeName for HttpCookieManager {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieManager";
}
impl ::core::convert::From<HttpCookieManager> for ::windows::core::IUnknown {
    fn from(value: HttpCookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookieManager> for ::windows::core::IUnknown {
    fn from(value: &HttpCookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpCookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpCookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookieManager> for ::windows::core::IInspectable {
    fn from(value: HttpCookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookieManager> for ::windows::core::IInspectable {
    fn from(value: &HttpCookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpCookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpCookieManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpCookieManager {}
unsafe impl ::core::marker::Sync for HttpCookieManager {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpFormUrlEncodedContent(::windows::core::IUnknown);
impl HttpFormUrlEncodedContent {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(content: Param0) -> ::windows::core::Result<HttpFormUrlEncodedContent> {
        Self::IHttpFormUrlEncodedContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<HttpFormUrlEncodedContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpFormUrlEncodedContentFactory<R, F: FnOnce(&IHttpFormUrlEncodedContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpFormUrlEncodedContent, IHttpFormUrlEncodedContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpFormUrlEncodedContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpFormUrlEncodedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpFormUrlEncodedContent {}
unsafe impl ::windows::core::RuntimeType for HttpFormUrlEncodedContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpFormUrlEncodedContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpFormUrlEncodedContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpFormUrlEncodedContent {
    const NAME: &'static str = "Windows.Web.Http.HttpFormUrlEncodedContent";
}
impl ::core::convert::From<HttpFormUrlEncodedContent> for ::windows::core::IUnknown {
    fn from(value: HttpFormUrlEncodedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpFormUrlEncodedContent> for ::windows::core::IUnknown {
    fn from(value: &HttpFormUrlEncodedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpFormUrlEncodedContent> for ::windows::core::IInspectable {
    fn from(value: HttpFormUrlEncodedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpFormUrlEncodedContent> for ::windows::core::IInspectable {
    fn from(value: &HttpFormUrlEncodedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpFormUrlEncodedContent {}
unsafe impl ::core::marker::Sync for HttpFormUrlEncodedContent {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpGetBufferResult(::windows::core::IUnknown);
impl HttpGetBufferResult {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetBufferResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetBufferResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetBufferResult {}
unsafe impl ::windows::core::RuntimeType for HttpGetBufferResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetBufferResult;{53d08e7c-e209-404e-9a49-742d8236fd3a})");
}
unsafe impl ::windows::core::Interface for HttpGetBufferResult {
    type Vtable = IHttpGetBufferResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d08e7c_e209_404e_9a49_742d8236fd3a);
}
impl ::windows::core::RuntimeName for HttpGetBufferResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetBufferResult";
}
impl ::core::convert::From<HttpGetBufferResult> for ::windows::core::IUnknown {
    fn from(value: HttpGetBufferResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetBufferResult> for ::windows::core::IUnknown {
    fn from(value: &HttpGetBufferResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetBufferResult> for ::windows::core::IInspectable {
    fn from(value: HttpGetBufferResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetBufferResult> for ::windows::core::IInspectable {
    fn from(value: &HttpGetBufferResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetBufferResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetBufferResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetBufferResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetBufferResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetBufferResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetBufferResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetBufferResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetBufferResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetBufferResult {}
unsafe impl ::core::marker::Sync for HttpGetBufferResult {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpGetInputStreamResult(::windows::core::IUnknown);
impl HttpGetInputStreamResult {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetInputStreamResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetInputStreamResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetInputStreamResult {}
unsafe impl ::windows::core::RuntimeType for HttpGetInputStreamResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetInputStreamResult;{d5d63463-13aa-4ee0-be95-a0c39fe91203})");
}
unsafe impl ::windows::core::Interface for HttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d63463_13aa_4ee0_be95_a0c39fe91203);
}
impl ::windows::core::RuntimeName for HttpGetInputStreamResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetInputStreamResult";
}
impl ::core::convert::From<HttpGetInputStreamResult> for ::windows::core::IUnknown {
    fn from(value: HttpGetInputStreamResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetInputStreamResult> for ::windows::core::IUnknown {
    fn from(value: &HttpGetInputStreamResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetInputStreamResult> for ::windows::core::IInspectable {
    fn from(value: HttpGetInputStreamResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetInputStreamResult> for ::windows::core::IInspectable {
    fn from(value: &HttpGetInputStreamResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetInputStreamResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetInputStreamResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetInputStreamResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetInputStreamResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetInputStreamResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetInputStreamResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetInputStreamResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetInputStreamResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetInputStreamResult {}
unsafe impl ::core::marker::Sync for HttpGetInputStreamResult {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpGetStringResult(::windows::core::IUnknown);
impl HttpGetStringResult {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetStringResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetStringResult {}
unsafe impl ::windows::core::RuntimeType for HttpGetStringResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetStringResult;{9bac466d-8509-4775-b16d-8953f47a7f5f})");
}
unsafe impl ::windows::core::Interface for HttpGetStringResult {
    type Vtable = IHttpGetStringResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bac466d_8509_4775_b16d_8953f47a7f5f);
}
impl ::windows::core::RuntimeName for HttpGetStringResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetStringResult";
}
impl ::core::convert::From<HttpGetStringResult> for ::windows::core::IUnknown {
    fn from(value: HttpGetStringResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetStringResult> for ::windows::core::IUnknown {
    fn from(value: &HttpGetStringResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetStringResult> for ::windows::core::IInspectable {
    fn from(value: HttpGetStringResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetStringResult> for ::windows::core::IInspectable {
    fn from(value: &HttpGetStringResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetStringResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetStringResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetStringResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetStringResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpGetStringResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpGetStringResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpGetStringResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpGetStringResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpGetStringResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetStringResult {}
unsafe impl ::core::marker::Sync for HttpGetStringResult {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpMethod(::windows::core::IUnknown);
impl HttpMethod {
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(method: Param0) -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), method.into_param().abi(), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Delete() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Get() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Head() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Options() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Patch() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Post() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Put() -> ::windows::core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMethodFactory<R, F: FnOnce(&IHttpMethodFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMethod, IHttpMethodFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHttpMethodStatics<R, F: FnOnce(&IHttpMethodStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMethod, IHttpMethodStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethod {}
unsafe impl ::windows::core::RuntimeType for HttpMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMethod;{728d4022-700d-4fe0-afa5-40299c58dbfd})");
}
unsafe impl ::windows::core::Interface for HttpMethod {
    type Vtable = IHttpMethodVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728d4022_700d_4fe0_afa5_40299c58dbfd);
}
impl ::windows::core::RuntimeName for HttpMethod {
    const NAME: &'static str = "Windows.Web.Http.HttpMethod";
}
impl ::core::convert::From<HttpMethod> for ::windows::core::IUnknown {
    fn from(value: HttpMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethod> for ::windows::core::IUnknown {
    fn from(value: &HttpMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMethod> for ::windows::core::IInspectable {
    fn from(value: HttpMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethod> for ::windows::core::IInspectable {
    fn from(value: &HttpMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMethod> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethod) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMethod> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethod) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMethod {}
unsafe impl ::core::marker::Sync for HttpMethod {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpMultipartContent(::windows::core::IUnknown);
impl HttpMultipartContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMultipartContent, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpMultipartContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn CreateWithSubtype<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subtype: Param0) -> ::windows::core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<HttpMultipartContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn CreateWithSubtypeAndBoundary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subtype: Param0, boundary: Param1) -> ::windows::core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), subtype.into_param().abi(), boundary.into_param().abi(), &mut result__).from_abi::<HttpMultipartContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<IHttpContent>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMultipartContentFactory<R, F: FnOnce(&IHttpMultipartContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMultipartContent, IHttpMultipartContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMultipartContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMultipartContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartContent {}
unsafe impl ::windows::core::RuntimeType for HttpMultipartContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpMultipartContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpMultipartContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpMultipartContent> for ::windows::core::IUnknown {
    fn from(value: HttpMultipartContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartContent> for ::windows::core::IUnknown {
    fn from(value: &HttpMultipartContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMultipartContent> for ::windows::core::IInspectable {
    fn from(value: HttpMultipartContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartContent> for ::windows::core::IInspectable {
    fn from(value: &HttpMultipartContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMultipartContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMultipartContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMultipartContent> for super::super::Foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMultipartContent> for super::super::Foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<IHttpContent>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMultipartContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMultipartContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpMultipartContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMultipartContent {}
unsafe impl ::core::marker::Sync for HttpMultipartContent {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpMultipartFormDataContent(::windows::core::IUnknown);
impl HttpMultipartFormDataContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMultipartFormDataContent, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>>(&self, content: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn AddWithName<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, content: Param0, name: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), content.into_param().abi(), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn AddWithNameAndFileName<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, content: Param0, name: Param1, filename: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), content.into_param().abi(), name.into_param().abi(), filename.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn CreateWithBoundary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(boundary: Param0) -> ::windows::core::Result<HttpMultipartFormDataContent> {
        Self::IHttpMultipartFormDataContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), boundary.into_param().abi(), &mut result__).from_abi::<HttpMultipartFormDataContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<IHttpContent>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMultipartFormDataContentFactory<R, F: FnOnce(&IHttpMultipartFormDataContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpMultipartFormDataContent, IHttpMultipartFormDataContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMultipartFormDataContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartFormDataContent {}
unsafe impl ::windows::core::RuntimeType for HttpMultipartFormDataContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartFormDataContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpMultipartFormDataContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpMultipartFormDataContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartFormDataContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpMultipartFormDataContent> for ::windows::core::IUnknown {
    fn from(value: HttpMultipartFormDataContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartFormDataContent> for ::windows::core::IUnknown {
    fn from(value: &HttpMultipartFormDataContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMultipartFormDataContent> for ::windows::core::IInspectable {
    fn from(value: HttpMultipartFormDataContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartFormDataContent> for ::windows::core::IInspectable {
    fn from(value: &HttpMultipartFormDataContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for super::super::Foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for super::super::Foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<IHttpContent>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<IHttpContent>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMultipartFormDataContent {}
unsafe impl ::core::marker::Sync for HttpMultipartFormDataContent {}
#[repr(C)]
#[doc = "*Required features: 'Web_Http', 'Foundation'*"]
#[cfg(feature = "Foundation")]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: ::core::option::Option<super::super::Foundation::IReference<u64>>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: ::core::option::Option<super::super::Foundation::IReference<u64>>,
    pub Retries: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for HttpProgress {
    fn clone(&self) -> Self {
        Self {
            Stage: self.Stage,
            BytesSent: self.BytesSent,
            TotalBytesToSend: self.TotalBytesToSend.clone(),
            BytesReceived: self.BytesReceived,
            TotalBytesToReceive: self.TotalBytesToReceive.clone(),
            Retries: self.Retries,
        }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for HttpProgress {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for HttpProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Web.Http.HttpProgress;enum(Windows.Web.Http.HttpProgressStage;i4);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u4)");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for HttpProgress {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for HttpProgress {
    fn eq(&self, other: &Self) -> bool {
        self.Stage == other.Stage && self.BytesSent == other.BytesSent && self.TotalBytesToSend == other.TotalBytesToSend && self.BytesReceived == other.BytesReceived && self.TotalBytesToReceive == other.TotalBytesToReceive && self.Retries == other.Retries
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for HttpProgress {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for HttpProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl ::core::marker::Copy for HttpProgressStage {}
impl ::core::clone::Clone for HttpProgressStage {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HttpProgressStage {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpProgressStage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProgressStage {}
unsafe impl ::windows::core::RuntimeType for HttpProgressStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpProgressStage;i4)");
}
impl ::windows::core::DefaultType for HttpProgressStage {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpRequestMessage(::windows::core::IUnknown);
impl HttpRequestMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpRequestMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Content(&self) -> ::windows::core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IHttpContent>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpRequestHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Method(&self) -> ::windows::core::Result<HttpMethod> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpMethod>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, HttpMethod>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequestUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TransportInformation(&self) -> ::windows::core::Result<HttpTransportInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpTransportInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, HttpMethod>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(method: Param0, uri: Param1) -> ::windows::core::Result<HttpRequestMessage> {
        Self::IHttpRequestMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), method.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi::<HttpRequestMessage>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpRequestMessageFactory<R, F: FnOnce(&IHttpRequestMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpRequestMessage, IHttpRequestMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestMessage {}
unsafe impl ::windows::core::RuntimeType for HttpRequestMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestMessage;{f5762b3c-74d4-4811-b5dc-9f8b4e2f9abf})");
}
unsafe impl ::windows::core::Interface for HttpRequestMessage {
    type Vtable = IHttpRequestMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5762b3c_74d4_4811_b5dc_9f8b4e2f9abf);
}
impl ::windows::core::RuntimeName for HttpRequestMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestMessage";
}
impl ::core::convert::From<HttpRequestMessage> for ::windows::core::IUnknown {
    fn from(value: HttpRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestMessage> for ::windows::core::IUnknown {
    fn from(value: &HttpRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestMessage> for ::windows::core::IInspectable {
    fn from(value: HttpRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestMessage> for ::windows::core::IInspectable {
    fn from(value: &HttpRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestMessage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestMessage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestMessage> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestMessage> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestMessage {}
unsafe impl ::core::marker::Sync for HttpRequestMessage {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpRequestResult(::windows::core::IUnknown);
impl HttpRequestResult {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestResult {}
unsafe impl ::windows::core::RuntimeType for HttpRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestResult;{6acf4da8-b5eb-4a35-a902-4217fbe820c5})");
}
unsafe impl ::windows::core::Interface for HttpRequestResult {
    type Vtable = IHttpRequestResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6acf4da8_b5eb_4a35_a902_4217fbe820c5);
}
impl ::windows::core::RuntimeName for HttpRequestResult {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestResult";
}
impl ::core::convert::From<HttpRequestResult> for ::windows::core::IUnknown {
    fn from(value: HttpRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestResult> for ::windows::core::IUnknown {
    fn from(value: &HttpRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestResult> for ::windows::core::IInspectable {
    fn from(value: HttpRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestResult> for ::windows::core::IInspectable {
    fn from(value: &HttpRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestResult> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestResult {}
unsafe impl ::core::marker::Sync for HttpRequestResult {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpResponseMessage(::windows::core::IUnknown);
impl HttpResponseMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpResponseMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Content(&self) -> ::windows::core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IHttpContent>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, IHttpContent>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpResponseHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpResponseHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn IsSuccessStatusCode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetReasonPhrase<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetRequestMessage<'a, Param0: ::windows::core::IntoParam<'a, HttpRequestMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Source(&self) -> ::windows::core::Result<HttpResponseMessageSource> {
        let this = self;
        unsafe {
            let mut result__: HttpResponseMessageSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessageSource>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetSource(&self, value: HttpResponseMessageSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn StatusCode(&self) -> ::windows::core::Result<HttpStatusCode> {
        let this = self;
        unsafe {
            let mut result__: HttpStatusCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpStatusCode>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetStatusCode(&self, value: HttpStatusCode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Version(&self) -> ::windows::core::Result<HttpVersion> {
        let this = self;
        unsafe {
            let mut result__: HttpVersion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpVersion>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn SetVersion(&self, value: HttpVersion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn EnsureSuccessStatusCode(&self) -> ::windows::core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn Create(statuscode: HttpStatusCode) -> ::windows::core::Result<HttpResponseMessage> {
        Self::IHttpResponseMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), statuscode, &mut result__).from_abi::<HttpResponseMessage>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpResponseMessageFactory<R, F: FnOnce(&IHttpResponseMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpResponseMessage, IHttpResponseMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpResponseMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpResponseMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseMessage {}
unsafe impl ::windows::core::RuntimeType for HttpResponseMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpResponseMessage;{fee200fb-8664-44e0-95d9-42696199bffc})");
}
unsafe impl ::windows::core::Interface for HttpResponseMessage {
    type Vtable = IHttpResponseMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee200fb_8664_44e0_95d9_42696199bffc);
}
impl ::windows::core::RuntimeName for HttpResponseMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpResponseMessage";
}
impl ::core::convert::From<HttpResponseMessage> for ::windows::core::IUnknown {
    fn from(value: HttpResponseMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseMessage> for ::windows::core::IUnknown {
    fn from(value: &HttpResponseMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpResponseMessage> for ::windows::core::IInspectable {
    fn from(value: HttpResponseMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseMessage> for ::windows::core::IInspectable {
    fn from(value: &HttpResponseMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpResponseMessage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpResponseMessage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpResponseMessage> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpResponseMessage> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpResponseMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpResponseMessage {}
unsafe impl ::core::marker::Sync for HttpResponseMessage {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: Self = Self(0i32);
    pub const Cache: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for HttpResponseMessageSource {}
impl ::core::clone::Clone for HttpResponseMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HttpResponseMessageSource {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpResponseMessageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseMessageSource {}
unsafe impl ::windows::core::RuntimeType for HttpResponseMessageSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpResponseMessageSource;i4)");
}
impl ::windows::core::DefaultType for HttpResponseMessageSource {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: Self = Self(0i32);
    pub const Continue: Self = Self(100i32);
    pub const SwitchingProtocols: Self = Self(101i32);
    pub const Processing: Self = Self(102i32);
    pub const Ok: Self = Self(200i32);
    pub const Created: Self = Self(201i32);
    pub const Accepted: Self = Self(202i32);
    pub const NonAuthoritativeInformation: Self = Self(203i32);
    pub const NoContent: Self = Self(204i32);
    pub const ResetContent: Self = Self(205i32);
    pub const PartialContent: Self = Self(206i32);
    pub const MultiStatus: Self = Self(207i32);
    pub const AlreadyReported: Self = Self(208i32);
    pub const IMUsed: Self = Self(226i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const PermanentRedirect: Self = Self(308i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const UnprocessableEntity: Self = Self(422i32);
    pub const Locked: Self = Self(423i32);
    pub const FailedDependency: Self = Self(424i32);
    pub const UpgradeRequired: Self = Self(426i32);
    pub const PreconditionRequired: Self = Self(428i32);
    pub const TooManyRequests: Self = Self(429i32);
    pub const RequestHeaderFieldsTooLarge: Self = Self(431i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
    pub const VariantAlsoNegotiates: Self = Self(506i32);
    pub const InsufficientStorage: Self = Self(507i32);
    pub const LoopDetected: Self = Self(508i32);
    pub const NotExtended: Self = Self(510i32);
    pub const NetworkAuthenticationRequired: Self = Self(511i32);
}
impl ::core::marker::Copy for HttpStatusCode {}
impl ::core::clone::Clone for HttpStatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HttpStatusCode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpStatusCode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStatusCode {}
unsafe impl ::windows::core::RuntimeType for HttpStatusCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpStatusCode;i4)");
}
impl ::windows::core::DefaultType for HttpStatusCode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpStreamContent(::windows::core::IUnknown);
impl HttpStreamContent {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromInputStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(content: Param0) -> ::windows::core::Result<HttpStreamContent> {
        Self::IHttpStreamContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<HttpStreamContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpStreamContentFactory<R, F: FnOnce(&IHttpStreamContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpStreamContent, IHttpStreamContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpStreamContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStreamContent {}
unsafe impl ::windows::core::RuntimeType for HttpStreamContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStreamContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpStreamContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpStreamContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStreamContent";
}
impl ::core::convert::From<HttpStreamContent> for ::windows::core::IUnknown {
    fn from(value: HttpStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStreamContent> for ::windows::core::IUnknown {
    fn from(value: &HttpStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpStreamContent> for ::windows::core::IInspectable {
    fn from(value: HttpStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStreamContent> for ::windows::core::IInspectable {
    fn from(value: &HttpStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpStreamContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpStreamContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStreamContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStreamContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpStreamContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpStreamContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpStreamContent {}
unsafe impl ::core::marker::Sync for HttpStreamContent {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpStringContent(::windows::core::IUnknown);
impl HttpStringContent {
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn CreateFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<HttpStringContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncoding<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0, encoding: super::super::Storage::Streams::UnicodeEncoding) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), content.into_param().abi(), encoding, &mut result__).from_abi::<HttpStringContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncodingAndMediaType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: Param2) -> ::windows::core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), content.into_param().abi(), encoding, mediatype.into_param().abi(), &mut result__).from_abi::<HttpStringContent>(result__)
        })
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpStringContentFactory<R, F: FnOnce(&IHttpStringContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HttpStringContent, IHttpStringContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpStringContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpStringContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStringContent {}
unsafe impl ::windows::core::RuntimeType for HttpStringContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStringContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
}
unsafe impl ::windows::core::Interface for HttpStringContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
impl ::windows::core::RuntimeName for HttpStringContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStringContent";
}
impl ::core::convert::From<HttpStringContent> for ::windows::core::IUnknown {
    fn from(value: HttpStringContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStringContent> for ::windows::core::IUnknown {
    fn from(value: &HttpStringContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpStringContent> for ::windows::core::IInspectable {
    fn from(value: HttpStringContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStringContent> for ::windows::core::IInspectable {
    fn from(value: &HttpStringContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpStringContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStringContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpStringContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStringContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStringContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStringContent> for IHttpContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpContent> for &HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpStringContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpStringContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpStringContent> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpStringContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpStringContent {}
unsafe impl ::core::marker::Sync for HttpStringContent {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpTransportInformation(::windows::core::IUnknown);
impl HttpTransportInformation {
    #[doc = "*Required features: 'Web_Http', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Networking_Sockets'*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: super::super::Networking::Sockets::SocketSslErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation_Collections', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpTransportInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransportInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransportInformation {}
unsafe impl ::windows::core::RuntimeType for HttpTransportInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpTransportInformation;{70127198-c6a7-4ed0-833a-83fd8b8f178d})");
}
unsafe impl ::windows::core::Interface for HttpTransportInformation {
    type Vtable = IHttpTransportInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70127198_c6a7_4ed0_833a_83fd8b8f178d);
}
impl ::windows::core::RuntimeName for HttpTransportInformation {
    const NAME: &'static str = "Windows.Web.Http.HttpTransportInformation";
}
impl ::core::convert::From<HttpTransportInformation> for ::windows::core::IUnknown {
    fn from(value: HttpTransportInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransportInformation> for ::windows::core::IUnknown {
    fn from(value: &HttpTransportInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransportInformation> for ::windows::core::IInspectable {
    fn from(value: HttpTransportInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransportInformation> for ::windows::core::IInspectable {
    fn from(value: &HttpTransportInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpTransportInformation> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransportInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransportInformation> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransportInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IStringable> for &HttpTransportInformation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransportInformation {}
unsafe impl ::core::marker::Sync for HttpTransportInformation {}
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: Self = Self(0i32);
    pub const Http10: Self = Self(1i32);
    pub const Http11: Self = Self(2i32);
    pub const Http20: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpVersion {}
impl ::core::clone::Clone for HttpVersion {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HttpVersion {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HttpVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpVersion {}
unsafe impl ::windows::core::RuntimeType for HttpVersion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpVersion;i4)");
}
impl ::windows::core::DefaultType for HttpVersion {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBufferContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpBufferContentFactory {
    type Vtable = IHttpBufferContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc20c193_c41f_4ff7_9123_6435736eadc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBufferContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, offset: u32, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClient {
    type Vtable = IHttpClientVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fda1151_3574_4880_a8ba_e6b1e0061f3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClientVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Web_Http_Headers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClient2 {
    type Vtable = IHttpClient2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdd83348_e8b7_4cec_b1b0_dc455fe72c92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpClientFactory {
    type Vtable = IHttpClientFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc30c4eca_e3fa_4f99_afb4_63cc65009462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClientFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Filters")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Filters"))] usize,
);
#[doc = "*Required features: 'Web_Http'*"]
#[repr(transparent)]
pub struct IHttpContent(::windows::core::IUnknown);
impl IHttpContent {
    #[doc = "*Required features: 'Web_Http', 'Web_Http_Headers'*"]
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http'*"]
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), length, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Web_Http', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IHttpContent> for ::windows::core::IInspectable {
    fn from(value: IHttpContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpContent> for ::windows::core::IInspectable {
    fn from(value: &IHttpContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHttpContent> for ::windows::core::IUnknown {
    fn from(value: IHttpContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpContent> for ::windows::core::IUnknown {
    fn from(value: &IHttpContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IHttpContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IHttpContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IHttpContent> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IHttpContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IHttpContent {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IHttpContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHttpContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpContent {}
unsafe impl ::windows::core::RuntimeType for IHttpContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6b14a441-fba7-4bd2-af0a-839de7c295da}");
}
unsafe impl ::windows::core::Interface for IHttpContent {
    type Vtable = IHttpContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookie(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookie {
    type Vtable = IHttpCookieVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f5488e2_cc2d_4779_86a7_88f10687d249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookieFactory {
    type Vtable = IHttpCookieFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0585a9_931c_4cd1_a96d_c21701785c5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domain: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpCookieManager {
    type Vtable = IHttpCookieManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a431780_cd4f_4e57_a84a_5b0a53d6bb96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows::core::RawPtr, thirdparty: bool, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpFormUrlEncodedContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpFormUrlEncodedContentFactory {
    type Vtable = IHttpFormUrlEncodedContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43f0138c_2f73_4302_b5f3_eae9238a5e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFormUrlEncodedContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetBufferResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetBufferResult {
    type Vtable = IHttpGetBufferResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d08e7c_e209_404e_9a49_742d8236fd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetBufferResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetInputStreamResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d63463_13aa_4ee0_be95_a0c39fe91203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetInputStreamResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetStringResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpGetStringResult {
    type Vtable = IHttpGetStringResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bac466d_8509_4775_b16d_8953f47a7f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetStringResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethod(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethod {
    type Vtable = IHttpMethodVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728d4022_700d_4fe0_afa5_40299c58dbfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethodFactory {
    type Vtable = IHttpMethodFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c51d10d_36d7_40f8_a86d_e759caf2f83f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMethodStatics {
    type Vtable = IHttpMethodStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d171f0_d99a_4153_8dc6_d68cc4cce317);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartContent {
    type Vtable = IHttpMultipartContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf916aff_9926_4ac9_aaf1_e0d04ef09bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartContentFactory {
    type Vtable = IHttpMultipartContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7eb42e62_0222_4f20_b372_47d5db5d33b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartFormDataContent {
    type Vtable = IHttpMultipartFormDataContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d337e2_e967_4624_b6d1_cf74604a4a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpMultipartFormDataContentFactory {
    type Vtable = IHttpMultipartFormDataContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa04d7311_5017_4622_93a8_49b24a4fcbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundary: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestMessage {
    type Vtable = IHttpRequestMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5762b3c_74d4_4811_b5dc_9f8b4e2f9abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestMessageFactory {
    type Vtable = IHttpRequestMessageFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bac994e_3886_412e_aec3_52ec7f25616f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessageFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpRequestResult {
    type Vtable = IHttpRequestResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6acf4da8_b5eb_4a35_a902_4217fbe820c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpResponseMessage {
    type Vtable = IHttpResponseMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee200fb_8664_44e0_95d9_42696199bffc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpResponseMessageSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpResponseMessageSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpStatusCode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpStatusCode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpVersion) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpVersion) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpResponseMessageFactory {
    type Vtable = IHttpResponseMessageFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52a8af99_f095_43da_b60f_7cfc2bc7ea2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessageFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statuscode: HttpStatusCode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStreamContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpStreamContentFactory {
    type Vtable = IHttpStreamContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3e64d9d_f725_407e_942f_0eda189809f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStreamContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStringContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpStringContentFactory {
    type Vtable = IHttpStringContentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46649d5b_2e93_48eb_8e61_19677878e57f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStringContentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: super::super::Storage::Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransportInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHttpTransportInformation {
    type Vtable = IHttpTransportInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70127198_c6a7_4ed0_833a_83fd8b8f178d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransportInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
);
