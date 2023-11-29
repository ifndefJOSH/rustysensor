//! `rustysensor` is a crate which provides functionality related to remote sensing applications
//! with a particular focus on embedded systems, accuracy, and electromagnetic sensing. It implements
//! a number of approximations, formulas, and methods used widely by sensing applications.
//!
//! It is split into five sub modules: `em`, `el_opt`, `muwave`, `photographic`, and `ranged`. These
//! delineate functionality and contain functions and constants related to those particular fields.
//!
//! # The Electromagnetic Module
//!
//! The `em` module of the library contains functions, constants, and other elements
//! useful in applications sensing generally via the electromagnetic
//! spectrum. This includes spectral and blackbody radiance, and the doppler
//! effect, as well as basic functions for angular frequency,
//! electromagnetic wavelength, and photon energy.
//!
//! The purpose of this portion of the library is to provide general
//! functionality with regards to the electromagnetic spectrum. Much of
//! remote sensing is based on EM radiation, and so this portion of the
//! library is fundamental to much of the rest of it, and therefore is the
//! first one included.
//!
//! # The Electro-Optical Module
//!
//! The `el_opt` portion of the library similarly expands on the electromagnetic section
//! by introducing functions useful when sensing optically. This includes
//! some near-IR and infrared sensing systems. However, in this section we
//! also focus in large part on TIR (temperature IR) systems.
//!
//! The reason this section (as well as many of the other sections here) is
//! included in the library is to focus on a more specific portion of the
//! electromagnetic spectrum which is useful to many systems. However, many
//! of the elements included in this section are still quite general-purpose
//! and can be used in a wide variety of EM-related applications
//!
//! # The Photographic Systems Module
//!
//! The `photographic` portion of the library is designed to focus on visible light and near-IR
//! bands in the electromagnetic spectrum. Photographic systems are
//! extremely important since nearly all life uses these bands of the EM
//! spectrum, partially because of the intensity at which our sun emits
//! them. Some of humanity's first artificial "remote sensing" systems
//! (cameras) utilized this band of the electromagnetic spectrum as well.
//!
//! Additionally, we include in this section functionality related to
//! portions of the infrared spectrum closest to visible light, as that
//! still falls under a "photographic" system. The purpose for this is that
//! many systems do not necessarily follow the sensitivity spectrum of the
//! human eye.
//!
//! # The Passive Microwave Module
//!
//! The `muwave` portion of the library specifically focuses on remote sensing
//! applications in the microwave band of the electromagnetic spectrum and
//! passive sensing. This includes functions which work with passive
//! antennas and any system which works in the microwave section of the
//! electromagnetic spectrum.
//!
//! It currently has no submodules or anything else beyond what is in that
//! module, but again, like the rest of the library, is structured to allow
//! for expansion.
//!
//! # The Scattering and Ranged Systems Module
//!
//! This portion of the library includes functionality for any and all system
//! which is active, ranged, or must pass through a medium which scatters
//! it. Therefore there is functionality in this section related to
//! scattering coefficients, backscattering, and the power pattern of an
//! active antenna. This functionality is included in a module called
//! `ranged`.
//!


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


#[cfg(test)]
mod tests {
    use super::*;

	// TODO: create unit tests
}
