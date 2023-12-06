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
#[requires(range > 0.0)]
#[requires(group_velocity > 0.0)]
#[ensures(ret > 0.0)]
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
#[requires(rise_time > 0.0)]
#[requires(snr > 0.0)]
#[ensures(ret > 0.0)]
pub fn accuracy(rise_time : f64, snr : f64) -> f64 {
	return rise_time / snr;
}

/// The range accuracy
#[requires(vg > 0.0)]
#[requires(tr_op.is_some() -> tr_op.unwrap() > 0.0)]
#[requires(s_op.is_some() -> s_op.unwrap() > 0.0)]
#[requires(v_op.is_some() -> v_op.unwrap() > 0.0)]
#[requires(h_op.is_some() -> h_op.unwrap() > 0.0)]
#[requires(p_op.is_some() -> p_op.unwrap() > 0.0)]
#[requires(del_theta_op.is_some() -> del_theta_op.unwrap() > 0.0)]
#[ensures(ret > 0.0)]
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
#[requires(vg > 0.0)]
#[requires(p_op.is_some() -> p_op.unwrap() > 0.0)]
pub fn range_ambiguity(vg : f64, p_op  : Option<f64>) -> f64 {
	let p : f64 = p_op.unwrap_or(1000.0);
	return vg / 2.0 * p;
}

/// The upper bound of possible periods. Note that this period is
/// an unreachable upper bound. This means that all periods which
/// can in practice be used must be strictly less than the period
/// returned here.
#[requires(vg > 0.0)]
#[requires(h_op.is_some() -> h_op.unwrap() > 0.0)]
pub fn longest_period(vg : f64, h_op  : Option<f64>) -> f64 {
	let h  : f64 = h_op.unwrap_or(200.0);
	return vg / 2.0 * h;
}

/// Returns `true` if the period `p` passed in is ideal according to `vg` and `h_op`
#[requires(p > 0.0)]
#[requires(vg > 0.0)]
#[requires(h_op.is_some() -> h_op.unwrap() > 0.0)]
pub fn is_ideal_period(p : f64, vg : f64, h_op : Option<f64>) -> bool {
	return p < longest_period(vg, h_op);
}

/// This function applies to scanning laser profilers.
/// Calculates the spacing of samples when sampling cross track,
/// given the frequency, the angle, phi, and h, the range.
/// `frequency` is the frequency of the sampling pattern's zig zag profile
#[requires(frequency > 0.0)]
#[requires(angle >= 0.0 && angle < 6.29)]
#[requires(impulse_period > 0.0)]
#[ensures(ret > 0.0)]
pub fn sampled_cross_track(frequency : f64, angle : f64, h : f64, impulse_period : f64) -> f64 {
	return 4.0 * angle * frequency * h / impulse_period;
}

/// This function applies to scanning laser profilers.
/// Computes the average sampling interval in along-track direction.
/// `velocity` is the velocity of the profiler, and `frequency is the
/// frequency of the sampling pattern
#[requires(velocity > 0.0)]
#[requires(frequency > 0.0)]
#[ensures(ret > 0.0)]
pub fn sampled_along_track(velocity : f64, frequency : f64) -> f64 {
	return velocity / frequency;
}

// Radar altimetry

/// Returns the min return time for a radar altimeter or other EM system at height `height`.
/// Computes using the formula $t = \frac{2h}{c}$.
#[requires(height > 0.0)]
#[ensures(ret > 0.0)]
pub fn min_return_time(height : f64) -> f64 {
	return 2.0 * height / C;
}

/// Calculates the footprint radius for a radar altimeter
///
/// Parameters:
/// - `rise_time`: The rise time of the sweep of the altimeter
/// - `height`: The distance from the ground of the altimeter
/// - `adjust_effective_height`: Whether or not to compensate for the curvature of the earth
///
/// Note: if you want to use a different planets' radius (such as defined in
/// `ranged::consts`, you can use `effective_height` and do not adjust.
#[requires(rise_time > 0.0)]
#[requires(height > 0.0)]
#[ensures(ret > 0.0)]
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
#[requires(height > 0.0)]
#[requires(radius.is_some() -> radius.unwrap() > 0.0)]
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
	return C * antenna_height / (antenna_diameter * frequency);
}

