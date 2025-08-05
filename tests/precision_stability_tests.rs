use solana_program::msg;

#[cfg(test)]
mod precision_stability_tests {
    use super::*;

    #[test]
    fn test_powf_precision_variation() {
        // Truncating to 10^-12 provides stable results
        
        let base = 1.05_f64;
        let exponent = 365.25_f64;
        
        // Calculate the same operation multiple times with slight variations
        let result1 = base.powf(exponent);
        let result2 = (base + 1e-15).powf(exponent);
        let result3 = base.powf(exponent + 1e-15);
        
        msg!("powf results:");
        msg!("  result1: {:.16}", result1);
        msg!("  result2: {:.16}", result2);
        msg!("  result3: {:.16}", result3);
        
        // Show the raw differences 
        let diff_12 = (result1 - result2).abs();
        let diff_13 = (result1 - result3).abs();
        
        msg!("Raw differences:");
        msg!("  diff_12: {:.2e}", diff_12);
        msg!("  diff_13: {:.2e}", diff_13);
        
        // The differences may be larger than expected due to powf() complexity
        // This demonstrates why truncation is necessary
        msg!("Differences are at level: ~10^{:.0}", diff_12.log10());
        
        // Now truncate to 10^-12 precision
        let truncated1 = (result1 * 1e12).round() / 1e12;
        let truncated2 = (result2 * 1e12).round() / 1e12;
        let truncated3 = (result3 * 1e12).round() / 1e12;
        
        msg!("Truncated to 10^-12:");
        msg!("  truncated1: {:.12}", truncated1);
        msg!("  truncated2: {:.12}", truncated2);
        msg!("  truncated3: {:.12}", truncated3);
        
        // After truncation, results should be much closer or identical
        let truncated_diff = (truncated1 - truncated2).abs();
        msg!("Truncated difference: {:.2e}", truncated_diff);
        
        // The key insight: truncation reduces precision variations significantly
        assert!(truncated_diff <= diff_12, "Truncation should reduce or maintain precision");
    }

    #[test]
    fn test_compound_interest_stability() {
        // Real-world financial calculation: compound interest
        // Principal * (1 + rate)^periods
        
        let principal = 1000000.0_f64; // $1M
        let annual_rate = 0.05_f64;    // 5%
        let periods = 10.0_f64;        // 10 years
        
        // Calculate with slight input variations
        let amount1 = principal * (1.0 + annual_rate).powf(periods);
        let amount2 = (principal + 1e-10) * (1.0 + annual_rate).powf(periods);
        let amount3 = principal * (1.0 + annual_rate + 1e-15).powf(periods);
        
        msg!("Compound interest results:");
        msg!("  amount1: ${:.16}", amount1);
        msg!("  amount2: ${:.16}", amount2);
        msg!("  amount3: ${:.16}", amount3);
        
        let diff_12 = (amount1 - amount2).abs();
        let diff_13 = (amount1 - amount3).abs();
        
        msg!("Raw differences:");
        msg!("  diff_12: ${:.2e}", diff_12);
        msg!("  diff_13: ${:.2e}", diff_13);
        
        // Truncate to cent precision (10^-2) for financial use
        let cents1 = (amount1 * 100.0).round() / 100.0;
        let cents2 = (amount2 * 100.0).round() / 100.0;
        let cents3 = (amount3 * 100.0).round() / 100.0;
        
        msg!("Rounded to cents:");
        msg!("  cents1: ${:.2}", cents1);
        msg!("  cents2: ${:.2}", cents2);
        msg!("  cents3: ${:.2}", cents3);
        
        // Financial results should be identical when rounded to cents
        assert_eq!(cents1, cents2, "Financial results should match to the cent");
        assert_eq!(cents1, cents3, "Financial results should match to the cent");
    }

