/** 🔌️ Owns the neutral scenario adapter protocol and registration contract. */
import type { TestLevel } from "../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

export const IMPLEMENTATIONS = ["rust", "typescript", "go", "python", "dotnet"] as const;

export type Implementation = (typeof IMPLEMENTATIONS)[number];

export const TEST_ROLES = ["oracle", "subject"] as const;

export type TestRole = (typeof TEST_ROLES)[number];

export const TEST_MODES = ["differential", "conformance", "round-trip", "property", "error"] as const;

export type TestMode = (typeof TEST_MODES)[number];

export type ComparisonProfile = string;

export type FeatureStep = Readonly<{ keyword: "Given" | "When" | "Then"; rawKeyword: string; text: string; docString?: string; dataTable?: readonly (readonly string[])[] }>;

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

export type ResolvedFixture = Readonly<{ uri: string; scope: "shared" | "asset" | "schema"; name: string; path: string; digest: string }>;

export type SubjectRawInputs = Readonly<Record<string, Readonly<Partial<Record<Implementation, string>>>>>;

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
  /** 📦️ Raw outputs produced by subject hosts before this oracle host starts, per scenario id then implementation. */
  subjectRawInputs?: SubjectRawInputs;
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
  /** 📥️ Bytes THIS scenario's subject host produced in `implementation`, for an `@oracle-input-subject-raw` oracle; throws when absent. */
  subjectRawBytes(implementation: Implementation): Uint8Array;
  /** 📦️ Directory a handler writes its produced artifact bundle into. */
  artifactDir: string;
  /** 📦️ Absolute path to write one named result artifact to — `<artifactDir>/<scenario id>/<role>/<filename>`, so a
   *  scenario's artifacts never overwrite another's; creates parent directories. */
  artifact(role: string, filename: string): string;
  /** 🎲️ Deterministic seed for this scenario, from its `@seed-…` tag. */
  seed: string;
  /** 🪆️ The Examples row id this scenario expands; throws for a plain scenario. */
  row(): string;
}>;

export type AdapterOutcome = Readonly<{
  raw?: string | Uint8Array;
  projection: unknown;
  /** 📦️ Named files this handler produced, relative to `ctx.artifactDir`. A BRep case returns its STEP and its mesh here rather than smuggling them through the projection. */
  artifacts?: readonly { role: string; path: string; mediaType: string }[];
  /** 🏭️ Set by a SUBJECT handler that invoked production dispatch. Omitting it is how a vector-replay adapter is detected. */
  productionDispatch?: { invoked: true; operation: string; bridgeVersion: number };
  diagnostics?: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
}>;

export type TestAdapter = Readonly<{ implementation: Implementation; scenarios: Readonly<Record<string, Readonly<{ subject?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome>; oracle?: (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> }>>> }>;

export function defineTestAdapter(adapter: TestAdapter): TestAdapter {
  return adapter;
}

export type PlatformId = `${"linux" | "darwin" | "win32"}-${"x64" | "arm64"}`;

export type SubsetTarget = Readonly<{
  artifact: string;
  standard: string;
  subset: string;
  surface?: string;
  compound?: readonly string[];
  selector?: Readonly<{ type: "entity-id" | "entity-path" | "entity-set" | "whole-subset"; value: string | readonly string[] }>;
}>;

export const MUTATION_OUTCOME_CLASSES = ["applied", "no-op", "empty", "disjoint", "rejected"] as const;

export type MutationOutcomeClass = (typeof MUTATION_OUTCOME_CLASSES)[number];

export type EngineFamily = Readonly<{ family: string; implementation: string; version: string }>;

export type FixtureInvariants = Readonly<{ local?: readonly string[]; enclosing?: readonly string[] }>;

export const FIXTURE_CLASSES = ["real-world", "handcrafted", "third-party-generated"] as const;

export type FixtureClass = (typeof FIXTURE_CLASSES)[number];

export type FixtureFile = Readonly<{ role: string; path: string; mediaType: string; sha256: string; bytes?: number }>;

export type FixtureGenerator = Readonly<{ oracle: string; packageVersion: string; engineFamily: string; engineVersion: string; command: string; seed?: string | number; platform: PlatformId; sourceDigest?: string; exportEngine?: EngineFamily }>;

export type FixtureProvenance = Readonly<{ source: "generated" | "authored" | "downloaded" | "vendored"; license: string; acquiredAt?: string; attribution?: string; url?: string; security?: "scanned-clean" | "unscanned" | "quarantined"; privacy?: "no-personal-data" | "reviewed" | "unreviewed" }>;

export type FixtureUnits = Readonly<{ length: string; angle: string; handedness?: "right" | "left"; up?: "y" | "z" }>;

export type ToleranceOverride = Readonly<{ reason: string; measuredBaseline: number; factor: number; approvedBy: string }>;

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
  invariants?: FixtureInvariants;
  comparisonPipeline?: string;
  reproducibilityDiffs?: readonly string[];
  /** 📁️ Repo-relative directory the manifest was read from; `files[].path` resolves against it. */
  manifestDir?: string;
}>;
