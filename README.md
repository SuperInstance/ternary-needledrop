# Ternary Needledrop

Initial-condition sensitivity analysis for ternary `{-1, 0, +1}` agent systems. The "needle drop" metaphor: **where you place the needle on the record changes the entire song**. A single perturbed agent can cascade into system-wide divergence — this crate measures exactly how much, how fast, and which starting positions are safe.

## Why It Matters

Ternary systems — cellular automata, GPU scheduling clusters, game-theoretic populations — exhibit extreme sensitivity to initial conditions. A one-bit flip at agent $i$ can, under chaotic dynamics, grow to infect the entire population within $O(\log n)$ steps. Conversely, some configurations are **equilibrium points** — no matter how long you run the system, they never change.

Understanding this dichotomy is essential for:

- **Fault tolerance**: How resilient is a fleet configuration to a single-agent crash?
- **Chaos detection**: Is the system governed by deterministic chaos (bounded but unpredictable) or true instability?
- **Bootstrapping**: Which initial states lead to stable equilibria vs. oscillations vs. chaos?

## How It Works

### Perturbation Divergence

Given an initial state $s \in \{-1, 0, +1\}^n$ and a perturbation at index $k$ (where $s_k \to -s_k$), we evolve both the original and perturbed states under a rule $f$ and measure the **Hamming divergence** at each step:

$$D(t) = \frac{1}{n} \sum_{i=0}^{n-1} \mathbf{1}\left[f^t(s)_i \neq f^t(s')_i\right]$$

$D(t) \in [0, 1]$: 0 means identical, 1 means fully diverged.

**Complexity:** $O(n \cdot T)$ for $T$ ticks of evolution (each tick applies $f$ which is $O(n)$ for majority rule).

### Lyapunov Exponent Estimation

The **maximal Lyapunov exponent** $\lambda$ quantifies the rate of divergence:

$$D(t) \approx D(0) \cdot e^{\lambda t}$$

Taking logs: $\lambda \approx \frac{1}{T \cdot \Delta t} \sum_{t=1}^{T-1} \ln\left(\frac{D(t)}{D(t-1)}\right)$

| $\lambda$ | Interpretation |
|---|---|
| $\lambda > 0$ | Chaotic — perturbations grow exponentially |
| $\lambda \approx 0$ | Marginal — perturbations stay constant |
| $\lambda < 0$ | Stable — perturbations decay |

### Entry-Point Sensitivity

For a set of candidate entry configurations $\{e_1, e_2, \ldots, e_m\}$, we evolve each for $T$ ticks and record the population mean $\mu_t = \frac{1}{n}\sum_i s_i^{(t)}$ at each step. Trajectories that diverge indicate high entry-point sensitivity.

### Butterfly Score

The **worst-case single-bit-flip divergence**:

$$B = \max_{k \in [0,n)} \left( \max_{t \in [0,T)} D_k(t) \right)$$

Returns both the critical index $k^*$ and the worst divergence $B$.

### Equilibrium Points

Uses a linear-congruential PRNG (constants from Knuth: $a = 6364136223846793005$, $c = 1$) to sample $K$ candidate initial states. Each is evolved for $T$ ticks; if the state returns to itself, it's a **fixed point** of $f$.

**Probability bound:** For majority rule on a $\sqrt{n} \times \sqrt{n}$ torus, the number of fixed points scales as $O(2^{n/2})$, so random sampling finds them with probability $\sim K / 3^n$.

## Quick Start

```rust
use ternary_needledrop::{
    perturbation_divergence, lyapunov_exponent,
    butterfly_score, find_equilibrium_points,
    majority_rule,
};

let state = vec![1, 0, -1, 1, 0, -1, 1, 0, -1];

// Perturb agent 0 and watch divergence for 10 ticks
let divs = perturbation_divergence(&state, 0, 10, majority_rule);
println!("Divergence trajectory: {:?}", divs);

// Estimate Lyapunov exponent
let lambda = lyapunov_exponent(&divs, 1.0);
println!("Lyapunov λ = {:.4}", lambda);

// Find the most fragile position
let (worst_idx, worst_div) = butterfly_score(&state, 10, majority_rule);
println!("Butterfly at index {} → max divergence {:.2}", worst_idx, worst_div);

// Search for equilibrium points
let stable = find_equilibrium_points(9, 200, 5, majority_rule);
println!("Found {} equilibrium configurations", stable.len());
```

## API

### Core Functions

| Function | Signature | Complexity |
|---|---|---|
| `perturbation_divergence` | `(state, idx, ticks, rule) → Vec<f64>` | $O(n \cdot T)$ |
| `lyapunov_exponent` | `(divergences, dt) → f64` | $O(T)$ |
| `entry_point_sensitivity` | `(size, entries, ticks, rule) → Vec<Vec<f64>>` | $O(m \cdot n \cdot T)$ |
| `butterfly_score` | `(state, ticks, rule) → (usize, f64)` | $O(n^2 \cdot T)$ |
| `find_equilibrium_points` | `(size, candidates, ticks, rule) → Vec<Vec<i8>>` | $O(K \cdot n \cdot T)$ |

### `majority_rule`

Built-in cellular automaton update rule on a $\sqrt{n} \times \sqrt{n}$ toroidal grid. Each cell takes the sign of the sum of its 8 neighbors (von Neumann + diagonal). Used as the default dynamics for testing.

## Architecture Notes

Ternary Needledrop connects to the **γ + η = C** framework as the **sensitivity analyzer**:

- **γ (gamma)** — the perturbation itself: flipping one agent's ternary state
- **η (eta)** — the system's nonlinear response: how the perturbation propagates through the neighbor graph
- **C** — **criticality**: when λ > 0, the system is at the edge of chaos — the most computationally rich regime (per Langton's edge-of-chaos hypothesis)

The crate is `#![forbid(unsafe_code)]` — pure safe Rust with zero dependencies.

## References

1. Lyapunov, A. M. (1892). *The General Problem of the Stability of Motion*. — Original definition of characteristic exponents.
2. Packard, N. H., Crutchfield, J. P., Farmer, J. D., & Shaw, R. S. (1980). "Geometry from a Time Series." *Physical Review Letters*, 45(9), 712. — Practical Lyapunov estimation from time series.
3. Langton, C. G. (1990). "Computation at the Edge of Chaos." *Physica D*, 42(1-3), 12-37. — Edge of chaos and λ ≈ 0.
4. Wolfram, S. (1984). "Universality and Complexity in Cellular Automata." *Physica D*, 10(1-2), 1-35. — Classification of CA dynamics.

## License

MIT
