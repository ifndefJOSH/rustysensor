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

fn travel_time(range : f64, group_velocity : f64) -> f64 {
	return 2.0 * range / group_velocity;
}

// Ranged systems

#[requires(signal.len() == noise.len())]
#[ensures(ret >= 0.0)]
fn averaging_rms_snr(signal : &[f64], noise : &[f64]) -> f64 {
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

fn accuracy(rise_time : f64, snr : f64) -> f64 {
	return rise_time / snr;
}

fn range_accuracy(
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

fn range_ambiguity(vg : f64, p_op  : Option<f64>) -> f64 {
	let p : f64 = p_op.unwrap_or(1000.0);
	return vg / 2.0 * p;
}

fn longest_period(vg : f64, h_op  : Option<f64>) -> f64 {
	let h  : f64 = h_op.unwrap_or(200.0);
	return vg / 2.0 * h;
}

fn is_ideal_period(p : f64, vg : f64, h_op : Option<f64>) -> bool {
	return p < longest_period(vg, h_op);
}

// fn sampled_cross_track(

// fn sampled_along_track(

// TODO

// Scattered systems

// most basic brdf: R = L1 / E
fn brdf_basic(radiance : f64, irradiance : f64) -> f64 {
	return radiance / irradiance;
}
// TODO: other brdf approximations

fn bistatic_scattering_coefficient_basic(radiance : f64, irradiance : f64, angle : f64) -> f64 {
	let r = brdf_basic(radiance, irradiance);
	return bistatic_scattering_coefficient(r, angle);
}

fn bistatic_scattering_coefficient(brdf : f64, angle : f64) -> f64 {
	return 4.0 * PI * brdf * angle.cos();
}
