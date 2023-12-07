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

use contracts::*;
use crate::em::consts::*;
// ===================== Passive Microwave Systems =====================

/// Antenna types supported by this library
///
/// 1. `Monopole`: A simple monopole antenna
/// 2. `ShortDipole`: A short dipole antenna
/// 3. `HalfWaveDipole`: A dipole antenna whose size is half the operating wavelength
/// 4. `YagiYudaSix`: A Yagi-Yuda antenna with six horizontal rods
/// 5. `Rectangular`: A rectangular-shaped antenna
/// 6. `Parabaloid`: A Circular paraboloid antenna
#[derive(PartialEq)]
pub enum AntennaType {
	Monopole          // A simple monopole antenna
	, ShortDipole     // A short dipole antenna
	, HalfWaveDipole  // A dipole antenna whose size is half the operating wavelength
	, YagiYudaSix     // A Yagi-Yuda antenna with six horizontal rods
	, Rectangular     // A rectangular-shaped antenna
	, Parabaloid      // A Circular paraboloid antenna
}

pub mod instruments {
	// Polarization types
	//     H: Horizontally polarized
	//     V: Vertically polarized
	//     R: Right polarized
	//     L: Left polarized
	//     VH: Vertically and horizontally polarized
	#[derive(Copy, Clone)]
	pub enum Polarization {
		H, V, R, L, VH
	}

	#[derive(Copy, Clone)]
	pub struct Band {
		f_min : f64          // frequency min (GHz)
		, f_max : f64        // frequency max (GHz)
		, b : f64            // bandwidth (MHz)
		, p : Polarization   // Polarization direction
		, delta_t : f64      // Temp differential (Kelvin)
		, res_x : u16        // Resolution horizontal (km)
		, res_y : u16        // Resolution vertical (km)
	}

	#[derive(Copy, Clone)]
	pub struct Channel {
		channel : u8
		, f_min : f64       // frequency min (GHz)
		, f_max : f64       // frequency max (GHz)
		, b : f64           // bandwidth (MHz)
		, p : Polarization  // Polarization direction
		, bands : u8        // band count
		, delta_t : f64     // Temp differential (Kelvin)
	}

	// SSMIS and MSMR tables
	/// Characteristic SSMIS bands. Channels 0-4 are most often used for Earth's
	/// surface and can also be used for SSM/I instruments.
	pub const ssmis : [Band; 24] = [
		Band{ f_min : 19.35, f_max : 19.35, b : 355.0, p : Polarization::H, delta_t : 0.35, res_x : 73, res_y : 47 }
		, Band{ f_min : 19.35, f_max : 19.35, b : 357.0, p : Polarization::V, delta_t : 0.35, res_x : 73, res_y : 47 }
		, Band{ f_min : 22.235, f_max : 22.235, b : 401.0, p : Polarization::V, delta_t : 0.45, res_x : 73, res_y : 47 }
		, Band{ f_min : 37.0, f_max : 37.0, b : 1616.0, p : Polarization::H, delta_t : 0.22, res_x : 41, res_y : 31 }
		, Band{ f_min : 37.0, f_max : 37.0, b : 1545.0, p : Polarization::V, delta_t : 0.22, res_x : 41, res_y : 31 }
		, Band{ f_min : 50.3, f_max : 50.3, b : 380.0, p : Polarization::H, delta_t : 0.34, res_x : 27, res_y : 18 }
		, Band{ f_min : 52.8, f_max : 52.8, b : 389.0, p : Polarization::H, delta_t : 0.32, res_x : 27, res_y : 18 }
		, Band{ f_min : 53.596, f_max : 53.596, b : 380.0, p : Polarization::H, delta_t : 0.33, res_x : 27, res_y : 18 }
		, Band{ f_min : 54.4, f_max : 54.4, b : 383.0, p : Polarization::H, delta_t : 0.33, res_x : 27, res_y : 18 }
		, Band{ f_min : 55.5, f_max : 55.5, b : 391.0, p : Polarization::H, delta_t : 0.34, res_x : 27, res_y : 18 }
		, Band{ f_min : 57.29, f_max : 57.29, b : 330.0, p : Polarization::R, delta_t : 0.41, res_x : 27, res_y : 18 }
		, Band{ f_min : 59.4, f_max : 59.4, b : 239.0, p : Polarization::R, delta_t : 0.4, res_x : 27, res_y : 18 }
		, Band{ f_min : 62.998, f_max : 63.569, b : 1.35, p : Polarization::R, delta_t : 2.7, res_x : 27, res_y : 18 }
		, Band{ f_min : 60.435, f_max : 61.151, b : 1.35, p : Polarization::R, delta_t : 2.7, res_x : 27, res_y : 18 }
		, Band{ f_min : 60.435, f_max : 61.151, b : 1.3, p : Polarization::R, delta_t : 1.9, res_x : 27, res_y : 18 }
		, Band{ f_min : 60.435, f_max : 61.151, b : 2.6, p : Polarization::R, delta_t : 1.3, res_x : 27, res_y : 18 }
		, Band{ f_min : 60.435, f_max : 61.151, b : 7.35, p : Polarization::R, delta_t : 0.8, res_x : 27, res_y : 18 }
		, Band{ f_min : 60.435, f_max : 61.151, b : 26.5, p : Polarization::R, delta_t : 0.9, res_x : 27, res_y : 18 }
		, Band{ f_min : 91.665, f_max : 91.665, b : 1411.0, p : Polarization::H, delta_t : 0.19, res_x : 14, res_y : 13 }
		, Band{ f_min : 91.665, f_max : 91.665, b : 1418.0, p : Polarization::V, delta_t : 0.19, res_x : 14, res_y : 13 }
		, Band{ f_min : 150.0, f_max : 150.0, b : 1642.0, p : Polarization::H, delta_t : 0.53, res_x : 14, res_y : 13 }
		, Band{ f_min : 182.311, f_max : 184.311, b : 513.0, p : Polarization::H, delta_t : 0.38, res_x : 14, res_y : 13 }
		, Band{ f_min : 180.311, f_max : 186.311, b : 1019.0, p : Polarization::H, delta_t : 0.39, res_x : 14, res_y : 13 }
		, Band{ f_min : 176.711, f_max : 189.911, b : 1526.0, p : Polarization::H, delta_t : 0.56, res_x : 14, res_y : 13 }];

