//! SigmaOS Software Manager
//! Plugin-based package management with multiple backend support, transaction handling, and dependency resolution.

pub mod config;
pub mod error;
pub mod service;
pub mod package;
pub mod installer;
pub mod cache;
pub mod software_manager;

pub use software_manager::{SoftwareManager, PackageBackend, PackageInfo, PackageCategory, InstallError};
pub use package::{Package, PackageRepository, Version, DependencyResolver};
pub use installer::{PackageInstaller, InstallTransaction};
pub use cache::{PackageCache, CacheEntry};
pub use config::Config;
pub use error::{ComponentError, Result};
pub use service::Service;
