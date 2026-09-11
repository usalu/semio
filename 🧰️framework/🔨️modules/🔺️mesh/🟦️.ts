// #region 🔺️Mesh
/// <reference types="vitest/importMeta" />
/** @emoji 🔺️ `@semio-tech/framework` — component scene protocol payloads shared by render hosts. */
import type { IconName } from "@semio-tech/assets";
import type { LocalizedLabel } from "../🛂️manifest/🤖️generated/🎚️ui-axes.ts";
import type { ActionDescriptor, PluginContextMenuRequest } from "../🛂️manifest/🟦️.ts";

//#region ComponentSceneProtocol
/** 🖼️ A 2D canvas surface scene payload — mirrors the wasm `componentScene` node's `canvas2d` field. */
export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
};

/** 🖱️ A render-time address for an on-demand context menu — bytes only, never items. At right-click
 * time the host resolves the nearest `menu` up the tree (or a scene's implicit surface-kind
 * convention id) and asks the owning plugin's `contextMenu` export to compute rows fresh; nothing
 * here is cached across clicks. */
export type UiMenuRef = {
  readonly id: string;
  readonly args?: Record<string, unknown>;
};

/** 🌐️ A 3D world surface scene payload — mirrors the wasm `componentScene` node's `world3d` field. */
/** 🖱️ One row of a resolved context menu — TS twin of the Rust `ContextMenuItemSpec`
 * (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs`). Plugins build these with
 * `MenuBuilder`; the host maps them through `ContextMenuController` (React) / `render_context_menu`
 * (wgpu) unchanged. */
export type ContextMenuItemSpec = {
  readonly id: string;
  readonly label?: string;
  readonly icon?: string;
  readonly color?: string;
  readonly shortcut?: string;
  readonly disabled?: boolean;
  readonly separator?: boolean;
  readonly checked?: boolean;
  readonly destructive?: boolean;
  readonly action?: string;
  readonly args?: Record<string, unknown>;
  readonly hoverAction?: string;
  readonly hoverArgs?: Record<string, unknown>;
  readonly children?: readonly ContextMenuItemSpec[];
};

//#region 🗂️ContextMenuOrganizer
/** 🗂️ Canonical ribbon-parent taxonomy — TS twin of the Rust `RIBBON_PARENT_CATEGORIES` const
 * (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs`) and of ui-react's closed
 * `UiRibbonParentCategory` union (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`).
 * Id spelling and order are load-bearing: `organizeContextMenu` sorts `menu.group.<category>`
 * rows by this order (unknown categories sort after, in emit order). */
const RIBBON_PARENT_CATEGORIES = [
  "history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings",
  "methods", "mode", "targets", "export", "tools", "utilities", "sync",
] as const;

/** 🗂️ Id prefix every category group row carries. A group row travels with `label: undefined` on
 * purpose — the shell that renders the menu owns the chrome vocabulary and resolves the label from
 * the category behind this prefix (React: `contextMenuGroupLabel`; wgpu: `ribbon_parent_label`). */
export const CONTEXT_MENU_GROUP_ID_PREFIX = "menu.group.";

/** 🗂️ The one group category `organizeContextMenu` synthesizes rather than reads off the taxonomy:
 * the overflow bucket surplus groups fold into once the row budget is exceeded. It is deliberately
 * NOT a `RIBBON_PARENT_CATEGORIES` id, so it resolves through its own chrome label instead. */
export const CONTEXT_MENU_OVERFLOW_CATEGORY = "more";

const CONTEXT_MENU_ROW_BUDGET = 9;
const CONTEXT_MENU_PRIMARY_BUDGET = 5;

function contextMenuIsBareSeparator(item: ContextMenuItemSpec): boolean {
  return item.separator === true && item.label === undefined;
}

/** 🗂️ D1: a separator carrying a `label` is a non-interactive section header, not a divider. */
function contextMenuIsHeader(item: ContextMenuItemSpec): boolean {
  return item.separator === true && item.label !== undefined;
}

function contextMenuIsGroupRow(item: ContextMenuItemSpec): boolean {
  return item.id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX);
}

function contextMenuGroupCategory(item: ContextMenuItemSpec): string {
  return item.id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX) ? item.id.slice(CONTEXT_MENU_GROUP_ID_PREFIX.length) : item.id;
}

