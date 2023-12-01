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
// ===================== Basic EM in free space =====================

/// General electromagnetic constants
pub mod consts {
	// ===================== Constants =====================
	// These values taken from wikipedia
	/// The speed of light
	pub const C : f64            = 299792458.0;
	// Free space constants
	/// The magnetic permeability of free space
	pub const MU_0 : f64         = 1.25663706212e-6;
	/// The electric permeability of free space in SI units
	pub const EPSILON_0_SI : f64 = 8.8541878128e-12;
	/// The electric permeability of free space in eV units
	pub const EPSILON_0_EV : f64 = 55.26349406;
	/// Free space impedance in SI units
	pub const Z0 : f64           = 376.73031366686166;    // (MU_0 / EPSILON_0_SI).sqrt();
	/// Free space impedance in eV units
	pub const Z0_EV : f64        = 0.0001507945904772106; // (MU_0 / EPSILON_0_EV).sqrt();
	// Other math constants
	pub const PI : f64           = core::f64::consts::PI;
	/// Boltzmann constant
	pub const K : f64            = 1.380649e-23;
	pub const SIGMA : f64        = 5.670367e-8;
	// Wave numbers
	/// Wave number constant in SI units
	pub const K_E_SI : f64       = 1.0 / (4.0 * PI * EPSILON_0_SI);
	/// Wave number constant in eV units
	pub const K_E_EV : f64       = 1.0 / (4.0 * PI * EPSILON_0_EV);
	// Planck Constant
	/// Planck constant in SI units
	pub const H : f64            = 6.62607015e-34;
	/// Planck constant in eV units
	pub const H_EV : f64         = 4.135667696e-15;
	// Electrons
	/// Mass of the electron, $m_e$, in kilograms
	pub const MASS_E : f64       = 9.1093837015e-31; // kg
	/// The charge of the electron, $e$, in coulombs
	pub const CHARGE_E : f64     = 1.602176634e-19;  // coulomb
	/// Earth blackbody irradiance
	pub const EARTH_IRRAD : f64  = 1.37e3;
	/// Mean exoatmospheric irradiance
	pub const EXOATMO_RAD : f64  = 2.02e7;
}

/// Tables of polarizability, composition, etc.
pub mod tables {
	use crate::em::consts::*;
	use std::borrow::Cow;
	#[derive(Clone, Debug)]
	struct Polarizability {
		optical : f64
		, radio : f64
	}
	/// The optical and radio polarizability of air
	pub const AIR_POLARIZABILITY : Polarizability = Polarizability{
		optical : 21.7e-30 / EPSILON_0_SI
		, radio : 21.4e-30 / EPSILON_0_SI
	};
	/// The optical and radio polarizability of CO2
	pub const CO2_POLARIZABILITY : Polarizability = Polarizability{
		optical : 33.6e-30 / EPSILON_0_SI
		, radio : 36.8e-30 / EPSILON_0_SI
	};
	/// The optical and radio polarizability of Hydrogen gas
	pub const HYDROGEN_POLARIZABILITY : Polarizability = Polarizability{
		optical : 9.8e-30 / EPSILON_0_SI
		, radio : 10.1e-30 / EPSILON_0_SI
	};
	/// The optical and radio polarizability of Oxygen gas
	pub const OXYGEN_POLARIZABILITY : Polarizability = Polarizability{
		optical : 20.2e-30 / EPSILON_0_SI
		, radio : 19.8e-30 / EPSILON_0_SI
	};
	/// The optical and radio polarizability of water vapor
	pub const H20_VAPOR_POLARIZABILITY : Polarizability = Polarizability{
		optical : 18.9e-30 / EPSILON_0_SI
		, radio : 368.0e-30 / EPSILON_0_SI
	};

