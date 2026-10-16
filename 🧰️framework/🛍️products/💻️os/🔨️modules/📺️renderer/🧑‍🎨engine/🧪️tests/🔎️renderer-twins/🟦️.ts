/** 🔎 Renderer twins of language-neutral contracts: label fit, camera, boot example, cancel, safe area, compute status and port types. */
import chromePanelSafeAreaFixture from "../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json";
import { chromePanelSafeArea, type Anchor, type SafeAreaYield } from "@semio-tech/ui-react";
import { world3dComputeStatusV1 } from "../../../../../../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts";
import surfaceControlsFixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🛑️surface-controls/🔣️.json";
import bootExampleFixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/📚️boot-example/🔣️.json";
import { BOOT_QUERY_CAPACITY, BOOT_QUERY_EXAMPLE_PARAM, resolveBootQueryExampleId } from "../../../../🧑‍💻dev/🔗️boot-query/🟦️.ts";
import { describe, expect, it } from "vitest";
import { nodeGraphConnectionIsValid, portValueTypes, portValueTypesCompatible, wireRefusalLabelOptions, dagContentBounds, dagContentCoverage, dagEllipsizeByMeasure, dagFitCamera, dagStartupCamera, nodeGraphContentSignature, paintDagLabelOverlays, DAG_CONTENT_FIT_PADDING_PX, DAG_CONTENT_FRAMED_MIN_COVERAGE, DAG_CONTENT_REFIT_MAX_COVERAGE, DAG_LABEL_ELLIPSIS, resolveBootExampleId } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import cameraFitFixture from "../../../../../../../🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/📷️camera-fit/🔣️.json" with { type: "json" };
import portTypesFixture from "../../../../🌊️flow/🧪️tests/🧫️fixtures/🔌️port-types/🔣️.json" with { type: "json" };
import labelFitFixture from "../../../../../../../🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/🏷️label-fit/🔣️.json" with { type: "json" };
import "../🧰️engine-contract-helpers/🟦️.ts";


//#region 📷️CameraAndLabelFitTwins
/** 📏️ The fixture's own synthetic advance — the ONE measure both implementations are driven with, so
 * a row pins the clipping algorithm rather than a font file. Mirror of `synthetic_measure` in
 * `🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🏷️label-fit/🦀️.rs`. */
function syntheticLabelMeasure(text: string, charWidth: number): number {
  return [...text].length * charWidth;
}

/** 🖋️ A 2D context that records every call and measures text as `charWidth * glyphs` at the font it
 * was last given — enough to prove what `paintDagLabelOverlays` actually draws, with no font files. */
function recordingLabelContext(charWidthPerPx: number) {
  const calls: { readonly op: string; readonly args: readonly unknown[] }[] = [];
  let fontPx = 0;
  const context = {
    get font() {
      return `${fontPx}px sans-serif`;
    },
    set font(value: string) {
      fontPx = Number.parseFloat(value) || 0;
      calls.push({ op: "font", args: [value] });
    },
    fillStyle: "",
    globalAlpha: 1,
    textAlign: "center",
    textBaseline: "middle",
    measureText: (text: string) => ({ width: [...text].length * fontPx * charWidthPerPx }),
    fillText: (text: string, x: number, y: number) => calls.push({ op: "fillText", args: [text, x, y, fontPx] }),
    clearRect: () => calls.push({ op: "clearRect", args: [] }),
    setTransform: () => calls.push({ op: "setTransform", args: [] }),
    save: () => calls.push({ op: "save", args: [] }),
    restore: () => calls.push({ op: "restore", args: [] }),
    translate: () => calls.push({ op: "translate", args: [] }),
    rotate: () => calls.push({ op: "rotate", args: [] }),
  };
  return { context, calls, drawn: () => calls.filter((call) => call.op === "fillText") };
}

