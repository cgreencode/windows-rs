#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows_sys::core::HRESULT;
    pub fn MapStorageSCode(stgscode: i32) -> i32;
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OpenIMsgOnIStg(
        lpmsgsess: *mut _MSGSESS,
        lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER,
        lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE,
        lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER,
        lpmalloc: super::super::System::Com::IMalloc,
        lpmapisup: *mut ::core::ffi::c_void,
        lpstg: super::super::System::Com::StructuredStorage::IStorage,
        lpfmsgcallrelease: *mut MSGCALLRELEASE,
        ulcallerdata: u32,
        ulflags: u32,
        lppmsg: *mut super::super::System::AddressBook::IMessage,
    ) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenIMsgSession(lpmalloc: super::super::System::Com::IMalloc, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows_sys::core::HRESULT;
}
pub const BlockRange: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186599, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const BlockRangeList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186600, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const BootOptions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904974, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3171631974, data2: 36355, data3: 4562, data4: [148, 246, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_DSN: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 582309681,
    data2: 62968,
    data3: 19747,
    data4: [189, 143, 135, 181, 35, 113, 167, 58],
};
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2231318154,
    data2: 64179,
    data3: 17367,
    data4: [188, 223, 105, 44, 91, 70, 230, 177],
};
pub const CATID_SMTP_LOG: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2479924536,
    data2: 11294,
    data3: 19304,
    data4: [167, 201, 215, 58, 138, 166, 238, 151],
};
pub const CATID_SMTP_MAXMSGSIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3958462942, data2: 42622, data3: 4562, data4: [148, 247, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_MSGTRACKLOG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3336524458, data2: 32176, data3: 4562, data4: [148, 244, 0, 192, 79, 121, 241, 214] };
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653650, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653645, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_MESSAGE_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653648, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653649, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653646, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_END: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653651, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_ON_SESSION_START: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133653647, data2: 3422, data3: 4562, data4: [170, 104, 0, 192, 79, 163, 91, 130] };
pub const CATID_SMTP_STORE_DRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1494702160, data2: 58675, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2516734627, data2: 2618, data3: 4562, data4: [158, 0, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1987155540, data2: 1446, data3: 4562, data4: [157, 253, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2746022669, data2: 33791, data3: 4562, data4: [158, 20, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 674509001, data2: 6224, data3: 4562, data4: [158, 3, 0, 192, 79, 163, 34, 186] };
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4282165795, data2: 185, data3: 4562, data4: [157, 251, 0, 192, 79, 163, 34, 186] };
pub const CLSID_SmtpCat: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2990290359, data2: 37401, data3: 4562, data4: [158, 23, 0, 192, 79, 163, 34, 186] };
#[repr(transparent)]
pub struct DDiscFormat2DataEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DDiscFormat2DataEvents {}
impl ::core::clone::Clone for DDiscFormat2DataEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DDiscFormat2EraseEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DDiscFormat2EraseEvents {}
impl ::core::clone::Clone for DDiscFormat2EraseEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DDiscFormat2RawCDEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DDiscFormat2RawCDEvents {}
impl ::core::clone::Clone for DDiscFormat2RawCDEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DDiscFormat2TrackAtOnceEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DDiscFormat2TrackAtOnceEvents {}
impl ::core::clone::Clone for DDiscFormat2TrackAtOnceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DDiscMaster2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DDiscMaster2Events {}
impl ::core::clone::Clone for DDiscMaster2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DFileSystemImageEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DFileSystemImageEvents {}
impl ::core::clone::Clone for DFileSystemImageEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DFileSystemImageImportEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DFileSystemImageImportEvents {}
impl ::core::clone::Clone for DFileSystemImageImportEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
impl ::core::marker::Copy for DISC_RECORDER_STATE_FLAGS {}
impl ::core::clone::Clone for DISC_RECORDER_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[repr(transparent)]
pub struct DWriteEngine2Events(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DWriteEngine2Events {}
impl ::core::clone::Clone for DWriteEngine2Events {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmulationType(pub i32);
pub const EmulationNone: EmulationType = EmulationType(0i32);
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
impl ::core::marker::Copy for EmulationType {}
impl ::core::clone::Clone for EmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EnumFsiItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904966, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const EnumProgressItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904970, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FileSystemImageResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904972, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FsiDirectoryItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904968, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const FsiFileItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904967, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
#[repr(transparent)]
pub struct FsiFileSystems(pub i32);
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
impl ::core::marker::Copy for FsiFileSystems {}
impl ::core::clone::Clone for FsiFileSystems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FsiItemType(pub i32);
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
impl ::core::marker::Copy for FsiItemType {}
impl ::core::clone::Clone for FsiItemType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FsiNamedStreams: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3333880045,
    data2: 27929,
    data3: 17588,
    data4: [181, 57, 177, 89, 183, 147, 163, 45],
};
pub const FsiStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904973, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const GUID_SMTPSVC_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 456918630, data2: 58480, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
pub const GUID_SMTP_SOURCE_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4217750748, data2: 58472, data3: 4561, data4: [170, 103, 0, 192, 79, 163, 69, 246] };
#[repr(transparent)]
pub struct IBlockRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockRange {}
impl ::core::clone::Clone for IBlockRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlockRangeList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockRangeList {}
impl ::core::clone::Clone for IBlockRangeList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBootOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBootOptions {}
impl ::core::clone::Clone for IBootOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBurnVerification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBurnVerification {}
impl ::core::clone::Clone for IBurnVerification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2 {}
impl ::core::clone::Clone for IDiscFormat2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2Data(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2Data {}
impl ::core::clone::Clone for IDiscFormat2Data {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2DataEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2DataEventArgs {}
impl ::core::clone::Clone for IDiscFormat2DataEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2Erase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2Erase {}
impl ::core::clone::Clone for IDiscFormat2Erase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2RawCD(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2RawCD {}
impl ::core::clone::Clone for IDiscFormat2RawCD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2RawCDEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2RawCDEventArgs {}
impl ::core::clone::Clone for IDiscFormat2RawCDEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnce(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2TrackAtOnce {}
impl ::core::clone::Clone for IDiscFormat2TrackAtOnce {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnceEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscFormat2TrackAtOnceEventArgs {}
impl ::core::clone::Clone for IDiscFormat2TrackAtOnceEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscMaster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscMaster {}
impl ::core::clone::Clone for IDiscMaster {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscMaster2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscMaster2 {}
impl ::core::clone::Clone for IDiscMaster2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscMasterProgressEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscMasterProgressEvents {}
impl ::core::clone::Clone for IDiscMasterProgressEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscRecorder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscRecorder {}
impl ::core::clone::Clone for IDiscRecorder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscRecorder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscRecorder2 {}
impl ::core::clone::Clone for IDiscRecorder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiscRecorder2Ex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiscRecorder2Ex {}
impl ::core::clone::Clone for IDiscRecorder2Ex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumDiscMasterFormats(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumDiscMasterFormats {}
impl ::core::clone::Clone for IEnumDiscMasterFormats {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumDiscRecorders(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumDiscRecorders {}
impl ::core::clone::Clone for IEnumDiscRecorders {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumFsiItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumFsiItems {}
impl ::core::clone::Clone for IEnumFsiItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumProgressItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumProgressItems {}
impl ::core::clone::Clone for IEnumProgressItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSystemImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSystemImage {}
impl ::core::clone::Clone for IFileSystemImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSystemImage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSystemImage2 {}
impl ::core::clone::Clone for IFileSystemImage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSystemImage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSystemImage3 {}
impl ::core::clone::Clone for IFileSystemImage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSystemImageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSystemImageResult {}
impl ::core::clone::Clone for IFileSystemImageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileSystemImageResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileSystemImageResult2 {}
impl ::core::clone::Clone for IFileSystemImageResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiDirectoryItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiDirectoryItem {}
impl ::core::clone::Clone for IFsiDirectoryItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiDirectoryItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiDirectoryItem2 {}
impl ::core::clone::Clone for IFsiDirectoryItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiFileItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiFileItem {}
impl ::core::clone::Clone for IFsiFileItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiFileItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiFileItem2 {}
impl ::core::clone::Clone for IFsiFileItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiItem {}
impl ::core::clone::Clone for IFsiItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFsiNamedStreams(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFsiNamedStreams {}
impl ::core::clone::Clone for IFsiNamedStreams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsoImageManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsoImageManager {}
impl ::core::clone::Clone for IIsoImageManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJolietDiscMaster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJolietDiscMaster {}
impl ::core::clone::Clone for IJolietDiscMaster {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
pub const IMAPILib2_MajorVersion: u32 = 1u32;
pub const IMAPILib2_MinorVersion: u32 = 0u32;
#[repr(transparent)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
impl ::core::marker::Copy for IMAPI_BURN_VERIFICATION_LEVEL {}
impl ::core::clone::Clone for IMAPI_BURN_VERIFICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_CD_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_CD_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
impl ::core::marker::Copy for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {}
impl ::core::clone::Clone for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IMAPI_E_ALREADYOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220958i32 as _);
pub const IMAPI_E_BADJOLIETNAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220963i32 as _);
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220946i32 as _);
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220948i32 as _);
pub const IMAPI_E_COMPRESSEDSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220952i32 as _);
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220972i32 as _);
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220975i32 as _);
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220974i32 as _);
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220973i32 as _);
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220954i32 as _);
pub const IMAPI_E_DISCFULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220964i32 as _);
pub const IMAPI_E_DISCINFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220967i32 as _);
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220951i32 as _);
pub const IMAPI_E_FILEACCESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220968i32 as _);
pub const IMAPI_E_FILEEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220956i32 as _);
pub const IMAPI_E_FILESYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220969i32 as _);
pub const IMAPI_E_GENERIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220978i32 as _);
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220970i32 as _);
pub const IMAPI_E_INITIALIZE_WRITE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220971i32 as _);
pub const IMAPI_E_INVALIDIMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220962i32 as _);
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220953i32 as _);
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220976i32 as _);
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const IMAPI_E_NOACTIVEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220961i32 as _);
pub const IMAPI_E_NOACTIVERECORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220960i32 as _);
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220950i32 as _);
pub const IMAPI_E_NOTINITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220980i32 as _);
pub const IMAPI_E_NOTOPENED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220981i32 as _);
pub const IMAPI_E_REMOVABLESTASH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220949i32 as _);
pub const IMAPI_E_STASHINUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220955i32 as _);
pub const IMAPI_E_TRACKNOTOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220966i32 as _);
pub const IMAPI_E_TRACKOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220965i32 as _);
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220947i32 as _);
pub const IMAPI_E_USERABORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220979i32 as _);
pub const IMAPI_E_WRONGDISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220957i32 as _);
pub const IMAPI_E_WRONGFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220959i32 as _);
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
impl ::core::marker::Copy for IMAPI_FEATURE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_FEATURE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_MEDIA_STATE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_TAO_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_MEDIA_PHYSICAL_TYPE {}
impl ::core::clone::Clone for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
impl ::core::marker::Copy for IMAPI_MEDIA_WRITE_PROTECT_STATE {}
impl ::core::clone::Clone for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
impl ::core::marker::Copy for IMAPI_MODE_PAGE_REQUEST_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_MODE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMAPI_PROFILE_TYPE {}
impl ::core::clone::Clone for IMAPI_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
impl ::core::marker::Copy for IMAPI_READ_TRACK_ADDRESS_TYPE {}
impl ::core::clone::Clone for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262657i32 as _);
pub const IMAPI_S_PROPERTIESIGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
#[repr(transparent)]
pub struct IMMPID_CPV_ENUM(pub i32);
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
impl ::core::marker::Copy for IMMPID_CPV_ENUM {}
impl ::core::clone::Clone for IMMPID_CPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMMPID_MPV_ENUM {}
impl ::core::clone::Clone for IMMPID_MPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMMPID_MP_ENUM {}
impl ::core::clone::Clone for IMMPID_MP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMMPID_NMP_ENUM {}
impl ::core::clone::Clone for IMMPID_NMP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMMPID_RPV_ENUM(pub i32);
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
impl ::core::marker::Copy for IMMPID_RPV_ENUM {}
impl ::core::clone::Clone for IMMPID_RPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IMMPID_RP_ENUM {}
impl ::core::clone::Clone for IMMPID_RP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::clone::Clone for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultisession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultisession {}
impl ::core::clone::Clone for IMultisession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultisessionRandomWrite(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultisessionRandomWrite {}
impl ::core::clone::Clone for IMultisessionRandomWrite {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultisessionSequential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultisessionSequential {}
impl ::core::clone::Clone for IMultisessionSequential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultisessionSequential2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultisessionSequential2 {}
impl ::core::clone::Clone for IMultisessionSequential2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressItem {}
impl ::core::clone::Clone for IProgressItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressItems {}
impl ::core::clone::Clone for IProgressItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRawCDImageCreator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRawCDImageCreator {}
impl ::core::clone::Clone for IRawCDImageCreator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRawCDImageTrackInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRawCDImageTrackInfo {}
impl ::core::clone::Clone for IRawCDImageTrackInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRedbookDiscMaster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRedbookDiscMaster {}
impl ::core::clone::Clone for IRedbookDiscMaster {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamConcatenate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamConcatenate {}
impl ::core::clone::Clone for IStreamConcatenate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamInterleave(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamInterleave {}
impl ::core::clone::Clone for IStreamInterleave {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStreamPseudoRandomBased(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStreamPseudoRandomBased {}
impl ::core::clone::Clone for IStreamPseudoRandomBased {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWriteEngine2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWriteEngine2 {}
impl ::core::clone::Clone for IWriteEngine2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWriteEngine2EventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWriteEngine2EventArgs {}
impl ::core::clone::Clone for IWriteEngine2EventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWriteSpeedDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWriteSpeedDescriptor {}
impl ::core::clone::Clone for IWriteSpeedDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MEDIA_FLAGS(pub i32);
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
impl ::core::marker::Copy for MEDIA_FLAGS {}
impl ::core::clone::Clone for MEDIA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MEDIA_TYPES(pub i32);
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
impl ::core::marker::Copy for MEDIA_TYPES {}
impl ::core::clone::Clone for MEDIA_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
pub const MPV_WRITE_CONTENT: u32 = 2u32;
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
pub const MP_STATUS_RETRY: u32 = 1u32;
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376569955, data2: 20901, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
pub const MSDiscRecorderObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376569953, data2: 20901, data3: 4563, data4: [145, 68, 0, 16, 75, 161, 28, 94] };
pub const MSEnumDiscRecordersObj: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2315474554,
    data2: 25547,
    data3: 19368,
    data4: [186, 246, 82, 17, 152, 22, 209, 239],
};
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = unsafe extern "system" fn(ulcallerdata: u32, lpmessage: super::super::System::AddressBook::IMessage);
pub const MsftDiscFormat2Data: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801514, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2Erase: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801515, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2RawCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801512, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscFormat2TrackAtOnce: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801513, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscMaster2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801518, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftDiscRecorder2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801517, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftFileSystemImage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904965, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const MsftIsoImageManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3471719266,
    data2: 36694,
    data3: 16470,
    data4: [134, 155, 239, 22, 145, 126, 62, 252],
};
pub const MsftMultisessionRandomWrite: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3037186596, data2: 8708, data3: 4573, data4: [150, 106, 0, 26, 160, 27, 188, 88] };
pub const MsftMultisessionSequential: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801506, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftRawCDImageCreator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630732129, data2: 40293, data3: 18894, data4: [179, 53, 64, 99, 13, 144, 18, 39] };
pub const MsftStreamConcatenate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801509, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamInterleave: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801508, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamPrng001: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801510, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftStreamZero: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801511, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftWriteEngine2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801516, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const MsftWriteSpeedDescriptor: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 657801507, data2: 32612, data3: 23311, data4: [143, 0, 93, 119, 175, 190, 38, 30] };
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
pub const NMP_PROCESS_POST: u32 = 1u32;
#[repr(transparent)]
pub struct PlatformId(pub i32);
pub const PlatformX86: PlatformId = PlatformId(0i32);
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
pub const PlatformMac: PlatformId = PlatformId(2i32);
pub const PlatformEFI: PlatformId = PlatformId(239i32);
impl ::core::marker::Copy for PlatformId {}
impl ::core::clone::Clone for PlatformId {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ProgressItem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904971, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
pub const ProgressItems: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 747904969, data2: 38747, data3: 22974, data4: [169, 96, 154, 42, 38, 40, 83, 165] };
#[repr(transparent)]
pub struct RECORDER_TYPES(pub i32);
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
impl ::core::marker::Copy for RECORDER_TYPES {}
impl ::core::clone::Clone for RECORDER_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RP_DELIVERED: u32 = 272u32;
pub const RP_DSN_HANDLED: u32 = 64u32;
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
pub const RP_ENPANDED: u32 = 8208u32;
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
pub const RP_EXPANDED: u32 = 8208u32;
pub const RP_FAILED: u32 = 2096u32;
pub const RP_GENERAL_FAILURE: u32 = 32u32;
pub const RP_HANDLED: u32 = 16u32;
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
pub const RP_UNRESOLVED: u32 = 4144u32;
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
#[repr(C)]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl ::core::marker::Copy for SPropAttrArray {}
impl ::core::clone::Clone for SPropAttrArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _MSGSESS(pub u8);
pub const tagIMMPID_CPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2728880938, data2: 58669, data3: 4561, data4: [170, 100, 0, 192, 79, 163, 91, 130] };
#[repr(C)]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *mut ::windows_sys::core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl ::core::marker::Copy for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::clone::Clone for tagIMMPID_GUIDLIST_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const tagIMMPID_MPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3420886790, data2: 51645, data3: 4561, data4: [159, 242, 0, 192, 79, 163, 115, 72] };
pub const tagIMMPID_MP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322456816, data2: 46020, data3: 4561, data4: [170, 146, 0, 170, 0, 107, 200, 11] };
pub const tagIMMPID_NMP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1949542826, data2: 8418, data3: 4562, data4: [148, 214, 0, 192, 79, 163, 121, 241] };
pub const tagIMMPID_RPV_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2045255753, data2: 54048, data3: 4561, data4: [159, 244, 0, 192, 79, 163, 115, 72] };
pub const tagIMMPID_RP_STRUCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2045255752, data2: 54048, data3: 4561, data4: [159, 244, 0, 192, 79, 163, 115, 72] };
