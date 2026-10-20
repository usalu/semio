import { parseViewport2d, type Viewport2d } from "@semio-tech/framework-ui-viewport-ts/2d/schema";
import { parseViewport3dOrbit, type Viewport3dOrbit } from "@semio-tech/framework-ui-viewport-ts/3d/schema";
import { FEM2D_DEMO_SNAPSHOT, FEM2D_STORY_EXAMPLE_ID, FEM3D_DEMO_SNAPSHOT, FEM3D_STORY_EXAMPLE_ID, femNextId, femStoryActionArgs, femStoryLabel, type FemStoryLocale } from "../🟦️.ts";
import { EMPTY_FEM2D_SNAPSHOT, EMPTY_FEM3D_SNAPSHOT, type Fem2dSnapshot, type Fem3dSnapshot } from "../📸️snapshot-projection/🟦️.ts";
import { FEM2D_DEFAULT_CAMERA, FEM3D_INITIAL_HOVER, FEM3D_INITIAL_SELECTION, FEM3D_INITIAL_VIEWPORT, FEM3D_INTERACTION_DOMAIN, FEM3D_SELECTION_SPEC, fem3dStoryTopology } from "../🎞️scenes/🟦️.ts";
import { type DomainHover, type DomainSelection, parseInteractionTarget, parseMergeMode } from "@semio-tech/framework-interaction/schema";
import { DEFAULT_HOVER_SPEC, nextHover, nextSelection } from "@semio-tech/framework-interaction";

//#region 🔖️ResultDisplay
/** 👁️ Result presentation owned by the current window and translated at the render boundary. */
export type FemStoryResultDisplay = { readonly sourceId: string | null; readonly mode: string; readonly modeIndex: number };

export const FEM_DEFAULT_RESULT_DISPLAY: FemStoryResultDisplay = { sourceId: null, mode: "static", modeIndex: 0 };

/** 📊️ The case id `render_static` resolves: the selected `sourceId` when the document knows it, else the first load case, else none. The Rust filters against `fem2d_solve_all`'s result keys; with no solver here the document's own case/combination ids stand in, which is the same set for a solvable document. */
export function femResolveResultCase(display: FemStoryResultDisplay, caseIds: readonly string[]): string | null {
  if (display.sourceId !== null && caseIds.includes(display.sourceId)) return display.sourceId;
  return caseIds[0] ?? null;
}

//#endregion 🔖️ResultDisplay

//#region 🔖️Fem2dReducer
export type Fem2dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem2dSnapshot;
  readonly camera: Viewport2d;
  readonly resultDisplay: FemStoryResultDisplay;
};

