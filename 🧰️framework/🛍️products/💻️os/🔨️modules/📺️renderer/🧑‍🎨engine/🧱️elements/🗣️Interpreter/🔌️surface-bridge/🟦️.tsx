/** 🔌️ The bridge from a retained surface node to the scene host registered for its kind: kind to scene field and lane tables, host identity, the scene node assembly, the gesture funnel and the registry-resolved host render.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { type ReactNode } from "react";
import {
  type ActionDescriptor,
  type ComponentKind,
  type ComponentSceneHostProps,
  type UiComponentSceneNode,
  type UiMenuRef,
  type SceneLane,
  BOARD2D_SCENE_LANES,
  CANVAS2D_SCENE_LANES,
  PAINT2D_SCENE_LANES,
  TILEDMAP_SCENE_LANES,
  WORLD3D_SCENE_LANES,
  NODE_GRAPH_SCENE_LANES,
  TEXT_EDITOR_SCENE_LANES,
  TABLE_SCENE_LANES
} from "@semio-tech/framework";
import { type SurfaceId, type SurfaceProps, type UiNodeId, type UiNodeRecord } from "@semio-tech/framework";
import { decodeScenePackValue } from "@semio-tech/framework/packs";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { ShellFaultBoundary } from "../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { resolveComponentSceneHost } from "../🗺️element-host-registry/🟦️.ts";
import { interpLabel, type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { VirtualFileSystemHost } from "../🗄️virtual-file-system/🟦️.tsx";

//#region 🔖️UiInterpreter
//#region SurfaceBridge
/** 🗺️ `SurfaceKind` wire tag → the `UiComponentSceneNode` optional field the matching host reads —
 * verbatim the same 15-entry convention `ComponentKind`/`resolveComponentSceneHost` already use,
 * duplicated here only because the bridge needs the FIELD NAME, not the dispatch target. */
const SURFACE_KIND_SCENE_FIELD: Record<string, string> = {
  "canvas-2d": "canvas2d",
  "world-3d": "world3d",
  "node-graph": "nodeGraph",
  "text-editor": "textEditor",
  table: "table",
  "paint-2d": "paint2d",
  "virtual-file-system": "virtualFileSystem",
  "tiled-map": "tiledMap",
  "board-2d": "board2d",
  "icon-render": "iconRender",
  "ink-canvas": "inkCanvas",
  "graph-timeline": "graphTimeline",
  "block-list": "blockList",
  "diff-view": "diffView",
  "event-feed": "eventFeed",
};

/** 🚚️ `SurfaceKind` wire tag → the lane table of every scene kind whose Rust `SceneDoc` overrides
 * `split_lanes` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/📜️doc/🦀️.rs`). A kind IN this table must be
 * rendered through {@link PagedSurfaceView}, which puts the lanes back; a kind that is absent renders
 * its decoded doc verbatim.
 *
 * ⚠️ The membership of this table IS the routing decision — {@link SurfaceView} asks it rather than
 * repeating a list of kinds. `paint-2d` used to be missing from that repeated list while
 * `Paint2dScene::split_lanes` split `documentSyncJson`/`assetsJson` unconditionally, so `Paint2dHost`
 * received `""` for both and its own `try/catch` swallowed the failure: the raster pane's composite and
 * navigator windows rendered completely empty, with no console error and no texture upload
 * (measured on :6033, ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). One table, one decision. */
const SURFACE_KIND_SCENE_LANES: Record<string, readonly SceneLane<Record<string, unknown>>[]> = {
  "text-editor": TEXT_EDITOR_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  table: TABLE_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "node-graph": NODE_GRAPH_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "world-3d": WORLD3D_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "canvas-2d": CANVAS2D_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "board-2d": BOARD2D_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "tiled-map": TILEDMAP_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
  "paint-2d": PAINT2D_SCENE_LANES as readonly SceneLane<Record<string, unknown>>[],
};

/** 🚚️ The lane table a surface kind publishes out of its doc, or `undefined` when it publishes none.
 * See {@link SURFACE_KIND_SCENE_LANES}. */
export function surfaceKindSceneLanes(kind: string): readonly SceneLane<Record<string, unknown>>[] | undefined {
  return SURFACE_KIND_SCENE_LANES[kind];
}

function menuRefFromContract(menu: UiNodeRecord["menu"]): UiMenuRef | undefined {
  if (!menu) return undefined;
  const args = menu.args && typeof menu.args === "object" && !Array.isArray(menu.args) ? (menu.args as Record<string, unknown>) : undefined;
  return { id: menu.id, args };
}

