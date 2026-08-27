//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { constants, cpSync, existsSync, linkSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { type BreachRecord, TEST_LEVELS, type TestLevel, findRepoRoot, getRepoMetaDir, runProbe, testLevelBudgetMs } from "../../../📚️library/📦️packages/🟦️typescript/📦️index.ts";
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
  const required = ["fileKinds", "pathExclusions", "testsDirName", "testFixturesDirName", "testFeatureFileKindId", "testCaseSlugPattern", "testAdapterFileKinds", "testImplementationIds", "testOutputCacheDirName", "testOutputMarkerFileKindId", "testOutputMarkerKind", "testOutputChildDirs", "testOracleRegistryLocation", "testSchemaLocation", "testContributionDirName", "testContributionFileKindId", "testDomainPath", "testPhases", "testLevellessPhases", "testMutationVocabularyDirName"];
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

  return { name: featureName, description: descriptionLines.join("\n").trim(), tags: featureTags, capability, oracle, noOracleDecision, comparison, mutationCatalog, background, scenarios, errors };
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
  // 🪆️A catalog's profile coordinates ARE its owner's: an artifact subset carries
  // `🏅️standards/🔖️<v>/🪆️subsets/✳️<s>` in its path and must restate it here, while an owner with no
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
    if (owner !== undefined && !owner.endsWith(`/🏅️standards/${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}`)) problems.push("catalog profile does not match its contribution owner");
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
 * 🧩️ Walks every non-excluded path for `<owner>/🧪️oracle/🔣️.json`. Discovery is by
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
const SOURCE_VECTOR_FILES = ["🦀️component.rs", "🦠️mutation/🔣️component.json", "📸️snapshot/⬅️before/🔣️component.json", "📸️snapshot/➡️after/🔣️component.json", "🎯️outcome/🔣️component.json"] as const;
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
  const jsonDiff = state === "source" ? "🔺️diff/🔣️component.json" : "🔺️diff/🔣️.json";
  const absentDiff = state === "source" ? "🔺️diff/🚫️component.absent" : "🔺️diff/🚫️.absent";
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
      const representedSource = new Set<string>();
      const representedProjected = new Set<string>();

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

      for (const mutationDirectoryName of childDirectories(sourceMutationRoot)) {
        const tests = join(sourceMutationRoot, mutationDirectoryName, "🧪️tests");
        for (const scenario of childDirectories(tests)) {
          const key = `${mutationDirectoryName}/${scenario}`;
          if (!representedSource.has(key)) breaches.push(breach("testing/contract", "mutation-vector-unregistered", relative(repoRoot, join(tests, scenario)).split(sep).join("/"), `Physical source vector ${key} is not registered`, "Unregistered physical evidence cannot be projected or verified deterministically.", "Add its exact mutation and canonical scenario identity to vectors."));
        }
      }
      for (const mutationDirectoryName of childDirectories(projectedProfileRoot)) {
        for (const scenario of childDirectories(join(projectedProfileRoot, mutationDirectoryName))) {
          const key = `${mutationDirectoryName}/${scenario}`;
          if (!representedProjected.has(key)) breaches.push(breach("testing/contract", "mutation-vector-unregistered", relative(repoRoot, join(projectedProfileRoot, key)).split(sep).join("/"), `Physical projected vector ${key} is not registered`, "Unregistered physical evidence cannot be verified or rolled back deterministically.", "Add its exact mutation and canonical scenario identity to vectors."));
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
  else if (!knownProfiles.has(feature.comparison)) breaches.push(breach("testing/contract", "unknown-comparison", discovered.featurePath, `Unknown comparison profile @comparison-${feature.comparison}`, "A profile is either one of the framework's domain-neutral profiles or one an owner contributes through its 🧪️oracle manifest.", `Add it to this owner's 🧪️oracle/🔣️.json, or use one of ${[...knownProfiles.keys()].sort().join(", ")}.`));
  if (feature.oracle === null && feature.noOracleDecision === null) {
    breaches.push(breach("testing/oracle", "missing-oracle", discovered.featurePath, "Feature declares neither @oracle-<id> nor @no-oracle-<decision-id>", "A test without a reference implementation or an explicitly recorded no-oracle decision proves only that the code agrees with itself.", "Register an oracle in the oracle registry, or record an approved no-oracle decision."));
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
  const contributionDir = testTaxonomy(repoRoot).testContributionDirName;
  const inContributionDir = (rel: string): boolean => rel.split("/").includes(contributionDir);
  const isTestOwned = (rel: string): boolean => caseDirs.has(rel) || inContributionDir(rel) || hostRoots.some((root) => rel === root || rel.startsWith(`${root}/`)) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`);
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

/** 🧾️ Repository-wide contract sweep across every discovered case. */
export function validateAllContracts(repoRoot: string, cases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): BreachRecord[] {
  const registry = loadOracleRegistry(repoRoot);
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
  // 🦠️And a vocabulary declared in the TREE but in no manifest is more invisible still: the catalog
  // check above can only see what a manifest declares, so an owner that handcrafts a mutation
  // vocabulary and never registers a catalog is exempt from the completeness gate entirely. Two
  // subsets shipped real, handcrafted vocabularies with no test at all through exactly this gap.
  // The directory name is taxonomy vocabulary, so this rule names no format, language or plugin.
  const vocabularyDir = testTaxonomy(repoRoot).testMutationVocabularyDirName;
  {
    walkDirectories(repoRoot, (abs, rel) => {
      if (isExcludedTestPath(repoRoot, rel)) return "skip";
      if (basename(abs) !== vocabularyDir) return "enter";
      const owner = dirname(dirname(rel));
      const claimed = registry.contributions.some((entry) => entry.owner === owner && entry.mutationCatalogs.length > 0);
      if (!claimed) {
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
  const claimed = new Set(discoverTestCases(repoRoot).map((discovered) => parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8")).mutationCatalog).filter((id): id is string => id !== null));
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
  breaches.push(...noOracleMisuseBreaches(registry));
  breaches.push(...fixtureProvenanceBreaches(repoRoot, registry));
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

/** 🚫️ Subset ids that name "everything" rather than a semantic scope. v2 refuses all of them. */
export const WILDCARD_SUBSET_IDS: readonly string[] = ["*", "any", "all", "unconstrained", ""];

/** 🚫️ Whether a subset id is a wildcard rather than a smallest semantic scope. */
export function isWildcardSubset(subset: string): boolean {
  return WILDCARD_SUBSET_IDS.includes(subset.trim().toLowerCase());
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

/** ✅️ The only oracle kinds that can DISCHARGE a mutation's external-oracle requirement. */
export const QUALIFYING_ORACLE_KINDS = ["third-party-library", "third-party-cli", "standards-reference-tool"] as const;
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
  if (isWildcardSubset(subset)) problems.push(`subset ${JSON.stringify(subset)} is a wildcard — a mutation must be owned by the smallest semantic subset`);
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
    if (typeof raw.subset === "string" && isWildcardSubset(raw.subset)) problems.push(`mutations[${index}].subset ${JSON.stringify(raw.subset)} is a wildcard`);
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
export function fixtureManifestProblems(value: unknown): string[] {
  if (!isPlainObject(value)) return ["fixture manifest is not an object"];
  const problems: string[] = [];
  if (value.schema !== "semio.repository-test.fixture/v2") problems.push('schema must be "semio.repository-test.fixture/v2"');
  if (typeof value.id !== "string" || !MANIFEST_MUTATION_ID_RE.test(value.id)) problems.push("id must be kebab-case");
  if (!(FIXTURE_CLASSES as readonly string[]).includes(String(value.class))) problems.push(`class must be one of ${FIXTURE_CLASSES.join("|")}`);
  if (!isPlainObject(value.target)) problems.push("target must name artifact, standard and subset");
  else {
    for (const key of ["artifact", "standard", "subset"] as const) if (typeof (value.target as Record<string, unknown>)[key] !== "string") problems.push(`target.${key} must be a string`);
    const subset = String((value.target as Record<string, unknown>).subset ?? "");
    if (isWildcardSubset(subset)) problems.push(`target.subset ${JSON.stringify(subset)} is a wildcard`);
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
export type PipelineVerdict = Readonly<{ pipeline: string; equal: boolean; verdicts: readonly AssertionVerdict[]; missingProbes: readonly string[]; unqualifiedStages: readonly string[] }>;

/**
 * ⚖️ Evaluates one stage's declarative assertions against one probe report. The vocabulary is small
 * and deliberately arithmetic-free beyond comparison: `<key>Max` bounds a measured number from above,
 * `<key>Min` from below, `<key>Equal` compares structurally, and a bare boolean/number/string
 * compares for equality. Anything a probe cannot measure it must report as `unsupported` rather than
 * letting the orchestrator compute it.
 */
export function evaluateStageAssertions(stageIndex: number, stage: ComparisonStage, report: ProbeReport): AssertionVerdict[] {
  const verdicts: AssertionVerdict[] = [];
  const optional = stage.optional === true;
  if (report.status !== "ok") {
    verdicts.push({ stage: stageIndex, probe: stage.probe, key: "status", expected: "ok", actual: report.status, ok: false, optional, reason: `probe reported ${report.status}` });
    return verdicts;
  }
  for (const [key, expected] of Object.entries(stage.assertions ?? {})) {
    const bound = key.endsWith("Max") ? "max" : key.endsWith("Min") ? "min" : key.endsWith("Equal") ? "equal" : "value";
    const measurementKey = bound === "value" ? key : key.slice(0, -{ max: 3, min: 3, equal: 5 }[bound]!);
    const actual = report.measurements[bound === "value" ? key : measurementKey];
    if (actual === undefined) {
      verdicts.push({ stage: stageIndex, probe: stage.probe, key, expected, actual: undefined, ok: false, optional, reason: `probe ${stage.probe} reported no measurement ${JSON.stringify(bound === "value" ? key : measurementKey)}` });
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
  for (const [index, stage] of pipeline.stages.entries()) {
    const probe = probes.get(stage.probe);
    if (probe === undefined) missingProbes.push(stage.probe);
    else if (!isQualifiedProbe(probe)) unqualifiedStages.push(stage.probe);
    const report = reports.get(index);
    if (report === undefined) {
      verdicts.push({ stage: index, probe: stage.probe, key: "report", expected: "a probe report", actual: undefined, ok: false, optional: stage.optional === true, reason: `stage ${index} (${stage.probe}) produced no report` });
      continue;
    }
    verdicts.push(...evaluateStageAssertions(index, stage, report));
  }
  return { pipeline: pipeline.id, equal: verdicts.every((verdict) => verdict.ok || verdict.optional), verdicts, missingProbes, unqualifiedStages };
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
    oracleEngineDigest: opts.oracle === undefined ? "" : `${engineFamilyId(opts.oracle.engine)}@${opts.oracle.engine?.version ?? "*"}`,
    probeDigest: setDigest(opts.probes.map((probe) => [probe.id, probe.lockDigest ?? `${probe.package}@${probe.version ?? "*"}`] as const)),
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
        const key = `${manifest.artifact}@${manifest.standard}::${mutation.id}`;
        ownerOf.set(key, [...(ownerOf.get(key) ?? []), `${contribution.manifestPath}#${coordinate}`]);
        if (isWildcardSubset(subset)) {
          breaches.push(breach("testing/contract", "wildcard-subset-owner", contribution.manifestPath, `Mutation ${mutation.id} is owned by wildcard subset ${JSON.stringify(subset)}`, "Testing at artifact level hides which part of the artifact a mutation actually changed, and lets one broad case stand in for every narrow one.", "Split the wildcard into real semantic subsets and give the mutation its smallest owner, or declare an explicit typed compound."));
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
  for (const contribution of registry.contributions) {
    for (const [index, fixture] of contribution.fixtureManifests.entries()) {
      const scope = contribution.manifestPath;
      for (const problem of fixtureManifestProblems(fixture)) {
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
    if (probe.productionReachable === true) {
      breaches.push(breach("testing/dependency", "probe-production-reachable", probe.id, `Probe ${probe.id} (${probe.package}) is production-reachable`, "A measurement tool linked into production stops being an independent measurement.", "Make the probe test-only."));
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
  if (realpathSync(cacheRoot) !== cacheRoot && !realpathSync(cacheRoot).startsWith(resolve(getRepoMetaDir(repoRoot)))) {
    throw new Error(`refusing to collect: ${cacheRoot} resolves outside the repository cache`);
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
export const RELEASE_GATED_DIMENSIONS: readonly CoverageDimension[] = ["runtimeMutationCoverage", "subsetOwnershipCoverage", "externalOracleCoverage", "productionBridgeCoverage", "fixtureProvenanceCoverage", "dependencyIsolationCoverage"];

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
export function measureCoverage(registry: OracleRegistry, rows: readonly CoverageRow[], results: readonly TestResult[], runtimeInventories: readonly RuntimeMutationInventory[]): DimensionMeasurement[] {
  const manifestMutations = registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => ({ manifest, mutation })));
  const owned = new Set(manifestMutations.map(({ manifest, mutation }) => `${manifest.artifact}@${manifest.standard}::${mutation.id}`));
  const runtimeIds = runtimeInventories.flatMap((inventory) => inventory.mutations.map((mutation) => `${inventory.artifact}@${inventory.standard}::${mutation.id}`));

  const runtimeMissing = runtimeIds.filter((id) => !owned.has(id));
  const wildcards = manifestMutations.filter(({ manifest, mutation }) => isWildcardSubset(owningSubsetOf(manifest, mutation))).map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const withoutOracle = manifestMutations
    .filter(({ mutation }) => !mutation.oracleRequirements.every((requirement) => registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(requirement.capability))))
    .map(({ manifest, mutation }) => `${manifest.artifact}::${mutation.id}`);
  const requiredCapabilities = [...new Set(manifestMutations.flatMap(({ mutation }) => mutation.oracleRequirements.map((requirement) => requirement.capability)))];
  const unsupportedCapabilities = requiredCapabilities.filter((capability) => !registry.oracles.some((oracle) => isQualifyingOracleKind(oracle.kind) && oracle.capabilities.includes(capability)));

  const subjectResults = results.filter((result) => result.role === "subject");
  const replaying = subjectResults.filter((result) => result.productionDispatch?.invoked !== true).map((result) => result.testId);

  const fixtures = registry.contributions.flatMap((contribution) => contribution.fixtureManifests);
  const withoutProvenance = fixtures.filter((fixture) => fixtureManifestProblems(fixture).length > 0).map((fixture) => fixture.id);
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
    measure("runtimeMutationCoverage", runtimeIds.length - runtimeMissing.length, runtimeIds.length, runtimeMissing),
    measure("subsetOwnershipCoverage", manifestMutations.length - wildcards.length, manifestMutations.length, wildcards),
    measure("externalOracleCoverage", manifestMutations.length - withoutOracle.length, manifestMutations.length, withoutOracle),
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
