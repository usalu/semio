import { resolveTestLevel } from "@semio-tech/process/test-budget";
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
import { runVitest } from "@semio-tech/repo-lib/testing";
import { BundleScript } from "@semio-tech/process/routing";



export class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    const { toolJobPuzzleReservedRoutesSelfTests } = await import("@semio-tech/puzzle-js/testing/tool-job-puzzle-reserved-routes");
    const { toolJobPuzzleReservedRoutesExact, toolJobPuzzle2dReservedRoutesExact } = await import("@semio-tech/repo-lib/laws");
    console.log(`puzzle-reserved-route-contributions: checks=${toolJobPuzzleReservedRoutesSelfTests({ toolJobPuzzleReservedRoutesExact, toolJobPuzzle2dReservedRoutesExact })}`);
    resolveTestLevel([], "quick");
    await runVitest(join(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript"), [], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

//#region 🔖️FixtureLint
/** 🧬️ One artifact mutation tree: the directory that directly contains the mutation leaves. */
type ArtifactTarget = { readonly label: string; readonly mutationsRoot: string };

/** 🔍️ Walks source directories without following package links, cache files or test inputs. */
const excludedDirectories = new Set(["node_modules", "target", "dist", "temp", "storybook-static", "🧪️tests", "🗑️generated", "🤖️generated"]);

function sourceDirectories(path: string): string[] {
  return readdirSync(path, { withFileTypes: true })
    .filter(entry => entry.isDirectory() && !entry.name.startsWith(".") && !excludedDirectories.has(entry.name))
    .map(entry => entry.name);
}

/** 🧬️ Discovers mutation vocabularies from their Rust enum declarations. */
function discoverArtifacts(repoRoot: string): ArtifactTarget[] {
  const found: ArtifactTarget[] = [];
  const walk = (dir: string): void => {
    for (const entry of sourceDirectories(dir)) {
      const child = join(dir, entry);
      if (entry === "🧬️mutations") {
        const target = { label: relative(repoRoot, child), mutationsRoot: child };
        if (mutationVariants(child).length) found.push(target);
      } else {
        walk(child);
      }
    }
  };
  walk(repoRoot);
  return found.sort((left, right) => left.label.localeCompare(right.label));
}

/** 📄️ The source-of-truth files every test case must carry. These are hand-authored and are what
 * the committed Rust test asserts against. `🔺️diff` is replaced by `🔺️diff/🚫️.absent`
 * when `🎯️outcome` declares `rejected` (contract D6). */
const CORE_CASE_FILES = ["🦠️mutation/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json"] as const;

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
const SNAPSHOT_CORE = "🔣️.json";
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"] as const;
const SNAPSHOT_REF = "🔗️component.ref.json";

type Finding = { readonly level: "error" | "warn"; readonly where: string; readonly what: string };

const dirsIn = (path: string): string[] =>
  existsSync(path) ? readdirSync(path, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => entry.name) : [];

function mutationVariants(root: string) {
  const owners = [root, ...sourceDirectories(root).map(entry => join(root, entry))];
  return owners.flatMap(owner => {
    const file = [join(owner, "🦀️.rs"), join(owner, "🦠️mutation", "🦀️.rs")].find(existsSync);
    if (!file) return [];
    const source = readFileSync(file, "utf8");
    return [...source.matchAll(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/g)].flatMap(match =>
      [...match[1].matchAll(/^\s+([A-Z]\w*)\(\s*([\w:]+)/gm)].map(variant => ({ name: variant[1], payload: variant[2].split("::").at(-1)! })));
  });
}

/** 🦠️ Finds declared mutation payloads under their domain groups. */
function declaredMutations(target: ArtifactTarget) {
  const mutationsRoot = target.mutationsRoot;
  const variants = mutationVariants(mutationsRoot);
  const names = new Set(variants.map(variant => variant.payload));
  const leaves: { dir: string; struct: string; path: string }[] = [];
  const walk = (path: string): void => {
    for (const entry of sourceDirectories(path)) {
      const child = join(path, entry);
      const file = [join(child, "🦠️mutation", "🦀️.rs"), join(child, "🦀️.rs")].find(existsSync);
      const source = file ? readFileSync(file, "utf8") : "";
      const payload = [...source.matchAll(/pub struct (\w+)/g)].find(match => names.has(match[1]))?.[1]
        ?? source.match(/#\[(?:dsl\(keyword|mutation_leaf\()[^\]]*\][\s\S]*?pub struct (\w+)/)?.[1];
      if (payload) leaves.push({ dir: relative(mutationsRoot, child), struct: payload, path: child });
      else walk(child);
    }
  };
  walk(mutationsRoot);
  return { mutationsRoot, variants, leaves };
}

/** ✅️ Enforces contract D1/D6: schema variants == mutation leaves == test subjects, and every test
 * case carries a complete codec set. Returns findings; never throws on a malformed tree. */
function lintArtifact(target: ArtifactTarget, { variants, leaves }: ReturnType<typeof declaredMutations>, full: boolean): Finding[] {
  const findings: Finding[] = [];

  const byStruct = new Set(leaves.map(leaf => leaf.struct));
  const referenced = new Set(variants.map((variant) => variant.payload));
  for (const leaf of leaves) {
    if (!referenced.has(leaf.struct)) {
      findings.push({ level: "error", where: `${target.label}/${leaf.dir}`, what: `payload ${leaf.struct} is declared but no enum variant wraps it` });
    }
  }
  for (const variant of variants) {
    if (!byStruct.has(variant.payload)) findings.push({ level: "error", where: target.label, what: `missing mutation payload ${variant.payload}` });
  }
  const fixtures = join(target.mutationsRoot, "..", "..", "🧪️tests", "🧫️fixtures", "🧬️mutations");
  for (const leaf of leaves) {
    const descriptor = join(leaf.path, "🔣️.json");
    const outcomes: string[] = (existsSync(descriptor) && JSON.parse(readFileSync(descriptor, "utf8"))?.outcomeClasses) || ["applied", "no-op", "rejected"];
    const cases = dirsIn(join(leaf.path, "🧪️tests"));
    if (cases.length === 0) {
      findings.push({ level: "error", where: `${target.label}/${leaf.dir}`, what: "no 🧪️tests cases — every mutation must be directly tested" });
      continue;
    }
    for (const testCase of cases) {
      const label = `${target.label}/${leaf.dir}/${testCase}`;
      const testDir = join(leaf.path, "🧪️tests", testCase);
      if (!existsSync(join(testDir, "🦀️.rs"))) findings.push({ level: "error", where: label, what: "missing Rust test" });
      findings.push(...lintCase(existsSync(fixtures) ? join(fixtures, leaf.dir, testCase) : testDir, label, full, outcomes));
    }
  }
  return findings;
}

/** 🧪️ Enforces one test case's file set. `full` also demands the derived encodings (contract D1);
 * without it only the hand-authored source-of-truth set is required and derived gaps are warnings. */
function lintCase(caseDir: string, label: string, full: boolean, outcomes: readonly string[]): Finding[] {
  const findings: Finding[] = [];
  const outcomeFile = join(caseDir, "🎯️outcome/🔣️.json");
  let rejected = false;
  if (existsSync(outcomeFile)) {
    try {
      const outcome = JSON.parse(readFileSync(outcomeFile, "utf8"));
      rejected = outcome.status === "rejected";
      if (!outcomes.includes(outcome.status)) findings.push({ level: "error", where: label, what: `🎯️outcome.status must be one of ${outcomes.join(", ")}, got ${JSON.stringify(outcome.status)}` });
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
  if (rejected && !existsSync(join(caseDir, "🔺️diff/🚫️.absent"))) {
    findings.push({ level: "error", where: label, what: "rejected case must carry 🔺️diff/🚫️.absent, not an invented empty patch" });
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
export class FixturesScript extends BundleScript {
  run(segments: string[]): void {
    const sub = segments[0] ?? "lint";
    if (sub !== "lint") {
      console.error("usage: repo ./📜️script.ts fixtures lint [--full]");
      process.exit(1);
    }
    const full = segments.includes("--full");
    const targets = discoverArtifacts(this.repoRoot);
    const vocabularies = targets.map(target => ({ target, vocabulary: declaredMutations(target) }));
    const findings = vocabularies.flatMap(({ target, vocabulary }) => lintArtifact(target, vocabulary, full));
    const errors = findings.filter((finding) => finding.level === "error");
    const warnings = findings.filter((finding) => finding.level === "warn");

    let totalLeaves = 0;
    let totalCovered = 0;
    const uncoveredByTree: string[] = [];
    for (const { target, vocabulary: { leaves } } of vocabularies) {
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
