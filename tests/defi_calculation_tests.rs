use solana_program::msg;

#[cfg(test)]
mod defi_calculation_tests {
    use super::*;

    #[test]
    fn test_amm_price_impact_calculation() {
        // Test Automated Market Maker price impact calculation
        // This demonstrates a real DeFi scenario where precision matters
        
        let reserve_x = 1_000_000.0_f64;  // 1M tokens
        let reserve_y = 2_000_000.0_f64;  // 2M tokens
        let trade_amount = 10_000.0_f64;  // 10K token trade
        
        // Constant product formula: x * y = k
        let k = reserve_x * reserve_y;
        
        // Calculate new reserves after trade
        let new_reserve_x = reserve_x + trade_amount;
        let new_reserve_y = k / new_reserve_x;
        
        // Price impact calculation
        let old_price = reserve_y / reserve_x;  // 2.0
        let new_price = new_reserve_y / new_reserve_x;
        let price_impact = (old_price - new_price) / old_price;
        
        msg!("=== AMM PRICE IMPACT TEST ===");
        msg!("Old price: {:.12}", old_price);
        msg!("New price: {:.12}", new_price);
        msg!("Price impact: {:.8}%", price_impact * 100.0);
        
        // ⚠️ LOGIC ERROR RISK: Never use strict equality for price comparisons
        // ❌ if new_price == expected_price { /* logic error! */ }
        
        // ✅ SAFE: Use epsilon comparison for price validation
        let expected_price = 1.9605920988138417_f64;  // Correctly calculated value
        let price_tolerance = 1e-12_f64;
        
        assert!((new_price - expected_price).abs() < price_tolerance,
                "Price calculation should be within tolerance");
        
        // Demonstrate precision truncation for UI display
        let display_price = (new_price * 1e8).round() / 1e8;  // 8 decimal places
        msg!("Display price (8 decimals): {:.8}", display_price);
    }

    #[test]
    fn test_compound_yield_farming_rewards() {
        // Test compound yield farming calculation over multiple periods
        // Shows how small precision errors can compound in DeFi protocols
        
        let initial_stake = 10_000.0_f64;
        let daily_apr = 0.0001_f64;  // 0.01% daily (3.65% annual)
        let days = 365_u32;
        
        // Method 1: Compound daily using powf()
        let final_amount_powf = initial_stake * (1.0 + daily_apr).powf(days as f64);
        
        // Method 2: Iterative compounding (more realistic for smart contracts)
        let mut amount_iterative = initial_stake;
        for day in 1..=days {
            amount_iterative *= 1.0 + daily_apr;
            
            // Log first few days to show accumulation
            if day <= 3 {
                msg!("Day {}: {:.12}", day, amount_iterative);
            }
        }
        
        msg!("=== COMPOUND YIELD FARMING TEST ===");
        msg!("Initial stake: ${:.2}", initial_stake);
        msg!("Daily APR: {:.4}%", daily_apr * 100.0);
        msg!("Final amount (powf): ${:.8}", final_amount_powf);
        msg!("Final amount (iterative): ${:.8}", amount_iterative);
        
        let difference = (final_amount_powf - amount_iterative).abs();
        msg!("Calculation difference: ${:.8}", difference);
        
        // ⚠️ CRITICAL: Different calculation methods can yield different results
        // This is why DeFi protocols must standardize their calculation approach
        
        // For financial applications, truncate to reasonable precision
        let final_powf_cents = (final_amount_powf * 100.0).round() / 100.0;
        let final_iter_cents = (amount_iterative * 100.0).round() / 100.0;
        
        msg!("Final amounts rounded to cents:");
        msg!("  powf method: ${:.2}", final_powf_cents);
        msg!("  iterative method: ${:.2}", final_iter_cents);
        
        // At cent precision, methods should agree
        assert_eq!(final_powf_cents, final_iter_cents,
                  "Methods should agree when rounded to cents");
    }

    #[test]
    fn test_liquidation_threshold_precision() {
        // Test liquidation threshold calculation - critical for lending protocols
        // Small precision errors here can cause premature or missed liquidations
        
        let collateral_value = 15_000.0_f64;  // $15,000 collateral
        let debt_value = 10_000.0_f64;        // $10,000 debt
        let liquidation_ratio = 1.5_f64;      // 150% collateralization required
        
        // Calculate current collateralization ratio
        let current_ratio = collateral_value / debt_value;  // Should be 1.5
        
        // Check if position should be liquidated
        let should_liquidate = current_ratio < liquidation_ratio;
        
        msg!("=== LIQUIDATION THRESHOLD TEST ===");
        msg!("Collateral value: ${:.2}", collateral_value);
        msg!("Debt value: ${:.2}", debt_value);
        msg!("Current ratio: {:.12}", current_ratio);
        msg!("Liquidation ratio: {:.12}", liquidation_ratio);
        msg!("Should liquidate: {}", should_liquidate);
        
        // ⚠️ CRITICAL LOGIC ERROR RISK: 
        // if current_ratio == liquidation_ratio { /* edge case handling */ }
        // This exact equality check could fail due to precision!
        
        // ✅ SAFE: Use epsilon comparison for threshold checks
        let threshold_tolerance = 1e-12_f64;
        let is_at_threshold = (current_ratio - liquidation_ratio).abs() < threshold_tolerance;
        let is_below_threshold = current_ratio < (liquidation_ratio - threshold_tolerance);
        
        msg!("Safe threshold checks:");
        msg!("  At threshold (within 1e-12): {}", is_at_threshold);
        msg!("  Below threshold (safe margin): {}", is_below_threshold);
        
        // Test with slightly different values that could cause precision issues
        let collateral_with_precision_loss = 15_000.000000000001_f64;
        let ratio_with_precision = collateral_with_precision_loss / debt_value;
        
        msg!("With precision variation:");
        msg!("  Modified ratio: {:.15}", ratio_with_precision);
        msg!("  Difference: {:.2e}", (ratio_with_precision - current_ratio).abs());
        
        // This tiny difference could cause different liquidation decisions!
        assert!((ratio_with_precision - current_ratio).abs() < 1e-12,
                "Precision variations should be within acceptable tolerance");
    }

