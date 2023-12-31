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

// ===================== Electro Optical Systems =====================

// mod el_opt {
use contracts::*;
use crate::em::consts::*;

/// Tables related to electro optical systems
pub mod tables {
	/// A simple range struct containing the lower and upper bound and the
	/// range index
	#[derive(Clone, Debug)]
	pub struct Range {
		pub index : u8
		, pub lbound : f64
		, pub ubound : f64
	}

	impl Range {
		/// Gets the bandwidth of the range
		pub fn bandwidth(&self) -> f64 {
			return self.ubound - self.lbound;
		}
	}

	/// ASTER VNIR ranges
	pub const aster : [Range; 9]
				= [Range { index : 1, lbound : 0.0, ubound : 0.6e-6 },
				Range { index : 2, lbound : 0.63e-6, ubound : 0.69e-6 },
				Range { index : 3, lbound : 0.76e-6, ubound : 0.86e-6 },
				Range { index : 4, lbound : 1.6e-6, ubound : 1.7e-6 },
				Range { index : 5, lbound : 2.145e-6, ubound : 2.185e-6 },
				Range { index : 6, lbound : 2.185e-6, ubound : 2.225e-6 },
				Range { index : 7, lbound : 2.235e-6, ubound : 2.285e-6 },
				Range { index : 8, lbound : 2.295e-6, ubound : 2.365e-6 },
				Range { index : 9, lbound : 2.365e-6, ubound : 2.430e-6 }];

	/// MODIS ranges
	pub const modis : [Range; 19]
				= [Range { index : 1, lbound : 6.2e-07, ubound : 6.7e-07 },
				Range { index : 2, lbound : 8.41e-07, ubound : 8.76e-07 },
				Range { index : 3, lbound : 4.59e-07, ubound : 4.79e-07 },
				Range { index : 4, lbound : 5.45e-07, ubound : 5.65e-07 },
				Range { index : 5, lbound : 1.23e-06, ubound : 1.25e-06 },
				Range { index : 6, lbound : 1.628e-06, ubound : 1.652e-06 },
				Range { index : 7, lbound : 2.105e-06, ubound : 2.155e-06 },
				Range { index : 8, lbound : 4.05e-07, ubound : 4.2e-07 },
				Range { index : 9, lbound : 4.38e-07, ubound : 4.48e-07 },
				Range { index : 10, lbound : 4.84e-07, ubound : 4.93e-07 },
				Range { index : 11, lbound : 5.26e-07, ubound : 5.36e-07 },
				Range { index : 12, lbound : 5.46e-07, ubound : 5.56e-07 },
				Range { index : 13, lbound : 6.62e-07, ubound : 6.72e-07 },
				Range { index : 14, lbound : 6.73e-07, ubound : 6.83e-07 },
				Range { index : 15, lbound : 7.43e-07, ubound : 7.53e-07 },
				Range { index : 16, lbound : 8.62e-07, ubound : 8.77e-07 },
				Range { index : 17, lbound : 8.9e-07, ubound : 9.2e-07 },
				Range { index : 18, lbound : 9.31e-07, ubound : 9.41e-07 },
				Range { index : 19, lbound : 9.15e-07, ubound : 9.65e-07 }];

	/// OCM 2 ranges
	pub const ocm_2  : [Range; 8]
				= [Range { index : 1, lbound : 4.04e-07, ubound : 4.24e-07 },
				Range { index : 2, lbound : 4.31e-07, ubound : 4.51e-07 },
				Range { index : 3, lbound : 4.76e-07, ubound : 4.96e-07 },
				Range { index : 4, lbound : 5e-07, ubound : 5.2e-07 },
				Range { index : 5, lbound : 5.46e-07, ubound : 5.66e-07 },
				Range { index : 6, lbound : 6.1e-07, ubound : 6.3e-07 },
				Range { index : 7, lbound : 7.25e-07, ubound : 7.55e-07 },
				Range { index : 8, lbound : 8.45e-07, ubound : 8.85e-07 }];

}

