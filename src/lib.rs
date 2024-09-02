//    dyconfig Configuration file parser through Wasm-WASI plugins
//    Copyright (C) 2024  Paolo De Donato
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Configuration file parser through Wasm-WASI plugins
//!
//! Dyparser allows you to load and parse configuration files through Wasm plugins so that your project
//! is not forced to follow any configuration file format. In this way OS delopers and sysadmins
//! can choose a single configuration file format and use it for every application that use
//! dyparser instead of using a different file format for each application.
//!
//! Dyparser uses WASI preview 2 libraries as plugins that implement the `dyparser` world, as
//! specified inside `wit/world.wit`, to parse configuration files to a
//! [`MultiSection<String>`](crate::sections::MultiSection) object. In particular, these plugins
//! doesn't deduce the type of each field value but bring them to the host as they are. The host
//! then can parse them to a more appropriate object.
//!
//! Dyparser uses the `directories` crate to find the appropriate Wasm plugin to use. In
//! particular, it will search the `default.wasm` Wasm plugin inside `{DATA_DIR}/plugins/` where
//! `{DATA_DIR}` if  the output of `directories::ProjectDirs::form("org", "loara",
//! dyparser").data_dir()"`. Currently (`directories` version `5.0.1`) these files are tested: 
//! - Linux: `${XDG_DATA_HOME}/dyparser/plugins/default.wasm`;
//! - Windows: `{FOLDERID_RoamingAppData}\loara\dyparser\data\plugins\default.wasm`;
//! - MacOS: `${HOME}/Library/Application Support/org.loara.dyparser/plugins/default.wasm`.
//!
//! ## WIT file
#![doc = include_str!("../wit/world.wit")]

extern crate wasmtime;
extern crate directories;

pub mod sections;
pub mod plugins;
