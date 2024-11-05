pub mod error;
pub mod operating_system;
pub mod package_type;

use std::{str::FromStr, sync::LazyLock};

use package_type::{Cargo, Generic, PackageType, Pgxn, Postgres};
use purl::GenericPurl;

use crate::error::{Error, Result};

static SUPPORTED_PACKAGE_TYPES: LazyLock<Vec<Box<dyn PackageType>>> = LazyLock::new(|| {
    vec![
        Box::new(Pgxn),
        Box::new(Generic),
        Box::new(Cargo),
        Box::new(Postgres),
    ]
});

/// Given a PURL, return the installation command to install the given package.
pub async fn resolve_package(purl: &str) -> Result<String> {
    let purl = GenericPurl::<String>::from_str(purl)?;

    let package_type = SUPPORTED_PACKAGE_TYPES
        .iter()
        .find(|supported_package_type| supported_package_type.name() == purl.package_type())
        .ok_or_else(|| Error::UnknownPackage(purl.package_type().to_string()))?;

    package_type.resolve_package(purl.name()).await
}

#[tokio::main]
async fn main() -> Result<()> {
    resolve_package("pkg:cargo/cargo-pgrx").await?;
    resolve_package("pkg:postgres/plpgsql").await?;
    resolve_package("pkg:pgxn/pgtap").await?;
    resolve_package("pkg:generic/curl").await?;

    Ok(())
}
