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

pub mod consts {
	/// The radius of the Earth in kilometers
	pub const EARTH_RAD   : f64 =  6371.0; // km
	/// The radius of the Moon, not accounting for variation, in kilometers
	pub const MOON_RAD    : f64 =  1737.4; // km
	/// The radius of Mercury, not accounting for variation, in kilometers
	pub const MERCURY_RAD : f64 =  2439.7; // km
	/// The radius of Venus, not accounting for variation, in kilometers
	pub const VENUS_RAD   : f64 =  6051.8; // km
	/// The radius of Mars, not accounting for variation, in kilometers
	pub const MARS_RAD    : f64 =  3389.5; // km
	/// The radius of Jupiter, not accounting for variation, in kilometers
	pub const JUPITER_RAD : f64 = 69911.0; // km
	/// The radius of Saturn, not accounting for variation, in kilometers
	pub const SATURN_RAD  : f64 = 58232.0; // km
	/// The radius of Neptune, not accounting for variation, in kilometers
	pub const NEPTUNE_RAD : f64 = 24622.0; // km
	/// The radius of Uranus, not accounting for variation, in kilometers
	pub const URANUS_RAD  : f64 = 25362.0; // km
	/// The radius of Pluto, not accounting for variation, in kilometers
	pub const PLUTO_RAD   : f64 =  1188.3; // km
	/// The radius of Charon, not accounting for variation, in kilometers
	pub const CHARON_RAD  : f64 =   606.0; // km
	/// Q'apla' (success!)
	pub const QUONOS_RAD  : f64 =  6410.2;
}

use crate::ranged::consts::*;

/// The travel time given range and group velocity
pub fn travel_time(range : f64, group_velocity : f64) -> f64 {
	return 2.0 * range / group_velocity;
}

// Ranged systems

/// The SNR computed using RMS power of signal and noise
#[requires(signal.len() == noise.len())]
#[ensures(ret >= 0.0)]
pub fn averaging_rms_snr(signal : &[f64], noise : &[f64]) -> f64 {
	let mut mean_square_signal : f64 = 0.0;
	for sample in signal.into_iter().enumerate() {
		mean_square_signal += sample.1.powi(2);
	}
	mean_square_signal /= signal.len() as f64;
	let mut mean_square_noise : f64 = 0.0;
	for sample in noise.into_iter().enumerate() {
		mean_square_noise += sample.1.powi(2);
	}
	mean_square_noise /= signal.len() as f64;
	return (mean_square_signal / mean_square_noise).sqrt()
}

// Laser profiling systems

/// The accuracy ratio given rise time and signal to noise ratio
pub fn accuracy(rise_time : f64, snr : f64) -> f64 {
	return rise_time / snr;
}

/// The range accuracy
pub fn range_accuracy(
	vg : f64
	, tr_op : Option<f64>
	, s_op  : Option<f64>
	, v_op  : Option<f64>
	, h_op  : Option<f64>
	, p_op  : Option<f64>
	, del_theta_op : Option<f64>
) -> f64 {
	// Uses the defaults for an airborne system
	let tr : f64 = tr_op.unwrap_or(5.0e-9);
	let s  : f64 = s_op.unwrap_or(1.0);
	let v  : f64 = v_op.unwrap_or(50.0);
	let h  : f64 = h_op.unwrap_or(200.0);
	let p  : f64 = p_op.unwrap_or(1000.0);
	let del_theta : f64 = del_theta_op.unwrap_or(0.001);
	return vg * tr / (2.0 * s) * (v / (p * h * del_theta)).sqrt();
}

// TODO: radar range equation

/// Range ambiguity. `p_op` defaults to `1000.0` if not provided
pub fn range_ambiguity(vg : f64, p_op  : Option<f64>) -> f64 {
	let p : f64 = p_op.unwrap_or(1000.0);
	return vg / 2.0 * p;
}

/// The upper bound of possible periods. Note that this period is
/// an unreachable upper bound.
pub fn longest_period(vg : f64, h_op  : Option<f64>) -> f64 {
	let h  : f64 = h_op.unwrap_or(200.0);
	return vg / 2.0 * h;
}

/// Returns `true` if the period `p` passed in is ideal according to `vg` and `h_op`
pub fn is_ideal_period(p : f64, vg : f64, h_op : Option<f64>) -> bool {
	return p < longest_period(vg, h_op);
}

/// This function applies to scanning laser profilers.
/// Calculates the spacing of samples when sampling cross track,
/// given the frequency, the angle, phi, and h, the range.
/// `frequency` is the frequency of the sampling pattern's zig zag profile
pub fn sampled_cross_track(frequency : f64, angle : f64, h : f64, impulse_period : f64) -> f64 {
	return 4.0 * angle * frequency * h / impulse_period;
}

/// This function applies to scanning laser profilers.
/// Computes the average sampling interval in along-track direction.
/// `velocity` is the velocity of the profiler, and `frequency is the
/// frequency of the sampling pattern
pub fn sampled_along_track(velocity : f64, frequency : f64) -> f64 {
	return velocity / frequency;
}

