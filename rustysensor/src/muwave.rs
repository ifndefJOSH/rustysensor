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

	mod instruments {
		enum Polarization {
			H, V, R, L
		}

		#[derive(Copy, Clone)]
		struct Band {
			f_min : f64    // frequency min (GHz)
			, f_max : f64  // frequency max (GHz)
			, b : f64      // bandwidth (MHz)
			, p : Polarization
			, delta_t : f64 // Temp differential (Kelvin)
			, res_x : u16   // Resolution horizontal (km)
			, res_y : u16   // Resolution vertical (km)
		}

		#[derive(Copy, Clone)]
		struct Channel {
			channel : u8
			, f_min : f64  // frequency min (GHz)
			, f_max : f64  // frequency max (GHz)
			, b : f64      // bandwidth (MHz)
			, p : Polarization
			, bands : u8   // band count
			, delta_t : f64 // Temp differential (Kelvin)
		}

		// TODO: SSMIS and MSMR tables
		// TODO: AMSU-A and MHS Tables
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
		if atype == AntennaType::MONOPOLE {
			return 0.0; // isomorphic
		}
		else if atype == AntennaType::SHORT_DIPOLE || atype == AntennaType::HALF_WAVE_DIPOLE {
			return 90.0;
		}
		else if atype == AntennaType::YAGI_YUDA_6 {
			return 42.0;
		}
		else if atype == AntennaType::RECTANGULAR {
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

	fn forward_gain(efficiency : f64, P: &dyn Fn(f64, f64) -> f64) -> f64 {
		// Directivity
		let d = 4 * PI / beam_solid_angle(P);
		return efficiency * d;
	}

	fn spectral_radiance(tb : f64, wavelength : f64) {
		return 2 * K * tb / wavelength.pow(2);
	}

	fn spectral_flux_density(tb : f64, wavelength : f64, small_angle : f64) -> f64 {
		return 2 * K * tb * small_angle / wavelength.pow(2);
	}

	fn effective_area(wavelength : f64, P: &dyn Fn(f64, f64) -> f64) -> f64 {
		let bsa = beam_solid_angle(P);
		return wavelength.pow(2) / bsa;
	}

	fn sensitivity(sys_temp : f64, C : Option<f64>, del_t : Option<f64>, del_f : Option<f64>) -> f64 {
		let c_val = C.unwrap_or(5);
		let d_t = del_t.unwrap_or(0.01);
		let d_f = del_f.unwrap_or(0.01);
		return C * sys_temp / (d_t * d_f).sqrt();
	}

	fn xpgr(t_19h : f64, t_37v : f64) -> f64 {
		return (t_19h - t_37v) / (t_19h + t_37v);
	}

	fn corrected_temp(tau : f64, T : Option<f64>) -> f64 {
		return T.unwrap_or(250.0) * (1 - (-tau).exp());
	}

}