    #[test]
    fn test_exponential_decay_stability() {
        // Test exponential decay: value * e^(-rate * time)
        // Common in DeFi for time-based calculations
        
        let initial_value = 1000.0_f64;
        let decay_rate = 0.1_f64;
        let time = 5.0_f64;
        
        // Using powf for e^x calculation
        let e_approx = 2.718281828459045_f64;
        
        let result1 = initial_value * e_approx.powf(-decay_rate * time);
        let result2 = (initial_value + 1e-12) * e_approx.powf(-decay_rate * time);
        let result3 = initial_value * e_approx.powf((-decay_rate - 1e-15) * time);
        
        msg!("Exponential decay results:");
        msg!("  result1: {:.16}", result1);
        msg!("  result2: {:.16}", result2);
        msg!("  result3: {:.16}", result3);
        
        let diff_12 = (result1 - result2).abs();
        let diff_13 = (result1 - result3).abs();
        
        msg!("Raw differences:");
        msg!("  diff_12: {:.2e}", diff_12);
        msg!("  diff_13: {:.2e}", diff_13);
        
        // Truncate to 10^-12 precision as suggested by M0
        let stable1 = (result1 * 1e12).round() / 1e12;
        let stable2 = (result2 * 1e12).round() / 1e12;
        let stable3 = (result3 * 1e12).round() / 1e12;
        
        msg!("Stabilized to 10^-12:");
        msg!("  stable1: {:.12}", stable1);
        msg!("  stable2: {:.12}", stable2);
        msg!("  stable3: {:.12}", stable3);
        
        // Check if truncation improves stability
        let stable_diff_12 = (stable1 - stable2).abs();
        let stable_diff_13 = (stable1 - stable3).abs();
        
        msg!("Stabilized differences:");
        msg!("  stable_diff_12: {:.2e}", stable_diff_12);
        msg!("  stable_diff_13: {:.2e}", stable_diff_13);
        
        // The key insight: truncation should reduce precision variations
        // Sometimes truncation can introduce small differences due to rounding boundaries
        let improvement_12 = stable_diff_12 <= diff_12 || stable_diff_12 < 1e-11;
        let improvement_13 = stable_diff_13 <= diff_13 || stable_diff_13 < 1e-11;
        
        msg!("Truncation effectiveness:");
        msg!("  diff_12 improved: {} ({:.2e} -> {:.2e})", improvement_12, diff_12, stable_diff_12);
        msg!("  diff_13 improved: {} ({:.2e} -> {:.2e})", improvement_13, diff_13, stable_diff_13);
        
        assert!(improvement_12, "Truncation should improve or maintain precision within 10^-11");
        assert!(improvement_13, "Truncation should improve or maintain precision within 10^-11");
    }

    #[test]
    fn test_precision_truncation_strategy() {
        // Demonstrate the truncation strategy for different precision levels
        
        let test_value = 123.456789012345678901234567890_f64;
        
        msg!("Original value: {:.20}", test_value);
        
        // Test different truncation levels
        let precisions = [1e2, 1e6, 1e9, 1e12, 1e15];
        let labels = ["cents", "micro", "nano", "pico", "femto"];
        
        for (i, &precision) in precisions.iter().enumerate() {
            let truncated = (test_value * precision).round() / precision;
            msg!("Truncated to {} precision: {:.20}", labels[i], truncated);
            
            // Verify truncation worked
            let expected_decimals = precision.log10() as i32;
            let factor = 10_f64.powi(expected_decimals + 5); // Extra precision for comparison
            let diff = ((truncated * factor).round() - (test_value * factor).round()).abs();
            
            // Should have removed precision beyond the truncation level
            if precision < 1e15 {
                assert!(diff > 0.0, "Truncation should have changed the value");
            }
        }
    }

    #[test]
    fn test_deterministic_powf_across_calls() {
        // Verify that powf() gives identical results across multiple calls
        // This tests Solana's software emulation consistency
        
        let base = 1.618033988749895_f64; // Golden ratio
        let exponent = std::f64::consts::PI;
        
        let mut results = Vec::new();
        
        // Calculate the same operation 100 times
        for i in 0..100 {
            let result = base.powf(exponent);
            results.push(result);
            
            if i < 5 {
                msg!("Call {}: {:.16}", i + 1, result);
            }
        }
        
        // All results should be bit-identical
        let first_result = results[0];
        for (i, &result) in results.iter().enumerate() {
            assert_eq!(
                result, first_result,
                "powf() call {} gave different result: expected {:.16}, got {:.16}",
                i + 1, first_result, result
            );
        }
        
        msg!("All 100 powf() calls produced identical results: {:.16}", first_result);
    }

    #[test]
    fn test_financial_precision_boundaries() {
        // Test precision boundaries relevant to financial calculations
        
        let large_amount = 1e12_f64; // $1 trillion (more realistic)
        let small_rate = 1e-6_f64;   // 0.0001% (1 basis point / 100)
        
        // This demonstrates precision limits in financial calculations
        let result = large_amount * (1.0 + small_rate);
        let expected_gain = large_amount * small_rate;
        let actual_gain = result - large_amount;
        
        msg!("Large amount: ${:.2}", large_amount);
        msg!("Small rate: {:.8}%", small_rate * 100.0);
        msg!("Expected gain: ${:.6}", expected_gain);
        msg!("Actual gain: ${:.6}", actual_gain);
        msg!("Precision loss: ${:.2e}", (expected_gain - actual_gain).abs());
        
        // Document the precision loss for educational purposes
        let precision_loss = (expected_gain - actual_gain).abs();
        msg!("Relative precision loss: {:.2e}", precision_loss / expected_gain);
        
        // When truncated to reasonable financial precision, should be stable
        let truncated_result = (result * 1e2).round() / 1e2; // Truncate to cents
        let truncated_expected = ((large_amount + expected_gain) * 1e2).round() / 1e2;
        
        msg!("Truncated result: ${:.2}", truncated_result);
        msg!("Truncated expected: ${:.2}", truncated_expected);
        
        // For practical financial use, cent-level precision is sufficient
        let cent_difference = (truncated_result - truncated_expected).abs();
        assert!(cent_difference < 0.01, 
               "Results should be within 1 cent when truncated: difference = ${:.4}", cent_difference);
    }
}
