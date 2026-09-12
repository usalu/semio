/**
 * 🖌️ The TypeScript twin of `🖱️ui/🧪️tests/📃️document-lease-owner-move/🦀️.rs`. Both read the SAME
 * neutral fixture (`🖱️ui/🧫️fixtures/📃️document-lease-owner-move/🔣️.json`): the Rust law proves the
 * alias-credit behaviour against the live document arena, and this one proves the wgpu frame path
 * actually obeys it — the chrome walk moves the exact owner instead of aliasing, a terminal document
 * cursor releases the chrome step, a superseded frame opportunity is not a recorded renderer fault,
 * the effect-storm budget is charged per round, and both browser drive loops are bounded by a share of
 * the one ratified interactive ceiling.
 *
 * Source-scanning is the honest oracle here: every rule lives inside a `wasm32`/GPU-only frame path
 * that no unit harness can construct, and the sources ARE the contract (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
 */
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import laws from "./laws.json";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");
const fixture = JSON.parse(readFileSync(resolve(suiteRoot, laws.fixture), "utf8")) as {
  readonly aliasCapacity: number;
  readonly publishedAliases: number;
  readonly aliasReads: { readonly admitted: number; readonly refusedAt: number; readonly refusal: string };
  readonly ownerMove: { readonly reads: number; readonly refusals: number };
  readonly ingressRefusals: readonly { readonly fault: string; readonly terminal: boolean }[];
  readonly frameOpportunity: readonly { readonly condition: string; readonly verdict: string }[];
  readonly effectStorm: { readonly budget: number; readonly chargedPer: string };
  readonly terminalCursor: Record<string, string>;
};

const source = (key: "shellSource" | "interpreterSource" | "rendererSource" | "frameJobSource" | "winitSource") => readFileSync(join(engineRoot, laws[key]), "utf8");

describe("wgpu retained document owner move", () => {
  it("keeps the fixture's own alias arithmetic self-consistent", () => {
    expect(fixture.aliasReads.admitted + fixture.publishedAliases).toBe(fixture.aliasCapacity);
    expect(fixture.aliasReads.refusedAt).toBe(fixture.aliasReads.admitted + 1);
    expect(fixture.ownerMove.refusals).toBe(0);
    expect(fixture.ownerMove.reads).toBeGreaterThan(fixture.aliasCapacity);
  });

  it("reads a window body by moving the exact owner, never by minting an alias", () => {
    const shell = source("shellSource");
    for (const reader of laws.documentReaders) expect(shell, `shell must read a document through ${reader}`).toContain(reader);
    const code = shell.split("\n").filter((line) => !line.trimStart().startsWith("//") && !line.trimStart().startsWith("///")).join("\n");
    expect(code, "a per-frame chrome read must never alias the document lease").not.toContain(laws.forbiddenDocumentReader);
  });

  it("releases the chrome step for a terminal document cursor and holds it for every other phase", () => {
    const shell = source("shellSource");
    const releases = [...shell.matchAll(new RegExp(`${laws.terminalCursorRelease}\\(\\)`, "gu"))];
    const phases = Object.entries(fixture.terminalCursor).filter(([phase]) => !phase.startsWith("$"));
    const holders = phases.filter(([, verdict]) => verdict === "hold").map(([phase]) => phase);
    expect(releases.length, "both the main window and the panel walk release a terminal cursor").toBe(2);
    expect(holders.length).toBe(phases.length - 2);
    const interpreter = source("interpreterSource");
    for (const phase of holders) expect(interpreter, `the cursor must still have a ${phase} phase to hold on`).toContain(`UiDocumentFramePhase::${phase[0]!.toUpperCase()}${phase.slice(1)}`);
  });

  it("retries every refusable ingress fault and only faults on the terminal ones", () => {
    const interpreter = source("interpreterSource");
    for (const row of fixture.ingressRefusals) {
      const named = interpreter.includes(`UiDocumentIngressFault::${row.fault}`);
      expect(named, `${row.fault} must be classified explicitly, not by a catch-all`).toBe(!row.terminal);
    }
  });

  it("treats a superseded frame opportunity as a rebuild, not as a recorded renderer fault", () => {
    const renderer = source("rendererSource");
    const superseded = [...renderer.matchAll(new RegExp(`AppFrameTransactionStep::${laws.supersededVariant}`, "gu"))];
    expect(superseded.length, "the variant plus one answer per guard").toBeGreaterThanOrEqual(3);
    expect(fixture.frameOpportunity.every((row) => row.verdict === "superseded"), "every freshness condition is a rebuild, not a fault").toBe(true);
    const start = renderer.indexOf("context.set_stage(self.stage_label());");
    const end = renderer.indexOf("self.base_witness = Some(current_witness);", start);
    expect(start, "the freshness guard opens the transaction step").toBeGreaterThan(0);
    expect(end, "the freshness guard closes on the base witness it adopts").toBeGreaterThan(start);
    expect(renderer.slice(start, end), "no freshness condition may record a renderer fault").not.toContain(laws.supersededFaultRecorder);
    expect(source("frameJobSource")).toContain(`AppFrameTransactionStep::${laws.supersededVariant}`);
  });

  it("charges the effect-storm budget once per flush round", () => {
    const renderer = source("rendererSource");
    expect(fixture.effectStorm.chargedPer).toBe("round");
    expect(renderer).toContain(`const EFFECT_STORM_BUDGET: u32 = ${fixture.effectStorm.budget};`);
    const charge = renderer.indexOf("self.effect_opportunities = next;");
    const round = renderer.indexOf(laws.effectStormChargeSite);
    expect(charge, "the charge sits on the transition into the stage").toBeGreaterThan(0);
    expect(round - charge).toBeGreaterThan(0);
    expect(round - charge).toBeLessThan(200);
  });

  it("bounds both browser drive loops by a share of the one ratified interactive ceiling", () => {
    const frameJob = source("frameJobSource");
    const winit = source("winitSource");
    for (const budget of laws.driveBudgets) expect(frameJob + winit, `the drive budget must name ${budget}`).toContain(budget);
    expect(frameJob).toContain("BROWSER_FRAME_BUILD_DRIVE_US: u64 = semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2");
  });
});