	/// A particular fraction of the atmosphere
	#[derive(Clone, Debug)]
	pub struct AtmosFraction {
		chemical : Cow<'static, str>
		, volume_frac : f64
		, mass : f64
	}
	/// Total composition of the atmosphere
	pub const composition : [AtmosFraction; 12] = [
		AtmosFraction{ chemical :   std::borrow::Cow::Borrowed("N2" ), volume_frac : 0.7808, mass : 8910.0 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("O2" ), volume_frac : 0.2095, mass : 2093.0 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("Ar" ), volume_frac : 9.34e-3, mass : 133.0 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("CO2"), volume_frac : 3.9e-4, mass : 6.4 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("H2O"), volume_frac : 2.8e-3, mass : 180.0 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("Ne" ), volume_frac : 1.8e-6, mass : 9.9e-3 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("SO2"), volume_frac : 1.0e-6, mass : 2.6e-2 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("H2" ), volume_frac : 5.0e-7, mass : 4.0e-4 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("O3" ), volume_frac : 1.0e-6, mass : 5.3e-3 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("N2O"), volume_frac : 2.7e-7, mass : 4.0e-3 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("Xe" ), volume_frac : 9.0e-8, mass : 4.0e-3 }
		, AtmosFraction{ chemical : std::borrow::Cow::Borrowed("NO2"), volume_frac : 2.0e-8, mass : 4.0e-4 }
		];
}

use crate::em::consts::*;

/// Computes angular frequency from regular frequency
#[requires(f > 0.0, "Frequency must be greater than zero Hz!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == f * 2.0 * PI)]
pub fn angular_frequency(f : f64) -> f64 {
	return f * 2.0 * PI;
}

/// Computes the electromagnetic wavelength given regular frequency
#[requires(f > 0.0, "Frequency must be greater than zero Hz!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == C / f)]
pub fn em_wavelength(f : f64) -> f64 {
	return C / f;
}

/// Computes the electromagnetic frequency from wavelength
#[requires(lambda > 0.0, "Wavelength must be greater than zero!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == C / lambda)]
pub fn em_frequency(lambda : f64) -> f64 {
	return C / lambda;
}

/// Computes wave number given wavelength
#[requires(lambda > 0.0, "Wavelength must be greater than zero!")]
pub fn wave_num(lambda : f64) -> u64 {
	return (2.0 * PI / lambda) as u64;
}

/// Computes photon energy given frequency
#[requires(f > 0.0, "Frequency must be greater than zero!")]
#[ensures(ret > 0.0)]
pub fn photon_energy(f : f64) -> f64 {
	return H * f;
}

/// Computes flux density of E field given its amplitude
// We can have negative amplitudes as amplitudes are squared
#[ensures(ret > 0.0)]
pub fn flux_density(amplitude : f64) -> f64 {
	return amplitude.powi(2) / (2.0 * Z0);
}

// TODO: stokes vector

/// Computes Doppler ratio given velocity of system and angle at system
#[requires(velocity >= 0.0, "Velocity must be greater than or equal to zero! (m/s)")]
#[requires(velocity < C, "You cannot go the speed of light!")]
#[requires(angle > 0.0 && angle < 2.0 * PI, "Angle (in radians) must be between 0 and 2PI")]
#[ensures(ret > 0.0)]
pub fn doppler_ratio(velocity : f64, angle : f64) -> f64 {
	return (1.0 - velocity.powi(2) / C.powi(2)) / (1.0 - velocity * angle.cos() / C).sqrt();
}


/// Computes the irradiance using a lambda which has either
/// the incoming or outgoing L values. If Lincoming, returns the irradiance,
/// if Loutgoing then returns the radiant exitance
#[requires(step.unwrap_or(0.01) > 0.0, "Cannot have zero or negative step for numerical integration.")]
#[ensures(ret > 0.0)]
pub fn irradiance(L : &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
	let s : f64 = step.unwrap_or(0.01);
	// Size of square for integration
	let s2 : f64 = s.powi(2);
	let mut sum : f64 = 0.0;
	let mut theta : f64 = 0.0;
	let mut phi : f64 = 0.0;
	while theta < PI / 2.0 {
		while phi < 2.0 * PI {
			sum += s2 * L(theta, phi) * theta.cos() * theta.sin() * s2;
			phi += s;
		}
		theta += s;
	}
	return sum;
}


