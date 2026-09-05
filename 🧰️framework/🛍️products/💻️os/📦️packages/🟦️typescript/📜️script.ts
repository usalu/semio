#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { BundleScript, ScriptRouter, getWorkspaceRoot, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

/** 🏗️ Routes the package generator through the shared workspace implementation. */
class GenerateWgpuScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "generate"); }
}
/** 🔎️ Checks the exact package artifacts without writing outputs. */
class CheckWgpuScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "check"); }
}
/** 🔮️ Streams the canonical read-only package preview. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "preview"); }
}


//#region 💡️InferencePortCheck
/** 💡️ One rendered phase of the host-owned inference port, restated independently of production. */
type OraclePreview = { schema: string; jobId: string; proposalHash: string; regionId: string; ring: readonly (readonly [number, number])[] };
type OracleStatus = { phase: string; jobId: string | null; cursor: number; completed: number; total: number; proposalHash: string | null; preview?: OraclePreview; cancelRequested: boolean; code: string | null };

/** 🎬️ One closed input the port's state machine accepts, as the neutral corpus spells it. */
type OracleEvent = { kind: string; receipt?: Record<string, unknown>; page?: Record<string, unknown>; code?: string };

/** 🧬️ The exact shape of `💡️gis-map-inference-port-v1`, so the oracle reads a typed corpus. */
type OracleCorpus = {
  phases: readonly string[];
  ariaRoles: Readonly<Record<string, string>>;
  codes: readonly string[];
  limits: Readonly<Record<string, number>>;
  serviceId: string;
  sampleJobId: string;
  otherJobId: string;
  proposalHash: string;
  preview: OraclePreview;
  initial: OracleStatus;
  successLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  cancelLifecycle: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  leaseRefusal: readonly { name: string; event: OracleEvent; expected: OracleStatus }[];
  hostileTransitions: readonly { name: string; from: OracleStatus; event: OracleEvent; expected: OracleStatus }[];
  crossFixture: { approvalFixture: string; serverLifecycleKinds: readonly string[]; serverCancelLifecycleKinds: readonly string[]; serverKindToPhase: Readonly<Record<string, string>> };
  nonclaims: readonly string[];
};

const ORACLE_TERMINALS = new Set(["applied", "cancelled", "stale", "failed"]);

/** 🗺️ An independent projection of one server answer onto a rendered phase. It is hand-written
 * against the packet's own state law, imports nothing from the production schema module, and exists
 * exactly so a defect in `reduceGisMapInferencePortV1` cannot hide behind itself. */
function oracleServerPhase(page: { state: string; proposalState: string; stale: boolean }): string {
  if (page.stale) return "stale";
  if (page.proposalState === "stale") return "stale";
  if (page.state === "cancelled") return "cancelled";
  if (page.proposalState === "cancelled") return "cancelled";
  if (page.state === "failed") return "failed";
  if (page.proposalState === "approved") return "applied";
  if (page.proposalState === "offered") return "offered";
  if (page.state === "succeeded") return "offered";
  return "running";
}

