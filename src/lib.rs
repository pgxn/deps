pub mod error;
pub mod operating_system;
pub mod package_type;

use std::str::FromStr;
use std::sync::LazyLock;

use operating_system::OperatingSystem;
use purl::GenericPurl;

use crate::error::{Error, Result};
use crate::package_type::{Cargo, Generic, PackageType, Pgxn, Postgres};

static SUPPORTED_PACKAGE_TYPES: LazyLock<Vec<Box<dyn PackageType>>> = LazyLock::new(|| {
    vec![
        Box::new(Pgxn),
        Box::new(Generic),
        Box::new(Cargo),
        Box::new(Postgres),
    ]
});

/// Given a PURL, return the installation command to install the given package.
pub async fn resolve_package(purl: &str, os: OperatingSystem) -> Result<String> {
    let purl = GenericPurl::<String>::from_str(purl)?;

    let package_type = SUPPORTED_PACKAGE_TYPES
        .iter()
        .find(|supported_package_type| supported_package_type.name() == purl.package_type())
        .ok_or_else(|| Error::UnknownPackage(purl.package_type().to_string()))?;

    package_type
        .resolve_package_for_operating_system(purl.name(), os)
        .await
}

#[cfg(test)]
mod tests {
    use crate::{operating_system::OperatingSystem, resolve_package};

    #[tokio::test]
    async fn resolves_generic_packages() {
        assert_eq!(
            resolve_package("pkg:generic/ripgrep", OperatingSystem::Debian)
                .await
                .unwrap(),
            "sudo apt-get install -y rust-ripgrep"
        );

        assert_eq!(
            resolve_package("pkg:generic/ripgrep", OperatingSystem::RedHat)
                .await
                .unwrap(),
            "sudo dnf install -y rust-ripgrep"
        );

        assert_eq!(
            resolve_package("pkg:generic/ripgrep", OperatingSystem::Mac)
                .await
                .unwrap(),
            "brew install ripgrep"
        );
    }

    #[tokio::test]
    async fn resolves_other_packages() {
        assert_eq!(
            resolve_package("pkg:cargo/cargo-pgrx", OperatingSystem::Debian)
                .await
                .unwrap(),
            "cargo install cargo-pgrx"
        );

        assert_eq!(
            resolve_package("pkg:pgxn/pgtap", OperatingSystem::Debian)
                .await
                .unwrap(),
            "pgxn install pgtap"
        );

        assert_eq!(
            resolve_package("pkg:pgxn/temporal-tables", OperatingSystem::Debian)
                .await
                .unwrap(),
            "pgxn install temporal-tables"
        );

        assert_eq!(
            resolve_package("pkg:postgres/plpgsql", OperatingSystem::Debian)
                .await
                .unwrap(),
            "pgxn install plpgsql"
        );
    }
}
