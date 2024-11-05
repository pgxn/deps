//!

pub mod repology;

use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait PackageType: Send + Sync {
    /// The name of this package type, as seen
    fn name(&self) -> &'static str;

    /// Obtain the installation command for the given package on this package type
    async fn resolve_package(&self, package_name: &str) -> Result<String>;
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

    async fn resolve_package(&self, package_name: &str) -> Result<String> {
        Ok(format!("pgxn install {package_name}"))
    }
}

#[async_trait]
impl PackageType for Postgres {
    fn name(&self) -> &'static str {
        "postgres"
    }

    async fn resolve_package(&self, package_name: &str) -> Result<String> {
        Ok(format!("pgxn install {package_name}"))
    }
}

#[async_trait]
impl PackageType for Generic {
    fn name(&self) -> &'static str {
        "generic"
    }

    async fn resolve_package(&self, _: &str) -> Result<String> {
        todo!()
    }
}

#[async_trait]
impl PackageType for Cargo {
    fn name(&self) -> &'static str {
        "cargo"
    }

    async fn resolve_package(&self, package_name: &str) -> Result<String> {
        Ok(format!("cargo install {package_name}"))
    }
}