    #[test]
    fn test_slippage_protection_calculation() {
        // Test slippage protection in DEX trades
        // Shows how precision affects trade execution and user protection
        
        let _input_amount = 1000.0_f64;
        let expected_output = 1950.0_f64;  // Expected ~1950 tokens out
        let max_slippage = 0.005_f64;      // 0.5% max slippage
        
        // Calculate minimum acceptable output
        let min_output = expected_output * (1.0 - max_slippage);
        
        // Simulate actual trade output with precision variations
        let actual_outputs = [
            1940.25_f64,           // Within slippage
            1940.249999999999_f64, // Precision boundary case
            1940.24_f64,           // Just outside slippage
        ];
        
        msg!("=== SLIPPAGE PROTECTION TEST ===");
        msg!("Expected output: {:.6}", expected_output);
        msg!("Max slippage: {:.2}%", max_slippage * 100.0);
        msg!("Minimum output: {:.12}", min_output);
        
        for (i, &actual) in actual_outputs.iter().enumerate() {
            let slippage = (expected_output - actual) / expected_output;
            let is_acceptable_strict = actual >= min_output;
            
            // ⚠️ PRECISION ISSUE: Strict comparison might fail at boundaries
            let tolerance = 1e-10_f64;  // Small tolerance for boundary cases
            let is_acceptable_safe = actual >= (min_output - tolerance);
            
            msg!("Test case {}: actual = {:.12}", i + 1, actual);
            msg!("  Slippage: {:.6}%", slippage * 100.0);
            msg!("  Acceptable (strict): {}", is_acceptable_strict);
            msg!("  Acceptable (safe): {}", is_acceptable_safe);
            
            if !is_acceptable_strict && is_acceptable_safe {
                msg!("  ⚠️ Precision boundary case detected!");
            }
        }
        
        // Demonstrate safe slippage checking
        let safe_min_output = min_output - 1e-10_f64;  // Small buffer for precision
        assert!(actual_outputs[1] >= safe_min_output,
                "Boundary case should pass with safe comparison");
    }

    #[test]
    fn test_oracle_price_aggregation() {
        // Test price oracle aggregation - critical for DeFi price feeds
        // Shows how precision affects price consensus and outlier detection
        
        let oracle_prices = [
            2150.123456789_f64,    // Oracle 1
            2150.123456790_f64,    // Oracle 2 (tiny difference)
            2150.123456788_f64,    // Oracle 3 (tiny difference)
            2150.123457000_f64,    // Oracle 4 (slightly larger difference)
            2149.999999999_f64,    // Oracle 5 (potential outlier)
        ];
        
        // Calculate median price (common oracle aggregation method)
        let mut sorted_prices = oracle_prices.clone();
        sorted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_price = sorted_prices[sorted_prices.len() / 2];
        
        // Calculate average price
        let avg_price: f64 = oracle_prices.iter().sum::<f64>() / oracle_prices.len() as f64;
        
        msg!("=== ORACLE PRICE AGGREGATION TEST ===");
        msg!("Individual oracle prices:");
        for (i, &price) in oracle_prices.iter().enumerate() {
            msg!("  Oracle {}: ${:.12}", i + 1, price);
        }
        msg!("Median price: ${:.12}", median_price);
        msg!("Average price: ${:.12}", avg_price);
        
        // Outlier detection using standard deviation
        let variance: f64 = oracle_prices.iter()
            .map(|&price| (price - avg_price).powi(2))
            .sum::<f64>() / oracle_prices.len() as f64;
        let std_dev = variance.sqrt();
        
        msg!("Standard deviation: ${:.12}", std_dev);
        
        // Check for outliers (prices > 2 standard deviations from mean)
        let outlier_threshold = 2.0 * std_dev;
        for (i, &price) in oracle_prices.iter().enumerate() {
            let deviation = (price - avg_price).abs();
            let is_outlier = deviation > outlier_threshold;
            
            if is_outlier {
                msg!("  ⚠️ Oracle {} is potential outlier (deviation: ${:.12})", 
                     i + 1, deviation);
            }
        }
        
        // ⚠️ PRECISION CRITICAL: Price differences of 1e-12 could affect:
        // - Arbitrage opportunities
        // - Liquidation triggers  
        // - Trading decisions
        
        // For price feeds, truncate to appropriate precision (e.g., 8 decimals)
        let display_median = (median_price * 1e8).round() / 1e8;
        msg!("Display median (8 decimals): ${:.8}", display_median);
        
        // Ensure price aggregation is stable within reasonable bounds
        assert!((median_price - avg_price).abs() < 1.0,
                "Median and average should be close for good price data");
    }
}