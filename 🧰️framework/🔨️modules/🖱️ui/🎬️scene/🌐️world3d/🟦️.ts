import type { World3dAnnotationLayer, World3dScalarField, World3dModellingOptions } from "../📏️world3d-modelling/🟦️.ts";
import type { SceneLaneRef } from "../🚚️lane-contract/🟦️.ts";

export type SceneViewportMask3d = "rectangle" | "ellipse";

export type World3dPresentation = {
  readonly showGrid: boolean;
  readonly showGizmo: boolean;
  readonly interactive: boolean;
  readonly viewportMask: SceneViewportMask3d;
  readonly clear: "theme" | "transparent";
  readonly sourceAspect?: number;
};

export const DEFAULT_WORLD3D_PRESENTATION: World3dPresentation = {
  showGrid: true,
  showGizmo: true,
  interactive: true,
  viewportMask: "rectangle",
  clear: "theme",
};

/** 🎭️ Parses one complete world presentation lane, rejecting non-finite projection extents. */
export function parseWorld3dPresentation(json: string | null | undefined): World3dPresentation {
  if (json === undefined || json === null) return DEFAULT_WORLD3D_PRESENTATION;
  try {
    const value = JSON.parse(json) as Record<string, unknown>;
    if (value === null || Array.isArray(value) || typeof value !== "object") return DEFAULT_WORLD3D_PRESENTATION;
    const keys = Object.keys(value);
    if (keys.some((key) => !["showGrid", "showGizmo", "interactive", "viewportMask", "clear", "sourceAspect"].includes(key))) return DEFAULT_WORLD3D_PRESENTATION;
    const sourceAspect = value.sourceAspect;
    if (sourceAspect !== undefined && (typeof sourceAspect !== "number" || !Number.isFinite(sourceAspect) || sourceAspect <= 0)) return DEFAULT_WORLD3D_PRESENTATION;
    if (value.showGrid !== undefined && typeof value.showGrid !== "boolean") return DEFAULT_WORLD3D_PRESENTATION;
    if (value.showGizmo !== undefined && typeof value.showGizmo !== "boolean") return DEFAULT_WORLD3D_PRESENTATION;
    if (value.interactive !== undefined && typeof value.interactive !== "boolean") return DEFAULT_WORLD3D_PRESENTATION;
    if (value.viewportMask !== undefined && value.viewportMask !== "rectangle" && value.viewportMask !== "ellipse") return DEFAULT_WORLD3D_PRESENTATION;
    if (value.clear !== undefined && value.clear !== "theme" && value.clear !== "transparent") return DEFAULT_WORLD3D_PRESENTATION;
    return {
      showGrid: value.showGrid ?? true,
      showGizmo: value.showGizmo ?? true,
      interactive: value.interactive ?? true,
      viewportMask: value.viewportMask ?? "rectangle",
      clear: value.clear ?? "theme",
      ...(sourceAspect === undefined ? {} : { sourceAspect }),
    } as World3dPresentation;
  } catch {
    return DEFAULT_WORLD3D_PRESENTATION;
  }
}

