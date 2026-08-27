#!/usr/bin/env bun
/** 🧩️ `@semio-tech/puzzle-plugin` router: `bun ./📜️script.ts test`. */
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";

//#region 🌉️BoardSessionPackage
class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root, skipEnvVar: "PUZZLE_BOARD_SKIP_WASM_BUILD", logPrefix: "puzzle/board", wasmBaseName: "semio_puzzle", shipProfile: "wasm-release", noDefaultFeatures: true,
      pkg: { name: "@semio-tech/puzzle-wasm", files: ["semio_puzzle_bg.wasm", "semio_puzzle.js", "semio_puzzle.d.ts", "semio_puzzle_bg.wasm.d.ts"], main: "semio_puzzle.js", module: "semio_puzzle.js", types: "semio_puzzle.d.ts" },
    });
  }
}
//#endregion 🌉️BoardSessionPackage

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-puzzle"], this.repoRoot);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️descriptor.semio` +
 * `🔣️descriptor.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-puzzle", join(this.root, "..", "..")));
  }
}

//#region 🔖️FixtureLint
/** 🧬️ One artifact mutation tree: the directory that directly contains the mutation leaves. */
type ArtifactTarget = { readonly label: string; readonly mutationsRoot: string };

/** 🔎️ Discovers every mutation tree in the repository — any directory named `🧬️mutations` holding
 * at least one leaf with a `🦠️mutation/🦀️component.rs`. Never a hand-maintained list: a new
 * artifact is in scope the moment it lands. */
function discoverArtifacts(repoRoot: string): ArtifactTarget[] {
  const found: ArtifactTarget[] = [];
  const skip = new Set(["node_modules", "target", ".git", ".nx", "storybook-static", "temp"]);
  const walk = (dir: string, depth: number): void => {
    if (depth > 14) return;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      return;
    }
    for (const entry of entries) {
      if (skip.has(entry)) continue;
      const child = join(dir, entry);
      let isDir = false;
      try {
        isDir = statSync(child).isDirectory();
      } catch {
        continue;
      }
      if (!isDir) continue;
      if (entry === "🧬️mutations") {
        const leaves = dirsIn(child).filter((leaf) => existsSync(join(child, leaf, "🦠️mutation/🦀️component.rs")));
        if (leaves.length > 0) found.push({ label: child.slice(repoRoot.length + 1), mutationsRoot: child });
        continue;
      }
      walk(child, depth + 1);
    }
  };
  walk(repoRoot, 0);
  return found.sort((left, right) => left.label.localeCompare(right.label));
}

/** 📛️ Directories under `🧬️mutations/` that are codec facets, not mutations. */
const NON_MUTATION_DIRS = new Set(["💾️binary", "📝️text"]);

/** 📄️ The source-of-truth files every test case must carry. These are hand-authored and are what
 * the committed Rust test asserts against. `🔺️diff` is replaced by `🔺️diff/🚫️component.absent`
 * when `🎯️outcome` declares `rejected` (contract D6). */
const CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"] as const;

/** 📄️ The derived encodings contract D1 targets. Produced from the core files by `fixtures
 * generate`, never hand-authored — a hand-forged binary would be a parallel implementation of the
 * codec it is supposed to test. Absent until the workspace compiles (contract D11). */
const DERIVED_CASE_FILES = [
  "🦠️mutation/🔧️component.op.semio",
  "🦠️mutation/📡️component.spr.semio",
  "🔺️diff/🩹️component.patch.semio",
  "🔺️diff/📡️component.patch.spr.semio",
] as const;

/** 📸️ A snapshot side is either the hand-authored JSON plus its derived encodings, or exactly one
 * typed reference to a canonical example (contract §2.2). Never both. */
const SNAPSHOT_CORE = "🔣️component.json";
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"] as const;
const SNAPSHOT_REF = "🔗️component.ref.json";

type Finding = { readonly level: "error" | "warn"; readonly where: string; readonly what: string };

const dirsIn = (path: string): string[] =>
  existsSync(path) ? readdirSync(path).filter((entry) => statSync(join(path, entry)).isDirectory()) : [];

/** 🧬️ Reads the declared mutation vocabulary from the schema itself — the enum variants and each
 * leaf's `#[dsl(keyword)]` — so coverage is checked against the schema, never against a hand list. */