function contextMenuTaxonomyRank(category: string): number {
  const index = (RIBBON_PARENT_CATEGORIES as readonly string[]).indexOf(category);
  return index === -1 ? RIBBON_PARENT_CATEGORIES.length : index;
}

function contextMenuSeparatorRow(seed: number): ContextMenuItemSpec {
  return { id: `separator-organized-${seed}`, separator: true };
}

/** 🗂️ Collapses a run of consecutive bare (unlabeled) separators down to one, then drops a bare
 * separator left at the very start or end (nothing to separate from/to). A labeled separator (header,
 * see `contextMenuIsHeader`) is never touched by this — it always survives in place, adjacent bare
 * separators collapse/drop around it independently. */
function contextMenuNormalizeSeparators(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const out: ContextMenuItemSpec[] = [];
  for (const item of items) {
    if (contextMenuIsBareSeparator(item) && out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1]!)) {
      continue;
    }
    out.push(item);
  }
  if (out.length > 0 && contextMenuIsBareSeparator(out[0]!)) {
    out.shift();
  }
  while (out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1]!)) {
    out.pop();
  }
  return out;
}

/** 🗂️ Merges rows sharing a `menu.group.<category>` id at the position of the first occurrence,
 * concatenating and deduping their `children` by id (first occurrence wins). */
function contextMenuMergeGroupRows(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const out: ContextMenuItemSpec[] = [];
  const groupIndex = new Map<string, number>();
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      const index = groupIndex.get(item.id);
      if (index !== undefined) {
        const children = out[index]!.children ? [...out[index]!.children!] : [];
        for (const child of item.children ?? []) {
          if (!children.some((existing) => existing.id === child.id)) {
            children.push(child);
          }
        }
        out[index] = { ...out[index]!, children };
      } else {
        groupIndex.set(item.id, out.length);
        out.push(item);
      }
    } else {
      out.push(item);
    }
  }
  return out;
}

/** 🗂️ ≤9-interactive-row emission (D2 rule): plain leaves/headers in emit order, then group rows in
 * taxonomy order (unknown categories after, emit order), then — only if any exist — a separator
 * followed by destructive leaves. */
function contextMenuEmitWithinBudget(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const leavesAndHeaders: ContextMenuItemSpec[] = [];
  const groupRows: ContextMenuItemSpec[] = [];
  const destructiveLeaves: ContextMenuItemSpec[] = [];
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      groupRows.push(item);
    } else if (item.destructive === true) {
      destructiveLeaves.push(item);
    } else {
      leavesAndHeaders.push(item);
    }
  }
  groupRows.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  const out = [...leavesAndHeaders, ...groupRows];
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}

/** 🗂️ >9-interactive-row emission (D2 rule): the first 5 plain leaves outside any header section stay
 * primaries; every header's trailing run of leaves folds into a group keyed by that header's own
 * (slugified) label; every other plain leaf buckets into `menu.group.<categoryOf(action) ?? "actions">`;
 * pre-existing group rows pass through unchanged; groups then sort in taxonomy order and, if the
 * primaries+groups row count is still over budget, the excess trailing groups fold into one
 * `menu.group.more`. Destructive leaves are carried separately and appended last, after a separator. */