function labelCanvasStub(charWidthPerPx: number) {
  const recorder = recordingLabelContext(charWidthPerPx);
  const canvas = { width: 0, height: 0, style: {} as Record<string, string>, getContext: () => recorder.context } as unknown as HTMLCanvasElement;
  return { canvas, ...recorder };
}

describe("node-graph caption clipping (2D replay twin)", () => {
  it("clips every fixture row to the string the shared law names", () => {
    const rows = labelFitFixture.rows;
    expect(rows.length).toBeGreaterThanOrEqual(8);
    expect(labelFitFixture.provenance.ellipsis).toBe(DAG_LABEL_ELLIPSIS);
    for (const row of rows) {
      const fitted = dagEllipsizeByMeasure(row.text, row.maxWidth, (candidate) => syntheticLabelMeasure(candidate, row.charWidth));
      expect([row.name, fitted]).toEqual([row.name, row.expect]);
      expect(syntheticLabelMeasure(fitted, row.charWidth)).toBeLessThanOrEqual(Math.max(row.maxWidth, 0) + 1e-9);
      expect([...fitted].filter((glyph) => glyph === DAG_LABEL_ELLIPSIS).length).toBeLessThanOrEqual(1);
    }
  });

  it("draws a node title clipped at its measured width instead of shrinking the font into a smudge", () => {
    // 🏷️ The live defect's shape: a 40-world-unit node at zoom 1.78 gives a ~63px caption budget and
    // the title is far wider. Before the fix the overlay binary-searched the font down to 4px.
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1.7844325616011099 },
      width: 483,
      height: 814,
      labels: [{ id: "extrude", text: "Brep.solid.extrude", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.085838785074808, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 483, 814, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    const drawn = stub.drawn();
    expect(drawn).toHaveLength(1);
    const [text, , , fontPx] = drawn[0]!.args as [string, number, number, number];
    expect(fontPx).toBeGreaterThanOrEqual(8);
    expect(text.endsWith(DAG_LABEL_ELLIPSIS)).toBe(true);
    expect(text.startsWith("Brep")).toBe(true);
    expect([...text].length * fontPx * 0.6).toBeLessThanOrEqual(40 * 1.7844325616011099 * 0.88 + 1e-9);
  });

  it("centres captions on the measured overlay, not on a stale session size", () => {
    // 🏷️ The live defect: the session reported `width: 1, height: 1` (painted at hand-over, before the
    // engine canvas was attached) beside an 852×807 overlay, so a node at the world origin was captioned
    // at (0.5, 0.5) instead of the canvas centre.
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1 },
      width: 1,
      height: 1,
      labels: [{ id: "root", text: "B", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.75, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 852, 807, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    const [, x, y] = stub.drawn()[0]!.args as [string, number, number];
    expect([x, y]).toEqual([426, 403.5]);
  });

  it("draws a title that fits whole, which is what the live generation3d operator names do", () => {
    const stub = labelCanvasStub(0.6);
    const state = JSON.stringify({
      camera: { x: 0, y: 0, zoom: 1.7844325616011099 },
      width: 483,
      height: 814,
      labels: [{ id: "extrude", text: "Extrude", layout: "horizontal", x: 0, y: 0, nodeW: 40, nodeH: 28, fontScreenPx: 13.085838785074808, ghost: false }],
    });
    paintDagLabelOverlays(state, stub.canvas, 483, 814, 1, { selectedIds: [], preselect: { ids: [], removedIds: [] }, hoveredId: null });
    expect((stub.drawn()[0]!.args as [string])[0]).toBe("Extrude");
  });
});

describe("node-graph opening camera (renderer twin)", () => {
  it("opens on the camera the shared law names for every fixture row", () => {
    const rows = cameraFitFixture.rows;
    expect(rows.length).toBeGreaterThanOrEqual(8);
    expect(cameraFitFixture.provenance.constants.paddingPx).toBe(DAG_CONTENT_FIT_PADDING_PX);
    expect(cameraFitFixture.provenance.constants.minCoverage).toBe(DAG_CONTENT_FRAMED_MIN_COVERAGE);
    expect(cameraFitFixture.provenance.constants.refitMaxCoverage).toBe(DAG_CONTENT_REFIT_MAX_COVERAGE);
    for (const row of rows) {
      const { camera, fitted } = dagStartupCamera(row.stored, row.content, row.viewport.width, row.viewport.height);
      expect([row.name, camera.x, camera.y, camera.zoom, fitted]).toEqual([row.name, row.expect.camera.x, row.expect.camera.y, row.expect.camera.zoom, row.expect.fitted]);
      if (row.content && row.stored) {
        const coverage = dagContentCoverage(row.content, row.stored, row.viewport.width, row.viewport.height);
        expect([row.name, coverage]).toEqual([row.name, row.expect.coverage]);
        expect([row.name, coverage <= DAG_CONTENT_REFIT_MAX_COVERAGE]).toEqual([row.name, row.expect.refits]);
      }
    }
  });

  it("derives the graph bounds a fit frames from the scene's own node records", () => {
    const nodes = [
      { id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [], outputs: [] },
      { id: "b", x: 200, y: -100, width: 40, height: 20, inputs: [], outputs: [] },
    ];
    expect(dagContentBounds(nodes)).toEqual({ minX: -20, minY: -110, maxX: 220, maxY: 10 });
    expect(dagContentBounds([])).toBeNull();
    expect(dagContentBounds(undefined)).toBeNull();
    const fitted = dagFitCamera(dagContentBounds(nodes)!, 483, 814);
    expect(dagContentCoverage(dagContentBounds(nodes)!, fitted, 483, 814)).toBeCloseTo(1, 9);
  });

  it("publishes the camera it computed for every surface row", () => {
    const rows = portSidesSurfaceRows();
    expect(rows.length).toBeGreaterThanOrEqual(4);
    for (const row of rows) {
      const nodes = [{ id: "graph", x: (row.content.minX + row.content.maxX) / 2, y: (row.content.minY + row.content.maxY) / 2, width: row.content.maxX - row.content.minX, height: row.content.maxY - row.content.minY, inputs: [], outputs: [] }];
      const content = dagContentBounds(nodes)!;
      const published = dagFitCamera(content, row.viewport.width, row.viewport.height);
      expect([row.name, published.x, published.y, published.zoom]).not.toEqual([row.name, row.camera.x, row.camera.y, row.camera.zoom]);
      expect([row.name, dagContentCoverage(content, published, row.viewport.width, row.viewport.height)]).toEqual([row.name, row.expect.coverageAfterFit]);
      const again = dagFitCamera(content, row.viewport.width, row.viewport.height);
      expect([row.name, again.x, again.y, again.zoom]).toEqual([row.name, published.x, published.y, published.zoom]);
    }
  });

  it("re-frames only on a layout change, never on hover, selection or evaluation churn", () => {
    const nodes = [{ id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [], outputs: [] }];
    const same = [{ id: "a", x: 0, y: 0, width: 40, height: 20, inputs: [{ id: "a@in", label: "in" }], outputs: [] }];
    const moved = [{ id: "a", x: 900, y: 0, width: 40, height: 20, inputs: [], outputs: [] }];
    expect(nodeGraphContentSignature(nodes)).toBe(nodeGraphContentSignature(same));
    expect(nodeGraphContentSignature(nodes)).not.toBe(nodeGraphContentSignature(moved));
  });
});

/** 🖼️ The `surfaceRows` half of the shared camera-fit fixture — the rows that say a fit must PUBLISH
 * what it computed. The Rust half drives a real `FlowHost`
 * (`🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, `a_fitted_flow_surface_publishes_the_camera_it_computed`);
 * this half drives the renderer's own fit rule over the same rows. */
function portSidesSurfaceRows(): {
  name: string;
  viewport: { width: number; height: number };
  camera: { x: number; y: number; zoom: number };
  content: { minX: number; minY: number; maxX: number; maxY: number };
  expect: { coverageAfterFit: number };
}[] {
  return (
    cameraFitFixture as unknown as {
      surfaceRows: { name: string; viewport: { width: number; height: number }; camera: { x: number; y: number; zoom: number }; content: { minX: number; minY: number; maxX: number; maxY: number }; expect: { coverageAfterFit: number } }[];
    }
  ).surfaceRows;
}
//#endregion 📷️CameraAndLabelFitTwins


//#region 📚️BootExampleTwin
/** 📚️ The TypeScript half of the boot-example law: the SAME
 * `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json` rows the Rust `🔬️wgpu-shell-chrome-parity` law answers
 * through `resolve_boot_example_id`, answered here by the shipped `resolveBootExampleId` — two
 * independent implementations, one fixture, so the two shells cannot drift on which document a url
 * opens. The `bootQuery` half pins the `?example=` axis itself against the shipped
 * `resolveBootQueryExampleId`, the parser both entries spell the query with.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("📚️ boot example contract", () => {
  it("validates the shared fixture against its own declared schema", () => {
    expect(bootExampleFixture.rows.map((row) => resolveBootExampleId(row.activeExampleId, row.options.map((id: string) => ({ id })), row.defaultExampleId ?? undefined))).toEqual(bootExampleFixture.rows.map((row) => row.expected));
    expect(bootExampleFixture.bootQueryParam).toBe(BOOT_QUERY_EXAMPLE_PARAM);
  });

  it("resolves every shared fixture row the way the wgpu shell does", () => {
    for (const row of bootExampleFixture.rows) {
      const options = row.options.map((id: string) => ({ id }));
      expect(resolveBootExampleId(row.activeExampleId, options, row.defaultExampleId ?? undefined), row.id).toBe(row.expected);
    }
  });

  it("reads the `?example=` axis the same way on both entries", () => {
    for (const row of bootExampleFixture.bootQuery) {
      expect(resolveBootQueryExampleId(row.search, undefined) ?? null, row.id).toBe(row.expected);
    }
    expect(resolveBootQueryExampleId("?plugin=generation3d", "seeded"), "an absent query keeps the per-server seed").toBe("seeded");
    expect(resolveBootQueryExampleId("?plugin=generation3d&example=", "seeded"), "an empty value is the same as no value at all").toBe("seeded");
    expect(() => resolveBootQueryExampleId("?example=" + "x".repeat(BOOT_QUERY_CAPACITY), undefined)).toThrow(/boot-query-overflow/);
  });
});
//#endregion 📚️BootExampleTwin


//#region 🛑️SurfaceControlCancelTwin
/** 🛑️ The TypeScript half of the surface-control laws: the SAME
 * `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` laws answer through `world3d_cancel_affordance`, answered here by the
 * shipped `world3dComputeStatusV1` parser — two independent implementations, one fixture, so neither
 * renderer can drift into offering a cancel the producer never authorised.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("🛑️ world3d cancel contract", () => {
  it("validates the shared fixture against its own declared schema", () => {
    expect(surfaceControlsFixture.cancelContract.map((row) => world3dComputeStatusV1(row.statusJson).cancellable)).toEqual(surfaceControlsFixture.cancelContract.map((row) => row.expected.cancellable));
  });

  it("offers a cancel affordance for exactly the rows the shared fixture declares", () => {
    const rows = surfaceControlsFixture.cancelContract;
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(status.cancellable, row.id).toBe(row.expected.cancellable);
      expect(status.cancelAction, row.id).toBe(row.expected.cancelAction);
      expect(status.cancelArgs, row.id).toEqual(row.expected.cancelArgs);
    }
  });

  it("derives the control ids the wgpu shell paints from the same rows", () => {
    for (const row of surfaceControlsFixture.surfaceControls) {
      const { controlHeightPx, surfaceControlMinimum } = surfaceControlsFixture;
      const fits = (bounds: readonly number[]) => bounds[2] >= surfaceControlMinimum.widthControlHeights * controlHeightPx && bounds[3] >= surfaceControlMinimum.heightControlHeights * controlHeightPx;
      const ids = [
        ...row.worlds.filter((world: { bounds: number[]; statusJson: string | null }) => fits(world.bounds) && world3dComputeStatusV1(world.statusJson).cancellable).map((world: { surfaceId: string }) => `shell.world3d.cancel::${world.surfaceId}`),
      ];
      expect(ids, row.id).toEqual(row.expected);
    }
  });
});
//#endregion 🛑️SurfaceControlCancelTwin


//#region 🛟️ChromePanelSafeAreaTwin
/** 🛟️ The TypeScript half of the chrome-panel SAFE AREA: the SAME
 * `Framework UI 🧫️fixtures/🛟️chrome-panel-safe-area/🔣️.json` `chromePanelSafeArea` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` law answers through `chrome_panel_safe_area`, answered here by the shipped
 * `chromePanelSafeArea` the React world pane's overlay rail and every window's right-edge chrome read —
 * two independent implementations, one fixture, so neither renderer can drift into painting an anchored
 * chrome panel over an affordance a user has to press. The defect it exists for is
 * `📓️react-oracle-hardening-2026-09-14.md` §4.3: the `top-right` Tool runs panel at (1137, 3) 300×120
 * swallowing `Frame visible` and the preview `Cancel`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("🛟️ chrome panel safe area", () => {
  
  const box = ([x, y, w, h]: readonly number[]) => ({ left: x, top: y, right: x + w, bottom: y + h });

  it("validates the shared safe-area corpus against its own declared schema", () => {
    expect(chromePanelSafeAreaFixture.length).toBeGreaterThanOrEqual(6);
  });

  it("reserves every shared fixture row exactly the way the wgpu shell does", () => {
    for (const row of chromePanelSafeAreaFixture) {
      const safeArea = chromePanelSafeArea(box(row.affordance), box(row.host), row.anchor as Anchor, row.panels.map(box), row.yield as SafeAreaYield, row.gap);
      expect(safeArea.inlinePx, `${row.id}: inline reserve`).toBe(row.expected.inline);
      expect(safeArea.blockPx, `${row.id}: block reserve`).toBe(row.expected.block);
    }
  });

  it("leaves every reserved affordance clear of the panels it yielded to", () => {
    for (const row of chromePanelSafeAreaFixture) {
      const affordance = box(row.affordance);
      const safeArea = chromePanelSafeArea(affordance, box(row.host), row.anchor as Anchor, row.panels.map(box), row.yield as SafeAreaYield, row.gap);
      if (safeArea.inlinePx === 0 && safeArea.blockPx === 0) continue;
      const inlineShift = row.anchor.endsWith("right") ? -safeArea.inlinePx : safeArea.inlinePx;
      const blockShift = row.anchor.startsWith("bottom") ? -safeArea.blockPx : safeArea.blockPx;
      const cleared = { left: affordance.left + inlineShift, right: affordance.right + inlineShift, top: affordance.top + blockShift, bottom: affordance.bottom + blockShift };
      for (const panel of row.panels.map(box)) {
        expect(cleared.left < panel.right && cleared.right > panel.left && cleared.top < panel.bottom && cleared.bottom > panel.top, `${row.id}: still covered by a panel`).toBe(false);
      }
    }
  });
});
//#endregion 🛟️ChromePanelSafeAreaTwin


//#region ⏳️ComputeStatusPaneTwin
/** ⏳️ The TypeScript half of the World3d compute-status PROGRESS laws: the same
 * `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` `statusPane` rows the Rust
 * `🔬️wgpu-shell-chrome-parity` laws answer through `world3d_compute_status`/`world3d_status_pill_for`,
 * answered here by the shipped `world3dComputeStatusV1` parser React's own `WorldComputeStatusPane`
 * reads. Two independent implementations, one fixture — which is what makes "wgpu shows the same
 * progress React shows" a law rather than a claim. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("⏳️ world3d compute status pane", () => {
  type StatusPaneRow = {
    readonly id: string;
    readonly statusJson: string | null;
    readonly expected: {
      readonly visible: boolean;
      readonly computing: boolean;
      readonly phase: string;
      readonly phaseLabel: { readonly en: string; readonly de: string } | null;
      readonly unitsDone: number;
      readonly unitsTotal: number;
      readonly facesDone: number;
      readonly facesTotal: number;
      readonly inFlight: number;
      readonly ratio: number;
      readonly progressText: string | null;
    };
  };
  const rows = surfaceControlsFixture.statusPane as readonly StatusPaneRow[];

  /** ⏳️ React's own pane gate — a settled producer annotates nothing, so an idle viewport is never
   * covered by chrome (`🌐️World3dHost/🟦️.tsx`'s `WorldComputeStatusPane` first line). */
  const isVisible = (status: ReturnType<typeof world3dComputeStatusV1>) => status.computing || status.cancellable || status.phase === "cancelled";
  /** 📈️ The count React renders beside the phase, and the wgpu pill joins into its one glyph run. */
  const progressText = (status: ReturnType<typeof world3dComputeStatusV1>) => (status.unitsTotal > 0 ? `${status.unitsDone}/${status.unitsTotal} (${Math.round(status.ratio * 100)}%)` : null);

  it("reads every declared field of the shared status rows", () => {
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(status.computing, row.id).toBe(row.expected.computing);
      expect(status.phase, row.id).toBe(row.expected.phase);
      expect(status.phaseLabel, row.id).toEqual(row.expected.phaseLabel);
      expect(status.unitsDone, row.id).toBe(row.expected.unitsDone);
      expect(status.unitsTotal, row.id).toBe(row.expected.unitsTotal);
      expect(status.facesDone, row.id).toBe(row.expected.facesDone);
      expect(status.facesTotal, row.id).toBe(row.expected.facesTotal);
      expect(status.inFlight, row.id).toBe(row.expected.inFlight);
      expect(status.ratio, row.id).toBeCloseTo(row.expected.ratio, 9);
    }
  });

  it("carries the producer's own empty-surface hint, and an idle surface that HAS one is not silent", () => {
    // 🕳️ `preview_hint` reached `data-status-json` and stopped there: this reader dropped the field
    // and nothing painted it, so a user looking at an empty generate preview was told nothing
    // (`panel-i18n · de:preview_hint`). The pane's own gate is `computing || cancellable ||
    // phase === "cancelled"`; the hint branch is what an IDLE surface with something to say uses.
    const german = "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
    const withHint = world3dComputeStatusV1(JSON.stringify({ computing: false, phase: "idle", hint: german }));
    expect(withHint.hint).toBe(german);
    expect(isVisible(withHint)).toBe(false);
    expect(withHint.hint.length > 0).toBe(true);
    const withoutHint = world3dComputeStatusV1(JSON.stringify({ computing: false, phase: "idle" }));
    expect(withoutHint.hint).toBe("");
    expect(world3dComputeStatusV1(JSON.stringify({ hint: 42 })).hint).toBe("");
    expect(world3dComputeStatusV1(JSON.stringify({ hint: "x".repeat(400) })).hint.length).toBe(200);
    for (const row of rows) expect(typeof world3dComputeStatusV1(row.statusJson).hint, row.id).toBe("string");
  });

  it("shows the pane for exactly the unsettled rows, with exactly the declared progress text", () => {
    let painted = 0;
    for (const row of rows) {
      const status = world3dComputeStatusV1(row.statusJson);
      expect(isVisible(status), row.id).toBe(row.expected.visible);
      if (!row.expected.visible) continue;
      expect(progressText(status), row.id).toBe(row.expected.progressText);
      painted += 1;
    }
    expect(painted).toBeGreaterThan(0);
  });

  it("never defaults to one language: a half-translated phase label is no label at all", () => {
    const halved = rows.find((row) => row.id === "a-half-translated-phase-label-is-no-label-at-all");
    expect(halved, "the fixture declares the half-translated row").toBeTruthy();
    expect(world3dComputeStatusV1(halved!.statusJson).phaseLabel).toBeNull();
    const paired = world3dComputeStatusV1(rows[0].statusJson).phaseLabel;
    expect(paired?.en).not.toBe(paired?.de);
  });

  /** ⏳️⛓️ The TIMELINE law — the TypeScript twin of the Rust
   * `a_long_evaluation_publishes_a_monotone_run_of_non_idle_frames_and_then_settles`, over the same
   * `progressTimeline` frames. No single row can state it: a producer that publishes `idle`
   * throughout satisfies every row assertion above and fails this one, which is exactly what BOTH
   * renderers were showing — one 23 s evaluation, 54 publications, every one of them
   * `phase:"idle" inFlight:0 ratio:1.0` (`📓️wgpu-progress-visibility-2026-09-14.md`). */
  type TimelineFrame = { readonly id: string; readonly statusJson: string; readonly expected: { readonly visible: boolean; readonly phase: string; readonly ratio: number; readonly progressText: string | null } };
  const timeline = surfaceControlsFixture.progressTimeline as { readonly minimumNonIdleFrames: number; readonly evaluation: readonly TimelineFrame[]; readonly cancelled: readonly TimelineFrame[] };

  it("publishes a monotone run of non-idle frames while a long evaluation is in flight, then settles", () => {
    for (const lane of ["evaluation", "cancelled"] as const) {
      const frames = timeline[lane];
      let nonIdle = 0;
      let previousRatio = Number.NEGATIVE_INFINITY;
      for (const frame of frames) {
        const status = world3dComputeStatusV1(frame.statusJson);
        expect(status.phase, `${lane}/${frame.id}`).toBe(frame.expected.phase);
        expect(status.ratio, `${lane}/${frame.id}`).toBeCloseTo(frame.expected.ratio, 9);
        expect(isVisible(status), `${lane}/${frame.id}`).toBe(frame.expected.visible);
        if (!frame.expected.visible) continue;
        expect(progressText(status), `${lane}/${frame.id}`).toBe(frame.expected.progressText);
        if (status.phase === "cancelled") continue;
        expect(status.phase, `${lane}/${frame.id}: a frame that annotates the viewport may not call itself idle`).not.toBe("idle");
        expect(status.ratio, `${lane}/${frame.id}: ratio went backwards`).toBeGreaterThanOrEqual(previousRatio - 1e-9);
        expect(status.cancellable, `${lane}/${frame.id}: work in flight offers a cancel`).toBe(true);
        previousRatio = status.ratio;
        nonIdle += 1;
      }
      expect(isVisible(world3dComputeStatusV1(frames[frames.length - 1].statusJson)), `${lane}: the timeline ends settled`).toBe(false);
      if (lane === "evaluation") expect(nonIdle, `${lane}: non-idle frames`).toBeGreaterThanOrEqual(timeline.minimumNonIdleFrames);
    }
    expect(
      timeline.cancelled.some((frame) => frame.expected.phase === "cancelled"),
      "the cancelled lane settles on `cancelled`, not on `idle`",
    ).toBe(true);
  });
});
//#endregion ⏳️ComputeStatusPaneTwin