function declaredMutations(target: ArtifactTarget) {
  const mutationsRoot = target.mutationsRoot;
  const aggregateFile = join(mutationsRoot, "🦀️component.rs");
  const aggregate = existsSync(aggregateFile) ? readFileSync(aggregateFile, "utf8") : "";
  // 🧺️ Some trees declare a SECOND mutation enum inside a leaf rather than in this aggregate
  // (`🛡️change-merge-policy` owns `MergePolicyConfigMutation`), so variants are collected from every
  // enum in the tree, not just the root one — otherwise a self-wrapped leaf reads as an orphan.
  const enumSources = [aggregate, ...dirsIn(mutationsRoot).flatMap((entry) => {
    // a leaf may declare its own wrapper enum either in its `🦠️mutation` component or in a file
    // sitting directly in the leaf directory — read both.
    const candidates = [join(mutationsRoot, entry, "🦠️mutation/🦀️component.rs"), join(mutationsRoot, entry, "🦀️component.rs")];
    return candidates.filter((file) => existsSync(file)).map((file) => readFileSync(file, "utf8"));
  })];
  const enumBody = enumSources.map((source) => source.match(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/)).find(Boolean) ?? null;
  const allVariantText = enumSources
    .flatMap((source) => [...source.matchAll(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/g)].map((match) => match[1]))
    .join("\n");
  // 🔑️ Variants are keyed by their PAYLOAD TYPE, not their own name: a leaf directory declares the
  // payload struct, and a variant may legitimately be named differently from it
  // (`Group(group::mutation::GroupNodes)`). Matching on the variant name instead reports false gaps.
  const variants = [...allVariantText.matchAll(/^\s+([A-Z][A-Za-z0-9]*)\(\s*([A-Za-z0-9_:]+)/gm)].map((match) => ({
    name: match[1],
    payload: match[2].split("::").pop() as string,
  }));
  void enumBody;

  const leaves = dirsIn(mutationsRoot)
    .filter((entry) => !NON_MUTATION_DIRS.has(entry))
    .filter((entry) => existsSync(join(mutationsRoot, entry, "🦠️mutation/🦀️component.rs")))
    .map((entry) => {
      const mutationFile = join(mutationsRoot, entry, "🦠️mutation/🦀️component.rs");
      const source = existsSync(mutationFile) ? readFileSync(mutationFile, "utf8") : "";
      return {
        dir: entry,
        keyword: source.match(/dsl\(keyword = "([^"]+)"\)/)?.[1] ?? null,
        struct: source.match(/^pub struct ([A-Za-z0-9]+)/m)?.[1] ?? null,
        path: join(mutationsRoot, entry),
      };
    });

  return { mutationsRoot, variants, leaves };
}

/** ✅️ Enforces contract D1/D6: schema variants == mutation leaves == test subjects, and every test
 * case carries a complete codec set. Returns findings; never throws on a malformed tree. */
function lintArtifact(target: ArtifactTarget, full: boolean): Finding[] {
  const findings: Finding[] = [];
  const { variants, leaves } = declaredMutations(target);

  const byStruct = new Map(leaves.filter((leaf) => leaf.struct).map((leaf) => [leaf.struct as string, leaf]));
  // 🌉️ A variant whose payload type is declared in NO leaf here is a delegation to another subset's
  // own mutation enum (`Brep(SemioBrepMutation)`) — that payload is covered in the tree that owns it,
  // so it is not a gap in this one. Only a leaf that no variant references is a real orphan.
  const referenced = new Set(variants.map((variant) => variant.payload));
  for (const leaf of leaves) {
    if (leaf.struct && variants.length > 0 && !referenced.has(leaf.struct)) {
      findings.push({ level: "error", where: `${target.label}/${leaf.dir}`, what: `payload ${leaf.struct} is declared but no enum variant wraps it` });
    }
  }
  for (const leaf of leaves) {
    
    const cases = dirsIn(join(leaf.path, "🧪️tests"));
    if (cases.length === 0) {
      findings.push({ level: "error", where: `${target.label}/${leaf.dir}`, what: "no 🧪️tests cases — every mutation must be directly tested" });
      continue;
    }
    for (const testCase of cases) findings.push(...lintCase(join(leaf.path, "🧪️tests", testCase), `${target.label}/${leaf.dir}/${testCase}`, full));
  }
  return findings;
}

/** 🧪️ Enforces one test case's file set. `full` also demands the derived encodings (contract D1);
 * without it only the hand-authored source-of-truth set is required and derived gaps are warnings. */
function lintCase(caseDir: string, label: string, full: boolean): Finding[] {
  const findings: Finding[] = [];
  const outcomeFile = join(caseDir, "🎯️outcome/🔣️component.json");
  let rejected = false;
  if (existsSync(outcomeFile)) {
    try {
      const outcome = JSON.parse(readFileSync(outcomeFile, "utf8"));
      rejected = outcome.status === "rejected";
      if (!["applied", "rejected"].includes(outcome.status)) findings.push({ level: "error", where: label, what: `🎯️outcome.status must be "applied" or "rejected", got ${JSON.stringify(outcome.status)}` });
      if (rejected && typeof outcome.code !== "string") findings.push({ level: "error", where: label, what: "rejected outcome must carry a machine-readable code" });
    } catch (error) {
      findings.push({ level: "error", where: label, what: `🎯️outcome is not valid JSON: ${(error as Error).message}` });
    }
  }

  for (const relative of CORE_CASE_FILES) {
    if (rejected && relative.startsWith("🔺️diff/")) continue;
    if (!existsSync(join(caseDir, relative))) findings.push({ level: "error", where: label, what: `missing ${relative}` });
  }
  for (const relative of DERIVED_CASE_FILES) {
    if (rejected && relative.startsWith("🔺️diff/")) continue;
    if (existsSync(join(caseDir, relative))) continue;
    findings.push({ level: full ? "error" : "warn", where: label, what: `missing derived ${relative} — run \`fixtures generate\`` });
  }
  if (rejected && !existsSync(join(caseDir, "🔺️diff/🚫️component.absent"))) {
    findings.push({ level: "error", where: label, what: "rejected case must carry 🔺️diff/🚫️component.absent, not an invented empty patch" });
  }

  for (const side of ["⬅️before", "➡️after"]) {
    const sideDir = join(caseDir, "📸️snapshot", side);
    if (!existsSync(sideDir)) {
      findings.push({ level: "error", where: label, what: `missing 📸️snapshot/${side}` });
      continue;
    }
    const hasRef = existsSync(join(sideDir, SNAPSHOT_REF));
    if (hasRef) {
      if (existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} has both a reference and inline encodings — exactly one is allowed` });
      findings.push(...lintReference(join(sideDir, SNAPSHOT_REF), `${label}/📸️snapshot/${side}`));
      continue;
    }
    if (!existsSync(join(sideDir, SNAPSHOT_CORE))) findings.push({ level: "error", where: label, what: `📸️snapshot/${side} is missing ${SNAPSHOT_CORE}` });
    for (const name of SNAPSHOT_DERIVED) {
      if (existsSync(join(sideDir, name))) continue;
      findings.push({ level: full ? "error" : "warn", where: label, what: `missing derived 📸️snapshot/${side}/${name} — run \`fixtures generate\`` });
    }
  }
  return findings;
}