function contextMenuEmitOverBudget(
  items: readonly ContextMenuItemSpec[],
  categoryOf: (id: string) => string | undefined,
): ContextMenuItemSpec[] {
  function bucketMut(buckets: ContextMenuItemSpec[], id: string): number {
    const index = buckets.findIndex((bucket) => bucket.id === id);
    if (index !== -1) return index;
    buckets.push({ id, label: undefined, children: [] });
    return buckets.length - 1;
  }

  const primaries: ContextMenuItemSpec[] = [];
  const existingGroups: ContextMenuItemSpec[] = [];
  const destructiveLeaves: ContextMenuItemSpec[] = [];
  const bucketedGroups: ContextMenuItemSpec[] = [];
  let currentHeaderKey: string | undefined;

  for (const item of items) {
    if (contextMenuIsHeader(item)) {
      currentHeaderKey = item.label;
      continue;
    }
    if (contextMenuIsGroupRow(item)) {
      existingGroups.push(item);
      currentHeaderKey = undefined;
      continue;
    }
    if (item.destructive === true) {
      destructiveLeaves.push(item);
      continue;
    }
    if (currentHeaderKey !== undefined) {
      const slug = currentHeaderKey.toLowerCase().split(/\s+/).join("-");
      const index = bucketMut(bucketedGroups, `${CONTEXT_MENU_GROUP_ID_PREFIX}${slug}`);
      bucketedGroups[index] = { ...bucketedGroups[index]!, children: [...(bucketedGroups[index]!.children ?? []), item] };
      continue;
    }
    if (primaries.length < CONTEXT_MENU_PRIMARY_BUDGET) {
      primaries.push(item);
      continue;
    }
    const category = categoryOf(item.action ?? item.id) ?? "actions";
    const index = bucketMut(bucketedGroups, `${CONTEXT_MENU_GROUP_ID_PREFIX}${category}`);
    bucketedGroups[index] = { ...bucketedGroups[index]!, children: [...(bucketedGroups[index]!.children ?? []), item] };
  }

  const groups = [...existingGroups, ...bucketedGroups];
  groups.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));

  let out = [...primaries, ...groups];
  if (out.length > CONTEXT_MENU_ROW_BUDGET) {
    const foldFrom = CONTEXT_MENU_ROW_BUDGET - 1;
    const overflowingGroups = out.slice(foldFrom);
    out = out.slice(0, foldFrom);
    const foldedChildren: ContextMenuItemSpec[] = [];
    for (const group of overflowingGroups) {
      foldedChildren.push(...(group.children ?? []));
    }
    out.push({ id: `${CONTEXT_MENU_GROUP_ID_PREFIX}${CONTEXT_MENU_OVERFLOW_CATEGORY}`, label: undefined, children: foldedChildren });
  }
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}

/** 🗂️ Pure organizer enforced at every context-menu funnel — recurses into `children`, normalizes
 * separators (labeled = kept header, bare leading/trailing/doubled = dropped), merges duplicate
 * `menu.group.<category>` rows (deduping their children by id), then applies the ≤9-row / >9-row
 * emission policy from D2 of the grouped-context-menu mechanism design
 * (`contextMenuEmitWithinBudget`/`contextMenuEmitOverBudget`). `categoryOf` resolves a leaf's
 * dispatched action id to a `RIBBON_PARENT_CATEGORIES` id (`undefined` buckets into `"actions"`) —
 * branch-for-branch twin of the Rust `organize_context_menu`
 * (`🧰️framework/🔨️modules/🖱️ui/🧊️wgpu/📦️packages/🦀️rust/📦️lib.rs`). */
export function organizeContextMenu(
  items: readonly ContextMenuItemSpec[],
  categoryOf: (id: string) => string | undefined,
): ContextMenuItemSpec[] {
  const mapped = items.map((item) => ({
    ...item,
    children: item.children ? organizeContextMenu(item.children, categoryOf) : item.children,
  }));
  const normalized = contextMenuMergeGroupRows(contextMenuNormalizeSeparators(mapped));
  const interactiveCount = normalized.filter((item) => item.separator !== true).length;
  return interactiveCount <= CONTEXT_MENU_ROW_BUDGET
    ? contextMenuEmitWithinBudget(normalized)
    : contextMenuEmitOverBudget(normalized, categoryOf);
}
//#endregion 🗂️ContextMenuOrganizer

