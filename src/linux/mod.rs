mod capture;
pub mod utils;
mod wayland_capture;
#[cfg(feature = "video-recorder")]
mod wayland_video_recorder;
pub mod xorg_capture;
#[cfg(feature = "video-recorder")]
mod xorg_video_recorder;

pub mod impl_monitor;
#[cfg(feature = "video-recorder")]
pub mod impl_video_recorder;
pub mod impl_window;
