use contracts::*;
// ===================== Basic EM in free space =====================

pub(crate) mod consts {
	// ===================== Constants =====================
	// These values taken from wikipedia
	pub(crate) const C : f64            = 299792458.0;
	// Free space constants
	pub(crate) const MU_0 : f64         = 1.25663706212e-6;
	pub(crate) const EPSILON_0_SI : f64 = 8.8541878128e-12;
	pub(crate) const EPSILON_0_EV : f64 = 55.26349406;
	pub(crate) const Z0 : f64           = 376.73031366686166;    // (MU_0 / EPSILON_0_SI).sqrt();
	pub(crate) const Z0_EV : f64        = 0.0001507945904772106; // (MU_0 / EPSILON_0_EV).sqrt();
	// Other math constants
	pub(crate) const PI : f64           = core::f64::consts::PI;
	// Boltzmann constant
	pub(crate) const K : f64            = 1.380649e-23;
	pub(crate) const SIGMA : f64        = 5.670367e-8;
	// Wave numbers
	pub(crate) const K_E_SI : f64       = 1.0 / (4.0 * PI * EPSILON_0_SI);
	pub(crate) const K_E_EV : f64       = 1.0 / (4.0 * PI * EPSILON_0_EV);
	// Planck Constant
	pub(crate) const H : f64            = 6.62607015e-34;
	pub(crate) const H_EV : f64         = 4.135667696e-15;
	// Electrons
	pub(crate) const MASS_E : f64       = 9.1093837015e-31; // kg
	pub(crate) const CHARGE_E : f64     = 1.602176634e-19;  // coulomb
	// Earth blackbody irradiance
	pub(crate) const EARTH_IRRAD : f64  = 1.37e3;
	// Mean exoatmospheric irradiance
	pub(crate) const EXOATMO_RAD : f64  = 2.02e7;
}

pub mod tables {
	use crate::em::consts::*;
	use std::borrow::Cow;
	#[derive(Clone, Debug)]
	struct Polarizability {
		optical : f64
		, radio : f64
	}
	const AIR_POLARIZABILITY : Polarizability = Polarizability{
		optical : 21.7e-30 / EPSILON_0_SI
		, radio : 21.4e-30 / EPSILON_0_SI
	};
	const CO2_POLARIZABILITY : Polarizability = Polarizability{
		optical : 33.6e-30 / EPSILON_0_SI
		, radio : 36.8e-30 / EPSILON_0_SI
	};
	const HYDROGEN_POLARIZABILITY : Polarizability = Polarizability{
		optical : 9.8e-30 / EPSILON_0_SI
		, radio : 10.1e-30 / EPSILON_0_SI
	};
	const OXYGEN_POLARIZABILITY : Polarizability = Polarizability{
		optical : 20.2e-30 / EPSILON_0_SI
		, radio : 19.8e-30 / EPSILON_0_SI
	};
	const H20_VAPOR_POLARIZABILITY : Polarizability = Polarizability{
		optical : 18.9e-30 / EPSILON_0_SI
		, radio : 368.0e-30 / EPSILON_0_SI
	};

