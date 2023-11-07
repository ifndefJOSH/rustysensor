
// ===================== Constants =====================
// These values taken from wikipedia
const C : f64            = 299792458;
// Free space constants
const MU_0 : f64         = 1.25663706212e-6;
const EPSILON_0_SI : f64 = 8.8541878128e-12;
const EPSILON_0_EV : f64 = 55.26349406;
const Z0 : f64           = sqrt(MU_0 / EPSILON_0_SI);
const Z0_EV : f64        = sqrt(MU_0 / EPSILON_0_EV);
// Other math constants
const PI : f64           = 3.141592653589793;
// Boltzmann constant
const K : f64            = 1.380649e-23;
const SIGMA : f64        = 5.670367e-8;
// Wave numbers
const K_E_SI : f64       = 1 / (4 * PI * EPSILON_0_SI);
const K_E_EV : f64       = 1 / (4 * PI * EPSILON_0_EV);
// Planck Constant
const H : f64            = 6.62607015e-34;
const H_EV : f64         = 4.135667696e-15;
// Earth blackbody irradiance
const EARTH_IRRAD : f64  = 1.37e3;
// Mean exoatmospheric irradiance
const EXOATMO_RAD : f64  = 2.02e7;

// ===================== Basic EM in free space =====================

fn angular_frequency(f : f64) -> f64 {
    return f * 2 * PI;
}

fn em_wavelength(f : f64) -> f64 {
    return C / f;
}

fn em_frequency(lambda : f64) -> f64 {
    return C / lambda;
}

fn wave_num(wavelength : f64) -> {
    return 2 * PI / wavelength;
}

fn photon_energy(f : f64) -> f64 {
    return H * f;
}

fn flux_density(amplitude : f64) -> f64 {
    return amplitude.pow(2) / (2 * Z0); 
}

// TODO: stokes vector

fn doppler_ratio(velocity : f64, angle : f64) -> f64 {
    return sqrt(1 - velocity.pow(2) / C.pow(2)) / (1 - velocity * cos(angle) / C);
}

/**
 * Computes the irradiance using a lambda which has either
 * the incoming or outgoing L values. If Lincoming, returns the irradiance,
 * if Loutgoing then returns the radiant exitance
 * */
fn irradiance(L : &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
    let s : f64 = step.unwrap_or(0.01);
    // Size of square for integration
    let s2 : f64 = s.pow(2);
    let mut sum : f64 = 0.0;
    let mut theta : f64 = 0.0;
    let mut phi : f64 = 0.0;
    while theta < PI / 2 {
        while phi < 2 * PI {
            sum += s2 * L(theta, phi) * cos(theta) * sin(theta);
            phi += s;
        }
        theta += s;
    }
}

/**
 * Computes L_f (the spectral radiance) using the Rayleigh-Jeans approximation
 * */
fn spectral_radiance_f(temp : f64, wavelength : f64) -> f64 {
    return 2 * K * temp / wavelength.pow(2);
}

/**
 * Computes L_lambda using Rayleigh-Jeans approximation
 * */
fn spectral_radiance_lambda(temp : f64, wavelength : f64) -> f64 {
    return 2 * K * temp * C / wavelength.pow(4);
}

/**
 * Computes total black body radiance
 * */
fn bb_radiation(temp : f64) -> f64 {
    return SIGMA * temp.pow(4);
}

/**
 * Finds a temperature from emissivity
 * */
// TODO

// TODO: Fraunhofer diffraction

// ===================== EM radiation interacting with matter =====================


// ===================== EM radiation interacting with Earths atmosphere =====================
