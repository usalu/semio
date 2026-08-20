#!/usr/bin/env bun
/** 🧩️ `@semio-tech/puzzle-plugin` router: `bun ./📜️script.ts test`. */
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";

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
/** 🧬️ Artifact whose mutation tree the fixture contract governs. `subset` is the `✳️any` leaf. */
type ArtifactTarget = { readonly artifact: string; readonly subset: string };

const ARTIFACTS: readonly ArtifactTarget[] = [
  { artifact: "puzzle5d", subset: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any" },
];

/** 📛️ Directories under `🧬️mutations/` that are codec facets, not mutations. */
const NON_MUTATION_DIRS = new Set(["💾️binary", "📝️text"]);

/** 📄️ The source-of-truth files every test case must carry. These are hand-authored and are what
 * the committed Rust test asserts against. `🔺️diff` is replaced by `🔺️diff/🚫️component.absent`
 * when `🎯️outcome` declares `rejected` (contract D6). */
const CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"] as const;

/** 📄️ The derived encodings contract D1 targets. Produced from the core files by `fixtures
 * generate`, never hand-authored — a hand-forged binary would be a parallel implementation of the
 * codec it is supposed to test. Absent until the workspace compiles (contract D11). */
const DERIVED_CASE_FILES = [
  "🦠️mutation/🔧️component.op.semio",
  "🦠️mutation/📡️component.spr.semio",
  "🔺️diff/🩹️component.patch.semio",
  "🔺️diff/📡️component.patch.spr.semio",
  "🔺️diff/🔣️component.json",
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
function declaredMutations(repoRoot: string, target: ArtifactTarget) {
  const mutationsRoot = join(repoRoot, target.subset, "🧬️schema/🧬️mutations");
  const aggregate = readFileSync(join(mutationsRoot, "🦀️component.rs"), "utf8");
  const enumBody = aggregate.match(/pub enum Puzzle5dMutation \{([\s\S]*?)\n\}/);
  const variants = enumBody ? [...enumBody[1].matchAll(/^\s{4}([A-Za-z0-9]+)\(/gm)].map((m) => m[1]) : [];

  const leaves = dirsIn(mutationsRoot)
    .filter((entry) => !NON_MUTATION_DIRS.has(entry))
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
function lintArtifact(repoRoot: string, target: ArtifactTarget, full: boolean): Finding[] {
  const findings: Finding[] = [];
  const { variants, leaves } = declaredMutations(repoRoot, target);

  const byStruct = new Map(leaves.filter((leaf) => leaf.struct).map((leaf) => [leaf.struct as string, leaf]));
  for (const variant of variants) {
    if (!byStruct.has(variant)) findings.push({ level: "error", where: `${target.artifact}:${variant}`, what: "enum variant has no mutation directory" });
  }
  for (const leaf of leaves) {
    if (!leaf.keyword) findings.push({ level: "error", where: `${target.artifact}/${leaf.dir}`, what: "no #[dsl(keyword = …)] found" });
    if (!leaf.struct) findings.push({ level: "error", where: `${target.artifact}/${leaf.dir}`, what: "no payload struct found" });
    if (leaf.struct && !variants.includes(leaf.struct)) findings.push({ level: "error", where: `${target.artifact}/${leaf.dir}`, what: `payload ${leaf.struct} is not a Puzzle5dMutation variant` });
    if (leaf.keyword && !leaf.dir.endsWith(leaf.keyword)) findings.push({ level: "error", where: `${target.artifact}/${leaf.dir}`, what: `directory does not end in its keyword "${leaf.keyword}"` });

    const cases = dirsIn(join(leaf.path, "🧪️tests"));
    if (cases.length === 0) {
      findings.push({ level: "error", where: `${target.artifact}/${leaf.dir}`, what: "no 🧪️tests cases — every mutation must be directly tested" });
      continue;
    }
    for (const testCase of cases) findings.push(...lintCase(join(leaf.path, "🧪️tests", testCase), `${target.artifact}/${leaf.dir}/${testCase}`, full));
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
    const findings = ARTIFACTS.flatMap((target) => lintArtifact(this.repoRoot, target, full));
    const errors = findings.filter((finding) => finding.level === "error");

    for (const target of ARTIFACTS) {
      const { variants, leaves } = declaredMutations(this.repoRoot, target);
      const covered = leaves.filter((leaf) => dirsIn(join(leaf.path, "🧪️tests")).length > 0).length;
      console.log(`🧬️ ${target.artifact}: ${variants.length} schema mutations · ${leaves.length} leaves · ${covered} covered · ${leaves.length - covered} uncovered`);
    }
    const warnings = findings.filter((finding) => finding.level === "warn");
    for (const finding of errors) console.log(`❌️ ${finding.where}: ${finding.what}`);
    if (warnings.length > 0) console.log(`⚠️ ${warnings.length} derived-encoding gap(s) pending \`fixtures generate\` (contract D1 target; run with --full to fail on them)`);
    console.log(errors.length === 0 ? "✅️ fixture contract satisfied" : `❌️ ${errors.length} error(s)`);
    process.exit(errors.length === 0 ? 0 : 1);
  }
}
//#endregion 🔖️FixtureLint

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("describe", DescribeScript)
  .register("fixtures", FixturesScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