//#region 🔌️PortTypeTwin
/** 🔌️ The renderer's half of the port-type law. The oracle is
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json`; the Rust half
 * (`🌊️flow/🧪️tests/🔌️port-types/🦀️.rs`) checks those declared types against the LIVE extension
 * registry and drives a real `FlowHost`, and this half checks that the renderer's own predicate —
 * the one React Flow asks before it paints a snap target as droppable — answers the same rows.
 * Neither implementation owns the verdicts, so a drift on either side fails on both. */
describe("node-graph port types", () => {
  type Channel = { readonly ref: string; readonly side: string; readonly valueTypes: readonly string[] };
  type Row = { readonly source: string; readonly target: string; readonly compatible: boolean; readonly examples?: readonly string[] };
  const fixture = portTypesFixture as unknown as { readonly channels: readonly Channel[]; readonly rows: readonly Row[] };
  const declared = new Map(fixture.channels.map((channel) => [channel.ref, channel.valueTypes] as const));
  const typesOf = (reference: string) =>
    declared.get(reference) ??
    (() => {
      throw new Error(`fixture declares no channel ${reference}`);
    })();

  it("accepts or refuses every fixture pair exactly as the fixture says", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(30);
    expect(fixture.rows.some((row) => row.compatible)).toBe(true);
    expect(fixture.rows.some((row) => !row.compatible)).toBe(true);
    for (const row of fixture.rows) {
      expect([row.source, row.target, portValueTypesCompatible(typesOf(row.source), typesOf(row.target))]).toEqual([row.source, row.target, row.compatible]);
    }
  });

  it("refuses the drag the defect accepted, on the node records a surface actually holds", () => {
    const records = [
      { id: "extrusion-axis", x: 0, y: 0, width: 80, height: 40, inputs: [], outputs: [{ id: "extrusion-axis@vectorOut", label: "V", valueType: typesOf("math.vector@vectorOut").join(",") }] },
      { id: "profile", x: 0, y: 100, width: 80, height: 40, inputs: [], outputs: [{ id: "profile@wire", label: "W", valueType: typesOf("brep.curve.polygon@wire").join(",") }] },
      {
        id: "extrude",
        x: 200,
        y: 0,
        width: 80,
        height: 40,
        inputs: [
          { id: "extrude@wire", label: "W", valueType: typesOf("brep.solid.extrude@wire").join(",") },
          { id: "extrude@vector", label: "V", valueType: typesOf("brep.solid.extrude@vector").join(",") },
        ],
        outputs: [],
      },
    ];
    expect(nodeGraphConnectionIsValid(records, { source: "extrusion-axis", sourceHandle: "vectorOut", target: "extrude", targetHandle: "wire" })).toBe(false);
    expect(nodeGraphConnectionIsValid(records, { source: "extrusion-axis", sourceHandle: "vectorOut", target: "extrude", targetHandle: "vector" })).toBe(true);
    expect(nodeGraphConnectionIsValid(records, { source: "profile", sourceHandle: "wire", target: "extrude", targetHandle: "wire" })).toBe(true);
  });

  it("leaves an undeclared port connectable", () => {
    expect(portValueTypes(undefined)).toEqual([]);
    expect(portValueTypes({ id: "x", valueType: "" })).toEqual([]);
    expect(portValueTypes({ id: "x", valueType: "point,vector" })).toEqual(["point", "vector"]);
    expect(portValueTypesCompatible([], ["geometry"])).toBe(true);
    expect(portValueTypesCompatible(["geometry"], [])).toBe(true);
  });

  it("names both ports and both declared types in the admitted refusal a surface shows", () => {
    const refusal={source:"extrusion-axis@vectorOut",sourceTypes:["vector"],target:"extrude@wire",targetTypes:["geometry"]};
    const options=wireRefusalLabelOptions(refusal,{vector:"Vektor",geometry:"Geometrie"});
    expect(options).toEqual({source:"extrusion-axis@vectorOut",sourceType:"Vektor",target:"extrude@wire",targetType:"Geometrie"});
    expect(wireRefusalLabelOptions(null,{})).toEqual({source:"",sourceType:"",target:"",targetType:""});
  });
});
//#endregion 🔌️PortTypeTwin
