# DoorWasm

**Replacement Door Planner & Calculator powered by WebAssembly**

An interactive productivity tool for planning door replacement projects. Accurately calculate materials, costs, and installation time for multiple openings, with core optimization logic running in WebAssembly (Rust).

## Features
- Multi-door project builder (different sizes and types)
- Precise material takeoff (slab/pre-hung, jambs, casing, hardware)
- Cost estimator with quality tiers
- Realistic time estimates using productivity rates
- "Bulk Optimizer" powered by WASM (minimizes waste/cost across standard sizes)
- Visual opening preview
- Prework checklist and productivity tips
- Save/load projects

## Why WebAssembly?
The heavy calculation and optimization logic is implemented in Rust and compiled to WASM. This demonstrates using native-speed modules for performance-critical parts of a web productivity tool (exact floating point, optimization algorithms) while keeping the UI in clean JavaScript ES modules.

## Tech Stack
- Static site (no build for the demo)
- Tailwind CSS (CDN)
- ES Modules for JS architecture
- WebAssembly (Rust) for core engine
- Fully client-side

## Live Site
https://bennettwellsnet.github.io/doorwasm/

## Source
The Rust code that produces the WASM is in the `rust/` directory. You can rebuild it with `wasm-pack build --target web`.

---

This is the fourth demo in the series (after spacexdemo, patio, and paintcalc), continuing the focus on practical home improvement tools with modern web architecture.
