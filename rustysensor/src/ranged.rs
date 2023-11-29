use contracts::*;

mod ranged {
	use contracts::*;

	fn travel_time(range : f64, group_velocity : f64) -> f64 {
		return 2.0 * range / group_velocity;
	}

	// Ranged systems

	#[requires(signal.length == noise.length)]
	#[ensures(ret >= 0.0)]
	fn averaging_rms_snr(signal : &[f64], noise : &[f64]) -> f64 {
		let mut mean_square_signal : f64 = 0.0;
		for sample in signal.into_iter().enumerate() {
			mean_square_signal += sample.1.powi(2);
		}
		mean_square_signal /= signal.length;
		let mut mean_square_noise : f64 = 0.0;
		for sample in noise.into_iter().enumerate() {
			mean_square_noise += sample.1.powi(2);
		}
		mean_square_noise /= signal.length;
		return (mean_square_signal / mean_square_noise).sqrt()
	}

	fn accuracy(rise_time : f64, snr : f64) -> f64 {
		return rise_time / snr;
	}

	fn range_accuracy(
		vg : f64
		, tr_op   : Option<f64>
		, S_op  : Option<f64>
		, v_op  : Option<f64>
		, H_op  : Option<f64>
		, p_op  : Option<f64>
		, del_theta_op : Option<f64>
	) -> f64 {
		// Uses the defaults for an airborne system
		let tr : f64 = tr_op.unwrap_or(5.0e-9);
		let S  : f64 = S_op.unwrap_or(1.0);
		let v  : f64 = v_op.unwrap_or(50.0);
		let H  : f64 = H_op.unwrap_or(200.0);
		let p  : f64 = p_op.unwrap_or(1000.0);
		let del_theta : f64 = del_theta_op.unwrap_or(0.001);
		return vg * tr / (2.0 * S) * (v / (p * H * del_theta)).sqrt();
	}

	fn range_ambiguity(vg : f64, p_op  : Option<f64>) -> f64 {
		let p : f64 = p_op.unwrap_or(1000.0);
		return vg / 2.0 * p;
	}

	fn longest_period(vg : f64, H_op  : Option<f64>) -> f64 {
		let H  : f64 = H_op.unwrap_or(200.0);
		return vg / 2.0 * P;
	}

	fn is_ideal_period(p : f64, vg : f64, H_op  : Option<f64>) -> bool {
		return p < longest_period(vg, H_op);
	}

	// fn sampled_cross_track(

	// fn sampled_along_track(

	// TODO

	// Scattered systems
}