/** 🔗️ A snapshot reference must resolve inside the artifact example tree and must not traverse. */
function lintReference(refFile: string, label: string): Finding[] {
  try {
    const ref = JSON.parse(readFileSync(refFile, "utf8"));
    for (const key of ["artifact", "standard", "subset", "example", "asset"]) {
      if (typeof ref[key] !== "string") return [{ level: "error", where: label, what: `reference is missing string field "${key}"` }];
    }
    if (Object.values(ref).some((value) => typeof value === "string" && (value.includes("..") || value.includes("/")))) {
      return [{ level: "error", where: label, what: "reference fields must not contain path traversal or separators" }];
    }
    return [];
  } catch (error) {
    return [{ level: "error", where: label, what: `reference is not valid JSON: ${(error as Error).message}` }];
  }
}

/** 🧹️ `fixtures lint` — the permanent coverage gate. Exits non-zero on any error finding. */
class FixturesScript extends BundleScript {
  run(segments: string[]): void {
    const sub = segments[0] ?? "lint";
    if (sub !== "lint") {
      console.error("usage: bun ./📜️script.ts fixtures lint [--full]");
      process.exit(1);
    }
    const full = segments.includes("--full");
    const targets = discoverArtifacts(this.repoRoot);
    const findings = targets.flatMap((target) => lintArtifact(target, full));
    const errors = findings.filter((finding) => finding.level === "error");
    const warnings = findings.filter((finding) => finding.level === "warn");

    let totalLeaves = 0;
    let totalCovered = 0;
    const uncoveredByTree: string[] = [];
    for (const target of targets) {
      const { leaves } = declaredMutations(target);
      const covered = leaves.filter((leaf) => dirsIn(join(leaf.path, "🧪️tests")).length > 0).length;
      totalLeaves += leaves.length;
      totalCovered += covered;
      if (covered < leaves.length) uncoveredByTree.push(`${leaves.length - covered}/${leaves.length}  ${target.label}`);
    }
    console.log(`🧬️ ${targets.length} artifact mutation trees · ${totalLeaves} mutations · ${totalCovered} covered · ${totalLeaves - totalCovered} uncovered`);
    if (segments.includes("--by-tree")) {
      for (const row of uncoveredByTree.sort((left, right) => Number(right.split("/")[0]) - Number(left.split("/")[0]))) console.log(`   ❌️ ${row}`);
    } else if (uncoveredByTree.length > 0) {
      console.log(`   ${uncoveredByTree.length} tree(s) with uncovered mutations — rerun with --by-tree to list them`);
    }
    for (const finding of errors.filter((f) => !f.what.startsWith("no 🧪️tests cases")).slice(0, 40)) console.log(`❌️ ${finding.where}: ${finding.what}`);
    if (warnings.length > 0) console.log(`⚠️ ${warnings.length} derived-encoding gap(s) pending \`fixtures generate\` (contract D1 target; run with --full to fail on them)`);
    console.log(errors.length === 0 ? "✅️ fixture contract satisfied" : `❌️ ${errors.length} error(s)`);
    process.exit(errors.length === 0 ? 0 : 1);
  }
}
//#endregion 🔖️FixtureLint

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", WasmScript)
  .register("test", TestScript)
  .register("describe", DescribeScript)
  .register("fixtures", FixturesScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
