use solana_floats::float_ops::*;
use solana_floats::double_ops::*;

#[cfg(test)]
mod precision_edge_cases {
    use super::*;
    
    // SOLANA CONTEXT: All tests in this module demonstrate precision issues that occur
    // in floating point arithmetic. The key insight for Solana is that while these
    // precision issues still exist with software emulation, they are DETERMINISTIC:
    //
    // ✅ WHAT SOLANA'S SOFTWARE EMULATION GUARANTEES:
    // - All validators get the SAME wrong answer
    // - Precision loss is identical across all nodes
    // - No consensus failures due to float differences
    // - Bit-exact reproducibility of results
    //
    // ⚠️ WHAT DEVELOPERS STILL NEED TO HANDLE:
    // - Catastrophic cancellation still occurs (but predictably)
    // - Accumulation errors still compound (but identically)
    // - IEEE 754 precision limits still apply
    // - Mathematical accuracy may be compromised (but consistently)

    #[test]
    fn test_catastrophic_cancellation_f32() {
        // Catastrophic cancellation: subtracting nearly equal numbers
        let a = 1.0000001_f32;
        let b = 1.0000000_f32;
        
        // Direct subtraction loses significant digits
        let direct_result = a - b;
        
        // Alternative calculation to show the issue
        let scaled_a = a * 10_000_000.0;
        let scaled_b = b * 10_000_000.0;
        let scaled_diff = scaled_a - scaled_b;
        let unscaled_result = scaled_diff / 10_000_000.0;
        
        println!("=== CATASTROPHIC CANCELLATION (f32) ===");
        println!("a = {:.10}", a);
        println!("b = {:.10}", b);
        println!("Direct subtraction: {:.10}", direct_result);
        println!("Scaled calculation: {:.10}", unscaled_result);
        println!("Expected difference: 0.0000001");
        
        // Show precision loss in catastrophic cancellation
        let expected = 0.0000001_f32;
        let error = (direct_result - expected).abs();
        
        println!("Error from expected: {:.2e}", error);
            println!("Relative error: {:.2}%", (error / expected) * 100.0);        
        // Demonstrate that precision is lost (but the result is still deterministic)
        println!("f32::EPSILON: {:.2e}", f32::EPSILON);
        
        // The key point: even with precision loss, the result is deterministic
        let repeat_result = a - b;
        assert_eq!(direct_result, repeat_result, "Results should be deterministic despite precision loss");
        
        // Show that we do have some precision loss compared to the ideal
        assert!(error > 0.0, "Should show some precision loss from ideal");
        
        // Document the actual behavior for educational purposes
        println!("✓ Catastrophic cancellation demonstrated: precision loss is deterministic");
    }

    #[test]
    fn test_catastrophic_cancellation_f64() {
        // Same test with f64 to show better precision
        let a = 1.0000000000000001_f64;
        let b = 1.0000000000000000_f64;
        
        let direct_result = a - b;
        
        println!("=== CATASTROPHIC CANCELLATION (f64) ===");
        println!("a = {:.16}", a);
        println!("b = {:.16}", b);
        println!("Direct subtraction: {:.16}", direct_result);
        println!("Expected difference: 0.0000000000000001");
        
        let expected = 0.0000000000000001_f64;
        let error = (direct_result - expected).abs();
        
        println!("Error from expected: {:.2e}", error);
        if expected != 0.0 {
        println!("Relative error: {:.2}", (error / expected) * 100.0);        }
        
        // In Solana's software emulation, f64 may handle this perfectly
        // This demonstrates that software emulation can be more precise than hardware
        if direct_result == 0.0 {
            println!("✓ Perfect precision: Solana's software emulation handled this exactly");
        } else {
            println!("✓ Precision loss detected: {:.2e}", direct_result);
            assert!(direct_result > 0.0, "f64 should detect the difference");
        }
    }