export type World3dScene = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  readonly selectionJson: string;
  readonly vorticesJson?: string;
  readonly attractionsJson?: string;
  readonly targetVolumesJson?: string;
  readonly referencesJson?: string;
  readonly brushPreviewJson?: string;
  readonly interactionJson?: string;
  readonly engagementPreviewJson?: string;
  readonly lodJson?: string;
  readonly chunkingJson?: string;
  readonly environmentJson?: string;
  readonly frameJson?: string;
  readonly fitJson?: string;
  /** 🌐️⛰️ GIS 3D terrain style/source descriptor, consumed by `🗺️WorldTerrainLayer`. */
  readonly terrainJson?: string;
  /** ☁️ Point-cloud rendering layers (10^5-10^6 points) — an array of `{ id, positionsB64 (base64 le
   * f32 xyz), colorsB64? (base64 u8 rgb), size, sizeAttenuation }`, consumed by `WorldPointCloudLayer`. */
  readonly pointsJson?: string;
  /** ⏳️ Off-main-thread compute status (`{"computing": true, "label": "…"}`) shown as an overlay while
   * a `flowEvalTick` chain resolves the meshes this scene renders. */
  readonly statusJson?: string;
  /** 🪟️ The framework interaction domain id this world window is bound to (see the Rust
   * `World3dScene::domain_id` doc comment). `undefined` leaves the window on plain plugin-private
   * actions (`setHover`/`worldPick`/`worldSelect`); when set, the renderer routes instance pick/hover
   * through `interactionSelect`/`interactionHover` on this domain instead — see
   * `world3dHoverActionArgs`/`world3dSelectionActionArgs`. */
  readonly domainId?: string;
  /** 🎯️ `domainId`'s bound domain granularity id for a plain (non-component) instance pick/hover hit. */
  readonly domainGranularityId?: string;
  /** 🚚️ On a scene SPINE (what arrives inside `SurfaceProps.doc`) this names every payload lane that
   * rides OUTSIDE it, with the byte length and content hash of each — see {@link WORLD3D_SCENE_LANES}
   * and the Rust `World3dScene::lanes` doc. Absent on an assembled scene built by hand. */
  readonly lanes?: readonly World3dSceneLaneRef[];
};

//#region 🚚️World3dSceneLanes
/** 🚚️ One entry of {@link World3dScene.lanes}. `bytes` is what tells a fully arrived lane from one
 * still spread across reconcile pages; `hash` (FNV-1a/64, lowercase hex, computed by the producer)
 * is what makes the spine itself differ exactly when a lane's content differs. */
export type World3dSceneLaneRef = {
  readonly lane: string;
  readonly bytes: number;
  readonly hash: string;
};

/** 🚚️ One world-3d payload lane: which {@link World3dScene} field it carries and which reserved node
 * key its retained text carrier is rooted at. */
export type World3dSceneLane = {
  readonly lane: string;
  readonly field: keyof World3dScene;
  readonly bodyKey: string;
  readonly optional: boolean;
};

/** 🚚️ Reserved carrier-key namespace — dotted and `framework.`-prefixed so a lane root can never
 * collide with an app-authored node id. */
export const WORLD3D_SCENE_LANE_KEY_PREFIX = "framework.scene.world3d.";

/** 🚚️ The eighteen world-3d payload fields that ride OUTSIDE the fixed-capacity surface doc, each as
 * its own retained, individually paged text carrier. `SurfaceDoc.bytes` is a hard 32 KiB
 * `UiFixedBytes` ceiling that cannot page, so a world whose payload scales with its document (a
 * measured 57 281-byte Nakagin Capsule Tower) can only publish with the payload split out; keeping the
 * lanes SEPARATE is what lets an unchanged lane cost nothing on a partial refresh.
 *
 * Mirrors the Rust `World3dSceneLane` / `WORLD3D_SCENE_LANE_*` in
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs`. Both sides are pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`. */
export const WORLD3D_SCENE_LANES: readonly World3dSceneLane[] = [
  { lane: "meshes", field: "meshesJson", bodyKey: "framework.scene.world3d.meshes", optional: false },
  { lane: "instances", field: "instancesJson", bodyKey: "framework.scene.world3d.instances", optional: false },
  { lane: "selection", field: "selectionJson", bodyKey: "framework.scene.world3d.selection", optional: false },
  { lane: "vortices", field: "vorticesJson", bodyKey: "framework.scene.world3d.vortices", optional: true },
  { lane: "attractions", field: "attractionsJson", bodyKey: "framework.scene.world3d.attractions", optional: true },
  { lane: "targetVolumes", field: "targetVolumesJson", bodyKey: "framework.scene.world3d.targetVolumes", optional: true },
  { lane: "references", field: "referencesJson", bodyKey: "framework.scene.world3d.references", optional: true },
  { lane: "brushPreview", field: "brushPreviewJson", bodyKey: "framework.scene.world3d.brushPreview", optional: true },
  { lane: "interaction", field: "interactionJson", bodyKey: "framework.scene.world3d.interaction", optional: true },
  { lane: "engagementPreview", field: "engagementPreviewJson", bodyKey: "framework.scene.world3d.engagementPreview", optional: true },
  { lane: "lod", field: "lodJson", bodyKey: "framework.scene.world3d.lod", optional: true },
  { lane: "chunking", field: "chunkingJson", bodyKey: "framework.scene.world3d.chunking", optional: true },
  { lane: "environment", field: "environmentJson", bodyKey: "framework.scene.world3d.environment", optional: true },
  { lane: "frame", field: "frameJson", bodyKey: "framework.scene.world3d.frame", optional: true },
  { lane: "fit", field: "fitJson", bodyKey: "framework.scene.world3d.fit", optional: true },
  { lane: "terrain", field: "terrainJson", bodyKey: "framework.scene.world3d.terrain", optional: true },
  { lane: "points", field: "pointsJson", bodyKey: "framework.scene.world3d.points", optional: true },
  { lane: "status", field: "statusJson", bodyKey: "framework.scene.world3d.status", optional: true },
];

