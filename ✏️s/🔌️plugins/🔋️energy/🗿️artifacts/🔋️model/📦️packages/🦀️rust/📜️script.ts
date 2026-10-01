#!/usr/bin/env bun
/** 📦️ energy model Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runWeatherChecks } from "../../../../🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-energy-model", { twins: [{ name: "epw-weather", run: runWeatherChecks }] });