export type World3dScene = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  /** 🚚️ Wave B44 (ticket 26/09/02/PUZZLE-3D-END-TO-END): the per-object delta that produced the
   * current {@link instancesJson} — `{base, revision, count, changed:[record], removed:[id]}`. A
   * consumer whose retained instance set is at `base` may apply `changed`/`removed` by id instead of
   * re-parsing the whole set; `instancesJson` stays authoritative, so ignoring this lane is always
   * correct. Producers with no incremental residency omit it. */
  readonly instancesDeltaJson?: string | null;
  readonly selectionJson: string;
  readonly vorticesJson?: string;
  readonly attractionsJson?: string;
  readonly targetVolumesJson?: string;
  readonly referencesJson?: string;
  readonly brushPreviewJson?: string;
  readonly interactionJson?: string;
  readonly engagementPreviewJson?: string;
  /** 🧲️ The sub-object pick targets this surface hit-tests against — a JSON array of
   * `{kind, id, point, points, typology, selectable, style}`, the wire half of the CAD picking
   * engine's `SpatialPickTarget`/`targetStyle`. Bounded on both ends: the producer caps what it
   * publishes and the consumer caps what it retains. See the Rust `World3dScene::pick_targets_json`. */
  readonly pickTargetsJson?: string;
  readonly lodJson?: string;
  readonly presentationJson?: string;
  readonly chunkingJson?: string;
  readonly environmentJson?: string;
  readonly frameJson?: string;
  readonly fitJson?: string;
  /** 🌐️⛰️ GIS 3D terrain style/source descriptor, consumed by `🗺️WorldTerrainLayer`. */
  readonly terrainJson?: string;
  /** ☁️ Point-cloud rendering layers (10^5-10^6 points) — an array of `{ id, positionsB64 (base64 le
   * f32 xyz), colorsB64? (base64 u8 rgb), size, sizeAttenuation }`, consumed by `WorldPointCloudLayer`. */
  readonly pointsJson?: string;
  /** 📏️ The annotation layer (linear dimensions, angles, point markers, leader labels) — a typed lane, see
   * {@link World3dAnnotationLayer}; read it through `parseWorld3dAnnotationLayer`. */
  readonly annotations?: World3dAnnotationLayer;
  /** 🌡️ The scalar analysis field painted as a heatmap with a legend — a typed lane, see
   * {@link World3dScalarField}; read it through `parseWorld3dScalarField`. */
  readonly scalarField?: World3dScalarField;
  /** ⚙️ Pick granularity filter, section plane and sub-element highlight tokens — a typed lane, see
   * {@link World3dModellingOptions}; read it through `parseWorld3dModellingOptions`. */
  readonly modellingOptions?: World3dModellingOptions;
  /** ⏳️ Off-main-thread compute status shown as an overlay while a `flowEvalTick` chain resolves the
   * meshes this scene renders — see {@link World3dComputeStatusV1} for the full declared shape and
   * {@link world3dComputeStatusV1} for the one parser every surface reads it through. */
  readonly statusJson?: string;
  /** ⏯️ The base64url `ToolRunTraceDelta` (`🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts`) the tool run ledger
   * pages to this window after the renderer's echoed `toolRunTraceCursor`. Absent while no run traces. */
  readonly toolRunTrace?: string | null;
  /** 🪟️ The framework interaction domain id this world window is bound to (see the Rust
   * `World3dScene::domain_id` doc comment). `undefined` leaves the window on plain plugin-private
   * actions (`setHover`/`worldPick`/`worldSelect`); when set, the renderer routes instance pick/hover
   * through `interactionSelect`/`interactionHover` on this domain instead — see
   * `world3dHoverActionArgs`/`world3dSelectionActionArgs`. */
  readonly domainId?: string;
  /** 🎯️ `domainId`'s bound domain granularity id for a plain (non-component) instance pick/hover hit. */
  readonly domainGranularityId?: string;
  /** 🛤️ On a scene SPINE (what arrives inside `SurfaceProps.doc`) this names every payload lane that
   * rides OUTSIDE it, with the byte length and content hash of each — see {@link WORLD3D_SCENE_LANES}
   * and the Rust `World3dScene::lanes` doc. Absent on an assembled scene built by hand. */
  readonly lanes?: readonly SceneLaneRef[];
};

/** 🌍️ An English/German pair carried by a producer that has no default language. */
export type World3dComputeLabelV1 = { readonly en: string; readonly de: string };

/**
 * 🚥️ The declared shape of {@link World3dScene.statusJson} — what a surface may render and
 * what a producer must therefore fill.
 *
 * 🛑️ `cancellable` + `cancelAction` are the CANCEL CONTRACT, and they are deliberately part of the
 * scene's own vocabulary rather than any plugin's: the shell offers the affordance while
 * `cancellable` is true and dispatches whatever id `cancelAction` names, so it never learns a
 * domain verb from code. A producer that can be stopped fills both; one that cannot fills neither
 * and gets no button (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
export type World3dComputeStatusV1 = {
  /** 🌀️ Whether work is still resolving — the spinner's own predicate. */
  readonly computing: boolean;
  /** 🏷️ A stable wire tag for the current phase (`idle`, `meshingFaces`, `cancelled`, `faulted`, …). */
  readonly phase: string;
  /** 🔖️ The phase's own English/German label, when the producer supplies one. */
  readonly phaseLabel: World3dComputeLabelV1 | null;
  /**
   * 💡️ Why this surface has nothing to show, in the producer's own words — already localized by the
   * producer, exactly as `phaseLabel` is.
   *
   * The generation3d generate preview publishes `labels.preview_hint` here whenever it delivered no
   * meshes and no instances ("(evaluate a generation to preview output)" /
   * "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)"), and until now this reader
   * dropped the field: the string was on the wire and on `data-status-json`, and NOTHING painted it,
   * so a user looking at an empty 3D pane was told nothing at all
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `panel-i18n · de:preview_hint`).
   */
  readonly hint: string;
  readonly unitsDone: number;
  readonly unitsTotal: number;
  readonly facesDone: number;
  readonly facesTotal: number;
  readonly inFlight: number;
  /** 📈️ Fraction in `[0, 1]`, as the producer reports it. */
  readonly ratio: number;
  /** 🛑️ Whether a cancel would stop anything RIGHT NOW. */
  readonly cancellable: boolean;
  /** ✖️ The action id a cancel affordance dispatches; empty when the producer declares none. */
  readonly cancelAction: string;
  /** 🧾️ The arguments the affordance dispatches `cancelAction` with — string and number values only (a
   * tool run's `{runId, generation}`); empty when the producer declares none. */
  readonly cancelArgs: Readonly<Record<string, string | number>>;
};

