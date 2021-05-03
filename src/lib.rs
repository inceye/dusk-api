// Plugin API used in Dusk project
//
// Copyright (C) 2021 by Andy Gozas <andy@gozas.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Crate, that is used while building a plugin system as a common
//! dependency by both plugin and plugin user to define the plugin
//! behavior and safely import and use the plugin
//!
//! # Plugin Side
//!
//! To quickly learn how to create a plugin and export functions from it see
//! [`export_freight!`] macro documentation
//!
//! # Importer Side
//!
//! To quickly learn how to import and use plugins see [`FreightProxy`]
//! documentation

#![deny(warnings)]

#![allow(unused_parens)]

#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)] 
#![warn(missing_copy_implementations)] 
#![warn(missing_debug_implementations)] 
#![warn(variant_size_differences)] 
#![warn(keyword_idents)]
#![warn(anonymous_parameters)]

#![warn(missing_abi)]

#![warn(meta_variable_misuse)]
#![warn(semicolon_in_expressions_from_macros)]
#![warn(absolute_paths_not_starting_with_crate)]

#![warn(missing_crate_level_docs)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(invalid_html_tags)]
#![warn(non_ascii_idents)]
#![warn(pointer_structural_match)]
#![warn(private_doc_tests)]
#![warn(single_use_lifetimes)]
#![warn(unaligned_references)]

use std::any::{Any, TypeId};

pub mod changelog;

pub mod version;
pub mod error;

pub mod declaration;
pub mod registration;
pub mod interplugin;

pub mod callables;
pub mod functions;
pub mod types;
pub mod traits;
pub mod modules;
pub mod freights;

pub use version::*;
pub use error::*;

pub use declaration::*;
pub use registration::*;
pub use interplugin::*;

pub use callables::*;
pub use functions::*;
pub use types::*;
pub use traits::*;
pub use modules::*;
pub use freights::*;