/// Computes $L_f$ (the spectral radiance) using the Rayleigh-Jeans approximation
/// It is computed using the following formula:
/// $$L_f = 2K\frac{T}{\lambda^2}$$
#[requires(temp > 0.0, "Cannot have zero or negative temperature (K)")]
#[requires(wavelength > 0.0, "Cannot have zero or negative wavelength (m)")]
pub fn spectral_radiance_f(temp : f64, wavelength : f64) -> f64 {
	return 2.0 * K * temp / wavelength.powi(2);
}

/// Computes $L_lambda$ using Rayleigh-Jeans approximation
/// It is computed using the following formula:
/// $$L_\lambda = 2K\frac{Tc}{\lambda^2}$$
#[requires(temp > 0.0, "Cannot have zero or negative temperature (K)")]
#[requires(wavelength > 0.0, "Cannot have zero or negative wavelength (m)")]
#[ensures(ret > 0.0)]
pub fn spectral_radiance_lambda(temp : f64, wavelength : f64) -> f64 {
	return 2.0 * K * temp * C / wavelength.powi(4);
}

/// Computes total black body radiance
/// Formula:
/// $radiation = \sigma T^4$
#[requires(temp > 0.0, "Cannot have negative or absolute zero temperature!")]
#[ensures(ret > 0.0)]
pub fn bb_radiation(temp : f64) -> f64 {
	return SIGMA * temp.powi(4);
}

// /**
// * Finds a temperature from emissivity
// * */
// TODO

/// Fraunhofer diffraction
/// Note: Fraunhofer diffraction generally just requires fft
/// For some window, w, we can just use this $sinc()$ approximation:
/// $diffraction = \frac{n w \sin(\theta)}{2}$
pub fn windowed_fraunhofer_diffraction(wnum : f64, window : f64, theta : f64) -> f64 {
	let sinc_arg = wnum * window * theta.sin() / 2.0;
	let sinc = sinc_arg.sin() / sinc_arg;
	return sinc;
}

// ===================== EM radiation interacting with matter =====================

/// Computes $\epsilon = \epsilon_r\epsilon_0$
pub fn electric_permeability(ratio : f64) -> f64 {
	return ratio * EPSILON_0_SI;
}

/// Computes $\mu = \mu\mu_0$
pub fn magnetic_permeability(ratio : f64) -> f64 {
	return ratio * MU_0;
}

/// Gets the ratio of the magnitudes of (90 degree phase) E field vs B field
/// in homogenous materials
pub fn homogeneous_material_eb_ratio(e_ratio : f64, mu_ratio : f64) -> f64 {
	return C / (e_ratio * mu_ratio).sqrt();
}

/// Computes refractive index given $\epsilon_r$ and $\mu_r$.
pub fn refractive_index(e_ratio : f64, mu_ratio : f64) -> f64 {
	return (e_ratio * mu_ratio).sqrt();
}

/// Computes absorption length
pub fn absorption_length(angular_frequency : f64, k : f64) -> f64 {
	return C / (2.0 *  angular_frequency * k);
}

/// At radio frequencies, we can use this approximation
/// to compute the absorption length for metals given conductivity
pub fn metal_absorption_length(angular_frequency : f64, conductivity : f64) -> f64 {
	return C * (EPSILON_0_SI / (2.0 * conductivity * angular_frequency)).sqrt();
}

/// Computes the dielectric constant of a gas
pub fn gas_dielectric_constant(num_density : u32, polarizability : f64) -> f64 {
	return 1.0 + (num_density as f64 * polarizability) / EPSILON_0_SI;
}