/** 🚚️ Resolves a retained node key back to the lane it carries, `undefined` for every other key. */
export function world3dSceneLaneForBodyKey(bodyKey: string): World3dSceneLane | undefined {
  return WORLD3D_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🚚️ Reassembles a decoded scene spine and its lane carrier texts back into the `World3dScene` every
 * render host already knows how to read — the exact inverse of the Rust `SceneDoc::split_lanes`.
 *
 * `laneTexts` is keyed by reserved carrier key ({@link World3dSceneLane.bodyKey}). A lane the spine
 * declares but `laneTexts` does not carry is left at its spine value (an empty string for a required
 * lane, absent for an optional one) rather than guessed: a caller that wants last-known-good content
 * across a partially arrived refresh supplies it in `laneTexts` itself, which is exactly what the
 * Interpreter's per-lane cache does. Never throws. */
export function world3dSceneFromLanes(spine: World3dScene, laneTexts: ReadonlyMap<string, string>): World3dScene {
  if (laneTexts.size === 0) return spine;
  const assembled: Record<string, unknown> = { ...spine };
  for (const lane of WORLD3D_SCENE_LANES) {
    const text = laneTexts.get(lane.bodyKey);
    if (text === undefined) continue;
    const prior = assembled[lane.field];
    if (lane.optional && text.length === 0 && typeof prior === "string" && prior.length > 0) continue;
    assembled[lane.field] = text;
  }
  return assembled as World3dScene;
}
//#endregion 🚚️World3dSceneLanes

/** 🔌️ One port on a node-graph node: identity + display label (direction is implied by whether the
 * record lives in the owning node's `inputs` or `outputs` array). `code`/`abbreviation`/`fullName`/
 * `resourceKind` are set only for OS-workflow app-instance nodes (the wire key stays `resourceKind` —
 * the rename to `artifactKind` is W4/`OsWorkflowNodeGraphPayload` scope, not this ticket's). */
export type NodeGraphPortRecord = {
  readonly id: string;
  readonly label?: string;
  readonly code?: string;
  readonly abbreviation?: string;
  readonly fullName?: string;
  readonly resourceKind?: string;
};

/** 🕸️ One node-graph node: identity, label, layout rect, typed ports. `instanceId`/`pluginId`/`appId`/
 * `icon` are set only for OS-workflow app-instance nodes (the space canvas's node-graph). */
export type NodeGraphNodeRecord = {
  readonly id: string;
  readonly label?: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly NodeGraphPortRecord[];
  readonly outputs: readonly NodeGraphPortRecord[];
  readonly instanceId?: string;
  readonly pluginId?: string;
  readonly appId?: string;
  readonly icon?: string;
};

/** 🕸️ One node-graph edge between two node/port endpoints. */
export type NodeGraphEdgeRecord = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
  readonly label?: string;
};

/** 📷️ Node-graph camera: pan position + zoom factor. */
export type NodeGraphViewport = {
  readonly x: number;
  readonly y: number;
  readonly zoom: number;
};

/** 🔎️ One spotlight/find result row for a node-graph surface. */
export type NodeGraphFindItem = {
  readonly id: string;
  readonly label: string;
  readonly category: string;
};

/** 🖱️ Hovered node id, if any. */
export type NodeGraphHover = {
  readonly nodeId?: string;
  /** 🔌️ Hovered channel (port) id on `nodeId`, matching the `"{nodeId}@{portId}"` pick id. */
  readonly portId?: string;
};