// Radar altimetry

/// Returns the min return time for a radar altimeter at height `height`.
pub fn min_return_time(height : f64) -> f64 {
	return 2 * height / C;
}

/// Calculates the footprint radius for a radar altimetre
pub fn footprint_radius(rise_time : f64, height : f64, adjust_effective_height : bool) -> f64 {
	if adjust_effective_height {
		return (C * height * rise_time).sqrt();
	}
	let e_height = effective_height(height, None);
	return (C * e_height * rise_time).sqrt();
}

/// Calculates the effective height accounting for the curvature of a spherical
/// celestial body. If no radius is provided, then it defaults to the Earth's
/// radius, in kilometers.
pub fn effective_height(height : f64, radius : Option<f64>) -> f64 {
	// default to earth's radius since most systems are here on earth
	let rad = radius.unwrap_or(EARTH_RAD);
	return 1.0 / (height.powi(-1) + rad.powi(-1));
}

/// Computes the approximate coherence length given a frequency band
#[requires(f_max > f_min)]
#[requires(f_min > 0.0)]
#[ensures(ret > 0.0)]
pub fn coherence_length(f_min : f64, f_max : f64) -> f64 {
	return C / (f_max - f_min);
}

/// Computes the approximate coherence width
#[requires(antenna_height > 0.0)]
#[requires(antenna_diameter > 0.0)]
#[requires(frequency > 0.0)]
#[ensures(ret > 0.0)]
pub fn coherence_width(antenna_height : f64, frequency : f64, antenna_diameter : f64) -> f64 {
	return c * antenna_height / (antenna_diameter * frequency);
}

// TODO

// Scattered systems

// most basic brdf: R = L1 / E
/// Basic brdf estimate `radiance / irradiance`
pub fn brdf_basic(radiance : f64, irradiance : f64) -> f64 {
	return radiance / irradiance;
}
// TODO: other brdf approximations

/// A basic means of calculating the bistatic scattering coefficient with
/// the approximation of the brdf in `brdf_basic`.
pub fn bistatic_scattering_coefficient_basic(radiance : f64, irradiance : f64, angle : f64) -> f64 {
	let r = brdf_basic(radiance, irradiance);
	return bistatic_scattering_coefficient(r, angle);
}

/// The more general `bistatic_scattering_coefficient` function which
/// takes a known brdf. If brdf is calculated using `brdf_basic` this is equivalent
/// to `bistatic_scattering_coefficient_basic`
pub fn bistatic_scattering_coefficient(brdf : f64, angle : f64) -> f64 {
	return 4.0 * PI * brdf * angle.cos();
}

/// Computes flux density in a bistatic dual-radar system. However, this is only
/// concerned with the flux density through the scanned surface.
pub fn bistatic_flux_density(
	antenna_gain        : f64 // The antenna's gain, Gt
	, transmitted_power : f64 // Transmitted power Pt
	, transmit_dist     : f64 // The distance from the transmitting antenna to the surface
) -> f64 {
	return antenna_gain * transmitted_power / (4.0 * PI * transmit_dist.powi(2));
}

// TODO: should we also have an irradiance function?

/// Computes the received radar power in a bistatic dual radar system
pub fn bistatic_radar_power(
	b_scat_coefficient  : f64 // Bistatic scattering coefficient, denoted "gamma"
	, collecting_area   : f64 // The size of the collecting area
	, effective_area    : f64 // Effective area of receiver, Ar
	, antenna_gain      : f64 // The antenna's gain, Gt
	, transmitted_power : f64 // Transmitted power Pt
	, incoming_angle    : f64 // Incoming angle, theta_0
	, exit_angle        : f64 // The exit angle, theta_1
	, transmit_dist     : f64 // The distance from the transmitting antenna to the surface
	, received_dist     : f64 // The distance to the receiving antenna from the surface
) -> f64 {
	let f = bistatic_flux_density(antenna_gain, transmitted_power, transmit_dist);
	let e = f * cos(incoming_angle);
	let l = b_scat_coefficient * e / (4.0 * PI * exit_angle.cos());
	return l * collecting_area * effective_area / received_dist.powi(2) * cos(exit_angle);
}

// Microwave scatterometry stuff

/// Computes doppler shift given speed, angle, and relative velocity
pub fn doppler_shift(transmitted_freq : f64, velocity : f64, angle : f64) -> f64 {
	return 2.0 * transmitted_freq * velocity / C * angle.sin();
}

/// Range resolution for an SLR system
pub fn range_resolution(tp : f64, theta : f64) -> f64 {
	return C * tp / (2.0 * theta.sin());
}

/// Phase delay for Synthetic Aperture Radar (SAR) systems
pub fn sar_phase_delay(
	wave_num   : f64 // wavenumber of the radiation
	, time     : f64 // Scatter time
	, velocity : f64 // velocity of the system
	, height   : f64 // vertical difference between system and scatterer
	, dist     : f64 // horizontal distance between system and scatterer
) -> f64 {
	return 2.0 * wave_num * (height.powi(2) + (velocity * time - height).powi(2)).sqrt();
}
