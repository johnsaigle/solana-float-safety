use solana_floats::double_ops::*;
use solana_floats::float_ops::*;

#[cfg(test)]
mod f64_financial_edge_cases {
    use super::*;

    #[test]
    fn test_f64_cryptocurrency_precision() {
        // Test precision for cryptocurrency amounts
        // Bitcoin has 8 decimal places, Ethereum has 18
        
        // Test Bitcoin-like precision (8 decimals)
        let btc_amount = 21_000_000.12345678_f64; // Max BTC supply with satoshi precision
        let satoshi = 0.00000001_f64;
        
        let result = add_doubles(btc_amount, satoshi);
        let difference = result - btc_amount;
        
        println!("=== CRYPTOCURRENCY PRECISION ===");
        println!("BTC amount: {:.8}", btc_amount);
        println!("Adding 1 satoshi: {:.8}", satoshi);
        println!("Result: {:.8}", result);
        println!("Difference: {:.8}", difference);
        println!("Satoshi preserved: {}", difference == satoshi);
        
        assert_eq!(difference, satoshi, "f64 should preserve satoshi precision");
        
        // Test Ethereum-like precision (18 decimals)
        let eth_amount = 1.0_f64;
        let wei = 1e-18_f64; // 1 wei = 10^-18 ETH
        
        let eth_result = add_doubles(eth_amount, wei);
        let eth_difference = eth_result - eth_amount;
        
        println!("\nETH amount: {:.18}", eth_amount);
        println!("Adding 1 wei: {:.18e}", wei);
        println!("Result: {:.18}", eth_result);
        println!("Difference: {:.18e}", eth_difference);
        println!("Wei preserved: {}", eth_difference == wei);
        
        // f64 might lose precision at 18 decimal places
        if eth_difference != wei {
            println!("WARNING: f64 loses precision at 18 decimal places (wei level)");
        }
    }

    #[test]
    fn test_f64_defi_liquidity_pool() {
        // Test DeFi liquidity pool constant product formula: x * y = k
        let reserve_x = 1_000_000.123456789_f64;
        let reserve_y = 2_000_000.987654321_f64;
        let k = multiply_doubles(reserve_x, reserve_y);
        
        println!("=== DEFI LIQUIDITY POOL (f64) ===");
        println!("Initial reserves: X={:.9}, Y={:.9}", reserve_x, reserve_y);
        println!("Constant k: {:.9}", k);
        
        // Simulate a swap: add 1000 to X, calculate new Y
        let new_x = add_doubles(reserve_x, 1000.0);
        let new_y = divide_doubles(k, new_x).unwrap();
        
        println!("After swap: X={:.9}, Y={:.9}", new_x, new_y);
        
        // Verify k is preserved
        let new_k = multiply_doubles(new_x, new_y);
        let k_difference = (new_k - k).abs();
        
        println!("New k: {:.9}", new_k);
        println!("K difference: {:.9e}", k_difference);
        println!("K preserved: {}", k_difference < 1e-10);
        
        // f64 should maintain better precision than f32
        assert!(k_difference < 1e-6, "f64 should maintain reasonable precision in DeFi math");
    }

    #[test]
    fn test_f64_interest_rate_precision() {
        // Test very small interest rates
        let principal = 1_000_000_000.0_f64; // 1 billion
        let annual_rate = 0.000001_f64; // 0.0001% annual rate
        let daily_rate = divide_doubles(annual_rate, 365.0).unwrap();
        
        println!("=== INTEREST RATE PRECISION (f64) ===");
        println!("Principal: ${:.2}", principal);
        println!("Annual rate: {:.6}%", annual_rate * 100.0);
        println!("Daily rate: {:.10e}", daily_rate);
        
        let mut balance = principal;
        let mut total_interest = 0.0_f64;
        
        // Compound daily for 30 days
        for day in 1..=30 {
            let daily_interest = multiply_doubles(balance, daily_rate);
            balance = add_doubles(balance, daily_interest);
            total_interest = add_doubles(total_interest, daily_interest);
            
            if day % 10 == 0 {
                println!("Day {}: Balance=${:.6}, Interest=${:.6}", 
                         day, balance, daily_interest);
            }
        }
        
        println!("Final balance: ${:.6}", balance);
        println!("Total interest: ${:.6}", total_interest);
        
        // Should accumulate meaningful interest even with tiny rates
        assert!(total_interest > 0.0, "Should accumulate some interest");
        assert!(balance > principal, "Balance should grow");
    }

