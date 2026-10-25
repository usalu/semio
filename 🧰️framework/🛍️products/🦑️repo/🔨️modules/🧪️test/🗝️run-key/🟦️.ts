/** ⌨️ Run keys
 *
 * The content-addressed key and manifest of one run.
 */

//#region 🔌️Adapters
import { type TestEvidence, type PlatformId } from "@semio-tech/framework/test/adapter";
import { type TestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { type ComparisonProfileSpec } from "../🗂️taxonomy/🟦️.ts";
import { digest, setDigest } from "../🔢️digests/🟦️.ts";
import { canonicalize } from "../🟰️comparison/🟦️.ts";
import { type OracleEntry } from "../🗃️oracle-registry/🟦️.ts";
import { engineFamilyId, type MutationManifest, mutationManifestDigest } from "../📑️mutation-manifest/🟦️.ts";
import { type ProbeEntry, type ResultArtifact } from "../🔬️probes/🟦️.ts";
import { type ToleranceProfile } from "../📏️tolerances/🟦️.ts";
import { type ComparisonPipeline } from "../🔀️comparison-pipelines/🟦️.ts";
import { type RetentionClass } from "../🔐️run-lease/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🗝️RunKey
/** 🗝️ Everything a reusable run result is keyed by. Anything absent here can silently reuse a stale result. */
export type RunKeyComponents = Readonly<{
  baselineSha: string;
  mutationManifestDigest: string;
  testEvidenceDigest: string;
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

/** 🔑️ The run key itself — a digest over the ordered component record. */
export function runKey(components: RunKeyComponents): string {
  return digest(JSON.stringify(canonicalize(components)));
}

/** 📮️ The full run manifest written beside a published run directory. */
export type RunManifest = Readonly<{ schema: "semio.repository-test.run-manifest/v2"; runId: string; runKey: string; baselineSha: string; startedAt?: string; finishedAt?: string; retention: RetentionClass; components: RunKeyComponents; artifacts?: readonly ResultArtifact[] }>;

/**
 * 🧮️ Assembles the run key of one execution. The TOLERANCE PROFILE is folded into
 * `comparisonProfileDigest` deliberately: a loosened tolerance that reused an earlier parity verdict
 * would report a pass nobody ever measured.
 */
export function computeRunKey(opts: {
  baselineSha: string;
  manifest: MutationManifest | null;
  fixtures: readonly TestEvidence[];
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
    testEvidenceDigest: setDigest(opts.fixtures.map((fixture) => [fixture.id, digest(JSON.stringify(canonicalize(fixture)))] as const)),
    fixtureFileDigests: setDigest(opts.fixtures.flatMap((fixture) => fixture.files.map((file) => [`${fixture.id}/${file.role}`, file.sha256] as const))),
    oracleLockDigest: opts.oracle === undefined ? "" : (opts.oracle.lockDigest ?? `${opts.oracle.package}@${opts.oracle.version ?? "*"}`),
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