	#[derive(Clone, Debug)]
	struct AtmosFraction {
		chemical : Cow<'static, str>
		, volume_frac : f64
		, mass : f64
	}
	const composition : [AtmosFraction; 12] = [
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

#[requires(f > 0.0, "Frequency must be greater than zero Hz!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == f * 2.0 * PI)]
fn angular_frequency(f : f64) -> f64 {
	return f * 2.0 * PI;
}

#[requires(f > 0.0, "Frequency must be greater than zero Hz!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == C / f)]
fn em_wavelength(f : f64) -> f64 {
	return C / f;
}

#[requires(lambda > 0.0, "Wavelength must be greater than zero!")]
#[ensures(ret > 0.0)]
#[debug_ensures(ret == C / lambda)]
fn em_frequency(lambda : f64) -> f64 {
	return C / lambda;
}

#[requires(lambda > 0.0, "Wavelength must be greater than zero!")]
fn wave_num(lambda : f64) -> u64 {
	return (2.0 * PI / lambda) as u64;
}

#[requires(f > 0.0, "Frequency must be greater than zero!")]
#[ensures(ret > 0.0)]
fn photon_energy(f : f64) -> f64 {
	return H * f;
}

// We can have negative amplitudes as amplitudes are squared
#[ensures(ret > 0.0)]
fn flux_density(amplitude : f64) -> f64 {
	return amplitude.powi(2) / (2.0 * Z0);
}

// TODO: stokes vector

#[requires(velocity >= 0.0, "Velocity must be greater than or equal to zero! (m/s)")]
#[requires(velocity < C, "You cannot go the speed of light!")]
#[requires(angle > 0.0 && angle < 2.0 * PI, "Angle (in radians) must be between 0 and 2PI")]
#[ensures(ret > 0.0)]
fn doppler_ratio(velocity : f64, angle : f64) -> f64 {
	return (1.0 - velocity.powi(2) / C.powi(2)) / (1.0 - velocity * angle.cos() / C).sqrt();
}

/**
* Computes the irradiance using a lambda which has either
* the incoming or outgoing L values. If Lincoming, returns the irradiance,
* if Loutgoing then returns the radiant exitance
* */
#[requires(step.unwrap_or(0.01) > 0.0, "Cannot have zero or negative step for numerical integration.")]
#[ensures(ret > 0.0)]
fn irradiance(L : &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
	let s : f64 = step.unwrap_or(0.01);
	// Size of square for integration
	let s2 : f64 = s.powi(2);
	let mut sum : f64 = 0.0;
	let mut theta : f64 = 0.0;
	let mut phi : f64 = 0.0;
	while theta < PI / 2.0 {
		while phi < 2.0 * PI {
			sum += s2 * L(theta, phi) * theta.cos() * theta.sin();
			phi += s;
		}
		theta += s;
	}
	return sum;
}

/**
* Computes L_f (the spectral radiance) using the Rayleigh-Jeans approximation
* */
#[requires(temp > 0.0, "Cannot have zero or negative temperature (K)")]
#[requires(wavelength > 0.0, "Cannot have zero or negative wavelength (m)")]
fn spectral_radiance_f(temp : f64, wavelength : f64) -> f64 {
	return 2.0 * K * temp / wavelength.powi(2);
}

/**
* Computes L_lambda using Rayleigh-Jeans approximation
* */
#[requires(temp > 0.0, "Cannot have zero or negative temperature (K)")]
#[requires(wavelength > 0.0, "Cannot have zero or negative wavelength (m)")]
#[ensures(ret > 0.0)]
fn spectral_radiance_lambda(temp : f64, wavelength : f64) -> f64 {
	return 2.0 * K * temp * C / wavelength.powi(4);
}

/**
* Computes total black body radiance
* */
#[requires(temp > 0.0, "Cannot have negative or absolute zero temperature!")]
#[ensures(ret > 0.0)]
fn bb_radiation(temp : f64) -> f64 {
	return SIGMA * temp.powi(4);
}

// /**
// * Finds a temperature from emissivity
// * */
// TODO

// Fraunhofer diffraction
// Note: Fraunhofer diffraction just requires fft
// Also, for some window, w,
fn windowed_fraunhofer_diffraction(wnum : f64, window : f64, theta : f64) -> f64 {
	let sinc_arg = wnum * window * theta.sin() / 2.0;
	let sinc = sinc_arg.sin() / sinc_arg;
	return sinc;
}

// ===================== EM radiation interacting with matter =====================
fn electric_permeability(ratio : f64) -> f64 {
	return ratio * EPSILON_0_SI;
}

fn magnetic_permeability(ratio : f64) -> f64 {
	return ratio * MU_0;
}

// Gets the ratio of the magnitudes of (90 degree phase) E field vs B field
// in homogenous materials
fn homogeneous_material_eb_ratio(e_ratio : f64, mu_ratio : f64) -> f64 {
	return C / (e_ratio * mu_ratio).sqrt();
}

fn refractive_index(e_ratio : f64, mu_ratio : f64) -> f64 {
	return (e_ratio * mu_ratio).sqrt();
}

fn absorption_length(angular_frequency : f64, k : f64) -> f64 {
	return C / (2.0 *  angular_frequency * k);
}

// At radio frequencies
fn metal_absorption_length(angular_frequency : f64, conductivity : f64) -> f64 {
	return C * (EPSILON_0_SI / (2.0 * conductivity * angular_frequency)).sqrt();
}

fn gas_dielectric_constant(num_density : u32, polarizability : f64) -> f64 {
	return 1.0 + (num_density as f64 * polarizability) / EPSILON_0_SI;
}

fn plasma_dielectric_constant(num_density : u32, angular_frequency : f64) -> f64 {
	return 1.0 - num_density as f64 * CHARGE_E / (EPSILON_0_SI * MASS_E * angular_frequency.powi(2));
}

fn gas_refractive_index(num_density : u32, polarizability : f64) -> f64 {
	return 1.0 + (num_density as f64 * polarizability) / (2.0 * EPSILON_0_SI);
}

fn metal_dielectric_tau(N : u32, conductivity : f64) -> f64 {
	return MASS_E * conductivity / (N as f64 * CHARGE_E.powi(2));
}

fn metal_dielectric_real(conductivity : f64, angular_frequency : f64, num_density : u32) -> f64 {
	let tau : f64 = metal_dielectric_tau(num_density, conductivity);
	return 1.0 - (conductivity * tau) / (EPSILON_0_SI * (1.0 + angular_frequency.powi(2) * tau.powi(2)));
}

fn metal_dielectric_imag(conductivity : f64, angular_frequency : f64, num_density : u32) -> f64 {
	let tau : f64 = metal_dielectric_tau(num_density, conductivity);
	let denom = EPSILON_0_SI * angular_frequency * (1.0 + angular_frequency.powi(2) * tau.powi(2));
	return conductivity / denom;
}

// The EM frequency at which a plasma becomes transparent
fn plasma_transparency_frequency(num_density : u32) -> f64 {
	let angular = (num_density as f64 * CHARGE_E / (EPSILON_0_SI * MASS_E)).sqrt();
	return angular / (2.0 * PI);
}

// Snell's law
fn exit_angle(entry_angle : f64, current_refractive : f64, new_refractive : f64) -> f64 {
	return (current_refractive * entry_angle.sin() / new_refractive).asin();
}

// ===================== EM radiation interacting with Earths atmosphere =====================

fn angstroem_attenuation(wavelength : f64, base_attenuation : f64, angstroem_exponent : Option<f64>) -> f64 {
	let n = angstroem_exponent.unwrap_or(4.0);
	return base_attenuation * wavelength.powf(0.0 - n);
}

fn fog_liquid_mass_density(num_density : u64, radius : f64) -> f64 {
	let WATER_DENSITY = 1.0;
	return 4.0 * PI * radius.powi(3) * num_density as f64 * WATER_DENSITY / 3.0
}

fn fog_scattering_coefficient(mass_density : f64, radius : f64) -> f64 {
	let WATER_DENSITY = 1.0;
	return 3.0 * mass_density / (4.0 * radius * WATER_DENSITY);
}

fn plasma_refractive_index(num_density : u32, angular_frequency : f64) -> f64 {
	return 1.0 - num_density as f64 * CHARGE_E.powi(2) / (2.0 * EPSILON_0_SI * MASS_E * angular_frequency.powi(2));
}

// Phase velocities may be higher than the speed of light
fn plasma_phase_velocity(num_density : u32, angular_frequency : f64) -> f64 {
	let n = plasma_refractive_index(num_density, angular_frequency);
	return C / n;
}
