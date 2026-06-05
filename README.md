# ternary-needledrop

**Initial condition sensitivity.** Where you start changes everything. Perturbation analysis, Lyapunov exponents, and butterfly effects in ternary systems.

## Why This Exists

In `ternary-tenforward`, the first thing each agent does is predict what others will say. The quality of that prediction depends on the initial state. We found that tiny changes in starting conditions — flipping one agent from +1 to -1 — could lead to dramatically different conversation trajectories.

This is chaos theory applied to ternary populations. The needle drops on a record, and where it lands determines the entire song. In our system, the "needle" is the initial state vector, and the "song" is the population dynamics that unfold.

This crate provides the tools to measure exactly how sensitive a ternary system is to its starting point: perturbation divergence tracking, Lyapunov exponent estimation, butterfly effect scoring, and equilibrium point detection.

## The Physics Behind It

### Sensitivity to Initial Conditions

The canonical example: two 9-element ternary grids evolving under majority rule. Start one with a single position flipped, and track how many positions diverge over time. In chaotic systems, the divergence grows exponentially. In stable systems, it saturates or shrinks.

For ternary populations specifically, the Z₃ group structure means there are exactly 3⁹ = 19,683 possible initial states for a 9-element grid. Each one traces a unique trajectory through state space. The question is: how different are those trajectories?

### Lyapunov Exponents

The Lyapunov exponent λ measures the average exponential rate of divergence between nearby trajectories:

```
λ = (1/dt) × ⟨ln(|δ(t+dt)| / |δ(t)|)⟩
```

- λ > 0: chaotic — small perturbations grow exponentially
- λ = 0: neutral — perturbations neither grow nor shrink
- λ < 0: stable — perturbations decay, system returns to attractor

In the RPS experiments, the population cycling with period ~50 had positive Lyapunov exponents: knowing the current state gives you about 50 ticks of predictive value before chaos takes over.

### The Butterfly Score

`butterfly_score` flips every position one at a time and finds which single flip causes the maximum divergence. The position that matters most is the butterfly — perturb it, and everything changes. This maps to real agent dynamics: certain agents are linchpins. Remove or flip them, and the entire conversation restructures.

### The 8-Ball Test

`find_equilibrium_points` searches for initial conditions that are fixed points — states that don't change under the evolution rule. These are the configurations where the system is perfectly balanced. Like an 8-ball sitting still on a table, they're rare and fragile.

Under majority rule on a 3×3 grid with periodic boundaries, uniform states (all +1, all -1, all 0) are trivial fixed points. The interesting ones are the non-trivial equilibria where competing influences perfectly cancel.

### Entry Point Sensitivity

Different starting states lead to different average population stances over time. `entry_point_sensitivity` runs multiple trajectories and tracks the population mean at each tick. If all trajectories converge to the same mean, the system is insensitive to initial conditions. If they diverge, the entry point matters.

This directly connects to the ten-forward finding that initial speaker states affect the first ~20 ticks of conversation before the self-balancing dynamics take over.

## Key Types and Functions

```rust
/// Perturbation: flip one agent and track divergence.
pub fn perturbation_divergence(
    original: &[i8],
    perturb_idx: usize,
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<f64>

/// Estimate Lyapunov exponent from perturbation growth.
pub fn lyapunov_exponent(divergences: &[f64], dt: f64) -> f64

/// Entry point sensitivity — compare trajectories from different starting points.
pub fn entry_point_sensitivity(
    size: usize,
    entries: &[Vec<i8>],
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<Vec<f64>>

/// Butterfly effect score — maximum divergence from any single-bit flip.
pub fn butterfly_score(
    state: &[i8],
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> (usize, f64)

/// Majority rule for testing (cellular automaton on 2D grid).
pub fn majority_rule(state: &[i8]) -> Vec<i8>

/// The 8-ball test — find initial conditions that remain stable.
pub fn find_equilibrium_points(
    size: usize,
    candidates: usize,
    ticks: usize,
    rule: fn(&[i8]) -> Vec<i8>,
) -> Vec<Vec<i8>>
```

## Usage

### Basic Perturbation Analysis

```rust
use ternary_needledrop::{perturbation_divergence, lyapunov_exponent, majority_rule};

let state = vec![1, 0, -1, 1, 0, -1, 1, 0, -1];  // 3×3 grid

// Flip position 0 and track divergence for 50 ticks
let divs = perturbation_divergence(&state, 0, 50, majority_rule);
// divs[i] = fraction of positions that differ at tick i

// Is the system chaotic?
let lambda = lyapunov_exponent(&divs, 1.0);
if lambda > 0.0 {
    println!("Chaotic: λ = {:.3}", lambda);
}
```

### Finding the Butterfly

```rust
use ternary_needledrop::butterfly_score;

let state = vec![1, 0, -1, 1, 0, -1, 1, 0, -1];
let (butterfly_idx, max_divergence) = butterfly_score(&state, 30, majority_rule);
println!("Position {} is the butterfly: max divergence = {:.2}",
         butterfly_idx, max_divergence);
```

### Entry Point Comparison

```rust
use ternary_needledrop::entry_point_sensitivity;

let configs = vec![
    vec![1, 1, 1, 0, 0, 0, -1, -1, -1],  // structured
    vec![-1, -1, -1, 0, 0, 0, 1, 1, 1],  // inverted
    vec![1, -1, 0, 1, -1, 0, 1, -1, 0],  // repeating pattern
];

let trajectories = entry_point_sensitivity(9, &configs, 100, majority_rule);
// trajectories[config_idx][tick] = average population stance
```

### Searching for Fixed Points

```rust
use ternary_needledrop::find_equilibrium_points;

// Test 1000 random initial states for stability
let fixed_points = find_equilibrium_points(9, 1000, 5, majority_rule);
println!("Found {} stable configurations", fixed_points.len());
for fp in &fixed_points {
    println!("  {:?}", fp);
}
```

## The Rules Engine

The `rule` parameter is a function `fn(&[i8]) -> Vec<i8>` that defines how the system evolves. The crate ships with `majority_rule` (2D cellular automaton with Moore neighborhood and periodic boundaries), but you can plug in any evolution rule:

- Ternary RPS dynamics from `ternary-tenforward`
- Custom cellular automata
- Stochastic update rules (though the function signature requires determinism)

## In the Ternary Fleet

This is the **sensitivity analysis** layer:

- `ternary-tenforward` — runs the dynamics that needledrop analyzes
- **ternary-needledrop** — tells you how sensitive those dynamics are to initial conditions
- `ternary-predict` — prediction accuracy is bounded by the Lyapunov exponent
- `ternary-speculate` — speculative execution depends on trajectory divergence rates

The key insight: if the Lyapunov exponent is positive (chaotic), prediction accuracy degrades exponentially with look-ahead distance. This is why the ten-forward prediction system has a practical horizon of ~20 ticks.

## References

- Lyapunov exponents: standard measure of chaos in dynamical systems
- Fibonacci period 8: the Pisano period for mod 3 creates natural timescales
- Butterfly effect: Edward Lorenz's observation that small perturbations can have large consequences
- RPS period ~50: the practical prediction horizon before chaos dominates

## License

MIT
