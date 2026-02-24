# Contributing to RDTR

Thank you for your interest in RDTR.

RDTR is a specification-first project. Stability, determinism, and clarity are more important than speed.

---

## Project Philosophy

RDTR is:

- Deterministic
- Declarative
- Keyboard-first
- Secure by design
- Spec-driven

RDTR is NOT:

- A browser clone
- A styling system
- A flexible rendering playground
- An imperative drawing API

---

## Repository Structure

- /compliance-tests     → Determinism and correctness tests
- /docs                 → Documentation and diagrams
- /engine               → Reference engine implementation
- /examples             → Example RDTR apps
- /protocol             → Binary encoding specification
- /specs                → Formal specification
- /wasm-host            → Host API bindings

---

## Contribution Types

### 1. Specification Improvements

Before implementing a change:

- Propose spec modification via RFC issue
- Include deterministic impact analysis
- Provide backward compatibility notes

### 2. Engine Improvements

All engine changes MUST:

- Preserve determinism
- Pass compliance tests
- Not introduce hidden state

### 3. Protocol Extensions

Must:

- Maintain binary compactness
- Avoid optional ambiguity
- Be versioned

---

## Development Setup

(Example for Rust engine)

```bash
git clone https://github.com/RDTR-org/spec.git rdtr
cd rdtr/engine
cargo build
cargo test
```

## Determinism Rule

The final frame MUST depend only on:

- LayoutTree
- OverlayStack
- TerminalSize

Any deviation is a bug.

## Mutation Validation Rule

WASM may not:

- Access filesystem
- Open sockets directly
- Allocate arbitrary terminal regions
- Bypass engine API

All side effects are host-controlled.

## Pull Request Guidelines

- One feature per PR
- Include test
- Update relevant spec section
- Provide rationale

## Governance Model (Draft)

- Spec changes require discussion
- Engine remains reference implementation
- Alternative engines are encouraged

We value clarity over cleverness.
We value determinism over flexibility.
We value simplicity over completeness.
