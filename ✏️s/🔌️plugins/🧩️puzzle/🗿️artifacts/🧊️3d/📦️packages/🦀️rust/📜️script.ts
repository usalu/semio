#!/usr/bin/env bun
/** 📦️ puzzle-3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";

/**
 * 🧵️ Every app-driving test in this crate polls one `VcsArtifactApp` dispatch/render future, whose
 * unoptimized state machine plus the retained tool-job poll chain under it outgrows libtest's default
 * 2 MiB per-test thread stack and aborts the binary with `fatal runtime error: stack overflow` before
 * any suite summary is reachable. Raised here — the one permanent script this package owns — so `nx
 * test`, `bun ./📜️script.ts test` and the launch.json entries that call them all inherit it on every
 * platform without a per-developer environment step. A value already present in the environment wins.
 */
process.env.RUST_MIN_STACK ??= String(128 * 1024 * 1024);

/**
 * 🎛️ The whole `editor` tree is `#[cfg(feature = "component-app-assembly")]` (crate root `🦀️.rs:14-21`),
 * so a default-feature `cargo test` compiles 286 of this crate's 577 tests and runs not one of the
 * `editor::puzzle3d` app tests — which is how a harness that could not construct its own app survived
 * a whole wave (📓️2026-09-09-wave-X-test-suite.md §6). Declared here so `nx test`, `bun ./📜️script.ts
 * test` and the launch.json entries that call them all measure the same suite.
 */
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-3d", { testFeatures: ["component-app-assembly"] });
