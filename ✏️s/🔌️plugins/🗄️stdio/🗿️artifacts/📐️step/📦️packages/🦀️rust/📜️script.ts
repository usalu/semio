#!/usr/bin/env bun
/** 📦️ step Rust artifact package router over every family package of the artifact workspace. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
const familyRoot = resolve(import.meta.dir, "../..");
const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-step", {packages:familyPackages,snapshotSqliteTests:["../../🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],snapshotSqliteTestBudgetMs:120000});
