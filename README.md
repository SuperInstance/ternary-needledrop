# ternary-needledrop

**The sound of vinyl that never existed. Surface noise, crackle, warmth, and the romance of imperfection.**

There's a reason producers still record to tape and press to vinyl when they could stay digital. It's not nostalgia — it's *physics*. A needle dragging through a groove creates sounds that digital processing can't replicate: the warm rumble of the turntable motor, the soft crackle of dust in the groove, the gentle wow and flutter of a platter that doesn't spin at exactly 33⅓ RPM.

These imperfections aren't bugs. They're features. The noise floor gives the music a bed to lie on. The crackle marks time between tracks. The warmth — that vague, unmeasurable "warmth" — is the sound of a physical system with resonances and inertia and mass.

This crate simulates all of that in ternary. The signal goes in clean. It comes out with character.

## What's Inside

- **`crackle(signal, density, seed)`** — add random pops and crackles. Density controls how often, seed makes it reproducible
- **`surface_noise(signal, level)`** — add a bed of low-level noise. The "room tone" of vinyl
- **`wow_and_flutter(signal, rate, depth)`** — pitch wobble from an imperfect motor. Rate = how fast it wobbles, depth = how much
- **`vinyl_process(signal, config)`** — the full chain: crackle + surface noise + wow + EQ curve. One function, full character
- **`VinylConfig`** — configurable: crackle density, noise level, wow rate, flutter depth, wear level
- **`wear(signal, plays)`** — simulate repeated playback. More plays = more high-frequency loss + more crackle. The record *degrades*

## Quick Example

```rust
use ternary_needledrop::*;

let clean = vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0];

// Add some crackle
let crackling = crackle(&clean, 0.1, 42);
// ~10% of samples get a random crackle pop

// Full vinyl treatment
let config = VinylConfig {
    crackle_density: 0.05,
    noise_level: 0.15,
    wow_rate: 0.3,
    wow_depth: 0.02,
    wear: 50, // 50 previous plays
};
let vinyl = vinyl_process(&clean, &config);

// Simulate aging: play it 100 more times
let worn = wear(&vinyl, 100);
// More noise, less detail. The record is dying beautifully.
```

## The Deeper Truth

**Imperfection is information.** A perfectly clean digital signal tells you exactly what was recorded and nothing else. A vinyl signal tells you what was recorded *plus* the temperature of the room where it was pressed, the dust in the air, how many times it's been played, and whether the turntable belt is getting old. The imperfections are a *timestamp* — evidence that something physical happened.

In ternary, this is particularly meaningful because ternary is already a low-information representation. Adding noise doesn't "obscure" the signal — it *enriches* it. A ternary signal with crackle has more information than a clean ternary signal, because each crackle pop is a data point about the (simulated) physical medium. The noise becomes signal.

The wear function is the most poignant: every play degrades the record slightly. High frequencies go first. Then the crackle increases. After enough plays, the music is barely recognizable beneath the noise — but the *rhythm* persists longer than anything else. The groove is the last thing to die.

**Use cases:**
- **Lo-fi production** — the authentic vinyl aesthetic, generated in code
- **Sound design** — age and degrade any ternary signal
- **Art installation** — generative music that physically degrades over time
- **Nostalgia engineering** — make digital sounds feel analog
- **Education** — hear what "warmth" and "character" actually mean in signal processing terms

## See Also

- **ternary-bite** — a different kind of degradation (digital, not analog)
- **ternary-echo** — echo + needledrop = dub vinyl
- **ternary-wave** — the clean signals you're aging
- **ternary-sampler** — sample from vinyl, then mangle
- **ternary-grain** — granular synthesis loves the texture of noise

## Install

```bash
cargo add ternary-needledrop
```

## License

MIT
