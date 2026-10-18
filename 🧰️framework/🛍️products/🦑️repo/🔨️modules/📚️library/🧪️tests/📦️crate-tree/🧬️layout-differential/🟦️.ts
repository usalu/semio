import { describe, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { applyCarveWrites, carveWrites, planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { hoistInnerAttributes, mountedPathFiles, verifyAppliedLayout } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { tiny, weaveFiles } from "../🏗️virtual-crate/🟦️.ts";

describe("artifact crate carve", () => {
  test("gates an applied carve: the mounted crates may not add an unresolved name to the pre-carve tree of HEAD", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-sbx-"));
    const artifact = "a/b/c";
    try {
      for (const [path, content] of Object.entries(weaveFiles(false))) { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), content); }
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const plan = await planArtifactCrates(repo, `${artifact}/📦️packages/🦀️rust`, { ...tiny });
      expect(plan.problems).toEqual([]);
      const { writes } = await carveWrites(plan);
      applyCarveWrites(repo, writes);
      const clean = await verifyAppliedLayout(repo, artifact, join(sandbox, "clean"));
      expect(clean.crates).toBeGreaterThan(1);
      expect(clean.baseCrates).toBe(0);
      expect(Object.values(clean.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      const broken = join(repo, artifact, "🚪️io/📝️text/🦀️.rs");
      writeFileSync(broken, readFileSync(broken, "utf8").replace("crate::schema::value()", "crate::schema::missing()"));
      const regressed = await verifyAppliedLayout(repo, artifact, join(sandbox, "regressed"));
      expect(Object.values(regressed.layout).some((row) => row.onlyCombined > 0)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("lays out the hand-built crates of the pre-carve tree the same way, so only a name the working tree adds is reported", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-base-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-base-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      const manifest = (name: string): string => `[package]\nname = "${name}"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n`;
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust", "🧩️extra/📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", manifest("demo-x"));
      write("🦀️.rs", "//! 🧪️ facade\npub const A: u8 = 1;\n");
      write("🧩️extra/📦️packages/🦀️rust/Cargo.toml", manifest("demo-x-extra"));
      write("🧩️extra/🦀️.rs", "//! 🧩️ hand-built crate\nuse crate::MacroMade;\npub fn a() -> u8 { demo_x::A }\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const clean = await verifyAppliedLayout(repo, artifact, join(sandbox, "clean"));
      expect(clean.crates).toBe(1);
      expect(clean.baseCrates).toBe(1);
      expect(Object.values(clean.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(clean.layout).every((row) => row.base === row.combined && row.base > 0)).toBe(true);
      write("🧩️extra/🦀️.rs", "//! 🧩️ hand-built crate\nuse crate::MacroMade;\nuse crate::Added;\npub fn a() -> u8 { demo_x::A }\n");
      const regressed = await verifyAppliedLayout(repo, artifact, join(sandbox, "regressed"), "HEAD", false, 5);
      expect(regressed.layout.native!.onlyCombined).toBe(1);
      expect(regressed.layout.native!.sampleOnlyCombined[0]).toContain("Added");
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("copies the working-tree file that a healed pre-carve file mounts when the pre-carve tree lacks it, so the base still parses", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-target-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-target-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\n#[path = \"🪟️views/🦀️.rs\"]\npub mod views;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\npub mod tools {\n    #[path = \"../🛠️tools/🦀️.rs\"]\n    pub mod simulation;\n}\n");
      write("🪟️views/🛠️tools/🦀️.rs", "//! 🛠️ tools\npub const B: u8 = 2;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\n#[path = \"🛠️tools\"]\npub mod tools {\n    #[path = \"🦀️.rs\"]\n    pub mod simulation;\n}\n#[path = \"🆕️added/🦀️.rs\"]\npub mod added;\n");
      write("🪟️views/🆕️added/🦀️.rs", "//! 🆕️ added\npub const C: u8 = 3;\n");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "target"), "HEAD", false, 5);
      expect(report.healedBase.map((path) => path.split("/").slice(-2).join("/")).sort()).toEqual(["🆕️added/🦀️.rs", "🪟️views/🦀️.rs"]);
      expect(Object.values(report.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(report.layout).every((row) => row.base === row.combined)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("leaves a binary-only package of the artifact out of the layout differential", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-bin-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-bin-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust", "🐚️shell/📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\npub const A: u8 = 1;\n");
      write("🐚️shell/📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x-shell"\nversion = "0.1.0"\n\n[[bin]]\nname = "demo-x-shell"\npath = "📦️bin.rs"\n');
      write("🐚️shell/📦️packages/🦀️rust/📦️bin.rs", "fn main() { println!(\"{}\", demo_x::A); }\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "bin"));
      expect(report.crates).toBe(0);
      expect(report.baseCrates).toBe(0);
      expect(Object.values(report.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("lists the module files a root mounts with flat path attributes, relative to the root's own directory", () => {
    const text = ['#[path = "🏊️arena-laws/🦀️.rs"]', "mod arena_laws;", '#[path = "../shared/🦀️.rs"] mod shared;', '#[path = "🧪️tests"] mod tests { #[path = "x/🦀️.rs"] mod x; }', ""].join("\n");
    expect(mountedPathFiles("a/b/🦀️.rs", text)).toEqual(["a/b/🏊️arena-laws/🦀️.rs", "a/shared/🦀️.rs", "a/b/x/🦀️.rs"]);
    expect(mountedPathFiles("a/b/🦀️.rs", "pub mod plain;\n")).toEqual([]);
  });

  test("hoists the inner attributes and inner docs that sit behind an item to the head of the file and leaves a clean file alone", () => {
    const broken = ["use std::fmt::Debug;", "#![allow(dead_code)]", "//! 🧪️ facade", "#[cfg(test)] use std::fmt::Display;", "#![allow(unused_imports)]", "pub mod kit {", "    //! inner doc of an inline module", "}", ""].join("\n");
    expect(hoistInnerAttributes(broken)).toBe(["#![allow(dead_code)]", "//! 🧪️ facade", "#![allow(unused_imports)]", "use std::fmt::Debug;", "#[cfg(test)] use std::fmt::Display;", "pub mod kit {", "    //! inner doc of an inline module", "}", ""].join("\n"));
    const clean = ["//! 🧪️ facade", "#![allow(dead_code)]", "use std::fmt::Debug;", "pub const A: u8 = 1;", ""].join("\n");
    expect(hoistInnerAttributes(clean)).toBe(clean);
  });

  test("repairs a pre-carve root with a misplaced inner attribute from its own text, so a carve that rewrote the root is not compared against its own carved root", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-hoist-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-hoist-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "use std::fmt::Debug;\n#![allow(dead_code)]\n//! 🧪️ facade\npub mod kit {\n    pub fn a() {}\n}\nuse crate::kit::missing;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      write("🦀️.rs", "//! 🧪️ facade\n#![allow(dead_code)]\nuse std::fmt::Debug;\npub mod kit {\n    pub fn a() {}\n}\nuse crate::kit::missing;\nuse crate::kit::added;\n");
      const report = await verifyAppliedLayout(repo, artifact, join(sandbox, "hoist"), "HEAD", false, 5);
      expect(report.healedBase.map((path) => path.split("/").slice(-1)[0])).toEqual(["🦀️.rs"]);
      expect(report.layout.native!.onlyCombined).toBe(1);
      expect(report.layout.native!.sampleOnlyCombined[0]).toContain("added");
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("heals a pre-carve file that rustc cannot parse with its working-tree text, so a repaired tree is still compared name for name", async () => {
    const repo = mkdtempSync(join(tmpdir(), "semio-applied-heal-"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-applied-heal-sbx-"));
    const artifact = "a/b/c";
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(repo, artifact, path)), { recursive: true }); writeFileSync(join(repo, artifact, path), text); };
      write("Cargo.toml", '[workspace]\nmembers = ["📦️packages/🦀️rust"]\n');
      write("📦️packages/🦀️rust/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n\n[lib]\npath = "../../🦀️.rs"\n');
      write("🦀️.rs", "//! 🧪️ facade\n#[path = \"🧰️kit/🦀️.rs\"]\npub mod kit;\n#[path = \"🪟️views/🦀️.rs\"]\npub mod views;\nuse crate::Imported;\n");
      write("🧰️kit/🦀️.rs", "//! 🧰️ kit\nuse std::fmt::Debug;\n#![allow(dead_code)]\npub const A: u8 = 1;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\npub mod tools {\n    #[path = \"../🛠️tools/🦀️.rs\"]\n    pub mod simulation;\n}\n");
      write("🪟️views/🛠️tools/🦀️.rs", "//! 🛠️ tools\npub const B: u8 = 2;\n");
      const git = (...args: string[]): void => { execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", ...args], { cwd: repo, stdio: "pipe" }); };
      git("init", "-q");
      git("add", "-A");
      git("commit", "-q", "-m", "baseline");
      const broken = await verifyAppliedLayout(repo, artifact, join(sandbox, "broken"));
      expect(broken.healedBase).toEqual([]);
      expect(Object.values(broken.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      write("🧰️kit/🦀️.rs", "//! 🧰️ kit\n#![allow(dead_code)]\nuse std::fmt::Debug;\npub const A: u8 = 1;\n");
      write("🪟️views/🦀️.rs", "//! 🪟️ views\n#[path = \"🛠️tools\"]\npub mod tools {\n    #[path = \"🦀️.rs\"]\n    pub mod simulation;\n}\n");
      const repaired = await verifyAppliedLayout(repo, artifact, join(sandbox, "repaired"));
      expect(repaired.healedBase.map((path) => path.split("/").slice(-2).join("/")).sort()).toEqual(["🧰️kit/🦀️.rs", "🪟️views/🦀️.rs"]);
      expect(Object.values(repaired.layout).map((row) => row.onlyCombined)).toEqual([0, 0]);
      expect(Object.values(repaired.layout).every((row) => row.base === row.combined)).toBe(true);
    } finally {
      rmSync(repo, { recursive: true, force: true });
      rmSync(sandbox, { recursive: true, force: true });
    }
  });
});
