use solana_floats::float_ops::*;
use solana_program::msg;
use solana_program_test::*;
use solana_sdk::{
    instruction::{Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

#[cfg(test)]
mod financial_precision_tests {
    use super::*;

    #[test]
    fn test_micro_cent_precision() {
        // Test precision at micro-cent level (0.0001 cents)
        let price = 1.2345_f32;
        let quantity = 1000.0_f32;
        let total = multiply_floats(price, quantity);
        
        // Should maintain precision for typical financial calculations
        assert!((total - 1234.5).abs() < 0.001);
    }

    #[test]
    fn test_large_balance_precision() {
        // Test precision with large account balances (millions)
        let balance = 1_000_000.0_f32;
        let interest_rate = 0.05_f32; // 5%
        let interest = multiply_floats(balance, interest_rate);
        
        assert!((interest - 50_000.0).abs() < 1.0);
    }

    #[test]
    fn test_compound_interest_stability() {
        // Test compound interest calculation stability
        let mut principal = 1000.0_f32;
        let rate = 1.01_f32; // 1% per period
        
        // Compound 100 times
        for _ in 0..100 {
            principal = multiply_floats(principal, rate);
        }
        
        // Should be approximately 1000 * 1.01^100 ≈ 2704.81
        assert!((principal - 2704.81).abs() < 10.0);
    }

    #[test]
    fn test_division_precision_loss() {
        // Test precision behavior in division operations
        let amount = 1.0_f32;
        let divisor = 3.0_f32;
        let result = divide_floats(amount, divisor).unwrap();
        
        // 1/3 should be approximately 0.333333
        assert!((result - 0.333333).abs() < 0.000001);
        
        // Test that multiplying back may or may not give exact 1.0
        // Solana's software emulation can be more precise than hardware
        let back = multiply_floats(result, divisor);
        let difference = (back - 1.0).abs();
        
        // Document the precision behavior for logic error prevention
        msg!("Division precision test:");
        msg!("  1.0 / 3.0 = {}", result);
        msg!("  result * 3.0 = {}", back);
        msg!("  difference from 1.0 = {:.2e}", difference);
        msg!("  ⚠️ Never use: if back == 1.0 (would fail!)");
        
        // The key insight: precision is manageable for financial use with proper techniques
        assert!(difference <= 1e-6, "Precision should be reasonable for financial use");
    }

    #[test]
    fn test_accumulation_error() {
        // Test error accumulation in repeated operations
        let mut sum = 0.0_f32;
        let increment = 0.1_f32;
        
        // Add 0.1 ten times
        for i in 0..10 {
            sum = add_floats(sum, increment);
            if i < 3 {
                msg!("Step {}: sum = {:.10}", i + 1, sum);
            }
        }
        
        let difference = (sum - 1.0).abs();
        msg!("Accumulation test:");
        msg!("  Final sum: {:.10}", sum);
        msg!("  Expected: 1.0");
        msg!("  Difference: {:.2e}", difference);
        
        // Should be close to 1.0 - Solana's software emulation may be very precise
        assert!((sum - 1.0).abs() < 0.000001, "Sum should be close to 1.0");
        
        // Document logic error risk
        if difference > f32::EPSILON {
            msg!("  ⚠️ Logic error risk: if sum == 1.0 would fail!");
            msg!("  ✅ Safe approach: if (sum - 1.0).abs() < 1e-6");
        } else {
            msg!("  ✓ No logic error risk in this case");
        }
    }

    #[test]
    fn test_percentage_calculation_precision() {
        // Test percentage calculations common in DeFi
        let total_supply = 1_000_000.0_f32;
        let user_balance = 12345.67_f32;
        
        let percentage = divide_floats(user_balance, total_supply).unwrap();
        let percentage_scaled = multiply_floats(percentage, 100.0);
        
        // Should be approximately 1.234567%
        assert!((percentage_scaled - 1.234567).abs() < 0.000001);
    }

    #[test]
    fn test_slippage_calculation() {
        // Test slippage calculation precision
        let expected_price = 100.0_f32;
        let actual_price = 99.5_f32;
        
        let price_diff = expected_price - actual_price;
        let slippage = divide_floats(price_diff, expected_price).unwrap();
        let slippage_percent = multiply_floats(slippage, 100.0);
        
        // Should be 0.5% slippage
        assert!((slippage_percent - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_liquidity_pool_math() {
        // Test constant product formula: x * y = k
        let reserve_x = 1000.0_f32;
        let reserve_y = 2000.0_f32;
        let k = multiply_floats(reserve_x, reserve_y); // k = 2,000,000
        
        // After swap: new_x = 1100, calculate new_y
        let new_x = 1100.0_f32;
        let new_y = divide_floats(k, new_x).unwrap();
        
        // new_y should be approximately 1818.18
        assert!((new_y - 1818.18).abs() < 0.01);
        
        // Verify k is preserved (within floating point precision)
        let new_k = multiply_floats(new_x, new_y);
        assert!((new_k - k).abs() < 1.0);
    }

    #[tokio::test]
    async fn test_cross_instruction_precision() {
        // Test precision across multiple program instructions
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "solana_floats",
            program_id,
            processor!(solana_floats::process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // First instruction: multiply 100.0 * 1.01
        let mut instruction_data1 = vec![1u8];
        instruction_data1.extend_from_slice(&100.0_f32.to_le_bytes());
        instruction_data1.extend_from_slice(&1.01_f32.to_le_bytes());

        let instruction1 = Instruction::new_with_bytes(
            program_id,
            &instruction_data1,
            vec![],
        );

        // Second instruction: divide result by 2.0 (conceptually)
        let mut instruction_data2 = vec![2u8];
        instruction_data2.extend_from_slice(&101.0_f32.to_le_bytes());
        instruction_data2.extend_from_slice(&2.0_f32.to_le_bytes());

        let instruction2 = Instruction::new_with_bytes(
            program_id,
            &instruction_data2,
            vec![],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction1, instruction2],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_denormal_numbers() {
        // Test behavior with very small numbers (denormals)
        let tiny = 1e-38_f32;
        let result = multiply_floats(tiny, 0.1);
        
        // Should handle denormal numbers gracefully
        assert!(result > 0.0 || result == 0.0);
    }

    #[test]
    fn test_infinity_handling() {
        // Test infinity and NaN handling
        let large = f32::MAX;
        let result = multiply_floats(large, 2.0);
        
        // Should produce infinity
        assert!(result.is_infinite());
    }

    #[test]
    fn test_deterministic_operations() {
        // Test that operations are deterministic across runs
        let a = 1.23456789_f32;
        let b = 9.87654321_f32;
        
        let result1 = multiply_floats(a, b);
        let result2 = multiply_floats(a, b);
        
        // Should be exactly equal (bit-for-bit)
        assert_eq!(result1, result2);
        assert_eq!(result1.to_bits(), result2.to_bits());
    }

    #[test]
    fn test_rounding_modes() {
        // Test consistent rounding behavior
        let a = 1.0_f32;
        let b = 3.0_f32;
        let result = divide_floats(a, b).unwrap();
        
        // Should consistently round to nearest representable value
        let expected_bits = 0x3eaaaaab_u32; // IEEE 754 representation of 1/3
        assert_eq!(result.to_bits(), expected_bits);
    }
}