/** ➕️ Variadic input/output slot on an operator catalogue entry. */
export type NodeGraphOperatorVariadicRecord = {
  readonly slotKey: string;
  readonly min: number;
  readonly max?: number;
};

/** 🔌️ Declared operator channel (input or output); `cardinality` rides as its serialized symbol
 * string (`"!"`/`"?"`/`"*"`/`"+"`/digits). */
export type NodeGraphOperatorChannelRecord = {
  readonly code: string;
  readonly abbreviation: string;
  readonly name: string;
  readonly fullName: string;
  readonly operators?: readonly string[];
  readonly default?: unknown;
  readonly label?: string;
  readonly cardinality: string;
};

/** 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette. */
export type NodeGraphOperatorRecord = {
  readonly id: string;
  readonly extension: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
  readonly inputs: readonly NodeGraphOperatorChannelRecord[];
  readonly outputs: readonly NodeGraphOperatorChannelRecord[];
  readonly variadicInput?: NodeGraphOperatorVariadicRecord;
  readonly variadicOutput?: NodeGraphOperatorVariadicRecord;
  readonly group?: readonly string[];
};

/** 🛍️ One drag-and-drop palette entry — mirrors the Rust `CatalogueItem`. */
export type AppCatalogueItem = {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly format?: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
};

/** 🛍️ One palette section — mirrors the Rust `CatalogueSection`/`CatalogueGroup` pair. */
export type AppCatalogueSection = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly AppCatalogueItem[];
  readonly groups?: readonly AppCatalogueSection[];
};

/**
 * 🛍️ The APP-STATIC catalogue an app publishes ONCE per instance on the reserved
 * `framework.section.catalogue` retained surface — mirrors the Rust `FlowAppCatalogue`
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs`).
 *
 * 🚨️ It is deliberately NOT part of {@link NodeGraphScene}. With the real `brep`/`math` operator sets
 * installed it is ~100 KB, and a surface payload is admitted against the fixed 32 KiB
 * `UI_FIXED_BYTES` capacity, so embedding it per scene made every node-graph window fail admission at
 * 111 031 B and render nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). A scene now references
 * an operator by KIND ID only, and the renderer caches this payload for the app's whole lifetime.
 */
export type AppCatalogue = {
  readonly operators?: readonly NodeGraphOperatorRecord[];
  readonly sections?: readonly AppCatalogueSection[];
};

/** 🕸️ A node-graph surface scene payload — mirrors the wasm `componentScene` node's `nodeGraph` field. */
export type NodeGraphScene = {
  readonly nodes: readonly NodeGraphNodeRecord[];
  readonly edges: readonly NodeGraphEdgeRecord[];
  readonly viewport?: NodeGraphViewport;
  readonly editable?: boolean;
  /** 🔌️ DOCUMENT-DERIVED operator records only — one per node this graph holds (the OS workflow window
   * derives one per workflow node so the canvas can lay its ports out). The app's REGISTERED operator
   * catalogue rides {@link AppCatalogue} on the reserved `framework.section.catalogue` surface instead;
   * a flow-backed scene leaves this empty. */
  readonly operators?: readonly NodeGraphOperatorRecord[];
  readonly findItems?: readonly NodeGraphFindItem[];
  readonly selection?: readonly string[];
  readonly hover?: NodeGraphHover;
  /** ✨️ Ids — nodes, edges, or `"{nodeId}@{portId}"` ports — the plugin wants highlighted, e.g.
   * because they are transitively hovered from another window. */
  readonly highlighted?: readonly string[];
  readonly previewOffJson?: string;
  readonly lodJson?: string;
  readonly controlsJson?: string;
  readonly clustersJson?: string;
  readonly computingJson?: string;
  readonly statusJson?: string;
  readonly capabilitiesJson?: string;
  readonly fixtureJson?: string;
  readonly presencePeersJson?: string;
  /** 🧵️ Channel-structured eval outputs from an off-main-thread `flowEvalTick` chain, applied via
   * `FlowWasmSession.applyEvalOutputsJson` — lets the canvas session pick up results without ever
   * evaluating itself. */
  readonly evalJson?: string;
};

/** 👥️ A live-collaboration cursor/selection peer shown on a shared surface. */
export type PresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly selectionCount: number;
};