/// Computes diffraction angle given number of slits (`n`), `wavelength`,
/// and observational distance `d`.
#[requires(wavelength > 0.0, "Wavelength must be greater than 0")]
#[requires(d > 0.0 && d < 1.0, "Distance must be nonzero, but not too large")]
#[ensures(ret > 0.0 && ret < 6.29)] // radians
pub fn diffraction_angle(n : u32, wavelength : f64, d : f64) -> f64 {
	return ((n as f64) * wavelength / d).asin();
}

/// Takes a wavelength within the valid ASTER VNIR band and returns
/// the band index, between `0` and `10`.
#[requires(lambda >= 0.52e-6 && lambda <= 2.43e-6, "Wavelength must be in ASTER VNIR region!")]
#[ensures(ret > 0 && ret < 10)]
pub fn aster(lambda : f64) -> u8 {
	if lambda <= 0.6e-6 {
		return 1;
	}
	else if lambda >= 0.63e-6 && lambda <= 0.69e-6 {
		return 2;
	}
	// Does not specify 3n vs 3b
	else if lambda >= 0.76e-6 && lambda <= 0.86e-6 {
		return 3;
	}
	else if lambda >= 1.6e-6 && lambda <= 1.7e-6 {
		return 4;
	}
	else if lambda >= 2.145e-6 && lambda <= 2.185e-6 {
		return 5;
	}
	else if lambda >= 2.185e-6 && lambda <= 2.225e-6 {
		return 6;
	}
	else if lambda >= 2.235e-6 && lambda <= 2.285e-6 {
		return 7;
	}
	else if lambda >= 2.295e-6 && lambda <= 2.365e-6 {
		return 8;
	}
	else if lambda >= 2.365e-6 && lambda <= 2.430e-6 {
		return 9;
	}
	else {
		assert!(false, "Invalid aser wavelength");
		return 1; // make rustc happy
	}
}

/// Takes a wavelength in the MODIS region and returns its associated band index
#[requires(lambda >= 4.05e-7 && lambda <= 2.155e-6, "Wavelength must be in accurate MODIS region!")]
#[ensures(ret > 0 && ret < 19)]
pub fn modis(lambda : f64) -> u8 {
	if lambda >= 6.2e-07 && lambda <= 6.7e-07 {
		return 1;
	}
	else if lambda >= 8.41e-07 && lambda <= 8.76e-07 {
		return 2;
	}
	else if lambda >= 4.59e-07 && lambda <= 4.79e-07 {
		return 3;
	}
	else if lambda >= 5.45e-07 && lambda <= 5.65e-07 {
		return 4;
	}
	else if lambda >= 1.23e-06 && lambda <= 1.25e-06 {
		return 5;
	}
	else if lambda >= 1.628e-06 && lambda <= 1.652e-06 {
		return 6;
	}
	else if lambda >= 2.105e-06 && lambda <= 2.155e-06 {
		return 7;
	}
	else if lambda >= 4.05e-07 && lambda <= 4.2e-07 {
		return 8;
	}
	else if lambda >= 4.38e-07 && lambda <= 4.48e-07 {
		return 9;
	}
	else if lambda >= 4.84e-07 && lambda <= 4.93e-07 {
		return 10;
	}
	else if lambda >= 5.26e-07 && lambda <= 5.36e-07 {
		return 11;
	}
	else if lambda >= 5.46e-07 && lambda <= 5.56e-07 {
		return 12;
	}
	else if lambda >= 6.62e-07 && lambda <= 6.72e-07 {
		return 13;
	}
	else if lambda >= 6.73e-07 && lambda <= 6.83e-07 {
		return 14;
	}
	else if lambda >= 7.43e-07 && lambda <= 7.53e-07 {
		return 15;
	}
	else if lambda >= 8.62e-07 && lambda <= 8.77e-07 {
		return 16;
	}
	else if lambda >= 8.9e-07 && lambda <= 9.2e-07 {
		return 17;
	}
	else if lambda >= 9.31e-07 && lambda <= 9.41e-07 {
		return 18;
	}
	else if lambda >= 9.15e-07 && lambda <= 9.65e-07 {
		return 19;
	}
	else {
		assert!(false, "Invalid modis wavelength");
		return 1; // make rustc happy
	}
}

