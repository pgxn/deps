pub mod repology;

use async_trait::async_trait;
use repology::RepologyClient;

use crate::{
    error::{Error, Result},
    operating_system::OperatingSystem,
};

#[async_trait]
pub trait PackageType: Send + Sync {
    /// The name of this package type, as seen
    fn name(&self) -> &'static str;

    /// Obtain the installation command for the given package on this package type
    /// and operating system.
    async fn resolve_package_for_operating_system(
        &self,
        package_name: &str,
        os: OperatingSystem,
    ) -> Result<String>;

    /// Obtain the installation command for the given package on this package type.
    /// Auto-detects the current operating system.
    async fn resolve_package(&self, package_name: &str) -> Result<String> {
        let os = OperatingSystem::detect()?;

        self.resolve_package_for_operating_system(package_name, os)
            .await
    }
}

/// PGXN package type
pub struct Pgxn;

/// Package type for PostgreSQL built-in extensions
pub struct Postgres;

/// Package type for generic dependencies
pub struct Generic;

/// Package type for Cargo dependencies
pub struct Cargo;

#[async_trait]
impl PackageType for Pgxn {
    fn name(&self) -> &'static str {
        "pgxn"
    }

    async fn resolve_package_for_operating_system(
        &self,
        package_name: &str,
        _: OperatingSystem,
    ) -> Result<String> {
        Ok(format!("pgxn install {package_name}"))
    }
}

#[async_trait]
impl PackageType for Postgres {
    fn name(&self) -> &'static str {
        "postgres"
    }

    async fn resolve_package_for_operating_system(
        &self,
        package_name: &str,
        _: OperatingSystem,
    ) -> Result<String> {
        Ok(format!("pgxn install {package_name}"))
    }
}

#[async_trait]
impl PackageType for Generic {
    fn name(&self) -> &'static str {
        "generic"
    }

    async fn resolve_package_for_operating_system(
        &self,
        package_name: &str,
        os: OperatingSystem,
    ) -> Result<String> {
        let client = RepologyClient::new();

        // All matching projects for the given package name
        let projects = client.get_projects(package_name, os).await?;

        let package_name = projects
            .into_iter()
            .next()
            .and_then(|project| project.srcname)
            .ok_or_else(|| Error::PackageNotFound(package_name.into()))?;

        let install_command = os
            .package_managers()
            .iter()
            .map(|package_manager| package_manager.install(&package_name))
            .next()
            .ok_or_else(|| Error::PackageNotFound(package_name))?;

        Ok(install_command)
    }
}

#[async_trait]
impl PackageType for Cargo {
    fn name(&self) -> &'static str {
        "cargo"
    }

    async fn resolve_package_for_operating_system(
        &self,
        package_name: &str,
        _: OperatingSystem,
    ) -> Result<String> {
        Ok(format!("cargo install {package_name}"))
    }
}
