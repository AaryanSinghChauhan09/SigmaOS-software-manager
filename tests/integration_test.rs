use anyhow::Result;
use sigmaos_package_component::{
    Config, Service, SoftwareManager, PackageRepository, Package, Version,
    PackageInstaller, PackageBackend, PackageInfo, PackageCategory, InstallError,
};
use std::sync::Mutex;
use std::collections::HashSet;

struct MockBackend {
    installed: Mutex<HashSet<String>>,
}

impl MockBackend {
    fn new() -> Self {
        Self {
            installed: Mutex::new(HashSet::new()),
        }
    }
}

impl PackageBackend for MockBackend {
    fn name(&self) -> &'static str {
        "mock"
    }

    fn search(&self, query: &str) -> Vec<PackageInfo> {
        if "firefox".contains(query) {
            vec![PackageInfo {
                name: "firefox".to_string(),
                version: "128.0".to_string(),
                description: "Web browser".to_string(),
                category: PackageCategory::Internet,
                size_kb: 95000,
                icon_name: "firefox".to_string(),
                backend: "mock",
                rating: 4.8,
            }]
        } else {
            Vec::new()
        }
    }

    fn install(&self, pkg: &PackageInfo) -> Result<(), InstallError> {
        self.installed.lock().unwrap().insert(pkg.name.clone());
        Ok(())
    }

    fn remove(&self, pkg_name: &str) -> Result<(), InstallError> {
        self.installed.lock().unwrap().remove(pkg_name);
        Ok(())
    }

    fn is_installed(&self, pkg_name: &str) -> bool {
        self.installed.lock().unwrap().contains(pkg_name)
    }

    fn list_installed(&self) -> Vec<PackageInfo> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_manager_search_and_install_aaa() {
        // Arrange
        let mut mgr = SoftwareManager::new();
        mgr.register_backend(Box::new(MockBackend::new()));

        // Act - Search
        let results = mgr.search("firefox");

        // Assert
        assert!(!results.is_empty());
        let pkg = &results[0];
        assert_eq!(pkg.name, "firefox");

        // Act - Install
        let install_res = mgr.install(pkg);

        // Assert
        assert!(install_res.is_ok());
        assert!(mgr.is_installed("firefox"));
    }

    #[test]
    fn test_package_repository_and_installer_aaa() -> Result<()> {
        // Arrange
        let mut repo = PackageRepository::new();
        let pkg = Package::new(
            "core-utils".to_string(),
            Version::new(2, 1, 0),
            "Core utility binaries".to_string(),
        );
        repo.add_package(pkg);

        // Act
        let retrieved = repo.get_package("core-utils");

        // Assert
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().version, Version::new(2, 1, 0));

        // Arrange - Installer
        let mut installer = PackageInstaller::default();
        let mut tx = installer.create_transaction("core-utils".to_string());

        // Act
        installer.install(&mut tx).map_err(|e| anyhow::anyhow!(e))?;

        // Assert
        assert!(tx.is_complete());
        Ok(())
    }

    #[test]
    fn test_service_lifecycle_aaa() -> Result<()> {
        // Arrange
        let config = Config::default();
        let mut service = Service::new(config);
        assert!(!service.is_running());

        // Act & Assert - Start
        service.start()?;
        assert!(service.is_running());

        // Act & Assert - Redundant start fails cleanly
        assert!(service.start().is_err());

        // Act & Assert - Stop
        service.stop()?;
        assert!(!service.is_running());

        // Act & Assert - Redundant stop fails cleanly
        assert!(service.stop().is_err());

        Ok(())
    }
}