/** 🕳️ Longest empty-surface hint this reader admits — chrome text, never a document. */
const WORLD3D_COMPUTE_HINT_MAXIMUM_CHARACTERS = 200;

const WORLD3D_EMPTY_COMPUTE_STATUS: World3dComputeStatusV1 = Object.freeze({
  computing: false,
  phase: "idle",
  phaseLabel: null,
  hint: "",
  unitsDone: 0,
  unitsTotal: 0,
  facesDone: 0,
  facesTotal: 0,
  inFlight: 0,
  ratio: 1,
  cancellable: false,
  cancelAction: "",
  cancelArgs: Object.freeze({}),
});

function world3dComputeLabel(value: unknown): World3dComputeLabelV1 | null {
  if (typeof value !== "object" || value === null) return null;
  const { en, de } = value as { en?: unknown; de?: unknown };
  if (typeof en !== "string" || typeof de !== "string" || en.length === 0 || de.length === 0) return null;
  return { en, de };
}

function world3dComputeArgs(value: unknown): Readonly<Record<string, string | number>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return {};
  return Object.fromEntries(Object.entries(value).filter((entry): entry is [string, string | number] => typeof entry[1] === "string" || (typeof entry[1] === "number" && Number.isFinite(entry[1]))));
}

function world3dComputeNumber(value: unknown): number {
  return typeof value === "number" && Number.isFinite(value) && value >= 0 ? value : 0;
}

/**
 * 📖️ The ONE reader of {@link World3dScene.statusJson}. Total: malformed JSON, a missing
 * field or a hostile type degrades to the neutral status rather than throwing inside a render — a
 * status overlay must never be able to blank the viewport it annotates.
 *
 * 🛑️ `cancellable` is honoured only alongside a non-empty `cancelAction`: an affordance with no
 * action to dispatch is a dead button, which is worse than no button.
 */
export function world3dComputeStatusV1(statusJson: string | undefined | null): World3dComputeStatusV1 {
  if (typeof statusJson !== "string" || statusJson.length === 0) return WORLD3D_EMPTY_COMPUTE_STATUS;
  let parsed: unknown;
  try {
    parsed = JSON.parse(statusJson);
  } catch {
    return WORLD3D_EMPTY_COMPUTE_STATUS;
  }
  if (typeof parsed !== "object" || parsed === null) return WORLD3D_EMPTY_COMPUTE_STATUS;
  const row = parsed as Record<string, unknown>;
  const progress = (typeof row.progress === "object" && row.progress !== null ? row.progress : {}) as Record<string, unknown>;
  const cancelAction = typeof row.cancelAction === "string" ? row.cancelAction : "";
  const unitsTotal = world3dComputeNumber(progress.unitsTotal);
  const unitsDone = Math.min(world3dComputeNumber(progress.unitsDone), unitsTotal === 0 ? Number.POSITIVE_INFINITY : unitsTotal);
  return {
    computing: row.computing === true,
    phase: typeof row.phase === "string" && row.phase.length > 0 ? row.phase : "idle",
    phaseLabel: world3dComputeLabel(row.phaseLabel),
    hint: typeof row.hint === "string" ? row.hint.slice(0, WORLD3D_COMPUTE_HINT_MAXIMUM_CHARACTERS) : "",
    unitsDone,
    unitsTotal,
    facesDone: world3dComputeNumber(progress.facesDone),
    facesTotal: world3dComputeNumber(progress.facesTotal),
    inFlight: world3dComputeNumber(progress.inFlight),
    ratio: typeof progress.ratio === "number" && Number.isFinite(progress.ratio) ? Math.min(Math.max(progress.ratio, 0), 1) : unitsTotal > 0 ? unitsDone / unitsTotal : 1,
    cancellable: row.cancellable === true && cancelAction.length > 0,
    cancelAction,
    cancelArgs: world3dComputeArgs(row.cancelArgs),
  };
}