/** 🧮️ The independent transition relation the corpus is checked against. */
function oracleReduce(current: OracleStatus, event: OracleEvent): OracleStatus {
  const idle: OracleStatus = { phase: "idle", jobId: null, cursor: 0, completed: 0, total: 0, proposalHash: null, cancelRequested: false, code: null };
  if (event.kind === "clear") return idle;
  if (ORACLE_TERMINALS.has(current.phase)) return current;
  if (event.kind === "start") return current.phase === "idle" ? { ...idle, phase: "submitting" } : current;
  if (event.kind === "lease-unverified") return current.phase === "idle" || current.phase === "submitting" ? { ...current, phase: "failed", code: "inference.lease-unverified" } : current;
  if (event.kind === "receipt") {
    if (current.phase !== "submitting") return current;
    const receipt = event.receipt as { jobId: string; state: string; proposalState: string; proposalHash?: string; cursor: number };
    const { preview: _preview, ...withoutPreview } = current;
    return { ...withoutPreview, phase: oracleServerPhase({ state: receipt.state, proposalState: receipt.proposalState, stale: false }), jobId: receipt.jobId, cursor: receipt.cursor, proposalHash: receipt.proposalHash ?? null };
  }
  if (event.kind === "page") {
    const page = event.page as { jobId: string; state: string; proposalState: string; cancelRequested: boolean; stale: boolean; proposalHash?: string; preview?: OraclePreview; progress: { completed: number; total: number }[]; nextCursor: number };
    if (current.jobId === null || current.jobId !== page.jobId) return current;
    const server = oracleServerPhase(page);
    const phase = current.phase === "approving" && !ORACLE_TERMINALS.has(server) ? "approving" : server;
    const last = page.progress.length === 0 ? undefined : page.progress[page.progress.length - 1];
    return {
      phase,
      jobId: current.jobId,
      cursor: Math.max(current.cursor, page.nextCursor),
      completed: last === undefined ? current.completed : last.completed,
      total: last === undefined ? current.total : last.total,
      proposalHash: page.proposalHash ?? null,
      ...((phase === "offered" || phase === "approving") && page.preview !== undefined ? { preview: page.preview } : {}),
      cancelRequested: current.cancelRequested || page.cancelRequested,
      code: phase === "failed" ? (current.code ?? "inference.storage") : current.code,
    };
  }
  if (event.kind === "approve") return current.phase === "offered" && current.proposalHash !== null && current.preview?.proposalHash === current.proposalHash && current.preview.jobId === current.jobId && !current.cancelRequested ? { ...current, phase: "approving" } : current;
  if (event.kind === "approval") {
    const receipt = event.receipt as { jobId: string; proposalHash: string; applied: boolean };
    if (current.phase !== "approving" || current.jobId !== receipt.jobId || current.proposalHash !== receipt.proposalHash) return current;
    const { preview: _preview, ...withoutPreview } = current;
    return receipt.applied ? { ...withoutPreview, phase: "applied" } : { ...withoutPreview, phase: "failed", code: "approval.commit-unavailable" };
  }
  if (event.kind === "cancel") return current.phase === "idle" ? current : { ...current, cancelRequested: true };
  if (event.kind === "failed") {
    const { preview: _preview, ...withoutPreview } = current;
    return { ...withoutPreview, phase: event.code === "inference.cancelled" ? "cancelled" : "failed", code: event.code ?? null };
  }
  throw new Error(`gis-map-inference-port oracle: unknown event ${event.kind}`);
}

/** ⚖️ Field-order-independent equality over the exact closed status shape. */
function sameStatus(left: OracleStatus, right: OracleStatus): boolean {
  const fields: readonly (keyof OracleStatus)[] = ["phase", "jobId", "cursor", "completed", "total", "proposalHash", "cancelRequested", "code"];
  return fields.every((field) => left[field] === right[field]) && JSON.stringify(left.preview) === JSON.stringify(right.preview);
}

/** 🧪️ Validates the neutral corpus with AJV 2020, walks both lifecycles and every hostile
 * transition through an INDEPENDENT hand-written state machine AND the production reducer, checks
 * the explicit EN/DE vocabulary is total with no default language, and cross-checks the corpus
 * against the hub's own `🗳️gis-map-proposal-approval-v1` lifecycles and limits plus the `🖥️shell`
 * cross-language twin. It never runs a real hub, a model, or a renderer. */
