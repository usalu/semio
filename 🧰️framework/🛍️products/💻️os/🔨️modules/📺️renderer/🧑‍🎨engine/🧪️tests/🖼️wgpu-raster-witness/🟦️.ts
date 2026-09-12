/**
 * 🖼️ The TypeScript twin of `🖱️ui/🧪️tests/🖼️raster-witness-lifecycle/🦀️.rs`. Both read the SAME neutral
 * fixture (`🖱️ui/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json`): the Rust law drives the production
 * witness ledger and proves its admission is total and its retirement bounded; this one proves the
 * ledger is the ONLY owner of that fact and that the renderer's present cursor mints, consumes and
 * commits exactly one operation witness per frame.
 *
 * Source-scanning is the honest oracle for the wiring half: the raster authority only ever runs behind
 * a real `wgpu::Device` inside a `wasm32` frame worker, which no unit harness can construct, and the
 * sources ARE the contract (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-raster-witness-effects-2026-09-12.md`).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import laws from "../../🧫️fixtures/🖼️wgpu-raster-witness/🔣️.json";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");

const fixture = JSON.parse(readFileSync(resolve(suiteRoot, laws.fixture), "utf8")) as {
  readonly witness: { readonly fields: readonly string[]; readonly retiredOneFieldPerStep: boolean; readonly readableOnlyWhenWhole: boolean };
  readonly operations: Record<string, { readonly sceneRevision: number; readonly previewGeneration: number; readonly operation: number }>;
  readonly admission: {
    readonly verdicts: readonly string[];
    readonly occupied: string;
    readonly rules: readonly { readonly name: string; readonly verdict?: string; readonly fault?: string; readonly candidate: string | null; readonly presenting: string | null; readonly commit?: string }[];
  };
  readonly presentation: { readonly armsOnlyItsOwnCandidate: string; readonly refusesASecondArming: string };
  readonly retirement: {
    readonly unitsPerStep: number;
    readonly slotsInOrder: readonly string[];
    readonly stepsPerOccupiedSlot: number;
    readonly stepsPerEmptySlot: number;
    readonly terminalStep: number;
    readonly expected: { readonly reservedAndPresented: number; readonly reservedOnly: number };
    readonly readmissionMustStayAdmissibleAtEveryStep: boolean;
    readonly ledgerIsReusableAfterTerminal: boolean;
  };
  readonly production: Record<string, string | readonly string[]>;
};

const source = (key: "drawSource" | "gpuSource" | "rendererSource" | "engineCanvasSource" | "rustLaw") => readFileSync(resolve(engineRoot, laws[key]), "utf8");
const text = (key: keyof typeof fixture.production) => fixture.production[key] as string;

describe("wgpu raster operation witness lifecycle", () => {
  it("keeps the fixture's own step arithmetic self-consistent", () => {
    const { stepsPerOccupiedSlot, stepsPerEmptySlot, terminalStep, expected, slotsInOrder } = fixture.retirement;
    expect(fixture.retirement.unitsPerStep).toBe(1);
    expect(slotsInOrder).toEqual(["candidate", "presenting"]);
    expect(stepsPerOccupiedSlot).toBe(fixture.witness.fields.length + 1);
    expect(stepsPerEmptySlot).toBe(1);
    expect(expected.reservedAndPresented).toBe(stepsPerOccupiedSlot * slotsInOrder.length + terminalStep);
    expect(expected.reservedOnly).toBe(stepsPerOccupiedSlot + stepsPerEmptySlot + terminalStep);
  });

  it("names a distinct operation for every fixture rule it compares", () => {
    const operations = Object.values(fixture.operations).map((value) => `${value.sceneRevision}/${value.previewGeneration}/${value.operation}`);
    expect(new Set(operations).size).toBe(operations.length);
    for (const rule of fixture.admission.rules) {
      for (const name of [rule.candidate, rule.presenting, rule.commit]) {
        if (name !== null && name !== undefined) expect(fixture.operations[name], `rule "${rule.name}" names a real operation`).toBeDefined();
      }
      expect(rule.verdict !== undefined || rule.fault !== undefined, `rule "${rule.name}" states an outcome`).toBe(true);
      if (rule.verdict !== undefined) expect(fixture.admission.verdicts).toContain(rule.verdict);
    }
  });

  it("puts the witness pair behind ONE device-free ledger, so the law can drive production", () => {
    const draw = source("drawSource");
    expect(draw).toContain(`pub struct ${text("ledger")} {`);
    const ledger = draw.slice(draw.indexOf(`pub struct ${text("ledger")} {`));
    for (const entry of ["admissionEntry", "retireEntry", "beginEntry", "armEntry", "candidateEntry"] as const) expect(ledger, `${text(entry)} is on the ledger`).toContain(`fn ${text(entry)}(`);
    const table = draw.slice(draw.indexOf(`pub struct ${text("table")} {`), draw.indexOf(`impl ${text("table")} {`));
    expect(table).toContain("witnesses: RasterOperationWitnessLedger,");
    expect(table, "the table no longer owns the two slots directly").not.toContain("candidate: RasterTextureWitnessSlot,");
    expect(table).not.toContain("presenting: RasterTextureWitnessSlot,");
  });

  it("keeps the retired-field bookkeeping off the staged-scan cursor", () => {
    const draw = source("drawSource");
    const cursor = draw.slice(draw.indexOf("struct RasterTextureRetirementCursor {"), draw.indexOf("pub struct RasterTextureTable {"));
    for (const field of fixture.production.retiredFieldsAreNotOnTheScanCursor as readonly string[]) expect(cursor, `${field} belongs to the ledger, not the scan`).not.toContain(field);
    expect(cursor).toContain("scan: usize,");
  });

  it("admits a retirement once and then answers its owner, never 'stale'", () => {
    const draw = source("drawSource");
    const admission = draw.slice(draw.indexOf(`pub fn ${text("admissionEntry")}(`), draw.indexOf(`pub fn ${text("beginEntry")}(`));
    expect(admission, "the in-flight owner is consulted before the raw slots").toContain(`if let Some(retiring) = self.${text("inFlightOwner")}`);
    expect(admission).toContain(fixture.admission.occupied);
    expect(admission.indexOf(text("inFlightOwner"))).toBeLessThan(admission.indexOf("candidate.is_empty()"));
    expect(admission, "an unreserved surface is 'not yet'").toContain("RasterWitnessAdmission::Nothing");
  });

  it("routes both the commit and the abort through that one admission", () => {
    const draw = source("drawSource");
    for (const entry of ["commitEntry", "abortEntry"] as const) {
      const body = draw.slice(draw.indexOf(`pub fn ${text(entry)}(`));
      const head = body.slice(0, body.indexOf("\n    }\n"));
      expect(head, `${text(entry)} goes through the ledger`).toContain(`self.witnesses.${text("beginEntry")}(witness)?`);
      expect(head, `${text(entry)} treats an empty ledger as 'not yet'`).toContain("RasterWitnessAdmission::Nothing");
      expect(head, `${text(entry)} no longer re-reads the slots itself`).not.toContain("self.candidate.get()");
      expect(head).not.toContain("self.presenting.get()");
    }
  });

  it("no longer re-validates the presentation witness inside the retirement it owns", () => {
    const draw = source("drawSource").slice(source("drawSource").indexOf(`impl ${text("table")} {`));
    const step = draw.slice(draw.indexOf("fn retirement_step(&mut self)"));
    const body = step.slice(0, step.indexOf("\n    }\n"));
    expect(body).toContain(`self.witnesses.${text("retireEntry")}()`);
    expect(body, "the per-step presentation re-read is what refused its own progress").not.toContain("raster retirement presentation witness was stale");
  });

  it("mints one operation witness per frame and commits that same one", () => {
    const renderer = source("rendererSource");
    const order = fixture.production.framePhaseOrder as readonly string[];
    const phases = renderer.slice(renderer.indexOf("enum AppPresentPhase {"), renderer.indexOf("struct AppPresentCursor {"));
    for (const phase of order) expect(phases, `${phase} is a real present phase`).toContain(`    ${phase},`);
    const mint = renderer.indexOf(`AppPresentPhase::${text("mintPhase")} => {`);
    const mintBody = renderer.slice(mint, renderer.indexOf(`AppPresentPhase::${text("consumePhase")};`, mint));
    expect(mintBody).toContain(`self.raster_operation_authority.${text("authorityBegin")}(expected.scene_revision, expected.input_generation)`);
    const commit = renderer.indexOf(`AppPresentPhase::${text("commitPhase")} => {`);
    const commitBody = renderer.slice(commit, renderer.indexOf("AppPresentPhase::ProgressAcknowledge;", commit));
    expect(commitBody).toContain(`self.raster_operation_authority.${text("authorityRelease")}(raster_witness)`);
    expect(commitBody).toContain("AppPresentedRetirement::commit(replacement.take_previous(), raster_witness)");
  });

  it("keeps the refusal able to name the three facts it compared", () => {
    const gpu = source("gpuSource");
    expect(gpu).toContain("presentation_witnesses()");
    expect(gpu).toMatch(/candidate \{candidate:\?\}, presenting \{presenting:\?\}, staged \{staged\}/);
  });

  it("feeds the engine surface's raster through the same candidate the frame minted", () => {
    const canvas = source("engineCanvasSource");
    expect(canvas).toContain("gpu.reserve_engine_texture(key, build.width, build.height, candidate_generation, expected)");
    expect(canvas).toContain('return Err("engine raster operation authority was stale before realization".to_string());');
  });

  it("shares its fixture with the Rust law, byte for byte", () => {
    expect(source("rustLaw")).toContain('include_str!("../../🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json")');
  });
});