/** 🌉️ True when `docSchema` (`"<kind>@<version>"`) is a shape this bridge can even attempt to decode
 * — the contract itself never validates it against `kind` (see `SurfaceProps`'s own doc: "a mismatch
 * ... is a scene-crate-level authoring bug, not a contract violation"), so this Interpreter is exactly
 * where that gate has to live. Only the SHAPE is checked (non-empty name + numeric version), never a
 * per-kind version registry this file has no visibility into — an unrecognised but well-formed schema
 * still renders through `sceneField`'s per-kind switch; only a malformed one is refused outright. */
function isWellFormedDocSchema(docSchema: string): boolean {
  const at = docSchema.lastIndexOf("@");
  return at > 0 && /^\d+$/.test(docSchema.slice(at + 1));
}

/** 🌉️ Bridges `Component::Surface`'s `SurfaceProps` (one opaque pack-encoded `doc.bytes` payload,
 * keyed by `docSchema`) onto the OLD `UiComponentSceneNode` the 14 scene-host elements (Canvas2dHost,
 * World3dHost, ...) still expect — those elements are outside this packet's OWNS and are unchanged,
 * so this Interpreter decodes the new contract's opaque payload into the exact per-kind scene field
 * shape those hosts already know how to read, rather than duplicating 14 host components. The
 * contract never parses `doc.bytes` itself (see `🦀️surface.rs`'s own doc); this is the one place that
 * decodes it, and only to hand it straight through unmodified.
 *
 * `surfaceId`/`controllerId` no longer exist on `SurfaceProps` (six placement fields were dropped in
 * the `ui-w4-core` mirror regeneration). `surfaceId` is therefore the OWNING DOCUMENT's surface —
 * exactly the `pluginSurfaceRef(instance, target.key)` key the plugin host's `surface_contexts` table
 * is keyed by (`1:puzzle3d-main-perspective` / `1:puzzle3d-main-top`) — because a surface host's
 * identity is the window it was mounted into, not its position in that window's node tree. Taking
 * `record.id` for it instead collapsed BOTH panes of one app onto `"1"`: the id is a per-document
 * DFS-order integer, so two windows of one program mint the same one and nothing downstream (a
 * dispatch's `args.surfaceId`, `data-surface-id`, a per-pane probe) could tell them apart
 * (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B20 defect 1). `controllerId` stays the record id — it
 * scopes per-app host registries (catalogue drop preview) that are deliberately pane-independent.
 * `paneId` carries the program's own authored surface id (`record.key`, set by `scene_surface`'s
 * `try_id`), the one thing the contract really did drop; `bindingId` has no contract equivalent and is
 * simply absent (both optional on `UiComponentSceneNode`). Returns `null`, never throws, on a
 * malformed `docSchema` or a decode failure — the caller renders a placeholder + logs the fault, per
 * this ticket's own "never throw, never drop the surrounding patch" rule for an unknown `doc_schema`. */
/** 🪪️ The three identities one surface host is mounted under, and the one place they are decided.
 *
 * - `surfaceId` — the OWNING DOCUMENT's surface (`1:puzzle3d-main-perspective`), i.e. the plugin
 *   host's own `surface_contexts` key. A surface host's identity is the WINDOW it renders in.
 * - `controllerId` — the record id. Per-app registries keyed on it (the catalogue drop preview) are
 *   deliberately pane-independent, so this one must NOT gain the window.
 * - `paneId` — the program's own authored surface id, the `try_id` `scene_surface` set as the node's
 *   `key`. It is what the dropped `SurfaceProps.surfaceId` used to carry, and is stable across
 *   refreshes where the record id is not.
 *
 * Taking the record id for `surfaceId` collapsed every pane of one app onto `"1"` — a per-document
 * DFS-order integer two windows of one program both mint (ticket 26/09/02/PUZZLE-3D-END-TO-END B20). */
export function surfaceHostIdentityV1(surface: SurfaceId, key: string, recordId: UiNodeId): { readonly surfaceId: string; readonly controllerId: string; readonly paneId: string | undefined } {
  return { surfaceId: String(surface), controllerId: String(recordId), paneId: key || undefined };
}

