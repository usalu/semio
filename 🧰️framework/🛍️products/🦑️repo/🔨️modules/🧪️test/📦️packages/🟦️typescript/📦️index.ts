//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { cpSync, existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
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

/** ⚖️ Owned, versioned semantic comparison profiles — adapters never compare, profiles do. */
export const COMPARISON_PROFILES = ["exact-bytes-v1", "utf8-text-v1", "ordered-json-v1", "unordered-json-v1", "floating-point-v1", "semantic-pdf-v1", "filesystem-tree-v1", "diagnostic-v1", "event-stream-v1"] as const;
export type ComparisonProfile = (typeof COMPARISON_PROFILES)[number];

/** 🗂️ Canonical taxonomy names, mirrored from `🔣️taxonomy.json` so a drift is a hard failure rather than a silent divergence. */
export type TestTaxonomy = Readonly<{
  testsDirName: string;
  testFixturesDirName: string;
  testFeatureFilename: string;
  testCaseSlugPattern: string;
  testAdapterFilenames: Readonly<Record<string, string>>;
  testImplementationIds: Readonly<Record<string, string>>;
  testExcludedPathPrefixes: readonly string[];
  testOutputCacheDirName: string;
  testOutputMarkerFilename: string;
  testOutputMarkerKind: string;
  testOutputChildDirs: readonly string[];
  testOracleRegistryPath: string;
  testSchemaPath: string;
}>;

let taxonomyCache: { root: string; value: TestTaxonomy } | null = null;

/** 🔣️ Loads the frozen test vocabulary out of the SSOT taxonomy. Never re-declare these strings locally. */
export function testTaxonomy(repoRoot: string): TestTaxonomy {
  if (taxonomyCache && taxonomyCache.root === repoRoot) return taxonomyCache.value;
  const parsed = JSON.parse(readFileSync(join(repoRoot, TAXONOMY_REL_PATH), "utf8")) as Record<string, unknown>;
  const required = ["testsDirName", "testFixturesDirName", "testFeatureFilename", "testCaseSlugPattern", "testAdapterFilenames", "testImplementationIds", "testExcludedPathPrefixes", "testOutputCacheDirName", "testOutputMarkerFilename", "testOutputMarkerKind", "testOutputChildDirs", "testOracleRegistryPath", "testSchemaPath"];
  const missing = required.filter((key) => parsed[key] === undefined);
  if (missing.length > 0) throw new Error(`🔣️taxonomy.json is missing the test contract keys: ${missing.join(", ")}`);
  const value = Object.fromEntries(required.map((key) => [key, parsed[key]])) as unknown as TestTaxonomy;
  taxonomyCache = { root: repoRoot, value };
  return value;
}