/// Returns the OCM-2 band given an OCM-2 wavelength
/// See also: `el_opt::tables::ocm_2`.
#[requires(lambda >= 4.04e-7 && lambda <= 8.85e-7, "Wavelength must be in accurate OCM-2 region!")]
#[ensures(ret > 0 && ret < 8)]
pub fn ocm_2(lambda : f64) -> u8 {
	if lambda >= 4.04e-07 && lambda <= 4.24e-07 {
		return 1;
	}
	else if lambda >= 4.31e-07 && lambda <= 4.51e-07 {
		return 2;
	}
	else if lambda >= 4.76e-07 && lambda <= 4.96e-07 {
		return 3;
	}
	else if lambda >= 5e-07 && lambda <= 5.2e-07 {
		return 4;
	}
	else if lambda >= 5.46e-07 && lambda <= 5.66e-07 {
		return 5;
	}
	else if lambda >= 6.1e-07 && lambda <= 6.3e-07 {
		return 6;
	}
	else if lambda >= 7.25e-07 && lambda <= 7.55e-07 {
		return 7;
	}
	else if lambda >= 8.45e-07 && lambda <= 8.85e-07 {
		return 8;
	}
	else {
		assert!(false, "Invalid modis wavelength");
		return 1; // make rustc happy
	}
}

// Untrained values for a0, a1, and a2
// Default just averages the two
static mut a0 : f64 = 0.0;
static mut a1 : f64 = 0.5;
static mut a2 : f64 = 0.5;

/// Calculates the surface temperature using the split-window approximation using pre-set
/// coefficients. It is recommended to use a linear least squares library such as the `lstsq` crate
/// to compute these coefficients, and then set them with `set_split_window_coeffs()`
pub unsafe fn surface_temp_split_window(temp_b1 : f64, temp_b2 : f64) -> f64 {
	return a0 + a1 * temp_b1 + a2 * temp_b2
}

/*
 * In this case, we have a vector
 * */
#[requires(temps_b0.len() == temps_b1.len() && temps_b1.len() == temps_b2.len())]
pub unsafe fn train_split_window(temps_b0 : &[f64], temps_b1 : &[f64], temps_b2 : &[f64]) {
	panic!("This function is not yet implemented! I am currently debating whether or not to include a minimizing least squares in this library (due to size and modularity), as it's been done already in a lot of other libraries. For now, use `set_split_window_coeffs` to manually set coefficients with those you get from another linear least squares library, such as the `lstsq` crate.");
	// a0 = 0.0;
	// a1 = 0.0;
	// a2 = 0.0;
	// for _i in 0..temps_b0.len() {
	// 	// TODO
	// }
}

/// Manually sets the split window coefficients
pub unsafe fn set_split_window_coeffs(a0_new : f64, a1_new : f64, a2_new : f64) {
	a0 = a0_new;
	a1 = a1_new;
	a2 = a2_new;
}

/// Computes the surface temp of a two-sensor system without also returning $\tau$
/// See also `surface_temp_tau()`.
#[requires(theta > 0.0 && theta < 6.28, "Angle must be greater than zero and less than 2PI")]
#[requires(temp_a > 0.0 && temp_b1 > 0.0 && temp_b2 > 0.0, "All temperatures must be greater than 0")]
#[requires((temp_b2 > temp_a) == (temp_b1 > temp_a))]
#[ensures(ret > 0.0)]
pub fn surface_temp(temp_b1 : f64, temp_b2 : f64, temp_a : f64, theta : f64) -> f64 {
	let mut tau : f64 = 0.0; // Filler instantiation
	return surface_temp_tau(temp_b1, temp_b2, temp_a, theta, &mut tau);
}