function surfacePropsToComponentSceneNode(record: UiNodeRecord, props: SurfaceProps, surface: SurfaceId, assemble?: SurfaceSceneAssembler): UiComponentSceneNode | null {
  if (!isWellFormedDocSchema(props.docSchema)) {
    console.error("[Interpreter] malformed Component::Surface docSchema", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema });
    return null;
  }
  let decoded: Record<string, unknown> | undefined;
  try {
    decoded = props.doc.bytes.length > 0 ? (decodeScenePackValue(new Uint8Array(props.doc.bytes)) as Record<string, unknown>) : undefined;
  } catch (error) {
    console.error("[Interpreter] failed to decode Component::Surface doc bytes", { nodeId: record.id, kind: props.kind, docSchema: props.docSchema, error });
    return null;
  }
  const sceneField = SURFACE_KIND_SCENE_FIELD[props.kind];
  const node: Record<string, unknown> = {
    type: "componentScene",
    ...surfaceHostIdentityV1(surface, record.key, record.id),
    componentKind: props.kind,
    menu: menuRefFromContract(record.menu),
  };
  if (sceneField && decoded !== undefined) node[sceneField] = assemble ? assemble(decoded) : decoded;
  return node as unknown as UiComponentSceneNode;
}

/** 🚚️ Reattaches a surface's out-of-doc payload lanes to the spine its `doc.bytes` decoded to. Only a
 * scene kind that declares lanes has one — `world-3d`, `canvas-2d`, `board-2d`, `tiled-map`,
 * `paint-2d` and `text-editor`. Every other kind renders its decoded
 * doc verbatim. A kind that splits but is not listed in {@link SurfaceView}'s route gets its lane
 * fields as EMPTY STRINGS, which is silent at every layer — see {@link PagedSurfaceView}. */
type SurfaceSceneAssembler = (spine: Record<string, unknown>) => Record<string, unknown>;

/** 🖱️ The `onAction` a scene host receives: the shell's funnel with every dispatch stamped `origin: "gesture"`
 * (`🎯️input-ledger` `InputOriginV1`) — a canvas/world/map/board host only ever forwards pointer and camera
 * streams, whose refusals must be logged, never toasted. Memoised per underlying callback so a host's own
 * `useCallback([onAction])` chains (Canvas2dHost rebuilds its canvas session on `dispatch` identity) stay
 * stable across renders. A stamp the host already set (a tutorial replay through a host) wins. */
const gestureOnActionByFunnel = new WeakMap<ComponentSceneHostProps["onAction"], ComponentSceneHostProps["onAction"]>();
function gestureOnActionFor(onAction: ComponentSceneHostProps["onAction"]): ComponentSceneHostProps["onAction"] {
  let wrapped = gestureOnActionByFunnel.get(onAction);
  if (wrapped === undefined) {
    wrapped = (action, lifecycle) => {
      const given = (action as { readonly provenance?: Record<string, unknown> }).provenance;
      const stamped = { ...action, provenance: { windowId: null, causedBy: null, ...given, origin: given?.origin ?? "gesture" } };
      return onAction(stamped as ActionDescriptor, lifecycle);
    };
    gestureOnActionByFunnel.set(onAction, wrapped);
  }
  return wrapped;
}

export function renderComponentSceneHost(
  record: UiNodeRecord,
  props: SurfaceProps,
  funnel: ComponentSceneHostProps["onAction"],
  surface: SurfaceId,
  requestContextMenu?: UiInterpreterContext["requestContextMenu"],
  assemble?: SurfaceSceneAssembler,
): ReactNode {
  const onAction = gestureOnActionFor(funnel);
  const node = surfacePropsToComponentSceneNode(record, props, surface, assemble);
  if (!node) {
    return (
      <p className="text-muted-foreground text-xs" data-unknown-surface-schema={props.docSchema}>
        {interpLabel("ui.common.unknownComponent")}: {props.kind}
      </p>
    );
  }
  if (props.kind === "virtual-file-system") {
    return (
      <ShellFaultBoundary boundaryId="surface-virtualFileSystem" fallbackLabel={shellLabel("ui.common.renderError")}>
        <VirtualFileSystemHost node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
      </ShellFaultBoundary>
    );
  }
  const Host = resolveComponentSceneHost(props.kind as ComponentKind);
  if (!Host) {
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {props.kind}
      </p>
    );
  }
  return (
    <ShellFaultBoundary boundaryId={`surface-${props.kind}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <Host node={node} onAction={onAction} requestContextMenu={requestContextMenu} />
    </ShellFaultBoundary>
  );
}
//#endregion SurfaceBridge
//#endregion 🔖️UiInterpreter
