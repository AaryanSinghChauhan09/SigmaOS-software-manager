//! SigmaOS Software Manager
//! Plugin-based package management with multiple backend support

pub mod software_manager;

pub use software_manager::SoftwareManager;
pub use software_manager::PackageBackend;
pub use software_manager::PackageInfo;