// TODO

// Scattered systems

// most basic brdf: R = L1 / E
/// Basic brdf estimate `radiance / irradiance`
#[requires(radiance > 0.0)]
#[requires(irradiance > 0.0)]
pub fn brdf_basic(radiance : f64, irradiance : f64) -> f64 {
	return radiance / irradiance;
}
// TODO: other brdf approximations

/// A basic means of calculating the bistatic scattering coefficient with
/// the approximation of the brdf in `brdf_basic`.
#[requires(radiance > 0.0)]
#[requires(irradiance > 0.0)]
#[requires(angle > 0.0)]
pub fn bistatic_scattering_coefficient_basic(radiance : f64, irradiance : f64, angle : f64) -> f64 {
	let r = brdf_basic(radiance, irradiance);
	return bistatic_scattering_coefficient(r, angle);
}

/// The more general `bistatic_scattering_coefficient` function which
/// takes a known brdf. If brdf is calculated using `brdf_basic` this is equivalent
/// to `bistatic_scattering_coefficient_basic`
#[requires(brdf > 0.0)]
#[requires(angle > 0.0)]
pub fn bistatic_scattering_coefficient(brdf : f64, angle : f64) -> f64 {
	return 4.0 * PI * brdf * angle.cos();
}

/// Computes flux density in a bistatic dual-radar system. However, this is only
/// concerned with the flux density through the scanned surface.
#[requires(antenna_gain > 0.0)]
#[requires(transmitted_power > 0.0)]
#[requires(transmit_dist > 0.0)]
pub fn bistatic_flux_density(
	antenna_gain        : f64 // The antenna's gain, Gt
	, transmitted_power : f64 // Transmitted power Pt
	, transmit_dist     : f64 // The distance from the transmitting antenna to the surface
) -> f64 {
	return antenna_gain * transmitted_power / (4.0 * PI * transmit_dist.powi(2));
}

// TODO: should we also have an irradiance function?

/// Computes the received radar power in a bistatic dual radar system
///
/// Params:
/// - `b_scat_coefficient`: Bistatic scattering coefficient, denoted "gamma"
/// - `collecting_area`   : The size of the collecting area
/// - `effective_area`    : Effective area of receiver, Ar
/// - `antenna_gain`      : The antenna's gain, Gt
/// - `transmitted_power` : Transmitted power Pt
/// - `incoming_angle`    : Incoming angle, theta_0
/// - `exit_angle`        : The exit angle, theta_1
/// - `transmit_dist`     : The distance from the transmitting antenna to the surface
/// - `received_dist`     : The distance to the receiving antenna from the surface
#[requires(b_scat_coefficient > 0.0)]
#[requires(collecting_area > 0.0)]
#[requires(effective_area > 0.0)]
#[requires(antenna_gain > 0.0)]
#[requires(transmitted_power > 0.0)]
#[requires(incoming_angle > 0.0)]
#[requires(exit_angle > 0.0)]
#[requires(transmit_dist > 0.0)]
#[requires(received_dist > 0.0)]
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
	let e = f * incoming_angle.cos();
	let l = b_scat_coefficient * e / (4.0 * PI * exit_angle.cos());
	return l * collecting_area * effective_area / received_dist.powi(2) * exit_angle.cos();
}

// Microwave scatterometry stuff

/// Computes doppler shift given speed, angle, and relative velocity
#[requires(transmitted_freq > 0.0)]
#[requires(velocity > 0.0)]
#[requires(angle >= 0.0 && angle <= 6.29)]
pub fn doppler_shift(transmitted_freq : f64, velocity : f64, angle : f64) -> f64 {
	return 2.0 * transmitted_freq * velocity / C * angle.sin();
}

/// Range resolution for an SLR system
#[requires(tp > 0.0)]
#[requires(theta >= 0.0 && theta <= 6.29)]
pub fn range_resolution(tp : f64, theta : f64) -> f64 {
	return C * tp / (2.0 * theta.sin());
}

