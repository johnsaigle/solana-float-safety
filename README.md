# Floating Point Accuracy in Solana

This project demonstrates floating point precision behavior in Solana and provides practical guidance for avoiding logic errors.

> [!WARNING]
> This is a research project. Do your own testing and manual review when assessing software security.

## Quick Start

```bash
# Test with actual Solana runtime behavior
make test
```

**Critical**: Always use `cargo test-sbf` (not `cargo test`) to see actual Solana behavior.

## Main Takeaways

### Determinism vs. Accuracy: The Key Distinction
**Determinism is solved in Solana** - no consensus issues. However, **precision limitations can cause logic errors**. 

**Key insight**: **1e-12 precision is the practical limit for reliable float operations.** Going more precise (1e-15, 1e-16) enters the realm of floating point noise where small variations can cause logic errors. Going less precise (1e-9, 1e-6) may miss meaningful differences.

The biggest risk is **strict equality comparisons** - always use epsilon tolerance around 1e-12 to avoid program failures.

### ⚠️ Accuracy: STILL A CONCERN
**Floating point accuracy limitations remain and cause logic errors:**

```rust
// This is deterministic (always the same wrong answer)
let result = 0.1 + 0.2;  // Always 0.30000000000000004
// But this will cause a logic error:
if result == 0.3 { /* This never executes! */ }
```

**The rest of this document focuses on accuracy issues.**

## Common Accuracy Problems

### The 0.1 + 0.2 Problem
```rust
let result = 0.1_f64 + 0.2_f64;
// Expected: 0.3, Actual: 0.30000000000000004
// ❌ Never use: if result == 0.3
// ✅ Always use: if (result - 0.3).abs() < 1e-15
// Use 1e-15 for pure math, 1e-12 for financial comparisons
```

### Accumulation Errors
```rust
let mut sum = 0.0_f32;
for _ in 0..10 {
    sum += 0.1;
}
// Expected: 1.0, Actual: 1.0000001192
// ❌ Never use: if sum == 1.0
// ✅ Always use: if (sum - 1.0).abs() < 1e-6
// Use 1e-6 for f32, 1e-12 for f64 financial calculations
```

### Complex Operations
```rust
let result = base.powf(exponent);
// ⚠️ Small input changes can cause large output differences
// ✅ Truncate for financial use: (result * 1e12).round() / 1e12
// 12 decimal places: precise enough for finance, coarse enough to eliminate noise
```

## Safe Usage Patterns

### 1. Epsilon Comparisons ⚠️ **CRITICAL**
```rust
// ❌ NEVER do this - causes logic errors:
if balance == required { /* ... */ }
if result == expected { /* ... */ }

// ✅ Always use epsilon tolerance:
if (balance - required).abs() <= 1e-12 { /* ... */ }
// 1e-12 chosen as safe tolerance for financial calculations
// (much larger than f64 precision ~1e-15, smaller than financial significance)
```

### 2. Precision Truncation for Financial Calculations
```rust
// Precision truncation for complex operations
let result = base.powf(exponent);
let stable = (result * 1e12).round() / 1e12;  // 12 decimals: financial sweet spot

// Financial calculations
let amount = principal * (1.0 + rate).powf(periods);
let final = (amount * 100.0).round() / 100.0;  // Cent precision

// Safe comparisons
let is_equal = (a - b).abs() <= 1e-12;  // Safe for financial comparisons
```

### 3. Integer Arithmetic When Possible
```rust
// Use integer cents instead of float dollars
let price_cents = 12345_u64;  // $123.45
let total_cents = price_cents * quantity;  // Exact
```

### 4. Fixed-Point Conversion
```rust
let scale = 1_000_000_u64;  // 6 decimal places
let fixed_point = (float_value * scale as f64).round() as u64;
```

## Test Results Summary

Our comprehensive testing shows:

✅ **powf() operations**: Work reliably with proper truncation  
✅ **Compound interest**: Stable when rounded to cents  
✅ **Financial calculations**: Accurate with proper precision management  
✅ **Complex operations**: Manageable with truncation strategies  

## For Code Auditors

### 🚨 Red Flags to Look For
- **Direct equality comparisons**: `if (a == b)` or `if (balance >= required)`
- **Strict comparisons without epsilon**: Any `==`, `!=`, `>=`, `<=` with floats
- **Uncontrolled accumulation**: Loops adding small float values
- **Missing precision truncation**: Financial calculations without rounding

### ✅ Good Patterns to Verify
- **Epsilon comparisons**: `(a - b).abs() <= tolerance`
- **Appropriate truncation**: Rounding to cents, basis points, etc.
- **Integer arithmetic**: Using cents/wei instead of dollars/ether
- **Bounded precision**: Operations within known precision limits

## Quick Reference

### ✅ Safe in Solana
- Basic arithmetic with proper rounding
- `powf()`, `sqrt()` with truncation
- Financial calculations with epsilon comparisons

### ❌ Logic Error Risks
- **Any direct equality comparison with floats**
- **Strict comparisons without tolerance**
- **Assuming perfect precision**

### 🧪 Testing
```bash
cargo test-sbf  # ✅ Shows real Solana behavior
cargo test      # ❌ Uses hardware FPU (different results)
```

## Further Reading
- [Floating Point accuracy problems](https://en.wikipedia.org/wiki/Floating-point_arithmetic#Accuracy_problems)
- [Catastrophic Cancellation](https://en.wikipedia.org/wiki/Catastrophic_cancellation)
- [IEEE 754 Standard](https://en.wikipedia.org/wiki/IEEE_754)
- [What Every Programmer Should Know About Floating-Point Arithmetic](https://floating-point-gui.de/)
