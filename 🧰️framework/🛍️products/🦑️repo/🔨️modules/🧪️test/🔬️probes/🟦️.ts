/** 🔬️ Measurement probes
 *
 * Probe entries, reports and qualification of external measurement probes.
 */

//#region 🔌️Adapters
import { type PlatformId, type EngineFamily } from "@semio-tech/framework/test/adapter";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
import { type OracleLinkedPackage } from "../🗃️oracle-registry/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔬️Probe
/** 🔧️ One EXTERNAL measurement tool. The orchestrator invokes it and compares what it emits — never computes. */
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

/** ⛩️ Whether a probe's strongest guarantee may be claimed by a gate. */
export function isQualifiedProbe(probe: ProbeEntry | undefined): boolean {
  return probe !== undefined && probe.qualification?.status === "qualified";
}

/** 📊️ The typed JSON one probe emits for one stage. `measurements` is probe-defined, by design. */
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