    #[test]
    fn test_f64_price_precision() {
        // Test price calculations with high precision
        let btc_price = 43_567.89123456_f64; // BTC price in USD
        let quantity = 0.00123456_f64; // Small BTC amount
        
        let total_value = multiply_doubles(btc_price, quantity);
        
        println!("=== PRICE PRECISION (f64) ===");
        println!("BTC price: ${:.8}", btc_price);
        println!("Quantity: {:.8} BTC", quantity);
        println!("Total value: ${:.8}", total_value);
        
        // Test fee calculation (0.1% trading fee)
        let fee_rate = 0.001_f64;
        let fee = multiply_doubles(total_value, fee_rate);
        let net_value = total_value - fee;
        
        println!("Fee (0.1%): ${:.8}", fee);
        println!("Net value: ${:.8}", net_value);
        
        // Verify precision is maintained
        let reconstructed_total = add_doubles(net_value, fee);
        let precision_error = (reconstructed_total - total_value).abs();
        
        println!("Precision error: ${:.12e}", precision_error);
        
        assert!(precision_error < 1e-10, "f64 should maintain high precision in price calculations");
    }

    #[test]
    fn test_f64_percentage_precision() {
        // Test percentage calculations with high precision
        let total_supply = 1_000_000_000.123456789_f64; // 1B+ tokens
        let user_balance = 0.000000001_f64; // Very small balance
        
        let percentage = divide_doubles(user_balance, total_supply).unwrap();
        let percentage_scaled = multiply_doubles(percentage, 100.0);
        
        println!("=== PERCENTAGE PRECISION (f64) ===");
        println!("Total supply: {:.9}", total_supply);
        println!("User balance: {:.9}", user_balance);
        println!("Percentage: {:.15e}%", percentage_scaled);
        
        // Test if we can detect this tiny percentage
        assert!(percentage > 0.0, "Should detect non-zero percentage");
        assert!(percentage_scaled > 0.0, "Scaled percentage should be positive");
        
        // Test reverse calculation
        let calculated_balance = multiply_doubles(total_supply, percentage);
        let balance_error = (calculated_balance - user_balance).abs();
        
        println!("Calculated balance: {:.15e}", calculated_balance);
        println!("Balance error: {:.15e}", balance_error);
        
        assert!(balance_error < 1e-15, "f64 should maintain precision in percentage calculations");
    }

    #[test]
    fn test_f64_slippage_calculation() {
        // Test slippage calculation with high precision
        let expected_price = 1.234567890123456_f64;
        let actual_price = 1.234567890123455_f64; // Tiny difference
        
        let price_diff = expected_price - actual_price;
        let slippage = divide_doubles(price_diff, expected_price).unwrap();
        let slippage_percent = multiply_doubles(slippage, 100.0);
        let slippage_bps = multiply_doubles(slippage, 10000.0); // basis points
        
        println!("=== SLIPPAGE CALCULATION (f64) ===");
        println!("Expected price: {:.15}", expected_price);
        println!("Actual price: {:.15}", actual_price);
        println!("Price difference: {:.15e}", price_diff);
        println!("Slippage: {:.15e}%", slippage_percent);
        println!("Slippage: {:.15e} bps", slippage_bps);
        
        // Should detect tiny slippage
        assert!(price_diff > 0.0, "Should detect price difference");
        assert!(slippage > 0.0, "Should calculate positive slippage");
        assert!(slippage_bps > 0.0, "Should show slippage in basis points");
    }

