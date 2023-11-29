/*

rustysensor: a remote sensing library written in pure Rust
Copyright (C) 2023 Josh Jeppson

This program is DUAL-LICENSED. If you have received this code
for free (i.e., you did not have to pay for a license agreement),
it is licensed under the GPLv3.

If so, this program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

NOTE: There is NO LINKING EXCEPTION to the open-source version of
this library. This means that if you wish to link against rustysensor
in a proprietary application, you will have to obtain a license agreement.
If you wish to do so, please reach out to the current maintainer.

*/

/// The Electromagnetics module
pub mod em;
/// The Electro Optical Module
pub mod el_opt;
/// The Microwave module
pub mod muwave;
/// The photographic systems module
pub mod photographic;
/// The ranged and scattered systems modules
pub mod ranged;

/// # RustySensor
///
/// `rustysensor` is a crate which provides functionality related to remote sensing applications
/// with a particular focus on embedded systems, accuracy, and electromagnetic sensing. It implements
/// a number of approximations, formulas, and methods used widely by sensing applications.

#[cfg(test)]
mod tests {
    use super::*;

	// TODO: create unit tests
}