/** 📝️ A text-editor surface scene payload — mirrors the wasm `componentScene` node's `textEditor` field. */
export type TextEditorScene = {
  readonly buffer: string;
  readonly language?: string;
  readonly selectionJson?: string;
  readonly tokensJson?: string;
  readonly diagnosticsJson?: string;
  readonly completionsJson?: string;
  readonly overlaysJson?: string;
  readonly occurrencesJson?: string;
  readonly placeholdersJson?: string;
  readonly extraCaretsJson?: string;
  readonly selectableSpansJson?: string;
  readonly settingsJson?: string;
  readonly cameraJson?: string;
  readonly hoverJson?: string;
  readonly newlineGatesJson?: string;
  readonly renameJson?: string;
};

export const nodeGraphActions = {
  select: "interactionSelect",
  hover: "interactionHover",
  edit: "nodeGraphEdit",
  parameter: "setGraphParameter",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;

export const textEditorActions = {
  edit: "textEdit",
  select: "textSelect",
  hover: "textHover",
  requestCompletions: "requestCompletions",
  commitRename: "commitRename",
  formatDocument: "formatDocument",
} as const;

/** 📋️ A table surface scene payload — mirrors the wasm `componentScene` node's `table` field. */
export type TableScene = {
  readonly columnsJson: string;
  readonly rowsJson: string;
  readonly selectionJson?: string;
  readonly rowDragMime?: string;
  readonly dropAction?: ActionDescriptor;
  readonly sortJson?: string;
};

/** 🖌️ A 2D paint surface scene payload — mirrors the wasm `componentScene` node's `paint2d` field. */
export type Paint2dScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly brushSize: number;
  readonly brushOpacity: number;
  readonly viewMode: string;
  readonly compositeViewportJson?: string;
};

/** 🎨️ An icon-render preview surface scene payload — mirrors the wasm `componentScene` node's `iconRender` field. */
export type IconRenderScene = {
  readonly requestJson: string;
  readonly footer?: string;
  readonly frameJson?: string;
};

/** 🗂️ A virtual-file-system browser surface scene payload — mirrors the wasm `componentScene` node's `virtualFileSystem` field. */
export type VirtualFileSystemScene = {
  readonly schemaJson: string;
  readonly rowsJson: string;
  readonly selectedRowIdsJson?: string;
  readonly hoveredRowId?: string;
  readonly emptyMessage?: string;
  readonly dragDropEnabled?: boolean;
};

/** 🗺️ A tiled map surface scene payload — mirrors the wasm `componentScene` node's `tiledMap` field. */
export type TiledMapScene = {
  readonly mapFixtureJson: string;
  readonly cameraJson: string;
  readonly renderMode: string;
  readonly vectorStyle: string;
  readonly lodMode: string;
  readonly tileUrlTemplate: string;
  readonly vectorTileUrlTemplate: string;
  readonly layerVisibilityJson: string;
  readonly layerStrokeScaleJson: string;
  readonly selectionJson: string;
  readonly hoverJson: string;
  readonly selectionMethod: string;
  readonly selectionMode: string;
};

/** 🧩️ A 2D board surface scene payload — mirrors the wasm `componentScene` node's `board2d` field. */
export type Board2dScene = {
  readonly fixtureJson: string;
  readonly cameraJson: string;
  readonly glyphCatalogsJson: string;
  readonly selectionJson: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeUtility?: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly brushWeightsJson: string;
  readonly placementCompatibilityJson: string;
  readonly lodMode: string;
};

/** 🖊️ An ink-canvas surface scene payload — mirrors the wasm `componentScene` node's `inkCanvas` field. `documentJson` is opaque to the framework: the owning program defines its shape, conventionally an array of items (e.g. stroke | shape | text | image) each carrying its own transform; `selectionJson` is a `string[]` of selected item ids. */
export type InkCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly viewMode: string;
  readonly interactive: boolean;
};

/** 🖊️ Renderer-to-plugin action names for ink-canvas surfaces (modeled after {@link nodeGraphActions}/{@link textEditorActions}). */
export const inkCanvasActions = {
  applyEvents: "inkApplyEvents",
  setSelection: "setSelection",
  setCamera: "setCamera",
  setHover: "setHover",
} as const;

