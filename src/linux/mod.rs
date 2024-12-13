mod capture;
mod utils;
#[cfg(feature = "wayland")]
mod wayland_capture;
mod xorg_capture;

pub mod impl_monitor;
pub mod impl_video_recorder;
pub mod impl_window;