	/// MSMR instrument bands
	pub const msmr : [Band; 4] = [
		Band{ f_min : 6.6, f_max : 6.6, b : 350.0, p : Polarization::VH, delta_t : 1.0, res_x : 105, res_y : 68 }
		, Band{ f_min : 10.65, f_max : 10.65, b : 100.0, p : Polarization::VH, delta_t : 1.0, res_x : 66, res_y : 43 }
		, Band{ f_min : 18.0, f_max : 6.6, b : 200.0, p : Polarization::VH, delta_t : 1.0, res_x : 40, res_y : 26 }
		, Band{ f_min : 21.0, f_max : 6.6, b : 400.0, p : Polarization::VH, delta_t : 1.0, res_x : 34, res_y : 22 }
		];
	// TODO: AMSU-A and MHS Tables
}

/// Computes the Johnson/Nyquist noise power of an antenna
/// Takes: `antenna_temp`: The temperature of the antenna
///        `band_size` : The bandwidth used by the antenna
#[requires(antenna_temp > 0.0)]
#[requires(band_size > 0.0)]
#[ensures(ret > 0.0)]
pub fn jnoise_power(antenna_temp : f64, band_size : f64) -> f64 {
	return K * antenna_temp * band_size;
}

/// Computes half power bandwidth. The `size` parameter is dependent on
/// the type of antenna:
/// 1. For rectangular, it's the size of the sides
/// 2. For Circular paraboloid it's the diameter
#[requires(lambda > 0.0)]
#[requires(size > 0.0)]
#[ensures(ret >= 0.0)]
pub fn hpbw(lambda : f64, size : f64, atype : AntennaType) -> f64 {
	if atype == AntennaType::Monopole {
		return 0.0; // isomorphic
	}
	else if atype == AntennaType::ShortDipole || atype == AntennaType::HalfWaveDipole {
		return 90.0;
	}
	else if atype == AntennaType::YagiYudaSix {
		return 42.0;
	}
	else if atype == AntennaType::Rectangular {
		return 51.0 * (lambda / size);
	}
	else { // Paraboloid
		return 72.0 * (lambda / size);
	}
}

/// Computes the directivity given beam solid angle
/// Takes: `bsa`, the beam solid angle
#[requires(bsa >= 0.0 && bsa <= 6.29)]
#[ensures(ret <= 2.0 && ret >= 0.0)]
pub fn directivity(bsa : f64) -> f64 {
	return 4.0 * PI / bsa;
}