/// Calculates surface temperature, $T_{b0}$ and optical thickness $\tau$
/// given data from two separate sensors and known temperatures at those
/// sensors.
///
/// First, $\tau$ is calculated using the following derivation
/// \begin{align*}
///     T_{b1} &= T_{b0}\exp(-\tau) + T_a (1 - \exp(-\tau)) \\
///     T_{b2} &= T_{b0}\exp(-\tau\sec(\theta)) + T_a (1 - \exp(-\tau\sec(\theta))) \\
///     T_{b0} &= \frac{T_{b2} - T_A(1 - \exp(-\tau\sec(\theta))}{\exp(-\tau\sec(\theta)} = \frac{T_{b1} - T_A(1 - \exp(-\tau))}{\exp(-\tau)} \\
///     \frac{T_{b2} - T_A}{\exp(-\tau\sec{\theta}} + T_A =  \frac{T_{b1} - T_A}{\exp(-\tau} + T_A \\
///      \ln\left(\frac{T_{b2} - T_A}{T_{b1} - T_A}\right) &= \tau\sec(\theta) \\
///      \tau &= \frac{1}{\sec \theta}\ln\left(\frac{T_{b2} - T_A}{T_{b1} - T_A}\right) \\
///      \tau &= \cos \theta \ln\left(\frac{T_{b2} - T_A}{T_{b1} - T_A}\right)
/// \end{align*}
/// We therefore compute $\tau$ using the last equation in that list and then use that to calculate $T_{b0}$.
#[requires(theta > 0.0 && theta < 6.28, "Angle must be greater than zero and less than 2PI")]
#[requires(temp_a > 0.0 && temp_b1 > 0.0 && temp_b2 > 0.0, "All temperatures must be greater than 0")]
#[requires((temp_b2 > temp_a) == (temp_b1 > temp_a))]
#[ensures(ret > 0.0)]
pub fn surface_temp_tau(temp_b1 : f64, temp_b2 : f64, temp_a : f64, theta : f64, tau : &mut f64) -> f64 {
	// find tau
	*tau = (theta.cos()) * ((temp_b2 - temp_a) / (temp_b1 - temp_a)).ln();
	// Used twice, so only calculate once
	let minus_tau_exp = (0.0 - *tau).exp();
	return (temp_b1 + temp_a * (1.0 - minus_tau_exp)) / minus_tau_exp;
}

/// Calculates average spectral radiance given $K_1$ and $K_2$, two parameters related to the
/// specific sensing system. Requires the surface temperature in order to do it.
#[requires(K1 > 0.0 && K2 > 0.0)]
#[requires(temp > 0.0)]
#[ensures(ret > 0.0)]
pub fn avg_spectral_radiance(K1 : f64, K2 : f64, temp : f64) -> f64 {
	return K1 / ((K2 / temp).exp() - 1.0);
}

/// Calculates the Earth's surface temperature given average spectral radiance and sensing system parameters $K_1$ and $K_2$
#[requires(K1 > 0.0 && K2 > 0.0)]
#[requires(avg_radiance > 0.0)]
#[ensures(ret > 0.0)]
pub fn earth_surface_temp(K1 : f64, K2 : f64, avg_radiance : f64) -> f64 {
	return K2 / (K1 / avg_radiance + 1.0).ln()
}

/// Calculates thermal inertia given heat capacity, material density, and thermal conductivity
#[requires(heat_capacity > 0.0)]
#[requires(density > 0.0)]
#[requires(thermal_conductivity > 0.0)]
#[ensures(ret > 0.0)]
pub fn thermal_inertia(heat_capacity : f64, density : f64, thermal_conductivity : f64) -> f64 {
	return (heat_capacity * density * thermal_conductivity).sqrt();
}

/// Calculates thermal wave speed via heat capacity, material density, thermal conductivity, and angular frequency
#[requires(heat_capacity > 0.0)]
#[requires(density > 0.0)]
#[requires(angular_frequency > 0.0)]
#[requires(thermal_conductivity > 0.0)]
#[ensures(ret > 0.0)]
pub fn thermal_wave_speed(heat_capacity : f64, density : f64, thermal_conductivity : f64, angular_frequency : f64) -> f64 {
	return ((2.0 * thermal_conductivity * angular_frequency) / (heat_capacity * density)).sqrt();
}

/// Calculates thermal diffusivity
#[requires(heat_capacity > 0.0)]
#[requires(density > 0.0)]
#[requires(thermal_conductivity > 0.0)]
#[ensures(ret > 0.0)]
pub fn thermal_diffusivity(heat_capacity : f64, density : f64, thermal_conductivity : f64) -> f64 {
	return thermal_conductivity / (heat_capacity * density);
}

/// Calculates the weight factor $\alpha$ of the upward heat flux used in the equation
/// $\alpha(T - \bar{T})$
#[requires(emissivity > 0.0)]
#[requires(mean_temp > 0.0)]
#[ensures(ret > 0.0)]
pub fn upward_heat_flux_weight(mean_temp : f64, emissivity : f64) -> f64 {
	return 4.0 * emissivity * SIGMA * mean_temp.powi(3);
}