async function proveGisMapInferencePortFixture(repoRoot: string): Promise<Record<string, number>> {
  const root = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🧫️fixtures", "💡️gis-map-inference-port-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as OracleCorpus;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  const validate = ajv.compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid GIS Map inference port corpus: ${JSON.stringify(validate.errors)}`);
  const hostileCorpora: readonly unknown[] = [
    { ...fixture, phases: fixture.phases.slice(1) },
    { ...fixture, ariaRoles: { ...fixture.ariaRoles, failed: "status" } },
    { ...fixture, limits: { ...fixture.limits, progressMaxCursor: 17 } },
    { ...fixture, codes: fixture.codes.slice(1) },
    { ...fixture, successLifecycle: fixture.successLifecycle.slice(1) },
    { ...fixture, nonclaims: [...fixture.nonclaims.slice(1), fixture.nonclaims[1]] },
    { ...fixture, serviceId: "s.gis.gismap.other" },
  ];
  for (const [index, candidate] of hostileCorpora.entries()) if (validate(candidate)) throw new Error(`GIS Map inference port corpus accepted hostile mutation ${index}`);

  const production = await import("../../🟦️.ts");
  if (JSON.stringify(production.parseGisMapInferencePreviewV1(fixture.preview)) !== JSON.stringify(fixture.preview)) throw new Error("production preview parser changed the validated projection");
  for (const candidate of [{ ...fixture.preview, jobId: fixture.otherJobId }, { ...fixture.preview, regionId: "substituted" }, { ...fixture.preview, ring: [[7, 46], [9, 46], [8, 48], [7, 48], [7, 46]] }]) {
    try {
      production.parseGisMapInferencePreviewV1(candidate);
      throw new Error("production preview parser accepted substituted geometry");
    } catch (error) {
      if (error instanceof Error && error.message === "production preview parser accepted substituted geometry") throw error;
    }
  }
  let transitions = 0;
  const walk = (steps: readonly { name: string; event: OracleEvent; expected: OracleStatus }[]): void => {
    let oracleState: OracleStatus = structuredClone(fixture.initial);
    let productionState: OracleStatus = production.idleGisMapInferencePortStatusV1() as OracleStatus;
    for (const step of steps) {
      oracleState = oracleReduce(oracleState, step.event);
      productionState = production.reduceGisMapInferencePortV1(productionState as never, step.event as never) as unknown as OracleStatus;
      if (!sameStatus(oracleState, step.expected)) throw new Error(`independent oracle disagrees with the corpus at ${step.name}: ${JSON.stringify(oracleState)}`);
      if (!sameStatus(productionState, step.expected)) throw new Error(`production reducer disagrees with the corpus at ${step.name}: ${JSON.stringify(productionState)}`);
      transitions += 1;
    }
  };
  walk(fixture.successLifecycle);
  walk(fixture.cancelLifecycle);
  walk(fixture.leaseRefusal);
  for (const row of fixture.hostileTransitions) {
    const viaOracle = oracleReduce(structuredClone(row.from), row.event);
    const viaProduction = production.reduceGisMapInferencePortV1(structuredClone(row.from) as never, row.event as never) as unknown as OracleStatus;
    if (!sameStatus(viaOracle, row.expected)) throw new Error(`independent oracle disagrees with hostile row ${row.name}: ${JSON.stringify(viaOracle)}`);
    if (!sameStatus(viaProduction, row.expected)) throw new Error(`production reducer disagrees with hostile row ${row.name}: ${JSON.stringify(viaProduction)}`);
    transitions += 1;
  }

  let strings = 0;
  for (const phase of fixture.phases) {
    const row = production.GIS_MAP_INFERENCE_PORT_TEXT_V1[phase as keyof typeof production.GIS_MAP_INFERENCE_PORT_TEXT_V1];
    if (row === undefined || Object.keys(row).sort().join(",") !== "de,en" || row.en.length === 0 || row.de.length === 0 || row.en === row.de) throw new Error(`phase ${phase} has no explicit EN/DE text`);
    if (production.gisMapInferencePortRoleV1(phase as never) !== fixture.ariaRoles[phase]) throw new Error(`phase ${phase} has the wrong live-region politeness`);
    strings += 2;
  }
  for (const code of fixture.codes) {
    const row = production.GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1[code as keyof typeof production.GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1];
    if (row === undefined || Object.keys(row).sort().join(",") !== "de,en" || row.en === row.de) throw new Error(`code ${code} has no explicit EN/DE text`);
    strings += 2;
  }
  for (const [control, row] of Object.entries(production.GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1)) {
    if (Object.keys(row).sort().join(",") !== "de,en" || row.en === row.de) throw new Error(`control ${control} has no explicit EN/DE text`);
    strings += 2;
  }
  if (production.GIS_MAP_INFERENCE_REQUEST_MAX_BYTES !== fixture.limits.requestMaxBytes
    || production.GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES !== fixture.limits.responseMaxBytes
    || production.GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR !== fixture.limits.progressMaxCursor
    || production.GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS !== fixture.limits.eventPageMaxItems
    || production.GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS !== fixture.limits.jobMaxLifetimeMs
    || production.GIS_MAP_INFERENCE_SERVICE_ID !== fixture.serviceId) {
    throw new Error("the production port constants and the neutral corpus disagree");
  }

  const approval = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️fixtures", "🗳️gis-map-proposal-approval-v1", "🔣️.json"), "utf8")) as {
    lifecycle: readonly { kind: string }[];
    cancelLifecycle: readonly { kind: string }[];
    limits: Readonly<Record<string, number>>;
    proposalHash: string;
    sampleJobId: string;
    preview: OraclePreview;
  };
  const approvalKinds = approval.lifecycle.map((row) => row.kind);
  const approvalCancelKinds = approval.cancelLifecycle.map((row) => row.kind);
  if (JSON.stringify(approvalKinds) !== JSON.stringify(fixture.crossFixture.serverLifecycleKinds)) throw new Error("the hub approval lifecycle and this corpus disagree");
  if (JSON.stringify(approvalCancelKinds) !== JSON.stringify(fixture.crossFixture.serverCancelLifecycleKinds)) throw new Error("the hub cancel lifecycle and this corpus disagree");
  for (const kind of [...approvalKinds, ...approvalCancelKinds]) if (!(kind in fixture.crossFixture.serverKindToPhase)) throw new Error(`hub lifecycle kind ${kind} has no rendered phase`);
  if (approval.limits.progressMaxCursor !== fixture.limits.progressMaxCursor || approval.limits.eventPageMaxItems !== fixture.limits.eventPageMaxItems || approval.limits.jobMaxLifetimeMs !== fixture.limits.jobMaxLifetimeMs || approval.limits.requestMaxBytes !== fixture.limits.requestMaxBytes) {
    throw new Error("the hub approval limits and this corpus disagree");
  }
  if (approval.proposalHash !== fixture.proposalHash || approval.sampleJobId !== fixture.sampleJobId) throw new Error("the hub approval identity and this corpus disagree");
  if (JSON.stringify(approval.preview) !== JSON.stringify(fixture.preview)) throw new Error("the Hub and browser preview projections disagree");
  const rustClients = [
    readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🦀️.rs"), "utf8"),
    readFileSync(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🌉️mcp", "💡️inference", "🦀️.rs"), "utf8"),
  ];
  for (const source of rustClients) {
    if (!source.includes("pub struct GisMapInferencePreviewV1") || !source.includes("pub preview: Option<GisMapInferencePreviewV1>") || !source.includes("pub ring: [[f64; 2]; 5]")) throw new Error("a strict Rust Hub client is missing the optional typed preview DTO");
    if (/derive\([^\n]*\bEq\b[^\n]*\)\n[^\n]*\npub struct GisMapInferenceEventPageV1/u.test(source)) throw new Error("a Rust events page unsafely derives Eq through floating-point preview geometry");
  }
  const directoryRust = rustClients[0]!;
  if (/derive\([^\n]*\bEq\b[^\n]*\)\n[^\n]*\npub struct GisMapInferencePortStatusV1/u.test(directoryRust) || !directoryRust.includes("next.preview = if matches!(phase") || !directoryRust.includes("let preview_matches = current.preview.as_ref()")) throw new Error("the shared Rust inference reducer does not retain and require the typed preview safely");

  const shellRoot = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🖥️shell");
  const mirror = readFileSync(join(shellRoot, "🤖️generated", "🟦️.ts"), "utf8");
  const declared = /export type InferencePortPhase = ([^;]+);/u.exec(mirror);
  if (declared === null) throw new Error("the 🖥️shell TypeScript mirror declares no InferencePortPhase");
  const mirrored = declared[1]!.split("|").map((part) => part.trim().replace(/^"|"$/gu, ""));
  if (JSON.stringify(mirrored) !== JSON.stringify(fixture.phases)) throw new Error(`the 🖥️shell twin's phases differ from this corpus: ${JSON.stringify(mirrored)}`);
  const shellRust = readFileSync(join(shellRoot, "🦀️.rs"), "utf8");
  let twinStrings = 0;
  for (const phase of fixture.phases) {
    const row = production.GIS_MAP_INFERENCE_PORT_TEXT_V1[phase as keyof typeof production.GIS_MAP_INFERENCE_PORT_TEXT_V1];
    for (const text of [row.en, row.de]) {
      if (!shellRust.includes(JSON.stringify(text))) throw new Error(`the 🖥️shell Rust twin is missing the ${phase} text ${JSON.stringify(text)}`);
      twinStrings += 1;
    }
  }

  return { ajv: 1, hostileCorpora: hostileCorpora.length, transitions, strings, twinStrings, crossFixture: 3 };
}

/** ⚖️ `os:gis-map-inference-port-check` — the neutral corpus, its independent oracle, and the
 * browser port's own vitest laws. `--browser` additionally runs the worker suite. */
class GisMapInferencePortCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const browser = segments.includes("--browser");
    const { rest } = resolveTestLevel(segments.filter((segment) => segment !== "--browser"));
    const receipts = await proveGisMapInferencePortFixture(this.repoRoot);
    console.log(`gis-map-inference-port-oracle: ${Object.entries(receipts).map(([key, value]) => `${key}=${value}`).join(" ")}`);
    if (browser) await runVitest(this.root, ["--testNamePattern", "gis map inference port", ...rest], "🧪️tests/🟦️.ts");
    console.log("gis-map-inference-port-check: no WGPU map rendering, no external model provider and no two-user process journey is run or claimed here.");
  }
}
//#endregion 💡️InferencePortCheck

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("generate-wgpu", GenerateWgpuScript)
  .register("check-wgpu", CheckWgpuScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("gis-map-inference-port-check", GisMapInferencePortCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