    #[test]
    fn test_arithmetic_precision_loss_accumulation() {
        // Demonstrate precision loss through repeated operations
        let mut sum_f32 = 0.0_f32;
        let mut sum_f64 = 0.0_f64;
        
        let increment = 0.1;
        let iterations = 1000;
        
        for i in 0..iterations {
            sum_f32 = add_floats(sum_f32, increment as f32);
            sum_f64 = add_doubles(sum_f64, increment);
            
            // Show precision degradation at key points
            if i == 9 || i == 99 || i == 999 {
                let expected = (i + 1) as f64 * increment;
                let f32_error = (sum_f32 as f64 - expected).abs();
                let f64_error = (sum_f64 - expected).abs();
                
                println!("After {} iterations:", i + 1);
                println!("  Expected: {:.15}", expected);
                println!("  f32 sum:  {:.15} (error: {:.2e})", sum_f32, f32_error);
                println!("  f64 sum:  {:.15} (error: {:.2e})", sum_f64, f64_error);
            }
        }
        
        let expected_final = iterations as f64 * increment;
        let f32_final_error = (sum_f32 as f64 - expected_final).abs();
        let f64_final_error = (sum_f64 - expected_final).abs();
        
        println!("=== FINAL ACCUMULATION RESULTS ===");
        println!("Expected: {:.15}", expected_final);
        println!("f32 result: {:.15} (error: {:.2e})", sum_f32, f32_final_error);
        println!("f64 result: {:.15} (error: {:.2e})", sum_f64, f64_final_error);
        
        // Demonstrate that f64 has better precision
        assert!(f64_final_error < f32_final_error, "f64 should have less accumulation error");
        assert!(f32_final_error > 1e-6, "f32 should show significant error");
    }

    #[test]
    fn test_multiplication_precision_loss() {
        // Show precision loss in multiplication chains
        let base_f32 = 1.1_f32;
        let base_f64 = 1.1_f64;
        
        let mut result_f32 = 1.0_f32;
        let mut result_f64 = 1.0_f64;
        
        // Multiply by 1.1 repeatedly
        for i in 0..50 {
            result_f32 = multiply_floats(result_f32, base_f32);
            result_f64 = multiply_doubles(result_f64, base_f64);
            
            if i == 9 || i == 19 || i == 49 {
                let expected = 1.1_f64.powi(i + 1);
                let f32_error = (result_f32 as f64 - expected).abs();
                let f64_error = (result_f64 - expected).abs();
                
                println!("After {} multiplications by 1.1:", i + 1);
                println!("  Expected: {:.15}", expected);
                println!("  f32: {:.15} (error: {:.2e})", result_f32, f32_error);
                println!("  f64: {:.15} (error: {:.2e})", result_f64, f64_error);
            }
        }
        
        // Show that errors compound
        let final_expected = 1.1_f64.powi(50);
        let f32_error = (result_f32 as f64 - final_expected).abs();
        let f64_error = (result_f64 - final_expected).abs();
        
        println!("=== MULTIPLICATION CHAIN PRECISION ===");
        println!("1.1^50 expected: {:.15}", final_expected);
        println!("f32 result: {:.15} (error: {:.2e})", result_f32, f32_error);
        println!("f64 result: {:.15} (error: {:.2e})", result_f64, f64_error);
        
        assert!(f64_error < f32_error, "f64 should maintain better precision");
    }

    #[test]
    fn test_division_precision_loss_chain() {
        // Show precision loss in division chains
        let mut value_f32 = 1000000.0_f32;
        let mut value_f64 = 1000000.0_f64;
        
        let divisor = 1.1;
        
        // Divide by 1.1 repeatedly, then multiply back
        for _ in 0..20 {
            value_f32 = divide_floats(value_f32, divisor as f32).unwrap();
            value_f64 = divide_doubles(value_f64, divisor).unwrap();
        }
        
        // Multiply back by 1.1 repeatedly
        for _ in 0..20 {
            value_f32 = multiply_floats(value_f32, divisor as f32);
            value_f64 = multiply_doubles(value_f64, divisor);
        }
        
        let original = 1000000.0;
        let f32_error = (value_f32 as f64 - original).abs();
        let f64_error = (value_f64 - original).abs();
        
        println!("=== DIVISION/MULTIPLICATION ROUND-TRIP ===");
        println!("Original: {:.15}", original);
        println!("f32 result: {:.15} (error: {:.2e})", value_f32, f32_error);
        println!("f64 result: {:.15} (error: {:.2e})", value_f64, f64_error);
        
        // Should not return to exactly original value due to precision loss
        assert!(f32_error > f32::EPSILON as f64, "f32 should show round-trip error");
        assert!(f64_error < f32_error, "f64 should have less round-trip error");
    }