/** 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type GraphTimelineScene = {
  readonly columnsJson: string;
};

/** 🧩️ A palette entry for a block kind insertable into a {@link BlockListScene}, contributed either by the host app's own built-ins or by a `playbookBlockKind` module contribution. */
export type BlockPaletteEntry = {
  readonly blockKind: string;
  readonly label: string;
  readonly iconId: IconName;
};

/** 🧩️ A strict, ordered list of steps/blocks for the Blockly-like list editor. `stepsJson` is a `PlaybookStep[]` array, `paletteJson` is a `BlockPaletteEntry[]` array of the block kinds available to insert. */
export type BlockListScene = {
  readonly stepsJson: string;
  readonly paletteJson: string;
  readonly selectedId?: string;
  readonly draggingId?: string;
};

/** 🆚️ A before/after text diff surface scene payload — mirrors the wasm `componentScene` node's `diffView` field. */
export type DiffViewScene = {
  readonly before: string;
  readonly after: string;
  readonly language?: string;
  readonly mode?: "unified" | "split";
};

/** 📰️ One entry of an {@link EventFeedScene}'s `entriesJson` array. */
export type EventFeedEntry = {
  readonly id: string;
  readonly timestampMs: number;
  readonly iconId: IconName;
  readonly title: string;
  readonly detail?: string;
  readonly tone?: string;
};

/** 📰️ A scrolling event/log feed surface scene payload — mirrors the wasm `componentScene` node's `eventFeed` field. `entriesJson` is an {@link EventFeedEntry}`[]` array. */
export type EventFeedScene = {
  readonly entriesJson: string;
  readonly follow?: boolean;
  readonly activateAction?: string;
};

/** 🔌️ A plugin-contributed external body rendered inline — mirrors the wasm `externalSlot` node. */
export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
  readonly menu?: UiMenuRef;
};

/** 🧭️ The dispatch key on {@link UiComponentSceneNode} — matches the lazy-loaded host component per `framework/os/renderer/js/react/index.tsx`. */
export type ComponentKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";

/** 🖥️ A native (non-declarative) rendering surface — mirrors the wasm `componentScene` node; the active `componentKind` selects which optional scene field is populated. */
export type UiComponentSceneNode = {
  readonly type: "componentScene";
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly componentKind: string;
  readonly paneId?: string;
  readonly bindingId?: string;
  /** 🖱️ Optional override of the implicit per-`componentKind` convention id (`"world3d"`,
   * `"nodeGraph"`, `"tiledMap"`, ...) the host uses when resolving which surface answers a
   * right-click — set only when a plugin needs a menu id other than the surface-kind default. */
  readonly menu?: UiMenuRef;
  readonly canvas2d?: Canvas2dScene;
  readonly world3d?: World3dScene;
  readonly nodeGraph?: NodeGraphScene;
  readonly textEditor?: TextEditorScene;
  readonly table?: TableScene;
  readonly paint2d?: Paint2dScene;
  readonly virtualFileSystem?: VirtualFileSystemScene;
  readonly tiledMap?: TiledMapScene;
  readonly board2d?: Board2dScene;
  readonly iconRender?: IconRenderScene;
  readonly inkCanvas?: InkCanvasScene;
  readonly graphTimeline?: GraphTimelineScene;
  readonly blockList?: BlockListScene;
  readonly diffView?: DiffViewScene;
  readonly eventFeed?: EventFeedScene;
};

/** 🧷️ Shared prop shape for the host components under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`. */
export type ComponentSceneHostProps = {
  readonly node: UiComponentSceneNode;
  /** 🏁️ Returns a promise that settles when the dispatched action's guest work is FINISHED — its typed
   * operation completed and the host applied that completion — not merely when the admitting round trip
   * returned. A background tick loop (`createInFlightSkippingInterval`) can only gate itself on the
   * previous tick if this promise means "settled": with a `void` contract the loop's own in-flight flag
   * cleared immediately and 120 ms ticks queued into the serialized guest until the per-actor turn queue
   * overflowed (measured 2026-09-09: 252 `fillBuildTick`s in 35 s, 38 rejected with `queue is full`).
   * A host with nothing to await may still return `void`. */
  readonly onAction: (action: ActionDescriptor) => void | Promise<void>;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion ComponentSceneProtocol
// #endregion 🔺️Mesh
