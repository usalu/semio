import type { BlockListScene } from "../🧬️schema/🧩️block-list/🟦️.ts";
import type { ActionDescriptor } from "../../../🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts";
import type { ContextMenuItemSpec, UiContextMenuRequest, UiMenuRef } from "../📋️context-menu/🟦️.ts";
import type { Canvas2dScene } from "../🟦️.ts";
import type { World3dScene } from "../🌐️world3d/🟦️.ts";
import type { NodeGraphScene, TextEditorScene } from "../🕸️node-graph/🟦️.ts";
import type { Board2dScene, DiffViewScene, EventFeedScene, GraphTimelineScene, IconRenderScene, InkCanvasScene, Paint2dScene, TableScene, TiledMapScene, VirtualFileSystemScene } from "../📄️documents/🟦️.ts";

/** 🔌️ A plugin-contributed external body rendered inline — mirrors the wasm `externalSlot` node. */
export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
  readonly hostStatus?: string;
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
   * overflowed (measured 2026-09-09: 252 polling ticks in 35 s, 38 rejected with `queue is full`).
   * A host with nothing to await may still return `void`. The React shell resolves it with the input
   * ledger's typed outcome (`applied | refused | superseded`), which is why the value is `unknown`. */
  readonly onAction: (action: ActionDescriptor, lifecycle?: ActionDispatchLifecycleV1) => void | Promise<unknown>;
  readonly requestContextMenu?: (request: UiContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};

/** ⏳️ One exact admitted typed operation exposed to the initiating control without widening its authority. */
export type ActionOperationControlV1 = Readonly<{
  operationId: string;
  generation: string;
  cancel: () => void | Promise<unknown>;
}>;

/** 📨️ Optional action admission observer; terminal settlement remains the `onAction` return value. */
export type ActionDispatchLifecycleV1 = Readonly<{
  started: (operation: ActionOperationControlV1) => void;
}>;
