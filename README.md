# Floating Point Accuracy and Determinism in Solana

This project demonstrates how Solana achieves deterministic floating point operations while highlighting the precision limitations that still exist.

> [!WARNING]
> Things change all the time. Do your own testing and manual review of source code when assessing software security.
> This is a research project created via vibe coding. Draw conclusions at your own peril.

## Reading the Repo

The point here is to run the tests and observe floating point operations, specifically with respect to their
accuracy and edge-cases at the boundaries.
All the important code is in `tests/`, not `src/`.

## Quick Start

```bash
# Runs the tests using the SBF target and prints the results
make test-verbose
```

### Why SBF Testing Matters

**Critical**: Always test with the SBF target (`cargo test-sbf` or `make test`) rather than native tests (`cargo test`). Only SBF tests accurately reflect the software float emulation that occurs in the Solana runtime.

- `cargo test` → Native hardware FPU (may behave differently)
- `cargo test-sbf` → Software float emulation (actual Solana behavior)

## Background: The Floating Point Problem

Floating point operations can be non-deterministic across different hardware platforms due to:

- **Different rounding modes** between CPU architectures
- **Varying precision** in intermediate calculations
- **Platform-specific handling** of edge cases (NaN, infinity, denormals)

For blockchains, this creates a critical problem: validators running on different hardware might get different results for the same calculation, leading to consensus failures and chain forks.

Inside the SVM however, these risks are mitigated.