/// Phase delay for Synthetic Aperture Radar (SAR) systems.
///
/// Params:
/// - `wave_num`: wavenumber of the radiation
/// - `time`    : Scatter time
/// - `velocity`: velocity of the system
/// - `height`  : vertical difference between system and scatterer
/// - `dist`    : horizontal distance between system and scatterer
#[requires(wave_num > 0.0)]
#[requires(time > 0.0)]
#[requires(velocity > 0.0)]
#[requires(height > 0.0)]
#[requires(dist > 0.0)]
pub fn sar_phase_delay(
	wave_num   : f64 // wavenumber of the radiation
	, time     : f64 // Scatter time
	, velocity : f64 // velocity of the system
	, height   : f64 // vertical difference between system and scatterer
	, dist     : f64 // horizontal distance between system and scatterer
) -> f64 {
	return 2.0 * wave_num * (height.powi(2) + (velocity * time - height).powi(2)).sqrt();
}

/// Computes the noise equivalent power for a LiDAR detector
///
/// Params:
/// - `area`: Area in cm^2
/// - `bandwidth` : bandwidth of the receiver
/// - `detectivity` : detectivity of the receiver
#[requires(area > 0.0)]
#[requires(bandwidth > 0.0)]
#[requires(detectivity > 0.0)]
pub fn noise_equiv_power(area : f64, bandwidth : f64, detectivity : f64) -> f64 {
	return (area * bandwidth).sqrt() / detectivity;
}

// Triangulation and trilateration

/// Triangulates the location between two points to the point equidistant between
/// those points, given known angle and distances.
///
/// ![Diagram from Wikipedia](https://upload.wikimedia.org/wikipedia/commons/thumb/8/8e/Triangulation-boat.png/310px-Triangulation-boat.png)<br>
/// *(Credit: Svjo, Wikipedia, CC-BY-SA 4.0)*
///
/// Parameters
/// - `dist`: The distance between the two measuring points. In the provided
/// diagram, this is the distance between points A and B.
/// - `angle1`: The angle measured by the first system, in radians.
/// $\alpha$ in the image.
/// - `angle2`: The angle measured by the second system, in radians.
/// $\beta$ in the image.
#[requires(dist > 0.0)]
pub fn triangulate(dist : f64, angle1 : f64, angle2 : f64) -> f64 {
	return dist * angle1.sin() * angle2.sin() / (angle1 + angle2).sin();
}

/// Trilaterates the location between three points.
///
/// See [this StackExchange post](https://math.stackexchange.com/questions/884807/find-x-location-using-3-known-x-y-location-using-trilateration)
/// for how this is calculated. (Thank you John from stackexchange)
#[requires(point1.len() == point2.len()
	&& point2.len() == point3.len()
	&& point3.len() == 2)]
#[requires(dist1 > 0.0)]
#[requires(dist2 > 0.0)]
#[requires(dist3 > 0.0)]
pub fn trilaterate(
	point1         : &[f64]
	, dist1        : f64
	, point2       : &[f64]
	, dist2        : f64
	, point3       : &[f64]
	, dist3        : f64
	, result_point : &mut [f64]
) {
	let x1 = point1[0];
	let y1 = point1[1];
	let x2 = point2[0];
	let y2 = point2[1];
	let x3 = point3[0];
	let y3 = point3[1];
	// Only compute squared distances once
	let dist1_sq = dist1.powi(2);
	let dist2_sq = dist2.powi(2);
	let dist3_sq = dist3.powi(2);
	// Only compute squared x and y once
	let x1_sq = x1.powi(2);
	let x2_sq = x2.powi(2);
	let x3_sq = x3.powi(2);
	let y1_sq = y1.powi(2);
	let y2_sq = y2.powi(2);
	let y3_sq = y3.powi(2);
	// Blorp
	let a = 2.0 * (x2 - x1);
	let b = 2.0 * (y2 - y1);
	let c = dist1_sq - dist2_sq - x1_sq + x2_sq - y1_sq + y2_sq;
	let d = 2.0 * (x3 - x2);
	let e = 2.0 * (y3 - y2);
	let f = dist2_sq - dist3_sq - x2_sq + x3_sq - y2_sq + y3_sq;
	// Only compute bd once since it's used twice
	let bd = b * d;
	// Results
	result_point[0] = (c * e - f * b) / (e * a - bd);
	result_point[1] = (c * d - a * f) / (bd - a * e);
}