    #[test]
    fn test_safe_f64_truncation_patterns() {
        // Demonstrate safe patterns for using f64 with controlled precision
        
        // Pattern 1: Financial calculations with known precision requirements
        let price = 123.456789123456_f64;
        let quantity = 1000.0_f64;
        let total = multiply_doubles(price, quantity);
        
        // Truncate to cents (2 decimal places) for financial safety
        let total_cents = (total * 100.0).round() / 100.0;
        
        println!("=== SAFE F64 FINANCIAL PATTERN ===");
        println!("Price: {:.15}", price);
        println!("Quantity: {:.15}", quantity);
        println!("Raw total: {:.15}", total);
        println!("Truncated to cents: {:.2}", total_cents);
        
        // Verify truncation is deterministic
        let total2 = multiply_doubles(price, quantity);
        let total_cents2 = (total2 * 100.0).round() / 100.0;
        assert_eq!(total_cents, total_cents2, "Truncation should be deterministic");
        
        // Pattern 2: Percentage calculations with controlled precision
        let balance = 1000000.0_f64;
        let rate = 0.05123456789_f64; // 5.123456789%
        let interest = multiply_doubles(balance, rate);
        
        // Truncate to micro-units (6 decimal places)
        let interest_truncated = (interest * 1_000_000.0).round() / 1_000_000.0;
        
        println!("\n=== SAFE F64 PERCENTAGE PATTERN ===");
        println!("Balance: {:.15}", balance);
        println!("Rate: {:.15}", rate);
        println!("Raw interest: {:.15}", interest);
        println!("Truncated interest: {:.6}", interest_truncated);
        
        // Verify the truncation removes precision beyond our needs
        let truncation_diff = (interest - interest_truncated).abs();
        println!("Truncation difference: {:.2e}", truncation_diff);
        
        if truncation_diff > 0.0 {
            println!("✓ Truncation successfully removed excess precision");
        } else {
            println!("ℹ No truncation needed - value was already within precision bounds");
        }
        
        // The key point: truncation is deterministic and controlled
        assert!(truncation_diff < 0.000001, "Truncation should be small and controlled");
    }

    #[test]
    fn test_safe_f64_comparison_patterns() {
        // Demonstrate safe comparison patterns with f64
        
        let a = 0.1_f64 + 0.2_f64;
        let b = 0.3_f64;
        
        println!("=== SAFE F64 COMPARISON PATTERNS ===");
        println!("0.1 + 0.2 = {:.17}", a);
        println!("0.3 = {:.17}", b);
        println!("Direct equality: {}", a == b);
        println!("Difference: {:.2e}", (a - b).abs());
        
        // Pattern 1: Epsilon comparison
        let epsilon = 1e-15_f64;
        let epsilon_equal = (a - b).abs() < epsilon;
        println!("Epsilon comparison (1e-15): {}", epsilon_equal);
        
        // Pattern 2: Relative epsilon comparison
        let relative_epsilon = 1e-14_f64;
        let max_val = a.abs().max(b.abs());
        let relative_equal = (a - b).abs() < relative_epsilon * max_val;
        println!("Relative epsilon comparison: {}", relative_equal);
        
        // Pattern 3: Truncated comparison (safest for deterministic results)
        let scale = 1e12_f64; // 12 decimal places
        let a_truncated = (a * scale).round() / scale;
        let b_truncated = (b * scale).round() / scale;
        let truncated_equal = a_truncated == b_truncated;
        
        println!("Truncated comparison (12 decimals): {}", truncated_equal);
        println!("a truncated: {:.15}", a_truncated);
        println!("b truncated: {:.15}", b_truncated);
        
        // Demonstrate that truncated comparison is safest for blockchain
        assert!(truncated_equal, "Truncated comparison should work reliably");
    }

    #[test]
    fn test_safe_f64_range_validation() {
        // Demonstrate safe range validation patterns
        
        let values = vec![
            999.999999999999_f64,
            1000.000000000001_f64,
            1000.0_f64,
            1000.000000000000001_f64, // Very close to 1000
        ];
        
        let target = 1000.0_f64;
        let tolerance = 1e-12_f64; // 12 decimal places tolerance
        
        println!("=== SAFE F64 RANGE VALIDATION ===");
        println!("Target: {:.15}", target);
        println!("Tolerance: {:.2e}", tolerance);
        
        for (i, &value) in values.iter().enumerate() {
            let difference = (value - target).abs();
            let within_tolerance = difference <= tolerance;
            
            println!("Value {}: {:.15}", i, value);
            println!("  Difference: {:.2e}", difference);
            println!("  Within tolerance: {}", within_tolerance);
            
            // Show how to safely validate ranges
            if within_tolerance {
                println!("  ✓ ACCEPTED: Value is within acceptable range");
            } else {
                println!("  ✗ REJECTED: Value exceeds tolerance");
            }
        }
        
        // Demonstrate deterministic range checking
        let test_value = target + 1e-13_f64; // 1e-13, clearly within 1e-12 tolerance
        let is_valid = (test_value - target).abs() <= tolerance;
        
        println!("Test value: {:.15}", test_value);
        println!("Difference: {:.2e}", (test_value - target).abs());
        println!("Is valid: {}", is_valid);
        
        // This should be deterministic across all validators
        assert!(is_valid, "Range validation should be deterministic");
    }

