// Copyright (C) 2021 by Andy Gozas <andy@gozas.me>
//
// This file is part of Dusk API.
//
// Dusk API is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Dusk API is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Dusk API.  If not, see <https://www.gnu.org/licenses/>.

//! Module, containing everything needed for version control of
//! plugin versions, compiler versions and API versions

/// Api version parameter, passed from the build script.
///
/// For the program that uses the plugin to work correctly it
/// has to use the same version of api, which is ensured by embedding
/// it as a static variable
pub static API_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Rust compiler version parameter, passed from the compiler.
///
/// If plugin is compiled with a different rust compiler version
/// it may be incompatible with the program using it, so before
/// proceeding to use the plugin a version check is needed.
///
/// For this to work, build script should set this environmental
/// variable, which is done for this crate like this
///
/// # build.rs
/// ``` rust, ignore
/// extern crate rustc_version;
///
/// fn main() {
///     let version = rustc_version::version().unwrap();
///     println!("cargo:rustc-env=RUSTC_VERSION={}", version);
/// }
/// ```
///
/// # Cargo.toml
/// ``` rust, ignore
/// [build-dependencies]
/// rustc_version = "0.3.0"
/// ```
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// A structure that holds a representation of plugin version
/// for easy comparison and storage.
///
/// The ordering is as follows
///
/// * Major
/// * Minor
/// * Release
/// * Build
///
/// e.g in 1.2.3.4, 1 is major, 2 is minor, 3 is release and 4 
/// is build
///
/// # Example
///
/// ```
/// let a = dusk_api::Version { major: 1, ..Default::default() };
/// let b = dusk_api::Version { minor: 1, ..Default::default() };
/// let c = dusk_api::Version { major: 0, minor: 2, release: 1, build: 0 };
///
/// assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater); 
/// assert_eq!(b.cmp(&c), std::cmp::Ordering::Less); 
/// assert_eq!(a.cmp(&c), std::cmp::Ordering::Greater); 
/// ```
#[derive(Copy, Clone, Debug, Eq)]
pub struct Version {

    /// Major version number
    pub major: usize,

    /// Minor version number
    pub minor: usize,

    /// Release version number
    pub release: usize,

    /// Build version number
    pub build: usize,
}

impl Ord for Version {
    fn cmp(self: &Self, other: &Self) -> std::cmp::Ordering {
        if self.major > other.major {
            return std::cmp::Ordering::Greater;
        }
        if self.major < other.major {
            return std::cmp::Ordering::Less;
        }
        if self.minor > other.minor {
            return std::cmp::Ordering::Greater;
        }
        if self.minor < other.minor {
            return std::cmp::Ordering::Less;
        }
        if self.release > other.release {
            return std::cmp::Ordering::Greater;
        }
        if self.release < other.release {
            return std::cmp::Ordering::Less;
        }
        if self.build > other.build {
            return std::cmp::Ordering::Greater;
        }
        if self.build < other.build {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Version {
    fn partial_cmp(
        self: &Self, 
        other: &Self,
    ) -> Option<std::cmp::Ordering> {

        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(self: &Self, other: &Self) -> bool {
        return self.cmp(other) == std::cmp::Ordering::Equal;
    }
}

impl Default for Version {
    fn default () -> Version {
        Version {
            major: 0,
            minor: 0,
            release: 0,
            build: 0,
        }
    }
}
