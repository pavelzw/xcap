mod capture;
#[cfg(all(not(feature = "wgc"), feature = "video-recorder"))]
mod dxgi_video_recorder;
#[cfg(not(feature = "wgc"))]
mod gdi;
mod utils;
#[cfg(feature = "wgc")]
mod wgc;
#[cfg(all(feature = "wgc", feature = "video-recorder"))]
mod wgc_video_recorder;

pub mod impl_monitor;
#[cfg(feature = "video-recorder")]
pub mod impl_video_recorder;
pub mod impl_window;
