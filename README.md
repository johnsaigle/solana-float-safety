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

**Critical**: Always test with the SBF target (`cargo test-sbf` or `make test`) rather than native tests (`cargo test`). Only SBF tests accurately reflect the software float emulation that occurs on Solana validators.

- `cargo test` → Native hardware FPU (may behave differently)
- `cargo test-sbf` → Software float emulation (actual Solana behavior)

## Background: The Floating Point Problem

Floating point operations can be non-deterministic across different hardware platforms due to:

- **Different rounding modes** between CPU architectures
- **Varying precision** in intermediate calculations
- **Platform-specific handling** of edge cases (NaN, infinity, denormals)

For blockchains, this creates a critical problem: validators running on different hardware might get different results for the same calculation, leading to consensus failures and chain forks.

### Further Reading on Float Issues
- [What Every Programmer Should Know About Floating-Point Arithmetic](https://floating-point-gui.de/)
- [IEEE 754 Standard](https://en.wikipedia.org/wiki/IEEE_754)
- [Catastrophic Cancellation](https://en.wikipedia.org/wiki/Catastrophic_cancellation)


## Precision Limitations Still Exist

While Solana solves the **determinism problem**, it doesn't eliminate the **precision problem**:

### Catastrophic Cancellation
```rust
let a = 1.0000001_f32;
let b = 1.0000000_f32;
let result = a - b;  // Expected: 0.0000001, Actual: 0.0000001192 (19% error)
```

### Accumulation Errors
```rust
// Adding 0.1 one thousand times
let mut sum = 0.0_f32;
for _ in 0..1000 {
    sum += 0.1;
}
// Expected: 100.0, Actual: 99.999046 (0.1% error)
```

### The Classic 0.1 + 0.2 Problem
```rust
let result = 0.1_f64 + 0.2_f64;
// Expected: 0.3, Actual: 0.30000000000000004
// BUT: Always the same wrong answer on all validators ✓
```

## Safe Float Usage Patterns

### 1. Controlled Precision Truncation
```rust
// Financial calculations with known precision requirements
let raw_total = price * quantity;
let safe_total = (raw_total * 100.0).round() / 100.0;  // Truncate to cents
```

### 2. Integer Arithmetic When Possible
```rust
// Use integer cents instead of float dollars
let price_cents = 12345_u64;  // $123.45
let quantity = 1000_u64;
let total_cents = price_cents * quantity;  // Exact integer math
```

### 3. Epsilon Comparisons
```rust
// Never use direct equality
let tolerance = 1e-12_f64;
let is_equal = (a - b).abs() < tolerance;
```

### 4. Fixed-Point Conversion
```rust
// Convert to fixed-point for exact arithmetic
let scale = 1_000_000_u64;  // 6 decimal places
let fixed_point = (float_value * scale as f64).round() as u64;
```

### 5. Use strict upper bounds
Constrain your program to use a limited subset of the values represented by float types so that you avoid boundary cases.

## Test Categories

### Precision Edge Cases (`tests/precision_edge_cases.rs`)
- **Catastrophic cancellation** examples
- **Arithmetic precision loss** accumulation
- **Safe f64 truncation** patterns
- **Blockchain-safe** float operations

### Financial Precision (`tests/financial_precision_tests.rs`)
- DeFi calculations (liquidity pools, slippage)
- Compound interest stability
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

## Conclusion

Solana's approach demonstrates that **deterministic floating point is achievable** through software emulation, but developers must still be aware of fundamental IEEE 754 precision limitations. The key is that precision loss is **predictable and identical** across all validators, preventing consensus issues while maintaining mathematical correctness within known bounds.

For blockchain applications, this means:
- **Safe**: Float operations won't cause chain forks
- **Predictable**: Precision loss follows consistent patterns  
- **Manageable**: Developers can use appropriate precision control techniques

The trade-off is performance (20-25x slower) for absolute determinism.

Consider this CU costs when choosing to use floats over ints in your programs.