/// Calculates upward heat flux given measured temperature, mean temperature, and the emissivity
/// of the surface
#[requires(emissivity > 0.0)]
#[requires(mean_temp > 0.0)]
#[requires(temp > 0.0)]
#[ensures(ret > 0.0)]
pub fn upward_heat_flux(temp : f64, mean_temp : f64, emissivity : f64) -> f64 {
	let alpha : f64 = upward_heat_flux_weight(mean_temp, emissivity);
	return alpha * (temp - mean_temp);
}

// TODO: Hosek-Wilkie and Preetham

// Hosek-Wilkie stuff

static mut params : Option<[[f64; 3]; 9]> = None;

// Coefficients used in the Hosek-Wilkie algorithm
#[requires(params.is_some())]
unsafe fn hw_A(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[0];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_B(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[1];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_C(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[2];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_D(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[3];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_E(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[4];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_F(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[5];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_G(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[6];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_H(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[7];
	return param[0] * param[1] * param[2];
}

#[requires(params.is_some())]
unsafe fn hw_I(zenith : f64, azimuth : f64) -> f64 {
	let param = params.unwrap()[8];
	return param[0] * param[1] * param[2];
}

/// Computes the value of $\chi$, the anisotropic term used in the Hosek-Wilkie algorithm
pub fn hosek_wilkie_anisotropic(g : f64, alpha : f64) -> f64 {
	let alph_cos = alpha.cos();
	return (1.0 + alph_cos.powi(2)) / (1.0 + g.powi(2) - 2.0 * g * alph_cos).powf(1.5);
}

/// Computes the radiance of the sky using the Hosek-Wilkie algorithm according to
/// their first paper.
///
/// - `zenith`: often denoted $\theta$, the solar zenith angle, i.e., the angle between the sun and the zenith.
/// - `azimuth`: often denoted $\gamma$, the solar azimuth angle.
/// - `turbidity`: the turbidity metric used
/// - `g_albedo`: The ground albedo used
///
/// **Note:** Hosek-Wilkie parameters are not provided due to licensing issues (rustysensor is GPLv3 and
/// the parameters are licensed under the BSD-3 license).
///
/// Reference Links:
/// - Parameters available: published by the author [here](https://cgg.mff.cuni.cz/projects/SkylightModelling/)
/// - The original paper [in PDF format](https://cgg.mff.cuni.cz/projects/SkylightModelling/HosekWilkie_SkylightModel_SIGGRAPH2012_Preprint_lowres.pdf)
#[requires(params.is_some())]
pub unsafe fn hosek_wilkie_luminance(zenith : f64, azimuth : f64) -> f64 {
	let A    = hw_A(zenith, azimuth);
	let B    = hw_B(zenith, azimuth);
	let C_hw = hw_C(zenith, azimuth);
	let D    = hw_D(zenith, azimuth);
	let E    = hw_E(zenith, azimuth);
	let F    = hw_F(zenith, azimuth);
	let G    = hw_G(zenith, azimuth);
	let H_hw = hw_H(zenith, azimuth);
	let I    = hw_I(zenith, azimuth);
	let chi  = hosek_wilkie_anisotropic(H_hw, azimuth);
	let zenith_cos = zenith.cos();
	return (
		1.0 + A * (B / zenith_cos + 0.01).exp()) * (C_hw
			+ D * (E * azimuth).exp()
			+ F * azimuth.cos().powi(2)
			+ G * chi
			+ I * zenith_cos.sqrt());
}

/// Sets the parameters for the Hosek Wilkie radiance pattern. Note that
/// patterns can be obtained in C from the original authors [here](https://cgg.mff.cuni.cz/projects/SkylightModelling/)
///
/// These parameters are not included due to licensing restrictions. See the file `LICENSING_COMPATIBILITY.txt` in the
/// Github repository for a full explaination as to why.
#[ensures(params.is_some())]
pub unsafe fn set_hosek_wilkie_params(new_params : [[f64; 3]; 9]) {
	params = Some(new_params);
}
