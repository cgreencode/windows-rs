#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioBuffer {}
impl ::core::clone::Clone for AudioBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioBufferAccessMode {}
impl ::core::clone::Clone for AudioBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioFrame {}
impl ::core::clone::Clone for AudioFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: Self = Self(0i32);
    pub const Raw: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioProcessing {}
impl ::core::clone::Clone for AudioProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoRepeatModeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoRepeatModeChangeRequestedEventArgs {}
impl ::core::clone::Clone for AutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioBuffer {}
impl ::core::clone::Clone for IAudioBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrame {}
impl ::core::clone::Clone for IAudioFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameFactory {}
impl ::core::clone::Clone for IAudioFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoRepeatModeChangeRequestedEventArgs {}
impl ::core::clone::Clone for IAutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageDisplayProperties {}
impl ::core::clone::Clone for IImageDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaControl {}
impl ::core::clone::Clone for IMediaControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaExtension {}
impl ::core::clone::Clone for IMediaExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaExtensionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaExtensionManager {}
impl ::core::clone::Clone for IMediaExtensionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaExtensionManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaExtensionManager2 {}
impl ::core::clone::Clone for IMediaExtensionManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrame {}
impl ::core::clone::Clone for IMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaMarker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaMarker {}
impl ::core::clone::Clone for IMediaMarker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaMarkerTypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaMarkerTypesStatics {}
impl ::core::clone::Clone for IMediaMarkerTypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaMarkers(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaMarkers {}
impl ::core::clone::Clone for IMediaMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProcessingTriggerDetails {}
impl ::core::clone::Clone for IMediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTimelineController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTimelineController {}
impl ::core::clone::Clone for IMediaTimelineController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTimelineController2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTimelineController2 {}
impl ::core::clone::Clone for IMediaTimelineController2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTimelineControllerFailedEventArgs {}
impl ::core::clone::Clone for IMediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMusicDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMusicDisplayProperties {}
impl ::core::clone::Clone for IMusicDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMusicDisplayProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMusicDisplayProperties2 {}
impl ::core::clone::Clone for IMusicDisplayProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMusicDisplayProperties3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMusicDisplayProperties3 {}
impl ::core::clone::Clone for IMusicDisplayProperties3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackPositionChangeRequestedEventArgs {}
impl ::core::clone::Clone for IPlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackRateChangeRequestedEventArgs {}
impl ::core::clone::Clone for IPlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShuffleEnabledChangeRequestedEventArgs {}
impl ::core::clone::Clone for IShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControls {}
impl ::core::clone::Clone for ISystemMediaTransportControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControls2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControls2 {}
impl ::core::clone::Clone for ISystemMediaTransportControls2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControlsButtonPressedEventArgs {}
impl ::core::clone::Clone for ISystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControlsDisplayUpdater {}
impl ::core::clone::Clone for ISystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControlsPropertyChangedEventArgs {}
impl ::core::clone::Clone for ISystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControlsStatics {}
impl ::core::clone::Clone for ISystemMediaTransportControlsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMediaTransportControlsTimelineProperties {}
impl ::core::clone::Clone for ISystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoDisplayProperties {}
impl ::core::clone::Clone for IVideoDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoDisplayProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoDisplayProperties2 {}
impl ::core::clone::Clone for IVideoDisplayProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoEffectsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoEffectsStatics {}
impl ::core::clone::Clone for IVideoEffectsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrame {}
impl ::core::clone::Clone for IVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrame2 {}
impl ::core::clone::Clone for IVideoFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrameFactory {}
impl ::core::clone::Clone for IVideoFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrameStatics {}
impl ::core::clone::Clone for IVideoFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageDisplayProperties {}
impl ::core::clone::Clone for ImageDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaExtensionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaExtensionManager {}
impl ::core::clone::Clone for MediaExtensionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: Self = Self(0i32);
    pub const Track: Self = Self(1i32);
    pub const List: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlaybackAutoRepeatMode {}
impl ::core::clone::Clone for MediaPlaybackAutoRepeatMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Changing: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackStatus {}
impl ::core::clone::Clone for MediaPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: Self = Self(0i32);
    pub const Music: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackType {}
impl ::core::clone::Clone for MediaPlaybackType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaProcessingTriggerDetails {}
impl ::core::clone::Clone for MediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct MediaTimeRange {
    pub Start: super::Foundation::TimeSpan,
    pub End: super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MediaTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MediaTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTimelineController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTimelineController {}
impl ::core::clone::Clone for MediaTimelineController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTimelineControllerFailedEventArgs {}
impl ::core::clone::Clone for MediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Stalled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaTimelineControllerState {}
impl ::core::clone::Clone for MediaTimelineControllerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MusicDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MusicDisplayProperties {}
impl ::core::clone::Clone for MusicDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackPositionChangeRequestedEventArgs {}
impl ::core::clone::Clone for PlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackRateChangeRequestedEventArgs {}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShuffleEnabledChangeRequestedEventArgs {}
impl ::core::clone::Clone for ShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
}
impl ::core::marker::Copy for SoundLevel {}
impl ::core::clone::Clone for SoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaTransportControls {}
impl ::core::clone::Clone for SystemMediaTransportControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsButton {}
impl ::core::clone::Clone for SystemMediaTransportControlsButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaTransportControlsButtonPressedEventArgs {}
impl ::core::clone::Clone for SystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaTransportControlsDisplayUpdater {}
impl ::core::clone::Clone for SystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: Self = Self(0i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsProperty {}
impl ::core::clone::Clone for SystemMediaTransportControlsProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaTransportControlsPropertyChangedEventArgs {}
impl ::core::clone::Clone for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMediaTransportControlsTimelineProperties {}
impl ::core::clone::Clone for SystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoDisplayProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoDisplayProperties {}
impl ::core::clone::Clone for VideoDisplayProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoFrame {}
impl ::core::clone::Clone for VideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
