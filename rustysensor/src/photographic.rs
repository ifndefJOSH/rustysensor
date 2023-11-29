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
// ===================== Photographic systems =====================

/// Computes distance from spacial resolution or spacial resolution
/// from distance. Because the relation is given as
/// $d = \frac{1}{2r}$
/// We can also write it as
/// $r = \frac{1}{2d}$
pub fn dist_res(res : f64) -> f64 {
	return 1.0 / (2.0 * res);
}

/// Computes modulation given min and max amplitudes (intensities)
/// Takes: `i_mx`, max intensity
///        `i_mn`, min intensity
pub fn modulation(i_mx : f64, i_mn : f64) -> f64 {
	return (i_mx - i_mn) / (i_mx + i_mn);
}

/// Computes focal length from object and image distance for a single lens system
pub fn focal_len(obj_dist : f64, image_dist : f64) -> f64 {
	let f_inv = 1.0 / obj_dist + 1.0 / image_dist;
	return 1.0 / f_inv;
}

/// Computes the actual (object) distance given focal length and image distance
pub fn actual_dist(image_dist : f64, focal_len : f64) -> f64 {
	let act_inv = 1.0 / focal_len - 1.0 / image_dist;
	return 1.0 / act_inv;
}

/// Calculates film illuminance for a lens
/// Takes: `f_num`: The f/number (sometimes called f-stops) of the lens
///        `lens_incident_luminance`: the incident luminance of the lens
pub fn film_illuminance(f_num : f64, lens_incident_luminance : f64) -> f64 {
	return PI * f_num.powi(2) * lens_incident_luminance / 4.0;
}

/// Performs a radial distortion on a single point (x, y) on an image.
/// The "slope" is the slope of the line for $L(r)$. If the slope is
/// positive, then barrel distortion occurs, else pincushion distortion
/// occurs. The x and y are given assuming (0, 0) is the center of the
/// image (the principle point), not the top-left as many libraries do.
pub fn radial_distort(x : &mut f64, y : &mut f64, slope : Option<f64>) {
	let m = slope.unwrap_or(0.1);
	let r = ((*x).powi(2) + (*y).powi(2)).sqrt();
	let lr = 1.0 + m * r;
	*x = *x + lr;
	*y = *y + lr;

}

// TODO: Future work: radially distort an entire image, include antialiasing

/// Calculates the location on an image of a point in threespace
/// with a camera also at a certain point. Also requires a focal length
/// Currently NOT GPU-accelerated
#[requires(out_pt.len() == 2)]
#[requires(camera_location.len() == 3)]
#[requires(object_location.len() == 3)]
#[requires(f_len > 0.0)]
pub fn image_location(out_pt : &mut [f64], camera_location : &[f64], object_location : &[f64], f_len : f64) -> f64 {
	let x_pr = object_location[0] - camera_location[0];
	let y_pr = object_location[1] - camera_location[1];
	let z_pr = object_location[2] - camera_location[2];
	let u = f_len * x_pr / z_pr;
	let v = f_len * y_pr / z_pr;
	(*out_pt)[0] = u;
	(*out_pt)[1] = v;
}

// TODO: nice eventual addition: given a specific height, find actual x and y location

/// Computes the distance ON THE IMAGE of the vertical object from the image's
/// principle point, i.e., the center. Must know the ground distance ON THE IMAGE
#[requires(f_len > 0.0)]
#[requires(ground_dist > 0.0)]
#[requires(camera_height > 0.0)]
pub fn principle_point_distance(f_len : f64, ground_dist : f64, camera_height : f64) -> f64 {
	return f_len * ground_dist / camera_height;
}

/// Computes the ground distance ON THE IMAGE given the relief distance, camera
/// height, and focal length
#[requires(f_len > 0.0)]
#[requires(princ_pt_dist > 0.0)]
#[requires(camera_height > 0.0)]
pub fn ground_dist(f_len : f64, princ_pt_dist : f64, camera_height : f64) -> f64 {
	return princ_pt_dist * camera_height / f_len;
}

/// Computes the relief displacement of a vertical object far from the ground
/// Takes a focal length, camera height, and object height
#[requires(f_len > 0.0)]
#[requires(ground_dist > 0.0)]
#[requires(camera_height > object_height && object_height > 0.0)]
pub fn relief_displacement(f_len : f64, ground_dist : f64, camera_height : f64, object_height : f64) -> f64 {
	let pt_dist = principle_point_distance(f_len, ground_dist, camera_height);
	return object_height * pt_dist / (camera_height - object_height);
}
