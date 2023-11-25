use contracts::*;
// ===================== Passive Microwave Systems =====================

mod muwave {
	use em::consts::*;

	enum AntennaType {
		MONOPOLE
		, SHORT_DIPOLE
		, HALF_WAVE_DIPOLE
		, YAGI_YUDA_6
		, RECTANGULAR
		, PARABOLOID // (Circular paraboloid)
	}
	/*
	 * Computes the Johnson/Nyquist noise power of an antenna
	 * */
	fn jnoise_power(antenna_temp : f64, band_size : f64) -> f64 {
		return K * antenna_temp * band_size;
	}

	/*
	 * Computes half power bandwidth. The `size` parameter is dependent on
	 * the type of antenna:
	 * 1. For rectangular, it's the size of the sides
	 * 2. For Circular paraboloid it's the diameter
	 * */
	fn hpbw(lambda : f64, size : f64, atype : AntennaType) -> f64 {
		if atype == MONOPOLE {
			return 0.0; // isomorphic
		}
		else if atype == SHORT_DIPOLE || atype == HALF_WAVE_DIPOLE {
			return 90.0;
		}
		else if atype == YAGI_YUDA_6 {
			return 42.0;
		}
		else if atype == RECTANGULAR {
			return 51.0 * (lambda / size);
		}
		else { // Parabaloid
			return 72.0 * (lambda / size);
		}
	}

	/**
	 * Computes the directivity given beam solid angle
	 * bsa: Beam solid angle
	 * */
	fn directivity(bsa : f64) -> f64 {
		return 4 * PI / bsa;
	}

	fn beam_solid_angle(P: &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
		let s : f64 = step.unwrap_or(0.01);
		// Size of square for integration
		let mut sum : f64 = 0.0;
		let mut theta : f64 = 0.0;
		let mut phi : f64 = 0.0;
		while theta < PI / 2 {
			while phi < 2 * PI {
				sum += P(theta, phi);
				phi += s;
			}
			theta += s;
		}
		return sum;
	}

	fn antenna_temp(TB: &dyn Fn(f64, f64) -> f64, P: &dyn Fn(f64, f64) -> f64, step : Option<f64>) -> f64 {
		let bsa = beam_solid_angle(P, step);
		let s : f64 = step.unwrap_or(0.01);
		// Size of square for integration
		let mut sum : f64 = 0.0;
		let mut theta : f64 = 0.0;
		let mut phi : f64 = 0.0;
		while theta < PI / 2 {
			while phi < 2 * PI {
				sum += TB(theta, phi) * P(theta, phi);
				phi += s;
			}
			theta += s;
		}
		return sum / bsa;
	}

}