/** 🎬️ The state a fem2d session starts in for one example id — `"demo"` loads the bundled fixture, every other id an empty document, exactly as `setActiveExample`'s handler branches. */
export function fem2dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem2dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM2D_STORY_EXAMPLE_ID ? FEM2D_DEMO_SNAPSHOT : EMPTY_FEM2D_SNAPSHOT,
    camera: FEM2D_DEFAULT_CAMERA,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem2dStoryAction(state: Fem2dStoryState, action: string, rawArgs: unknown): Fem2dStoryState {
  const args = femStoryActionArgs(rawArgs);
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem2dStoryStateFor(exampleId, state.locale).snapshot };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y }] } };
    }
    case "setCamera": {
      return { ...state, camera: parseViewport2d({ x: args.x, y: args.y, zoom: args.zoom }) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}

//#endregion 🔖️Fem2dReducer

//#region 🔖️Fem3dReducer
export type Fem3dStoryState = {
  readonly exampleId: string;
  readonly locale: FemStoryLocale;
  readonly snapshot: Fem3dSnapshot;
  readonly camera: Viewport3dOrbit;
  readonly resultDisplay: FemStoryResultDisplay;
  readonly selection: DomainSelection;
  readonly hover: DomainHover;
};

/** 🎬️ Opens a story window with the registered example and explicit OS locale. */
export function fem3dStoryStateFor(exampleId: string, locale: FemStoryLocale): Fem3dStoryState {
  return {
    exampleId,
    locale,
    snapshot: exampleId === FEM3D_STORY_EXAMPLE_ID ? FEM3D_DEMO_SNAPSHOT : EMPTY_FEM3D_SNAPSHOT,
    camera: FEM3D_INITIAL_VIEWPORT,
    resultDisplay: FEM_DEFAULT_RESULT_DISPLAY,
    selection: FEM3D_INITIAL_SELECTION,
    hover: FEM3D_INITIAL_HOVER,
  };
}

/** 🧩️ Mirrors document commands and exact-window preferences while preserving the supplied OS locale. */
export function reduceFem3dStoryAction(state: Fem3dStoryState, action: string, rawArgs: unknown): Fem3dStoryState {
  const args = femStoryActionArgs(rawArgs);
  if (action === "interactionSelect" || action === "interactionHover") {
    if (args.domainId !== FEM3D_INTERACTION_DOMAIN) return state;
    const rows: unknown = typeof args.targets === "string" ? JSON.parse(args.targets) : args.targets;
    if (!Array.isArray(rows)) throw new TypeError("FEM interaction targets must be an array");
    const targets = rows.map((row) => parseInteractionTarget(row));
    const { topology, granularities } = fem3dStoryTopology(state.snapshot);
    if (targets.some((target) => granularities.get(target.id) !== target.granularity)) return state;
    if (action === "interactionHover") return { ...state, hover: nextHover(DEFAULT_HOVER_SPEC, topology, { channel: typeof args.channel === "string" ? args.channel : "pointer", targets }) };
    const merge = parseMergeMode(args.merge);
    if (!FEM3D_SELECTION_SPEC.merges.includes(merge)) return state;
    const selection = targets.length === 0 && merge === "replace" ? FEM3D_INITIAL_SELECTION : nextSelection(FEM3D_SELECTION_SPEC, state.selection, topology, { targets, merge, mode: "multiple" });
    return { ...state, selection };
  }
  switch (action) {
    case "setActiveExample": {
      const exampleId = typeof args.exampleId === "string" ? args.exampleId : undefined;
      return exampleId === undefined ? state : { ...state, exampleId, snapshot: fem3dStoryStateFor(exampleId, state.locale).snapshot, selection: FEM3D_INITIAL_SELECTION, hover: FEM3D_INITIAL_HOVER };
    }
    case "addNode": {
      const x = typeof args.x === "number" ? args.x : 0;
      const y = typeof args.y === "number" ? args.y : 0;
      const z = typeof args.z === "number" ? args.z : 0;
      const id = femNextId(
        state.snapshot.nodes.map((node) => node.id),
        "n",
      );
      return { ...state, snapshot: { ...state.snapshot, nodes: [...state.snapshot.nodes, { id, x, y, z }] } };
    }
    case "setCamera": {
      const camera = args.camera;
      return { ...state, camera: parseViewport3dOrbit(camera) };
    }
    case "setResultDisplay": {
      return {
        ...state,
        resultDisplay: {
          sourceId: typeof args.sourceId === "string" ? args.sourceId : null,
          mode: typeof args.mode === "string" ? args.mode : "static",
          modeIndex: typeof args.modeIndex === "number" ? args.modeIndex : 0,
        },
      };
    }
    default:
      return state;
  }
}

//#endregion 🔖️Fem3dReducer

//#region 🔖️RenderSummaries
/** 📋️ The bilingual document readout every fem2d story shows beside its canvas — the counts the window's own scene is built from, so the panel stays assertable even when a layer kind is empty. */
export function fem2dSummaryLines(snapshot: Fem2dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("regions", locale)}: ${snapshot.regions.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** 📋️ The fem3d counterpart, with `solids` in place of `regions`. */
export function fem3dSummaryLines(snapshot: Fem3dSnapshot, locale: FemStoryLocale): readonly string[] {
  return [
    `${femStoryLabel("nodes", locale)}: ${snapshot.nodes.length}`,
    `${femStoryLabel("members", locale)}: ${snapshot.elements.length}`,
    `${femStoryLabel("supports", locale)}: ${snapshot.supports.length}`,
    `${femStoryLabel("solids", locale)}: ${snapshot.solids.length}`,
    `${femStoryLabel("materials", locale)}: ${snapshot.materials.length}`,
    `${femStoryLabel("sections", locale)}: ${snapshot.sections.length}`,
    `${femStoryLabel("loadCases", locale)}: ${snapshot.loadCases.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("combinations", locale)}: ${snapshot.combinations.map((entry) => entry.id).join(", ") || "—"}`,
    `${femStoryLabel("deformationScale", locale)}: ${snapshot.analysis.deformationScale}`,
  ];
}

/** 📊️ The results-window caption line: `render_static`'s resolved case, or its `"No load case defined"` placeholder branch. */
export function femResultCaptionLine(caseId: string | null, display: FemStoryResultDisplay, locale: FemStoryLocale): string {
  if (caseId === null) return femStoryLabel("noLoadCase", locale);
  const suffix = display.mode === "static" ? "" : ` #${display.modeIndex}`;
  return `${femStoryLabel("activeCase", locale)}: ${caseId} — ${femStoryLabel("displayMode", locale)} ${display.mode}${suffix}`;
}

//#endregion 🔖️RenderSummaries
