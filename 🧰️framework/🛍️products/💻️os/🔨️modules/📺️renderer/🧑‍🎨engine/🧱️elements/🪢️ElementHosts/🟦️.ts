/** 🪢️ `🪢️ElementHosts` — the composition root of the concrete element hosts: it registers every scene host, the extension hosts and the document selection with the Interpreter's element-host registry, so the Interpreter itself imports no host.
 *
 * Roots that mount retained documents (the React target, the shell host) call {@link registerElementHostsV1} at module load. Tests that mount a surface through the Interpreter call it too and retire it with the returned function.
 *
 * @see ../🗣️Interpreter/🗺️element-host-registry/🟦️.ts
 */

import { registerComponentSceneHostV1, registerExtensionHostV1, registerLeftoverSelectionPortV1 } from "../🗣️Interpreter/🗺️element-host-registry/🟦️.ts";
import { Board2dHost } from "../🖥️Board2dHost/🟦️.tsx";
import { BlockListHost } from "../🧩️BlockListHost/🟦️.tsx";
import { Canvas2dHost } from "../📐️Canvas2dHost/🟦️.tsx";
import { DiffViewHost } from "../🔺️DiffViewHost/🟦️.tsx";
import { EventFeedHost } from "../📡️EventFeedHost/🟦️.tsx";
import { GraphTimelineHost } from "../🌳️GraphTimelineHost/🟦️.tsx";
import { IconRenderHost } from "../🖼️IconRenderHost/🟦️.tsx";
import { InkCanvasHost } from "../🖋️InkCanvasHost/🟦️.tsx";
import { MEDIA_TRANSPORT_EXTENSION_ID, MediaTransportHost } from "../🎬️MediaTransportHost/🟦️.tsx";
import { NodeGraphHost } from "../🕸️NodeGraph/🟦️.tsx";
import { Paint2dHost } from "../🖌️Paint2dHost/🟦️.tsx";
import { TableHost } from "../📊️Table/🟦️.tsx";
import { TextEditorHost } from "../✏️TextEditor/🟦️.tsx";
import { TiledMapHost } from "../🧭️TiledMapHost/🟦️.tsx";
import { World3dHost, leftoverTreeItemSelectedV1, leftoverWorldSelectionOverlayV1, subscribeLeftoverWorldSelectionV1 } from "../🌐️World3dHost/🟦️.tsx";

function mediaTransportExtensionHost(extensionId: string): typeof MediaTransportHost | undefined {
  return extensionId === MEDIA_TRANSPORT_EXTENSION_ID ? MediaTransportHost : undefined;
}

/** 🪢️ Registers every concrete element host with the Interpreter and returns the function that retires exactly these registrations; asking again replaces the scene hosts and the document selection and adds no second extension resolver. Every host and constant is read when a surface renders, so a root may call this while the host modules are still evaluating. */
export function registerElementHostsV1(): () => void {
  const retire = [
    registerComponentSceneHostV1("canvas-2d", () => Canvas2dHost),
    registerComponentSceneHostV1("world-3d", () => World3dHost),
    registerComponentSceneHostV1("node-graph", () => NodeGraphHost),
    registerComponentSceneHostV1("text-editor", () => TextEditorHost),
    registerComponentSceneHostV1("table", () => TableHost),
    registerComponentSceneHostV1("paint-2d", () => Paint2dHost),
    registerComponentSceneHostV1("tiled-map", () => TiledMapHost),
    registerComponentSceneHostV1("board-2d", () => Board2dHost),
    registerComponentSceneHostV1("icon-render", () => IconRenderHost),
    registerComponentSceneHostV1("ink-canvas", () => InkCanvasHost),
    registerComponentSceneHostV1("graph-timeline", () => GraphTimelineHost),
    registerComponentSceneHostV1("block-list", () => BlockListHost),
    registerComponentSceneHostV1("diff-view", () => DiffViewHost),
    registerComponentSceneHostV1("event-feed", () => EventFeedHost),
    registerExtensionHostV1(mediaTransportExtensionHost),
    registerLeftoverSelectionPortV1({
      subscribe: (listener) => subscribeLeftoverWorldSelectionV1(listener),
      overlay: () => leftoverWorldSelectionOverlayV1(),
      itemSelected: (itemId, leftoverIds) => leftoverTreeItemSelectedV1(itemId, leftoverIds),
    }),
  ];
  return () => {
    for (const retireOne of retire) retireOne();
  };
}
