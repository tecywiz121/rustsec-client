//! Types for representing Cargo.lock files

use db::AdvisoryDatabase;
use advisory::Advisory;
use error::{Error, Result};
use semver::Version;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;
use util;

/// Entry from Cargo.lock's `[[package]]` array
/// TODO: serde macros or switch to cargo's builtin types
#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    /// Name of a dependent crate
    pub name: String,

    /// Version of dependent crate
    pub version: Version,
}

/// Parsed Cargo.lock file containing dependencies
#[derive(Debug, PartialEq, Clone)]
pub struct Lockfile {
    /// Dependencies enumerated in the lockfile
    pub packages: Vec<Package>,
}

/// A vulnerable package and the associated advisory
#[derive(Debug, PartialEq, Clone)]
pub struct Vulnerability<'a> {
    /// A security advisory for which the package is vulnerable
    pub advisory: &'a Advisory,

    /// A vulnerable package
    pub package: &'a Package,
}

impl Lockfile {
    /// Load lockfile from disk
    pub fn load(filename: &str) -> Result<Self> {
        let path = Path::new(filename);
        let mut file = File::open(&path).or(Err(Error::IO))?;
        let mut toml = String::new();

        file.read_to_string(&mut toml).or(Err(Error::IO))?;
        Self::from_toml(&toml)
    }

    /// Load lockfile from a TOML string
    pub fn from_toml(string: &str) -> Result<Self> {
        let toml = string.parse::<toml::Value>().or(Err(Error::Parse))?;

        let packages_toml = match toml.get("package") {
            Some(&toml::Value::Array(ref arr)) => arr,
            None => return Ok(Lockfile { packages: Vec::new() }),
            _ => return Err(Error::InvalidAttribute),
        };

        let mut packages = Vec::new();

        for package in packages_toml {
            match *package {
                toml::Value::Table(ref table) => {
                    packages.push(Package {
                        name: util::parse_mandatory_string(table, "name")?,
                        version: util::parse_version(table, "version")?,
                    })
                }
                _ => return Err(Error::InvalidAttribute),
            }
        }

        Ok(Lockfile { packages: packages })
    }

    /// Find all relevant vulnerabilities for this lockfile using the given database
    pub fn vulnerabilities<'a>(&'a self, db: &'a AdvisoryDatabase) -> Vec<Vulnerability<'a>> {
        let mut result = Vec::new();

        for package in &self.packages {
            for advisory in db.find_vulns_for_crate(&package.name, &package.version) {
                result.push(Vulnerability {
                    advisory: advisory,
                    package: package,
                })
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use lockfile::Lockfile;

    #[test]
    fn load_cargo_lockfile() {
        let lockfile = Lockfile::load("Cargo.lock").unwrap();
        assert!(lockfile.packages.len() > 0);
    }
}