/** 🚫️ `compose/**` is excluded here, in the discovery library — never only by a workflow path filter. */
export function isExcludedTestPath(repoRoot: string, relPath: string): boolean {
  const normalized = relPath.split(sep).join("/");
  return testTaxonomy(repoRoot).testExcludedPathPrefixes.some((prefix) => normalized === prefix.replace(/\/$/, "") || normalized.startsWith(prefix) || normalized.includes(`/${prefix}`));
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
  const comparison = (COMPARISON_PROFILES as readonly string[]).includes(comparisonRaw ?? "") ? (comparisonRaw as ComparisonProfile) : null;
  if (comparisonRaw !== null && comparison === null) errors.push(`Unknown comparison profile @comparison-${comparisonRaw}`);

  const seen = new Set<string>();
  for (const scenario of scenarios) {
    if (seen.has(scenario.id)) errors.push(`Duplicate scenario id @id-${scenario.id}`);
    seen.add(scenario.id);
  }

  return { name: featureName, description: descriptionLines.join("\n").trim(), tags: featureTags, capability, oracle, noOracleDecision, comparison, background, scenarios, errors };
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
      const featurePath = join(caseDir, taxonomy.testFeatureFilename);
      if (!existsSync(featurePath)) continue;
      const adapters: Partial<Record<Implementation, string>> = {};
      for (const [key, filename] of Object.entries(taxonomy.testAdapterFilenames)) {
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
export type OracleEntry = Readonly<{ id: string; ecosystem: string; package: string; version?: string; capabilities: readonly string[]; comparisonProfiles: readonly ComparisonProfile[]; license: string; testOnly: true; homepage?: string; rationale?: string; hostPath?: string }>;

/** 📇️ A recorded decision that a capability legitimately has no credible reference implementation. */
export type NoOracleDecision = Readonly<{ id: string; capabilities: readonly string[]; rationale: string; substitutes: readonly string[] }>;

export type OracleRegistry = Readonly<{ schemaVersion: number; oracles: readonly OracleEntry[]; noOracleDecisions: readonly NoOracleDecision[] }>;

/** 📇️ Loads the central oracle registry. Approved libraries live here, never in a production manifest. */
export function loadOracleRegistry(repoRoot: string): OracleRegistry {
  const parsed = JSON.parse(readFileSync(join(repoRoot, testTaxonomy(repoRoot).testOracleRegistryPath), "utf8")) as OracleRegistry;
  return { schemaVersion: parsed.schemaVersion, oracles: parsed.oracles ?? [], noOracleDecisions: parsed.noOracleDecisions ?? [] };
}
//#endregion 📇️Registry

//#region 🧫️Fixtures
/** 🧫️ One resolved fixture — explicit scheme, never shadow-based, digest pinned at plan time. */
export type ResolvedFixture = Readonly<{ uri: string; scope: "shared" | "local"; name: string; path: string; digest: string }>;

const FIXTURE_URI_RE = /\b(shared|local):\/\/([^\s"'`,;)\]]+)/g;

/** 🧫️ Extracts every `shared://` / `local://` reference appearing anywhere in a feature's text. */
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
 */
export function resolveFixtures(repoRoot: string, discovered: DiscoveredCase, uris: readonly string[]): { fixtures: ResolvedFixture[]; missing: string[] } {
  const fixtures: ResolvedFixture[] = [];
  const missing: string[] = [];
  for (const uri of uris) {
    const [scheme, name] = uri.split("://") as ["shared" | "local", string];
    const baseRel = scheme === "shared" ? discovered.sharedFixtureDir : discovered.localFixtureDir;
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
  schemaVersion: 1;
  owner: string;
  case: string;
  featurePath: string;
  featureHash: string;
  featureName: string;
  description: string;
  capability: string;
  oracle: string | null;
  noOracleDecision: string | null;
  comparison: ComparisonProfile;
  background: readonly FeatureStep[];
  scenarios: readonly FeatureScenario[];
  adapters: Readonly<Partial<Record<Implementation, string>>>;
  fixtures: readonly ResolvedFixture[];
  workDir: string;
  resultsPath: string;
  outputDir: string;
  level: TestLevel;
  role: TestRole;
  implementation: Implementation;
}>;

/** 🎚️ Levels at or below `level` — running a level runs every lower level, cumulatively. */
export function levelsUpTo(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(0, TEST_LEVELS.indexOf(level) + 1);
}

/** 📋️ Builds the plan skeleton shared by every role/implementation execution of one case. */
export function buildCasePlan(repoRoot: string, discovered: DiscoveredCase, level: TestLevel): { plan: Omit<TestCasePlan, "role" | "implementation" | "workDir" | "resultsPath" | "outputDir">; feature: ParsedFeature; missingFixtures: string[] } {
  const source = readFileSync(join(repoRoot, discovered.featurePath), "utf8");
  const feature = parseFeature(source);
  const { fixtures, missing } = resolveFixtures(repoRoot, discovered, fixtureUrisIn(feature));
  const selectable = new Set(levelsUpTo(level));
  return {
    feature,
    missingFixtures: missing,
    plan: {
      schemaVersion: 1,
      owner: discovered.owner,
      case: discovered.case,
      featurePath: discovered.featurePath,
      featureHash: digest(source),
      featureName: feature.name,
      description: feature.description,
      capability: feature.capability ?? "",
      oracle: feature.oracle,
      noOracleDecision: feature.noOracleDecision,
      comparison: feature.comparison ?? "ordered-json-v1",
      background: feature.background,
      scenarios: feature.scenarios.filter((scenario) => selectable.has(scenario.level)),
      adapters: discovered.adapters,
      fixtures,
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
  writeFileSync(join(absDir, taxonomy.testOutputMarkerFilename), `${JSON.stringify({ kind: taxonomy.testOutputMarkerKind, ...marker }, null, 2)}\n`);
}

/** 🧾️ Reads a directory's ownership marker, or `null` when it carries none. */
export function readOutputMarker(repoRoot: string, absDir: string): OutputMarker | null {
  const taxonomy = testTaxonomy(repoRoot);
  const path = join(absDir, taxonomy.testOutputMarkerFilename);
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

const FLOAT_TOLERANCE = 1e-9;

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

/** 🧭️ Fields no two PDF producers can agree on, and which the PDF specification does not make normative. */
const PDF_NONDETERMINISTIC_KEYS = new Set(["objectNumber", "xrefOffset", "producer", "creationDate", "modificationDate", "documentId", "fileSize", "byteLength", "generation", "streamFilter", "streamLength"]);

function stripKeys(value: unknown, keys: ReadonlySet<string>): unknown {
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
  if (typeof value === "number") return Math.round(value / tolerance) * tolerance;
  if (Array.isArray(value)) return value.map((entry) => roundFloats(entry, tolerance));
  if (isPlainObject(value)) return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, roundFloats(entry, tolerance)]));
  return value;
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

/**
 * ⚖️ Applies one owned comparison profile. Comparison logic lives here — not in adapters — so two
 * implementations cannot quietly agree on different notions of "the same result".
 */
export function compareProjections(profile: ComparisonProfile, oracle: unknown, subject: unknown): ComparisonVerdict {
  const diffs: ComparisonDiff[] = [];
  switch (profile) {
    case "exact-bytes-v1": {
      const a = typeof oracle === "string" ? oracle : JSON.stringify(oracle);
      const b = typeof subject === "string" ? subject : JSON.stringify(subject);
      if (a !== b) diffs.push({ path: "$", oracle: digest(a), subject: digest(b), reason: "byte-exact comparison failed" });
      break;
    }
    case "utf8-text-v1": {
      const norm = (value: unknown): string =>
        String(value)
          .replace(/\r\n/g, "\n")
          .replace(/[ \t]+$/gm, "")
          .replace(/\n+$/, "")
          .normalize("NFC");
      if (norm(oracle) !== norm(subject)) diffs.push({ path: "$", oracle: norm(oracle), subject: norm(subject), reason: "normalized text differs" });
      break;
    }
    case "ordered-json-v1":
      diffValues("$", canonicalize(oracle), canonicalize(subject), diffs, 0);
      break;
    case "unordered-json-v1":
      diffValues("$", sortDeep(oracle), sortDeep(subject), diffs, 0);
      break;
    case "floating-point-v1":
      diffValues("$", canonicalize(oracle), canonicalize(subject), diffs, FLOAT_TOLERANCE);
      break;
    case "semantic-pdf-v1":
      diffValues("$", canonicalize(roundFloats(stripKeys(oracle, PDF_NONDETERMINISTIC_KEYS), 1e-4)), canonicalize(roundFloats(stripKeys(subject, PDF_NONDETERMINISTIC_KEYS), 1e-4)), diffs, 1e-4);
      break;
    case "filesystem-tree-v1": {
      const norm = (value: unknown): unknown => (Array.isArray(value) ? [...value].map(String).sort() : sortDeep(value));
      diffValues("$", norm(oracle), norm(subject), diffs, 0);
      break;
    }
    case "diagnostic-v1": {
      const norm = (value: unknown): unknown => sortDeep(stripKeys(value, new Set(["line", "column", "offset", "span", "detail"])));
      diffValues("$", norm(oracle), norm(subject), diffs, 0);
      break;
    }
    case "event-stream-v1": {
      const norm = (value: unknown): unknown => canonicalize(stripKeys(value, new Set(["timestamp", "durationMs", "elapsedMs", "pid", "threadId"])));
      diffValues("$", norm(oracle), norm(subject), diffs, 0);
      break;
    }
  }
  return { profile, equal: diffs.length === 0, diffs };
}

/** ⚖️ Digest of a projection under a profile's canonical form — the value results carry as `projectionHash`. */
export function projectionHash(profile: ComparisonProfile, projection: unknown): string {
  switch (profile) {
    case "exact-bytes-v1":
      return digest(typeof projection === "string" ? projection : JSON.stringify(projection));
    case "unordered-json-v1":
    case "filesystem-tree-v1":
    case "diagnostic-v1":
      return digest(JSON.stringify(sortDeep(projection)));
    case "semantic-pdf-v1":
      return digest(JSON.stringify(canonicalize(roundFloats(stripKeys(projection, PDF_NONDETERMINISTIC_KEYS), 1e-4))));
    case "event-stream-v1":
      return digest(JSON.stringify(canonicalize(stripKeys(projection, new Set(["timestamp", "durationMs", "elapsedMs", "pid", "threadId"])))));
    default:
      return digest(JSON.stringify(canonicalize(projection)));
  }
}
//#endregion ⚖️Comparison

//#region 📤️Results
/** 📤️ One executed `(scenario, implementation, role)` — the single shape every native host emits. */
export type TestResult = Readonly<{
  testId: string;
  owner: string;
  case: string;
  scenario: string;
  implementation: Implementation;
  role: TestRole;
  level: TestLevel;
  status: "passed" | "failed" | "errored";
  durationMs: number;
  seed?: string;
  featureHash?: string;
  fixtureHash?: string;
  sourceHash?: string;
  dependencyFingerprint?: string;
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
    breaches.push(breach("testing/contract", "no-adapter", scope, "Test case has no implementation adapter", "A feature with no adapter can never execute, so it silently contributes zero coverage.", `Add at least one ${Object.values(taxonomy.testAdapterFilenames).join(" / ")} adapter.`));
  }

  const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
  for (const error of feature.errors) breaches.push(breach("testing/contract", "feature-syntax", discovered.featurePath, error, "The feature file must parse under the repository's restricted Gherkin profile.", "Fix the feature file syntax."));
  if (feature.capability === null) breaches.push(breach("testing/contract", "missing-capability", discovered.featurePath, "Feature is missing its @capability-<id> tag", "Every feature declares the capability it specifies so owners and implementations can be matched to it.", "Add a feature-level @capability-<id> tag."));
  if (feature.comparison === null) breaches.push(breach("testing/contract", "missing-comparison", discovered.featurePath, "Feature is missing a valid @comparison-<profile> tag", "Comparison belongs to an owned, tested profile — never to an adapter.", `Add one of ${COMPARISON_PROFILES.join(", ")}.`));
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

  const uris = fixtureUrisIn(feature);
  for (const uri of resolveFixtures(repoRoot, discovered, uris).missing) {
    breaches.push(breach("testing/fixture", "missing-fixture", discovered.featurePath, `Fixture ${uri} does not resolve`, "Fixture lookup is explicit, so an unresolved URI is a contract error rather than a runtime surprise.", `Add the file under ${uri.startsWith("shared://") ? `${discovered.owner}/${taxonomy.testFixturesDirName}` : `${discovered.caseDir}/${taxonomy.testFixturesDirName}`}.`));
  }

  const referenced = new Set(uris.map((uri) => uri.split("://")[1]));
  for (const file of fixtureFilesUnder(repoRoot, discovered.localFixtureDir)) {
    const name = file.slice(`${discovered.localFixtureDir}/`.length);
    if (!referenced.has(name)) breaches.push(breach("testing/fixture", "orphan-fixture", file, `Case-local fixture ${name} is referenced by no scenario`, "An unreferenced case-local fixture is either dead weight or evidence of a scenario that was silently deleted.", `Reference it as local://${name}, or delete it.`, "medium"));
  }

  for (const entry of readdirSync(join(repoRoot, discovered.caseDir), { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (entry.name !== taxonomy.testFixturesDirName) breaches.push(breach("testing/taxonomy", "unknown-case-child", `${discovered.caseDir}/${entry.name}`, `Unexpected directory ${entry.name} inside a test case`, `A case holds exactly one ${taxonomy.testFeatureFilename}, its adapters and an optional ${taxonomy.testFixturesDirName}.`, "Move the directory to its owner, or delete it."));
      continue;
    }
    if (entry.name === taxonomy.testFeatureFilename) continue;
    if (Object.values(taxonomy.testAdapterFilenames).includes(entry.name)) continue;
    breaches.push(breach("testing/taxonomy", "unknown-adapter-filename", `${discovered.caseDir}/${entry.name}`, `Unknown file ${entry.name} inside a test case`, "Only the feature file and taxonomy-declared adapters may live in a case directory; anything else is a committed generated wrapper or a stray scratch file.", `Rename it to one of ${Object.values(taxonomy.testAdapterFilenames).join(", ")}, or delete it.`));
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
  const hostRoots = registry.oracles.map((entry) => entry.hostPath).filter((value): value is string => value !== undefined);
  const caseDirs = new Set(discoverTestCases(repoRoot).map((entry) => entry.caseDir));
  const names = registry.oracles.map((entry) => [entry.id, entry.package.replace(/-/g, "_"), entry.package] as const);
  const isTestOwned = (rel: string): boolean => caseDirs.has(rel) || hostRoots.some((root) => rel === root || rel.startsWith(`${root}/`)) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`);
  const hits: { path: string; oracle: string }[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (isTestOwned(rel)) return "skip";
    for (const entry of readdirSync(abs, { withFileTypes: true })) {
      if (!entry.isFile() || !/\.(rs|ts|tsx|go|py|cs)$/.test(entry.name)) continue;
      const filePath = `${rel}/${entry.name}`;
      let content: string;
      try {
        content = readFileSync(join(repoRoot, filePath), "utf8");
      } catch {
        continue;
      }
      for (const [id, moduleName, packageName] of names) {
        if (new RegExp(`(^|[^A-Za-z0-9_])(use\\s+${moduleName}\\b|extern\\s+crate\\s+${moduleName}\\b|from\\s+["']${packageName}["']|require\\(["']${packageName}["']\\))`).test(content)) hits.push({ path: filePath, oracle: id });
      }
    }
    return "enter";
  });
  return hits;
}


/** 🔒️ The shrink-only migration baseline: how much legacy test debt each area is still allowed. */
export type MigrationBaseline = Readonly<{ schemaVersion: number; unmanagedTests: { total: number; byArea: Record<string, number> }; ownerStatus: Record<string, string> }>;

/** 🔒️ Ordered migration ladder every owner walks; an owner advances only on machine-readable evidence. */
export const MIGRATION_STATUSES = ["discovered", "surveyed", "contract-ready", "oracle-green", "subject-green", "parity-green", "coverage-green", "dependency-clean", "legacy-removed", "ci-enforced", "complete"] as const;
export type MigrationStatus = (typeof MIGRATION_STATUSES)[number];

/** 🔒️ Loads the committed migration baseline. */
export function loadMigrationBaseline(repoRoot: string): MigrationBaseline {
  const path = join(repoRoot, TEST_DOMAIN_REL_PATH, "📇️registry", "🔒️migration.json");
  if (!existsSync(path)) return { schemaVersion: 1, unmanagedTests: { total: 0, byArea: {} }, ownerStatus: {} };
  return JSON.parse(readFileSync(path, "utf8")) as MigrationBaseline;
}

/** 🧾️ Repository-wide contract sweep across every discovered case. */
export function validateAllContracts(repoRoot: string, cases: readonly DiscoveredCase[] = discoverTestCases(repoRoot)): BreachRecord[] {
  const registry = loadOracleRegistry(repoRoot);
  const breaches = cases.flatMap((discovered) => validateCaseContract(repoRoot, discovered, registry));
  for (const leak of cases.filter((discovered) => discovered.owner.startsWith("compose/"))) {
    breaches.push(breach("testing/discovery", "compose-leak", leak.caseDir, "Discovery returned a compose/ path", "compose/** is a hard forbidden path for this project and must be excluded in the discovery library itself.", "Fix testExcludedPathPrefixes in 🔣️taxonomy.json."));
  }
  for (const hit of oracleImportsInProduction(repoRoot)) {
    breaches.push(breach("testing/dependency", "oracle-in-production", hit.path, `Production source imports the registered oracle ${hit.oracle}`, "An oracle is evidence a test host gathers. Once production code can reach it, the differential test compares an implementation with itself and the dependency stops being test-only.", "Move the usage into the oracle host, or remove the oracle from the registry."));
  }
  // 🔒️Shrink-only migration ratchet: the legacy backlog may only get smaller. Reported as one
  // ratcheted count per area rather than thousands of individual findings, so the signal stays
  // meaningful while Phase 6 migrates owners.
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
export type TestCleanRemoval = Readonly<{ category: string; path: string; files: number; bytes: number; reason: "all" | "stale" | "incomplete" }>;

/** 🧹️ What a clean run did (or, in dry mode, would do) — identical structure either way. */
export type TestCleanReport = Readonly<{ dry: boolean; removals: readonly TestCleanRemoval[]; protectedPaths: readonly string[]; skippedUnmarked: readonly string[] }>;

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
 * followed — so no tracked fixture, no source file and no `compose/` path can be reached from here.
 */
export function cleanTestOutputs(repoRoot: string, opts: { dry?: boolean; stale?: boolean; liveTestIds?: ReadonlySet<string> } = {}): TestCleanReport {
  const dry = opts.dry ?? false;
  const taxonomy = testTaxonomy(repoRoot);
  const root = resolve(testCacheRoot(repoRoot));
  const removals: TestCleanRemoval[] = [];
  const skippedUnmarked: string[] = [];
  const protectedPaths = [relative(repoRoot, join(getRepoMetaDir(repoRoot), "⚡️cache")).split(sep).join("/")];
  if (!existsSync(root)) return { dry, removals, protectedPaths, skippedUnmarked };

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
      if (opts.stale === true && !isStale && !isIncomplete) continue;
      const { files, bytes } = countTree(abs);
      removals.push({ category: child, path: relative(repoRoot, abs).split(sep).join("/"), files, bytes, reason: isStale ? "stale" : isIncomplete ? "incomplete" : "all" });
      if (!dry) rmSync(abs, { recursive: true, force: true });
    }
  }
  return { dry, removals, protectedPaths, skippedUnmarked };
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
  const registered = new Set(registry.oracles.map((entry) => entry.package));
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
  /** 🎲️ Deterministic seed for this scenario, from its `@seed-…` tag. */
  seed: string;
}>;

/** 🧭️ What a scenario handler returns: the raw artifact plus the projection the profile compares. */
export type AdapterOutcome = Readonly<{ raw?: string | Uint8Array; projection: unknown; diagnostics?: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[] }>;

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
  return {
    plan,
    scenario,
    role,
    repoRoot,
    workDir,
    seed: scenario.seed ?? "0",
    fixture: lookup,
    fixtureBytes: (uri) => readFileSync(lookup(uri)),
    copyFixture: (uri, as) => {
      const source = lookup(uri);
      const target = join(workDir, as ?? basename(source));
      mkdirSync(dirname(target), { recursive: true });
      cpSync(source, target);
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
  const ownerId = `${discovered.owner}::${discovered.case}`;
  markOutputDir(repoRoot, workDir, { testId: ownerId, cacheKey });
  markOutputDir(repoRoot, outputDir, { testId: ownerId, cacheKey });
  const plan: TestCasePlan = { ...base, role, implementation, workDir, outputDir, resultsPath: join(outputDir, "📤️results.jsonl") };
  const planPath = join(workDir, "📋️plan.json");
  writeFileSync(planPath, `${JSON.stringify(plan, null, 2)}\n`);
  return { plan, feature, missingFixtures, planPath };
}

/** 🏁️ Marks a generated output directory complete, so an interrupted run stays distinguishable from a finished one. */
export function markRunComplete(absDir: string): void {
  writeFileSync(join(absDir, "🏁️done"), "");
}

/** 🧮️ Pairs every subject result with its oracle counterpart and applies the case's comparison profile. */
export function evaluateParity(profile: ComparisonProfile, results: readonly TestResult[]): { verdicts: { testId: string; profile: ComparisonProfile; equal: boolean; diffs: number; verdict: ComparisonVerdict }[]; unmatched: string[] } {
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
    const verdict = compareProjections(profile, oracle.output.projection, result.output.projection);
    verdicts.push({ testId: result.testId, profile, equal: verdict.equal, diffs: verdict.diffs.length, verdict });
  }
  return { verdicts, unmatched };
}

/** 🧮️ Pairwise subject equivalence, so two implementations cannot exploit different oracle ambiguities. */
export function evaluateCrossSubjectParity(profile: ComparisonProfile, results: readonly TestResult[]): { pair: string; equal: boolean; diffs: number }[] {
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
        const verdict = compareProjections(profile, sorted[i]!.output.projection, sorted[j]!.output.projection);
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

//#region 🧭️Root
/** 📁️ Repository root, resolved from this package's own location. */
export function repoRootFromHere(): string {
  return findRepoRoot(dirname(decodeURIComponent(new URL(import.meta.url).pathname)));
}
//#endregion 🧭️Root
