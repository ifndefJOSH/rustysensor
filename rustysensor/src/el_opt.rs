use contracts::*;
// ===================== Electro Optical Systems =====================

mod el_opt {
	mod tables {
		#[derive(Clone, Debug)]
		struct Range {
			index : u8
			, lbound : f64
			, ubound : f64
		}
		let aster = [Range { index : 1, lbound : 0.0, ubound : 0.6e-6 },
					Range { index : 2, lbound : 0.63e-6, ubound : 0.69e-6 },
					Range { index : 3, lbound : 0.76e-6, ubound : 0.86e-6 },
					Range { index : 4, lbound : 1.6e-6, ubound : 1.7e-6 },
					Range { index : 5, lbound : 2.145e-6, ubound : 2.185e-6 },
					Range { index : 6, lbound : 2.185e-6, ubound : 2.225e-6 },
					Range { index : 7, lbound : 2.235e-6, ubound : 2.285e-6 },
					Range { index : 8, lbound : 2.295e-6, ubound : 2.365e-6 },
					Range { index : 9, lbound : 2.365e-6, ubound : 2.430e-6 }];

		let modis = [Range { index : 1, lbound : 6.2e-07, ubound : 6.7e-07 },
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

		let ocm_2 = [Range { index : 1, lbound : 4.04e-07, ubound : 4.24e-07 },
					Range { index : 2, lbound : 4.31e-07, ubound : 4.51e-07 },
					Range { index : 3, lbound : 4.76e-07, ubound : 4.96e-07 },
					Range { index : 4, lbound : 5e-07, ubound : 5.2e-07 },
					Range { index : 5, lbound : 5.46e-07, ubound : 5.66e-07 },
					Range { index : 6, lbound : 6.1e-07, ubound : 6.3e-07 },
					Range { index : 7, lbound : 7.25e-07, ubound : 7.55e-07 },
					Range { index : 8, lbound : 8.45e-07, ubound : 8.85e-07 }];
	}
	#[requires(wavelength > 0, "Wavelength must be greater than 0")]
	#[requires(d > 0 && d < 1, "Distance must be nonzero, but not too large")]
	#[ensures(ret > 0 && ret < 6.29)] // radians
	fn diffraction_angle(n : u32, wavelength : f64, d : f64) -> f64 {
		return ((n as f64) * wavelength / d).asin();
	}

	#[requires(lambda >= 0.52e-6 && lambda <= 2.43e-6, "Wavelength must be in ASTER VNIR region!")]
	#[ensures(ret > 0 && ret < 10)]
	fn aster(lamda : f64) -> u8 {
		if lambda =< 0.6e-6 {
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

	#[requires(lambda >= 4.05e-7 && lambda <= 2.155e-6, "Wavelength must be in accurate MODIS region!")]
	#[ensures(ret > 0 && ret < 19)]
	fn modis(lambda : f64) -> u8 {
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

	#[requires(lambda >= 4.04e-7 && lambda <= 8.85-7, "Wavelength must be in accurate OCM-2 region!")]
	#[ensures(ret > 0 && ret < 8)]
	fn ocm_2(lambda : f64 -> u8 {
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

	#[requires(theta > 0 && theta < 6.28, "Angle must be greater than zero and less than 2PI")]
	#[requires(temp_a > 0 && temp_b1 > 0 && temp_b2 > 0, "All temperatures must be greater than 0")
	#[requires(temp_b2 > temp_a == temp_b1 > temp_a)]
	#[ensures(ret > 0)]
	fn surface_temp(temp_b1 : f64, temp_b2 : f64, temp_a, theta : f64) -> f64 {
		let mut tau : f64 = 0.0; // Filler instantiation
		return surface_temp_tau(temp_b1, temp_b2, temp_a, theta, tau);
	}

	/*
	 * Calculates surface temperature, $T_{b0}$ and optical thickness $\tau$
	 * given data from two separate sensors and known temperatures at those
	 * sensors.
	 *
	 * First, $\tau$ is calculated using the following derivation
	 * \begin{align*}
	 *     T_{b1} &= T_{b0}\exp(-\tau) + T_a (1 - \exp(-\tau)) \\
	 *     T_{b2} &= T_{b0}\exp(-\tau\sec(\theta)) + T_a (1 - \exp(-\tau\sec(\theta))) \\
	 *     T_{b0} &= \frac{T_{b2} - T_A(1 - \exp(-\tau\sec(\theta))}{\exp(-\tau\sec(\theta)} = \frac{T_{b1} - T_A(1 - \exp(-\tau))}{\exp(-\tau)} \\
	 *     \frac{T_{b2} - T_A}{\exp(-\tau\sec{\theta}} + T_A =  \frac{T_{b1} - T_A}{\exp(-\tau} + T_A \\
	 *      \ln\left(\frac{T_{b2} - T_A}{T_{b1} - T_A}\right) &= \tau\sec(\theta) \\
	 *      \tau &= \frac{1}{\sec \theta}\ln\left(\frac{T_{b2} - T_A}{T_{b1} - T_A}\right)
	 * \end{align*}
	 * We therefore compute $\tau$ using the last equation in that list and then use that to calculate $T_{b0}$.
	 * */
	#[requires(theta > 0 && theta < 6.28, "Angle must be greater than zero and less than 2PI")]
	#[requires(temp_a > 0 && temp_b1 > 0 && temp_b2 > 0, "All temperatures must be greater than 0")
	#[requires(temp_b2 > temp_a == temp_b1 > temp_a)]
	#[ensures(ret > 0)]
	fn surface_temp_tau(temp_b1 : f64, temp_b2 : f64, temp_a, theta : f64, tau : &mut f64) -> f64 {
		// find tau
		tau = (1 / theta.sec()) * ((temp_b2 - temp_a) / (temp_b1 - temp_a)).ln();
		return (temp_b1 + temp_a * (1 - (-tau).exp())) / (-tau).exp();
	}
}