    #[test]
    fn test_f64_compound_rounding_errors() {
        // Test compound rounding errors over many operations
        let initial_balance = 1000.0_f64;
        let mut balance_f32 = initial_balance as f32;
        let mut balance_f64 = initial_balance;
        
        // Perform 10,000 small operations
        let operations = 10_000;
        let small_change = 0.0001_f64; // 0.01% change each time
        
        for i in 0..operations {
            let multiplier_f32 = (1.0 + small_change) as f32;
            let multiplier_f64 = 1.0 + small_change;
            
            balance_f32 = multiply_floats(balance_f32, multiplier_f32);
            balance_f64 = multiply_doubles(balance_f64, multiplier_f64);
            
            if i % 1000 == 0 {
                println!("Operation {}: f32={:.10}, f64={:.10}", i, balance_f32, balance_f64);
            }
        }
        
        println!("=== COMPOUND ROUNDING ERRORS ===");
        println!("Initial balance: {:.10}", initial_balance);
        println!("Final f32 balance: {:.10}", balance_f32);
        println!("Final f64 balance: {:.10}", balance_f64);
        
        let f32_error = ((balance_f32 as f64) - balance_f64).abs();
        let relative_error = f32_error / balance_f64;
        
        println!("Absolute error: {:.10e}", f32_error);
        println!("Relative error: {:.10e}", relative_error);
        
        // f64 should accumulate significantly less error
        assert!(relative_error > 1e-10, "Should show measurable difference between f32 and f64");
    }

    #[test]
    fn test_f64_oracle_price_aggregation() {
        // Test price aggregation from multiple oracles
        let oracle_prices = vec![
            1.234567890123456_f64,
            1.234567890123457_f64,
            1.234567890123455_f64,
            1.234567890123458_f64,
            1.234567890123454_f64,
        ];
        
        println!("=== ORACLE PRICE AGGREGATION (f64) ===");
        for (i, &price) in oracle_prices.iter().enumerate() {
            println!("Oracle {}: {:.15}", i + 1, price);
        }
        
        // Calculate average price
        let sum = oracle_prices.iter().fold(0.0_f64, |acc, &price| add_doubles(acc, price));
        let count = oracle_prices.len() as f64;
        let average = divide_doubles(sum, count).unwrap();
        
        println!("Sum: {:.15}", sum);
        println!("Count: {}", count);
        println!("Average: {:.15}", average);
        
        // Calculate variance
        let variance_sum = oracle_prices.iter()
            .map(|&price| {
                let diff = price - average;
                multiply_doubles(diff, diff)
            })
            .fold(0.0_f64, |acc, var| add_doubles(acc, var));
        
        let variance = divide_doubles(variance_sum, count).unwrap();
        let std_dev = variance.sqrt();
        
        println!("Variance: {:.15e}", variance);
        println!("Std deviation: {:.15e}", std_dev);
        
        // Should maintain precision in statistical calculations
        assert!(variance > 0.0, "Should detect price variance");
        assert!(std_dev > 0.0, "Should calculate standard deviation");
    }

    #[test]
    fn test_f64_vs_f32_fork_scenarios() {
        // Test scenarios that could cause forks between f32 and f64 systems
        
        println!("=== FORK SCENARIO TESTING ===");
        
        // Scenario 1: Balance comparison at precision limits
        let balance_base = 16_777_216.0; // 2^24 - f32 precision limit
        let tiny_amount = 0.5;
        
        let f32_result = add_floats(balance_base as f32, tiny_amount as f32);
        let f64_result = add_doubles(balance_base, tiny_amount);
        
        println!("Scenario 1 - Precision limit comparison:");
        println!("Base: {}", balance_base);
        println!("Adding: {}", tiny_amount);
        println!("f32 result: {}", f32_result);
        println!("f64 result: {}", f64_result);
        println!("Results equal: {}", f32_result as f64 == f64_result);
        
        // Scenario 2: Compound interest divergence
        let mut f32_balance = 1000.0_f32;
        let mut f64_balance = 1000.0_f64;
        let rate = 1.0000001; // Very small rate
        
        for _ in 0..10000 {
            f32_balance = multiply_floats(f32_balance, rate as f32);
            f64_balance = multiply_doubles(f64_balance, rate);
        }
        
        println!("\nScenario 2 - Compound interest divergence:");
        println!("f32 final: {:.10}", f32_balance);
        println!("f64 final: {:.10}", f64_balance);
        println!("Difference: {:.10e}", (f64_balance - f32_balance as f64).abs());
        
        // Document potential fork conditions
        let precision_fork_risk = f32_result as f64 != f64_result;
        let compound_fork_risk = (f64_balance - f32_balance as f64).abs() > 1e-6;
        
        println!("\nFork Risk Assessment:");
        println!("Precision fork risk: {}", precision_fork_risk);
        println!("Compound fork risk: {}", compound_fork_risk);
        
        if precision_fork_risk || compound_fork_risk {
            println!("WARNING: Mixed f32/f64 systems could fork!");
        }
    }
}