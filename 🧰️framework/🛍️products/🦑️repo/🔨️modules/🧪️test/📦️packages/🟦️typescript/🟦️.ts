//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { constants, cpSync, existsSync, linkSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { type BreachRecord, TEST_LEVELS, type TestLevel, findRepoRoot, getRepoMetaDir, runProbe, testLevelBudgetMs } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔣️Contract
/** 🔣️ Repo-relative path of the SSOT taxonomy the whole test platform reads its vocabulary from. */
export const TAXONOMY_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

/** 🧪️ Repo-relative root of the testing domain that owns this contract. */
export const TEST_DOMAIN_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";

/** 🧪️ Implementations a test case may claim an adapter for. */
export const IMPLEMENTATIONS = ["rust", "typescript", "go", "python", "dotnet"] as const;
export type Implementation = (typeof IMPLEMENTATIONS)[number];

/** 🎭️ The two roles one scenario is executed in — the third-party reference and this repository's code. */
export const TEST_ROLES = ["oracle", "subject"] as const;
export type TestRole = (typeof TEST_ROLES)[number];

/** 🎯️ Execution modes a scenario declares with `@mode-…`. */
export const TEST_MODES = ["differential", "conformance", "round-trip", "property", "error"] as const;
export type TestMode = (typeof TEST_MODES)[number];

/**
 * ⚖️ A comparison profile id. Deliberately an OPEN string rather than a closed union: the framework
 * owns the comparison MECHANISM and its domain-neutral profiles, while a profile that knows a file
 * format is contributed by the owner of that format. Adding one must never require editing this file.
 */
export type ComparisonProfile = string;

/**
 * ⚖️ The declarative policy of one profile. Everything a profile can say is data, so a new profile is
 * a manifest entry — never new framework code.
 */
export type ComparisonProfileSpec = Readonly<{
  id: string;
  /** 📖️ Why this profile exists and what it deliberately treats as producer freedom. */
  description?: string;
  /** 🔢️ `ordered` keeps array order significant; `set` compares arrays as multisets. */
  arrays?: "ordered" | "set";
  /** 🚫️ Keys stripped before comparison — the producer freedom this format allows. */
  ignoreKeys?: readonly string[];
  /** 📏️ Numeric tolerance, and the grid values are rounded onto before comparison. */
  tolerance?: number;
  /** 🔤️ `utf8` normalizes line endings, trailing whitespace and Unicode form. */
  text?: "none" | "utf8";
  /** 💾️ Compare the whole projection as an opaque byte string. */
  bytes?: boolean;
  /** ⚖️ The multi-artifact, externally-probed pipeline this profile delegates to. A profile that names one compares an artifact BUNDLE; the structural fields above then describe only its projection half. */
  pipeline?: string;
}>;

/** ⚖️ The domain-neutral profiles the framework itself owns. Nothing here knows a file format. */
export const CORE_COMPARISON_PROFILES: readonly ComparisonProfileSpec[] = [
  { id: "exact-bytes-v1", description: "Byte-for-byte identity — used only where byte determinism is itself the requirement.", bytes: true },
  { id: "utf8-text-v1", description: "Text identity after normalizing line endings, trailing whitespace and Unicode form.", text: "utf8" },
  { id: "ordered-json-v1", description: "Structural identity with array order significant; key order never is." },
  { id: "unordered-json-v1", description: "Structural identity with arrays compared as multisets.", arrays: "set" },
  { id: "floating-point-v1", description: "Structural identity with a numeric tolerance for representation noise.", tolerance: 1e-9 },
  { id: "filesystem-tree-v1", description: "A directory listing, where enumeration order is never normative.", arrays: "set" },
  { id: "diagnostic-v1", description: "Diagnostics compared by kind and message, not by position or rendering.", arrays: "set", ignoreKeys: ["line", "column", "offset", "span", "detail"] },
  { id: "event-stream-v1", description: "An event stream compared by content, not by wall-clock or process identity.", ignoreKeys: ["timestamp", "durationMs", "elapsedMs", "pid", "threadId"] },
];

type TestFileKind = Readonly<{ emoji: string; extensionChains: readonly string[] }>;
type TestPathExclusion = Readonly<{ path: string; mode: "opaque"; reason: string }>;
type TestLocation = Readonly<{ directoryPath: string; fileKindId: string }>;

/** 🗂️ Canonical v7 test taxonomy, mirrored from `🔣️taxonomy.json` so drift fails closed. */
export type TestTaxonomy = Readonly<{
  fileKinds: Readonly<Record<string, TestFileKind>>;
  pathExclusions: Readonly<Record<string, TestPathExclusion>>;
  testsDirName: string;
  testFixturesDirName: string;
  testFeatureFileKindId: string;
  testCaseSlugPattern: string;
  testAdapterFileKinds: Readonly<Record<string, string>>;
  testImplementationIds: Readonly<Record<string, string>>;
  testOutputCacheDirName: string;
  testOutputMarkerFileKindId: string;
  testOutputMarkerKind: string;
  testOutputChildDirs: readonly string[];
  testOracleRegistryLocation: TestLocation;
  testSchemaLocation: TestLocation;
  testContributionDirName: string;
  testContributionFileKindId: string;
  /** 🔬️ Directory name of an owner's external measurement probes — test-owned by what it is. */
  testProbeDirName: string;
  /** 🏭️ Directory name of an owner's third-party fixture generator — test-owned by what it is. */
  testGeneratorDirName: string;
  /** 🏭️ Directory name of an owner's production mutation bridge. Production-side by design: it links the real implementation so `listMutations` answers from dispatch. */
  testBridgeDirName: string;
  testDomainPath: string;
  testPhases: readonly string[];
  testLevellessPhases: readonly string[];
  testMutationVocabularyDirName: string;
}>;

let taxonomyCache: { root: string; value: TestTaxonomy } | null = null;

/** 🔣️ Loads the frozen test vocabulary out of the SSOT taxonomy. Never re-declare these strings locally. */
export function testTaxonomy(repoRoot: string): TestTaxonomy {
  if (taxonomyCache && taxonomyCache.root === repoRoot) return taxonomyCache.value;
  const parsed = JSON.parse(readFileSync(join(repoRoot, TAXONOMY_REL_PATH), "utf8")) as Record<string, unknown>;
  const required = ["fileKinds", "pathExclusions", "testsDirName", "testFixturesDirName", "testFeatureFileKindId", "testCaseSlugPattern", "testAdapterFileKinds", "testImplementationIds", "testOutputCacheDirName", "testOutputMarkerFileKindId", "testOutputMarkerKind", "testOutputChildDirs", "testOracleRegistryLocation", "testSchemaLocation", "testContributionDirName", "testContributionFileKindId", "testProbeDirName", "testGeneratorDirName", "testBridgeDirName", "testDomainPath", "testPhases", "testLevellessPhases", "testMutationVocabularyDirName"];
  const missing = required.filter((key) => parsed[key] === undefined);
  if (missing.length > 0) throw new Error(`🔣️taxonomy.json is missing the test contract keys: ${missing.join(", ")}`);
  const value = Object.fromEntries(required.map((key) => [key, parsed[key]])) as unknown as TestTaxonomy;
  taxonomyCache = { root: repoRoot, value };
  return value;
}

/** 📄️ Resolves the taxonomy-ordered kind-only filename for a test contract. */
export function testFilenameForKind(taxonomy: TestTaxonomy, kindId: string): string {
  const kind = taxonomy.fileKinds[kindId];
  if (!kind || kind.extensionChains.length === 0) throw new Error(`🔣️taxonomy.json file kind "${kindId}" has no canonical filename`);
  return `${kind.emoji}${kind.extensionChains[0]}`;
}

/** 📍️ Resolves a taxonomy location to its canonical repository-relative file path. */
export function testLocationPath(taxonomy: TestTaxonomy, location: TestLocation): string {
  return `${location.directoryPath}/${testFilenameForKind(taxonomy, location.fileKindId)}`;
}

/** 🥒️ Canonical feature filename. */
export function testFeatureFilename(taxonomy: TestTaxonomy): string {
  return testFilenameForKind(taxonomy, taxonomy.testFeatureFileKindId);
}

/** 🔌️ Canonical implementation adapter filenames keyed by language identifier. */
export function testAdapterFilenames(taxonomy: TestTaxonomy): Readonly<Record<string, string>> {
  return Object.fromEntries(Object.entries(taxonomy.testAdapterFileKinds).map(([key, kindId]) => [key, testFilenameForKind(taxonomy, kindId)]));
}

/** 🚫️ Excluded paths come from the taxonomy and are applied HERE, in the discovery library — never
 * only by a CI path filter. This function names no area; which ones are excluded is vocabulary. */
export function isExcludedTestPath(repoRoot: string, relPath: string): boolean {
  const normalized = relPath.split(sep).join("/");
  // 🚫️The repository's OWN meta directory is excluded structurally, not by a listed path: it holds
  // tickets, caches, notes and metrics, and a ticket folder routinely contains a SCRATCH COPY of a
  // plugin subtree. Those copies were discovered as real owners and real test cases — at one point
  // the only four cases discovery could find were scratch copies inside one ticket, while every
  // committed case was invisible. It is derived from `getRepoMetaDir` rather than spelled here so
  // relocating the cache is still a vocabulary change.
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  if (metaRoot.length > 0 && !metaRoot.startsWith("..") && (normalized === metaRoot || normalized.startsWith(`${metaRoot}/`))) return true;
  return Object.values(testTaxonomy(repoRoot).pathExclusions).some(({ path }) => normalized === path.replace(/\/$/, "") || normalized.startsWith(path) || normalized.includes(`/${path}`));
}

/** 🧭️ Maps a taxonomy adapter filename key (`🦀️rust`) to its stable implementation id (`rust`). */
export function implementationOfAdapterKey(repoRoot: string, key: string): Implementation | null {
  const id = testTaxonomy(repoRoot).testImplementationIds[key];
  return (IMPLEMENTATIONS as readonly string[]).includes(id ?? "") ? (id as Implementation) : null;
}
//#endregion 🔣️Contract

//#region #⃣Digest
/** #⃣ Stable content digest used for feature/fixture/source/output identity. Truncated sha256, hex. */
export function digest(input: string | Uint8Array): string {
  return createHash("sha256").update(input).digest("hex").slice(0, 32);
}

/** #⃣ Digest of a file's bytes; the empty digest for a missing file so a plan stays serializable. */
export function fileDigest(absPath: string): string {
  if (!existsSync(absPath)) return digest("");
  return digest(readFileSync(absPath));
}

/** #⃣ Order-independent digest of a set of `(name, digest)` pairs. */
export function setDigest(pairs: readonly (readonly [string, string])[]): string {
  return digest(
    [...pairs]
      .map(([a, b]) => `${a} ${b}`)
      .sort()
      .join(""),
  );
}
//#endregion #⃣Digest

//#region 🥒️Gherkin
/** 🥒️ One `Given`/`When`/`Then` step. `And`/`But` inherit the previous step's canonical keyword. */
export type FeatureStep = Readonly<{ keyword: "Given" | "When" | "Then"; rawKeyword: string; text: string; docString?: string; dataTable?: readonly (readonly string[])[] }>;

/** 🥒️ One executable scenario, after `Scenario Outline` expansion. */
export type FeatureScenario = Readonly<{
  id: string;
  name: string;
  level: TestLevel;
  mode: TestMode;
  tags: readonly string[];
  steps: readonly FeatureStep[];
  seed?: string;
  platforms?: readonly string[];
  requires?: readonly string[];
  implementations?: readonly Implementation[];
  outlineOf?: string;
  line: number;
}>;

/** 🥒️ The parsed, language-neutral behavioural contract of one test case. */
export type ParsedFeature = Readonly<{
  name: string;
  description: string;
  tags: readonly string[];
  capability: string | null;
  oracle: string | null;
  noOracleDecision: string | null;
  comparison: ComparisonProfile | null;
  /** 📥️ The produced subject artifact an oracle must consume, if it validates bytes rather than re-producing them. */
  oracleInput: "subject-raw" | null;
  /** 🦠️ The mutation catalog this feature claims to cover exhaustively, from `@mutations-<id>`. */
  mutationCatalog: string | null;
  background: readonly FeatureStep[];
  scenarios: readonly FeatureScenario[];
  errors: readonly string[];
}>;

const STEP_KEYWORDS = new Set(["Given", "When", "Then", "And", "But", "*"]);

function tagValue(tags: readonly string[], prefix: string): string | null {
  const hit = tags.find((tag) => tag.startsWith(prefix));
  return hit ? hit.slice(prefix.length) : null;
}

function tagValues(tags: readonly string[], prefix: string): string[] {
  return tags.filter((tag) => tag.startsWith(prefix)).map((tag) => tag.slice(prefix.length));
}

function splitTags(line: string): string[] {
  return line
    .split(/\s+/)
    .map((piece) => piece.trim())
    .filter((piece) => piece.startsWith("@"));
}

function splitTableRow(line: string): string[] {
  const trimmed = line.trim();
  const inner = trimmed.slice(1, trimmed.length - 1);
  return inner.split("|").map((cell) => cell.trim().replace(/\\\|/g, "|"));
}

function substitute(text: string, row: Readonly<Record<string, string>>): string {
  return text.replace(/<([^<>]+)>/g, (whole, key: string) => (key in row ? row[key]! : whole));
}

type FeatureBlock = { kind: "none" | "feature" | "background" | "scenario" | "outline"; name: string; tags: string[]; steps: FeatureStep[]; examples: Record<string, string>[]; line: number };

/**
 * 🥒️ Parses the repository's restricted Gherkin profile into one owned plan. The coordinator parses
 * a feature exactly once and hands every native host the resulting plan — no host re-reads or
 * reinterprets `component.feature`, which is what keeps five languages provably in agreement.
 * @see https://cucumber.io/docs/gherkin/reference/
 */
export function parseFeature(source: string): ParsedFeature {
  const errors: string[] = [];
  const lines = source.split(/\r?\n/);

  let featureName = "";
  const descriptionLines: string[] = [];
  let featureTags: string[] = [];
  let pendingTags: string[] = [];
  const background: FeatureStep[] = [];
  const scenarios: FeatureScenario[] = [];

  let block: FeatureBlock = { kind: "none", name: "", tags: [], steps: [], examples: [], line: 0 };
  let lastKeyword: "Given" | "When" | "Then" = "Given";
  let exampleHeader: string[] | null = null;
  let inExamples = false;

  const flush = (): void => {
    if (block.kind === "background") background.push(...block.steps);
    if (block.kind === "scenario") scenarios.push(...materializeScenario(block, null, errors));
    if (block.kind === "outline") {
      if (block.examples.length === 0) errors.push(`Scenario Outline "${block.name}" (line ${block.line}) has no Examples rows`);
      block.examples.forEach((row, index) => scenarios.push(...materializeScenario(block, { row, index }, errors)));
    }
    block = { kind: "none", name: "", tags: [], steps: [], examples: [], line: 0 };
    exampleHeader = null;
    inExamples = false;
  };

  for (let i = 0; i < lines.length; i += 1) {
    const raw = lines[i]!;
    const line = raw.trim();
    const lineNo = i + 1;
    if (line === "" || line.startsWith("#")) continue;

    if (line.startsWith("@")) {
      pendingTags = [...pendingTags, ...splitTags(line)];
      continue;
    }

    const header = line.match(/^(Feature|Background|Scenario Outline|Scenario Template|Scenario|Example|Examples|Scenarios):\s*(.*)$/);
    if (header) {
      const keyword = header[1]!;
      const name = header[2]!.trim();
      if (keyword === "Examples" || keyword === "Scenarios") {
        if (block.kind !== "outline") errors.push(`Examples at line ${lineNo} does not follow a Scenario Outline`);
        inExamples = true;
        exampleHeader = null;
        pendingTags = [];
        continue;
      }
      if (keyword === "Feature") {
        flush();
        featureName = name;
        featureTags = pendingTags;
        pendingTags = [];
        block = { kind: "feature", name, tags: [], steps: [], examples: [], line: lineNo };
        continue;
      }
      flush();
      if (keyword === "Background") {
        block = { kind: "background", name, tags: [], steps: [], examples: [], line: lineNo };
      } else if (keyword === "Scenario Outline" || keyword === "Scenario Template") {
        block = { kind: "outline", name, tags: pendingTags, steps: [], examples: [], line: lineNo };
      } else {
        block = { kind: "scenario", name, tags: pendingTags, steps: [], examples: [], line: lineNo };
      }
      pendingTags = [];
      lastKeyword = "Given";
      continue;
    }

    if (line.startsWith("|")) {
      const cells = splitTableRow(line);
      if (inExamples) {
        if (exampleHeader === null) {
          exampleHeader = cells;
        } else {
          const row: Record<string, string> = {};
          exampleHeader.forEach((key, index) => {
            row[key] = cells[index] ?? "";
          });
          block.examples.push(row);
        }
        continue;
      }
      const target = block.steps[block.steps.length - 1];
      if (!target) {
        errors.push(`Data table at line ${lineNo} does not follow a step`);
        continue;
      }
      block.steps[block.steps.length - 1] = { ...target, dataTable: [...(target.dataTable ?? []), cells] };
      continue;
    }

    if (line === '"""' || line === "```") {
      const closer = line;
      const body: string[] = [];
      let j = i + 1;
      while (j < lines.length && lines[j]!.trim() !== closer) {
        body.push(lines[j]!);
        j += 1;
      }
      if (j >= lines.length) errors.push(`Unterminated doc string opened at line ${lineNo}`);
      const target = block.steps[block.steps.length - 1];
      if (!target) errors.push(`Doc string at line ${lineNo} does not follow a step`);
      else block.steps[block.steps.length - 1] = { ...target, docString: dedent(body) };
      i = j;
      continue;
    }

    const stepMatch = line.match(/^(Given|When|Then|And|But|\*)\s+(.*)$/);
    if (stepMatch && STEP_KEYWORDS.has(stepMatch[1]!)) {
      const rawKeyword = stepMatch[1]!;
      const keyword: "Given" | "When" | "Then" = rawKeyword === "Given" || rawKeyword === "When" || rawKeyword === "Then" ? rawKeyword : lastKeyword;
      lastKeyword = keyword;
      if (block.kind === "none" || block.kind === "feature") {
        errors.push(`Step at line ${lineNo} is outside a Background or Scenario`);
        continue;
      }
      block.steps.push({ keyword, rawKeyword, text: stepMatch[2]!.trim() });
      continue;
    }

    if (block.kind === "feature") {
      descriptionLines.push(raw.trim());
      continue;
    }
    errors.push(`Unrecognized line ${lineNo}: ${JSON.stringify(line)}`);
  }
  flush();

  if (featureName === "") errors.push("Feature has no `Feature:` header");
  const capability = tagValue(featureTags, "@capability-");
  const oracle = tagValue(featureTags, "@oracle-");
  const noOracleDecision = tagValue(featureTags, "@no-oracle-");
  const comparisonRaw = tagValue(featureTags, "@comparison-");
  const oracleInput = tagValue(featureTags, "@oracle-input-");
  const mutationCatalog = tagValue(featureTags, "@mutations-");
  // 🧭️The parser records the declared profile; whether that profile EXISTS is registry knowledge and
  // is checked in the contract phase, so the Gherkin profile stays independent of which formats the
  // repository happens to own today.
  const comparison: ComparisonProfile | null = comparisonRaw;

  const seen = new Set<string>();
  for (const scenario of scenarios) {
    if (seen.has(scenario.id)) errors.push(`Duplicate scenario id @id-${scenario.id}`);
    seen.add(scenario.id);
  }

  if (oracleInput !== null && oracleInput !== "subject-raw") errors.push(`Unknown oracle input @oracle-input-${oracleInput}`);
  return { name: featureName, description: descriptionLines.join("\n").trim(), tags: featureTags, capability, oracle, noOracleDecision, comparison, oracleInput: oracleInput === "subject-raw" ? oracleInput : null, mutationCatalog, background, scenarios, errors };
}

function dedent(body: readonly string[]): string {
  const indents = body.filter((line) => line.trim() !== "").map((line) => line.length - line.trimStart().length);
  const shift = indents.length === 0 ? 0 : Math.min(...indents);
  return body.map((line) => line.slice(shift)).join("\n");
}

function materializeScenario(block: FeatureBlock, example: { row: Record<string, string>; index: number } | null, errors: string[]): FeatureScenario[] {
  const tags = block.tags;
  const baseId = tagValue(tags, "@id-");
  const levels = tagValues(tags, "@level-").filter((value) => (TEST_LEVELS as readonly string[]).includes(value)) as TestLevel[];
  const modes = tagValues(tags, "@mode-").filter((value) => (TEST_MODES as readonly string[]).includes(value)) as TestMode[];
  const where = `"${block.name}" (line ${block.line})`;
  if (baseId === null) errors.push(`Scenario ${where} is missing its @id-<stable-id> tag`);
  if (levels.length !== 1) errors.push(`Scenario ${where} must carry exactly one @level-<fundamental|quick|long|exhaustive> tag (found ${levels.length})`);
  if (modes.length !== 1) errors.push(`Scenario ${where} must carry exactly one @mode-<differential|conformance|round-trip|property|error> tag (found ${modes.length})`);
  if (baseId === null || levels.length !== 1 || modes.length !== 1) return [];
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(baseId)) {
    errors.push(`Scenario id @id-${baseId} is not kebab-case`);
    return [];
  }
  const row = example?.row ?? {};
  const id = example === null ? baseId : `${baseId}-${row.id ?? String(example.index + 1)}`;
  const implementations = tagValues(tags, "@implementation-").filter((value) => (IMPLEMENTATIONS as readonly string[]).includes(value)) as Implementation[];
  const steps = block.steps.map((step) => ({ ...step, text: substitute(step.text, row), docString: step.docString === undefined ? undefined : substitute(step.docString, row), dataTable: step.dataTable?.map((cells) => cells.map((cell) => substitute(cell, row))) }));
  return [
    {
      id,
      name:
        example === null
          ? block.name
          : `${block.name} [${Object.entries(row)
              .map(([key, value]) => `${key}=${value}`)
              .join(", ")}]`,
      level: levels[0]!,
      mode: modes[0]!,
      tags,
      steps,
      seed: tagValue(tags, "@seed-") ?? undefined,
      platforms: tagValues(tags, "@platform-"),
      requires: tagValues(tags, "@requires-"),
      implementations: implementations.length > 0 ? implementations : undefined,
      outlineOf: example === null ? undefined : baseId,
      line: block.line,
    },
  ];
}
//#endregion 🥒️Gherkin

//#region 🔍️Discovery
/** 🔍️ One discovered test case: its owner, slug, feature, adapters and fixtures. */
export type DiscoveredCase = Readonly<{
  owner: string;
  ownerName: string;
  case: string;
  caseDir: string;
  featurePath: string;
  adapters: Readonly<Partial<Record<Implementation, string>>>;
  sharedFixtureDir: string | null;
  localFixtureDir: string | null;
  projectName: string;
}>;

const SKIP_DIR_NAMES = new Set(["node_modules", ".git", "target", "dist", "build", "out", "storybook-static", ".venv", "__pycache__", "obj", "bin"]);

function walkDirectories(root: string, onDir: (absDir: string, relDir: string) => "enter" | "skip"): void {
  const stack: string[] = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      const abs = join(dir, name);
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (!st.isDirectory() || st.isSymbolicLink()) continue;
      if (SKIP_DIR_NAMES.has(name)) continue;
      if (onDir(abs, relative(root, abs).split(sep).join("/")) === "enter") stack.push(abs);
    }
  }
}

/** 🏷️ Deterministic Nx project name for one case — stable across platforms and safe for the CLI. */
export function testProjectName(ownerRel: string, caseSlug: string): string {
  const slug = ownerRel
    .split("/")
    .map((segment) => segment.replace(/[^a-zA-Z0-9]+/g, "").toLowerCase())
    .filter(Boolean)
    .join("-");
  return `test-${slug || "root"}-${digest(ownerRel).slice(0, 6)}-${caseSlug}`;
}

/**
 * 🔍️ Walks every non-excluded path for `**​/🧪️tests/<case>/component.feature`. The owner is the
 * directory that holds `🧪️tests`; a case found under `📦️packages/<lang>` is still returned so the
 * taxonomy policy can report it rather than silently dropping the evidence.
 */
export function discoverTestCases(repoRoot: string): DiscoveredCase[] {
  const taxonomy = testTaxonomy(repoRoot);
  const featureFilename = testFeatureFilename(taxonomy);
  const adapterFilenames = testAdapterFilenames(taxonomy);
  const found: DiscoveredCase[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== taxonomy.testsDirName) return "enter";
    const ownerAbs = dirname(abs);
    const ownerRel = relative(repoRoot, ownerAbs).split(sep).join("/") || ".";
    for (const entry of readdirSync(abs)) {
      const caseDir = join(abs, entry);
      let st;
      try {
        st = lstatSync(caseDir);
      } catch {
        continue;
      }
      if (!st.isDirectory()) continue;
      const featurePath = join(caseDir, featureFilename);
      if (!existsSync(featurePath)) continue;
      const adapters: Partial<Record<Implementation, string>> = {};
      for (const [key, filename] of Object.entries(adapterFilenames)) {
        const impl = implementationOfAdapterKey(repoRoot, key);
        if (!impl) continue;
        const adapterPath = join(caseDir, filename);
        if (existsSync(adapterPath)) adapters[impl] = relative(repoRoot, adapterPath).split(sep).join("/");
      }
      const sharedFixtureDir = join(ownerAbs, taxonomy.testFixturesDirName);
      const localFixtureDir = join(caseDir, taxonomy.testFixturesDirName);
      found.push({
        owner: ownerRel,
        ownerName: basename(ownerAbs),
        case: entry,
        caseDir: relative(repoRoot, caseDir).split(sep).join("/"),
        featurePath: relative(repoRoot, featurePath).split(sep).join("/"),
        adapters,
        sharedFixtureDir: existsSync(sharedFixtureDir) ? relative(repoRoot, sharedFixtureDir).split(sep).join("/") : null,
        localFixtureDir: existsSync(localFixtureDir) ? relative(repoRoot, localFixtureDir).split(sep).join("/") : null,
        projectName: testProjectName(ownerRel, entry),
      });
    }
    return "skip";
  });
  return found.sort((a, b) => a.projectName.localeCompare(b.projectName));
}
//#endregion 🔍️Discovery

//#region 📇️Registry
/** 📇️ One approved third-party reference implementation, test-only by construction. */
export type OracleEntry = Readonly<{
  id: string;
  /** 🎭️ What this reference IS. Only a `QUALIFYING_ORACLE_KINDS` member can discharge a mutation's external-oracle requirement; everything else is a required supplement. */
  kind?: OracleKind;
  ecosystem: string;
  package: string;
  packages?: readonly OracleLinkedPackage[];
  version?: string;
  lockDigest?: string;
  source?: { repository?: string; commit?: string; license?: string };
  /** ⚙️ The kernel this reference actually sits on. Independence is accounted per FAMILY: two wrappers around one kernel are one oracle. */
  engine?: EngineFamily;
  capabilities: readonly string[];
  comparisonProfiles: readonly ComparisonProfile[];
  license: string;
  testOnly: true;
  productionReachable?: boolean;
  networkDuringExecution?: boolean;
  platforms?: readonly PlatformId[];
  homepage?: string;
  rationale?: string;
  hostPath?: string;
  productionDebt?: { reachableFrom: readonly string[]; owner: string; plan: string };
  /** 🌱️ Required, and checked by `nativeSecondImplementationBreaches`, when `kind` is `verified-native-second-implementation` — absent otherwise. */
  nativeSecondImplementation?: NativeSecondImplementationEvidence;
}>;

/**
 * 🧩️ One further third-party package a composed reference links beyond its primary one, pinned and
 * licensed in its own right. A secondary package that inherited the entry's version would be
 * recorded under a version it is not at, which is a worse record than none.
 */
export type OracleLinkedPackage = Readonly<{ package: string; version: string; license: string; homepage?: string; role: string }>;

/**
 * 🧩️ Every third-party package one registered oracle actually links, each with its own pin. Most name
 * a single package; a composed reference names several, because some formats have no single credible
 * crate — an OOXML container needs an archive reader AND an XML reader, and a spreadsheet needs a
 * reader AND a writer. Declaring only one of them would leave the other unclassified, which is
 * precisely what the dependency ratchet exists to prevent.
 */
export function oracleLinkedPackages(entry: OracleEntry): OracleLinkedPackage[] {
  const primary: OracleLinkedPackage = { package: entry.package, version: entry.version ?? "*", license: entry.license, homepage: entry.homepage, role: "primary reference" };
  const byName = new Map<string, OracleLinkedPackage>();
  for (const linked of [primary, ...(entry.packages ?? [])]) if (linked.package.length > 0 && !byName.has(linked.package)) byName.set(linked.package, linked);
  return [...byName.values()];
}

/** 🧩️ The names alone of every package one registered oracle links — what the import probe scans for. */
export function oraclePackages(entry: OracleEntry): string[] {
  return oracleLinkedPackages(entry).map((linked) => linked.package);
}

/** 📇️ A recorded decision that a capability legitimately has no credible reference implementation. */
export type NoOracleDecision = Readonly<{ id: string; capabilities: readonly string[]; rationale: string; substitutes: readonly string[] }>;

/**
 * 🦠️ One owner's declared mutation vocabulary for an artifact: the complete list of mutation kinds
 * the implementation can apply. This is the ground truth the completeness gate holds a feature
 * against, so "exhaustive" becomes a machine-checked claim instead of a hand-counted one.
 *
 * The framework never parses implementation source to learn this list — that would make it depend on
 * a language and on the shape of somebody else's enum. The OWNER declares it here and proves the
 * declaration honest with its own adjacent test.
 */
/** 🧪️ One canonical projected scenario identity. */
export type MutationVectorScenario = Readonly<{ id: string; directoryName: string }>;

/** 🧬️ One physical mutation directory and every scenario bundle it contributes to projection. */
export type MutationVector = Readonly<{ mutationId: string; sourceMutationDirectoryName: string; mutationDirectoryName: string; scenarios: readonly MutationVectorScenario[] }>;

/** 🦠️ Runtime capability vocabulary plus its independent physical projection registry. */
export type MutationCatalog = Readonly<{
  id: string;
  capability: string;
  standardDirectoryName?: string;
  subsetDirectoryName?: string;
  kinds: readonly string[];
  vectors: readonly MutationVector[];
  deferredKinds?: readonly string[];
}>;

const MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

/** 🪆️The path segment that marks an owner as carrying standards/subsets profile coordinates. */
const PROFILE_MARKER = "/🏅️standards/";

/** 🪆️Whether the first standards marker in an owner contains these exact subset coordinates. */
function ownerContainsProfile(owner: string, standardDirectoryName: string, subsetDirectoryName: string): boolean {
  const markerIndex = owner.indexOf(PROFILE_MARKER);
  if (markerIndex < 0) return false;
  const profile = `${PROFILE_MARKER}${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}`;
  const suffix = owner.slice(markerIndex);
  return suffix === profile || suffix.startsWith(`${profile}/`);
}

/** 🧾️ Validates the strict catalog record without consulting implementation source or runtime kinds. */
export function mutationCatalogProblems(value: unknown, owner?: string): string[] {
  if (!isPlainObject(value)) return ["catalog is not an object"];
  const problems: string[] = [];
  const allowed = new Set(["id", "capability", "standardDirectoryName", "subsetDirectoryName", "kinds", "vectors", "deferredKinds"]);
  for (const key of Object.keys(value)) if (!allowed.has(key)) problems.push(`unknown catalog field ${key}`);
  for (const key of ["id", "capability"] as const) {
    if (typeof value[key] !== "string" || value[key].length === 0) problems.push(`${key} must be a non-empty string`);
    else if (value[key] !== value[key].normalize("NFC")) problems.push(`${key} must be NFC`);
  }
  // 🪆️A catalog's profile coordinates ARE its owner's: an artifact subset, or a nested facet within it,
  // carries `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>` in its path and must restate it here. An owner with no
  // such coordinates — a framework facet whose vocabulary is versioned with the product rather than with a
  // published standard — has none to restate and must not invent one. Requiring them unconditionally is what
  // made those owners' catalogs unrepresentable, and an unrepresentable catalog is an unmeasured vocabulary.
  const profiled = owner === undefined ? value.standardDirectoryName !== undefined || value.subsetDirectoryName !== undefined : owner.includes(PROFILE_MARKER);
  const standardDirectoryName = typeof value.standardDirectoryName === "string" ? value.standardDirectoryName : "";
  const subsetDirectoryName = typeof value.subsetDirectoryName === "string" ? value.subsetDirectoryName : "";
  if (profiled) {
    for (const key of ["standardDirectoryName", "subsetDirectoryName"] as const) {
      if (typeof value[key] !== "string" || value[key].length === 0) problems.push(`${key} must be a non-empty string`);
      else if (value[key] !== value[key].normalize("NFC")) problems.push(`${key} must be NFC`);
    }
    if (!standardDirectoryName.startsWith("🔖️")) problems.push("standardDirectoryName must start with 🔖️");
    if (!subsetDirectoryName.startsWith("✳️")) problems.push("subsetDirectoryName must start with ✳️");
    if (owner !== undefined && !ownerContainsProfile(owner, standardDirectoryName, subsetDirectoryName)) problems.push("catalog profile does not match its contribution owner");
  } else {
    if (value.standardDirectoryName !== undefined) problems.push("standardDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
    if (value.subsetDirectoryName !== undefined) problems.push("subsetDirectoryName is only declarable by an owner that carries standards/subsets coordinates");
  }
  if (!Array.isArray(value.kinds) || value.kinds.length === 0 || value.kinds.some((kind) => typeof kind !== "string" || kind.length === 0)) problems.push("kinds must be a non-empty string array");
  else if (new Set(value.kinds).size !== value.kinds.length) problems.push("kinds must be unique");
  if (value.deferredKinds !== undefined && (!Array.isArray(value.deferredKinds) || value.deferredKinds.some((kind) => typeof kind !== "string" || kind.length === 0) || new Set(value.deferredKinds).size !== value.deferredKinds.length)) problems.push("deferredKinds must be a unique string array");
  if (!Array.isArray(value.vectors)) {
    problems.push("vectors must be an array");
    return problems;
  }
  const mutationIds = new Set<string>();
  const mutationDirectories = new Set<string>();
  for (const [vectorIndex, candidate] of value.vectors.entries()) {
    if (!isPlainObject(candidate)) {
      problems.push(`vectors[${vectorIndex}] is not an object`);
      continue;
    }
    for (const key of Object.keys(candidate)) if (!["mutationId", "sourceMutationDirectoryName", "mutationDirectoryName", "scenarios"].includes(key)) problems.push(`vectors[${vectorIndex}] has unknown field ${key}`);
    const mutationId = typeof candidate.mutationId === "string" ? candidate.mutationId : "";
    const sourceMutationDirectoryName = typeof candidate.sourceMutationDirectoryName === "string" ? candidate.sourceMutationDirectoryName : "";
    const mutationDirectoryName = typeof candidate.mutationDirectoryName === "string" ? candidate.mutationDirectoryName : "";
    if (!MUTATION_ID_RE.test(mutationId)) problems.push(`vectors[${vectorIndex}].mutationId must be kebab-case`);
    if (mutationIds.has(mutationId)) problems.push(`vectors mutationId ${mutationId} is duplicated`);
    mutationIds.add(mutationId);
    if (sourceMutationDirectoryName.length === 0 || sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].sourceMutationDirectoryName must be non-empty NFC`);
    if ((sourceMutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/)?.[0] ?? "") !== mutationId) problems.push(`vectors[${vectorIndex}].sourceMutationDirectoryName does not render mutationId ${mutationId}`);
    if (mutationDirectoryName.length === 0 || mutationDirectoryName !== mutationDirectoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].mutationDirectoryName must be non-empty NFC`);
    if ((mutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/)?.[0] ?? "") !== mutationId) problems.push(`vectors[${vectorIndex}].mutationDirectoryName does not render mutationId ${mutationId}`);
    if (mutationDirectories.has(mutationDirectoryName)) problems.push(`vectors mutationDirectoryName ${mutationDirectoryName} is duplicated`);
    mutationDirectories.add(mutationDirectoryName);
    if (!Array.isArray(candidate.scenarios) || candidate.scenarios.length === 0) {
      problems.push(`vectors[${vectorIndex}].scenarios must be a non-empty array`);
      continue;
    }
    const scenarioIds = new Set<string>();
    for (const [scenarioIndex, scenario] of candidate.scenarios.entries()) {
      if (!isPlainObject(scenario)) {
        problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}] is not an object`);
        continue;
      }
      for (const key of Object.keys(scenario)) if (!["id", "directoryName"].includes(key)) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}] has unknown field ${key}`);
      const id = typeof scenario.id === "string" ? scenario.id : "";
      const directoryName = typeof scenario.directoryName === "string" ? scenario.directoryName : "";
      if (!MUTATION_ID_RE.test(id)) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].id must be kebab-case`);
      if (scenarioIds.has(id)) problems.push(`vectors[${vectorIndex}] scenario id ${id} is duplicated`);
      scenarioIds.add(id);
      if (directoryName !== `🧪️${id}`) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName must equal 🧪️${id}`);
      if (directoryName !== directoryName.normalize("NFC")) problems.push(`vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName must be NFC`);
    }
  }
  return problems;
}

/**
 * 🧾️ Reads an owner's catalogs and REPORTS what is malformed instead of throwing.
 *
 * Throwing here was a blinding failure: `discoverTestContributions` walks the whole repository, so
 * one bad manifest aborted the scan and every other owner's catalogs, oracles, probes and fixtures
 * became invisible — the registry loaded empty and every gate that reads it reported green over
 * nothing. A malformed catalog is a contract BREACH, and a breach must be visible beside all the
 * others, not be the reason none of them can be seen.
 */
function strictMutationCatalogs(value: unknown, owner: string, manifestPath: string, problems: string[]): MutationCatalog[] {
  if (value === undefined) return [];
  if (!Array.isArray(value)) {
    problems.push("mutationCatalogs must be an array");
    return [];
  }
  const accepted: MutationCatalog[] = [];
  for (const [index, catalog] of value.entries()) {
    const found = mutationCatalogProblems(catalog, owner);
    if (found.length > 0) {
      for (const problem of found) problems.push(`mutationCatalogs[${index}] ${problem}`);
      continue;
    }
    accepted.push(catalog as MutationCatalog);
  }
  return accepted;
}

/**
 * 🧩️ A package an owner contributes so its adapters can reach their reference libraries.
 *
 * `path` is what separates the two kinds. WITH a path the entry names LOCAL source linked by path —
 * an in-repo crate, module or workspace package — which is not an external dependency and is
 * provisioned by generating a manifest that points at it. WITHOUT a path the entry names an EXTERNAL
 * distribution the generated host must obtain from that ecosystem's own registry, which is exactly
 * what `externalOracleHostPackages` hands to the dependency classification so a reference library
 * can never reach a host through a side door.
 *
 * `module` is the import name when it differs from the distribution name (`Pillow` → `PIL`);
 * `version` pins the distribution; `features` are the Cargo features of a local crate.
 */
export type OracleHostPackage = Readonly<{ implementation: Implementation; package: string; path?: string; version?: string; module?: string; features?: readonly string[] }>;

/** 🧩️ The name a host imports a contributed package by — the declared module, else the package name. */
export function oracleHostModule(entry: OracleHostPackage): string {
  return entry.module ?? entry.package.replace(/-/g, "_");
}

/**
 * 🧩️ One owner's contribution to the test platform. This is the OPEN half of open/closed: an owner
 * that needs a reference implementation, a native oracle crate or a format-specific comparison
 * profile declares them here, and the framework discovers the manifest without ever naming the
 * owner, the plugin or the format.
 */
export type TestContribution = Readonly<{
  owner: string;
  manifestPath: string;
  oracles: readonly OracleEntry[];
  noOracleDecisions: readonly NoOracleDecision[];
  comparisonProfiles: readonly ComparisonProfileSpec[];
  oracleHostPackages: readonly OracleHostPackage[];
  /** 🦠️ The mutation vocabularies this owner claims exhaustive coverage of. */
  mutationCatalogs: readonly MutationCatalog[];
  /** 🧬️ The authoritative subset-scoped mutation manifests this owner declares. */
  mutationManifests: readonly MutationManifest[];
  /** 🧫️ The provenance-carrying fixture manifests this owner commits. */
  fixtureManifests: readonly FixtureManifest[];
  /** 🔬️ External measurement tools this owner contributes to the comparison pipeline. */
  probes: readonly ProbeEntry[];
  /** ⚖️ Multi-artifact comparison pipelines this owner contributes. */
  comparisonPipelines: readonly ComparisonPipeline[];
  /** 📏️ Scale-relative tolerance profiles this owner contributes. */
  toleranceProfiles: readonly ToleranceProfile[];
  /** 🧾️ What is malformed in this manifest. Reported by the contract phase; never thrown, because a throw during a repository-wide walk hides every other owner. */
  problems: readonly string[];
  /** 🔒️ Where this owner stands on the migration ladder, declared by the owner itself. */
  migrationStatus?: Readonly<Record<string, string>>;
}>;

export type OracleRegistry = Readonly<{
  schemaVersion: number;
  oracles: readonly OracleEntry[];
  probes: readonly ProbeEntry[];
  noOracleDecisions: readonly NoOracleDecision[];
  comparisonProfiles: readonly ComparisonProfileSpec[];
  comparisonPipelines: readonly ComparisonPipeline[];
  toleranceProfiles: readonly ToleranceProfile[];
  oracleHostPackages: readonly OracleHostPackage[];
  mutationCatalogs: readonly MutationCatalog[];
  mutationManifests: readonly MutationManifest[];
  fixtureManifests: readonly FixtureManifest[];
  contributions: readonly TestContribution[];
}>;

function readContribution(repoRoot: string, owner: string, manifestPath: string): TestContribution | null {
  let parsed: Record<string, unknown>;
  try {
    parsed = JSON.parse(readFileSync(join(repoRoot, manifestPath), "utf8")) as Record<string, unknown>;
  } catch {
    return null;
  }
  // 🧫️A fixture manifest's `files[].path` is relative to the CONTRIBUTION, so the directory is stamped
  // in at read time. Resolving it later against the process cwd would silently read the wrong bytes.
  const manifestDir = dirname(manifestPath);
  const problems: string[] = [];
  const mutationCatalogs = strictMutationCatalogs(parsed.mutationCatalogs, owner, manifestPath, problems);
  return {
    owner,
    manifestPath,
    problems,
    oracles: (parsed.oracles as OracleEntry[] | undefined) ?? [],
    probes: (parsed.probes as ProbeEntry[] | undefined) ?? [],
    noOracleDecisions: (parsed.noOracleDecisions as NoOracleDecision[] | undefined) ?? [],
    comparisonProfiles: (parsed.comparisonProfiles as ComparisonProfileSpec[] | undefined) ?? [],
    comparisonPipelines: (parsed.comparisonPipelines as ComparisonPipeline[] | undefined) ?? [],
    toleranceProfiles: (parsed.toleranceProfiles as ToleranceProfile[] | undefined) ?? [],
    oracleHostPackages: (parsed.oracleHostPackages as OracleHostPackage[] | undefined) ?? [],
    mutationCatalogs,
    mutationManifests: (parsed.mutationManifests as MutationManifest[] | undefined) ?? [],
    fixtureManifests: ((parsed.fixtureManifests as FixtureManifest[] | undefined) ?? []).map((fixture) => ({ ...fixture, manifestDir: fixture.manifestDir ?? manifestDir })),
    migrationStatus: (parsed.migrationStatus as Record<string, string> | undefined) ?? {},
  };
}

/**
 * 🧩️ Walks every non-excluded path for `<owner>/🔣️oracle.json`. Discovery is by
 * convention, so a new owner extends the platform by adding a file — never by editing the framework.
 */
export function discoverTestContributions(repoRoot: string): TestContribution[] {
  const cached = contributionCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const found: TestContribution[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== taxonomy.testContributionDirName) return "enter";
    const manifest = join(abs, testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId));
    if (existsSync(manifest)) {
      const owner = relative(repoRoot, dirname(abs)).split(sep).join("/") || ".";
      const contribution = readContribution(repoRoot, owner, relative(repoRoot, manifest).split(sep).join("/"));
      if (contribution !== null) found.push(contribution);
    }
    return "skip";
  });
  found.sort((a, b) => a.owner.localeCompare(b.owner));
  contributionCache.set(repoRoot, found);
  return found;
}

/** 🧩️ Discovery walks the whole repository, and the registry is consulted per case and per role, so
 * the result is memoized for the life of the process. `clearContributionCache` exists for tests that
 * add or remove a manifest mid-run. */
const contributionCache = new Map<string, TestContribution[]>();

/** 🧩️ Forgets the memoized contribution scan. */
export function clearContributionCache(): void {
  contributionCache.clear();
}

/**
 * 📇️ The effective registry: the framework's own core manifest merged with every discovered owner
 * contribution. The framework manifest holds only what is domain-neutral; nothing in this function
 * knows that PDF, PNG or any other format exists.
 */
export function loadOracleRegistry(repoRoot: string): OracleRegistry {
  const core = (() => {
    try {
      const taxonomy = testTaxonomy(repoRoot);
      return JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as Partial<OracleRegistry> & { schemaVersion: number };
    } catch {
      return { schemaVersion: 2 } as Partial<OracleRegistry> & { schemaVersion: number };
    }
  })();
  const contributions = discoverTestContributions(repoRoot);
  return {
    schemaVersion: core.schemaVersion ?? 2,
    oracles: [...(core.oracles ?? []), ...contributions.flatMap((entry) => entry.oracles)],
    probes: [...(core.probes ?? []), ...contributions.flatMap((entry) => entry.probes)],
    noOracleDecisions: [...(core.noOracleDecisions ?? []), ...contributions.flatMap((entry) => entry.noOracleDecisions)],
    comparisonProfiles: [...CORE_COMPARISON_PROFILES, ...(core.comparisonProfiles ?? []), ...contributions.flatMap((entry) => entry.comparisonProfiles)],
    comparisonPipelines: [...(core.comparisonPipelines ?? []), ...contributions.flatMap((entry) => entry.comparisonPipelines)],
    toleranceProfiles: [...(core.toleranceProfiles ?? []), ...contributions.flatMap((entry) => entry.toleranceProfiles)],
    oracleHostPackages: [...(core.oracleHostPackages ?? []), ...contributions.flatMap((entry) => entry.oracleHostPackages)],
    mutationCatalogs: [...(core.mutationCatalogs ?? []), ...contributions.flatMap((entry) => entry.mutationCatalogs)],
    mutationManifests: [...(core.mutationManifests ?? []), ...contributions.flatMap((entry) => entry.mutationManifests)],
    fixtureManifests: [...(core.fixtureManifests ?? []), ...contributions.flatMap((entry) => entry.fixtureManifests)],
    contributions,
  };
}

/** ⚖️ The effective profile table: core profiles plus every contributed one, keyed by id. */
export function profileTable(registry: OracleRegistry): ReadonlyMap<string, ComparisonProfileSpec> {
  return new Map(registry.comparisonProfiles.map((spec) => [spec.id, spec]));
}

/** 🧩️ The native oracle packages one owner's adapters may reach, walking up to the nearest contributor. */
export function oracleHostPackagesFor(registry: OracleRegistry, owner: string, implementation: Implementation): OracleHostPackage[] {
  const contributing = registry.contributions.filter((entry) => owner === entry.owner || owner.startsWith(`${entry.owner}/`));
  return contributing.flatMap((entry) => entry.oracleHostPackages).filter((entry) => entry.implementation === implementation);
}
//#endregion 📇️Registry

//#region 🧫️Fixtures
/** 🧫️ One resolved fixture — explicit scheme, never shadow-based, digest pinned at plan time. */
export type ResolvedFixture = Readonly<{ uri: string; scope: "shared" | "local" | "asset"; name: string; path: string; digest: string }>;

const FIXTURE_URI_RE = /\b(shared|local|asset):\/\/([^\s"'`,;)\]]+)/g;

/** 🧫️ Extracts every `shared://` / `local://` / `asset://` reference appearing anywhere in a feature's text. */
export function fixtureUrisIn(feature: ParsedFeature): string[] {
  const haystack = [feature.description, ...feature.background.flatMap((step) => [step.text, step.docString ?? "", ...(step.dataTable ?? []).flat()]), ...feature.scenarios.flatMap((scenario) => scenario.steps.flatMap((step) => [step.text, step.docString ?? "", ...(step.dataTable ?? []).flat()]))].join("\n");
  const uris = new Set<string>();
  for (const match of haystack.matchAll(FIXTURE_URI_RE)) uris.add(`${match[1]}://${match[2]}`);
  return [...uris].sort();
}

/**
 * 🧫️ Resolves fixture URIs against the owner and case fixture directories. Resolution is explicit:
 * a `local://` name never shadows a `shared://` one, so adding a case-local file can never silently
 * change what an existing scenario reads.
 *
 * `asset://` resolves against the OWNER ROOT rather than a fixture directory. Real-world artifacts
 * are already committed where the domain keeps them (examples, assets), and they are large; copying
 * a multi-megabyte document into a fixtures directory would duplicate history for no gain. The path
 * escape guard and the plan-time digest pin are identical for all three schemes.
 */
export function resolveFixtures(repoRoot: string, discovered: DiscoveredCase, uris: readonly string[]): { fixtures: ResolvedFixture[]; missing: string[] } {
  const fixtures: ResolvedFixture[] = [];
  const missing: string[] = [];
  for (const uri of uris) {
    const [scheme, name] = uri.split("://") as ["shared" | "local" | "asset", string];
    const baseRel = scheme === "shared" ? discovered.sharedFixtureDir : scheme === "asset" ? discovered.owner : discovered.localFixtureDir;
    if (baseRel === null) {
      missing.push(uri);
      continue;
    }
    const abs = join(repoRoot, baseRel, name);
    const guard = resolve(join(repoRoot, baseRel));
    if (!resolve(abs).startsWith(guard + sep) || !existsSync(abs)) {
      missing.push(uri);
      continue;
    }
    fixtures.push({ uri, scope: scheme, name, path: `${baseRel}/${name}`, digest: fileDigest(abs) });
  }
  return { fixtures, missing };
}

/** 🧫️ Every file under a fixture directory, repo-relative, for orphan detection and immutability proofs. */
export function fixtureFilesUnder(repoRoot: string, relDir: string | null): string[] {
  if (relDir === null) return [];
  const abs = join(repoRoot, relDir);
  if (!existsSync(abs)) return [];
  const out: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else out.push(relative(repoRoot, full).split(sep).join("/"));
    }
  };
  walk(abs);
  return out.sort();
}
//#endregion 🧫️Fixtures

//#region 📋️Plan
/** 📋️ The owned execution plan one native host receives — hosts never re-parse the feature. */
export type TestCasePlan = Readonly<{
  schemaVersion: 2;
  baselineSha?: string;
  owner: string;
  case: string;
  featurePath: string;
  featureHash: string;
  featureName: string;
  description: string;
  capability: string;
  /** 🪆️ The smallest owning subset this case is scoped to. A case that mutates an artifact without one is unscoped, which v2 forbids. */
  target: SubsetTarget | null;
  /** #⃣ Digest of the owning mutation manifest, so a manifest edit invalidates every cached result of this case. */
  mutationManifestDigest: string | null;
  oracle: string | null;
  noOracleDecision: string | null;
  comparison: ComparisonProfile;
  /** 📥️ The subject artifact an oracle consumes when the feature declares an external byte decoder. */
  oracleInput: "subject-raw" | null;
  /** 📦️ Raw outputs produced by subject hosts before this oracle host starts. */
  subjectRawInputs?: Readonly<Partial<Record<Implementation, string>>>;
  /** ⚖️ The multi-artifact, externally-probed pipeline this case compares under, when it produces more than a projection. */
  comparisonPipeline: string | null;
  toleranceProfile: string | null;
  background: readonly FeatureStep[];
  scenarios: readonly FeatureScenario[];
  adapters: Readonly<Partial<Record<Implementation, string>>>;
  fixtures: readonly ResolvedFixture[];
  /** 🧫️ The full provenance record of every fixture this case reads — hosts never re-derive it. */
  fixtureManifests: readonly FixtureManifest[];
  workDir: string;
  resultsPath: string;
  outputDir: string;
  /** 📦️ Where a host writes its produced artifact bundle. Separate from `workDir` so a mutable scratch copy is never mistaken for a result. */
  artifactDir: string;
  level: TestLevel;
  role: TestRole;
  implementation: Implementation;
  platform: PlatformId;
}>;

/** 🎚️ Levels at or below `level` — running a level runs every lower level, cumulatively. */
export function levelsUpTo(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(0, TEST_LEVELS.indexOf(level) + 1);
}

/** 📋️ Builds the plan skeleton shared by every role/implementation execution of one case. */
export function buildCasePlan(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, registry: OracleRegistry = loadOracleRegistry(repoRoot)): { plan: Omit<TestCasePlan, "role" | "implementation" | "workDir" | "resultsPath" | "outputDir" | "artifactDir">; feature: ParsedFeature; missingFixtures: string[] } {
  const source = readFileSync(join(repoRoot, discovered.featurePath), "utf8");
  const feature = parseFeature(source);
  const { fixtures, missing } = resolveFixtures(repoRoot, discovered, fixtureUrisIn(feature));
  const selectable = new Set(levelsUpTo(level));
  // 🪆️A case is scoped by the manifest whose owner it lives under, walking up to the nearest one. A
  // case with no owning manifest gets a null target rather than a guessed artifact-wide one — that
  // absence is what the contract phase reports, and inventing a scope here would hide it.
  const owning = registry.contributions
    .filter((entry) => discovered.owner === entry.owner || discovered.owner.startsWith(`${entry.owner}/`) || entry.owner.startsWith(`${discovered.owner}/`))
    .flatMap((entry) => entry.mutationManifests)
    .find((manifest) => feature.capability === null || manifest.mutations.some((mutation) => mutation.capability === feature.capability));
  const profile = profileTable(registry).get(feature.comparison ?? "");
  const pipeline = profile?.pipeline ?? null;
  const toleranceProfile = pipeline === null ? null : (pipelineTable(registry).get(pipeline)?.toleranceProfile ?? null);
  const fixtureManifests = registry.contributions
    .filter((entry) => discovered.owner === entry.owner || discovered.owner.startsWith(`${entry.owner}/`) || entry.owner.startsWith(`${discovered.owner}/`))
    .flatMap((entry) => entry.fixtureManifests);
  return {
    feature,
    missingFixtures: missing,
    plan: {
      schemaVersion: 2,
      baselineSha: process.env.SEMIO_BASELINE_SHA,
      owner: discovered.owner,
      case: discovered.case,
      featurePath: discovered.featurePath,
      featureHash: digest(source),
      featureName: feature.name,
      description: feature.description,
      capability: feature.capability ?? "",
      target: owning === undefined ? null : { artifact: owning.artifact, standard: owning.standard, subset: owning.subset },
      mutationManifestDigest: owning === undefined ? null : mutationManifestDigest(owning),
      oracle: feature.oracle,
      noOracleDecision: feature.noOracleDecision,
      comparison: feature.comparison ?? "ordered-json-v1",
      oracleInput: feature.oracleInput,
      comparisonPipeline: pipeline,
      toleranceProfile,
      background: feature.background,
      scenarios: feature.scenarios.filter((scenario) => selectable.has(scenario.level)),
      adapters: discovered.adapters,
      fixtures,
      fixtureManifests,
      platform: currentPlatform(),
      level,
    },
  };
}

/** 🆔️ The stable identifier every executable result carries. Display names may change; this may not. */
export function testId(owner: string, caseSlug: string, scenario: string, implementation: Implementation, role: TestRole): string {
  return `${owner}::${caseSlug}::${scenario}::${implementation}::${role}`;
}
//#endregion 📋️Plan

//#region ⚡️Cache
/** ⚡️ Root of every generated test artifact. Nothing outside this tree is ever written or deleted. */
export function testCacheRoot(repoRoot: string): string {
  return join(getRepoMetaDir(repoRoot), "⚡️cache", testTaxonomy(repoRoot).testOutputCacheDirName);
}

/** ⚡️ One of the six generated output roots (`work`, `hosts`, `oracles`, `results`, `diffs`, `reports`). */
export function testCacheDir(repoRoot: string, child: string): string {
  const taxonomy = testTaxonomy(repoRoot);
  if (!taxonomy.testOutputChildDirs.includes(child)) throw new Error(`unknown test cache child dir ${JSON.stringify(child)}`);
  return join(testCacheRoot(repoRoot), child);
}

/** 🏷️ Agent-scoped build/output root so concurrent sessions never contend on one target directory. */
export function agentCacheRoot(repoRoot: string, agentId = process.env.SEMIO_AGENT_ID ?? "local"): string {
  return join(getRepoMetaDir(repoRoot), "⚡️cache", "agents", agentId.replace(/[^A-Za-z0-9._-]+/g, "_"));
}

/** 🧾️ Ownership marker written into every generated output root; only marked trees are deletable. */
export type OutputMarker = Readonly<{ kind: string; testId: string; cacheKey: string }>;

/** 🧾️ Creates a generated directory and stamps it with its ownership marker. */
export function markOutputDir(repoRoot: string, absDir: string, marker: Omit<OutputMarker, "kind">): void {
  const taxonomy = testTaxonomy(repoRoot);
  const root = resolve(testCacheRoot(repoRoot));
  if (!resolve(absDir).startsWith(root + sep) && resolve(absDir) !== root) throw new Error(`refusing to mark ${absDir}: outside the test cache root`);
  mkdirSync(absDir, { recursive: true });
  writeFileSync(join(absDir, testFilenameForKind(taxonomy, taxonomy.testOutputMarkerFileKindId)), `${JSON.stringify({ kind: taxonomy.testOutputMarkerKind, ...marker }, null, 2)}\n`);
}

/** 🧾️ Reads a directory's ownership marker, or `null` when it carries none. */
export function readOutputMarker(repoRoot: string, absDir: string): OutputMarker | null {
  const taxonomy = testTaxonomy(repoRoot);
  const path = join(absDir, testFilenameForKind(taxonomy, taxonomy.testOutputMarkerFileKindId));
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as OutputMarker;
    return parsed.kind === taxonomy.testOutputMarkerKind ? parsed : null;
  } catch {
    return null;
  }
}
//#endregion ⚡️Cache

//#region ⚖️Comparison
/** ⚖️ One field-level difference between an oracle projection and a subject projection. */
export type ComparisonDiff = Readonly<{ path: string; oracle: unknown; subject: unknown; reason: string }>;

/** ⚖️ The verdict of one comparison profile over two projections. */
export type ComparisonVerdict = Readonly<{ profile: ComparisonProfile; equal: boolean; diffs: readonly ComparisonDiff[] }>;

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** ⚖️ Sorts object keys recursively so serialization order is never mistaken for a semantic difference. */
export function canonicalize(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  return value;
}

function sortDeep(value: unknown): unknown {
  if (Array.isArray(value)) return [...value.map(sortDeep)].sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)));
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, sortDeep(value[key])]),
    );
  return value;
}

function stripKeys(value: unknown, keys: ReadonlySet<string>): unknown {
  if (keys.size === 0) return value;
  if (Array.isArray(value)) return value.map((entry) => stripKeys(entry, keys));
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.entries(value)
        .filter(([key]) => !keys.has(key))
        .map(([key, entry]) => [key, stripKeys(entry, keys)]),
    );
  return value;
}

function roundFloats(value: unknown, tolerance: number): unknown {
  if (tolerance <= 0) return value;
  if (typeof value === "number") return Math.round(value / tolerance) * tolerance;
  if (Array.isArray(value)) return value.map((entry) => roundFloats(entry, tolerance));
  if (isPlainObject(value)) return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, roundFloats(entry, tolerance)]));
  return value;
}

function normalizeText(value: unknown): string {
  return String(value)
    .replace(/\r\n/g, "\n")
    .replace(/[ \t]+$/gm, "")
    .replace(/\n+$/, "")
    .normalize("NFC");
}

function diffValues(path: string, oracle: unknown, subject: unknown, out: ComparisonDiff[], tolerance: number): void {
  if (typeof oracle === "number" && typeof subject === "number") {
    if (Math.abs(oracle - subject) > tolerance) out.push({ path, oracle, subject, reason: `numbers differ by more than ${tolerance}` });
    return;
  }
  if (Array.isArray(oracle) && Array.isArray(subject)) {
    if (oracle.length !== subject.length) out.push({ path, oracle: oracle.length, subject: subject.length, reason: "array length differs" });
    for (let i = 0; i < Math.max(oracle.length, subject.length); i += 1) diffValues(`${path}[${i}]`, oracle[i], subject[i], out, tolerance);
    return;
  }
  if (isPlainObject(oracle) && isPlainObject(subject)) {
    for (const key of new Set([...Object.keys(oracle), ...Object.keys(subject)])) {
      if (!(key in oracle)) out.push({ path: `${path}.${key}`, oracle: undefined, subject: subject[key], reason: "present only in subject" });
      else if (!(key in subject)) out.push({ path: `${path}.${key}`, oracle: oracle[key], subject: undefined, reason: "present only in oracle" });
      else diffValues(`${path}.${key}`, oracle[key], subject[key], out, tolerance);
    }
    return;
  }
  if (JSON.stringify(oracle) !== JSON.stringify(subject)) out.push({ path, oracle, subject, reason: "values differ" });
}

/** ⚖️ Applies a profile's declared policy to one projection, producing its canonical form. */
function projectUnder(spec: ComparisonProfileSpec, value: unknown): unknown {
  const stripped = stripKeys(value, new Set(spec.ignoreKeys ?? []));
  const rounded = roundFloats(stripped, spec.tolerance ?? 0);
  return spec.arrays === "set" ? sortDeep(rounded) : canonicalize(rounded);
}

/**
 * ⚖️ Applies one comparison profile. The MECHANISM lives here; the per-format policy is data, so a
 * new format needs a manifest entry rather than a new branch. Comparison never lives in an adapter,
 * so two implementations cannot quietly agree on different notions of "the same result".
 */
export function compareProjections(profile: ComparisonProfile, oracle: unknown, subject: unknown, profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): ComparisonVerdict {
  const spec = profiles.get(profile);
  if (spec === undefined) return { profile, equal: false, diffs: [{ path: "$", oracle: profile, subject: [...profiles.keys()].sort(), reason: `unknown comparison profile ${JSON.stringify(profile)} — register it in an owner's ${"🧪️oracle"} manifest` }] };
  const diffs: ComparisonDiff[] = [];
  if (spec.bytes === true) {
    const left = typeof oracle === "string" ? oracle : JSON.stringify(oracle);
    const right = typeof subject === "string" ? subject : JSON.stringify(subject);
    if (left !== right) diffs.push({ path: "$", oracle: digest(left), subject: digest(right), reason: "byte-exact comparison failed" });
    return { profile, equal: diffs.length === 0, diffs };
  }
  if (spec.text === "utf8") {
    if (normalizeText(oracle) !== normalizeText(subject)) diffs.push({ path: "$", oracle: normalizeText(oracle), subject: normalizeText(subject), reason: "normalized text differs" });
    return { profile, equal: diffs.length === 0, diffs };
  }
  diffValues("$", projectUnder(spec, oracle), projectUnder(spec, subject), diffs, spec.tolerance ?? 0);
  return { profile, equal: diffs.length === 0, diffs };
}

/** ⚖️ Digest of a projection under a profile's canonical form — the value results carry as `projectionHash`. */
export function projectionHash(profile: ComparisonProfile, projection: unknown, profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): string {
  const spec = profiles.get(profile);
  if (spec === undefined) return digest(JSON.stringify(canonicalize(projection)));
  if (spec.bytes === true) return digest(typeof projection === "string" ? projection : JSON.stringify(projection));
  if (spec.text === "utf8") return digest(normalizeText(projection));
  return digest(JSON.stringify(projectUnder(spec, projection)));
}

let coreProfileTableCache: ReadonlyMap<string, ComparisonProfileSpec> | null = null;

/** ⚖️ The framework's own domain-neutral profile table, for callers with no contributions loaded. */
export function coreProfileTable(): ReadonlyMap<string, ComparisonProfileSpec> {
  coreProfileTableCache ??= new Map(CORE_COMPARISON_PROFILES.map((spec) => [spec.id, spec]));
  return coreProfileTableCache;
}
//#endregion ⚖️Comparison

//#region 📤️Results
/** 📤️ One executed `(scenario, implementation, role)` — the single shape every native host emits. */
export type TestResult = Readonly<{
  schemaVersion?: 2;
  testId: string;
  baselineSha?: string;
  owner: string;
  case: string;
  scenario: string;
  implementation: Implementation;
  role: TestRole;
  level: TestLevel;
  platform?: PlatformId;
  status: "passed" | "failed" | "errored";
  durationMs: number;
  seed?: string;
  featureHash?: string;
  fixtureHash?: string;
  sourceHash?: string;
  dependencyFingerprint?: string;
  runKey?: string;
  mutation?: string;
  outcome?: MutationOutcomeClass;
  /** 🏭️ Proof this execution reached PRODUCTION dispatch. A subject result without it may never count toward production-bridge coverage — replaying a committed vector is exactly what it exists to distinguish. */
  productionDispatch?: { invoked: true; operation: string; bridgeVersion: number };
  /** 📦️ Every file this execution produced, addressed by ROLE so a comparison stage never names a path. */
  artifacts?: readonly ResultArtifact[];
  output: { rawHash: string; projectionHash: string; rawPath?: string; projectionPath?: string; projection?: unknown };
  diagnostics: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
}>;

/** 📤️ Validates one host-emitted record against the owned result schema, returning its problems. */
export function validateResult(value: unknown): string[] {
  const problems: string[] = [];
  if (!isPlainObject(value)) return ["result is not an object"];
  for (const key of ["testId", "owner", "case", "scenario", "implementation", "role", "level", "status", "durationMs", "output"]) if (value[key] === undefined) problems.push(`missing field ${key}`);
  if (value.implementation !== undefined && !(IMPLEMENTATIONS as readonly string[]).includes(String(value.implementation))) problems.push(`unknown implementation ${String(value.implementation)}`);
  if (value.role !== undefined && !(TEST_ROLES as readonly string[]).includes(String(value.role))) problems.push(`unknown role ${String(value.role)}`);
  if (value.level !== undefined && !(TEST_LEVELS as readonly string[]).includes(String(value.level))) problems.push(`unknown level ${String(value.level)}`);
  if (value.status !== undefined && !["passed", "failed", "errored"].includes(String(value.status))) problems.push(`unknown status ${String(value.status)}`);
  if (value.output !== undefined && !isPlainObject(value.output)) problems.push("output is not an object");
  return problems;
}

/** 📤️ Reads a host's JSONL result stream, rejecting malformed records loudly rather than skipping them. */
export function readResults(absPath: string): { results: TestResult[]; problems: string[] } {
  if (!existsSync(absPath)) return { results: [], problems: [`no result stream at ${absPath}`] };
  const results: TestResult[] = [];
  const problems: string[] = [];
  for (const [index, line] of readFileSync(absPath, "utf8").split(/\r?\n/).entries()) {
    if (line.trim() === "") continue;
    let parsed: unknown;
    try {
      parsed = JSON.parse(line);
    } catch (error) {
      problems.push(`line ${index + 1}: not JSON (${(error as Error).message})`);
      continue;
    }
    const issues = validateResult(parsed);
    if (issues.length > 0) problems.push(`line ${index + 1}: ${issues.join("; ")}`);
    else results.push(parsed as TestResult);
  }
  return { results, problems };
}
//#endregion 📤️Results

//#region 🧾️Contract
/** 🧾️ Breach families this domain owns. Every finding carries one of these `kind` values. */
export const TESTING_BREACH_KINDS = ["testing/taxonomy", "testing/contract", "testing/fixture", "testing/oracle", "testing/dependency", "testing/discovery"] as const;
export type TestingBreachKind = (typeof TESTING_BREACH_KINDS)[number];

function breach(kind: TestingBreachKind, id: string, scope: string, summary: string, reason: string, solution: string, priority: BreachRecord["priority"] = "high"): BreachRecord {
  return { id, kind, scope, summary, reason, solution, priority };
}

const SOURCE_VECTOR_DIRECTORIES = ["🦠️mutation", "📸️snapshot", "📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🔺️diff", "🎯️outcome"] as const;
const PROJECTED_VECTOR_DIRECTORIES = SOURCE_VECTOR_DIRECTORIES;
const SOURCE_VECTOR_FILES = ["🦀️.rs", "🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🎯️outcome/🔣️.json"] as const;
const PROJECTED_VECTOR_FILES = ["🦀️.rs", "🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🎯️outcome/🔣️.json"] as const;

function childDirectories(abs: string): string[] {
  if (!existsSync(abs)) return [];
  return readdirSync(abs, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink()).map((entry) => entry.name).sort();
}

function vectorBundleNodes(abs: string): string[] {
  if (!existsSync(abs) || !lstatSync(abs).isDirectory()) return [];
  const nodes: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      const rel = relative(abs, full).split(sep).join("/");
      if (entry.isDirectory() && !entry.isSymbolicLink()) {
        nodes.push(`${rel}/`);
        walk(full);
      } else nodes.push(entry.isSymbolicLink() ? `${rel}@symlink` : rel);
    }
  };
  walk(abs);
  return nodes.sort();
}

function expectedVectorBundle(abs: string, state: "source" | "projected"): string[] | null {
  const directories = state === "source" ? SOURCE_VECTOR_DIRECTORIES : PROJECTED_VECTOR_DIRECTORIES;
  const files = state === "source" ? SOURCE_VECTOR_FILES : PROJECTED_VECTOR_FILES;
  const jsonDiff = state === "source" ? "🔺️diff/🔣️.json" : "🔺️diff/🔣️.json";
  const absentDiff = state === "source" ? "🔺️diff/🚫️.absent" : "🔺️diff/🚫️.absent";
  const variants = [jsonDiff, absentDiff].filter((rel) => existsSync(join(abs, rel)) && lstatSync(join(abs, rel)).isFile());
  if (variants.length !== 1) return null;
  return [...directories.map((rel) => `${rel}/`), ...files, variants[0]].sort();
}

function bundleBreach(scope: string, state: "source" | "projected"): BreachRecord | null {
  const expected = expectedVectorBundle(scope, state);
  const actual = vectorBundleNodes(scope);
  if (expected !== null && JSON.stringify(actual) === JSON.stringify(expected)) return null;
  return breach("testing/contract", "mutation-vector-bundle-invalid", scope, `Physical mutation vector is not the exact ${state} 13-node bundle`, "Projection is safe only when every source or projected scenario has the closed fixture shape and exactly one diff alternative.", "Restore the registered Rust, mutation, snapshot, diff and outcome leaves with no extra children or symlinks.");
}

/**
 * 🧬️ Audits every registered physical vector against either its complete source bundle or complete
 * projected bundle. Runtime mutation kinds are deliberately not consulted: they describe dispatch
 * capability, while vectors describe checked-in physical evidence.
 */
export function mutationVectorRegistryBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    // 🧾️The represented sets are OWNER-scoped, not catalog-scoped. An owner may declare several
    // catalogs over ONE `🧬️mutations` tree — `🎚️config` declares three, for opening, merge-policy and
    // identity — and a per-catalog sweep then reported every OTHER catalog's vectors as unregistered.
    // Five real scenarios became ten spurious findings that way: the evidence was registered, just not
    // by the catalog that happened to be walking.
    const representedSource = new Set<string>();
    const representedProjected = new Set<string>();
    const sweepRoots: { sourceMutationRoot: string; projectedProfileRoot: string }[] = [];
    for (const catalog of contribution.mutationCatalogs) {
      const profileProblems = mutationCatalogProblems(catalog, contribution.owner);
      for (const problem of profileProblems) breaches.push(breach("testing/contract", "mutation-vector-catalog-invalid", contribution.manifestPath, problem, "The physical vector registry is a strict owner-scoped contract.", "Correct the catalog record without aliases or optional legacy fields."));
      if (profileProblems.length > 0) continue;
      const markerIndex = contribution.owner.indexOf(PROFILE_MARKER);
      const sourceMutationRoot = join(repoRoot, contribution.owner, "🧬️schema", "🧬️mutations");
      // 🪆️A profile-less owner has no projected profile root to walk — its vectors live only in source
      // state — so the projected half of the comparison is an empty set rather than a guessed path.
      const projectedProfileRoot =
        markerIndex < 0
          ? join(repoRoot, contribution.owner, "🧪️tests", "🪆️")
          : join(repoRoot, contribution.owner.slice(0, markerIndex), "🧪️tests", `🪆️${(catalog.standardDirectoryName ?? "").slice("🔖️".length)}-${(catalog.subsetDirectoryName ?? "").slice("✳️".length)}`);
      sweepRoots.push({ sourceMutationRoot, projectedProfileRoot });

      for (const vector of catalog.vectors) {
        const sourceTests = join(sourceMutationRoot, vector.sourceMutationDirectoryName, "🧪️tests");
        const projectedMutation = join(projectedProfileRoot, vector.mutationDirectoryName);
        const sourceScenarios = childDirectories(sourceTests);
        const projectedScenarios = childDirectories(projectedMutation);
        for (const scenario of vector.scenarios) {
          const sourceKey = `${vector.sourceMutationDirectoryName}/${scenario.id}`;
          const projectedKey = `${vector.mutationDirectoryName}/${scenario.directoryName}`;
          const sourceAbs = join(sourceTests, scenario.id);
          const projectedAbs = join(projectedMutation, scenario.directoryName);
          const sourceExists = sourceScenarios.includes(scenario.id);
          const projectedExists = projectedScenarios.includes(scenario.directoryName);
          if (sourceExists && projectedExists) {
            representedSource.add(sourceKey);
            representedProjected.add(projectedKey);
            breaches.push(breach("testing/contract", "mutation-vector-mixed-state", contribution.manifestPath, `Vector ${sourceKey} exists in both source and projected storage`, "A projection is transactional; duplicate physical ownership makes references and rollback ambiguous.", "Complete or roll back the projection transaction."));
            continue;
          }
          if (sourceExists) {
            representedSource.add(sourceKey);
            const finding = bundleBreach(sourceAbs, "source");
            if (finding) breaches.push(finding);
            continue;
          }
          if (projectedExists) {
            representedProjected.add(projectedKey);
            const finding = bundleBreach(projectedAbs, "projected");
            if (finding) breaches.push(finding);
            continue;
          }
          if (vector.scenarios.length === 1 && sourceScenarios.length === 1 && projectedScenarios.length === 0) {
            const actual = sourceScenarios[0];
            representedSource.add(`${vector.sourceMutationDirectoryName}/${actual}`);
            breaches.push(breach("testing/contract", "mutation-vector-source-id-mismatch", `${contribution.owner}/🧬️schema/🧬️mutations/${vector.sourceMutationDirectoryName}/🧪️tests/${actual}`, `Source scenario ${actual} must be transactionally renamed to canonical id ${scenario.id}`, "The catalog stores the canonical post-projection identity; changing only the catalog would leave physical storage and its references stale.", "Apply the registered path projection and structured reference edits in one transaction."));
            const finding = bundleBreach(join(sourceTests, actual), "source");
            if (finding) breaches.push(finding);
            continue;
          }
          breaches.push(breach("testing/contract", "mutation-vector-missing", contribution.manifestPath, `Registered vector ${sourceKey} has neither a source nor projected bundle`, "Every registered physical identity must resolve exactly once.", "Restore the source bundle or complete the projection transaction."));
        }
      }

    }

    // 🧾️One sweep per owner, after every catalog has contributed what it registers. Each physical root
    // is visited once however many catalogs share it, so a scenario is reported as unregistered only
    // when NO catalog of this owner claims it.
    for (const root of new Map(sweepRoots.map((entry) => [entry.sourceMutationRoot, entry])).values()) {
      for (const mutationDirectoryName of childDirectories(root.sourceMutationRoot)) {
        const tests = join(root.sourceMutationRoot, mutationDirectoryName, "🧪️tests");
        for (const scenario of childDirectories(tests)) {
          const key = `${mutationDirectoryName}/${scenario}`;
          if (!representedSource.has(key)) breaches.push(breach("testing/contract", "mutation-vector-unregistered", relative(repoRoot, join(tests, scenario)).split(sep).join("/"), `Physical source vector ${key} is not registered`, "Unregistered physical evidence cannot be projected or verified deterministically.", "Add its exact mutation and canonical scenario identity to vectors."));
        }
      }
    }
    for (const root of new Map(sweepRoots.map((entry) => [entry.projectedProfileRoot, entry])).values()) {
      for (const mutationDirectoryName of childDirectories(root.projectedProfileRoot)) {
        for (const scenario of childDirectories(join(root.projectedProfileRoot, mutationDirectoryName))) {
          const key = `${mutationDirectoryName}/${scenario}`;
          if (!representedProjected.has(key)) breaches.push(breach("testing/contract", "mutation-vector-unregistered", relative(repoRoot, join(root.projectedProfileRoot, key)).split(sep).join("/"), `Physical projected vector ${key} is not registered`, "Unregistered physical evidence cannot be verified or rolled back deterministically.", "Add its exact mutation and canonical scenario identity to vectors."));
        }
      }
    }
  }
  return breaches;
}

/**
 * 🦠️ The completeness gate. A feature that tags `@mutations-<catalog>` claims to exercise the WHOLE
 * declared mutation vocabulary of an artifact; this turns that claim into arithmetic. Every kind the
 * owner declares must appear as both a `mutate-<kind>` scenario (the mutation actually applied and
 * compared against the reference implementation) and an `inverse-<kind>` scenario (the algebraic law
 * that the mutation is undoable), or the case is not exhaustive and says so.
 *
 * The framework reads a DECLARED list, never implementation source: which language the artifact is
 * written in, and whether its vocabulary is an enum, a descriptor table or a directory of leaves, is
 * the owner's business. The owner proves the declaration matches its own code with its own test.
 */
export function mutationCoverageBreaches(discovered: DiscoveredCase, feature: ParsedFeature, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  if (feature.mutationCatalog === null) return breaches;
  const catalog = registry.mutationCatalogs.find((entry) => entry.id === feature.mutationCatalog);
  if (catalog === undefined) {
    breaches.push(breach("testing/contract", "unknown-mutation-catalog", discovered.featurePath, `Unknown mutation catalog @mutations-${feature.mutationCatalog}`, "A catalog is the declared vocabulary a feature is measured against; claiming one that is not declared measures the feature against nothing.", `Declare the catalog in the owner's ${"🧪️oracle"} contribution manifest.`));
    return breaches;
  }
  if (catalog.capability !== "" && feature.capability !== null && catalog.capability !== feature.capability) {
    breaches.push(breach("testing/contract", "mutation-catalog-capability-mismatch", discovered.featurePath, `Catalog ${catalog.id} declares capability ${catalog.capability} but the feature declares ${feature.capability}`, "A vocabulary belongs to the capability it mutates; pointing a feature at another capability's catalog would report coverage of behaviour it never exercises.", "Align the feature's @capability- tag with the catalog, or claim the matching catalog."));
  }
  const ids = new Set(feature.scenarios.map((scenario) => scenario.id));
  const missingMutate = catalog.kinds.filter((kind) => !ids.has(`mutate-${kind}`));
  const missingInverse = catalog.kinds.filter((kind) => !ids.has(`inverse-${kind}`));
  if (missingMutate.length > 0) {
    breaches.push(breach("testing/contract", "mutation-kind-uncovered", discovered.featurePath, `${missingMutate.length} of ${catalog.kinds.length} mutation kind(s) in catalog ${catalog.id} have no mutate scenario: ${missingMutate.join(", ")}`, "Exhaustive coverage is the claim this feature makes by tagging the catalog. An unexercised kind is a mutation the implementation may corrupt with nothing to notice.", `Add a row to the Examples table for each kind so the scenario id ${"mutate-<kind>"} exists.`));
  }
  if (missingInverse.length > 0) {
    breaches.push(breach("testing/contract", "mutation-inverse-uncovered", discovered.featurePath, `${missingInverse.length} mutation kind(s) in catalog ${catalog.id} have no inverse scenario: ${missingInverse.join(", ")}`, "A mutation that cannot be undone breaks the undo history the whole event-sourced runtime rests on, and the failure only ever shows up in a user's session.", `Add the kind to the inverse-law Examples table so the scenario id ${"inverse-<kind>"} exists.`));
  }
  const stray = [...ids].filter((id) => id.startsWith("mutate-") && !catalog.kinds.includes(id.slice("mutate-".length)));
  if (stray.length > 0) {
    breaches.push(breach("testing/contract", "mutation-kind-undeclared", discovered.featurePath, `Scenario(s) ${stray.join(", ")} exercise mutation kinds the catalog does not declare`, "The catalog is what the completeness gate counts against; a kind exercised but not declared means the declared vocabulary is out of date and the gate is measuring the wrong set.", "Add the kind to the catalog, or rename the scenario if it is not a mutation case."));
  }
  if ((catalog.deferredKinds ?? []).length > 0) {
    breaches.push(breach("testing/contract", "mutation-kinds-deferred", discovered.featurePath, `Catalog ${catalog.id} defers ${(catalog.deferredKinds ?? []).length} kind(s): ${(catalog.deferredKinds ?? []).join(", ")}`, "A deferred kind is untested surface that no longer shows up as missing. Recording it keeps the debt visible instead of letting the gate report green over it.", "Cover the kinds and empty deferredKinds, or state why they cannot exist.", "medium"));
  }
  return breaches;
}

/**
 * 🪆️ Law 1's placement half, made mechanical. `discoverTestCases` takes a case's PARENT as its
 * owner, so a case sitting one level above a subset directory is discovered and measured against
 * that wrong owner — its `sharedFixtureDir` resolves to the ARTIFACT's fixtures, not the subset's,
 * silently defeating the very placement this ticket spent 642 `unsplit-artifact-subset` closures on.
 *
 * The one signal this trusts is the catalog's own `subsetDirectoryName` — `mutationCatalogProblems`
 * already REQUIRES and VALIDATES it (`catalog profile does not match its contribution owner`)
 * whenever the catalog's own contribution carries standards/subsets coordinates, so a bare
 * `@mutations-<id>` tag resolves to exactly one real, already-audited subset id; nothing here
 * re-derives it from an adapter import or a fixture URI, both of which name a real subset for a pure
 * container/round-trip case too (confirmed live: `gif/create-and-round-trip-gif`,
 * `jpg/create-and-read-jpeg` and `zip/create-and-edit-archive` each import exactly one subset's `io`
 * while genuinely spanning or exceeding it) and would misfire on precisely the three cases C4 confirmed
 * must stay put. A catalog with no `subsetDirectoryName` — unprofiled, not a standards/subsets
 * vocabulary — yields no verdict rather than a guess, and a feature with no `@mutations-` tag at all
 * is never a candidate, mirroring the same early return `mutationCoverageBreaches` itself takes.
 */
export function caseAboveSubsetBreaches(discovered: DiscoveredCase, feature: ParsedFeature, registry: OracleRegistry): BreachRecord[] {
  if (feature.mutationCatalog === null) return [];
  const catalog = registry.mutationCatalogs.find((entry) => entry.id === feature.mutationCatalog);
  if (catalog?.subsetDirectoryName === undefined || catalog.subsetDirectoryName.length === 0) return [];
  if (subsetCoordinatesOfOwner(discovered.owner) !== null) return [];
  const subset = catalog.subsetDirectoryName.slice("✳️".length);
  return [
    breach(
      "testing/taxonomy",
      "case-above-subset",
      discovered.caseDir,
      `Case claims @mutations-${feature.mutationCatalog}, owned by exactly one subset (${subset}), but sits at ${discovered.owner}, above every subset`,
      "A mutation is owned by the smallest semantic subset. A case sitting above that subset is discovered with the ARTIFACT as its owner, so its sharedFixtureDir resolves against the artifact's fixtures rather than the subset's, and every reader of the tree sees an artifact-wide case for evidence that actually proves one narrow scope.",
      `Move 🧪️tests/${discovered.case} under the ${catalog.subsetDirectoryName} subset's own 🧪️tests, beside catalog ${catalog.id}.`,
    ),
  ];
}

/**
 * 🧾️ The contract phase: everything checkable without executing a single test. A case that fails
 * here can never be reported as passing, because the plan the hosts would receive is not well formed.
 */
export function validateCaseContract(repoRoot: string, discovered: DiscoveredCase, registry: OracleRegistry): BreachRecord[] {
  const taxonomy = testTaxonomy(repoRoot);
  const breaches: BreachRecord[] = [];
  const scope = discovered.caseDir;

  if (!new RegExp(taxonomy.testCaseSlugPattern).test(discovered.case)) {
    breaches.push(breach("testing/taxonomy", "case-slug", scope, `Test case directory ${JSON.stringify(discovered.case)} is not kebab-case`, `Case names must match ${taxonomy.testCaseSlugPattern} so a stable identifier can be derived from the path.`, "Rename the directory to kebab-case."));
  }
  if (discovered.owner.split("/").includes("📦️packages")) {
    breaches.push(breach("testing/taxonomy", "case-in-language-package", scope, "Test case lives under 📦️packages — the feature contract would become language-owned", "A behaviour is owned by the nearest language-neutral domain owner, never by one implementation's package.", `Move ${taxonomy.testsDirName}/${discovered.case} up to the owning domain root.`));
  }
  if (Object.keys(discovered.adapters).length === 0) {
    breaches.push(breach("testing/contract", "no-adapter", scope, "Test case has no implementation adapter", "A feature with no adapter can never execute, so it silently contributes zero coverage.", `Add at least one ${Object.values(testAdapterFilenames(taxonomy)).join(" / ")} adapter.`));
  }

  const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
  for (const error of feature.errors) breaches.push(breach("testing/contract", "feature-syntax", discovered.featurePath, error, "The feature file must parse under the repository's restricted Gherkin profile.", "Fix the feature file syntax."));
  if (feature.capability === null) breaches.push(breach("testing/contract", "missing-capability", discovered.featurePath, "Feature is missing its @capability-<id> tag", "Every feature declares the capability it specifies so owners and implementations can be matched to it.", "Add a feature-level @capability-<id> tag."));
  const knownProfiles = profileTable(registry);
  if (feature.comparison === null) breaches.push(breach("testing/contract", "missing-comparison", discovered.featurePath, "Feature is missing a @comparison-<profile> tag", "Comparison belongs to an owned, tested profile — never to an adapter.", `Add one of ${[...knownProfiles.keys()].sort().join(", ")}.`));
  else if (!knownProfiles.has(feature.comparison)) breaches.push(breach("testing/contract", "unknown-comparison", discovered.featurePath, `Unknown comparison profile @comparison-${feature.comparison}`, "A profile is either one of the framework's domain-neutral profiles or one an owner contributes through its 🧪️oracle manifest.", `Add it to this owner's 🔣️oracle.json, or use one of ${[...knownProfiles.keys()].sort().join(", ")}.`));
  if (feature.oracle === null && feature.noOracleDecision === null) {
    breaches.push(breach("testing/oracle", "missing-oracle", discovered.featurePath, "Feature declares neither @oracle-<id> nor @no-oracle-<decision-id>", "A test without a reference implementation or an explicitly recorded no-oracle decision proves only that the code agrees with itself.", "Register an oracle in the oracle registry, or record an approved no-oracle decision."));
  }
  if (feature.oracleInput !== null && feature.oracle === null) {
    breaches.push(breach("testing/oracle", "oracle-input-without-oracle", discovered.featurePath, `@oracle-input-${feature.oracleInput} needs an @oracle-<id>`, "A subject artifact is evidence for an external decoder, never a substitute for one.", "Register the external oracle that consumes the subject artifact."));
  }
  if (feature.oracle !== null && !registry.oracles.some((entry) => entry.id === feature.oracle)) {
    breaches.push(breach("testing/oracle", "unknown-oracle", discovered.featurePath, `Unknown oracle id @oracle-${feature.oracle}`, "Oracles must be centrally registered so their license, test-only status and capabilities are auditable.", "Add the oracle to the registry, or reference an existing id."));
  }
  if (feature.noOracleDecision !== null && !registry.noOracleDecisions.some((entry) => entry.id === feature.noOracleDecision)) {
    breaches.push(breach("testing/oracle", "unknown-no-oracle-decision", discovered.featurePath, `Unknown no-oracle decision @no-oracle-${feature.noOracleDecision}`, "A no-oracle decision must be recorded with its rationale and substitutes, not asserted inline.", "Add the decision to the registry's noOracleDecisions."));
  }
  const oracleEntry = registry.oracles.find((entry) => entry.id === feature.oracle);
  if (oracleEntry && feature.comparison && !oracleEntry.comparisonProfiles.includes(feature.comparison)) {
    breaches.push(breach("testing/oracle", "oracle-profile-mismatch", discovered.featurePath, `Oracle ${oracleEntry.id} does not declare comparison profile ${feature.comparison}`, "An oracle can only be trusted for the projections it was surveyed against.", `Use one of ${oracleEntry.comparisonProfiles.join(", ")}, or extend the registry entry deliberately.`));
  }
  if (oracleEntry && feature.capability && !oracleEntry.capabilities.includes(feature.capability)) {
    breaches.push(breach("testing/oracle", "oracle-capability-mismatch", discovered.featurePath, `Oracle ${oracleEntry.id} does not declare capability ${feature.capability}`, "Reusing an approved oracle outside its surveyed capability set silently skips the research step.", "Extend the registry entry after re-surveying, or pick a different oracle."));
  }
  if (feature.scenarios.length === 0) breaches.push(breach("testing/contract", "no-scenarios", discovered.featurePath, "Feature declares no scenarios", "An empty feature reports as green while testing nothing.", "Add at least one scenario with @id, @level and @mode tags."));

  // 🧭️A differential scenario compares two producers. Without an oracle it therefore needs a second
  // independently written implementation, and the recorded decision has to actually claim that
  // substitute — otherwise the scenario would be comparing one implementation with nothing.
  const decision = registry.noOracleDecisions.find((entry) => entry.id === feature.noOracleDecision);
  if (feature.oracle === null && decision !== undefined) {
    const differential = feature.scenarios.filter((scenario) => scenario.mode === "differential");
    const claimsImplementations = decision.substitutes.includes("independent-implementations");
    if (differential.length > 0 && Object.keys(discovered.adapters).length < 2 && !claimsImplementations) {
      breaches.push(breach("testing/oracle", "differential-without-evidence", discovered.featurePath, `${differential.length} @mode-differential scenario(s) have neither an oracle nor a second implementation`, `The no-oracle decision ${decision.id} rests on ${decision.substitutes.join(", ")}, none of which discharge a differential comparison.`, "Register an oracle, add a second implementation, or restate the scenarios as @mode-conformance / @mode-property with their vectors in the feature."));
    }
    if (claimsImplementations && Object.keys(discovered.adapters).length < 2) {
      breaches.push(breach("testing/oracle", "claimed-implementations-missing", discovered.caseDir, `No-oracle decision ${decision.id} claims the independent-implementations substitute but the case has ${Object.keys(discovered.adapters).length} adapter(s)`, "A substitute that is claimed but not present is the same as having no evidence at all.", "Add the second implementation's adapter, or record a decision that rests on the substitutes actually in place."));
    }
  }

  breaches.push(...mutationCoverageBreaches(discovered, feature, registry));
  breaches.push(...caseAboveSubsetBreaches(discovered, feature, registry));

  const uris = fixtureUrisIn(feature);
  for (const uri of resolveFixtures(repoRoot, discovered, uris).missing) {
    breaches.push(breach("testing/fixture", "missing-fixture", discovered.featurePath, `Fixture ${uri} does not resolve`, "Fixture lookup is explicit, so an unresolved URI is a contract error rather than a runtime surprise.", `Add the file under ${uri.startsWith("shared://") ? `${discovered.owner}/${taxonomy.testFixturesDirName}` : uri.startsWith("asset://") ? discovered.owner : `${discovered.caseDir}/${taxonomy.testFixturesDirName}`}.`));
  }

  const referenced = new Set(uris.map((uri) => uri.split("://")[1]));
  for (const file of fixtureFilesUnder(repoRoot, discovered.localFixtureDir)) {
    const name = file.slice(`${discovered.localFixtureDir}/`.length);
    if (!referenced.has(name)) breaches.push(breach("testing/fixture", "orphan-fixture", file, `Case-local fixture ${name} is referenced by no scenario`, "An unreferenced case-local fixture is either dead weight or evidence of a scenario that was silently deleted.", `Reference it as local://${name}, or delete it.`, "medium"));
  }

  for (const entry of readdirSync(join(repoRoot, discovered.caseDir), { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (entry.name !== taxonomy.testFixturesDirName) breaches.push(breach("testing/taxonomy", "unknown-case-child", `${discovered.caseDir}/${entry.name}`, `Unexpected directory ${entry.name} inside a test case`, `A case holds exactly one ${testFeatureFilename(taxonomy)}, its adapters and an optional ${taxonomy.testFixturesDirName}.`, "Move the directory to its owner, or delete it."));
      continue;
    }
    if (entry.name === testFeatureFilename(taxonomy)) continue;
    if (Object.values(testAdapterFilenames(taxonomy)).includes(entry.name)) continue;
    breaches.push(breach("testing/taxonomy", "unknown-adapter-filename", `${discovered.caseDir}/${entry.name}`, `Unknown file ${entry.name} inside a test case`, "Only the feature file and taxonomy-declared adapters may live in a case directory; anything else is a committed generated wrapper or a stray scratch file.", `Rename it to one of ${Object.values(testAdapterFilenames(taxonomy)).join(", ")}, or delete it.`));
  }

  return breaches;
}


/** 🗺️ One legacy, still-unmanaged executable test file found outside the canonical owner-root tree. */
export type UnmanagedTest = Readonly<{ path: string; area: string; framework: string }>;

const UNMANAGED_TEST_PATTERNS: readonly (readonly [RegExp, string])[] = [
  [/\.test\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/, "vitest/bun"],
  [/\.spec\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/, "vitest/playwright"],
  [/_test\.go$/, "go test"],
  [/^test_.+\.py$/, "pytest"],
  [/Tests?\.cs$/, "xunit"],
];

/**
 * 🗺️ Surveys every executable test that still lives outside the canonical owner-root tree. These are
 * not failures — they are the migration backlog, reported as one ratcheted count rather than
 * thousands of findings, so an owner can be declared migrated only when its own entry reaches zero.
 * Files inside a canonical case directory are excluded: those are adapters, not legacy tests.
 */
export function surveyUnmanagedTests(repoRoot: string): UnmanagedTest[] {
  const taxonomy = testTaxonomy(repoRoot);
  const canonical = new Set(discoverTestCases(repoRoot).map((entry) => entry.caseDir));
  const found: UnmanagedTest[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (canonical.has(rel)) return "skip";
    if (basename(abs) === taxonomy.testFixturesDirName) return "skip";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile()) continue;
      const match = UNMANAGED_TEST_PATTERNS.find(([pattern]) => pattern.test(entry.name));
      if (match === undefined) continue;
      found.push({ path: `${rel}/${entry.name}`, area: rel.split("/")[0] ?? ".", framework: match[1] });
    }
    return "enter";
  });
  return found.sort((a, b) => a.path.localeCompare(b.path));
}

/**
 * 🚫️ Production source files that import a registered oracle package. An oracle is evidence a test
 * host gathers; the moment production code can reach it, the differential test compares an
 * implementation with itself and the dependency stops being test-only.
 *
 * The exclusion set is derived from a FULL repository discovery, never from the caller's selected
 * scope — otherwise narrowing a run to one case would make every other case's adapter look like
 * production source importing an oracle.
 */
export function oracleImportsInProduction(repoRoot: string): { path: string; oracle: string }[] {
  const registry = loadOracleRegistry(repoRoot);
  // 🧩️An owner's whole contribution directory is test-owned BY DEFINITION — that is what the
  // directory is for — so it is derived from the discovered manifests rather than listed by hand.
  const contributionRoots = registry.contributions.map((entry) => entry.manifestPath.slice(0, entry.manifestPath.lastIndexOf("/")));
  const hostRoots = [...contributionRoots, ...registry.oracles.map((entry) => entry.hostPath).filter((value): value is string => value !== undefined)];
  const caseDirs = new Set(discoverTestCases(repoRoot).map((entry) => entry.caseDir));
  // 🧩️Both halves of a host's third-party surface are probed: the packages a registered oracle
  // names, and the EXTERNAL distributions an owner puts on a generated host's import path. The
  // second half is how a Python or npm reference library reaches an adapter, so leaving it out
  // would make the purity rule enforceable in Rust only.
  const probes = [
    ...registry.oracles.flatMap((entry) => oraclePackages(entry).map((name) => ({ label: entry.id, name, ...importProbe(entry.ecosystem, name) }))),
    ...externalOracleHostPackages(registry).map((entry) => ({ label: `host package ${entry.ecosystem}:${entry.name}`, name: entry.name, ...importProbe(entry.ecosystem, entry.name) })),
  ];
  // 🧩️A contribution directory is test-owned because of WHAT IT IS, which the taxonomy names, not
  // because its manifest happens to parse. Deriving ownership from the discovered manifests alone
  // made a directory production source the moment its JSON was absent or malformed — so an owner
  // adding one would see its own reference libraries reported as a production dependency.
  // 🧩️A contribution, a probe and a fixture generator are all test-owned by WHAT THEY ARE. All three
  // link a reference library on purpose; a scan that only knew the first reported the test platform's
  // own measurement tools as a production dependency on the library they exist to invoke.
  const taxonomy = testTaxonomy(repoRoot);
  const testOwnedDirs = [taxonomy.testContributionDirName, taxonomy.testProbeDirName, taxonomy.testGeneratorDirName];
  const inContributionDir = (rel: string): boolean => rel.split("/").some((segment) => testOwnedDirs.includes(segment));
  // 🚫️The repository's OWN meta directory — tickets, caches, notes, metrics — is never production
  // source. Walking into it reported a scratch file inside somebody's ticket folder as a production
  // import of a registered oracle, which is a finding about a scratch file, not about what ships.
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  const isRepositoryMeta = (rel: string): boolean => rel === metaRoot || rel.startsWith(`${metaRoot}/`);
  const isTestOwned = (rel: string): boolean => isRepositoryMeta(rel) || caseDirs.has(rel) || inContributionDir(rel) || hostRoots.some((root) => rel === root || rel.startsWith(`${root}/`)) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`);
  // 🔒️Recorded, shrink-only production debt: a package that was ALREADY production-reachable before
  // it was registered as an oracle. The path is named in the registry entry so the debt is visible
  // in the report instead of silently excused, and `dependency` prints it every run.
  // 🔒️Debt is recorded against a PACKAGE at a path, not against an oracle id, so the same library
  // registered twice — once as an oracle, once as a host package — is excused once rather than
  // reported twice with only one of the two suppressible.
  const recordedDebt = new Set(registry.oracles.flatMap((entry) => (entry.productionDebt?.reachableFrom ?? []).flatMap((path) => oraclePackages(entry).map((name) => `${path}::${name}`))));
  const hits: { path: string; oracle: string }[] = [];
  const reported = new Set<string>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (isTestOwned(rel)) return "skip";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !/\.(rs|ts|tsx|js|jsx|mjs|cjs|go|py|cs)$/.test(entry.name)) continue;
      const filePath = `${rel}/${entry.name}`;
      let content: string;
      try {
        content = readFileSync(join(repoRoot, filePath), "utf8");
      } catch {
        continue;
      }
      for (const probe of probes) {
        if (recordedDebt.has(`${filePath}::${probe.name}`)) continue;
        if (!probe.files.test(entry.name)) continue;
        if (reported.has(`${filePath}::${probe.name}`) || !probe.pattern.test(content)) continue;
        reported.add(`${filePath}::${probe.name}`);
        hits.push({ path: filePath, oracle: probe.label });
      }
    }
    return "enter";
  });
  return hits;
}


/**
 * 🔒️ The repo-wide shrink-only migration ratchet. It carries AREA counts only — never a list of
 * owners — so deleting an implementation area simply surveys as zero and passes, and no framework
 * file goes stale when one disappears.
 */
export type MigrationBaseline = Readonly<{ schemaVersion: number; unmanagedTests: { total: number; byArea: Record<string, number> } }>;

/** 🔒️ Repo-relative path of the ratchet, beside the dependency baseline: repository state, not module state. */
export const MIGRATION_BASELINE_REL_PATH = "🔒️migration.json";

/** 🔒️ Ordered migration ladder every owner walks; an owner advances only on machine-readable evidence. */
export const MIGRATION_STATUSES = ["discovered", "surveyed", "contract-ready", "oracle-green", "subject-green", "parity-green", "coverage-green", "dependency-clean", "legacy-removed", "ci-enforced", "complete"] as const;
export type MigrationStatus = (typeof MIGRATION_STATUSES)[number];

/** 🔒️ Loads the committed repo-wide ratchet. */
export function loadMigrationBaseline(repoRoot: string): MigrationBaseline {
  const path = join(repoRoot, MIGRATION_BASELINE_REL_PATH);
  if (!existsSync(path)) return { schemaVersion: 1, unmanagedTests: { total: 0, byArea: {} } };
  return JSON.parse(readFileSync(path, "utf8")) as MigrationBaseline;
}

/**
 * 🔒️ Every owner's declared migration status, collected from the owners themselves. The framework
 * holds no list of owners, so an owner that is deleted takes its status with it.
 */
export function migrationStatusByOwner(repoRoot: string): Record<string, string> {
  const collected: Record<string, string> = {};
  const core = (() => {
    try {
      const taxonomy = testTaxonomy(repoRoot);
      return JSON.parse(readFileSync(join(repoRoot, testLocationPath(taxonomy, taxonomy.testOracleRegistryLocation)), "utf8")) as { migrationStatus?: Record<string, string> };
    } catch {
      return {};
    }
  })();
  for (const [owner, status] of Object.entries(core.migrationStatus ?? {})) collected[owner] = status;
  for (const contribution of discoverTestContributions(repoRoot)) for (const [owner, status] of Object.entries(contribution.migrationStatus ?? {})) collected[owner] = status;
  return collected;
}

/**
 * 🎯️ Limits mutation-catalog enforcement to artifact standards and owners that expose Gherkin
 * behaviour. Framework-native Rust tests also use `🧬️mutations` for embedded fixture and module
 * trees, but without a standard profile or an owner-level feature there is no artifact contract for a
 * catalog to measure. Standards remain unconditional, while exact feature ownership keeps unprofiled
 * artifact surfaces in scope without letting an unrelated descendant feature claim a fixture tree.
 */
export function mutationVocabularyRequiresCatalog(vocabularyRel: string, featureOwners: ReadonlySet<string>): boolean {
  const owner = dirname(dirname(vocabularyRel));
  return owner.includes(PROFILE_MARKER) || featureOwners.has(owner);
}

/** 🧾️ Repository-wide contract sweep across every discovered case. */
export function validateAllContracts(repoRoot: string, cases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): BreachRecord[] {
  const registry = loadOracleRegistry(repoRoot);
  const allCases = discoverTestCases(repoRoot);
  const featureOwners = new Set(allCases.map((discovered) => discovered.owner));
  const breaches = cases.flatMap((discovered) => validateCaseContract(repoRoot, discovered, registry));
  // 🚫️Self-check: discovery must never return a path the taxonomy excludes. The excluded set is
  // vocabulary, so this check names no area of its own.
  for (const leak of cases.filter((discovered) => isExcludedTestPath(repoRoot, discovered.caseDir))) {
    breaches.push(breach("testing/discovery", "excluded-path-leak", leak.caseDir, "Discovery returned a path the taxonomy excludes", "An excluded area is excluded in the discovery library itself, not by a caller's filter.", "Fix pathExclusions in 🔣️taxonomy.json."));
  }
  for (const hit of oracleImportsInProduction(repoRoot)) {
    breaches.push(breach("testing/dependency", "oracle-in-production", hit.path, `Production source imports the registered oracle ${hit.oracle}`, "An oracle is evidence a test host gathers. Once production code can reach it, the differential test compares an implementation with itself and the dependency stops being test-only.", "Move the usage into the oracle host, or remove the oracle from the registry."));
  }
  // 🔒️Shrink-only migration ratchet: the legacy backlog may only get smaller. Reported as one
  // ratcheted count per area rather than thousands of individual findings, so the signal stays
  // meaningful while Phase 6 migrates owners.
  // 🦠️A catalog-governed vocabulary declared in the TREE but in no manifest is more invisible still:
  // the catalog check above can only see what a manifest declares. Standards and owners with Gherkin
  // behaviour therefore stay in scope even when they have not registered anything yet.
  const vocabularyDir = testTaxonomy(repoRoot).testMutationVocabularyDirName;
  {
    walkDirectories(repoRoot, (abs, rel) => {
      if (isExcludedTestPath(repoRoot, rel)) return "skip";
      if (basename(abs) !== vocabularyDir) return "enter";
      const owner = dirname(dirname(rel));
      const claimed = registry.contributions.some((entry) => entry.owner === owner && entry.mutationCatalogs.length > 0);
      if (!claimed && mutationVocabularyRequiresCatalog(rel, featureOwners)) {
        const taxonomy = testTaxonomy(repoRoot);
        breaches.push(breach("testing/contract", "unregistered-mutation-vocabulary", rel, `A mutation vocabulary is declared here but no catalog registers it`, "The completeness gate measures a feature against a declared catalog. A vocabulary with no catalog is not measured at all — it looks finished and is untested.", `Add a ${taxonomy.testContributionDirName}/${testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId)} beside it declaring a mutationCatalog, and a case that claims it.`));
      }
      return "skip";
    });
  }

  // 🦠️A declared mutation vocabulary that no feature claims is worse than an undeclared one: it reads
  // as covered surface in the manifest while nothing exercises it.
  //
  // 🔍️The claimed set is derived from a FULL discovery rather than from `cases`. Callers narrow the
  // sweep — every generated Nx target runs `--case <one>` — and a repository-wide question answered
  // over one caller's selection would report every other case's catalog as unclaimed.
  const claimed = new Set(allCases.map((discovered) => parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")).mutationCatalog).filter((id): id is string => id !== null));
  for (const catalog of registry.mutationCatalogs) {
    if (claimed.has(catalog.id)) continue;
    breaches.push(breach("testing/contract", "mutation-catalog-unclaimed", catalog.id, `Mutation catalog ${catalog.id} (${catalog.kinds.length} kinds) is claimed by no feature`, "The catalog declares what an artifact can do to itself. Declared and unclaimed means the whole vocabulary is untested while the manifest suggests otherwise.", `Add a case whose feature tags @mutations-${catalog.id}, or remove the catalog.`));
  }

  // 🧬️v2: the three gates v1 structurally could not run — runtime dispatch against the owner manifest
  // against the claimed test inventory, the qualifying-oracle requirement, and fixture provenance.
  for (const contribution of registry.contributions) {
    for (const problem of contribution.problems) {
      breaches.push(breach("testing/contract", "contribution-manifest-invalid", contribution.manifestPath, problem, "An owner contribution is the platform's only view of that owner's oracles, probes, catalogs, manifests and fixtures. A malformed one is not skipped surface — it is surface the gates cannot see at all.", "Correct the contribution record."));
    }
  }
  breaches.push(...mutationInventoryBreaches(repoRoot, registry));
  breaches.push(...capabilityManifestBreaches(registry));
  breaches.push(...binaryProtocolDriftBreaches(repoRoot, registry));
  breaches.push(...stubSerializerBreaches(repoRoot));
  breaches.push(...reimplementationOracleBreaches(repoRoot, registry));
  breaches.push(...nativeSecondImplementationBreaches(registry));
  breaches.push(...registryRecordBreaches(registry));
  breaches.push(...noOracleMisuseBreaches(registry));
  breaches.push(...fixtureProvenanceBreaches(repoRoot, registry));
  breaches.push(...mutationFixtureBreaches(registry));
  breaches.push(...isolationBreaches(registry));

  const baseline = loadMigrationBaseline(repoRoot);
  const byArea = surveyUnmanagedTests(repoRoot).reduce((map, entry) => map.set(entry.area, (map.get(entry.area) ?? 0) + 1), new Map<string, number>());
  for (const [area, count] of [...byArea].sort((a, b) => b[1] - a[1])) {
    const allowed = baseline.unmanagedTests.byArea[area] ?? 0;
    if (count <= allowed) continue;
    breaches.push(breach("testing/discovery", "unmanaged-tests", area, `${count} executable test file(s) outside the canonical owner-root test tree, baseline allows ${allowed}`, "New tests belong in 🧪️tests/<case>/component.feature under their language-neutral owner. The legacy backlog is shrink-only.", `Move the behaviour into a case with one adapter per claimed implementation and delete the legacy test in the same change, or lower the baseline in 📇️registry/🔒️migration.json only when the count actually dropped.`));
  }
  return breaches;
}
//#endregion 🧾️Contract

//#region 🧹️Clean
/** 🧹️ One removable generated test artifact, classified for the report. */
export type TestCleanRemoval = Readonly<{ category: string; path: string; files: number; bytes: number; reason: "all" | "stale" | "incomplete" | "oversized" }>;

/** 🧹️ What a clean run did (or, in dry mode, would do) — identical structure either way. */
export type TestCleanReport = Readonly<{ dry: boolean; removals: readonly TestCleanRemoval[]; protectedPaths: readonly string[]; skippedUnmarked: readonly string[]; retained: readonly { category: string; path: string; files: number; bytes: number }[] }>;

function countTree(abs: string): { files: number; bytes: number } {
  let files = 0;
  let bytes = 0;
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) walk(full);
      else {
        files += 1;
        try {
          bytes += statSync(full).size;
        } catch {
          /* raced away */
        }
      }
    }
  };
  try {
    walk(abs);
  } catch {
    /* raced away */
  }
  return { files, bytes };
}

/**
 * 🧹️ Removes generated test state and nothing else. Every candidate is resolved and proven to sit
 * beneath the canonical test-output root, must carry an ownership marker, and symlinks are never
 * followed — so no tracked fixture, no source file and no excluded path can be reached from here.
 */
export function cleanTestOutputs(repoRoot: string, opts: { dry?: boolean; stale?: boolean; over?: number; liveTestIds?: ReadonlySet<string> } = {}): TestCleanReport {
  const dry = opts.dry ?? false;
  const taxonomy = testTaxonomy(repoRoot);
  const root = resolve(testCacheRoot(repoRoot));
  const removals: TestCleanRemoval[] = [];
  const retained: { category: string; path: string; files: number; bytes: number }[] = [];
  const skippedUnmarked: string[] = [];
  const protectedPaths = [relative(repoRoot, join(getRepoMetaDir(repoRoot), "⚡️cache")).split(sep).join("/")];
  if (!existsSync(root)) return { dry, removals, protectedPaths, skippedUnmarked, retained };

  for (const child of taxonomy.testOutputChildDirs) {
    const childRoot = join(root, child);
    if (!existsSync(childRoot)) continue;
    for (const entry of readdirSync(childRoot, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
      const abs = join(childRoot, entry.name);
      let real = abs;
      try {
        real = realpathSync(abs);
      } catch {
        /* keep the unresolved path so the guard below rejects it */
      }
      const marker = readOutputMarker(repoRoot, abs);
      if (!resolve(real).startsWith(root + sep) || marker === null) {
        skippedUnmarked.push(relative(repoRoot, abs).split(sep).join("/"));
        continue;
      }
      const isStale = opts.liveTestIds !== undefined && !opts.liveTestIds.has(marker.testId);
      const isIncomplete = !existsSync(join(abs, "🏁️done"));
      const { files, bytes } = countTree(abs);
      const rel = relative(repoRoot, abs).split(sep).join("/");
      // 📏️`--over` prunes by size alone. A case that reads a real-world artifact copies it into every
      // work directory it runs in, so the cache grows with the size of the evidence rather than with
      // the number of tests, and a selective sweep is the only one worth running by then.
      const isOversized = opts.over !== undefined && bytes > opts.over;
      const selective = opts.stale === true || opts.over !== undefined;
      if (selective && !isStale && !isIncomplete && !isOversized) {
        retained.push({ category: child, path: rel, files, bytes });
        continue;
      }
      removals.push({ category: child, path: rel, files, bytes, reason: isStale ? "stale" : isIncomplete ? "incomplete" : isOversized ? "oversized" : "all" });
      if (!dry) rmSync(abs, { recursive: true, force: true });
    }
  }
  return { dry, removals, protectedPaths, skippedUnmarked, retained };
}

/** 🧹️ Renders a clean report; dry-run and applied output differ only in the action verb. */
export function formatCleanReport(repoRoot: string, report: TestCleanReport): string {
  const verb = report.dry ? "would-remove" : "removed";
  const totals = report.removals.reduce((acc, row) => ({ files: acc.files + row.files, bytes: acc.bytes + row.bytes }), { files: 0, bytes: 0 });
  const lines = [`[clean test] ${report.dry ? "dry-run" : "applied"} removals=${report.removals.length} files=${totals.files} bytes=${totals.bytes}`];
  for (const category of testTaxonomy(repoRoot).testOutputChildDirs) {
    const rows = report.removals.filter((row) => row.category === category);
    lines.push(`[clean test] ${category}: ${rows.length} (files=${rows.reduce((n, r) => n + r.files, 0)} bytes=${rows.reduce((n, r) => n + r.bytes, 0)})`);
  }
  for (const row of report.removals) lines.push(`[clean test] ${verb} ${row.category} ${row.path} (${row.reason}, files=${row.files}, bytes=${row.bytes})`);
  const kept = report.retained.reduce((acc, row) => ({ files: acc.files + row.files, bytes: acc.bytes + row.bytes }), { files: 0, bytes: 0 });
  if (report.retained.length > 0) lines.push(`[clean test] retained ${report.retained.length} (files=${kept.files} bytes=${kept.bytes})`);
  for (const path of report.skippedUnmarked) lines.push(`[clean test] skipped-unmarked ${path}`);
  for (const path of report.protectedPaths) lines.push(`[clean test] protected ${path}`);
  return lines.join("\n");
}
//#endregion 🧹️Clean

//#region 📊️Reports
/** 📊️ Aggregated view of one run, per level, implementation and owner. */
export type RunSummary = Readonly<{
  level: TestLevel;
  cases: number;
  scenarios: number;
  executed: number;
  passed: number;
  failed: number;
  errored: number;
  byImplementation: Readonly<Record<string, { passed: number; failed: number; errored: number }>>;
  parity: readonly { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number }[];
  problems: readonly string[];
}>;

/** 📊️ Builds the owned run summary from raw host results plus the parity verdicts. */
export function summarizeRun(level: TestLevel, cases: number, scenarios: number, results: readonly TestResult[], parity: RunSummary["parity"], problems: readonly string[]): RunSummary {
  const byImplementation: Record<string, { passed: number; failed: number; errored: number }> = {};
  for (const result of results) {
    const bucket = (byImplementation[result.implementation] ??= { passed: 0, failed: 0, errored: 0 });
    bucket[result.status] += 1;
  }
  return {
    level,
    cases,
    scenarios,
    executed: results.length,
    passed: results.filter((r) => r.status === "passed").length,
    failed: results.filter((r) => r.status === "failed").length,
    errored: results.filter((r) => r.status === "errored").length,
    byImplementation,
    parity,
    problems,
  };
}

/** 📊️ Renders JUnit XML so any CI surface can consume the run without knowing this protocol. */
export function renderJUnit(results: readonly TestResult[]): string {
  const escape = (value: string): string => value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
  const suites = new Map<string, TestResult[]>();
  for (const result of results) {
    const key = `${result.owner}::${result.case}`;
    if (!suites.has(key)) suites.set(key, []);
    suites.get(key)!.push(result);
  }
  const body = [...suites.entries()]
    .map(([name, rows]) => {
      const cases = rows
        .map((row) => {
          const inner = row.status === "passed" ? "" : `<failure message="${escape(row.diagnostics.map((d) => d.message).join("; ") || row.status)}">${escape(row.diagnostics.map((d) => d.detail ?? d.message).join("\n"))}</failure>`;
          return `    <testcase classname="${escape(name)}" name="${escape(`${row.scenario}::${row.implementation}::${row.role}`)}" time="${(row.durationMs / 1000).toFixed(3)}">${inner}</testcase>`;
        })
        .join("\n");
      return `  <testsuite name="${escape(name)}" tests="${rows.length}" failures="${rows.filter((r) => r.status !== "passed").length}">\n${cases}\n  </testsuite>`;
    })
    .join("\n");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<testsuites>\n${body}\n</testsuites>\n`;
}

/** 📊️ Renders a human-readable semantic diff for one failed parity comparison. */
export function renderDiff(testIdentity: string, verdict: ComparisonVerdict): string {
  const lines = [`# ${testIdentity}`, `profile: ${verdict.profile}`, `equal: ${verdict.equal}`, ""];
  for (const diff of verdict.diffs.slice(0, 200)) lines.push(`${diff.path}: ${diff.reason}\n  oracle:  ${JSON.stringify(diff.oracle)}\n  subject: ${JSON.stringify(diff.subject)}`);
  if (verdict.diffs.length > 200) lines.push(`… ${verdict.diffs.length - 200} further differences omitted`);
  return `${lines.join("\n")}\n`;
}
//#endregion 📊️Reports

//#region 🔒️Dependencies
/** 🔒️ Ecosystems the dependency classification covers — all five the repository actually ships. */
export const DEPENDENCY_ECOSYSTEMS = ["rust", "js", "go", "python", "dotnet"] as const;
export type DependencyEcosystem = (typeof DEPENDENCY_ECOSYSTEMS)[number];

/** 🔒️ Phase classification. Manifest placement is evidence, not proof — reachability decides. */
export const DEPENDENCY_CLASSES = ["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"] as const;
export type DependencyClass = (typeof DEPENDENCY_CLASSES)[number];

/** 🔒️ One classified external dependency in the shrink-only baseline. */
export type ClassifiedDependency = Readonly<{ ecosystem: DependencyEcosystem; name: string; version: string; kinds: readonly DependencyClass[]; users: readonly string[]; productionReachable: boolean; oracleIds?: readonly string[]; capabilities?: readonly string[] }>;

/** 🔒️ Maps the legacy four-value `kinds` vocabulary onto the five phase classes. */
export function classifyLegacyKind(kind: string, oracleIds: readonly string[]): DependencyClass {
  if (oracleIds.length > 0) return "test-oracle";
  switch (kind) {
    case "runtime":
      return "production-runtime";
    case "build":
      return "production-build";
    case "test":
      return "test-runner";
    default:
      return "repository-tooling";
  }
}

/** 🔒️ Whether a class can be reached from a production target — the only thing the gate cares about. */
export function isProductionClass(kind: DependencyClass): boolean {
  return kind === "production-runtime" || kind === "production-build";
}

/** 🔒️ The classification ecosystem an implementation's packages belong to. */
export function dependencyEcosystemOf(implementation: Implementation): DependencyEcosystem {
  return implementation === "typescript" ? "js" : implementation;
}

/** 🔒️ The classification ecosystem a registry `ecosystem` string names. */
export function dependencyEcosystemOfRegistryValue(ecosystem: string): DependencyEcosystem {
  return (ecosystem === "javascript" ? "js" : ecosystem) as DependencyEcosystem;
}

/**
 * 🔒️ Every EXTERNAL distribution the contributed test hosts must provision — a contributed host
 * package that carries no `path`. A Rust host reaches its reference libraries through a local crate
 * whose own manifest is already classified, but a Python or npm host names the distribution
 * directly, so without this the declaration would put a third-party library on a host's import path
 * while the ratchet never saw it. Only owner contributions are walked: the framework's own core
 * manifest is domain-neutral and contributes no host package at all.
 */
export function externalOracleHostPackages(registry: OracleRegistry): { ecosystem: DependencyEcosystem; name: string; version: string; users: string[] }[] {
  const byKey = new Map<string, { ecosystem: DependencyEcosystem; name: string; version: string; users: string[] }>();
  for (const contribution of registry.contributions) {
    for (const entry of contribution.oracleHostPackages) {
      if (entry.path !== undefined) continue;
      const ecosystem = dependencyEcosystemOf(entry.implementation);
      const existing = byKey.get(`${ecosystem}:${entry.package}`);
      if (existing === undefined) byKey.set(`${ecosystem}:${entry.package}`, { ecosystem, name: entry.package, version: entry.version ?? "*", users: [contribution.manifestPath] });
      else if (!existing.users.includes(contribution.manifestPath)) existing.users.push(contribution.manifestPath);
    }
  }
  return [...byKey.values()].sort((a, b) => a.ecosystem.localeCompare(b.ecosystem) || a.name.localeCompare(b.name));
}

/**
 * 🚫️ How one ecosystem spells "this source file reaches that package", and which files can spell it
 * at all. Kept as a table because a single regular expression pretending to be five languages at
 * once either misses a form — Python's `import x` was invisible to the purity gate — or matches a
 * stdlib module of an unrelated language and reports a breach that does not exist.
 */
export function importProbe(ecosystem: string, packageName: string): { pattern: RegExp; files: RegExp } {
  const identifier = packageName.replace(/-/g, "_").replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const quoted = packageName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  switch (dependencyEcosystemOfRegistryValue(ecosystem)) {
    case "rust":
      return { pattern: new RegExp(`(^|[^A-Za-z0-9_])(use\\s+${identifier}\\b|extern\\s+crate\\s+${identifier}\\b)`), files: /\.rs$/ };
    case "python":
      return { pattern: new RegExp(`^\\s*(import\\s+${identifier}\\b|from\\s+${identifier}[.\\s]|import\\s+[A-Za-z0-9_,\\s]*\\b${identifier}\\b\\s*$)`, "m"), files: /\.py$/ };
    case "go":
      return { pattern: new RegExp(`["'\`]${quoted}(/[^"'\`]*)?["'\`]`), files: /\.go$/ };
    case "dotnet":
      return { pattern: new RegExp(`^\\s*(global\\s+)?using\\s+(static\\s+)?${identifier}\\b`, "m"), files: /\.cs$/ };
    default:
      return { pattern: new RegExp(`(from\\s+["'\`]${quoted}["'\`]|require\\(["'\`]${quoted}["'\`]\\)|import\\(["'\`]${quoted}["'\`]\\))`), files: /\.(ts|tsx|js|jsx|mjs|cjs)$/ };
  }
}

/** 🔒️ Gate verdict for a candidate baseline against the committed one. */
export type DependencyRatchet = Readonly<{ ok: boolean; newProduction: readonly string[]; unregisteredTestDeps: readonly string[]; removed: readonly string[]; productionCount: number; baselineProductionCount: number }>;

/**
 * 🔒️ The shrink-only ratchet: a new production-reachable external dependency is always forbidden,
 * a new test dependency is only permitted when an oracle registry entry claims it, and removing
 * dependencies always passes.
 */
export function ratchetDependencies(baseline: readonly ClassifiedDependency[], candidate: readonly ClassifiedDependency[], registry: OracleRegistry): DependencyRatchet {
  const key = (entry: ClassifiedDependency): string => `${entry.ecosystem}:${entry.name}`;
  const baselineKeys = new Set(baseline.map(key));
  const candidateKeys = new Set(candidate.map(key));
  const registered = new Set(registry.oracles.flatMap((entry) => oraclePackages(entry)));
  const newProduction = candidate.filter((entry) => !baselineKeys.has(key(entry)) && entry.kinds.some(isProductionClass)).map(key);
  const unregisteredTestDeps = candidate.filter((entry) => !baselineKeys.has(key(entry)) && entry.kinds.includes("test-oracle") && !registered.has(entry.name)).map(key);
  const removed = [...baselineKeys].filter((entry) => !candidateKeys.has(entry));
  const productionCount = candidate.filter((entry) => entry.productionReachable).length;
  const baselineProductionCount = baseline.filter((entry) => entry.productionReachable).length;
  return { ok: newProduction.length === 0 && unregisteredTestDeps.length === 0 && productionCount <= baselineProductionCount, newProduction, unregisteredTestDeps, removed, productionCount, baselineProductionCount };
}

/**
 * 🔎️ Scans the LIVE tree for every externally-sourced dependency each ecosystem declares.
 *
 * The ratchet exists to refuse a NEW production-reachable external dependency, and it was being fed
 * the committed baseline as BOTH of its arguments — so `newProduction` and `unregisteredTestDeps` were
 * provably always empty whatever a developer added to a `package.json`, a `Cargo.toml`, a `go.mod` or
 * a `.csproj`. The function itself is sound; it simply had nothing to compare against. This produces
 * the missing half.
 *
 * A declaration is TEST-OWNED when the file declaring it lives in a test-owned location — a case
 * directory, a `🧪️oracle` / `🔬️probes` / `🏭️generator` directory, the test domain itself, or the
 * repository's own meta directory. Everything else is production.
 */
export function scanDeclaredDependencies(repoRoot: string, registry: OracleRegistry = loadOracleRegistry(repoRoot)): ClassifiedDependency[] {
  const taxonomy = testTaxonomy(repoRoot);
  const testOwnedDirs = [taxonomy.testContributionDirName, taxonomy.testProbeDirName, taxonomy.testGeneratorDirName, taxonomy.testsDirName];
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  const isTestOwned = (rel: string): boolean => rel === metaRoot || rel.startsWith(`${metaRoot}/`) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`) || rel.split("/").some((segment) => testOwnedDirs.includes(segment));
  const registered = new Set(registry.oracles.flatMap((entry) => oraclePackages(entry)));
  const probePackages = new Set(registry.probes.map((probe) => probe.package));

  const found = new Map<string, { entry: ClassifiedDependency; production: boolean }>();
  const record = (ecosystem: DependencyEcosystem, name: string, version: string, user: string, production: boolean): void => {
    if (name.length === 0) return;
    const id = `${ecosystem}:${name}`;
    const previous = found.get(id);
    // 🔒️Production wins over test: one production declaration makes the package production-reachable
    // however many test-owned files also declare it.
    const isProduction = production || (previous?.production ?? false);
    const kinds: ClassifiedDependency["kinds"] = isProduction ? ["production-runtime"] : registered.has(name) || probePackages.has(name) ? ["test-oracle"] : ["test-runner"];
    found.set(id, {
      production: isProduction,
      entry: {
        ecosystem,
        name,
        version: previous?.entry.version ?? version,
        kinds,
        users: [...new Set([...(previous?.entry.users ?? []), user])].sort(),
        productionReachable: isProduction,
        oracleIds: registered.has(name) ? registry.oracles.filter((oracle) => oraclePackages(oracle).includes(name)).map((oracle) => oracle.id).sort() : undefined,
      },
    });
  };

  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    const production = !isTestOwned(rel);
    for (const name of readdirSync(abs, { withFileTypes: true })) {
      if (!name.isFile()) continue;
      const filePath = `${rel}/${name.name}`;
      let source: string;
      try {
        source = readFileSync(join(repoRoot, filePath), "utf8");
      } catch {
        continue;
      }
      if (name.name === "package.json") {
        try {
          const parsed = JSON.parse(source) as { dependencies?: Record<string, string>; devDependencies?: Record<string, string> };
          for (const [dependency, version] of Object.entries(parsed.dependencies ?? {})) if (!version.startsWith("workspace:") && !version.startsWith("file:") && !version.startsWith("link:")) record("js", dependency, version, filePath, production);
          for (const [dependency, version] of Object.entries(parsed.devDependencies ?? {})) if (!version.startsWith("workspace:") && !version.startsWith("file:") && !version.startsWith("link:")) record("js", dependency, version, filePath, false);
        } catch {
          /* 🧭️A malformed package.json is somebody else's gate to report. */
        }
        continue;
      }
      if (name.name === "Cargo.toml") {
        // 🦀️Only registry crates count. THREE things are not external distributions and each was
        // being counted as one: a `path = "…"` dependency is in-repository source; a
        // `{ workspace = true }` dependency resolves through the root manifest, which declares the
        // repository's own crates by path; and the keys of a `[[bench]]`/`[[bin]]` table are not
        // dependencies at all — a `^\[([^\]]+)\]$` heading match cannot match `[[bench]]`, so the
        // section silently stayed on `dependencies` and `harness`, `name` and `path` were reported as
        // crates. Any line opening a bracket now resets the section.
        let section = "";
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (trimmed.startsWith("[")) {
            section = trimmed.match(/^\[([^\]]+)\]$/)?.[1] ?? "";
            continue;
          }
          if (!/^(dependencies|build-dependencies|dev-dependencies|workspace\.dependencies|target\..+\.dependencies)$/.test(section)) continue;
          const declaration = line.match(/^\s*([A-Za-z0-9_-]+)\s*=\s*(.+)$/);
          if (declaration === null) continue;
          const value = declaration[2]!;
          if (/\bpath\s*=/.test(value) || /\bworkspace\s*=\s*true/.test(value)) continue;
          const version = value.match(/"([^"]+)"/)?.[1] ?? "*";
          record("rust", declaration[1]!, version, filePath, production && section !== "dev-dependencies");
        }
        continue;
      }
      if (name.name === "go.mod") {
        for (const line of source.split(/\r?\n/)) {
          const declaration = line.trim().match(/^([a-z0-9.\-]+\/[^\s]+)\s+(v[^\s]+)/);
          if (declaration !== null && declaration[1]!.includes(".")) record("go", declaration[1]!, declaration[2]!, filePath, production);
        }
        continue;
      }
      if (name.name.endsWith(".csproj")) {
        for (const declaration of source.matchAll(/<PackageReference\s+Include="([^"]+)"(?:\s+Version="([^"]+)")?/g)) {
          record("dotnet", declaration[1]!, declaration[2] ?? "*", filePath, production && !/test/i.test(name.name));
        }
        continue;
      }
      if (name.name === "requirements.txt") {
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (trimmed.length === 0 || trimmed.startsWith("#") || trimmed.startsWith("-")) continue;
          const declaration = trimmed.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;#]+))?/);
          if (declaration !== null && declaration[1]!.length > 1) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
        }
        continue;
      }
      if (name.name === "pyproject.toml") {
        // 🐍️ONLY the `dependencies = [ … ]` array. Reading every `key = value` line reported
        // `name`, `version`, `requires-python` and `package` as production dependencies — a parser
        // that finds four packages in a file declaring none is worse than no parser.
        let inArray = false;
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (/^(dependencies|optional-dependencies\.[A-Za-z0-9_-]+)\s*=\s*\[/.test(trimmed)) {
            inArray = true;
            if (trimmed.endsWith("]")) inArray = false;
            for (const quoted of trimmed.matchAll(/"([^"]+)"/g)) {
              const declaration = quoted[1]!.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;]+))?/);
              if (declaration !== null) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
            }
            continue;
          }
          if (!inArray) continue;
          if (trimmed.startsWith("]")) {
            inArray = false;
            continue;
          }
          for (const quoted of trimmed.matchAll(/"([^"]+)"/g)) {
            const declaration = quoted[1]!.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;]+))?/);
            if (declaration !== null) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
          }
        }
      }
    }
    return "enter";
  });
  return [...found.values()].map((row) => row.entry).sort((a, b) => `${a.ecosystem}:${a.name}`.localeCompare(`${b.ecosystem}:${b.name}`));
}

/** 🐹️ Production dependency closure of a Go module — `go list` deps of the non-test build. */
export function goProductionClosure(repoRoot: string, moduleDir: string): string[] {
  const probe = runProbe("go", ["list", "-deps", "./..."], { cwd: join(repoRoot, moduleDir), env: { ...process.env, GOWORK: join(repoRoot, "go.work") }, budgetMs: testLevelBudgetMs("long") });
  if ((probe.status ?? 1) !== 0) return [];
  return probe.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.includes(".") && line.includes("/"));
}

/** 🐍️ Runtime imports of a Python package — source-level import analysis of non-test modules. */
export function pythonRuntimeImports(repoRoot: string, packageDir: string): string[] {
  const abs = join(repoRoot, packageDir);
  if (!existsSync(abs)) return [];
  const imports = new Set<string>();
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (!SKIP_DIR_NAMES.has(entry.name)) walk(full);
        continue;
      }
      if (!entry.name.endsWith(".py") || entry.name.startsWith("test_") || entry.name.endsWith("_test.py")) continue;
      for (const line of readFileSync(full, "utf8").split(/\r?\n/)) {
        const match = line.match(/^\s*(?:from\s+([A-Za-z0-9_.]+)\s+import|import\s+([A-Za-z0-9_.]+))/);
        const module = (match?.[1] ?? match?.[2])?.split(".")[0];
        if (module && !module.startsWith("_")) imports.add(module);
      }
    }
  };
  walk(abs);
  return [...imports].sort();
}

/** 🔷️ Package references of a .NET project, split by whether the project is a test asset. */
export function dotnetPackageReferences(repoRoot: string, csprojRel: string): { packages: string[]; isTestProject: boolean } {
  const abs = join(repoRoot, csprojRel);
  if (!existsSync(abs)) return { packages: [], isTestProject: false };
  const content = readFileSync(abs, "utf8");
  const packages = [...content.matchAll(/<PackageReference\s+Include="([^"]+)"/g)].map((match) => match[1]!);
  const isTestProject = /<IsTestProject>\s*true\s*<\/IsTestProject>/i.test(content) || packages.some((name) => name.startsWith("xunit") || name.startsWith("Microsoft.NET.Test"));
  return { packages, isTestProject };
}
//#endregion 🔒️Dependencies

//#region 🧭️Adapters
/** 🧭️ Context one scenario handler receives — its plan slice, resolved fixtures and its work directory. */
export type AdapterContext = Readonly<{
  plan: TestCasePlan;
  scenario: FeatureScenario;
  role: TestRole;
  repoRoot: string;
  workDir: string;
  /** 🧫️ Absolute path of a resolved fixture; throws for an unresolved URI rather than returning a silent default. */
  fixture(uri: string): string;
  /** 🧫️ Bytes of a resolved fixture. */
  fixtureBytes(uri: string): Uint8Array;
  /** 🧫️ Copies an immutable fixture into the case's work directory and returns the mutable copy's path. */
  copyFixture(uri: string, as?: string): string;
  /** 📦️ Directory a handler writes its produced artifact bundle into. */
  artifactDir: string;
  /** 📦️ Absolute path to write one named result artifact to; creates parent directories. */
  artifact(role: string, filename: string): string;
  /** 🎲️ Deterministic seed for this scenario, from its `@seed-…` tag. */
  seed: string;
}>;

/** 🧭️ What a scenario handler returns: the raw artifact plus the projection the profile compares. */
export type AdapterOutcome = Readonly<{
  raw?: string | Uint8Array;
  projection: unknown;
  /** 📦️ Named files this handler produced, relative to `ctx.artifactDir`. A BRep case returns its STEP and its mesh here rather than smuggling them through the projection. */
  artifacts?: readonly { role: string; path: string; mediaType: string }[];
  /** 🏭️ Set by a SUBJECT handler that invoked production dispatch. Omitting it is how a vector-replay adapter is detected. */
  productionDispatch?: { invoked: true; operation: string; bridgeVersion: number };
  diagnostics?: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
}>;

/** 🧭️ One implementation's registration for a case: which scenarios it serves, in which roles. */
export type TestAdapter = Readonly<{ implementation: Implementation; scenarios: Readonly<Record<string, Readonly<{ subject?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome>; oracle?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> }>>> }>;

/** 🧭️ Declares a TypeScript adapter. The coordinator validates the registration against the feature. */
export function defineTestAdapter(adapter: TestAdapter): TestAdapter {
  return adapter;
}

/** 🧭️ Checks a registration against the plan: no unknown scenario, no unregistered scenario. */
export function validateRegistration(plan: TestCasePlan, adapter: TestAdapter, role: TestRole): string[] {
  const problems: string[] = [];
  const planned = new Set(plan.scenarios.map((scenario) => scenario.id));
  for (const id of Object.keys(adapter.scenarios)) if (!planned.has(id)) problems.push(`adapter registers unknown scenario ${JSON.stringify(id)}`);
  for (const scenario of plan.scenarios) {
    const entry = adapter.scenarios[scenario.id];
    if (entry === undefined || entry[role] === undefined) problems.push(`adapter has no ${role} registration for scenario ${JSON.stringify(scenario.id)}`);
  }
  return problems;
}

/** 🧭️ Builds the context handed to one scenario handler, wiring fixture resolution and the work dir. */
export function makeAdapterContext(repoRoot: string, plan: TestCasePlan, scenario: FeatureScenario, role: TestRole): AdapterContext {
  const workDir = isAbsolute(plan.workDir) ? plan.workDir : join(repoRoot, plan.workDir);
  mkdirSync(workDir, { recursive: true });
  const lookup = (uri: string): string => {
    const fixture = plan.fixtures.find((entry) => entry.uri === uri);
    if (fixture === undefined) throw new Error(`fixture ${uri} is not part of this plan — declare it in the feature file`);
    return join(repoRoot, fixture.path);
  };
  const artifactDir = plan.artifactDir === undefined ? join(workDir, "📦️artifacts") : isAbsolute(plan.artifactDir) ? plan.artifactDir : join(repoRoot, plan.artifactDir);
  mkdirSync(artifactDir, { recursive: true });
  return {
    plan,
    scenario,
    role,
    repoRoot,
    workDir,
    artifactDir,
    artifact: (role_, filename) => {
      const target = join(artifactDir, role_, filename);
      mkdirSync(dirname(target), { recursive: true });
      return target;
    },
    seed: scenario.seed ?? "0",
    fixture: lookup,
    fixtureBytes: (uri) => readFileSync(lookup(uri)),
    copyFixture: (uri, as) => {
      const source = lookup(uri);
      const target = join(workDir, as ?? basename(source));
      mkdirSync(dirname(target), { recursive: true });
      // 🧫️Clone-on-write where the filesystem offers it. Real-world fixtures are megabytes and every
      // work directory takes its own copy, so this is the difference between a constant and a linear
      // cost. It must stay a COPY and never a hard link — the caller is handed a MUTABLE path, and a
      // link would let a mutation scenario write through into the committed fixture.
      cpSync(source, target, { mode: constants.COPYFILE_FICLONE });
      return target;
    },
  };
}
//#endregion 🧭️Adapters

//#region 🏃️Coordinator
/** 🏃️ Everything one execution of a case at a level needs, resolved to absolute cache paths. */
export function planExecution(repoRoot: string, discovered: DiscoveredCase, level: TestLevel, role: TestRole, implementation: Implementation): { plan: TestCasePlan; feature: ParsedFeature; missingFixtures: string[]; planPath: string } {
  const { plan: base, feature, missingFixtures } = buildCasePlan(repoRoot, discovered, level);
  const cacheKey = digest(`${discovered.projectName}|${base.featureHash}|${setDigest(base.fixtures.map((f) => [f.name, f.digest] as const))}|${level}|${role}|${implementation}`);
  const workDir = join(testCacheDir(repoRoot, "work"), `${discovered.projectName}-${role}-${implementation}`);
  const outputDir = join(testCacheDir(repoRoot, "results"), `${discovered.projectName}-${role}-${implementation}`);
  // 📦️Produced artifacts live BESIDE the mutable scratch copy, never inside it: a run that writes its
  // STEP output into the same directory it copied its operands into cannot later say which is which.
  const artifactDir = join(outputDir, "📦️artifacts");
  const ownerId = `${discovered.owner}::${discovered.case}`;
  // 🏁️A directory being planned is by definition not finished. Clearing any completion marker left
  // by a previous run keeps "in progress" and "complete" honest across re-runs, which is what stops
  // `clean test --stale` from removing a live run's state.
  for (const dir of [workDir, outputDir]) {
    markOutputDir(repoRoot, dir, { testId: ownerId, cacheKey });
    rmSync(join(dir, "🏁️done"), { force: true });
  }
  mkdirSync(artifactDir, { recursive: true });
  const plan: TestCasePlan = { ...base, role, implementation, workDir, outputDir, artifactDir, resultsPath: join(outputDir, "📤️results.jsonl") };
  const planPath = join(workDir, "📋️plan.json");
  writeFileSync(planPath, `${JSON.stringify(plan, null, 2)}\n`);
  return { plan, feature, missingFixtures, planPath };
}

/** 🏁️ Marks a generated output directory complete, so an interrupted run stays distinguishable from a finished one. */
export function markRunComplete(absDir: string): void {
  // 🏁️A run can legitimately finish without the directory existing — a case whose scenarios were all
  // filtered out at this level never writes anything into its work directory. Crashing the whole
  // sweep at the finish line for that is a worse failure than the empty directory it complains about.
  mkdirSync(absDir, { recursive: true });
  writeFileSync(join(absDir, "🏁️done"), "");
}

/** 🧮️ Pairs every subject result with its oracle counterpart and applies the case's comparison profile. */
export function evaluateParity(profile: ComparisonProfile, results: readonly TestResult[], profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): { verdicts: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number; verdict: ComparisonVerdict }[]; unmatched: string[] } {
  const oracles = new Map<string, TestResult>();
  for (const result of results) if (result.role === "oracle") oracles.set(`${result.owner}::${result.case}::${result.scenario}`, result);
  const verdicts: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number; verdict: ComparisonVerdict }[] = [];
  const unmatched: string[] = [];
  for (const result of results) {
    if (result.role !== "subject") continue;
    const oracle = oracles.get(`${result.owner}::${result.case}::${result.scenario}`);
    if (oracle === undefined) {
      unmatched.push(result.testId);
      continue;
    }
    const verdict = compareProjections(profile, oracle.output.projection, result.output.projection, profiles);
    verdicts.push({ testId: result.testId, profile, equal: verdict.equal, diffs: verdict.diffs.length, verdict });
  }
  return { verdicts, unmatched };
}

/** 🧮️ Pairwise subject equivalence, so two implementations cannot exploit different oracle ambiguities. */
export function evaluateCrossSubjectParity(profile: ComparisonProfile, results: readonly TestResult[], profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): { pair: string; equal: boolean; diffs: number }[] {
  const byScenario = new Map<string, TestResult[]>();
  for (const result of results) {
    if (result.role !== "subject") continue;
    const key = `${result.owner}::${result.case}::${result.scenario}`;
    if (!byScenario.has(key)) byScenario.set(key, []);
    byScenario.get(key)!.push(result);
  }
  const out: { pair: string; equal: boolean; diffs: number }[] = [];
  for (const [key, rows] of byScenario) {
    const sorted = [...rows].sort((a, b) => a.implementation.localeCompare(b.implementation));
    for (let i = 0; i < sorted.length; i += 1) {
      for (let j = i + 1; j < sorted.length; j += 1) {
        const verdict = compareProjections(profile, sorted[i]!.output.projection, sorted[j]!.output.projection, profiles);
        out.push({ pair: `${key}::${sorted[i]!.implementation}~${sorted[j]!.implementation}`, equal: verdict.equal, diffs: verdict.diffs.length });
      }
    }
  }
  return out;
}
//#endregion 🏃️Coordinator


//#region 📈️Metrics
/**
 * 📈️ The seven non-aggregate metrics of §13. A single well-covered implementation must never be able
 * to mask an untested one in another language, so every metric that CAN be per-implementation is.
 */
export type CoverageMetrics = Readonly<{
  /** 🎬️ Feature scenarios that were actually executed, over scenarios declared. */
  scenarioCoverage: { executed: number; declared: number; ratio: number; missing: readonly string[] };
  /** 🌐️ Claimed implementations that produced a result, over implementations claimed by an adapter. */
  implementationCoverage: Readonly<Record<string, { executed: number; declared: number; ratio: number }>>;
  /** ⚖️ Executed subject results that passed a parity comparison, per implementation. */
  parityCoverage: Readonly<Record<string, { compared: number; equal: number; ratio: number }>>;
  /** 🔮️ Cases backed by an approved oracle or an approved no-oracle decision, over cases. */
  oracleCoverage: { backed: number; cases: number; ratio: number; unbacked: readonly string[] };
  /** 🔒️ External dependencies that are not production-reachable, over external dependencies. */
  dependencyCleanCoverage: { clean: number; total: number; ratio: number };
}>;

/** 📈️ Per-implementation source coverage, so an aggregate percentage cannot hide an untested language. */
export type ImplementationCoverage = Readonly<{ implementation: string; lines: { covered: number; total: number; ratio: number }; branches: { covered: number; total: number; ratio: number } | null }>;

function ratio(part: number, whole: number): number {
  return whole === 0 ? 1 : Number((part / whole).toFixed(6));
}

/**
 * 📈️ Computes the run's non-aggregate metrics from the discovered cases and the emitted results.
 * Adapters, generated hosts, fixtures and oracle implementations are not repository source and are
 * therefore never counted as covered production code — only the claimed implementations are.
 */
export function computeCoverageMetrics(repoRoot: string, cases: readonly DiscoveredCase[], results: readonly TestResult[], parity: readonly { testId: string; equal: boolean }[], baseline: readonly ClassifiedDependency[]): CoverageMetrics {
  const registry = loadOracleRegistry(repoRoot);

  const declaredScenarios = new Set<string>();
  const backedCases: string[] = [];
  const unbackedCases: string[] = [];
  const declaredByImplementation = new Map<string, Set<string>>();
  for (const discovered of cases) {
    const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
    const backed = (feature.oracle !== null && registry.oracles.some((entry) => entry.id === feature.oracle)) || (feature.noOracleDecision !== null && registry.noOracleDecisions.some((entry) => entry.id === feature.noOracleDecision));
    (backed ? backedCases : unbackedCases).push(discovered.caseDir);
    for (const scenario of feature.scenarios) {
      declaredScenarios.add(`${discovered.owner}::${discovered.case}::${scenario.id}`);
      for (const implementation of Object.keys(discovered.adapters)) {
        if (!declaredByImplementation.has(implementation)) declaredByImplementation.set(implementation, new Set());
        declaredByImplementation.get(implementation)!.add(`${discovered.owner}::${discovered.case}::${scenario.id}`);
      }
    }
  }

  const executedScenarios = new Set(results.map((result) => `${result.owner}::${result.case}::${result.scenario}`));
  const executedByImplementation = new Map<string, Set<string>>();
  for (const result of results) {
    if (!executedByImplementation.has(result.implementation)) executedByImplementation.set(result.implementation, new Set());
    executedByImplementation.get(result.implementation)!.add(`${result.owner}::${result.case}::${result.scenario}`);
  }

  const implementationCoverage: Record<string, { executed: number; declared: number; ratio: number }> = {};
  for (const [implementation, declared] of declaredByImplementation) {
    const executed = executedByImplementation.get(implementation)?.size ?? 0;
    implementationCoverage[implementation] = { executed, declared: declared.size, ratio: ratio(executed, declared.size) };
  }

  const parityByImplementation: Record<string, { compared: number; equal: number; ratio: number }> = {};
  for (const verdict of parity) {
    // 🧭️A verdict id is either a stable test id (`…::<implementation>::subject`) or a cross-subject
    // pair (`…::<a>~<b>`); both name every implementation they speak for.
    const implementations = verdict.testId.includes("~") ? (verdict.testId.split("::").pop() ?? "").split("~") : [verdict.testId.split("::").at(-2) ?? ""];
    for (const implementation of implementations.filter(Boolean)) {
      const bucket = (parityByImplementation[implementation] ??= { compared: 0, equal: 0, ratio: 1 });
      bucket.compared += 1;
      if (verdict.equal) bucket.equal += 1;
      bucket.ratio = ratio(bucket.equal, bucket.compared);
    }
  }

  const external = baseline.filter((entry) => entry.kinds.length > 0);
  return {
    scenarioCoverage: { executed: executedScenarios.size, declared: declaredScenarios.size, ratio: ratio(executedScenarios.size, declaredScenarios.size), missing: [...declaredScenarios].filter((id) => !executedScenarios.has(id)).sort() },
    implementationCoverage,
    parityCoverage: parityByImplementation,
    oracleCoverage: { backed: backedCases.length, cases: cases.length, ratio: ratio(backedCases.length, cases.length), unbacked: unbackedCases.sort() },
    dependencyCleanCoverage: { clean: external.filter((entry) => !entry.productionReachable).length, total: external.length, ratio: ratio(external.filter((entry) => !entry.productionReachable).length, external.length) },
  };
}

/**
 * 📈️ Enforces the non-aggregate gates. Scenario, implementation, parity and oracle coverage must all
 * be complete for the selected scope; source coverage is enforced per implementation against the
 * repository threshold, never as one blended percentage.
 */
export function enforceMetricGates(metrics: CoverageMetrics, perImplementation: readonly ImplementationCoverage[], sourceThreshold: number): string[] {
  const failures: string[] = [];
  if (metrics.scenarioCoverage.ratio < 1) failures.push(`scenario coverage ${metrics.scenarioCoverage.executed}/${metrics.scenarioCoverage.declared}: ${metrics.scenarioCoverage.missing.slice(0, 10).join(", ")}`);
  for (const [implementation, row] of Object.entries(metrics.implementationCoverage)) if (row.ratio < 1) failures.push(`implementation coverage ${implementation} ${row.executed}/${row.declared} — a claimed implementation did not run`);
  for (const [implementation, row] of Object.entries(metrics.parityCoverage)) if (row.ratio < 1) failures.push(`parity coverage ${implementation} ${row.equal}/${row.compared}`);
  if (metrics.oracleCoverage.ratio < 1) failures.push(`oracle coverage ${metrics.oracleCoverage.backed}/${metrics.oracleCoverage.cases}: ${metrics.oracleCoverage.unbacked.join(", ")}`);
  for (const row of perImplementation) {
    if (row.lines.ratio * 100 < sourceThreshold) failures.push(`${row.implementation} line coverage ${(row.lines.ratio * 100).toFixed(2)}% below ${sourceThreshold}%`);
    if (row.branches !== null && row.branches.ratio * 100 < sourceThreshold) failures.push(`${row.implementation} branch coverage ${(row.branches.ratio * 100).toFixed(2)}% below ${sourceThreshold}%`);
  }
  return failures;
}

/** 📈️ Renders the metrics as the repository dashboard row set of §22. */
export function formatMetrics(metrics: CoverageMetrics, perImplementation: readonly ImplementationCoverage[]): string {
  const lines = [
    `[metrics] scenario coverage      ${metrics.scenarioCoverage.executed}/${metrics.scenarioCoverage.declared} (${(metrics.scenarioCoverage.ratio * 100).toFixed(1)}%)`,
    `[metrics] oracle coverage        ${metrics.oracleCoverage.backed}/${metrics.oracleCoverage.cases} (${(metrics.oracleCoverage.ratio * 100).toFixed(1)}%)`,
    `[metrics] dependency-clean       ${metrics.dependencyCleanCoverage.clean}/${metrics.dependencyCleanCoverage.total} (${(metrics.dependencyCleanCoverage.ratio * 100).toFixed(1)}%)`,
  ];
  for (const [implementation, row] of Object.entries(metrics.implementationCoverage).sort()) lines.push(`[metrics] implementation ${implementation.padEnd(11)} ${row.executed}/${row.declared} scenarios`);
  for (const [implementation, row] of Object.entries(metrics.parityCoverage).sort()) lines.push(`[metrics] parity         ${implementation.padEnd(11)} ${row.equal}/${row.compared} comparisons equal`);
  for (const row of perImplementation) lines.push(`[metrics] source         ${row.implementation.padEnd(11)} lines ${(row.lines.ratio * 100).toFixed(2)}%${row.branches === null ? " (branches not reported)" : ` branches ${(row.branches.ratio * 100).toFixed(2)}%`}`);
  return lines.join("\n");
}
//#endregion 📈️Metrics

//#region 🪆️Subset
/** 🖥️ A supported execution platform, the last coordinate of every coverage row. */
export type PlatformId = `${"linux" | "darwin" | "win32"}-${"x64" | "arm64"}`;

/** 🖥️ This process's platform coordinate. Never inferred from a hostname or a CI variable. */
export function currentPlatform(): PlatformId {
  const os = process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}` as PlatformId;
}

/**
 * 🪆️ The SMALLEST semantic subset one mutation owns, plus the selector addressing the entity inside
 * it. There is deliberately no wildcard: an operation that spans subsets declares an explicit typed
 * `compound`, and falling back to the whole artifact is a contract failure rather than a default.
 */
export type SubsetTarget = Readonly<{
  artifact: string;
  standard: string;
  subset: string;
  compound?: readonly string[];
  selector?: Readonly<{ type: "entity-id" | "entity-path" | "entity-set" | "whole-subset"; value: string | readonly string[] }>;
}>;

/** 🚫️ Subset ids that name "everything" rather than a semantic scope. */
export const WILDCARD_SUBSET_IDS: readonly string[] = ["*", "any", "all", "unconstrained", ""];

/** 🚫️ Whether a subset id SPELLS a wildcard. Whether it IS one depends on what else the artifact declares — see `isWildcardSubsetFor`. */
export function isWildcardSubset(subset: string): boolean {
  return WILDCARD_SUBSET_IDS.includes(subset.trim().toLowerCase());
}

/**
 * 🪆️ What subsets one artifact/standard actually declares, read from its own
 * `🪆️subsets/<taxonomy filename>` component. Cached, because the gate asks per mutation.
 */
const declaredSubsetsCache = new Map<string, ReadonlyMap<string, readonly string[]>>();

export function declaredSubsets(repoRoot: string): ReadonlyMap<string, readonly string[]> {
  const cached = declaredSubsetsCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const found = new Map<string, readonly string[]>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== "🪆️subsets") return "enter";
    for (const candidate of [filename, "🔣️.json"]) {
      const manifest = join(abs, candidate);
      if (!existsSync(manifest)) continue;
      try {
        const parsed = JSON.parse(readFileSync(manifest, "utf8")) as { artifact?: string; standard?: string; subsets?: Record<string, unknown> };
        if (typeof parsed.artifact === "string") found.set(`${parsed.artifact}@${parsed.standard ?? ""}`, Object.keys(parsed.subsets ?? {}));
      } catch {
        /* 🧭️A malformed subsets component is the taxonomy phase's finding, not this one's. */
      }
      break;
    }
    return "skip";
  });
  declaredSubsetsCache.set(repoRoot, found);
  return found;
}

/** 🪆️ Whether an artifact has RECORDED that it genuinely has one scope, rather than merely not having been split. */
export function subsetPolicyIsSingle(repoRoot: string, artifact: string, standard: string): boolean {
  return declaredSubsetPolicies(repoRoot).get(`${artifact}@${standard}`) === "single";
}

const declaredSubsetPolicyCache = new Map<string, ReadonlyMap<string, string>>();

/** 🪆️ The `subsetPolicy` each artifact records beside its subsets, if any. */
export function declaredSubsetPolicies(repoRoot: string): ReadonlyMap<string, string> {
  const cached = declaredSubsetPolicyCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const found = new Map<string, string>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== "🪆️subsets") return "enter";
    for (const candidate of [filename, "🔣️.json"]) {
      const manifestPath = join(abs, candidate);
      if (!existsSync(manifestPath)) continue;
      try {
        const parsed = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: string; standard?: string; subsetPolicy?: string };
        if (typeof parsed.artifact === "string" && typeof parsed.subsetPolicy === "string") found.set(`${parsed.artifact}@${parsed.standard ?? ""}`, parsed.subsetPolicy);
      } catch {
        /* 🧭️Reported by the taxonomy phase. */
      }
      break;
    }
    return "skip";
  });
  declaredSubsetPolicyCache.set(repoRoot, found);
  return found;
}

/**
 * 🚫️ Whether a subset is a wildcard IN ITS ARTIFACT'S CONTEXT.
 *
 * `✳️any` means two different things depending on what sits beside it. For `s.stdio.step@ap214`, which
 * declares `cc1`…`cc6`, it lumps seven conformance classes into one bucket and hides which of them a
 * mutation was actually exercised against — a genuine wildcard. For the 82 artifacts whose ONLY
 * declared subset is `*`, there is nothing narrower to be scoped to, and calling it a wildcard would
 * demand a split that the format itself does not have. Refusing the spelling unconditionally produced
 * 82 findings nobody could act on, which is how a gate teaches people to ignore it.
 */
export function isWildcardSubsetFor(repoRoot: string, artifact: string, standard: string, subset: string): boolean {
  if (!isWildcardSubset(subset)) return false;
  const siblings = declaredSubsets(repoRoot).get(`${artifact}@${standard}`) ?? [];
  return siblings.filter((candidate) => !isWildcardSubset(candidate)).length > 0;
}

/** 🪆️ Renders a subset target as the stable coordinate every report row and cache key is keyed by. */
export function subsetCoordinate(target: SubsetTarget): string {
  const scope = target.compound !== undefined ? `(${[...target.compound].sort().join("+")})` : target.subset;
  return `${target.artifact}@${target.standard}/${scope}`;
}

/** 🪆️ Reads the standards/subsets coordinates out of an owner path, or `null` when it carries none. */
export function subsetCoordinatesOfOwner(owner: string): { standardDirectoryName: string; subsetDirectoryName: string; standard: string; subset: string } | null {
  const match = owner.match(/\/🏅️standards\/(🔖️[^/]+)\/🪆️subsets\/(✳️[^/]+)$/);
  if (match === null) return null;
  return { standardDirectoryName: match[1]!, subsetDirectoryName: match[2]!, standard: match[1]!.slice("🔖️".length), subset: match[2]!.slice("✳️".length) };
}
//#endregion 🪆️Subset

//#region 🧬️Manifest
/** 🎯️ The semantic class a fixture DECLARES for its mutation. "Any non-crash result" is not a class. */
export const MUTATION_OUTCOME_CLASSES = ["applied", "no-op", "empty", "disjoint", "rejected"] as const;
export type MutationOutcomeClass = (typeof MUTATION_OUTCOME_CLASSES)[number];

/**
 * ✅️ The only oracle kinds that can DISCHARGE a mutation's external-oracle requirement.
 *
 * `verified-native-second-implementation` is the one exception to "a second implementation never
 * discharges" (see `SUPPLEMENTAL_ORACLE_KINDS` below), and it exists ONLY for a format no third party
 * can, even in principle, implement — `isSemioNativeArtifact` draws that boundary, and
 * `nativeSecondImplementationBreaches` makes the claim EARNED rather than merely asserted: a false or
 * lazy claim under this kind fails loudly, by design.
 */
export const QUALIFYING_ORACLE_KINDS = ["third-party-library", "third-party-cli", "standards-reference-tool", "verified-native-second-implementation"] as const;
export type QualifyingOracleKind = (typeof QUALIFYING_ORACLE_KINDS)[number];

/**
 * ➕️ Required supplements that can never substitute for a qualifying oracle. `cross-semio-implementation`
 * names a second implementation written INSIDE this repository from this repository's own schemas: it
 * catches transcription errors and cannot catch a misread specification, because both halves read the
 * same one.
 */
export const SUPPLEMENTAL_ORACLE_KINDS = ["metamorphic", "inverse", "round-trip", "property", "cross-semio-implementation"] as const;
export type SupplementalOracleKind = (typeof SUPPLEMENTAL_ORACLE_KINDS)[number];

export type OracleKind = QualifyingOracleKind | SupplementalOracleKind;

/** ✅️ Whether an oracle kind discharges an external-oracle requirement. */
export function isQualifyingOracleKind(kind: OracleKind | undefined): kind is QualifyingOracleKind {
  return kind !== undefined && (QUALIFYING_ORACLE_KINDS as readonly string[]).includes(kind);
}

/**
 * 🚫️ `s.stdio.*` names a real interchange format this repository DECODES — a genuine third party can,
 * in principle, implement one, so it always owes a real `third-party-library` / `third-party-cli` /
 * `standards-reference-tool` reference. `s.stdio.semio` is the one exception: it IS the format this
 * repository defines, so no vendor implements it by construction. This is the categorical boundary
 * `verified-native-second-implementation` may never cross — encoded here as the actual check
 * `nativeSecondImplementationBreaches` runs, not merely documented in a comment beside it.
 */
export function isSemioNativeArtifact(artifact: string): boolean {
  return artifact.length > 0 && !(artifact.startsWith("s.stdio.") && artifact !== "s.stdio.semio");
}

/**
 * 🌱️ The structured, machine-checked evidence a `verified-native-second-implementation` oracle entry
 * must carry. Every field is required; a field this function cannot itself verify (`specificationSource`,
 * the survey's `reason` prose) is still required to be PRESENT and non-empty rather than omitted — an
 * unfalsifiable claim recorded in the open is a far better failure mode than a silent gap, which is
 * exactly the failure this rule family exists to prevent.
 */
export type NativeSecondImplementationEvidence = Readonly<{
  /** 🎯️ The `mutationManifests[].artifact` id this claims native status for — cross-checked against `isSemioNativeArtifact` and against a real manifest this same contribution owns. */
  format: string;
  /** 🔍️ The recorded negative search: what was checked, and why each candidate considered was declined. */
  noThirdPartySurvey: Readonly<{ ecosystemsSearched: readonly string[]; candidatesConsidered: readonly Readonly<{ package: string; reason: string }>[] }>;
  /** 🗣️ The production subject's own implementation language — this reference is checked to differ from it. */
  subjectImplementationLanguage: string;
  /** 🗣️ The language this reference is actually written in. */
  secondImplementationLanguage: string;
  /** 📖️ The written specification this was authored FROM — never the subject's own source. */
  specificationSource: string;
  /** 🧫️ Fixture-backed vector evidence: how many committed vectors, and which capabilities they exercise. */
  fixtureCoverage: Readonly<{ vectors: number; capabilitiesCovered: readonly string[] }>;
}>;

/** ⚙️ The kernel a reference or a probe actually sits on — the unit independence is accounted in. */
export type EngineFamily = Readonly<{ family: string; implementation: string; version: string }>;

/** ⚙️ Stable identity of one engine family, ignoring the wrapper that exposes it. */
export function engineFamilyId(engine: EngineFamily | undefined): string {
  return engine === undefined ? "unknown" : engine.family;
}

/** 🎯️ What a mutation needs from an external reference before it may be registered or released. */
export type OracleRequirement = Readonly<{ capability: string; qualifyingKind: QualifyingOracleKind; distinctEngineFamilies?: number }>;

/** 🦠️ One mutation as its domain owner declares it — the single source feeding production and tests. */
export type ManifestMutation = Readonly<{
  id: string;
  capability: string;
  subset?: string;
  compound?: readonly string[];
  selectorSchema?: string;
  payloadSchema?: string;
  resultSchema?: string;
  outcomes: readonly MutationOutcomeClass[];
  productionDispatch: Readonly<{ operation: string; bridgeVersion: number; variant?: string }>;
  oracleRequirements: readonly OracleRequirement[];
  invariants?: Readonly<{ local?: readonly string[]; enclosing?: readonly string[] }>;
  normativeTopologyCounts?: boolean;
}>;

/** 🧬️ One owner's authoritative mutation inventory for one artifact/standard/subset. */
export type MutationManifest = Readonly<{
  schema: "semio.repository-test.mutation-manifest/v2";
  artifact: string;
  standard: string;
  subset: string;
  standardDirectoryName?: string;
  subsetDirectoryName?: string;
  mutations: readonly ManifestMutation[];
}>;

/** 🏭️ What production dispatch actually offers, emitted by running production code — never by parsing it. */
export type RuntimeMutationInventory = Readonly<{
  schema: "semio.repository-test.runtime-inventory/v2";
  artifact: string;
  standard: string;
  subset: string;
  bridgeVersion: number;
  producedBy?: string;
  mutations: readonly Readonly<{ id: string; variant: string; verb?: string; entity?: string; record?: string; outcomes: readonly MutationOutcomeClass[] }>[];
}>;

const MANIFEST_MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const CAPABILITY_ID_RE = /^[a-z0-9]+(?:[.-][a-z0-9]+)*$/;

/** 🧾️ Validates one manifest strictly. A manifest is the authority, so a malformed one is fatal. */
export function mutationManifestProblems(value: unknown, owner?: string): string[] {
  if (!isPlainObject(value)) return ["manifest is not an object"];
  const problems: string[] = [];
  if (value.schema !== "semio.repository-test.mutation-manifest/v2") problems.push('schema must be "semio.repository-test.mutation-manifest/v2"');
  for (const key of ["artifact", "standard", "subset"] as const) {
    if (typeof value[key] !== "string" || (value[key] as string).length === 0) problems.push(`${key} must be a non-empty string`);
  }
  const subset = typeof value.subset === "string" ? value.subset : "";
  // 🪆️Whether a wildcard SPELLING is actually a wildcard depends on what else the artifact declares,
  // which this record-level validator cannot see — so it is decided by `mutationInventoryBreaches`,
  // which can. Refusing the spelling here made a manifest unrepresentable for the 82 artifacts whose
  // only declared subset IS `*`, and an unrepresentable manifest is an unmeasured vocabulary: exactly
  // the outcome v2 exists to remove.
  if (subset.length === 0) problems.push("subset must be a non-empty string");
  const coordinates = owner === undefined ? null : subsetCoordinatesOfOwner(owner);
  if (coordinates !== null) {
    if (typeof value.standardDirectoryName === "string" && value.standardDirectoryName !== coordinates.standardDirectoryName) problems.push("standardDirectoryName does not match the owner path");
    if (typeof value.subsetDirectoryName === "string" && value.subsetDirectoryName !== coordinates.subsetDirectoryName) problems.push("subsetDirectoryName does not match the owner path");
    if (typeof value.standard === "string" && value.standard !== coordinates.standard) problems.push(`standard ${JSON.stringify(value.standard)} does not match the owner path standard ${JSON.stringify(coordinates.standard)}`);
  }
  if (!Array.isArray(value.mutations) || value.mutations.length === 0) {
    problems.push("mutations must be a non-empty array");
    return problems;
  }
  const seen = new Set<string>();
  for (const [index, raw] of value.mutations.entries()) {
    if (!isPlainObject(raw)) {
      problems.push(`mutations[${index}] is not an object`);
      continue;
    }
    const id = typeof raw.id === "string" ? raw.id : "";
    if (!MANIFEST_MUTATION_ID_RE.test(id)) problems.push(`mutations[${index}].id must be kebab-case`);
    if (seen.has(id)) problems.push(`mutations[${index}].id ${id} is duplicated — a mutation has exactly one owner`);
    seen.add(id);
    if (typeof raw.capability !== "string" || !CAPABILITY_ID_RE.test(raw.capability)) problems.push(`mutations[${index}].capability must be a dotted/kebab id`);
    if (raw.compound !== undefined && (!Array.isArray(raw.compound) || raw.compound.length < 2)) problems.push(`mutations[${index}].compound must name at least two subsets`);
    if (!Array.isArray(raw.outcomes) || raw.outcomes.length === 0 || raw.outcomes.some((outcome) => !(MUTATION_OUTCOME_CLASSES as readonly string[]).includes(String(outcome)))) {
      problems.push(`mutations[${index}].outcomes must be a non-empty array of ${MUTATION_OUTCOME_CLASSES.join("|")}`);
    } else if (new Set(raw.outcomes as string[]).size !== (raw.outcomes as string[]).length) problems.push(`mutations[${index}].outcomes must be unique`);
    if (!isPlainObject(raw.productionDispatch) || typeof raw.productionDispatch.operation !== "string" || typeof raw.productionDispatch.bridgeVersion !== "number") {
      problems.push(`mutations[${index}].productionDispatch must declare operation and bridgeVersion`);
    }
    if (!Array.isArray(raw.oracleRequirements) || raw.oracleRequirements.length === 0) {
      problems.push(`mutations[${index}].oracleRequirements must name at least one qualifying capability`);
    } else {
      for (const [requirementIndex, requirement] of raw.oracleRequirements.entries()) {
        if (!isPlainObject(requirement)) {
          problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}] is not an object`);
          continue;
        }
        if (typeof requirement.capability !== "string" || !CAPABILITY_ID_RE.test(requirement.capability)) problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}].capability must be a dotted/kebab id`);
        if (!(QUALIFYING_ORACLE_KINDS as readonly string[]).includes(String(requirement.qualifyingKind))) problems.push(`mutations[${index}].oracleRequirements[${requirementIndex}].qualifyingKind must be one of ${QUALIFYING_ORACLE_KINDS.join("|")}`);
      }
    }
  }
  return problems;
}

/** 🪆️ The subset one manifest mutation is owned by — its own override, else the manifest's. */
export function owningSubsetOf(manifest: MutationManifest, mutation: ManifestMutation): string {
  return mutation.subset ?? manifest.subset;
}

/** 🎯️ The full subset target of one manifest mutation. */
export function manifestTarget(manifest: MutationManifest, mutation: ManifestMutation): SubsetTarget {
  return { artifact: manifest.artifact, standard: manifest.standard, subset: owningSubsetOf(manifest, mutation), compound: mutation.compound };
}

/** #⃣ Content digest of one manifest — part of every run key, so a manifest edit invalidates caches. */
export function mutationManifestDigest(manifest: MutationManifest): string {
  return digest(JSON.stringify(canonicalize(manifest)));
}

/** 🧬️ Every manifest in the registry that owns a mutation with this id, for duplicate-owner detection. */
export function manifestsOwning(registry: OracleRegistry, mutationId: string): { manifest: MutationManifest; mutation: ManifestMutation }[] {
  const found: { manifest: MutationManifest; mutation: ManifestMutation }[] = [];
  for (const manifest of registry.mutationManifests) for (const mutation of manifest.mutations) if (mutation.id === mutationId) found.push({ manifest, mutation });
  return found;
}

/**
 * 🏭️ Where a generated runtime inventory is cached. A subset's inventory is produced by RUNNING the
 * production bridge; it is cache state, never committed source, so a stale checked-in copy can never
 * be mistaken for what the runtime offers today.
 */
export function runtimeInventoryPath(repoRoot: string, target: Pick<SubsetTarget, "artifact" | "standard" | "subset">): string {
  return join(testCacheDir(repoRoot, "results"), "🏭️inventory", `${target.artifact}@${target.standard}@${target.subset}.json`.replace(/[^A-Za-z0-9@._-]+/g, "_"));
}

/** 🏭️ Reads a generated runtime inventory, or `null` when the bridge has not been run for that subset. */
export function readRuntimeInventory(repoRoot: string, target: Pick<SubsetTarget, "artifact" | "standard" | "subset">): RuntimeMutationInventory | null {
  const path = runtimeInventoryPath(repoRoot, target);
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as RuntimeMutationInventory;
    return parsed.schema === "semio.repository-test.runtime-inventory/v2" ? parsed : null;
  } catch {
    return null;
  }
}

/** 🏭️ Writes a bridge-produced runtime inventory into the cache. */
export function writeRuntimeInventory(repoRoot: string, inventory: RuntimeMutationInventory): string {
  const path = runtimeInventoryPath(repoRoot, inventory);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(inventory, null, 2)}\n`);
  return path;
}

/** 🧾️ The three-way difference the equality gate reports on. Every non-empty field is a hard failure. */
export type InventoryEquality = Readonly<{
  target: string;
  runtimeOnly: readonly string[];
  manifestOnly: readonly string[];
  testOnly: readonly string[];
  outcomeMismatches: readonly { mutation: string; runtime: readonly string[]; manifest: readonly string[] }[];
  variantMismatches: readonly { mutation: string; runtime: string; manifest: string }[];
  runtimeMissing: boolean;
}>;

/**
 * 🧾️ Compares production dispatch, the owner manifest and the claimed test inventory EXACTLY. This is
 * what closes v1's blind spot: a mutation can be reachable in production and absent from the catalog
 * without any v1 gate proving the omission, because v1 never consulted dispatch at all.
 */
export function compareInventories(manifest: MutationManifest, runtime: RuntimeMutationInventory | null, claimedTestKinds: readonly string[]): InventoryEquality {
  const target = subsetCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset: manifest.subset });
  const manifestIds = new Set(manifest.mutations.map((mutation) => mutation.id));
  const testIds = new Set(claimedTestKinds);
  if (runtime === null) {
    return { target, runtimeOnly: [], manifestOnly: [], testOnly: [...testIds].filter((id) => !manifestIds.has(id)).sort(), outcomeMismatches: [], variantMismatches: [], runtimeMissing: true };
  }
  const runtimeIds = new Set(runtime.mutations.map((mutation) => mutation.id));
  const outcomeMismatches: { mutation: string; runtime: readonly string[]; manifest: readonly string[] }[] = [];
  const variantMismatches: { mutation: string; runtime: string; manifest: string }[] = [];
  for (const mutation of manifest.mutations) {
    const runtimeRow = runtime.mutations.find((row) => row.id === mutation.id);
    if (runtimeRow === undefined) continue;
    const runtimeOutcomes = [...runtimeRow.outcomes].sort();
    const manifestOutcomes = [...mutation.outcomes].sort();
    if (JSON.stringify(runtimeOutcomes) !== JSON.stringify(manifestOutcomes)) outcomeMismatches.push({ mutation: mutation.id, runtime: runtimeOutcomes, manifest: manifestOutcomes });
    const declaredVariant = mutation.productionDispatch.variant;
    if (declaredVariant !== undefined && declaredVariant !== runtimeRow.variant) variantMismatches.push({ mutation: mutation.id, runtime: runtimeRow.variant, manifest: declaredVariant });
  }
  return {
    target,
    runtimeOnly: [...runtimeIds].filter((id) => !manifestIds.has(id)).sort(),
    manifestOnly: [...manifestIds].filter((id) => !runtimeIds.has(id)).sort(),
    testOnly: [...testIds].filter((id) => !manifestIds.has(id)).sort(),
    outcomeMismatches,
    variantMismatches,
    runtimeMissing: false,
  };
}
//#endregion 🧬️Manifest

//#region 🧫️Fixture
/** 🧫️ What a fixture IS. Every fixture is exactly one of these — there is no fourth, unlabelled kind. */
export const FIXTURE_CLASSES = ["real-world", "handcrafted", "third-party-generated"] as const;
export type FixtureClass = (typeof FIXTURE_CLASSES)[number];

/** 🧫️ One file inside a fixture bundle, addressed by ROLE so nothing downstream names a path. */
export type FixtureFile = Readonly<{ role: string; path: string; mediaType: string; sha256: string; bytes?: number }>;

/** 🏭️ Exactly how a third-party-generated fixture was produced, in enough detail to re-run it. */
export type FixtureGenerator = Readonly<{ oracle: string; packageVersion: string; engineFamily: string; engineVersion: string; command: string; seed?: string | number; platform: PlatformId; sourceDigest?: string }>;

/** 📜️ Where a fixture came from and under what licence it may be committed. */
export type FixtureProvenance = Readonly<{ source: "generated" | "authored" | "downloaded" | "vendored"; license: string; acquiredAt?: string; attribution?: string; url?: string; security?: "scanned-clean" | "unscanned" | "quarantined"; privacy?: "no-personal-data" | "reviewed" | "unreviewed" }>;

/** 📐️ The units and frame a fixture's numbers are expressed in. Absent units make every metric meaningless. */
export type FixtureUnits = Readonly<{ length: string; angle: string; handedness?: "right" | "left"; up?: "y" | "z" }>;

/** ⚠️ A fixture-level loosening of its tolerance profile — every field mandatory, and always reported. */
export type ToleranceOverride = Readonly<{ reason: string; measuredBaseline: number; factor: number; approvedBy: string }>;

/** 🧫️ One immutable fixture bundle with complete provenance. */
export type FixtureManifest = Readonly<{
  schema: "semio.repository-test.fixture/v2";
  id: string;
  class: FixtureClass;
  target: SubsetTarget;
  mutation?: string;
  outcome?: MutationOutcomeClass;
  units: FixtureUnits;
  files: readonly FixtureFile[];
  generator?: FixtureGenerator;
  provenance: FixtureProvenance;
  comparisonProfile: string;
  toleranceProfile?: string;
  toleranceOverride?: ToleranceOverride;
  reproducible: boolean;
  family?: string;
  notes?: string;
  /** 📁️ Repo-relative directory the manifest was read from; `files[].path` resolves against it. */
  manifestDir?: string;
}>;

const SHA256_RE = /^sha256:[0-9a-f]{64}$/;

/** #⃣ The canonical `sha256:<hex>` digest of a file's bytes, as every manifest and blob id spells it. */
export function contentDigest(bytes: Uint8Array): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

/** #⃣ The canonical content digest of a file on disk. */
export function contentDigestOf(absPath: string): string {
  return contentDigest(readFileSync(absPath));
}

/** 🧾️ Validates one fixture manifest strictly. A fixture with incomplete provenance is a contract failure. */
export function fixtureManifestProblems(value: unknown, repoRoot?: string): string[] {
  if (!isPlainObject(value)) return ["fixture manifest is not an object"];
  const problems: string[] = [];
  if (value.schema !== "semio.repository-test.fixture/v2") problems.push('schema must be "semio.repository-test.fixture/v2"');
  if (typeof value.id !== "string" || !MANIFEST_MUTATION_ID_RE.test(value.id)) problems.push("id must be kebab-case");
  if (!(FIXTURE_CLASSES as readonly string[]).includes(String(value.class))) problems.push(`class must be one of ${FIXTURE_CLASSES.join("|")}`);
  if (!isPlainObject(value.target)) problems.push("target must name artifact, standard and subset");
  else {
    for (const key of ["artifact", "standard", "subset"] as const) if (typeof (value.target as Record<string, unknown>)[key] !== "string") problems.push(`target.${key} must be a string`);
    const subset = String((value.target as Record<string, unknown>).subset ?? "");
    // 🪆️Settled the same way a MUTATION's scope is, when the repository is available to ask. A subset
    // literally named `any` is a hard breach only when the artifact has real sibling subsets to be
    // scoped against; an artifact with exactly one subset is not "everything", it is a naming choice,
    // and `subsetPolicy: "single"` settles it. Judging fixtures by the bare spelling while judging
    // mutations by the resolved rule made 27 fixtures of genuinely single-subset owners unregisterable.
    const artifact = String((value.target as Record<string, unknown>).artifact ?? "");
    const standard = String((value.target as Record<string, unknown>).standard ?? "");
    const wildcard = repoRoot === undefined ? isWildcardSubset(subset) : isWildcardSubsetFor(repoRoot, artifact, standard, subset);
    if (wildcard) problems.push(`target.subset ${JSON.stringify(subset)} is a wildcard`);
  }
  if (value.outcome !== undefined && !(MUTATION_OUTCOME_CLASSES as readonly string[]).includes(String(value.outcome))) problems.push(`outcome must be one of ${MUTATION_OUTCOME_CLASSES.join("|")}`);
  if (!isPlainObject(value.units) || typeof value.units.length !== "string" || typeof value.units.angle !== "string") problems.push("units must declare length and angle");
  if (!Array.isArray(value.files) || value.files.length === 0) problems.push("files must be a non-empty array");
  else {
    const roles = new Set<string>();
    for (const [index, file] of value.files.entries()) {
      if (!isPlainObject(file)) {
        problems.push(`files[${index}] is not an object`);
        continue;
      }
      for (const key of ["role", "path", "mediaType"] as const) if (typeof file[key] !== "string" || (file[key] as string).length === 0) problems.push(`files[${index}].${key} must be a non-empty string`);
      if (typeof file.sha256 !== "string" || !SHA256_RE.test(file.sha256)) problems.push(`files[${index}].sha256 must be sha256:<64 hex>`);
      const role = String(file.role ?? "");
      if (roles.has(role)) problems.push(`files[${index}].role ${role} is duplicated — a role addresses exactly one file`);
      roles.add(role);
    }
  }
  if (!isPlainObject(value.provenance) || typeof value.provenance.license !== "string" || (value.provenance.license as string).length === 0) problems.push("provenance.license is required — a fixture without an acceptable licence is a contract failure, not an undocumented exception");
  if (typeof value.comparisonProfile !== "string" || value.comparisonProfile.length === 0) problems.push("comparisonProfile must be a non-empty string");
  if (typeof value.reproducible !== "boolean") problems.push("reproducible must be a boolean");
  if (value.class === "third-party-generated") {
    if (!isPlainObject(value.generator)) problems.push("a third-party-generated fixture must record its generator");
    else for (const key of ["oracle", "packageVersion", "engineFamily", "engineVersion", "command", "platform"] as const) {
      if (typeof (value.generator as Record<string, unknown>)[key] !== "string" || String((value.generator as Record<string, unknown>)[key]).length === 0) problems.push(`generator.${key} must be a non-empty string`);
    }
  }
  if (value.toleranceOverride !== undefined) {
    const override = value.toleranceOverride;
    if (!isPlainObject(override)) problems.push("toleranceOverride must be an object");
    else {
      if (typeof override.reason !== "string" || override.reason.length < 20) problems.push("toleranceOverride.reason must state WHY in at least 20 characters");
      if (typeof override.measuredBaseline !== "number") problems.push("toleranceOverride.measuredBaseline must be the measured value the override is sized against");
      if (typeof override.factor !== "number" || override.factor < 1) problems.push("toleranceOverride.factor must be at least 1");
      if (typeof override.approvedBy !== "string" || override.approvedBy.length === 0) problems.push("toleranceOverride.approvedBy must name the approving owner");
    }
  }
  return problems;
}

/** 🧫️ Absolute path of one role inside a fixture bundle. */
export function fixtureFilePath(repoRoot: string, manifest: FixtureManifest, role: string): string {
  const file = manifest.files.find((entry) => entry.role === role);
  if (file === undefined) throw new Error(`fixture ${manifest.id} has no file for role ${JSON.stringify(role)}`);
  return isAbsolute(file.path) ? file.path : join(repoRoot, manifest.manifestDir ?? "", file.path);
}

/** 🧾️ One verified fixture file. `expected`/`actual` differ exactly when the committed bytes changed. */
export type FixtureVerification = Readonly<{ fixture: string; role: string; path: string; expected: string; actual: string; ok: boolean; missing: boolean }>;

/** 🧾️ Re-hashes every file of a fixture against its manifest. Source fixtures are immutable after review. */
export function verifyFixture(repoRoot: string, manifest: FixtureManifest): FixtureVerification[] {
  return manifest.files.map((file) => {
    const abs = isAbsolute(file.path) ? file.path : join(repoRoot, manifest.manifestDir ?? "", file.path);
    if (!existsSync(abs)) return { fixture: manifest.id, role: file.role, path: file.path, expected: file.sha256, actual: "", ok: false, missing: true };
    const actual = contentDigestOf(abs);
    return { fixture: manifest.id, role: file.role, path: file.path, expected: file.sha256, actual, ok: actual === file.sha256, missing: false };
  });
}

/** #⃣ Order-independent digest of every file in a fixture bundle — one component of the run key. */
export function fixtureBundleDigest(manifest: FixtureManifest): string {
  return setDigest(manifest.files.map((file) => [file.role, file.sha256] as const));
}
//#endregion 🧫️Fixture

//#region 🪪️LeafDescriptor
/**
 * 🪪️ One mutation leaf's own declaration, the fourteen-field record the `dsl::Mutations` derive reads
 * at expansion time from `🧬️mutations/<kind>/<taxonomy json filename>`.
 *
 * This is the single source Protocol v2 wanted and could not previously reach. It is DECLARATIVE and
 * LANGUAGE-NEUTRAL — a JSON file beside the Rust, read by the derive to generate production
 * registration — so a manifest built from it is generated from the same record production is, rather
 * than restated beside it. Crucially it carries `outcomeClasses`, which is the one field a manifest
 * cannot honestly invent: only the implementation knows which classes a mutation can reach.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs — `parse_mutation_leaf_descriptor`
 */
export type MutationLeafDescriptor = Readonly<{
  schemaVersion: number;
  owner: string;
  semanticKind: string;
  displayName: string;
  emoji: string;
  aggregateVariant: string;
  payloadSchema: string;
  textOpcode: string | null;
  binaryTag: number | null;
  invertibility: "self" | "explicit-mutation" | "plan" | "non-invertible";
  diffParticipation: "detect" | "apply-only" | "plan" | "none";
  outcomeClasses: readonly string[];
  composition: "atomic" | "composite";
  requiredLanguageSurfaces: readonly string[];
}>;

/** 🎯️ Maps the implementation's outcome vocabulary onto the protocol's declared classes. */
export function outcomeClassesOf(descriptor: MutationLeafDescriptor): MutationOutcomeClass[] {
  // 🎯️`Info`/`Warning` are DIAGNOSTIC severities on an outcome, not outcome classes — a mutation that
  // applies with a warning still applied. `Error`/`Fatal` are the refusal. Collapsing them here is the
  // one piece of vocabulary translation between the two records, and it is done in one place.
  const mapped = new Set<MutationOutcomeClass>();
  for (const raw of descriptor.outcomeClasses) {
    const value = raw.toLowerCase();
    if (value === "applied" || value === "info" || value === "warning") mapped.add("applied");
    else if (value === "error" || value === "fatal" || value === "rejected") mapped.add("rejected");
    else if ((MUTATION_OUTCOME_CLASSES as readonly string[]).includes(value)) mapped.add(value as MutationOutcomeClass);
  }
  if (mapped.size === 0) mapped.add("applied");
  return [...mapped];
}

/** 🪪️ Every mutation leaf descriptor beneath one owner, keyed by its semantic kind. */
/**
 * 🦠️ Whether a directory under `🧬️mutations` is a MUTATION LEAF rather than a shared facet.
 *
 * `💾️binary`, `📝️text` and `🧬️schema` live beside the leaves and are not leaves. Counting them as such
 * made every owner look permanently undescribed — 21 owners whose every real leaf carried a descriptor
 * were reported as 4, because three facet directories could never have one.
 */
export function isMutationLeafDirectory(repoRoot: string, name: string): boolean {
  const pattern = (testTaxonomy(repoRoot) as unknown as { mutationDirectoryPattern?: string }).mutationDirectoryPattern;
  if (typeof pattern === "string") {
    try {
      return new RegExp(pattern, "u").test(name);
    } catch {
      /* 🧭️Fall through to the structural rule below. */
    }
  }
  return /[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/.test(name) && !["💾️binary", "📝️text", "🧬️schema"].includes(name);
}

export function readLeafDescriptors(repoRoot: string, owner: string): Map<string, MutationLeafDescriptor> {
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const root = join(repoRoot, owner, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  const found = new Map<string, MutationLeafDescriptor>();
  if (!existsSync(root)) return found;
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.isSymbolicLink() || !isMutationLeafDirectory(repoRoot, entry.name)) continue;
    const descriptorPath = join(root, entry.name, filename);
    if (!existsSync(descriptorPath)) continue;
    try {
      const parsed = JSON.parse(readFileSync(descriptorPath, "utf8")) as MutationLeafDescriptor;
      if (typeof parsed.semanticKind === "string" && Array.isArray(parsed.outcomeClasses)) found.set(parsed.semanticKind, parsed);
    } catch {
      /* 🧭️A malformed descriptor is the derive's own gate to report; it fails the build. */
    }
  }
  return found;
}

/** 🪪️ How many of an owner's mutation leaves carry a descriptor — the ratio that gates generation. */
export function leafDescriptorCoverage(repoRoot: string, owner: string): { leaves: number; described: number; missing: string[] } {
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const root = join(repoRoot, owner, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  if (!existsSync(root)) return { leaves: 0, described: 0, missing: [] };
  const directories = readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink() && isMutationLeafDirectory(repoRoot, entry.name)).map((entry) => entry.name);
  const missing = directories.filter((name) => !existsSync(join(root, name, filename)));
  return { leaves: directories.length, described: directories.length - missing.length, missing };
}

/**
 * 🧬️ Builds a v2 mutation manifest for one owner FROM ITS LEAF DESCRIPTORS.
 *
 * Returns `null` when any leaf lacks a descriptor. That refusal is the whole point: a manifest whose
 * outcome classes were guessed for even one mutation would report coverage of an outcome nobody can
 * reach, or hide one everybody can — and a partial manifest is worse than none, because the gate would
 * then measure against a denominator that silently omits the undescribed leaves.
 */
export function manifestFromLeafDescriptors(repoRoot: string, owner: string, capability: string): MutationManifest | null {
  const coordinates = subsetCoordinatesOfOwner(owner);
  if (coordinates === null) return null;
  const coverage = leafDescriptorCoverage(repoRoot, owner);
  if (coverage.leaves === 0 || coverage.missing.length > 0) return null;
  const descriptors = readLeafDescriptors(repoRoot, owner);
  if (descriptors.size !== coverage.leaves) return null;
  const artifact = artifactOfOwner(repoRoot, owner);
  if (artifact === null) return null;
  return {
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact,
    standard: coordinates.standard,
    subset: coordinates.subset,
    standardDirectoryName: coordinates.standardDirectoryName,
    subsetDirectoryName: coordinates.subsetDirectoryName,
    mutations: [...descriptors.values()]
      .sort((a, b) => a.semanticKind.localeCompare(b.semanticKind))
      .map((descriptor) => ({
        id: descriptor.semanticKind,
        capability,
        payloadSchema: descriptor.payloadSchema,
        outcomes: outcomeClassesOf(descriptor),
        productionDispatch: { operation: descriptor.textOpcode ?? descriptor.semanticKind, bridgeVersion: 1, variant: descriptor.aggregateVariant },
        oracleRequirements: [{ capability, qualifyingKind: "third-party-library" as const }],
      })),
  };
}

/** 🗿️ The artifact id an owner path belongs to, read from its own 🪆️subsets component. */
export function artifactOfOwner(repoRoot: string, owner: string): string | null {
  const marker = owner.indexOf("/🏅️standards/");
  if (marker < 0) return null;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const subsetsRoot = join(repoRoot, owner.slice(0, owner.lastIndexOf("/🪆️subsets/") + "/🪆️subsets".length));
  for (const candidate of [filename, "🔣️.json"]) {
    const manifestPath = join(subsetsRoot, candidate);
    if (!existsSync(manifestPath)) continue;
    try {
      const parsed = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: string };
      if (typeof parsed.artifact === "string") return parsed.artifact;
    } catch {
      return null;
    }
  }
  return null;
}
//#endregion 🪪️LeafDescriptor

//#region 🏗️LeafScaffold
/** 🏗️ One derived descriptor field, with the file and line that justifies it. */
export type DerivedField = Readonly<{ value: unknown; evidence: string }>;

/** 🏗️ A scaffolded descriptor, or the exact reason one field could not be derived. */
export type LeafScaffold = Readonly<{ leaf: string; kind: string; descriptor: MutationLeafDescriptor | null; evidence: Readonly<Record<string, string>>; refused: readonly string[] }>;

function readFirst(paths: readonly string[]): { path: string; text: string } | null {
  for (const path of paths) {
    if (!existsSync(path)) continue;
    try {
      return { path, text: readFileSync(path, "utf8") };
    } catch {
      continue;
    }
  }
  return null;
}

function titleCase(kind: string): string {
  return kind.split("-").map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join(" ");
}

function pascalCase(kind: string): string {
  return kind.split("-").map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join("");
}

/**
 * 🏗️ Derives one mutation leaf's descriptor FROM THE LEAF ITSELF, and refuses any field it cannot
 * justify with a file and a line.
 *
 * The descriptor is the single source a v2 manifest is generated from, and its `outcomeClasses` is the
 * one field nobody can honestly invent from outside the implementation — so this does not invent it.
 * It reads the leaf's own `🔺️diff`, where the semantics actually live: `MutationOutcome::error(…)` is a
 * REJECTED path, `MutationOutcome::empty()` is a NO-OP path, and `MutationOutcome::new(…)` is an
 * APPLIED path. Same for the rest: the aggregate variant comes from the `pub struct` the leaf declares,
 * the binary tag from the `record <kind> tag=<n>` line in the owner's binary protocol, the text opcode
 * from the `#[dsl(keyword = "…")]` attribute. A field with no such line is REFUSED and the leaf is
 * reported, never guessed — a scaffolded descriptor that guessed one field would put a wrong outcome
 * class into production registration AND into the coverage denominator at the same time.
 */
export function scaffoldLeafDescriptor(repoRoot: string, ownerRel: string, leafDirName: string): LeafScaffold {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  const leafAbs = join(vocabulary, leafDirName);
  const leafRel = `${ownerRel}/🧬️schema/${taxonomy.testMutationVocabularyDirName}/${leafDirName}`;
  const kind = leafDirName.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)?.[0] ?? "";
  const emoji = kind.length > 0 ? leafDirName.slice(0, leafDirName.length - kind.length) : "";
  const evidence: Record<string, string> = {};
  const refused: string[] = [];

  if (kind.length === 0) refused.push("semanticKind: the directory name does not end in a two-or-more-segment kebab identifier");
  if (emoji.length === 0) refused.push("emoji: the directory name carries no leading emoji");

  // 🦀️The payload struct, and therefore the aggregate variant, is whatever the leaf actually declares.
  // 🧭️`🧬️operation` is the same payload directory under a rename a concurrent session is applying; both
  // spellings are READ here so a half-migrated tree is still measurable. Nothing is written under either.
  const mutationSource = readFirst([join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🦀️.rs"), join(leafAbs, "🦀️.rs")]);
  let aggregateVariant = "";
  if (mutationSource === null) refused.push("aggregateVariant: the leaf declares no Rust source");
  else {
    // 📄️`set-snapshot` is the whole-document replace, and its payload IS the artifact's snapshot type —
    // there is no dedicated struct to find, which is why all 50 of these refused. The type is stated in
    // the leaf's own apply signature (`apply(projection: &mut DwgSnapshot, …)`), so it is read from
    // there rather than assumed, and the variant follows the name the aggregate already uses for it.
    const snapshotApply = mutationSource.text.match(/fn apply\s*\(\s*[a-z_]+\s*:\s*&mut\s+([A-Za-z0-9_]+)/)?.[1] ?? "";
    const declared = mutationSource.text.match(/^pub struct ([A-Za-z0-9_]+)/m)?.[1] ?? mutationSource.text.match(/^pub enum ([A-Za-z0-9_]+)/m)?.[1] ?? (kind === "set-snapshot" && snapshotApply.length > 0 ? "SetSnapshot" : "");
    if (declared.length === 0) refused.push(`aggregateVariant: no \`pub struct\` in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`);
    else {
      aggregateVariant = declared;
      evidence.aggregateVariant = `pub struct ${declared} in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`;
      if (declared !== pascalCase(kind)) evidence.aggregateVariant += ` (note: differs from the PascalCase of ${kind}, so the declaration wins)`;
    }
  }

  // 🎯️OUTCOME CLASSES, from the diff implementation. This is the field the whole exercise exists for.
  // 🔺️The diff is not always a SUBDIRECTORY. In the taxonomy's canonical leaf layout — the one the
  // `MutationLeaf` derive requires, and the one this migration produces — payload, diff and inverse all
  // live in the leaf's single `🦀️.rs`. 315 leaves are already in that shape, and reading only
  // `🔺️diff/` refused every one of them for evidence that was sitting in the file next door.
  const diffSource = readFirst([join(leafAbs, "🔺️diff", "🦀️.rs"), join(leafAbs, "🔺️diff", "🦀️.rs"), join(leafAbs, "🦀️.rs"), join(leafAbs, "🦀️.rs")]);
  const outcomes = new Set<string>();
  if (diffSource === null) refused.push("outcomeClasses: the leaf declares no 🔺️diff implementation to read them from");
  else {
    const lines = diffSource.text.split(/\r?\n/);
    const cite: string[] = [];
    for (const [index, line] of lines.entries()) {
      if (/MutationOutcome::error\s*\(/.test(line)) {
        outcomes.add("error");
        cite.push(`error@${index + 1}`);
      }
      if (/MutationOutcome::empty\s*\(/.test(line)) {
        outcomes.add("info");
        cite.push(`empty@${index + 1}`);
      }
      if (/MutationOutcome::new\s*\(/.test(line)) {
        outcomes.add("applied");
        cite.push(`new@${index + 1}`);
      }
    }
    if (outcomes.size === 0) refused.push(`outcomeClasses: no MutationOutcome:: call in ${relative(repoRoot, diffSource.path).split(sep).join("/")}`);
    else evidence.outcomeClasses = `${relative(repoRoot, diffSource.path).split(sep).join("/")} — ${cite.join(", ")}`;
  }

  // 🎯️SECOND EVIDENCE SOURCE, used when the leaf has no 🔺️diff to read: the committed mutation vectors.
  // Every scenario bundle records what the implementation ACTUALLY produced in its `🎯️outcome`, so the
  // union across a leaf's scenarios is a lower bound on the classes it reaches — observed behaviour
  // rather than inferred behaviour. It is a LOWER bound and the evidence string says so, because a
  // class no committed scenario exercises is invisible here.
  if (outcomes.size === 0) {
    const scenarios = join(leafAbs, taxonomy.testsDirName);
    if (existsSync(scenarios)) {
      const observed: string[] = [];
      for (const scenario of readdirSync(scenarios, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink())) {
        const outcome = readFirst([join(scenarios, scenario.name, "🎯️outcome", "🔣️.json"), join(scenarios, scenario.name, "🎯️outcome", "🔣️.json")]);
        if (outcome === null) continue;
        try {
          const status = String((JSON.parse(outcome.text) as { status?: unknown }).status ?? "");
          if (status.length === 0) continue;
          const mapped = status === "applied" ? "applied" : status === "rejected" || status === "error" ? "error" : status === "no-op" || status === "empty" ? "info" : "";
          if (mapped.length > 0) {
            outcomes.add(mapped);
            observed.push(`${scenario.name}=${status}`);
          }
        } catch {
          /* 🧭️A malformed vector is the vector audit's finding, not this one's. */
        }
      }
      if (observed.length > 0) {
        const index = refused.findIndex((why) => why.startsWith("outcomeClasses"));
        if (index >= 0) refused.splice(index, 1);
        evidence.outcomeClasses = `LOWER BOUND from ${observed.length} committed vector(s): ${observed.slice(0, 4).join(", ")} — the leaf has no 🔺️diff, so this records observed outcomes only and a class no scenario exercises is not represented`;
      }
    }
  }

  // 🔢️The binary tag is declared once, in the owner's binary protocol, and must stay that number.
  const binaryProtocol = readFirst([join(vocabulary, "💾️binary", "📡️.protocol.semio"), join(vocabulary, "💾️binary", "📡️.protocol.semio")]);
  let binaryTag: number | null = null;
  if (binaryProtocol !== null) {
    const lines = binaryProtocol.text.split(/\r?\n/);
    const line = lines.findIndex((candidate) => new RegExp(`record\\s+${kind}\\s+tag=(\\d+)`).test(candidate));
    if (line >= 0) {
      binaryTag = Number(lines[line]!.match(/tag=(\d+)/)![1]);
      evidence.binaryTag = `${relative(repoRoot, binaryProtocol.path).split(sep).join("/")}:${line + 1}`;
    } else {
      // 🔢️A binary surface that does not carry this kind is STALE, and inventing a tag would put a
      // wire number into production registration that nothing on the wire agrees with. The CAD binary
      // protocol is exactly this case: it still lists the fourteen verbs retired in ticket
      // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 3 and none of the twenty that replaced them.
      // 🔢️A STALE wire protocol is real drift and is reported as such — every one of the hundred owners
      // that has a binary protocol is missing records, 1 361 kinds in total. But it is NOT a reason to
      // block this mutation from being testable: the descriptor contract permits a null `binaryTag`,
      // the two concerns are orthogonal, and conflating them would hold subset-scoped external-oracle
      // coverage hostage to a wire format nothing in the test path reads. The drift is surfaced by
      // `binaryProtocolDriftBreaches` instead, where it can be acted on without blocking anything.
      const declared = lines.filter((candidate) => /^record\s/.test(candidate)).length;
      evidence.binaryTag = `null — ${relative(repoRoot, binaryProtocol.path).split(sep).join("/")} declares ${declared} record(s) and none is ${JSON.stringify(kind)}; the wire protocol is stale for this vocabulary`;
    }
  }

  // 🔤️The text opcode is the DSL keyword the payload declares, not an assumption about its name.
  let textOpcode: string | null = null;
  if (mutationSource !== null) {
    const keyword = mutationSource.text.match(/#\[dsl\(keyword\s*=\s*"([a-z0-9-]+)"\)\]/)?.[1];
    if (keyword !== undefined) {
      textOpcode = keyword;
      evidence.textOpcode = `#[dsl(keyword = "${keyword}")] in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`;
      if (keyword !== kind) refused.push(`textOpcode: the DSL keyword ${JSON.stringify(keyword)} disagrees with the directory kind ${JSON.stringify(kind)}`);
    }
  }

  const hasInverse = existsSync(join(leafAbs, "↩️inverse"));
  const hasPlan = existsSync(join(leafAbs, "🧩️plan"));
  const hasText = existsSync(join(vocabulary, "📝️text"));
  const hasBinary = binaryProtocol !== null;
  // 🧬️Two payload-schema conventions are in the tree and both are legitimate: the descriptor-linked
  // `🧬️schema/<json>` beside the leaf, and the flat `🔣️.schema.json` the projection names as its
  // source filename. Whichever the leaf actually has is the one the descriptor points at.
  const schemaCandidates: readonly [string, string][] = [
    [`🧬️schema/${testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId)}`, join(leafAbs, "🧬️schema", testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId))],
    ["🔣️.schema.json", join(leafAbs, "🔣️.schema.json")],
  ];
  const payloadSchema = schemaCandidates.find(([, abs]) => existsSync(abs))?.[0] ?? "";
  const hasSchema = payloadSchema.length > 0;
  if (hasSchema) evidence.payloadSchema = `${payloadSchema} exists beside the leaf`;
  evidence.invertibility = hasPlan ? "🧩️plan facet present" : hasInverse ? "↩️inverse facet present" : "neither ↩️inverse nor 🧩️plan is present";
  evidence.diffParticipation = diffSource === null ? "no 🔺️diff facet" : "🔺️diff facet present";
  evidence.requiredLanguageSurfaces = `facets present: rust${hasSchema ? `, ${payloadSchema}` : ""}${hasText ? ", 📝️text" : ""}${hasBinary ? ", 💾️binary" : ""}`;

  // 🧬️The payload schema is a taxonomy LOCATION, not a guess: `mutationPayloadSchemaLocation` puts it
  // at `🧬️schema/<json>` beside the leaf. A leaf without one has no payload contract to point at.
  if (!hasSchema) refused.push(`payloadSchema: the leaf carries neither 🧬️schema/${testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId)} nor 🔣️.schema.json, so there is no payload contract to reference`);
  if (refused.length > 0) return { leaf: leafRel, kind, descriptor: null, evidence, refused };

  const surfaces = ["rust", ...(hasSchema ? ["json-schema"] : []), ...(hasText ? ["text"] : []), ...(hasBinary ? ["binary"] : [])];
  return {
    leaf: leafRel,
    kind,
    evidence,
    refused,
    descriptor: {
      schemaVersion: 1,
      owner: leafRel,
      semanticKind: kind,
      displayName: titleCase(kind),
      emoji,
      aggregateVariant,
      payloadSchema,
      textOpcode,
      binaryTag,
      invertibility: hasPlan ? "plan" : hasInverse ? "explicit-mutation" : "non-invertible",
      diffParticipation: diffSource === null ? "none" : "detect",
      outcomeClasses: [...outcomes],
      composition: "atomic",
      requiredLanguageSurfaces: surfaces,
    },
  };
}

/**
 * 🔢️ Where the binary wire protocol has fallen behind the mutation vocabulary it is supposed to carry.
 *
 * Reported separately from descriptor derivation on purpose: a stale wire format is a real defect and a
 * different one from "this mutation has no test", and blocking the second on the first would hold every
 * mutation hostage to a file nothing in the test path reads.
 */
export function binaryProtocolDriftBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const taxonomy = testTaxonomy(repoRoot);
  const breaches: BreachRecord[] = [];
  for (const owner of [...new Set(registry.contributions.map((entry) => entry.owner))]) {
    const vocabulary = join(repoRoot, owner, "🧬️schema", taxonomy.testMutationVocabularyDirName);
    if (!existsSync(vocabulary)) continue;
    const kinds = readdirSync(vocabulary, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && !entry.isSymbolicLink() && /[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/.test(entry.name))
      .map((entry) => entry.name.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)![0]);
    if (kinds.length === 0) continue;
    const protocol = readFirst([join(vocabulary, "💾️binary", "📡️.protocol.semio"), join(vocabulary, "💾️binary", "📡️.protocol.semio")]);
    if (protocol === null) continue;
    const declared = new Set([...protocol.text.matchAll(/^record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)/gm)].map((match) => match[1]!));
    const missing = kinds.filter((kind) => !declared.has(kind));
    const orphaned = [...declared].filter((kind) => !kinds.includes(kind));
    if (missing.length === 0 && orphaned.length === 0) continue;
    breaches.push(
      breach(
        "testing/contract",
        "binary-protocol-drift",
        relative(repoRoot, protocol.path).split(sep).join("/"),
        `${missing.length} mutation kind(s) have no wire record and ${orphaned.length} record(s) name a kind that no longer exists${orphaned.length > 0 ? `: ${orphaned.slice(0, 6).join(", ")}` : ""}`,
        "A wire protocol that has fallen behind its vocabulary cannot carry the mutations the implementation actually dispatches, and its orphaned records describe verbs nothing can emit.",
        "Regenerate the protocol from the current mutation leaves, assigning a tag to each new kind and removing the records whose kind is gone.",
        "medium",
      ),
    );
  }
  return breaches;
}

/** 🏗️ Scaffolds every leaf of one owner, so roster-level uniqueness can be checked before anything is written. */
export function scaffoldOwnerDescriptors(repoRoot: string, ownerRel: string): LeafScaffold[] {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  if (!existsSync(vocabulary)) return [];
  return readdirSync(vocabulary, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.isSymbolicLink() && isMutationLeafDirectory(repoRoot, entry.name))
    .map((entry) => scaffoldLeafDescriptor(repoRoot, ownerRel, entry.name))
    .sort((a, b) => a.kind.localeCompare(b.kind));
}
//#endregion 🏗️LeafScaffold

//#region 🧬️PayloadSchema
/** 🧬️ A derived payload schema, or the exact Rust type that defeated the derivation. */
export type PayloadSchemaDerivation = Readonly<{ leaf: string; kind: string; schema: Record<string, unknown> | null; struct: string; refused: readonly string[] }>;

const RUST_SCALARS: Readonly<Record<string, Record<string, unknown>>> = {
  String: { type: "string" },
  str: { type: "string" },
  bool: { type: "boolean" },
  u8: { type: "integer", minimum: 0 },
  u16: { type: "integer", minimum: 0 },
  u32: { type: "integer", minimum: 0 },
  u64: { type: "integer", minimum: 0 },
  usize: { type: "integer", minimum: 0 },
  i8: { type: "integer" },
  i16: { type: "integer" },
  i32: { type: "integer" },
  i64: { type: "integer" },
  isize: { type: "integer" },
  f32: { type: "number" },
  f64: { type: "number" },
};

/** 🐍️ serde's `rename_all = "camelCase"` applied to one snake_case field name. */
function camelCase(field: string): string {
  return field.replace(/_([a-z0-9])/g, (_, character: string) => character.toUpperCase());
}

/**
 * 🧬️ Maps ONE Rust type onto JSON Schema, or returns null when it is not a shape this can decide.
 *
 * Refusing an unknown type is the whole discipline. Emitting a permissive `{"type": "object"}` for a
 * type it does not understand would produce a payload contract that accepts anything — which is
 * indistinguishable, to every downstream gate, from a contract that was carefully written to accept
 * exactly the right thing.
 */

/** 🌱️ True when a named type is the JSON value model — the six variants, whatever it is called. */
function jsonValueShaped(name: string, resolve?: (candidate: string) => Record<string, unknown> | null): boolean {
  if (resolve === undefined || !/^[A-Z][A-Za-z0-9_]*$/.test(name)) return false;
  const body = jsonValueBodies.get(name);
  if (body === undefined) return false;
  const variants = new Set(body);
  return ["Null", "Bool", "Number", "String", "Array", "Object"].every((required) => variants.has(required));
}

const jsonValueBodies = new Map<string, string[]>();

export function rustTypeToJsonSchema(rustType: string, resolve?: (name: string) => Record<string, unknown> | null, seen: ReadonlySet<string> = new Set()): Record<string, unknown> | null {
  const type = rustType.trim();
  const scalar = RUST_SCALARS[type];
  if (scalar !== undefined) return { ...scalar };
  // 🌱️OPEN-ENDED VALUE TYPES. `DslValue` is literally the JSON value model — `Null | Bool | Number |
  // String | Array | Object` — and `serde_json::Value` is the same thing. Their honest schema is "any
  // JSON value", which is what `{}` means; refusing them was over-strict, not principled, and it
  // blocked whole owners because the scaffolder needs EVERY leaf of an owner described before it will
  // emit any of them.
  if (/^(?:[a-z_]+::)*(?:DslValue|JsonValue|Value)$/.test(type)) return { description: "any JSON value" };
  // 🌱️STRUCTURAL detection of the same thing under another name. `GltfJson` is `Null | Bool(bool) |
  // Number(f64) | String(String) | Array(Vec<Self>) | Object(Vec<(String, Self)>)` — the JSON value
  // model exactly, so its schema is "any JSON value" whatever the enum is called. Matching on the SHAPE
  // rather than on a list of names is what makes this a rule instead of a special case.
  if (jsonValueShaped(type, resolve)) return { description: "any JSON value" };
  const option = type.match(/^Option\s*<\s*(.+)\s*>$/s);
  if (option !== null) return rustTypeToJsonSchema(option[1]!, resolve, seen);
  const vector = type.match(/^Vec\s*<\s*(.+)\s*>$/s);
  if (vector !== null) {
    const items = rustTypeToJsonSchema(vector[1]!, resolve, seen);
    return items === null ? null : { type: "array", items };
  }
  // 🎚️A tuple is a positional array; serde writes it with one schema per position.
  const tuple = type.match(/^\(\s*(.+)\s*\)$/s);
  if (tuple !== null && tuple[1]!.includes(",")) {
    const parts: string[] = [];
    let level = 0;
    let current = "";
    for (const character of tuple[1]!) {
      if (character === "<" || character === "(" || character === "[") level += 1;
      if (character === ">" || character === ")" || character === "]") level -= 1;
      if (character === "," && level === 0) {
        parts.push(current);
        current = "";
        continue;
      }
      current += character;
    }
    parts.push(current);
    const items = parts.map((part) => rustTypeToJsonSchema(part.trim(), resolve, seen));
    if (items.some((item) => item === null)) return null;
    return { type: "array", prefixItems: items, minItems: items.length, maxItems: items.length };
  }
  // 🔗️`ArtifactChild<S>` is the framework's CHILD HANDLE, and its wire shape does not depend on `S`:
  // the phantom marker and the local owner are both `#[serde(skip)]`, leaving `child_id` and `target`
  // (an `ArtifactRef` of `artifact_id` + `dialect`). Read from the struct, not guessed — and it is what
  // every composite artifact uses to point at its children, so refusing it refused them all.
  const child = type.match(/^(?:[a-z_]+::)*ArtifactChild\s*<.*>$/s);
  if (child !== null) {
    return {
      type: "object",
      properties: { childId: { type: "string" }, target: { type: "object", properties: { artifactId: { type: "string" }, dialect: { type: "string" } }, required: ["artifactId", "dialect"] } },
      required: ["childId", "target"],
    };
  }
  const boxed = type.match(/^Box\s*<\s*(.+)\s*>$/s);
  if (boxed !== null) return rustTypeToJsonSchema(boxed[1]!, resolve, seen);
  // 🧭️A field may name its type by full path — `crate::artifacts::puzzle3d::Puzzle3dScale`. The
  // definition is indexed under the bare name, and the qualifier says where to look, which the
  // caller's own proximity resolution already handles.
  const qualified = type.match(/^(?:crate|super|self)(?:::[A-Za-z0-9_]+)*::([A-Z][A-Za-z0-9_]*)$/);
  if (qualified !== null) return rustTypeToJsonSchema(qualified[1]!, resolve, seen);
  // 📏️The element may itself be a qualified path or a generic — `[SemioPoint3; 4]`,
  // `[crate::…::Scale; 3]` — not only a bare identifier.
  const array = type.match(/^\[\s*(.+?)\s*;\s*(\d+)\s*\]$/s);
  if (array !== null) {
    const items = rustTypeToJsonSchema(array[1]!, resolve, seen);
    return items === null ? null : { type: "array", items, minItems: Number(array[2]), maxItems: Number(array[2]) };
  }
  const map = type.match(/^(?:HashMap|BTreeMap)\s*<\s*String\s*,\s*(.+)\s*>$/s);
  if (map !== null) {
    const values = rustTypeToJsonSchema(map[1]!, resolve, seen);
    return values === null ? null : { type: "object", additionalProperties: values };
  }
  // 🔁️A NEWTYPE or ALIAS is transparent on the wire — `EntityId(pub u64)` serialises as a number, and
  // 130 leaves were refused for it alone. Resolving one level and recursing is not a guess about the
  // domain type; it is reading the declaration serde reads. The `seen` set stops a cyclic alias from
  // recursing forever rather than being caught by a stack overflow.
  const bare = type.match(/^([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)$/);
  if (bare !== null && resolve !== undefined && !seen.has(bare[1]!)) {
    const inner = resolve(bare[1]!.split("::").pop()!);
    if (inner !== null) return inner;
  }
  return null;
}

/**
 * 🔁️ Indexes every transparent Rust type in the repository — `pub struct X(pub Inner);` newtypes and
 * `pub type X = Inner;` aliases — so a payload field declared with one can still be projected onto the
 * shape it actually serialises as.
 */
export function transparentRustTypes(repoRoot: string): ReadonlyMap<string, string> {
  return rustTypeIndex(repoRoot).transparent;
}

/**
 * 🗂️ One pass over every Rust file, indexing the two shapes a payload field can legitimately name:
 * TRANSPARENT types (newtypes and aliases, which serialise as their inner type) and COMPOSITE structs
 * (which serialise as an object of their own fields).
 *
 * A composite is only recorded when its name is UNAMBIGUOUS across the repository. Two different
 * `ObjRef` structs in two plugins are two different contracts, and picking either would put one
 * plugin's field list into the other's payload schema — a contract that validates the wrong shape is
 * worse than an absent one, which is why an ambiguous name is dropped rather than resolved.
 */
export function rustTypeIndex(repoRoot: string): { transparent: ReadonlyMap<string, string>; composite: ReadonlyMap<string, string>; enums: ReadonlyMap<string, string>; tagged: ReadonlyMap<string, string>; placed: ReadonlyMap<string, { path: string; body: string }[]>; placedEnums: ReadonlyMap<string, { path: string; body: string }[]> } {
  const cached = rustIndexCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const found = new Map<string, string>();
  const composites = new Map<string, string>();
  const enums = new Map<string, string>();
  const tagged = new Map<string, string>();
  const placed = new Map<string, { path: string; body: string }[]>();
  const placedEnums = new Map<string, { path: string; body: string }[]>();
  const ambiguous = new Set<string>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
      let text: string;
      try {
        text = withoutTestModules(readFileSync(join(abs, entry.name), "utf8"));
      } catch {
        continue;
      }
      for (const match of text.matchAll(/^pub struct ([A-Za-z0-9_]+)\s*\(\s*(?:pub\s+)?([^),]+)\s*\)\s*;/gm)) if (!found.has(match[1]!)) found.set(match[1]!, match[2]!.trim());
      for (const match of text.matchAll(/^pub type ([A-Za-z0-9_]+)\s*=\s*([^;]+);/gm)) if (!found.has(match[1]!)) found.set(match[1]!, match[2]!.trim());
      // 🧱️BRACE-MATCH the struct body. The old terminator was `\n}`, which a SINGLE-LINE struct —
      // `pub struct GltfBindNodeMeshPayload { pub node: usize, pub mesh: usize }` — never satisfies, so
      // the body ran on past the closing brace and swallowed the next `pub fn validate(...)`. Every
      // field parsed out of that run-on was garbage, and the leaf was refused for a type that does not
      // exist. It gated all 120 of `gltf`'s leaves.
      for (const match of text.matchAll(/^ {0,4}pub struct ([A-Za-z0-9_]+)\s*\{/gm)) {
        const name = match[1]!;
        const open = match.index! + match[0].length - 1;
        let depth = 0;
        let close = -1;
        for (let at = open; at < text.length; at += 1) {
          if (text[at] === "{") depth += 1;
          else if (text[at] === "}") {
            depth -= 1;
            if (depth === 0) {
              close = at;
              break;
            }
          }
        }
        if (close === -1) continue;
        const body = text.slice(open + 1, close);
        const existing = composites.get(name);
        if (existing !== undefined && existing !== body) ambiguous.add(name);
        else composites.set(name, body);
        // 📍️A name is not unique in this repository, and it is not supposed to be: `FemMaterial` is one
        // struct in `fem/◻️2d` and a different one in `fem/🧊3d`, exactly as the taxonomy intends. Keying
        // the index by bare name made those AMBIGUOUS and dropped them, which refused every leaf that
        // mentioned them. Rust resolves by module path; so does this, by remembering where each
        // definition lives and letting the caller's own owner path pick the nearest one.
        const located = placed.get(name) ?? [];
        if (!located.some((entry) => entry.body === body)) located.push({ path: rel, body });
        placed.set(name, located);
      }
      // 🏷️TAGGED ENUMS — struct variants under `#[serde(tag = "…")]`. These serialise to an object
      // carrying the tag plus that variant's own fields, so the schema is a `oneOf` over those shapes.
      // They defeated `semio@v1/brep` entirely: `BrepCurve` and `BrepSurface` are the vocabulary of the
      // whole subset (line/circle/ellipse/nurbs, plane/cylinder/cone/…), so 4 of its 13 leaves could not
      // be described and the owner could not be scaffolded at all.
      for (const match of text.matchAll(/#\[serde\(([^)]*tag\s*=[^)]*)\)\]\s*\n(?:#\[[^\]]*\]\s*\n)*pub enum ([A-Za-z0-9_]+)\s*\{([\s\S]*?)\n\}/gm)) {
        const attrs = match[1]!;
        const name = match[2]!;
        const body = match[3]!;
        const tag = attrs.match(/tag\s*=\s*"([^"]+)"/)?.[1];
        if (tag === undefined) continue;
        const renameVariants = attrs.match(/rename_all\s*=\s*"([^"]+)"/)?.[1];
        const renameFields = attrs.match(/rename_all_fields\s*=\s*"([^"]+)"/)?.[1];
        const variants: { name: string; body: string }[] = [];
        const re = /([A-Za-z][A-Za-z0-9_]*)\s*\{([^{}]*)\}/g;
        for (let v = re.exec(body); v !== null; v = re.exec(body)) variants.push({ name: v[1]!, body: v[2]! });
        if (variants.length === 0) continue;
        const encoded = `tagged:${JSON.stringify({ tag, renameVariants, renameFields, variants })}`;
        const prior = tagged.get(name);
        if (prior !== undefined && prior !== encoded) ambiguous.add(name);
        else tagged.set(name, encoded);
        const locatedTagged = placedEnums.get(name) ?? [];
        if (!locatedTagged.some((entry) => entry.body === encoded)) locatedTagged.push({ path: rel, body: encoded });
        placedEnums.set(name, locatedTagged);
      }
      // 🌱️Record every enum's variant NAMES so the JSON-value shape can be recognised structurally.
      for (const match of text.matchAll(/^ {0,4}pub enum ([A-Za-z0-9_]+)\s*\{([\s\S]*?)\n {0,4}\}/gm)) {
        const variants = (match[2] ?? "")
          .replace(/\/\/.*$/gm, "")
          .replace(/#\[[^\]]*\]/g, "")
          .split(/[,\n]/)
          .map((entry) => entry.trim().match(/^([A-Za-z][A-Za-z0-9_]*)/)?.[1] ?? "")
          .filter((entry) => entry.length > 0);
        if (variants.length > 0) jsonValueBodies.set(match[1]!, variants);
      }
      // 🔤️FIELDLESS ENUMS. A unit-variant enum serialises to a plain string, so it has a perfectly
      // derivable JSON Schema — `{ type: "string", enum: [...] }` — and refusing it blocked far more
      // than itself: `SemioTopology` defeated `SemioPrimitive` and `SemioMesh` transitively, and with
      // them the whole `semio@v1/mesh` owner, whose 17 leaves could not be scaffolded because 3 of them
      // mentioned it. `rename_all` is honoured because it decides the wire strings.
      for (const match of text.matchAll(/(#\[serde\(([^)]*)\)\]\s*)?^ {0,4}pub enum ([A-Za-z0-9_]+)\s*\{([\s\S]*?)\n {0,4}\}/gm)) {
        const attrs = match[2] ?? "";
        const name = match[3]!;
        const body = match[4]!;
        if (/[(:{]/.test(body.replace(/\/\/.*$/gm, "").replace(/#\[[^\]]*\]/g, ""))) continue;
        const variants = body
          .replace(/\/\/.*$/gm, "")
          .replace(/#\[[^\]]*\]/g, "")
          .split(",")
          .map((v) => v.trim())
          .filter((v) => /^[A-Za-z][A-Za-z0-9_]*$/.test(v));
        if (variants.length === 0) continue;
        const rename = attrs.match(/rename_all\s*=\s*"([^"]+)"/)?.[1];
        const wire = variants.map((v) => (rename === "camelCase" ? camelCase(v) : rename === "kebab-case" ? v.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase() : rename === "snake_case" ? v.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase() : rename === "lowercase" ? v.toLowerCase() : v));
        const encoded = `enum:${JSON.stringify(wire)}`;
        const prior = enums.get(name);
        if (prior !== undefined && prior !== encoded) ambiguous.add(name);
        else enums.set(name, encoded);
        const locatedEnum = placedEnums.get(name) ?? [];
        if (!locatedEnum.some((entry) => entry.body === encoded)) locatedEnum.push({ path: rel, body: encoded });
        placedEnums.set(name, locatedEnum);
      }
    }
    return "enter";
  });
  for (const name of ambiguous) {
    composites.delete(name);
    enums.delete(name);
    tagged.delete(name);
  }
  const index = { transparent: found, composite: composites, enums, tagged, placed, placedEnums };
  rustIndexCache.set(repoRoot, index);
  return index;
}

const rustIndexCache = new Map<string, { transparent: ReadonlyMap<string, string>; composite: ReadonlyMap<string, string>; enums: ReadonlyMap<string, string>; tagged: ReadonlyMap<string, string>; placed: ReadonlyMap<string, { path: string; body: string }[]>; placedEnums: ReadonlyMap<string, { path: string; body: string }[]> }>();

/** 🧬️ Splits a struct body into `(fieldName, rustType)` pairs, ignoring attributes and comments. */
function structFields(body: string): { name: string; type: string; flatten: boolean }[] {
  const fields: { name: string; type: string; flatten: boolean }[] = [];
  // 🧭️Depth-aware split: a field type may itself contain commas (`HashMap<String, Vec<T>>`), so the
  // naive `body.split(",")` produced half-types and refused schemas that were perfectly derivable.
  let depth = 0;
  let current = "";
  const parts: string[] = [];
  for (const character of body) {
    if (character === "<" || character === "(" || character === "[") depth += 1;
    if (character === ">" || character === ")" || character === "]") depth -= 1;
    if (character === "," && depth === 0) {
      parts.push(current);
      current = "";
      continue;
    }
    current += character;
  }
  parts.push(current);
  for (const raw of parts) {
    const line = raw
      .split("\n")
      .map((candidate) => candidate.trim())
      .filter((candidate) => candidate.length > 0 && !candidate.startsWith("//") && !candidate.startsWith("#["))
      .join(" ")
      .trim();
    const declaration = line.match(/^pub\s+([a-z_][a-z0-9_]*)\s*:\s*(.+)$/s);
    // 🫓️`#[serde(flatten)]` inlines the nested type's OWN properties into this object — there is no
    // property under this field's name at all. The attribute is stripped above with every other, so it
    // is read off the raw fragment before that happens.
    if (declaration !== null) fields.push({ name: declaration[1]!, type: declaration[2]!.trim(), flatten: /#\[serde\([^)]*\bflatten\b[^)]*\)\]/.test(raw) });
  }
  return fields;
}

/**
 * 🧬️ Derives one mutation leaf's payload schema from the Rust payload struct it declares.
 *
 * A payload schema is the deepest blocker in the chain: you cannot author a fixture for a mutation
 * whose payload has no contract, so 1 394 leaves were unreachable by any amount of testing effort. The
 * struct IS the contract — it is what serde serialises on the wire — so the schema is a projection of
 * it, not a second declaration to keep in sync.
 */
export function derivePayloadSchema(repoRoot: string, ownerRel: string, leafDirName: string): PayloadSchemaDerivation {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  const leafAbs = join(vocabulary, leafDirName);
  const leafRel = `${ownerRel}/🧬️schema/${taxonomy.testMutationVocabularyDirName}/${leafDirName}`;
  const kind = leafDirName.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)?.[0] ?? "";
  const source = readFirst([join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🦀️.rs"), join(leafAbs, "🦀️.rs")]);
  if (source === null) return { leaf: leafRel, kind, schema: null, struct: "", refused: ["no Rust source in the leaf"] };

  // 🧱️Same brace-matching the index needed: `\n}` never terminates a SINGLE-LINE struct, so the body
  // ran past its closing brace into the following `pub fn validate(..)` and every field parsed out of
  // it was fiction. This is the leaf-level twin of that bug and it gated all 120 of `gltf`'s leaves.
  const declaration = source.text.match(/pub struct ([A-Za-z0-9_]+)\s*(\{)?/);
  // 📄️`set-snapshot` carries no payload struct because its payload IS the artifact's snapshot, and the
  // type is stated in the leaf's own apply signature — `apply(projection: &mut DwgSnapshot, ..)`. Read
  // from there, never assumed. It is by far the commonest refusal left: ~35 single-leaf owners, each
  // blocking itself entirely for a struct that was never supposed to exist.
  if (declaration === null || (declaration[2] === undefined && kind === "set-snapshot")) {
    const applied = source.text.match(/fn apply\s*\(\s*[a-z_]+\s*:\s*&mut\s+([A-Za-z0-9_]+)/)?.[1];
    if (kind === "set-snapshot" && applied !== undefined) {
      const index = rustTypeIndex(repoRoot);
      const resolveSnapshot = (name: string, depth = 0, chain: ReadonlySet<string> = new Set()): Record<string, unknown> | null => {
        if (depth > 5 || chain.has(name)) return null;
        const transparent = index.transparent.get(name);
        if (transparent !== undefined) return rustTypeToJsonSchema(transparent, (next) => resolveSnapshot(next, depth + 1, new Set([...chain, name])), new Set([name]));
        const enumerated = index.enums.get(name);
        if (enumerated !== undefined) return { type: "string", enum: JSON.parse(enumerated.slice("enum:".length)) as string[] };
        // 📍️Nearest-definition fallback, exactly as the field resolver does. Snapshot type names repeat
        // across subsets by design — `DwgSnapshot`, `GifSnapshot`, `PptxSnapshot` — so the bare-name
        // index drops them as ambiguous, and this resolver had no fallback. That single omission refused
        // ~25 single-leaf `set-snapshot` owners, each of which is its whole owner.
        let composite = index.composite.get(name);
        if (composite === undefined) {
          const located = index.placed.get(name) ?? [];
          const segments = ownerRel.split("/");
          let best: { path: string; body: string } | undefined;
          let bestScore = 0;
          for (const entry of located) {
            const parts = entry.path.split("/");
            let score = 0;
            while (score < parts.length && score < segments.length && parts[score] === segments[score]) score += 1;
            if (score > bestScore) {
              bestScore = score;
              best = entry;
            }
          }
          if (best === undefined) return null;
          composite = best.body;
        }
        const properties: Record<string, unknown> = {};
        const required: string[] = [];
        for (const field of structFields(composite)) {
          const schema = rustTypeToJsonSchema(field.type, (next) => resolveSnapshot(next, depth + 1, new Set([...chain, name])), new Set([name]));
          if (schema === null) return null;
          properties[camelCase(field.name)] = schema;
          if (!/^Option\s*</.test(field.type)) required.push(camelCase(field.name));
        }
        return { type: "object", properties, required, additionalProperties: false };
      };
      const schema = resolveSnapshot(applied);
      if (schema !== null) return { leaf: leafRel, kind, schema, struct: applied, refused: [] };
      return { leaf: leafRel, kind, schema: null, struct: applied, refused: [`field snapshot: ${applied} is not a shape this derivation decides`] };
    }
    if (declaration === null) return { leaf: leafRel, kind, schema: null, struct: "", refused: [`no \`pub struct\` in ${relative(repoRoot, source.path).split(sep).join("/")}`] };
  }
  const struct = declaration[1]!;
  // 🧬️A unit struct (`pub struct DeleteShapeModel;`) is a payload with no fields, and its schema is the
  // empty object — that is a real contract, not a missing one.
  let body = "";
  if (declaration[2] !== undefined) {
    const open = declaration.index! + declaration[0].length - 1;
    let depth = 0;
    for (let at = open; at < source.text.length; at += 1) {
      if (source.text[at] === "{") depth += 1;
      else if (source.text[at] === "}") {
        depth -= 1;
        if (depth === 0) {
          body = source.text.slice(open + 1, at);
          break;
        }
      }
    }
  }
  const fields = structFields(body);

  const properties: Record<string, unknown> = {};
  const required: string[] = [];
  const refused: string[] = [];
  const index = rustTypeIndex(repoRoot);
  const resolve = (name: string, depth = 0, chain: ReadonlySet<string> = new Set()): Record<string, unknown> | null => {
    // 🔁️Depth and cycle limits are both needed: depth bounds how far a payload contract may reach into
    // the domain before it stops being a payload contract, and the chain stops a struct that contains
    // itself from recursing forever.
    if (depth > 5 || chain.has(name)) return null;
    const transparent = index.transparent.get(name);
    if (transparent !== undefined) return rustTypeToJsonSchema(transparent, (next) => resolve(next, depth + 1, new Set([...chain, name])), new Set([name]));
    // 🔤️A fieldless enum is a closed set of wire strings — the most precise schema of all, and cheap.
    // 📍️Nearest definition wins for enums exactly as for structs: `Priority` is one enum in
    // architect's kernel and a different one in the OS db policy, and dropping it as "ambiguous" took
    // `EntityHeader` with it, and with that every register type in `architect/program` — 128 leaves
    // refused for a name collision two directories apart.
    const nearest = (buckets: ReadonlyMap<string, { path: string; body: string }[]>): string | undefined => {
      const located = buckets.get(name) ?? [];
      if (located.length === 0) return undefined;
      const segments = ownerRel.split("/");
      let best: { path: string; body: string } | undefined;
      let bestScore = 0;
      for (const entry of located) {
        const parts = entry.path.split("/");
        let score = 0;
        while (score < parts.length && score < segments.length && parts[score] === segments[score]) score += 1;
        if (score > bestScore) {
          bestScore = score;
          best = entry;
        }
      }
      return best?.body;
    };
    const enumerated = index.enums.get(name) ?? nearest(index.placedEnums);
    if (enumerated !== undefined && enumerated.startsWith("enum:")) return { type: "string", enum: JSON.parse(enumerated.slice("enum:".length)) as string[] };
    // 🏷️A tagged enum is a `oneOf` over its variants, each an object carrying the tag plus its own
    // fields. Every variant must resolve — a partial union would silently accept shapes the Rust type
    // rejects, which is worse than refusing the schema outright.
    const taggedRaw = index.tagged.get(name) ?? (enumerated !== undefined && enumerated.startsWith("tagged:") ? enumerated : undefined);
    if (taggedRaw !== undefined) {
      const spec = JSON.parse(taggedRaw.slice("tagged:".length)) as { tag: string; renameVariants?: string; renameFields?: string; variants: { name: string; body: string }[] };
      const rename = (value: string, style: string | undefined): string => (style === "camelCase" ? camelCase(value) : style === "kebab-case" ? value.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase() : style === "snake_case" ? value.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase() : style === "lowercase" ? value.toLowerCase() : value);
      const branches: Record<string, unknown>[] = [];
      for (const variant of spec.variants) {
        const properties: Record<string, unknown> = { [spec.tag]: { const: rename(variant.name, spec.renameVariants) } };
        const required: string[] = [spec.tag];
        for (const field of structFields(variant.body)) {
          const schema = rustTypeToJsonSchema(field.type, (next) => resolve(next, depth + 1, new Set([...chain, name])), new Set([name]));
          if (schema === null) return null;
          const key = rename(field.name, spec.renameFields);
          properties[key] = schema;
          if (!/^Option\s*</.test(field.type)) required.push(key);
        }
        branches.push({ type: "object", properties, required, additionalProperties: false });
      }
      return branches.length === 0 ? null : { oneOf: branches };
    }
    let composite = index.composite.get(name);
    if (composite === undefined) {
      // 📍️Same name, several real definitions. Pick the one whose file shares the longest path prefix
      // with the owner we are deriving for — the nearest definition in the taxonomy, which is the one
      // Rust itself would resolve to from that module.
      const located = index.placed.get(name) ?? [];
      if (located.length === 0) return null;
      const segments = ownerRel.split("/");
      let best = located[0]!;
      let bestScore = -1;
      for (const entry of located) {
        const parts = entry.path.split("/");
        let score = 0;
        while (score < parts.length && score < segments.length && parts[score] === segments[score]) score += 1;
        if (score > bestScore) {
          bestScore = score;
          best = entry;
        }
      }
      if (bestScore <= 0) return null;
      composite = best.body;
    }
    const properties: Record<string, unknown> = {};
    const required: string[] = [];
    for (const field of structFields(composite)) {
      const schema = rustTypeToJsonSchema(field.type, (next) => resolve(next, depth + 1, new Set([...chain, name])), new Set([name]));
      if (schema === null) return null;
      if (field.flatten) {
        // 🫓️Merge the flattened type's own members up into this object, exactly as serde does on the wire.
        const inner = schema as { properties?: Record<string, unknown>; required?: string[] };
        if (inner.properties === undefined) return null;
        for (const [key, value] of Object.entries(inner.properties)) properties[key] = value;
        for (const key of inner.required ?? []) required.push(key);
        continue;
      }
      properties[camelCase(field.name)] = schema;
      if (!/^Option\s*</.test(field.type)) required.push(camelCase(field.name));
    }
    return { title: name, type: "object", additionalProperties: false, ...(required.length > 0 ? { required: required.sort() } : {}), properties };
  };
  for (const field of fields) {
    const schema = rustTypeToJsonSchema(field.type, resolve);
    if (schema === null) {
      refused.push(`field ${field.name}: ${field.type} is not a shape this derivation decides`);
      continue;
    }
    properties[camelCase(field.name)] = schema;
    if (!/^Option\s*</.test(field.type)) required.push(camelCase(field.name));
  }
  if (refused.length > 0) return { leaf: leafRel, kind, schema: null, struct, refused };
  return {
    leaf: leafRel,
    kind,
    struct,
    refused,
    schema: {
      $schema: "http://json-schema.org/draft-07/schema#",
      title: struct,
      type: "object",
      additionalProperties: false,
      ...(required.length > 0 ? { required: required.sort() } : {}),
      properties,
    },
  };
}

/** 🧬️ Derives every leaf's payload schema for one owner. */
export function derivePayloadSchemas(repoRoot: string, ownerRel: string): PayloadSchemaDerivation[] {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, "🧬️schema", taxonomy.testMutationVocabularyDirName);
  if (!existsSync(vocabulary)) return [];
  return readdirSync(vocabulary, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.isSymbolicLink() && isMutationLeafDirectory(repoRoot, entry.name))
    .map((entry) => derivePayloadSchema(repoRoot, ownerRel, entry.name))
    .sort((a, b) => a.kind.localeCompare(b.kind));
}
//#endregion 🧬️PayloadSchema

//#region 🗄️Storage
/**
 * 🗄️ Content-addressed fixture storage. Every agent and every run would otherwise take its own copy of
 * a megabyte-scale STEP and mesh corpus, so blobs are stored ONCE by digest and referenced into run
 * directories by the cheapest safe mechanism the filesystem offers.
 */
export function fixtureBlobRoot(repoRoot: string): string {
  return join(testCacheRoot(repoRoot), "fixtures", "blobs", "sha256");
}

/** 🗄️ Where fixture manifests are cached by id, for GC's mark phase and for cross-run reuse. */
export function fixtureManifestRoot(repoRoot: string): string {
  return join(testCacheRoot(repoRoot), "fixtures", "manifests");
}

/** 🗄️ Absolute path of one blob. Sharded by the first two hex digits so a directory never grows unbounded. */
export function fixtureBlobPath(repoRoot: string, sha256: string): string {
  if (!SHA256_RE.test(sha256)) throw new Error(`not a content digest: ${JSON.stringify(sha256)}`);
  const hex = sha256.slice("sha256:".length);
  return join(fixtureBlobRoot(repoRoot), hex.slice(0, 2), hex);
}

/**
 * 🗄️ Installs bytes under their digest, atomically. Two agents generating the same fixture concurrently
 * is the normal case, not the exception: each writes a private temporary file and renames it into
 * place, so a reader never observes a partial blob and neither writer clobbers the other.
 */
export function installFixtureBlob(repoRoot: string, bytes: Uint8Array): string {
  const sha256 = contentDigest(bytes);
  const target = fixtureBlobPath(repoRoot, sha256);
  if (existsSync(target)) return sha256;
  mkdirSync(dirname(target), { recursive: true });
  const staging = `${target}.${process.pid}.${agentId()}.tmp`;
  writeFileSync(staging, bytes);
  try {
    renameSync(staging, target);
  } catch {
    // 🏁️Another agent won the race and the blob is already correct by construction — its name IS its content.
    rmSync(staging, { force: true });
  }
  return sha256;
}

/** 🗄️ Installs a file from disk under its digest without reading it twice into memory when possible. */
export function installFixtureFile(repoRoot: string, absPath: string): string {
  return installFixtureBlob(repoRoot, readFileSync(absPath));
}

/** 🗄️ How one blob was materialized into a run directory, so the report can show what the cache saved. */
export type MaterializeMode = "reflink" | "hardlink" | "copy";

/**
 * 🗄️ References a blob into a run directory: copy-on-write clone first, hard link second, plain copy
 * last. A hard link is only safe for a file the consumer will not write through — `mutable: true`
 * forces a real copy, because a mutation scenario handed a link would write into shared storage.
 */
export function materializeFixtureBlob(repoRoot: string, sha256: string, targetPath: string, opts: { mutable?: boolean } = {}): MaterializeMode {
  const source = fixtureBlobPath(repoRoot, sha256);
  if (!existsSync(source)) throw new Error(`fixture blob ${sha256} is not in the store`);
  mkdirSync(dirname(targetPath), { recursive: true });
  rmSync(targetPath, { force: true });
  try {
    cpSync(source, targetPath, { mode: constants.COPYFILE_FICLONE_FORCE });
    return "reflink";
  } catch {
    /* 🧭️No copy-on-write on this filesystem. */
  }
  if (opts.mutable !== true) {
    try {
      linkSync(source, targetPath);
      return "hardlink";
    } catch {
      /* 🧭️Cross-device or a filesystem without hard links. */
    }
  }
  cpSync(source, targetPath);
  return "copy";
}

/** 🗄️ Publishes a fixture manifest into the cache so GC can mark its blobs as referenced. */
export function publishFixtureManifest(repoRoot: string, manifest: FixtureManifest): string {
  const path = join(fixtureManifestRoot(repoRoot), `${manifest.id}.json`);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`);
  return path;
}
//#endregion 🗄️Storage

//#region 🔬️Probe
/** 🔬️ One EXTERNAL measurement tool. The orchestrator invokes it and compares what it emits — never computes. */
export type ProbeEntry = Readonly<{
  id: string;
  kind: "external-process" | "external-library";
  ecosystem: string;
  package: string;
  version?: string;
  lockDigest?: string;
  engine?: EngineFamily;
  capabilities: readonly string[];
  outputSchema: string;
  deterministic: boolean;
  seedRequired?: boolean;
  platforms?: readonly PlatformId[];
  license: string;
  testOnly: true;
  productionReachable?: boolean;
  networkDuringExecution?: boolean;
  command?: readonly string[];
  rationale?: string;
  qualification?: ProbeQualification;
  /** 🧩️ Further third-party packages this probe links beyond its primary one, each pinned and licensed in its own right. */
  packages?: readonly OracleLinkedPackage[];
  /** 🔒️ Pre-existing production reachability, recorded rather than hidden. Shrink-only, exactly as for an oracle. */
  productionDebt?: { reachableFrom: readonly string[]; owner: string; plan: string };
}>;

/**
 * 🎓️ The recorded outcome of a qualification spike. A `provisional` probe may RUN and REPORT; no
 * release gate may claim its strongest guarantee. This is what keeps "canonical STEP byte equality"
 * from being promised before an external canonicaliser has actually been qualified.
 */
export type ProbeQualification = Readonly<{ status: "qualified" | "provisional" | "rejected"; evidence: string; checkedAt?: string; criteria?: readonly { id: string; met: boolean; detail?: string }[] }>;

/** 🎓️ Whether a probe's strongest guarantee may be claimed by a gate. */
export function isQualifiedProbe(probe: ProbeEntry | undefined): boolean {
  return probe !== undefined && probe.qualification?.status === "qualified";
}

/** 🔬️ The typed JSON one probe emits for one stage. `measurements` is probe-defined, by design. */
export type ProbeReport = Readonly<{
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion?: string;
  engine?: EngineFamily;
  status: "ok" | "failed" | "unsupported";
  seed?: string | number;
  durationMs?: number;
  measurements: Readonly<Record<string, unknown>>;
  outputs?: readonly ResultArtifact[];
  diagnostics?: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
}>;

/** 📦️ One file a host or a probe produced, addressed by role. */
export type ResultArtifact = Readonly<{ role: string; path: string; mediaType: string; sha256: string; bytes?: number }>;

/** 🧾️ Validates a probe's emitted report before a single assertion is evaluated against it. */
export function probeReportProblems(value: unknown): string[] {
  if (!isPlainObject(value)) return ["probe report is not an object"];
  const problems: string[] = [];
  if (value.schema !== "semio.repository-test.probe-report/v2") problems.push('schema must be "semio.repository-test.probe-report/v2"');
  if (typeof value.probe !== "string" || value.probe.length === 0) problems.push("probe must name the emitting probe id");
  if (!["ok", "failed", "unsupported"].includes(String(value.status))) problems.push("status must be ok|failed|unsupported");
  if (!isPlainObject(value.measurements)) problems.push("measurements must be an object");
  return problems;
}
//#endregion 🔬️Probe

//#region 📏️Tolerance
/**
 * 📏️ Scale-relative tolerance policy. One absolute number cannot serve a micro-scale fixture and a
 * fixture translated a kilometre from the origin at the same time, so the effective tolerance is
 * `max(absolute, relative × reference)` and normalised metrics divide by the reference scale.
 */
export type ToleranceProfile = Readonly<{
  id: string;
  description?: string;
  absoluteLength?: number;
  relativeLength?: number;
  absoluteArea?: number;
  relativeArea?: number;
  absoluteVolume?: number;
  relativeVolume?: number;
  normalizedHausdorffMax?: number;
  normalizedCentroidDistanceMax?: number;
  maxOverrideFactor?: number;
}>;

/** 📏️ The domain-neutral profiles the framework owns. An owner contributes further ones by manifest. */
export const CORE_TOLERANCE_PROFILES: readonly ToleranceProfile[] = [
  { id: "analytic-strict", description: "Analytic primitives whose exact answer is known in closed form; only representation noise is permitted.", absoluteLength: 1e-9, relativeLength: 1e-12, absoluteArea: 1e-9, relativeArea: 1e-11, absoluteVolume: 1e-9, relativeVolume: 1e-11, normalizedHausdorffMax: 1e-9, normalizedCentroidDistanceMax: 1e-10, maxOverrideFactor: 10 },
  { id: "mechanical-standard", description: "Ordinary mechanical solids at millimetre scale — the default for the Boolean corpus.", absoluteLength: 1e-7, relativeLength: 1e-9, absoluteArea: 1e-7, relativeArea: 1e-8, absoluteVolume: 1e-7, relativeVolume: 1e-8, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 10 },
  { id: "contact-sensitive", description: "Exact-contact, tangency and coincident-face cases, where kernels legitimately disagree about the last bits of a classification boundary.", absoluteLength: 1e-6, relativeLength: 1e-8, absoluteArea: 1e-6, relativeArea: 1e-7, absoluteVolume: 1e-6, relativeVolume: 1e-7, normalizedHausdorffMax: 1e-5, normalizedCentroidDistanceMax: 1e-7, maxOverrideFactor: 5 },
  { id: "epsilon-degenerate", description: "Slivers, tiny edges and near-coplanar faces, where the RESULT CLASS is the assertion and the metric is secondary.", absoluteLength: 1e-5, relativeLength: 1e-7, absoluteArea: 1e-5, relativeArea: 1e-6, absoluteVolume: 1e-5, relativeVolume: 1e-6, normalizedHausdorffMax: 1e-4, normalizedCentroidDistanceMax: 1e-6, maxOverrideFactor: 4 },
  { id: "large-coordinate", description: "Geometry translated far from the origin, where absolute error grows with the coordinate and only the relative term is meaningful.", absoluteLength: 1e-4, relativeLength: 1e-9, absoluteArea: 1e-4, relativeArea: 1e-8, absoluteVolume: 1e-4, relativeVolume: 1e-8, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 5 },
  { id: "micro-scale", description: "Sub-millimetre geometry, where an absolute floor sized for millimetres would swallow the whole model.", absoluteLength: 1e-11, relativeLength: 1e-8, absoluteArea: 1e-11, relativeArea: 1e-7, absoluteVolume: 1e-11, relativeVolume: 1e-7, normalizedHausdorffMax: 1e-6, normalizedCentroidDistanceMax: 1e-8, maxOverrideFactor: 5 },
  { id: "real-world-import", description: "Third-party real-world models whose own authoring tolerance is unknown and larger than anything we generate.", absoluteLength: 1e-4, relativeLength: 1e-6, absoluteArea: 1e-4, relativeArea: 1e-5, absoluteVolume: 1e-4, relativeVolume: 1e-5, normalizedHausdorffMax: 1e-4, normalizedCentroidDistanceMax: 1e-5, maxOverrideFactor: 3 },
];

/** 📏️ The effective profile table: core profiles plus every contributed one, keyed by id. */
export function toleranceProfileTable(registry: OracleRegistry): ReadonlyMap<string, ToleranceProfile> {
  return new Map([...CORE_TOLERANCE_PROFILES, ...registry.toleranceProfiles].map((profile) => [profile.id, profile]));
}

/** 📏️ `max(absolute, relative × reference)` — the one rule every dimensional tolerance is resolved by. */
export function resolveTolerance(absolute: number | undefined, relative: number | undefined, reference: number): number {
  return Math.max(absolute ?? 0, (relative ?? 0) * Math.abs(reference));
}

/** 📏️ The four resolved dimensional tolerances of one profile against one fixture's reference scale. */
export type ResolvedTolerances = Readonly<{ length: number; area: number; volume: number; normalizedHausdorffMax: number; normalizedCentroidDistanceMax: number; overrideFactor: number; overridden: boolean }>;

/**
 * 📏️ Resolves a profile against a measured reference scale and an optional fixture override. The
 * override is CAPPED by the profile's own `maxOverrideFactor`, so a fixture cannot quietly widen its
 * gate until everything passes.
 */
export function resolveToleranceProfile(profile: ToleranceProfile, reference: { diagonal: number; area: number; volume: number }, override?: ToleranceOverride): ResolvedTolerances {
  const cap = profile.maxOverrideFactor ?? 1;
  const factor = override === undefined ? 1 : Math.min(override.factor, cap);
  return {
    length: resolveTolerance(profile.absoluteLength, profile.relativeLength, reference.diagonal) * factor,
    area: resolveTolerance(profile.absoluteArea, profile.relativeArea, reference.area) * factor,
    volume: resolveTolerance(profile.absoluteVolume, profile.relativeVolume, reference.volume) * factor,
    normalizedHausdorffMax: (profile.normalizedHausdorffMax ?? 0) * factor,
    normalizedCentroidDistanceMax: (profile.normalizedCentroidDistanceMax ?? 0) * factor,
    overrideFactor: factor,
    overridden: override !== undefined,
  };
}
//#endregion 📏️Tolerance

//#region ⚖️Pipeline
/** ⚖️ One stage: a probe, the roles it reads, the roles it produces and the assertions on its output. */
export type ComparisonStage = Readonly<{
  probe: string;
  description?: string;
  inputs: readonly string[];
  outputs?: readonly string[];
  args?: readonly string[];
  seed?: string | number;
  /** 🚧️ A stage whose probe is not yet qualified. It RUNS and REPORTS; no release gate may claim its guarantee. */
  optional?: boolean;
  assertions?: Readonly<Record<string, unknown>>;
}>;

/** ⚖️ An ordered, multi-artifact, externally-probed comparison. */
export type ComparisonPipeline = Readonly<{ id: string; description?: string; toleranceProfile?: string; referenceScale?: "bounding-box-diagonal" | "unit" | "declared"; stages: readonly ComparisonStage[] }>;

/** ⚖️ One evaluated assertion. `expected` is what the stage declared, `actual` what the probe measured. */
export type AssertionVerdict = Readonly<{ stage: number; probe: string; key: string; expected: unknown; actual: unknown; ok: boolean; optional: boolean; reason: string }>;

/** ⚖️ The verdict of one whole pipeline run. */
export type PipelineVerdict = Readonly<{
  pipeline: string;
  equal: boolean;
  verdicts: readonly AssertionVerdict[];
  missingProbes: readonly string[];
  unqualifiedStages: readonly string[];
  /** 🚫️ Stages marked `optional` that are NOT excusable — their probe is qualified, or unregistered. The mark is ignored and reported. */
  overclaimedOptional: readonly string[];
}>;

/**
 * ⚖️ Evaluates one stage's declarative assertions against one probe report. The vocabulary is small
 * and deliberately arithmetic-free beyond comparison: `<key>Max` bounds a measured number from above,
 * `<key>Min` from below, `<key>Equal` compares structurally, and a bare boolean/number/string
 * compares for equality. Anything a probe cannot measure it must report as `unsupported` rather than
 * letting the orchestrator compute it.
 */
export function evaluateStageAssertions(stageIndex: number, stage: ComparisonStage, report: ProbeReport, effectiveOptional?: boolean): AssertionVerdict[] {
  const verdicts: AssertionVerdict[] = [];
  // 🚫️The caller decides whether `optional` actually applies, because only it knows the probe's
  // qualification. Called without that context this stays conservative and honours the raw mark.
  const optional = effectiveOptional ?? stage.optional === true;
  if (report.status !== "ok") {
    verdicts.push({ stage: stageIndex, probe: stage.probe, key: "status", expected: "ok", actual: report.status, ok: false, optional, reason: `probe reported ${report.status}` });
    return verdicts;
  }
  for (const [key, expected] of Object.entries(stage.assertions ?? {})) {
    const bound = key.endsWith("Max") ? "max" : key.endsWith("Min") ? "min" : key.endsWith("Equal") ? "equal" : "value";
    // 🔑️The EXACT key wins over the suffix-stripped one. A probe may legitimately name its own
    // measurement `connectedComponentsEqual`, and stripping unconditionally looked up
    // `connectedComponents`, found nothing, and failed a stage whose measurement was right there.
    const strippedKey = bound === "value" ? key : key.slice(0, -{ max: 3, min: 3, equal: 5 }[bound]!);
    const measurementKey = Object.hasOwn(report.measurements, key) ? key : strippedKey;
    const actual = report.measurements[measurementKey];
    if (actual === undefined) {
      verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual: undefined, ok: false, optional, reason: `probe ${stage.probe} reported no measurement ${JSON.stringify(strippedKey)}${strippedKey === key ? "" : ` (nor ${JSON.stringify(key)})`}` });
      continue;
    }
    if (bound === "max" || bound === "min") {
      const limit = Number(expected);
      const measured = Number(actual);
      const ok = Number.isFinite(measured) && Number.isFinite(limit) && (bound === "max" ? measured <= limit : measured >= limit);
      verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual, ok, optional, reason: ok ? "" : `${measurementKey} ${measured} violates ${bound} ${limit}` });
      continue;
    }
    const ok = JSON.stringify(canonicalize(actual)) === JSON.stringify(canonicalize(expected));
    verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual, ok, optional, reason: ok ? "" : `${bound === "equal" ? measurementKey : key} is not equal to the declared value` });
  }
  return verdicts;
}

/**
 * ⚖️ Evaluates a whole pipeline against the reports its stages produced. A stage with no report is a
 * failure, not a skip: an unmeasured assertion that reads as green is the exact defect this pipeline
 * replaces v1's single generic tolerance to prevent.
 */
export function evaluatePipeline(pipeline: ComparisonPipeline, reports: ReadonlyMap<number, ProbeReport>, probes: ReadonlyMap<string, ProbeEntry>): PipelineVerdict {
  const verdicts: AssertionVerdict[] = [];
  const missingProbes: string[] = [];
  const unqualifiedStages: string[] = [];
  const overclaimedOptional: string[] = [];
  for (const [index, stage] of pipeline.stages.entries()) {
    const probe = probes.get(stage.probe);
    if (probe === undefined) missingProbes.push(stage.probe);
    else if (!isQualifiedProbe(probe)) unqualifiedStages.push(stage.probe);
    // 🚫️`optional` EXCUSES ONLY WHAT IT WAS FOR: a stage whose probe is registered and not yet
    // qualified. It was a free boolean, so a stage using a fully QUALIFIED probe could be marked
    // optional and that probe's own `status: "failed"` was then silently excused — a qualifying
    // reference reporting failure while the pipeline read `equal: true`, which is the sharpest
    // possible violation of what this protocol claims. An UNREGISTERED probe cannot excuse anything
    // either, or deleting a registration would become a way to switch a gate off.
    const excusable = probe !== undefined && !isQualifiedProbe(probe);
    if (stage.optional === true && !excusable) overclaimedOptional.push(stage.probe);
    const effectiveOptional = stage.optional === true && excusable;
    const report = reports.get(index);
    if (report === undefined) {
      verdicts.push({ stage: index, probe: stage.probe, key: "report", expected: "a probe report", actual: undefined, ok: false, optional: effectiveOptional, reason: `stage ${index} (${stage.probe}) produced no report` });
      continue;
    }
    verdicts.push(...evaluateStageAssertions(index, stage, report, effectiveOptional));
  }
  // 🚫️A stage whose probe is not registered at all cannot be evaluated, so the pipeline is not equal —
  // `missingProbes` was computed and then ignored, which made an unrunnable pipeline read as passing.
  const equal = missingProbes.length === 0 && verdicts.every((verdict) => verdict.ok || verdict.optional);
  return { pipeline: pipeline.id, equal, verdicts, missingProbes, unqualifiedStages, overclaimedOptional };
}

/** ⚖️ The effective pipeline table: every contributed pipeline, keyed by id. */
export function pipelineTable(registry: OracleRegistry): ReadonlyMap<string, ComparisonPipeline> {
  return new Map(registry.comparisonPipelines.map((pipeline) => [pipeline.id, pipeline]));
}

/** 🔬️ The effective probe table, keyed by id. */
export function probeTable(registry: OracleRegistry): ReadonlyMap<string, ProbeEntry> {
  return new Map(registry.probes.map((probe) => [probe.id, probe]));
}
//#endregion ⚖️Pipeline

//#region 🔐️Lease
/** 🗄️ What a cached path IS, which decides when it may be removed. */
export const RETENTION_CLASSES = ["ephemeral-success", "last-success-proof", "failure-evidence", "fixture-generation", "toolchain", "downloaded-source", "pinned"] as const;
export type RetentionClass = (typeof RETENTION_CLASSES)[number];

/** 🗄️ Retention classes routine stale cleanup may NEVER remove. Failure evidence outlives the run that produced it. */
export const PROTECTED_RETENTION_CLASSES: readonly RetentionClass[] = ["failure-evidence", "last-success-proof", "pinned", "toolchain"];

export const LEASE_STATES = ["creating", "active", "complete", "failed", "abandoned"] as const;
export type LeaseState = (typeof LEASE_STATES)[number];

/** 🔐️ One agent's claim on one run directory. */
export type RunLease = Readonly<{
  schema: "semio.repository-test.lease/v2";
  runId: string;
  agentId: string;
  pid?: number;
  state: LeaseState;
  createdAt: string;
  heartbeatAt: string;
  retention: RetentionClass;
  runKey?: string;
  testId?: string;
}>;

/** 🏷️ This process's agent identity, from `SEMIO_AGENT_ID`. Never inferred from a username or a hostname. */
export function agentId(): string {
  return (process.env.SEMIO_AGENT_ID ?? "local").replace(/[^A-Za-z0-9._-]+/g, "_");
}

/** 🔐️ How long an `active` lease may go without a heartbeat before it is even a CANDIDATE for reclaim. */
export const LEASE_STALE_MS = 45 * 60 * 1000;

const LEASE_FILENAME = "🔐️lease.json";

/** 🔐️ Path of one run directory's lease file. */
export function leasePath(absRunDir: string): string {
  return join(absRunDir, LEASE_FILENAME);
}

/** 🔐️ Writes or refreshes a lease. A run directory without one is never treated as reclaimable state. */
export function writeLease(absRunDir: string, lease: Omit<RunLease, "schema">): RunLease {
  const record: RunLease = { schema: "semio.repository-test.lease/v2", ...lease };
  mkdirSync(absRunDir, { recursive: true });
  writeFileSync(leasePath(absRunDir), `${JSON.stringify(record, null, 2)}\n`);
  return record;
}

/** 🔐️ Reads a lease, or `null` when the directory carries none. */
export function readLease(absRunDir: string): RunLease | null {
  const path = leasePath(absRunDir);
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as RunLease;
    return parsed.schema === "semio.repository-test.lease/v2" ? parsed : null;
  } catch {
    return null;
  }
}

/** 🔐️ Refreshes a lease's heartbeat in place, so a long run is never mistaken for an abandoned one. */
export function heartbeatLease(absRunDir: string, nowIso = new Date().toISOString()): RunLease | null {
  const lease = readLease(absRunDir);
  if (lease === null) return null;
  return writeLease(absRunDir, { ...lease, heartbeatAt: nowIso });
}

/** 🔐️ Whether a process is still alive. A stale heartbeat alone is never enough to reclaim a lease. */
export function processAlive(pid: number | undefined): boolean {
  if (pid === undefined || !Number.isInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/**
 * 🔐️ Whether a lease may be reclaimed. An `active` lease is reclaimable only when BOTH its heartbeat
 * is older than the documented timeout AND its process is gone — a slow run and a dead one look
 * identical from the timestamp alone, and deleting the slow one destroys a peer agent's work.
 */
export function leaseReclaimable(lease: RunLease | null, nowMs = Date.now(), staleMs = LEASE_STALE_MS): boolean {
  if (lease === null) return false;
  if (lease.state === "complete" || lease.state === "abandoned") return true;
  if (lease.state === "failed") return false;
  const heartbeat = Date.parse(lease.heartbeatAt);
  const stale = Number.isFinite(heartbeat) && nowMs - heartbeat > staleMs;
  return stale && !processAlive(lease.pid) && lease.agentId !== agentId();
}

/**
 * 🔐️ Runs `body` inside a private temporary directory and publishes it with ONE atomic rename. A
 * reader therefore observes either the previous complete directory or the new complete directory, and
 * never a half-written one. An interrupted generation leaves only the temporary, which GC sweeps.
 */
export function withAtomicRunDir<T>(absFinalDir: string, retention: RetentionClass, body: (tempDir: string) => T): { result: T; lease: RunLease } {
  const runId = `${Date.now().toString(36)}-${agentId()}-${process.pid.toString(36)}`;
  const tempDir = `${absFinalDir}.${runId}.creating`;
  mkdirSync(dirname(absFinalDir), { recursive: true });
  mkdirSync(tempDir, { recursive: true });
  const now = new Date().toISOString();
  let lease = writeLease(tempDir, { runId, agentId: agentId(), pid: process.pid, state: "creating", createdAt: now, heartbeatAt: now, retention });
  try {
    const result = body(tempDir);
    lease = writeLease(tempDir, { ...lease, state: "complete", heartbeatAt: new Date().toISOString() });
    rmSync(absFinalDir, { recursive: true, force: true });
    renameSync(tempDir, absFinalDir);
    return { result, lease };
  } catch (error) {
    writeLease(tempDir, { ...lease, state: "failed", retention: "failure-evidence", heartbeatAt: new Date().toISOString() });
    throw error;
  }
}
//#endregion 🔐️Lease

//#region 🗝️RunKey
/** 🗝️ Everything a reusable run result is keyed by. Anything absent here can silently reuse a stale result. */
export type RunKeyComponents = Readonly<{
  baselineSha: string;
  mutationManifestDigest: string;
  fixtureManifestDigest: string;
  fixtureFileDigests: string;
  oracleLockDigest: string;
  oracleEngineDigest: string;
  probeDigest: string;
  comparisonProfileDigest: string;
  subjectDigest: string;
  platform: PlatformId;
  seed?: string | number;
  level: TestLevel;
}>;

/** 🗝️ The run key itself — a digest over the ordered component record. */
export function runKey(components: RunKeyComponents): string {
  return digest(JSON.stringify(canonicalize(components)));
}

/** 🗝️ The full run manifest written beside a published run directory. */
export type RunManifest = Readonly<{ schema: "semio.repository-test.run-manifest/v2"; runId: string; runKey: string; baselineSha: string; startedAt?: string; finishedAt?: string; retention: RetentionClass; components: RunKeyComponents; artifacts?: readonly ResultArtifact[] }>;

/**
 * 🗝️ Assembles the run key of one execution. The TOLERANCE PROFILE is folded into
 * `comparisonProfileDigest` deliberately: a loosened tolerance that reused an earlier parity verdict
 * would report a pass nobody ever measured.
 */
export function computeRunKey(opts: {
  baselineSha: string;
  manifest: MutationManifest | null;
  fixtures: readonly FixtureManifest[];
  oracle: OracleEntry | undefined;
  probes: readonly ProbeEntry[];
  comparison: ComparisonProfileSpec | undefined;
  pipeline: ComparisonPipeline | undefined;
  tolerance: ToleranceProfile | undefined;
  subjectDigest: string;
  platform: PlatformId;
  seed?: string | number;
  level: TestLevel;
}): { key: string; components: RunKeyComponents } {
  const components: RunKeyComponents = {
    baselineSha: opts.baselineSha,
    mutationManifestDigest: opts.manifest === null ? "" : mutationManifestDigest(opts.manifest),
    fixtureManifestDigest: setDigest(opts.fixtures.map((fixture) => [fixture.id, digest(JSON.stringify(canonicalize(fixture)))] as const)),
    fixtureFileDigests: setDigest(opts.fixtures.flatMap((fixture) => fixture.files.map((file) => [`${fixture.id}/${file.role}`, file.sha256] as const))),
    oracleLockDigest: opts.oracle === undefined ? "" : (opts.oracle.lockDigest ?? `${opts.oracle.package}@${opts.oracle.version ?? "*"}`),
    // ⚙️KIND and QUALIFICATION belong in the key, not beside it. Reclassifying an oracle from
    // `cross-semio-implementation` to `third-party-library`, or promoting a probe from `provisional` to
    // `qualified`, changes what a verdict MEANS — and without them in the key the reclassified run
    // reused the old verdict and the promotion was never actually measured.
    oracleEngineDigest: opts.oracle === undefined ? "" : `${opts.oracle.kind ?? "unclassified"}/${engineFamilyId(opts.oracle.engine)}@${opts.oracle.engine?.version ?? "*"}`,
    probeDigest: setDigest(opts.probes.map((probe) => [probe.id, `${probe.qualification?.status ?? "unqualified"}/${probe.lockDigest ?? `${probe.package}@${probe.version ?? "*"}`}`] as const)),
    comparisonProfileDigest: digest(JSON.stringify(canonicalize({ profile: opts.comparison ?? null, pipeline: opts.pipeline ?? null, tolerance: opts.tolerance ?? null }))),
    subjectDigest: opts.subjectDigest,
    platform: opts.platform,
    seed: opts.seed,
    level: opts.level,
  };
  return { key: runKey(components), components };
}
//#endregion 🗝️RunKey

//#region 🧾️ContractV2
/**
 * 🧾️ The v2 mutation-completeness gate. It compares PRODUCTION DISPATCH, the owner manifest and the
 * claimed test inventory for exact equality, which is the thing v1 could not do: v1's audit compared
 * a catalog with checked-in physical evidence and said so in its own comment, so a mutation reachable
 * through dispatch but missing from the catalog left no trace anywhere.
 */
export function mutationInventoryBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const ownerOf = new Map<string, string[]>();

  for (const contribution of registry.contributions) {
    for (const [index, manifest] of contribution.mutationManifests.entries()) {
      const problems = mutationManifestProblems(manifest, contribution.owner);
      for (const problem of problems) {
        breaches.push(breach("testing/contract", "mutation-manifest-invalid", contribution.manifestPath, `mutationManifests[${index}] ${problem}`, "A manifest is the authority the runtime and the tests are both measured against; a malformed one measures nothing.", "Correct the manifest record."));
      }
      if (problems.length > 0) continue;

      const claimed = registry.mutationCatalogs.filter((catalog) => catalog.capability !== "" && manifest.mutations.some((mutation) => mutation.capability === catalog.capability)).flatMap((catalog) => catalog.kinds);
      const runtime = readRuntimeInventory(repoRoot, manifest);
      const equality = compareInventories(manifest, runtime, claimed);

      if (equality.runtimeMissing) {
        breaches.push(breach("testing/contract", "runtime-inventory-missing", contribution.manifestPath, `No runtime inventory has been produced for ${equality.target}`, "Runtime completeness is a measurement, not a claim. Without the production bridge's own answer the manifest is unverified.", "Run `test inventory --artifact <id> --standard <v> --subset <s>` so the production bridge emits its inventory."));
      }
      for (const id of equality.runtimeOnly) {
        breaches.push(breach("testing/contract", "runtime-only-mutation", contribution.manifestPath, `${equality.target}: mutation ${id} is reachable through production dispatch but appears in no owner manifest`, "An unowned executable mutation is untested surface that no coverage number can see.", "Add it to the owning subset's manifest with its outcomes, dispatch and oracle requirement."));
      }
      for (const id of equality.manifestOnly) {
        breaches.push(breach("testing/contract", "manifest-only-mutation", contribution.manifestPath, `${equality.target}: mutation ${id} is declared but production dispatch does not offer it`, "A manifest row with no dispatch behind it inflates coverage with a mutation nobody can invoke.", "Remove the row, or implement and register the dispatch variant."));
      }
      for (const id of equality.testOnly) {
        breaches.push(breach("testing/contract", "test-only-mutation", contribution.manifestPath, `${equality.target}: the test catalog claims mutation ${id}, which no manifest owns`, "A test-only kind measures the test suite against itself.", "Add the mutation to its owning manifest, or drop the catalog kind."));
      }
      for (const mismatch of equality.outcomeMismatches) {
        breaches.push(breach("testing/contract", "mutation-outcome-mismatch", contribution.manifestPath, `${equality.target}: mutation ${mismatch.mutation} declares outcomes [${mismatch.manifest.join(", ")}] but dispatch reports [${mismatch.runtime.join(", ")}]`, "A fixture declares one expected outcome class; if the declared set disagrees with dispatch, some reachable outcome has no fixture at all.", "Align the manifest's outcomes with the dispatch descriptor."));
      }
      for (const mismatch of equality.variantMismatches) {
        breaches.push(breach("testing/contract", "mutation-variant-mismatch", contribution.manifestPath, `${equality.target}: mutation ${mismatch.mutation} names dispatch variant ${mismatch.manifest} but the runtime reports ${mismatch.runtime}`, "Name-checked dispatch is what keeps manifest↔runtime equality from silently degrading into an order comparison.", "Correct productionDispatch.variant."));
      }

      for (const mutation of manifest.mutations) {
        const subset = owningSubsetOf(manifest, mutation);
        const coordinate = subsetCoordinate({ artifact: manifest.artifact, standard: manifest.standard, subset });
        // 🪆️THE KEY MUST CARRY THE SUBSET. Without it this check is artifact-level inside a platform whose
        // entire point is subset-level scoping, and it reports a DUPLICATE whenever two genuinely different
        // subsets of one artifact happen to share a mutation name — `semio@v1/cad` and `semio@v1/document`
        // both declaring `set-snapshot`, `brep` and `mesh` both declaring `move-vertex`. Those are distinct
        // mutations of distinct scopes, which is exactly what the taxonomy is for; only the SAME mutation
        // of the SAME subset claimed by two manifests is a real duplicate.
        const key = `${coordinate}::${mutation.id}`;
        ownerOf.set(key, [...(ownerOf.get(key) ?? []), `${contribution.manifestPath}#${coordinate}`]);
        if (isWildcardSubset(subset)) {
          const siblings = (declaredSubsets(repoRoot).get(`${manifest.artifact}@${manifest.standard}`) ?? []).filter((candidate) => !isWildcardSubset(candidate));
          if (siblings.length > 0) {
            // 🚫️The artifact HAS narrower scopes and this mutation declined them. Hard failure.
            breaches.push(breach("testing/contract", "wildcard-subset-owner", contribution.manifestPath, `Mutation ${mutation.id} is owned by wildcard subset ${JSON.stringify(subset)} while ${manifest.artifact}@${manifest.standard} declares ${siblings.length} real subset(s): ${siblings.join(", ")}`, "Testing at artifact level hides which part of the artifact a mutation actually changed, and lets one broad case stand in for every narrow one.", `Give the mutation its smallest owner among ${siblings.join(", ")}, or declare an explicit typed compound.`));
          } else if (!subsetPolicyIsSingle(repoRoot, manifest.artifact, manifest.standard)) {
            // 🪆️The artifact declares NO narrower scope at all. That is sometimes the truth — RFC 8259
            // JSON has no conformance classes — and sometimes an artifact that simply has not been
            // split yet, which is the case the whole subset-scoping requirement exists for: `s.cad.cad`
            // addresses shape, building, energy, structure, drawing, node and reference scopes through
            // ONE `*` bucket. The two are indistinguishable from here, so this is a MEDIUM finding
            // asking the owner to decide, not a hard failure asserting which one it is. Recording
            // `"subsetPolicy": "single"` with a rationale in the artifact's own 🪆️subsets component
            // settles it and silences this.
            breaches.push(breach("testing/contract", "unsplit-artifact-subset", contribution.manifestPath, `Mutation ${mutation.id} is owned by ${JSON.stringify(subset)} and ${manifest.artifact}@${manifest.standard} declares no narrower subset at all`, "A mutation is owned by the SMALLEST semantic subset. An artifact with a single catch-all subset is either a format that genuinely has no narrower scope, or one that has not been split yet — and until the owner says which, every mutation on it is scoped to the whole artifact.", `Declare the real subsets of ${manifest.artifact}@${manifest.standard} and give this mutation its smallest owner, or record "subsetPolicy": "single" with a rationale in that artifact's 🪆️subsets component.`, "medium"));
          }
        }
        breaches.push(...oracleRequirementBreaches(registry, contribution.manifestPath, manifest, mutation));
      }
    }
  }

  for (const [key, owners] of ownerOf) {
    if (owners.length > 1) {
      breaches.push(breach("testing/contract", "duplicate-mutation-owner", owners[0]!, `Mutation ${key} is owned by ${owners.length} manifests: ${owners.join(", ")}`, "Exactly one smallest semantic subset owns a mutation; two owners means two coverage numbers for one behaviour and no way to say which is right.", "Keep the narrowest owner and remove the others."));
    }
  }
  return breaches;
}

/**
 * ✅️ Every mutation needs a QUALIFYING third-party oracle. A second implementation written inside this
 * repository from this repository's own schemas is registered as `cross-semio-implementation` and is a
 * required supplement, never a substitute: both halves read the same specification, so a misreading
 * of it produces two agreeing wrong answers.
 */
export function oracleRequirementBreaches(registry: OracleRegistry, scope: string, manifest: MutationManifest, mutation: ManifestMutation): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const coordinate = subsetCoordinate(manifestTarget(manifest, mutation));
  for (const requirement of mutation.oracleRequirements) {
    const supplying = registry.oracles.filter((oracle) => oracle.capabilities.includes(requirement.capability));
    const qualifying = supplying.filter((oracle) => isQualifyingOracleKind(oracle.kind));
    if (qualifying.length === 0) {
      const supplemental = supplying.map((oracle) => `${oracle.id} (${oracle.kind ?? "unclassified"})`);
      breaches.push(
        breach(
          "testing/oracle",
          "missing-external-oracle",
          scope,
          `${coordinate}: mutation ${mutation.id} requires a ${requirement.qualifyingKind} for capability ${requirement.capability}, and none is registered${supplemental.length > 0 ? ` (only ${supplemental.join(", ")})` : ""}`,
          "An expected result produced by reimplementing our own semantics proves the two implementations agree, not that either is right. Metamorphic, inverse, round-trip and property tests are required supplements and none of them discharges this.",
          `Register a ${QUALIFYING_ORACLE_KINDS.join(" / ")} that declares capability ${requirement.capability}, or block the mutation's release until one exists.`,
        ),
      );
      continue;
    }
    const families = new Set(qualifying.map((oracle) => engineFamilyId(oracle.engine)));
    const required = requirement.distinctEngineFamilies ?? 1;
    if (families.size < required) {
      breaches.push(
        breach(
          "testing/oracle",
          "insufficient-engine-independence",
          scope,
          `${coordinate}: mutation ${mutation.id} requires ${required} independent engine families for ${requirement.capability} but the registered oracles span ${families.size} (${[...families].sort().join(", ")})`,
          "Two wrappers around one kernel are one oracle. Counting them as two turns a shared-kernel bug into a passing comparison.",
          "Qualify a reference on a different engine family, or lower the requirement deliberately with a recorded rationale.",
          "medium",
        ),
      );
    }
  }
  return breaches;
}

/**
 * 🌱️ `verified-native-second-implementation` is EARNED, never asserted. It may discharge a mutation's
 * external-oracle requirement ONLY when the registered entry carries, and this function verifies, every
 * one of: a semio-native artifact (`isSemioNativeArtifact` — never `s.stdio.*` other than `s.stdio.semio`
 * itself, so a format with a real third-party option is refused however good the entry's survey reads);
 * a structured negative search naming what was checked, not free rationale prose; a second-implementation
 * language that differs from the subject's own (a same-language or transliterated reference catches a
 * typo and nothing else — a misread specification still produces two agreeing wrong answers); a
 * specification source distinct from the subject's own code; fixture-backed vectors; and 100% capability
 * coverage of every manifest this same contribution owns for that format — a PARTIAL second
 * implementation must stay `cross-semio-implementation` and discharge nothing, exactly as it did before
 * this kind existed. Any entry missing or failing one of these fires a distinct, actionable breach rather
 * than a generic rejection, so a lazy or false claim fails loudly instead of silently passing.
 */
export function nativeSecondImplementationBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    for (const oracle of contribution.oracles) {
      if (oracle.kind !== "verified-native-second-implementation") continue;
      const scope = `${contribution.owner}#${oracle.id}`;
      const evidence = oracle.nativeSecondImplementation;
      if (evidence === undefined) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} claims verified-native-second-implementation with no nativeSecondImplementation evidence recorded`,
            "This kind discharges an external-oracle requirement exactly as a real third party would; the claim must be earned with recorded evidence, never merely asserted by the kind field alone.",
            "Record format, noThirdPartySurvey, subjectImplementationLanguage, secondImplementationLanguage, specificationSource and fixtureCoverage, or reclassify as cross-semio-implementation.",
          ),
        );
        continue;
      }
      if (!isSemioNativeArtifact(evidence.format)) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-not-native",
            scope,
            `${oracle.id} claims native status for ${JSON.stringify(evidence.format)}, which names a real interchange format`,
            "A format under s.stdio.* other than s.stdio.semio is one a genuine third party can implement, whatever this entry's survey argues; this kind is categorically unavailable there.",
            "Register a real third-party-library / third-party-cli / standards-reference-tool reference instead, or narrow the mutation's requirement.",
          ),
        );
        continue;
      }
      const owningManifests = contribution.mutationManifests.filter((manifest) => manifest.artifact === evidence.format);
      const manifestCapabilities = new Set(owningManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
      if (owningManifests.length === 0 || manifestCapabilities.size === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} claims format ${JSON.stringify(evidence.format)}, but this owner's own contribution manifests none of it`,
            "The claim must tie to a real, owned mutation vocabulary; a format nothing here manifests cannot be verified as fully covered.",
            "Point format at the artifact id this same contribution's own mutationManifests entry declares.",
          ),
        );
        continue;
      }
      const ownCapabilities = new Set(oracle.capabilities);
      const uncovered = [...manifestCapabilities].filter((capability) => !ownCapabilities.has(capability));
      if (uncovered.length > 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-partial-coverage",
            scope,
            `${oracle.id} covers ${ownCapabilities.size} of ${manifestCapabilities.size} capabilities ${JSON.stringify(evidence.format)} manifests; uncovered: ${uncovered.join(", ")}`,
            "A partial second implementation must stay cross-semio-implementation and discharge nothing — the 100% requirement exists precisely so this kind cannot be used to quietly cover part of a vocabulary while looking complete.",
            "Extend the reference's capabilities to the full manifest vocabulary, or reclassify as cross-semio-implementation.",
          ),
        );
        continue;
      }
      const survey = evidence.noThirdPartySurvey;
      const candidatesProblem = survey === undefined || survey.ecosystemsSearched.length === 0 || survey.candidatesConsidered.length === 0 || survey.candidatesConsidered.some((candidate) => candidate.package.trim().length === 0 || candidate.reason.trim().length < 10);
      if (candidatesProblem) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no credible noThirdPartySurvey`,
            "A negative search is not credible unless it names the ecosystems it looked in and, for each candidate it declined, a real structural reason — a rationale-free claim that 'nothing exists' is exactly the unfalsifiable assertion this field exists to rule out.",
            "Record ecosystemsSearched and at least one candidatesConsidered entry with a real package name and a structural reason (10+ characters) it does not qualify.",
          ),
        );
        continue;
      }
      const subjectLanguage = evidence.subjectImplementationLanguage.trim().toLowerCase();
      const secondLanguage = evidence.secondImplementationLanguage.trim().toLowerCase();
      if (subjectLanguage.length === 0 || secondLanguage.length === 0 || subjectLanguage === secondLanguage) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-same-language",
            scope,
            `${oracle.id} declares subjectImplementationLanguage ${JSON.stringify(evidence.subjectImplementationLanguage)} and secondImplementationLanguage ${JSON.stringify(evidence.secondImplementationLanguage)}`,
            "A reference written in the subject's own language, or transliterated from it, catches a typo and nothing else: both halves would still read the same specification, so a misreading of it produces two agreeing wrong answers.",
            "Write the reference in a genuinely different language from the subject, and record both truthfully.",
          ),
        );
        continue;
      }
      if (evidence.specificationSource.trim().length === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no specificationSource`,
            "The whole discharge depends on this reference being read from a written specification, not from the subject's own source — an empty field cannot demonstrate that.",
            "Name the specification document(s) this reference was authored from.",
          ),
        );
        continue;
      }
      if (evidence.fixtureCoverage === undefined || evidence.fixtureCoverage.vectors <= 0 || evidence.fixtureCoverage.capabilitiesCovered.length === 0) {
        breaches.push(
          breach(
            "testing/oracle",
            "native-second-implementation-unearned",
            scope,
            `${oracle.id} records no fixture-backed vectors`,
            "Every mutation this kind discharges must be tested over fixtures — a real-world example, a handcrafted vector, or one generated by a qualifying oracle — never merely a language claim with no committed evidence behind it.",
            "Record how many committed vectors back this reference and which capabilities they exercise.",
          ),
        );
      }
    }
  }
  return breaches;
}

/**
 * 🧾️ Every mutation CAPABILITY the repository declares must be owned by a v2 mutation manifest.
 *
 * `mutationCatalogs` (v1's kind/vector vocabulary) and `mutationManifests` (v2's oracle/dispatch/outcome
 * vocabulary) were two independent registries with no cross-check, and `buildCoverageMatrix` reads only
 * the second. An owner could therefore be 100% v1-complete — every `mutate-<kind>` and `inverse-<kind>`
 * scenario present — while contributing ZERO rows to `test matrix --enforce`, the actual release gate.
 * Because the gated denominators pool across the whole registry, one properly-manifested owner kept
 * them non-empty and the omission never even tripped the empty-denominator guard: a whole capability
 * with un-oracled, un-inventoried mutations was not "missing", it was invisible.
 */
export function capabilityManifestBreaches(registry: OracleRegistry): BreachRecord[] {
  const manifested = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    for (const catalog of contribution.mutationCatalogs) {
      if (catalog.capability === "" || manifested.has(catalog.capability)) continue;
      breaches.push(
        breach(
          "testing/contract",
          "capability-without-manifest",
          contribution.manifestPath,
          `Catalog ${catalog.id} declares capability ${catalog.capability} (${catalog.kinds.length} kind(s)) and no mutation manifest owns it`,
          "The coverage matrix is enumerated from manifests. A capability with no manifest contributes no rows at all, so its mutations are not reported as uncovered — they are absent from the denominator and invisible to every release gate.",
          `Add a mutationManifests entry for ${catalog.capability} declaring each kind's subset, outcomes, production dispatch and oracle requirement.`,
        ),
      );
    }
  }
  return breaches;
}

/** 🧾️ Validators for the v2 registry records that shipped with none — a record nothing checks is a record that can say anything. */
export function registryRecordBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const probes = probeTable(registry);
  const tolerances = toleranceProfileTable(registry);
  for (const probe of registry.probes) {
    if (!MANIFEST_MUTATION_ID_RE.test(probe.id)) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe id ${JSON.stringify(probe.id)} is not kebab-case`, "Probe ids are referenced by every pipeline stage; a free-form id cannot be joined reliably.", "Rename the probe."));
    if (probe.capabilities.length === 0) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} declares no capabilities`, "A probe that claims no capability can never be selected deliberately.", "Declare what it measures."));
    if (probe.outputSchema !== "semio.repository-test.probe-report/v2") breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} declares outputSchema ${JSON.stringify(probe.outputSchema)}`, "The orchestrator only evaluates the typed report shape it knows.", 'Emit "semio.repository-test.probe-report/v2".'));
    if (probe.qualification !== undefined && probe.qualification.evidence.trim().length === 0) breaches.push(breach("testing/contract", "probe-record-invalid", probe.id, `Probe ${probe.id} claims qualification ${probe.qualification.status} with no evidence`, "A qualification status is a claim about a measurement somebody made; without the evidence it is an assertion.", "Record what was measured, and where."));
  }
  for (const profile of registry.toleranceProfiles) {
    if (!MANIFEST_MUTATION_ID_RE.test(profile.id)) breaches.push(breach("testing/contract", "tolerance-record-invalid", profile.id, `Tolerance profile id ${JSON.stringify(profile.id)} is not kebab-case`, "Profile ids are joined from fixtures and pipelines.", "Rename the profile."));
    for (const [key, value] of Object.entries(profile)) {
      if (typeof value === "number" && !Number.isFinite(value)) breaches.push(breach("testing/contract", "tolerance-record-invalid", profile.id, `Tolerance profile ${profile.id} field ${key} is not finite`, "A non-finite tolerance accepts everything.", "Give it a finite value."));
    }
  }
  for (const pipeline of registry.comparisonPipelines) {
    if (pipeline.stages.length === 0) breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} declares no stages`, "A pipeline with no stage compares nothing and reports equal.", "Declare at least one stage."));
    if (pipeline.toleranceProfile !== undefined && !tolerances.has(pipeline.toleranceProfile)) {
      breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} names unknown tolerance profile ${pipeline.toleranceProfile}`, "An unresolved profile means every dimensional assertion is resolved against nothing.", `Use one of ${[...tolerances.keys()].sort().join(", ")}.`));
    }
    if (pipeline.stages.every((stage) => stage.optional === true)) {
      breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Every stage of pipeline ${pipeline.id} is optional`, "A pipeline whose every stage is excusable gates nothing while reporting a verdict.", "Make at least one stage gating."));
    }
    for (const [index, stage] of pipeline.stages.entries()) {
      const probe = probes.get(stage.probe);
      if (probe === undefined) {
        breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} stage ${index} names unregistered probe ${stage.probe}`, "An unregistered probe has no licence, pin, engine family or qualification on record, and the stage cannot run.", "Register the probe, or remove the stage."));
        continue;
      }
      // 🚫️`optional` is for a probe that is registered and NOT YET QUALIFIED. Marking a qualified
      // probe's stage optional would excuse that probe's own reported failure.
      if (stage.optional === true && isQualifiedProbe(probe)) {
        breaches.push(breach("testing/contract", "optional-stage-overclaimed", pipeline.id, `Pipeline ${pipeline.id} stage ${index} marks QUALIFIED probe ${stage.probe} optional`, "`optional` exists for a probe whose qualification spike has not passed. On a qualified probe it would excuse that probe's own status: failed, which is the one thing this protocol must never do.", "Remove `optional`, or downgrade the probe's qualification honestly."));
      }
      if (stage.inputs.length === 0) breaches.push(breach("testing/contract", "pipeline-record-invalid", pipeline.id, `Pipeline ${pipeline.id} stage ${index} reads no inputs`, "A stage with no input measures nothing.", "Name the artifact roles it reads."));
    }
  }
  return breaches;
}

/**
 * 🎭️ A declared export dialect whose serializer does not actually emit that format.
 *
 * This is the root cause beneath a large share of "no qualifying oracle": a carrier oracle works by
 * having a third-party reader of a STANDARD FORMAT verify what a mutation produced — `brepjs` reads the
 * STEP a Boolean should have written. That is impossible when the exporter writes the artifact's own
 * internal DSL text under the standard format's extension, because no third-party reader of PNG, OBJ,
 * STL or Markdown can parse it. The artifact declares an export capability it does not have.
 *
 * 45% of the repository's serializers are in this state (85 of 187), and Markdown and DXF have no real
 * exporter at all. The finding is severity-graded rather than uniform: for a BINARY format, DSL text is
 * unambiguously not that format; for a text format it is still wrong but a reader may at least not
 * crash, so it is reported one level lower rather than asserted with false confidence.
 */
export const BINARY_CARRIER_FORMATS: readonly string[] = ["png", "jpg", "jpeg", "gif", "tiff", "bmp", "stl", "ply", "las", "gltf", "glb", "zip", "pdf", "dwg", "docx", "xlsx", "pptx", "mp3", "mp4", "wav"];

/** 🧪️ Strip `#[cfg(test)]` modules before judging a serializer. Their round-trip PROOFS legitimately call
 *  the very functions a stub detector watches for, so scanning them flagged two genuinely real carriers —
 *  `semio@v1/cad → step` and `semio@v1/drawing → svg` — as stubs. A gate must be wrong in neither
 *  direction: a false stub hides a usable carrier exactly as a missed stub invents one. */
function withoutTestModules(text: string): string {
  let out = "";
  let at = 0;
  for (;;) {
    const found = text.indexOf("#[cfg(test)]", at);
    if (found === -1) return out + text.slice(at);
    out += text.slice(at, found);
    const open = text.indexOf("{", found);
    if (open === -1) return out;
    let depth = 0;
    let i = open;
    for (; i < text.length; i += 1) {
      if (text[i] === "{") depth += 1;
      else if (text[i] === "}") {
        depth -= 1;
        if (depth === 0) break;
      }
    }
    at = i + 1;
  }
}

/** 🕳️ A serializer whose body NEVER READS ITS INPUT cannot represent a mutation, whatever else it does.
 *  This one predicate subsumes several shapes that were each hiding separately: an `Err("not
 *  implemented")` stub, a bridge returning `XmlDocument::default()` regardless of argument, and a DWG
 *  leaf that ignores its parameter and returns a hardcoded empty document. Detecting the CAUSE rather
 *  than each spelling is what keeps the gate from needing a new rule per author. */
function ignoresItsInput(text: string): boolean {
  for (const match of text.matchAll(/fn\s+serialize(?:_bytes|_text)?\s*(?:<[^>]*>)?\s*\(([^)]*)\)/g)) {
    const parameter = (match[1] ?? "").split(",")[0]?.trim() ?? "";
    const name = parameter.split(":")[0]?.trim().replace(/^(mut|&)\s*/, "") ?? "";
    if (!name || name === "self" || name === "&self") continue;
    const open = text.indexOf("{", (match.index ?? 0) + match[0].length);
    if (open === -1) continue;
    let depth = 0;
    let i = open;
    for (; i < text.length; i += 1) {
      if (text[i] === "{") depth += 1;
      else if (text[i] === "}") {
        depth -= 1;
        if (depth === 0) break;
      }
    }
    const body = text.slice(open + 1, i);
    // 🧭️`let _ = X;` is the idiom these stubs use to silence the unused-parameter warning; it is not a read.
    const reads = new RegExp(`\\b${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`).test(body.replace(new RegExp(`let\\s+_\\s*=\\s*${name}\\s*;`, "g"), ""));
    if (!reads) return true;
  }
  return false;
}

/**
 * 🫥️ A QUALIFYING ORACLE MUST NOT BE THIS REPOSITORY'S OWN SECOND IMPLEMENTATION WEARING A LIBRARY'S NAME.
 *
 * The shape this catches, and it was found in five owners covering 156 mutations: an entry registered
 * `third-party-library` on the strength of a real crate — `json`, `png`, `image` — where the crate only
 * parses or encodes, and WHAT THE MUTATION SHOULD PRODUCE is computed by the owner's own
 * `🦀️oracle.rs`. `gltf`'s implemented seven kinds and refused the other 113 while the
 * manifest named it against all 120; `png`'s says in its own doc comment that it "deliberately mirrors"
 * the production `diff`. Both halves then read the same specification, so a misreading of it yields two
 * agreeing wrong answers — which is the one failure a differential test exists to prevent.
 *
 * A crate that decodes the artifact discharges "the result is a well-formed file of this format". It does
 * not discharge "the mutation computed the right answer". Only the second is what a mutation oracle is for.
 *
 * 🎯 ENTRY-GRANULAR, NOT FILE-GRANULAR. This owner's shared `🦀️oracle.rs` is exactly one Rust
 * compilation unit; a `verified-native-second-implementation`/`third-party-library`/… entry can only be
 * THE CODE this file's predicting dispatch belongs to when that entry's own `ecosystem` is `"rust"` —
 * anything else (`python`'s `ifcopenshell`, `javascript`'s `yauzl`, …) executes in an entirely different
 * runtime the file never touches, so the file's content is not evidence about it, however incriminating
 * that same file is about a Rust sibling entry sitting in the very same registry contribution. Filtering
 * to `implicable` BEFORE reading the file — rather than reading it once and then deciding which of
 * `qualifying`'s ids to name in the message — is what keeps a genuine third-party reader from being swept
 * in with a mislabelled Rust reimplementation merely for sharing a directory: the file is not even opened
 * unless some entry could actually be the code it describes.
 */
export function reimplementationOracleBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const contribution of registry.contributions) {
    const qualifying = contribution.oracles.filter((oracle) => isQualifyingOracleKind(oracle.kind));
    const implicable = qualifying.filter((oracle) => oracle.ecosystem === "rust");
    if (implicable.length === 0) continue;
    const own = join(repoRoot, contribution.owner, "🧪️oracle", "🦀️.rs");
    if (!existsSync(own)) continue;
    let text: string;
    try {
      text = readFileSync(own, "utf8");
    } catch {
      continue;
    }
    const predicts = /fn\s+(apply_kind|apply)\s*\(/.test(text) && /has no oracle implementation/.test(text);
    const admits = /independent implementation|second implementation|deliberately mirrors|reimplemented from scratch/i.test(text);
    if (!predicts && !admits) continue;
    // 📖️AN OWNER MAY HOLD BOTH MECHANISMS AT ONCE, and the distinction is the whole point. A predicting
    // `🦀️oracle.rs` sitting in the directory does not taint a qualifying oracle that is
    // judged by READER probes over committed fixtures — there the expected state is never computed, it
    // is the `after` half of a byte-reproducible fixture, and a third-party library parses both sides.
    // Flagging on the file's mere presence could not tell "a genuine new reader" from "the old mistake",
    // which is exactly what the gltf retrofit reported back. A registered comparison pipeline over
    // QUALIFIED probes is the discriminator: it means something other than our own Rust is the judge.
    const judgedByProbes =
      contribution.comparisonPipelines.length > 0 &&
      contribution.probes.length > 0 &&
      contribution.probes.some((probe) => probe.qualification?.status === "qualified" || (probe as { qualified?: boolean }).qualified === true);
    if (judgedByProbes) continue;
    breaches.push(
      breach(
        "testing/oracle",
        "reimplementation-registered-as-third-party",
        `${contribution.owner}/🦀️oracle.rs`,
        `${implicable.map((oracle) => oracle.id).join(", ")} is registered as a qualifying third-party oracle, but this owner predicts mutation output in its own Rust`,
        `The crate parses or encodes; the expected RESULT of each mutation is computed here. Both halves of the comparison then read the same specification, so a misreading of it produces two agreeing wrong answers — the one failure mode a differential test exists to prevent.`,
        `Reclassify the entry as cross-semio-implementation (a required supplement, never a substitute) and register a domain-aware third-party reader for the mutation semantics, or narrow the crate's capability to file well-formedness only.`,
        "high",
      ),
    );
  }
  return breaches;
}

/** 🏭️ A fixture whose `after` half was written by OUR code is not evidence, whatever the registration says.
 *
 * A generator may legitimately link a third-party crate to lay out a seed document and to serialise.
 * What it may never do is obtain the MUTATED bytes, or the expected PROJECTION of them, from this
 * repository's own mutation engine: then both halves of the comparison descend from one implementation
 * and a shared misreading of the specification yields two agreeing wrong answers. The third-party
 * library is reduced to a codec for our own answer.
 *
 * Keyed on the disqualifying SYMBOLS rather than on any `semio-*` dependency, because a generator that
 * merely borrows a JSON helper from the harness is not writing fixtures with our semantics, and a gate
 * that cannot tell those apart is one people learn to ignore.
 *
 * @see ../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pdf-fixtures-are-not-admissible-evidence.md
 */
export function fixtureWriterProvenanceBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const disqualifyingAll = /\b(oracle_apply_mutation|oracle_apply|apply_mutation|project_conformance|oracle_inverse_spec|oracle_round_trip)\b/g;
  for (const contribution of registry.contributions) {
    if (contribution.fixtureManifests.length === 0) continue;
    const generator = join(repoRoot, contribution.owner, "🏭️generator");
    if (!existsSync(generator)) continue;
    const offenders = new Set<string>();
    walkDirectories(generator, (abs, rel) => {
      if (rel.includes("target")) return "skip";
      let names: string[];
      try {
        names = readdirSync(abs);
      } catch {
        return "enter";
      }
      for (const name of names) {
        if (!name.endsWith(".rs")) continue;
        let text: string;
        try {
          text = readFileSync(join(abs, name), "utf8");
        } catch {
          continue;
        }
        for (const line of text.split("\n")) {
          if (!line.startsWith("use ") || !/\bsemio_/.test(line)) continue;
          for (const found of line.matchAll(disqualifyingAll)) offenders.add(found[0]);
        }
      }
      return "enter";
    });
    if (offenders.size === 0) continue;
    breaches.push(
      breach(
        "testing/fixture",
        "fixture-after-state-written-by-our-own-code",
        `${contribution.owner}/🏭️generator`,
        `${contribution.fixtureManifests.length} committed fixture(s) obtain their mutated state from this repository's own engine (${[...offenders].sort().join(", ")})`,
        `The generator imports the mutation application and/or the expected projection from a semio crate, so the \`after\` half of every pair here is our own computation wearing a third-party crate's name. A reader registered over these fixtures would be judging a state we predicted, which is the exact substitution Protocol v2 exists to forbid.`,
        `Apply each mutation through the third-party library's OWN public API in the generator and drop the semio import, so the after bytes are written by something other than us; only then register a reader oracle over them. Writer and reader being the same third-party library is fine and is the established precedent.`,
        "high",
      ),
    );
  }
  return breaches;
}

export function stubSerializerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (!rel.includes("🧵️serializers")) return "enter";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".rs")) continue;
      let text: string;
      try {
        text = withoutTestModules(readFileSync(join(abs, entry.name), "utf8"));
      } catch {
        continue;
      }
      // 🧭️Any serializing entry point counts, not `serialize_bytes` alone. Several owners deleted their
      // byte path entirely and kept only `serialize_text` returning `print_dsl` — cad exports STL, OBJ,
      // glTF, IFC and STEP that way. Keying the gate on `serialize_bytes` skipped every one of them and
      // reported the whole owner as having real carriers.
      if (!/fn\s+serialize(_bytes|_text)?\s*[(<]/.test(text)) continue;
      const printsDsl = /print_dsl\s*\(/.test(text);
      // 🕳️THE SECOND STUB SHAPE, and the worse of the two. `encode_pack` the SOURCE snapshot, then
      // `decode_pack` those very bytes AS THE TARGET type — one artifact's binary envelope reinterpreted
      // as another's. It is not an unimplemented export, it is type confusion: at best it fails on the
      // envelope id, at worst a lenient decode yields a structurally valid document full of noise. A gate
      // that only looked for `print_dsl` read these as REAL exporters, which is how they survived.
      const transmutes = /encode_pack\s*\(\s*[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text) && /decode_pack\s*\(\s*&?[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text);
      // 🫥️THE QUIETEST SHAPE OF ALL. `serde_json::to_value` the source, then `from_value` into the target.
      // Every non-`schema` field of these target snapshots carries `#[serde(default)]` and nothing rejects
      // unknown keys, so deserialization SUCCEEDS and yields an EMPTY document — `architect/program`'s
      // xlsx export turns 266 mutable registers into a workbook with no sheets, and reports Ok. It neither
      // prints DSL nor transmutes an envelope, so both earlier detectors passed it, and 266 mutations were
      // counted as having a real carrier on the strength of an exporter that can never show a mutation.
      const coerces = /to_value\s*\(\s*&?[A-Za-z_][A-Za-z0-9_]*\s*\)/.test(text) && /from_value\s*[:(<]/.test(text);
      const inert = ignoresItsInput(text);
      if (!printsDsl && !transmutes && !coerces && !inert) continue;
      const format = rel.match(/🗿️artifacts\/[^/]*?([a-z0-9]+)\/🔖️/)?.[1] ?? "";
      // 🧭️An artifact whose own carrier IS text or JSON may legitimately print its DSL; the finding is
      // about a STANDARD format that the DSL is not.
      if (format === "txt" || format === "json" || format === "") continue;
      const binary = BINARY_CARRIER_FORMATS.includes(format);
      breaches.push(
        breach(
          "testing/contract",
          "stub-serializer",
          `${rel}/${entry.name}`,
          printsDsl
            ? `The ${format} serializer emits the artifact's internal DSL text, not ${format}`
            : transmutes
              ? `The ${format} serializer reinterprets this artifact's own pack bytes as a ${format} snapshot`
              : coerces
                ? `The ${format} serializer coerces this artifact through serde into an empty ${format} document`
                : `The ${format} serializer never reads its input`,
          inert && !printsDsl && !transmutes && !coerces
            ? `Its serialize function never mentions its own input parameter, so its output cannot depend on the snapshot and no mutation can ever be observable in it. This covers an explicit not-implemented stub, a bridge returning Default::default() whatever it is given, and a leaf that silences the unused-parameter warning with \`let _ = x;\` and then ignores it.`
            : coerces && !printsDsl && !transmutes
            ? `It serializes the source snapshot to a serde_json Value and deserializes that Value into the ${format} snapshot type. Because the target's fields carry #[serde(default)] and unknown keys are ignored, this SUCCEEDS and produces an empty ${format} document for every input — so no mutation is ever observable in the bytes, and the export reports Ok while carrying nothing.`
            : transmutes && !printsDsl
            ? `It encodes the source snapshot with encode_pack and decodes those same bytes as the ${format} snapshot type. That is not an export, it is a reinterpretation of one artifact's envelope as another's — it either fails on the envelope id or yields a structurally valid ${format} document whose content is unrelated to the source. No third-party ${format} reader can verify a mutation through it.`
            : binary
            ? `${format} is a binary format and DSL text is not it. The artifact declares an export dialect it does not implement, so no third-party reader of ${format} can verify what a mutation produced — which is exactly how a carrier oracle works.`
            : `The artifact declares a ${format} export dialect and writes its own DSL text instead. A third-party ${format} reader cannot verify what a mutation produced.`,
          `Implement the ${format} serializer, or remove ${format} from this subset's exportDialects so the capability is not claimed.`,
          transmutes || coerces || inert || binary ? "high" : "medium",
        ),
      );
    }
    return "enter";
  });
  return breaches;
}

/** 🚫️ A no-oracle decision may never stand in for a mutation's external-oracle requirement. */
export function noOracleMisuseBreaches(registry: OracleRegistry): BreachRecord[] {
  const mutationCapabilities = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.flatMap((mutation) => [mutation.capability, ...mutation.oracleRequirements.map((requirement) => requirement.capability)])));
  const breaches: BreachRecord[] = [];
  for (const decision of registry.noOracleDecisions) {
    const covered = decision.capabilities.filter((capability) => mutationCapabilities.has(capability));
    if (covered.length > 0) {
      breaches.push(breach("testing/oracle", "no-oracle-covers-mutation", decision.id, `No-oracle decision ${decision.id} claims mutation capability/capabilities ${covered.join(", ")}`, "A runtime mutation's oracle requirement is discharged only by a qualifying third-party reference; recording a decision instead would make the gap invisible rather than blocking.", "Remove the mutation capability from the decision and register a qualifying oracle, or block the mutation."));
    }
  }
  return breaches;
}

/** 🧫️ Fixture provenance, digest, licence and reproducibility — every one a hard contract condition. */
export function fixtureProvenanceBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const oracles = new Map(registry.oracles.map((oracle) => [oracle.id, oracle]));
  // 📏️A fixture naming a tolerance profile nobody defines is a fixture whose every dimensional
  // assertion resolves against nothing — the name reads as policy while carrying none.
  const tolerances = toleranceProfileTable(registry);
  const profiles = profileTable(registry);
  for (const contribution of registry.contributions) {
    for (const [index, fixture] of contribution.fixtureManifests.entries()) {
      const scope = contribution.manifestPath;
      for (const problem of fixtureManifestProblems(fixture, repoRoot)) {
        breaches.push(breach("testing/fixture", "fixture-manifest-invalid", scope, `fixtureManifests[${index}] ${problem}`, "A fixture is evidence; evidence with incomplete provenance cannot be audited or reproduced.", "Complete the fixture manifest record."));
      }
      for (const verification of verifyFixture(repoRoot, fixture)) {
        if (verification.missing) breaches.push(breach("testing/fixture", "fixture-file-missing", `${scope}#${fixture.id}/${verification.role}`, `Fixture file ${verification.path} does not exist`, "A manifest that names a file nobody can read is a coverage claim with no artifact behind it.", "Restore the file or remove the role."));
        else if (!verification.ok) breaches.push(breach("testing/fixture", "fixture-digest-mismatch", `${scope}#${fixture.id}/${verification.role}`, `Fixture file ${verification.path} hashes ${verification.actual}, manifest says ${verification.expected}`, "Source fixtures are immutable after review; a changed fixture is a NEW identity, never an edit in place.", "Restore the reviewed bytes, or mint a new fixture id and digest."));
      }
      if (fixture.class === "third-party-generated" && fixture.generator !== undefined) {
        const oracle = oracles.get(fixture.generator.oracle);
        if (oracle === undefined) breaches.push(breach("testing/fixture", "fixture-generator-unregistered", `${scope}#${fixture.id}`, `Fixture generator oracle ${fixture.generator.oracle} is not registered`, "A generated fixture's authority is the tool that generated it; an unregistered tool has no licence, pin or engine family on record.", "Register the generating oracle."));
        else if (!isQualifyingOracleKind(oracle.kind)) breaches.push(breach("testing/fixture", "fixture-generated-by-non-qualifying-oracle", `${scope}#${fixture.id}`, `Fixture ${fixture.id} is generated by ${oracle.id}, whose kind is ${oracle.kind ?? "unclassified"}`, "An expected result produced by our own second implementation is not third-party evidence.", `Regenerate it with a ${QUALIFYING_ORACLE_KINDS.join(" / ")}.`));
        if (!fixture.reproducible) breaches.push(breach("testing/fixture", "fixture-not-reproducible", `${scope}#${fixture.id}`, `Generated fixture ${fixture.id} is marked non-reproducible`, "A generated expectation nobody can regenerate is indistinguishable from a hand-edited one.", "Make generation deterministic, or document the external canonicalization that makes it reproducible.", "medium"));
      }
      if (fixture.toleranceProfile !== undefined && !tolerances.has(fixture.toleranceProfile)) {
        breaches.push(breach("testing/fixture", "fixture-tolerance-profile-unknown", `${scope}#${fixture.id}`, `Fixture ${fixture.id} names tolerance profile ${fixture.toleranceProfile}, which no profile defines`, "An unresolved profile means every dimensional assertion for this fixture is resolved against nothing, while the manifest reads as though a policy applies.", `Use one of ${[...tolerances.keys()].sort().join(", ")}, or contribute the profile.`));
      }
      if (fixture.comparisonProfile.length > 0 && !profiles.has(fixture.comparisonProfile)) {
        breaches.push(breach("testing/fixture", "fixture-comparison-profile-unknown", `${scope}#${fixture.id}`, `Fixture ${fixture.id} names comparison profile ${fixture.comparisonProfile}, which no profile defines`, "The comparison profile decides how this fixture is compared at all; an unresolved one silently falls back to a default nobody chose.", "Contribute the profile, or name one that exists."));
      }
      if (fixture.toleranceOverride !== undefined) {
        const profile = toleranceProfileTable(registry).get(fixture.toleranceProfile ?? "");
        const cap = profile?.maxOverrideFactor ?? 1;
        if (fixture.toleranceOverride.factor > cap) {
          breaches.push(breach("testing/fixture", "tolerance-override-exceeds-cap", `${scope}#${fixture.id}`, `Fixture ${fixture.id} overrides its tolerance by ${fixture.toleranceOverride.factor}×, above the profile cap of ${cap}×`, "An uncapped override turns a failing comparison into a passing one and nothing in the report would say so.", "Lower the override, or change the fixture's tolerance profile deliberately."));
        }
      }
    }
  }
  return breaches;
}

/**
 * 🧫️ Law 2, made mechanical for the v2 manifest. `fixtureProvenanceBreaches` above already audits
 * every REGISTERED fixture's own honesty — present on disk, correct digest, licensed, generated by a
 * qualifying oracle when generated at all — and `mutationVectorRegistryBreaches` already audits the
 * v1 physical `🦠️mutation`/`📸️snapshot`/`🔺️diff`/`🎯️outcome` bundle's own shape. What NEITHER can see
 * is a v2-declared mutation with NO evidence registered against it in EITHER form: the coordinate
 * `buildCoverageMatrix` reports as `status: "missing"` in a RUN's coverage report, but which this
 * static contract phase — computable with no execution at all — never previously failed on.
 * `mutation-kind-uncovered`/`mutation-inverse-uncovered` police the v1 Gherkin catalog's Examples
 * table, not whether either fixture form actually exists; none of the five rules named in this
 * function's own brief reads a v2 `mutationManifests` entry's fixture coverage at all.
 *
 * The minimum honest evidence this accepts is deliberately EITHER of the two forms already live in
 * this repository, not a mandate to adopt one: a v2 `FixtureManifest` (schema `fixture/v2`) whose
 * `target` names this mutation's own artifact/standard/subset and whose `mutation` field names this
 * mutation's own id — exactly what the exemplar (`📷️png@1.2/✳️any`'s `change-background`) carries —
 * OR a v1 physical vector registered in a `mutationCatalogs[].vectors` entry sharing this mutation's
 * OWN `capability` (the same correlation `mutationInventoryBreaches` already uses to match a v1
 * catalog's claimed kinds against a v2 manifest) whose `mutationId` names this mutation's own id, as
 * `🏛️architect/🏛️program` carries for all 266 of its mutations. A survey of the live registry BEFORE
 * settling on this design found 1,650 "violations" against the v2-only form — investigating the
 * largest, `s.architect.program`, showed a real, checked-in, handcrafted before/after JSON bundle for
 * every one of its 266 mutations, registered only in the older v1 form: counting those as untested
 * would have been dishonest. Whichever form supplies the evidence, its OWN existing rule
 * (`fixtureProvenanceBreaches` / `mutationVectorRegistryBreaches`) is what proves the claim is not a
 * lie — this rule only proves a claim of SOME form exists at all. One fixture per mutation id, not one
 * per declared outcome class: an unfixtured OUTCOME of an otherwise-fixtured mutation is a
 * coverage-report gap (`measureCoverage`'s own `status: "missing"` rows), not a law-2 placement gap,
 * and duplicating that measurement here would contradict the run-dependent coverage dimension instead
 * of scoping cleanly around it.
 */
export function mutationFixtureBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const fixturedMutations = new Set<string>();
  for (const contribution of registry.contributions) {
    for (const fixture of contribution.fixtureManifests) {
      if (fixture?.target?.artifact === undefined || fixture.mutation === undefined) continue;
      fixturedMutations.add(`${fixture.target.artifact}@${fixture.target.standard}/${fixture.target.subset}::${fixture.mutation}`);
    }
  }
  const vectoredMutationsByCapability = new Map<string, Set<string>>();
  for (const contribution of registry.contributions) {
    for (const catalog of contribution.mutationCatalogs) {
      if (catalog.capability === "") continue;
      const ids = vectoredMutationsByCapability.get(catalog.capability) ?? new Set<string>();
      for (const vector of catalog.vectors) ids.add(vector.mutationId);
      vectoredMutationsByCapability.set(catalog.capability, ids);
    }
  }
  for (const contribution of registry.contributions) {
    for (const manifest of contribution.mutationManifests) {
      if (mutationManifestProblems(manifest, contribution.owner).length > 0) continue;
      for (const mutation of manifest.mutations) {
        const subset = owningSubsetOf(manifest, mutation);
        if (fixturedMutations.has(`${manifest.artifact}@${manifest.standard}/${subset}::${mutation.id}`)) continue;
        if (vectoredMutationsByCapability.get(mutation.capability)?.has(mutation.id) === true) continue;
        breaches.push(
          breach(
            "testing/fixture",
            "mutation-without-fixture",
            contribution.manifestPath,
            `Mutation ${mutation.id} of ${subsetCoordinate(manifestTarget(manifest, mutation))} is declared with no fixture-backed vector`,
            "A mutation manifest declares what dispatch CLAIMS it can do; a fixture is the one thing that tests the claim against a real-world example, a handcrafted vector, or a qualifying-oracle-generated before/after pair, rather than the implementation grading its own homework.",
            `Add a fixtureManifests entry (schema semio.repository-test.fixture/v2) with target ${JSON.stringify({ artifact: manifest.artifact, standard: manifest.standard, subset })} and mutation ${JSON.stringify(mutation.id)}, or register a physical vector for it in a mutationCatalogs[] entry sharing capability ${JSON.stringify(mutation.capability)} — either way, backed by real-world/handcrafted/oracle-generated evidence.`,
          ),
        );
      }
    }
  }
  return breaches;
}

/** 🔒️ Oracle and probe packages must be unreachable from production, and offline during execution. */
export function isolationBreaches(registry: OracleRegistry): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const oracle of registry.oracles) {
    if (oracle.productionReachable === true && oracle.productionDebt === undefined) {
      breaches.push(breach("testing/dependency", "oracle-production-reachable", oracle.id, `Oracle ${oracle.id} (${oracle.package}) is production-reachable and records no debt`, "A reference the production code can also reach is not an independent reference — the bug would be in both halves.", "Remove the production edge, or record it as shrink-only productionDebt with an owner and a plan."));
    }
    if (oracle.networkDuringExecution === true) {
      breaches.push(breach("testing/dependency", "oracle-needs-network", oracle.id, `Oracle ${oracle.id} requires network access during execution`, "Tests execute offline once dependencies and fixture blobs are provisioned; a network call makes a run non-reproducible and CI-flaky.", "Provision the data ahead of the run and read it from the fixture store."));
    }
  }
  for (const probe of registry.probes) {
    // 🔒️RECORDED DEBT IS EXEMPT, and only recorded debt — the same rule oracles get, and for the same
    // reason: asserting unconditionally meant that honestly recording a package's reachability failed
    // the very gate demanding it be recorded, leaving an owner the choice of hiding it or staying red.
    // An UNRECORDED production-reachable probe is still a hard failure.
    if (probe.productionReachable === true && probe.productionDebt === undefined) {
      breaches.push(breach("testing/dependency", "probe-production-reachable", probe.id, `Probe ${probe.id} (${probe.package}) is production-reachable and records no debt`, "A measurement tool linked into production stops being an independent measurement.", "Make the probe test-only, or record the reachability as shrink-only productionDebt with an owner and a plan."));
    }
    if (probe.productionDebt !== undefined && (probe.productionDebt.reachableFrom.length === 0 || probe.productionDebt.owner.length === 0 || probe.productionDebt.plan.length === 0)) {
      breaches.push(breach("testing/dependency", "probe-debt-incomplete", probe.id, `Probe ${probe.id} records productionDebt without a path, an owner or a retirement plan`, "Debt that names nobody and no plan is not a record, it is an excuse.", "Name the reachable path, the owning module and how the debt shrinks."));
    }
    if (probe.networkDuringExecution === true) {
      breaches.push(breach("testing/dependency", "probe-needs-network", probe.id, `Probe ${probe.id} requires network access during execution`, "A probe that phones home cannot produce a reproducible measurement.", "Vendor or pre-provision whatever it fetches."));
    }
    if (probe.deterministic === false && probe.seedRequired !== true) {
      breaches.push(breach("testing/contract", "probe-nondeterministic-unseeded", probe.id, `Probe ${probe.id} is non-deterministic and declares no seed requirement`, "Approximate measurements — sampled Hausdorff distance above all — depend on sampling. Without a recorded seed the same comparison can pass and fail on identical inputs.", "Declare seedRequired and take the seed from the stage."));
    }
  }
  return breaches;
}

/** 🚫️ A subject that replays committed vectors proves only that the vectors were copied correctly. */
export function vectorReplayBreaches(results: readonly TestResult[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const result of results) {
    if (result.role !== "subject" || result.status === "errored") continue;
    if (result.productionDispatch?.invoked !== true) {
      breaches.push(breach("testing/contract", "subject-without-production-dispatch", result.testId, `Subject result ${result.testId} carries no production-dispatch proof`, "A subject adapter that replays a committed output vector measures the vector, not the code. The dispatch record is the only thing that distinguishes them.", "Invoke the production mutation bridge from the adapter and report the operation and bridge version."));
    }
  }
  return breaches;
}

/** ⚙️ An oracle sharing the subject's engine family is an interoperability check, not an independence one. */
export function engineIndependenceBreaches(registry: OracleRegistry, subjectEngines: ReadonlyMap<string, EngineFamily>): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const manifest of registry.mutationManifests) {
    const subject = subjectEngines.get(manifest.artifact);
    if (subject === undefined) continue;
    for (const mutation of manifest.mutations) {
      for (const requirement of mutation.oracleRequirements) {
        const qualifying = registry.oracles.filter((oracle) => oracle.capabilities.includes(requirement.capability) && isQualifyingOracleKind(oracle.kind));
        const independent = qualifying.filter((oracle) => engineFamilyId(oracle.engine) !== subject.family);
        if (qualifying.length > 0 && independent.length === 0) {
          breaches.push(
            breach(
              "testing/oracle",
              "oracle-shares-subject-engine",
              `${manifest.artifact}@${manifest.standard}/${owningSubsetOf(manifest, mutation)}`,
              `Every qualifying oracle for ${requirement.capability} sits on engine family ${subject.family}, which is also the subject's`,
              "A shared kernel makes the comparison circular: the reference and the code under test would agree on the kernel's own defects.",
              "Qualify a second exact-kernel family, and keep the independent mesh-side probe in the pipeline meanwhile.",
              "medium",
            ),
          );
        }
      }
    }
  }
  return breaches;
}
//#endregion 🧾️ContractV2

//#region 🧹️GarbageCollection
/** 🧹️ One candidate the sweep considered, with everything a reader needs to judge the decision. */
export type GcCandidate = Readonly<{ path: string; bytes: number; files: number; retention: RetentionClass; runId: string | null; agentId: string | null; lastAccessMs: number; eligible: boolean; reason: string; heldBy: readonly string[] }>;

/** 🧹️ The result of one mark-and-sweep pass. `dry` is the default everywhere it is offered. */
export type GcReport = Readonly<{ dry: boolean; candidates: readonly GcCandidate[]; removed: readonly string[]; markedBlobs: number; sweptBlobs: number; reclaimedBytes: number }>;

function directorySize(abs: string): { bytes: number; files: number } {
  let bytes = 0;
  let files = 0;
  const stack = [abs];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: import("node:fs").Dirent[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      const full = join(dir, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) {
        stack.push(full);
        continue;
      }
      try {
        bytes += statSync(full).size;
        files += 1;
      } catch {
        /* 🧭️Raced with another agent's atomic publish; it is not ours to account for. */
      }
    }
  }
  return { bytes, files };
}

/**
 * 🧹️ Marks every blob any committed fixture manifest, retained run manifest or pinned evidence
 * references. Sweeping is the complement, so an unreachable blob is one nothing can reach — never one
 * that merely looks old.
 */
export function markReferencedBlobs(repoRoot: string, registry: OracleRegistry): Set<string> {
  const marked = new Set<string>();
  for (const contribution of registry.contributions) for (const fixture of contribution.fixtureManifests) for (const file of fixture.files) marked.add(file.sha256);
  const manifestRoot = fixtureManifestRoot(repoRoot);
  if (existsSync(manifestRoot)) {
    for (const name of readdirSync(manifestRoot)) {
      try {
        const fixture = JSON.parse(readFileSync(join(manifestRoot, name), "utf8")) as FixtureManifest;
        for (const file of fixture.files ?? []) marked.add(file.sha256);
      } catch {
        /* 🧭️A manifest mid-write by a peer agent is not evidence that its blobs are unreachable. */
      }
    }
  }
  const resultsRoot = testCacheDir(repoRoot, "results");
  if (existsSync(resultsRoot)) {
    const stack = [resultsRoot];
    while (stack.length > 0) {
      const dir = stack.pop()!;
      let entries: import("node:fs").Dirent[];
      try {
        entries = readdirSync(dir, { withFileTypes: true });
      } catch {
        continue;
      }
      for (const entry of entries) {
        const full = join(dir, entry.name);
        if (entry.isSymbolicLink()) continue;
        if (entry.isDirectory()) {
          stack.push(full);
          continue;
        }
        if (entry.name !== "🗝️run.json") continue;
        try {
          const manifest = JSON.parse(readFileSync(full, "utf8")) as RunManifest;
          for (const artifact of manifest.artifacts ?? []) marked.add(artifact.sha256);
        } catch {
          /* 🧭️Same. */
        }
      }
    }
  }
  return marked;
}

/**
 * 🧹️ Mark-and-sweep over the fixture blob store plus a retention-aware scan of run directories.
 * Refuses to act at all unless the cache root looks like this repository's cache, and never follows a
 * symlink out of it — the two failure modes that turn a cleaner into a data-loss incident.
 */
export function collectGarbage(repoRoot: string, registry: OracleRegistry, opts: { dry?: boolean; olderThanMs?: number; overBytes?: number; agent?: string; retention?: readonly RetentionClass[]; nowMs?: number } = {}): GcReport {
  const dry = opts.dry !== false;
  const nowMs = opts.nowMs ?? Date.now();
  const cacheRoot = resolve(testCacheRoot(repoRoot));
  if (!existsSync(cacheRoot)) return { dry, candidates: [], removed: [], markedBlobs: 0, sweptBlobs: 0, reclaimedBytes: 0 };
  // 🚫️Both sides are RESOLVED before they are compared. Comparing a resolved path against an
  // unresolved literal rejected every legitimate cache root on a platform where the repository itself
  // sits behind a symlink — `/var` → `/private/var` on macOS is the ordinary case, not the attack —
  // while still letting a cache root that is symlinked OUT of the repository through unexamined.
  const resolvedCacheRoot = realpathSync(cacheRoot);
  const resolvedMetaRoot = realpathSync(resolve(getRepoMetaDir(repoRoot)));
  if (resolvedCacheRoot !== resolvedMetaRoot && !resolvedCacheRoot.startsWith(resolvedMetaRoot + sep)) {
    throw new Error(`refusing to collect: ${cacheRoot} resolves to ${resolvedCacheRoot}, outside the repository cache at ${resolvedMetaRoot}`);
  }

  const candidates: GcCandidate[] = [];
  const removed: string[] = [];
  let reclaimedBytes = 0;

  for (const child of ["work", "hosts", "oracles", "results", "diffs"]) {
    const root = join(cacheRoot, child);
    if (!existsSync(root)) continue;
    const stack: string[] = [root];
    while (stack.length > 0) {
      const dir = stack.pop()!;
      let entries: import("node:fs").Dirent[];
      try {
        entries = readdirSync(dir, { withFileTypes: true });
      } catch {
        continue;
      }
      for (const entry of entries) {
        if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
        const abs = join(dir, entry.name);
        const lease = readLease(abs);
        if (lease === null) {
          stack.push(abs);
          continue;
        }
        const { bytes, files } = directorySize(abs);
        const held: string[] = [];
        if (lease.state === "active" || lease.state === "creating") held.push(`lease ${lease.state} (agent ${lease.agentId})`);
        if (PROTECTED_RETENTION_CLASSES.includes(lease.retention)) held.push(`retention ${lease.retention}`);
        if (lease.agentId === agentId() && lease.state !== "complete") held.push("owned by this agent");
        const age = nowMs - Date.parse(lease.heartbeatAt);
        const oldEnough = opts.olderThanMs === undefined || (Number.isFinite(age) && age > opts.olderThanMs);
        const bigEnough = opts.overBytes === undefined || bytes > opts.overBytes;
        const agentMatches = opts.agent === undefined || lease.agentId === opts.agent;
        const retentionMatches = opts.retention === undefined || opts.retention.includes(lease.retention);
        const reclaimable = leaseReclaimable(lease, nowMs);
        const eligible = held.length === 0 && oldEnough && bigEnough && agentMatches && retentionMatches && (lease.state === "complete" || reclaimable);
        const reason = held.length > 0 ? `held: ${held.join("; ")}` : !oldEnough ? "newer than --older-than" : !bigEnough ? "smaller than --over-size" : !agentMatches ? "another agent's run" : !retentionMatches ? "retention class not selected" : eligible ? `${lease.state} run, reclaimable` : "lease is not reclaimable";
        candidates.push({ path: relative(repoRoot, abs).split(sep).join("/"), bytes, files, retention: lease.retention, runId: lease.runId, agentId: lease.agentId, lastAccessMs: age, eligible, reason, heldBy: held });
        if (eligible && !dry) {
          rmSync(abs, { recursive: true, force: true });
          removed.push(relative(repoRoot, abs).split(sep).join("/"));
          reclaimedBytes += bytes;
        }
      }
    }
  }

  const marked = markReferencedBlobs(repoRoot, registry);
  let sweptBlobs = 0;
  const blobRoot = fixtureBlobRoot(repoRoot);
  if (existsSync(blobRoot)) {
    for (const shard of readdirSync(blobRoot)) {
      const shardDir = join(blobRoot, shard);
      if (!existsSync(shardDir) || !lstatSync(shardDir).isDirectory()) continue;
      for (const name of readdirSync(shardDir)) {
        if (name.endsWith(".tmp")) continue;
        const sha256 = `sha256:${name}`;
        if (marked.has(sha256)) continue;
        const abs = join(shardDir, name);
        let bytes = 0;
        try {
          bytes = statSync(abs).size;
        } catch {
          continue;
        }
        candidates.push({ path: relative(repoRoot, abs).split(sep).join("/"), bytes, files: 1, retention: "ephemeral-success", runId: null, agentId: null, lastAccessMs: 0, eligible: true, reason: "blob is referenced by no fixture manifest, retained run manifest or pinned evidence", heldBy: [] });
        if (!dry) {
          rmSync(abs, { force: true });
          sweptBlobs += 1;
          reclaimedBytes += bytes;
        }
      }
    }
  }
  return { dry, candidates, removed, markedBlobs: marked.size, sweptBlobs, reclaimedBytes };
}

/** 🧹️ Renders a GC pass so every decision is legible: what, how big, whose, and why it was or was not eligible. */
export function formatGcReport(report: GcReport): string {
  const lines = [`[gc] ${report.dry ? "DRY RUN — nothing removed" : "SWEEPING"} — ${report.candidates.length} candidate(s), ${report.markedBlobs} blob(s) marked reachable`];
  for (const candidate of [...report.candidates].sort((a, b) => b.bytes - a.bytes)) {
    lines.push(`[gc] ${candidate.eligible ? "eligible" : "held    "} ${(candidate.bytes / 1024 / 1024).toFixed(2)} MiB ${candidate.retention.padEnd(19)} ${candidate.agentId ?? "-"} ${candidate.path} — ${candidate.reason}`);
  }
  lines.push(`[gc] ${report.removed.length} run director(ies) removed, ${report.sweptBlobs} blob(s) swept, ${(report.reclaimedBytes / 1024 / 1024).toFixed(2)} MiB reclaimed`);
  return lines.join("\n");
}
//#endregion 🧹️GarbageCollection

//#region 📈️CoverageV2
/** 📈️ Every dimension the release gate reads. A gate that reads nothing else cannot be satisfied by an aggregate. */
export const COVERAGE_DIMENSIONS = [
  "runtimeMutationCoverage",
  "subsetOwnershipCoverage",
  "externalOracleCoverage",
  "oracleEvidenceCoverage",
  "oracleCapabilityCoverage",
  "productionBridgeCoverage",
  "fixtureClassCoverage",
  "fixtureProvenanceCoverage",
  "expectedOutcomeCoverage",
  "inverseCoverage",
  "metamorphicCoverage",
  "comparisonMetricCoverage",
  "determinismCoverage",
  "implementationCoverage",
  "platformCoverage",
  "fixtureReproducibilityCoverage",
  "dependencyIsolationCoverage",
] as const;
export type CoverageDimension = (typeof COVERAGE_DIMENSIONS)[number];

/** 📈️ Dimensions that must be exactly 100% to release. */
export const RELEASE_GATED_DIMENSIONS: readonly CoverageDimension[] = ["runtimeMutationCoverage", "subsetOwnershipCoverage", "externalOracleCoverage", "oracleEvidenceCoverage", "productionBridgeCoverage", "fixtureProvenanceCoverage", "dependencyIsolationCoverage"];

/** 📊️ One report row, keyed by the FULL coordinate. Reporting at artifact level is what this shape prevents. */
export type CoverageRow = Readonly<{
  baselineSha: string;
  artifact: string;
  standard: string;
  subset: string;
  mutation: string;
  outcome: MutationOutcomeClass;
  fixture: string;
  fixtureClass?: FixtureClass;
  oracle: string;
  oracleKind?: OracleKind;
  oracleEngineFamily?: string;
  subjectEngineFamily?: string;
  probes: readonly string[];
  implementation: Implementation | "";
  platform: PlatformId;
  comparisonProfile: string;
  toleranceOverridden?: boolean;
  status: "passed" | "failed" | "errored" | "missing";
  notes?: string;
}>;

/** 📈️ One dimension's measured ratio plus the exact coordinates that are missing. */
export type DimensionMeasurement = Readonly<{ dimension: CoverageDimension; covered: number; total: number; ratio: number; missing: readonly string[] }>;

function measure(dimension: CoverageDimension, covered: number, total: number, missing: readonly string[]): DimensionMeasurement {
  return { dimension, covered, total, ratio: total === 0 ? 1 : covered / total, missing: missing.slice(0, 200) };
}

/**
 * 📈️ Builds the complete v2 coverage matrix from the registry, the runtime inventories and the run's
 * results. The matrix is enumerated from the MANIFESTS, not from the results, so a mutation nobody
 * wrote a test for appears as a `missing` row instead of being absent from the denominator.
 */
export function buildCoverageMatrix(repoRoot: string, registry: OracleRegistry, results: readonly TestResult[], baselineSha: string): CoverageRow[] {
  const rows: CoverageRow[] = [];
  const platform = currentPlatform();
  const oracles = new Map(registry.oracles.map((oracle) => [oracle.id, oracle]));
  const fixturesByTarget = new Map<string, FixtureManifest[]>();
  for (const contribution of registry.contributions) {
    for (const fixture of contribution.fixtureManifests) {
      // 🛡️A malformed fixture must not take the whole matrix down. Three fixtures registered without a
      // `target` threw here, and every caller that swallowed the throw reported coverage computed over a
      // silently smaller set — the provenance dimension read 300/300 against 303 registered fixtures,
      // because the three that FAILED it had been dropped from the denominator by the crash.
      if (fixture?.target?.artifact === undefined) continue;
      const key = `${fixture.target.artifact}@${fixture.target.standard}/${fixture.target.subset}::${fixture.mutation ?? ""}::${fixture.outcome ?? ""}`;
      fixturesByTarget.set(key, [...(fixturesByTarget.get(key) ?? []), fixture]);
    }
  }
  const resultsByMutation = new Map<string, TestResult[]>();
  for (const result of results) {
    if (result.mutation === undefined) continue;
    const key = `${result.mutation}::${result.outcome ?? ""}`;
    resultsByMutation.set(key, [...(resultsByMutation.get(key) ?? []), result]);
  }

  for (const manifest of registry.mutationManifests) {
    for (const mutation of manifest.mutations) {
      const subset = owningSubsetOf(manifest, mutation);
      for (const outcome of mutation.outcomes) {
        const fixtureKey = `${manifest.artifact}@${manifest.standard}/${subset}::${mutation.id}::${outcome}`;
        const fixtures = fixturesByTarget.get(fixtureKey) ?? [];
        const qualifying = registry.oracles.filter((oracle) => isQualifyingOracleKind(oracle.kind) && mutation.oracleRequirements.some((requirement) => oracle.capabilities.includes(requirement.capability)));
        const executions = resultsByMutation.get(`${mutation.id}::${outcome}`) ?? [];
        if (fixtures.length === 0 || executions.length === 0) {
          rows.push({
            baselineSha,
            artifact: manifest.artifact,
            standard: manifest.standard,
            subset,
            mutation: mutation.id,
            outcome,
            fixture: fixtures[0]?.id ?? "",
            fixtureClass: fixtures[0]?.class,
            oracle: qualifying[0]?.id ?? "",
            oracleKind: qualifying[0]?.kind,
            oracleEngineFamily: engineFamilyId(qualifying[0]?.engine),
            probes: [],
            implementation: "",
            platform,
            comparisonProfile: fixtures[0]?.comparisonProfile ?? "",
            toleranceOverridden: fixtures[0]?.toleranceOverride !== undefined,
            status: "missing",
            notes: fixtures.length === 0 ? "no fixture declares this mutation × outcome" : "no execution produced a result for this coordinate",
          });
          continue;
        }
        for (const fixture of fixtures) {
          for (const execution of executions) {
            const oracle = oracles.get(qualifying[0]?.id ?? "");
            rows.push({
              baselineSha,
              artifact: manifest.artifact,
              standard: manifest.standard,
              subset,
              mutation: mutation.id,
              outcome,
              fixture: fixture.id,
              fixtureClass: fixture.class,
              oracle: oracle?.id ?? "",
              oracleKind: oracle?.kind,
              oracleEngineFamily: engineFamilyId(oracle?.engine),
              probes: [],
              implementation: execution.implementation,
              platform: execution.platform ?? platform,
              comparisonProfile: fixture.comparisonProfile,
              toleranceOverridden: fixture.toleranceOverride !== undefined,
              status: execution.status,
            });
          }
        }
      }
    }
  }
  return rows;
}

/** 📈️ Measures all sixteen dimensions from the matrix, the registry and the run's results. */
export function measureCoverage(registry: OracleRegistry, rows: readonly CoverageRow[], results: readonly TestResult[], runtimeInventories: readonly RuntimeMutationInventory[], repoRoot: string = repoRootFromHere()): DimensionMeasurement[] {
  const manifestMutations = registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => ({ manifest, mutation })));
  const owned = new Set(manifestMutations.map(({ manifest, mutation }) => `${manifest.artifact}@${manifest.standard}::${mutation.id}`));
  const runtimeIds = runtimeInventories.flatMap((inventory) => inventory.mutations.map((mutation) => `${inventory.artifact}@${inventory.standard}::${mutation.id}`));
  // 🏭️A SUBSET WITH NO INVENTORY IS UNMEASURED, not covered. Pooling denominators across the registry
  // meant one subset that had run the bridge kept the denominator non-empty, and every subset that had
  // never run it simply contributed nothing — invisible rather than reported. Each such subset is
  // counted as one uncovered coordinate so it appears in the gate's `missing` list by name.
  const inventoried = new Set(runtimeInventories.map((inventory) => subsetCoordinate(inventory)));
  const uninventoried = registry.mutationManifests.map((manifest) => subsetCoordinate(manifest)).filter((coordinate) => !inventoried.has(coordinate)).map((coordinate) => `${coordinate} (no runtime inventory)`);

  const runtimeMissing = [...runtimeIds.filter((id) => !owned.has(id)), ...uninventoried];
  const wildcards = manifestMutations.filter(({ manifest, mutation }) => isWildcardSubsetFor(repoRoot, manifest.artifact, manifest.standard, owningSubsetOf(manifest, mutation))).map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const withoutOracle = manifestMutations
    .filter(({ mutation }) => !mutation.oracleRequirements.every((requirement) => registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(requirement.capability) && (requirement.oracle === undefined || oracle.id === requirement.oracle))))
    .map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const fixtureSubsets = new Set(registry.contributions.flatMap((contribution) => contribution.fixtureManifests).filter((fixture) => fixture?.target?.artifact !== undefined).map((fixture) => `${fixture.target.artifact}@${fixture.target.standard}/${fixture.target.subset}`));
  // 🧪️EVIDENCE IS THE CONJUNCTION, not the fixture alone. Counting only "a fixture targets this subset"
  // let a mutation with NO discharged oracle — an `-uncarried` kind in a subset that happens to have
  // fixtures — count as measured, and evidence then exceeded registration: 213 against 210, which is
  // impossible by construction. A mutation is evidenced only when a qualifying oracle discharges it AND
  // something exists to run that oracle against.
  const oracled = new Set(withoutOracle);
  const withoutEvidence = manifestMutations
    .filter(({ manifest, mutation }) => oracled.has(`${manifest.artifact}::${mutation.id}`) || !fixtureSubsets.has(`${manifest.artifact}@${manifest.standard}/${manifest.subset}`))
    .map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const requiredCapabilities = [...new Set(manifestMutations.flatMap(({ mutation }) => mutation.oracleRequirements.map((requirement) => requirement.capability)))];
  const unsupportedCapabilities = requiredCapabilities.filter((capability) => !registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(capability)));

  const subjectResults = results.filter((result) => result.role === "subject");
  const replaying = subjectResults.filter((result) => result.productionDispatch?.invoked !== true).map((result) => result.testId);

  const fixtures = registry.contributions.flatMap((contribution) => contribution.fixtureManifests);
  const withoutProvenance = fixtures.filter((fixture) => fixtureManifestProblems(fixture, repoRoot).length > 0).map((fixture) => fixture.id);
  const notReproducible = fixtures.filter((fixture) => fixture.class === "third-party-generated" && !fixture.reproducible).map((fixture) => fixture.id);
  const outcomeCoordinates = manifestMutations.flatMap(({ manifest, mutation }) => mutation.outcomes.map((outcome) => `${manifest.artifact}::${mutation.id}::${outcome}`));
  const coveredOutcomes = new Set(rows.filter((row) => row.status === "passed").map((row) => `${row.artifact}::${row.mutation}::${row.outcome}`));

  const inverseIds = new Set(results.filter((result) => result.scenario.startsWith("inverse-")).map((result) => result.scenario.slice("inverse-".length)));
  const metamorphicIds = new Set(results.filter((result) => result.scenario.startsWith("metamorphic-")).map((result) => result.scenario.slice("metamorphic-".length)));
  const allIds = manifestMutations.map(({ mutation }) => mutation.id);

  const determinismChecked = new Set(results.filter((result) => result.scenario.startsWith("determinism-")).map((result) => result.scenario.slice("determinism-".length)));
  const implementations = new Set(results.map((result) => result.implementation));
  const platforms = new Set(rows.map((row) => row.platform));
  const metricRows = rows.filter((row) => row.status !== "missing");
  const withMetric = metricRows.filter((row) => row.comparisonProfile.length > 0);
  const leakyOracles = registry.oracles.filter((oracle) => oracle.productionReachable === true && oracle.productionDebt === undefined).map((oracle) => oracle.id);
  const leakyProbes = registry.probes.filter((probe) => probe.productionReachable === true).map((probe) => probe.id);

  return [
    measure("runtimeMutationCoverage", runtimeIds.length - runtimeIds.filter((id) => !owned.has(id)).length, runtimeIds.length + uninventoried.length, runtimeMissing),
    measure("subsetOwnershipCoverage", manifestMutations.length - wildcards.length, manifestMutations.length, wildcards),
    measure("externalOracleCoverage", manifestMutations.length - withoutOracle.length, manifestMutations.length, withoutOracle),
    // 🧪️AN ORACLE THAT HAS NEVER BEEN RUN AGAINST ANYTHING PROVES NOTHING. `externalOracleCoverage`
    // asks only whether a qualifying oracle is REGISTERED for a mutation; it says nothing about whether
    // any artifact exists to run it on. 271 of 369 manifested mutations had zero fixtures targeting their
    // subset while counting as covered — the empty-denominator failure this protocol already forbids at
    // the dimension level, reappearing one level down at the mutation level.
    measure("oracleEvidenceCoverage", manifestMutations.length - withoutEvidence.length, manifestMutations.length, withoutEvidence),
    measure("oracleCapabilityCoverage", requiredCapabilities.length - unsupportedCapabilities.length, requiredCapabilities.length, unsupportedCapabilities),
    measure("productionBridgeCoverage", subjectResults.length - replaying.length, subjectResults.length, replaying),
    measure("fixtureClassCoverage", FIXTURE_CLASSES.filter((klass) => fixtures.some((fixture) => fixture.class === klass)).length, FIXTURE_CLASSES.length, FIXTURE_CLASSES.filter((klass) => !fixtures.some((fixture) => fixture.class === klass))),
    measure("fixtureProvenanceCoverage", fixtures.length - withoutProvenance.length, fixtures.length, withoutProvenance),
    measure("expectedOutcomeCoverage", outcomeCoordinates.filter((coordinate) => coveredOutcomes.has(coordinate)).length, outcomeCoordinates.length, outcomeCoordinates.filter((coordinate) => !coveredOutcomes.has(coordinate))),
    measure("inverseCoverage", allIds.filter((id) => inverseIds.has(id)).length, allIds.length, allIds.filter((id) => !inverseIds.has(id))),
    measure("metamorphicCoverage", allIds.filter((id) => metamorphicIds.has(id)).length, allIds.length, allIds.filter((id) => !metamorphicIds.has(id))),
    measure("comparisonMetricCoverage", withMetric.length, metricRows.length, metricRows.filter((row) => row.comparisonProfile.length === 0).map((row) => `${row.artifact}::${row.mutation}`)),
    measure("determinismCoverage", allIds.filter((id) => determinismChecked.has(id)).length, allIds.length, allIds.filter((id) => !determinismChecked.has(id))),
    measure("implementationCoverage", implementations.size, Math.max(implementations.size, 1), []),
    measure("platformCoverage", platforms.size, Math.max(platforms.size, 1), []),
    measure("fixtureReproducibilityCoverage", fixtures.length - notReproducible.length, fixtures.length, notReproducible),
    measure("dependencyIsolationCoverage", registry.oracles.length + registry.probes.length - leakyOracles.length - leakyProbes.length, registry.oracles.length + registry.probes.length, [...leakyOracles, ...leakyProbes]),
  ];
}

/** 🚦️ The release gates. Every one is exact: `= 100%`, `= 0`, never "at least". */
export function enforceReleaseGates(measurements: readonly DimensionMeasurement[], counts: { deferredMutations: number; skipped: number; wildcardOwners: number; unregisteredRuntimeMutations: number }): string[] {
  const failures: string[] = [];
  const byDimension = new Map(measurements.map((measurement) => [measurement.dimension, measurement]));
  for (const dimension of RELEASE_GATED_DIMENSIONS) {
    const measurement = byDimension.get(dimension);
    if (measurement === undefined) {
      failures.push(`${dimension} was never measured — a gate that reads nothing cannot be satisfied`);
      continue;
    }
    // 🚦️An EMPTY denominator is not 100%. `measure` reports ratio 1 for an empty set so the display
    // reads "n/a" rather than a false 0%, but a release gate that accepted it would be satisfied by a
    // run in which nothing was measured at all — which is precisely the failure the gate exists for.
    if (measurement.total === 0) {
      failures.push(`${dimension} has an EMPTY denominator — nothing was measured, and an unmeasured dimension cannot be 100%`);
      continue;
    }
    if (measurement.ratio < 1) failures.push(`${dimension} is ${(measurement.ratio * 100).toFixed(2)}% (${measurement.covered}/${measurement.total}); release requires 100% — missing: ${measurement.missing.slice(0, 10).join(", ")}${measurement.missing.length > 10 ? ` (+${measurement.missing.length - 10} more)` : ""}`);
  }
  if (counts.deferredMutations !== 0) failures.push(`${counts.deferredMutations} deferred mutation(s); release requires 0`);
  if (counts.skipped !== 0) failures.push(`${counts.skipped} skip/quarantine(s); release requires 0`);
  if (counts.wildcardOwners !== 0) failures.push(`${counts.wildcardOwners} wildcard subset owner(s); release requires 0`);
  if (counts.unregisteredRuntimeMutations !== 0) failures.push(`${counts.unregisteredRuntimeMutations} unregistered runtime mutation(s); release requires 0`);
  return failures;
}

/** 📊️ Renders the matrix as the report's own questions, each answered by an explicit list. */
export function formatCoverageQuestions(registry: OracleRegistry, rows: readonly CoverageRow[], measurements: readonly DimensionMeasurement[]): string {
  const fixtures = registry.contributions.flatMap((contribution) => contribution.fixtureManifests);
  const byDimension = new Map(measurements.map((measurement) => [measurement.dimension, measurement]));
  const untested = (byDimension.get("runtimeMutationCoverage")?.missing ?? []).join(", ") || "none";
  const noOracle = (byDimension.get("externalOracleCoverage")?.missing ?? []).join(", ") || "none";
  const semioDerived = registry.oracles.filter((oracle) => oracle.kind === "cross-semio-implementation").map((oracle) => oracle.id).join(", ") || "none";
  const wildcard = (byDimension.get("subsetOwnershipCoverage")?.missing ?? []).join(", ") || "none";
  const provenance = (byDimension.get("fixtureProvenanceCoverage")?.missing ?? []).join(", ") || "none";
  const reproducibility = (byDimension.get("fixtureReproducibilityCoverage")?.missing ?? []).join(", ") || "none";
  const subsets = [...new Set(fixtures.map((fixture) => subsetCoordinate(fixture.target)))];
  const withoutRealWorld = subsets.filter((coordinate) => !fixtures.some((fixture) => subsetCoordinate(fixture.target) === coordinate && fixture.class === "real-world")).join(", ") || "none";
  const overridden = rows.filter((row) => row.toleranceOverridden === true).map((row) => `${row.mutation}/${row.fixture}`).join(", ") || "none";
  const sharedEngine = rows.filter((row) => row.oracleEngineFamily !== undefined && row.subjectEngineFamily !== undefined && row.oracleEngineFamily === row.subjectEngineFamily).map((row) => `${row.mutation} (${row.oracleEngineFamily})`).join(", ") || "none";
  const platformSpecific = (() => {
    const byCoordinate = new Map<string, Set<string>>();
    for (const row of rows) {
      const key = `${row.mutation}::${row.fixture}`;
      if (row.status === "failed") byCoordinate.set(key, (byCoordinate.get(key) ?? new Set()).add(row.platform));
    }
    const all = new Set(rows.map((row) => row.platform));
    return [...byCoordinate].filter(([, failing]) => failing.size < all.size).map(([key]) => key).join(", ") || "none";
  })();
  return [
    "[report] Which runtime mutations are untested?                       " + untested,
    "[report] Which mutations have no external oracle?                    " + noOracle,
    "[report] Which tests still use a Semio-derived oracle?               " + semioDerived,
    "[report] Which mutations remain under wildcard ownership?            " + wildcard,
    "[report] Which fixtures lack provenance?                             " + provenance,
    "[report] Which fixtures are not reproducible?                        " + reproducibility,
    "[report] Which subsets have no real-world fixture?                   " + withoutRealWorld,
    "[report] Which results are tolerance-sensitive (overridden)?         " + overridden,
    "[report] Which failures occur on only one platform?                  " + platformSpecific,
    "[report] Which oracle and subject share an underlying engine family? " + sharedEngine,
  ].join("\n");
}
//#endregion 📈️CoverageV2

//#region 🧭️Root
/** 📁️ Repository root, resolved from this package's own location. */
export function repoRootFromHere(): string {
  return findRepoRoot(dirname(decodeURIComponent(new URL(import.meta.url).pathname)));
}
//#endregion 🧭️Root