/// Computes beam solid angle from power pattern via numerical integration
/// Takes: `P` a dynamic function taking $\theta$ and $\phi$ in radians in that order and providing the power pattern's value at that angle.
///        `step` the step for numerical integration (if `None` is passed in, defaults to `0.01`)
#[requires(step.is_some() -> step.unwrap() > 0.0)]
pub fn beam_solid_angle(P: &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
	let s : f64 = step.unwrap_or(0.01);
	// Size of square for integration
	let s2 = s.powi(2);
	let mut sum : f64 = 0.0;
	let mut theta : f64 = 0.0;
	let mut phi : f64 = 0.0;
	while theta < PI / 2.0 {
		while phi < 2.0 * PI {
			sum += P(theta, phi) * s2;
			phi += s;
		}
		theta += s;
	}
	return sum;
}

/// Computes antenna temperature via numerical integration
/// Once again, default `step` is `0.01`.
#[requires(step.is_some() -> step.unwrap() > 0.0)]
pub fn antenna_temp(TB: &dyn Fn(f64, f64) -> f64, P: &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
	let bsa = beam_solid_angle(P, step);
	let s : f64 = step.unwrap_or(0.01);
	// Size of square for integration
	let s2 = s.powi(2);
	let mut sum : f64 = 0.0;
	let mut theta : f64 = 0.0;
	let mut phi : f64 = 0.0;
	while theta < PI / 2.0 {
		while phi < 2.0 * PI {
			sum += TB(theta, phi) * P(theta, phi) * s2;
			phi += s;
		}
		theta += s;
	}
	return sum / bsa;
}


/// Computes forward gain using power pattern and efficiency
///
/// Note: Uses default step in `beam_solid_angle` integration.
#[requires(efficiency > 0.0)]
pub fn forward_gain(efficiency : f64, P: &dyn Fn(f64, f64) -> f64) -> f64 {
	// Directivity
	let d = 4.0 * PI / beam_solid_angle(P, None);
	return efficiency * d;
}

/// Computes spectral radiance given temperature and wavelength
#[requires(tb > 0.0)]
#[requires(wavelength > 0.0)]
#[ensures(ret > 0.0)]
pub fn spectral_radiance(tb : f64, wavelength : f64) -> f64{
	return 2.0 * K * tb / wavelength.powi(2);
}

/// Computes spectral flux density. Requires the small angle to be less
/// than a radian
#[requires(tb > 0.0)]
#[requires(wavelength > 0.0)]
#[requires(small_angle > 0.0 && small_angle < 1.0)]
pub fn spectral_flux_density(tb : f64, wavelength : f64, small_angle : f64) -> f64 {
	return 2.0 * K * tb * small_angle / wavelength.powi(2);
}

/// Computes the effective area of an antenna
#[requires(wavelength > 0.0)]
#[ensures(ret > 0.0)]
pub fn effective_area(wavelength : f64, P: &dyn Fn(f64, f64) -> f64) -> f64 {
	let bsa = beam_solid_angle(P, None);
	return wavelength.powi(2) / bsa;
}

/// Computes antenna sensitivity ($\Delta T$)
#[requires(sys_temp > 0.0)]
pub fn sensitivity(sys_temp : f64, c : Option<f64>, del_t : Option<f64>, del_f : Option<f64>) -> f64 {
	let c_val = c.unwrap_or(5.0);
	let d_t = del_t.unwrap_or(0.01);
	let d_f = del_f.unwrap_or(0.01);
	return c_val * sys_temp / (d_t * d_f).sqrt();
}

/// Computes cross-polarization gradient ratio ($XPGR$)
#[requires(t_19h > 0.0)]
#[requires(t_37v > 0.0)]
#[ensures(t_19h > t_37v -> ret > 0.0)]
pub fn xpgr(t_19h : f64, t_37v : f64) -> f64 {
	return (t_19h - t_37v) / (t_19h + t_37v);
}

/// Computes polarization ratio ($PR$)
#[requires(t_19h > 0.0)]
#[requires(t_19v > 0.0)]
#[ensures(t_19v > t_19h -> ret > 0.0)]
pub fn polarization_ratio(t_19h : f64, t_19v : f64) -> f64 {
	return (t_19v - t_19h) / (t_19v + t_19h);
}

/// Computes gradient ratio ($GR$)
#[requires(t_19v > 0.0)]
#[requires(t_37v > 0.0)]
#[ensures(t_19v < t_37v -> ret > 0.0)]
pub fn gradient_ratio(t_19v : f64, t_37v : f64) -> f64 {
	return (t_37v - t_19v) / (t_37v + t_19v);
}

/// Computes upwelling component ($T_b$) of temperature through atmosphere.
#[requires(tau > 0.0)]
pub fn upwelling_component(tau : f64, T : &dyn Fn(f64) -> f64) -> f64 {
	return T(1.0 - (-tau).exp());
}