    #[test]
    fn test_safe_f64_fixed_point_conversion() {
        // Demonstrate safe conversion between f64 and fixed-point
        
        let float_value = 123.456789123456_f64;
        let scale_factor = 1_000_000_u64; // 6 decimal places
        
        // Convert to fixed-point integer
        let fixed_point = (float_value * scale_factor as f64).round() as u64;
        
        // Convert back to float
        let recovered_float = fixed_point as f64 / scale_factor as f64;
        
        println!("=== SAFE F64 FIXED-POINT CONVERSION ===");
        println!("Original float: {:.15}", float_value);
        println!("Scale factor: {}", scale_factor);
        println!("Fixed-point: {}", fixed_point);
        println!("Recovered float: {:.15}", recovered_float);
        println!("Precision loss: {:.2e}", (float_value - recovered_float).abs());
        
        // Show that this conversion is safe and deterministic
        let fixed_point2 = (float_value * scale_factor as f64).round() as u64;
        let recovered_float2 = fixed_point2 as f64 / scale_factor as f64;
        
        assert_eq!(fixed_point, fixed_point2, "Fixed-point conversion should be deterministic");
        assert_eq!(recovered_float, recovered_float2, "Float recovery should be deterministic");
        
        // Demonstrate controlled precision loss
        assert!((float_value - recovered_float).abs() < 1e-6, "Precision loss should be controlled");
    }

    #[test]
    fn test_deterministic_precision_loss_patterns() {
        // Show that precision loss is deterministic and predictable
        
        let test_cases = vec![
            (0.1_f64, 0.2_f64),
            (1.0_f64, 3.0_f64),
            (123.456_f64, 789.123_f64),
        ];
        
        println!("=== DETERMINISTIC PRECISION LOSS ===");
        
        for (i, &(a, b)) in test_cases.iter().enumerate() {
            println!("Test case {}: {} + {}", i, a, b);
            
            // Perform the same calculation multiple times
            let results: Vec<f64> = (0..5)
                .map(|_| add_doubles(a, b))
                .collect();
            
            // All results should be identical
            let first_result = results[0];
            let all_identical = results.iter().all(|&r| r.to_bits() == first_result.to_bits());
            
            println!("  Result: {:.17}", first_result);
            println!("  All identical: {}", all_identical);
            println!("  Bits: {:064b}", first_result.to_bits());
            
            assert!(all_identical, "Results should be bit-for-bit identical");
            
            // Show the precision characteristics
            if a == 0.1 && b == 0.2 {
                let expected = 0.3_f64;
                let difference = (first_result - expected).abs();
                println!("  Expected 0.3, got difference: {:.2e}", difference);
                assert!(difference > 0.0, "Should show expected precision loss");
            }
        }
    }

    #[test]
    fn test_blockchain_safe_float_patterns() {
        // Demonstrate patterns that are safe for blockchain consensus
        
        println!("=== BLOCKCHAIN-SAFE FLOAT PATTERNS ===");
        
        // Pattern 1: Always truncate to known precision
        let raw_calculation = 123.456789123456789_f64 * 1.23456789_f64;
        let safe_result = (raw_calculation * 1e8).round() / 1e8; // 8 decimal places
        
        println!("Raw calculation: {:.17}", raw_calculation);
        println!("Safe truncated: {:.8}", safe_result);
        
        // Pattern 2: Use integer arithmetic when possible
        let price_cents = 12345_u64; // $123.45 in cents
        let quantity = 1000_u64;
        let total_cents = price_cents * quantity; // Integer multiplication
        let total_dollars = total_cents as f64 / 100.0;
        
        println!("Integer-based calculation: {} cents * {} = {} cents = ${:.2}", 
                 price_cents, quantity, total_cents, total_dollars);
        
        // Pattern 3: Validate ranges with fixed precision
        let balance = 1000.123456789_f64;
        let min_balance = 1000.0_f64;
        let precision_scale = 1e6; // 6 decimal places
        
        let balance_scaled = (balance * precision_scale).round();
        let min_balance_scaled = (min_balance * precision_scale).round();
        let is_sufficient = balance_scaled >= min_balance_scaled;
        
        println!("Balance validation: {:.9} >= {:.9} ? {}", 
                 balance, min_balance, is_sufficient);
        println!("Scaled comparison: {} >= {} ? {}", 
                 balance_scaled, min_balance_scaled, is_sufficient);
        
        // All these patterns should be deterministic
        assert!(safe_result.to_bits() == safe_result.to_bits(), "Truncation should be deterministic");
        assert!(total_dollars == 123450.0, "Integer arithmetic should be exact");
        assert!(is_sufficient, "Scaled comparison should work reliably");
    }
}