### Further Reading on Float Issues
- [What Every Programmer Should Know About Floating-Point Arithmetic](https://floating-point-gui.de/)
- [IEEE 754 Standard](https://en.wikipedia.org/wiki/IEEE_754)
- [Catastrophic Cancellation](https://en.wikipedia.org/wiki/Catastrophic_cancellation)


## Precision Behavior in Solana

While Solana solves the **determinism problem**, precision characteristics still exist but are **more predictable** than expected:

### Solana's Consistent Precision
Solana's software emulation provides **deterministic precision behavior**:

```rust
// f64 catastrophic cancellation - deterministic results
let a = 1.0000000000000002_f64;
let b = 1.0000000000000000_f64;
let result = a - b;  // Gets consistent results across all validators
// May be perfect precision or predictable precision loss
```

### Accumulation Errors (Deterministic)
```rust
// Adding 0.1 one thousand times
let mut sum = 0.0_f32;
for _ in 0..1000 {
    sum += 0.1;
}
// Expected: 100.0, Actual: 99.999046 (0.1% error)
// Key: Identical error across all validators ✓
```

### The Classic 0.1 + 0.2 Problem
```rust
let result = 0.1_f64 + 0.2_f64;
// Expected: 0.3, Actual: 0.30000000000000004
// BUT: Always the same wrong answer on all validators ✓
```

### Complex Operations Like `powf()`
```rust
// Advanced approach: operations like powf() have precision variations
let base = 1.05_f64;
let result = base.powf(365.25);  // ~10^-5 variation possible

// Solution: Truncate to stable precision
let stable = (result * 1e12).round() / 1e12;  // 10^-12 precision
```

## Safe Float Usage Patterns

### 1. Precision Truncation Strategy ⭐ **RECOMMENDED**
```rust
// For complex operations like powf(), truncate to stable precision
let base = 1.05_f64;
let result = base.powf(365.25);
let stable_result = (result * 1e12).round() / 1e12;  // 10^-12 precision

// This eliminates ~10^-16 variations while maintaining financial accuracy
```

### 2. Financial Precision Control
```rust
// Financial calculations with known precision requirements
let raw_total = price * quantity;
let safe_total = (raw_total * 100.0).round() / 100.0;  // Truncate to cents

// For compound interest
let amount = principal * (1.0 + rate).powf(periods);
let final_amount = (amount * 100.0).round() / 100.0;  // Cent precision
```

### 3. Integer Arithmetic When Possible
```rust
// Use integer cents instead of float dollars
let price_cents = 12345_u64;  // $123.45
let quantity = 1000_u64;
let total_cents = price_cents * quantity;  // Exact integer math
```

### 4. Epsilon Comparisons
```rust
// Never use direct equality
let tolerance = 1e-12_f64;  // Recommended tolerance for financial calculations
let is_equal = (a - b).abs() <= tolerance;
```

### 5. Fixed-Point Conversion
```rust
// Convert to fixed-point for exact arithmetic
let scale = 1_000_000_u64;  // 6 decimal places
let fixed_point = (float_value * scale as f64).round() as u64;
```

### 6. Deterministic Range Validation
```rust
// Safe range checking with controlled precision
let target = 1000.0_f64;
let tolerance = 1e-12_f64;
let is_valid = (value - target).abs() <= tolerance;
```

### 7. Use strict upper bounds
Constrain your program to use a limited subset of the values represented by float types so that you avoid boundary cases.

## Test Categories

### Precision Stability Tests (`tests/precision_stability_tests.rs`)
Demonstrates advanced precision management techniques:
- **`powf()` precision variations** and 10^-12 truncation stability
- **Compound interest calculations** with cent-level precision
- **Exponential decay** for time-based DeFi operations
- **Deterministic `powf()` behavior** across multiple calls
- **Financial precision boundaries** for large amounts with small rates

### Precision Edge Cases (`tests/precision_edge_cases.rs`)
- **Catastrophic cancellation** examples showing Solana's precision handling
- **Arithmetic precision loss** accumulation
- **Safe f64 truncation** patterns
- **Blockchain-safe** float operations

### Financial Precision (`tests/financial_precision_tests.rs`)
- DeFi calculations (liquidity pools, slippage)
- Compound interest stability demonstrating actual Solana behavior
- Percentage calculations
- Cross-instruction precision

### f64 vs f32 Comparison (`tests/f64_precision_tests.rs`)
- Precision limits comparison
- Large balance handling
- Accumulation error differences
- Deterministic behavior validation

## How Solana Solves Determinism

Solana achieves deterministic floating point execution through **complete software emulation**:

### 1. No Native Float Instructions
The SBPF (Solana Bytecode Format) virtual machine contains **no native floating-point opcodes**. All float operations are handled through software libraries.

See: https://github.com/anza-xyz/sbpf/blob/main/src/ebpf.rs

### 2. Software-Only Compilation Target
Programs compile with `target_os = "solana"` which triggers software float emulation instead of hardware FPU operations:

```rust
// Automatically uses software emulation
let result = 1.5_f32 + 2.3_f32;  // No hardware FPU involved
```

### 3. Consistent Build Environment
- LLVM compiler with standardized float libraries
- Deterministic linking with libc float emulation functions
- Version-controlled SBPF instruction set (V0-V4)

## Test Results Summary

Running the comprehensive test suite reveals Solana's floating point behavior:

```bash
# All tests pass - demonstrating robust precision handling
make test

# Key findings from precision_stability_tests.rs:
✓ powf() operations: Deterministic across all calls
✓ Compound interest: Stable when rounded to cents  
✓ Exponential decay: Manageable with 10^-12 truncation
✓ Financial boundaries: Predictable precision within tolerance
✓ Truncation strategy: Effectively stabilizes complex calculations

# Precision edge cases show Solana's advantages:
✓ Catastrophic cancellation: Often handled perfectly (0.0 difference)
✓ Range validation: Deterministic within specified tolerances
✓ Blockchain patterns: All safety patterns work as expected
```

## Key Findings and Conclusions

### Solana's Emulation Addresses Floating Point Issues

Our comprehensive testing reveals that **Solana's software emulation provides deterministic precision**.

1. **Consistent Catastrophic Cancellation Handling**: Operations produce the same results across all validators
2. **Deterministic `powf()` Behavior**: Complex operations like `powf()` produce identical results across all calls
3. **Predictable Precision Patterns**: When precision loss occurs, it follows deterministic patterns

### Precision Management Validation

The precision stability tests validate advanced precision management techniques:
- **10^-12 truncation strategy** effectively stabilizes complex financial calculations
- **`powf()` operations** can be safely used with appropriate precision management
- **Financial calculations** remain accurate within required bounds when properly truncated

### Practical Implications for Developers

Solana's approach demonstrates that **deterministic floating point is achievable with consistent precision**:

- **Safe**: Float operations won't cause chain forks
- **Predictable**: Precision behavior is consistent across all validators
- **Manageable**: Developers can use proven precision control techniques (like the truncation strategy)
- **Reliable**: Complex operations like compound interest and exponential calculations work reliably
- **Manageable**: Developers can use proven precision control techniques (like the truncation strategy)

## Quick Reference for Developers

### ✅ Safe to Use in Solana Programs
- **Basic arithmetic**: `+`, `-`, `*`, `/` (with proper rounding)
- **Power operations**: `powf()`, `sqrt()` (with precision truncation strategy)
- **Trigonometric functions**: `sin()`, `cos()`, etc. (deterministic)
- **Compound interest**: `principal * (1 + rate).powf(periods)` (round to lowest unit)
- **Percentage calculations**: With appropriate precision control

### 🛡️ Recommended Patterns

Work within safe tolerance ranges:

```rust
// Precision truncation for complex operations
let result = base.powf(exponent);
let stable = (result * 1e12).round() / 1e12;

// Safe comparisons
let is_equal = (a - b).abs() <= 1e-12;
```

### ⚠️ Still Avoid
- **Direct equality comparisons**: Use epsilon tolerance instead
- **Uncontrolled accumulation**: Sum many small values without bounds checking
- **Extreme precision requirements**: Beyond ~12-15 decimal places

### 🧪 Testing Your Code
Always test with SBF target to see actual Solana behavior:
```bash
cargo test-sbf  # ✅ Shows real Solana precision
cargo test      # ❌ Uses hardware FPU (different results)
```