/// Computes the dielectric constant of a plasma
pub fn plasma_dielectric_constant(num_density : u32, angular_frequency : f64) -> f64 {
	return 1.0 - num_density as f64 * CHARGE_E / (EPSILON_0_SI * MASS_E * angular_frequency.powi(2));
}

/// Computes the refractive index of a gas
pub fn gas_refractive_index(num_density : u32, polarizability : f64) -> f64 {
	return 1.0 + (num_density as f64 * polarizability) / (2.0 * EPSILON_0_SI);
}

/// Computes the $\tau$ component of $\epsilon$ for metals
fn metal_dielectric_tau(N : u32, conductivity : f64) -> f64 {
	return MASS_E * conductivity / (N as f64 * CHARGE_E.powi(2));
}

/// Computes the real part of $\epsilon$ for a metal
pub fn metal_dielectric_real(conductivity : f64, angular_frequency : f64, num_density : u32) -> f64 {
	let tau : f64 = metal_dielectric_tau(num_density, conductivity);
	return 1.0 - (conductivity * tau) / (EPSILON_0_SI * (1.0 + angular_frequency.powi(2) * tau.powi(2)));
}

/// Computes the imaginary part of $\epsilon$ for a metal
pub fn metal_dielectric_imag(conductivity : f64, angular_frequency : f64, num_density : u32) -> f64 {
	let tau : f64 = metal_dielectric_tau(num_density, conductivity);
	let denom = EPSILON_0_SI * angular_frequency * (1.0 + angular_frequency.powi(2) * tau.powi(2));
	return conductivity / denom;
}

/// Computes the EM frequency at which a plasma becomes transparent
pub fn plasma_transparency_frequency(num_density : u32) -> f64 {
	let angular = (num_density as f64 * CHARGE_E / (EPSILON_0_SI * MASS_E)).sqrt();
	return angular / (2.0 * PI);
}

/// Using Snell's law, computes the exit angle of an entrant ray of
/// light given two refracting indexes and an entrance angle.
/// Snell's law is given by $n_1 \sin(\theta_1) = n_2 \sin(\theta_2)$
pub fn exit_angle(entry_angle : f64, current_refractive : f64, new_refractive : f64) -> f64 {
	return (current_refractive * entry_angle.sin() / new_refractive).asin();
}

// ===================== EM radiation interacting with Earths atmosphere =====================

/// Computes the Angström attenuation given the base attenuation
pub fn angstroem_attenuation(wavelength : f64, base_attenuation : f64, angstroem_exponent : Option<f64>) -> f64 {
	let n = angstroem_exponent.unwrap_or(4.0);
	return base_attenuation * wavelength.powf(0.0 - n);
}

/// Computes the liquid mass density of fog
pub fn fog_liquid_mass_density(num_density : u64, radius : f64) -> f64 {
	let WATER_DENSITY = 1.0;
	return 4.0 * PI * radius.powi(3) * num_density as f64 * WATER_DENSITY / 3.0
}

/// Computes the scattering coefficient of fog.
pub fn fog_scattering_coefficient(mass_density : f64, radius : f64) -> f64 {
	let WATER_DENSITY = 1.0;
	return 3.0 * mass_density / (4.0 * radius * WATER_DENSITY);
}

/// Computes the refractive index of a plasma
pub fn plasma_refractive_index(num_density : u32, angular_frequency : f64) -> f64 {
	return 1.0 - num_density as f64 * CHARGE_E.powi(2) / (2.0 * EPSILON_0_SI * MASS_E * angular_frequency.powi(2));
}

/// Computes the phase velocity of a plasma.
/// NOTE: Phase velocities may be higher than the speed of light
pub fn plasma_phase_velocity(num_density : u32, angular_frequency : f64) -> f64 {
	let n = plasma_refractive_index(num_density, angular_frequency);
	return C / n;
}
