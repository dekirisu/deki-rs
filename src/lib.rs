pub use deki_core as core;
pub use deki_macros as macros;

#[cfg(feature = "derive_more")]
pub use deki_core::derive_more;

#[cfg(feature = "lerp")]
pub use deki_core::lerp;

#[cfg(feature = "random")]
pub use deki_core::random;

#[cfg(feature = "proc")]
pub use deki_proc as proc;
