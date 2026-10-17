/** 🗺️ The Interpreter reaches every concrete element host only through its registry: nothing is mounted until the composition root registers it, a retired registration falls back to the visible placeholder, and `🪢️ElementHosts` registers exactly the fourteen scene hosts, the media transport extension and the document selection.
 *
 * Independent oracle: the host identities are compared against the host modules' own exports, and the mounted surface is read back from the DOM, not from the registry.
 */
import { createElement, type ReactElement } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import type { ComponentKind, ComponentSceneHostProps, Component, UiNodeRecord } from "@semio-tech/framework";
import { UiDocumentStore } from "../../../📃️UiDocumentStore/🟦️.tsx";
import { registerElementHostsV1 } from "../../../🪢️ElementHosts/🟦️.ts";
import { Board2dHost } from "../../../🖥️Board2dHost/🟦️.tsx";
import { BlockListHost } from "../../../🧩️BlockListHost/🟦️.tsx";
import { Canvas2dHost } from "../../../📐️Canvas2dHost/🟦️.tsx";
import { DiffViewHost } from "../../../🔺️DiffViewHost/🟦️.tsx";
import { EventFeedHost } from "../../../📡️EventFeedHost/🟦️.tsx";
import { GraphTimelineHost } from "../../../🌳️GraphTimelineHost/🟦️.tsx";
import { IconRenderHost } from "../../../🖼️IconRenderHost/🟦️.tsx";
import { InkCanvasHost } from "../../../🖋️InkCanvasHost/🟦️.tsx";
import { MEDIA_TRANSPORT_EXTENSION_ID, MediaTransportHost } from "../../../🎬️MediaTransportHost/🟦️.tsx";
import { NodeGraphHost } from "../../../🕸️NodeGraph/🟦️.tsx";
import { Paint2dHost } from "../../../🖌️Paint2dHost/🟦️.tsx";
import { TableHost } from "../../../📊️Table/🟦️.tsx";
import { TextEditorHost } from "../../../✏️TextEditor/🟦️.tsx";
import { TiledMapHost } from "../../../🧭️TiledMapHost/🟦️.tsx";
import { World3dHost } from "../../../🌐️World3dHost/🟦️.tsx";
import { leftoverTreeItemSelectedV1, leftoverWorldSelectionOverlayV1, publishLeftoverWorldSelectionV1 } from "../../../🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { UiNodeView } from "../../🧱️node-tree/🟦️.tsx";
import {
  leftoverItemSelectedV1,
  leftoverSelectionOverlayV1,
  registerComponentSceneHostV1,
  registerExtensionHostV1,
  registerLeftoverSelectionPortV1,
  resolveComponentSceneHost,
  resolveExtensionHost,
  subscribeLeftoverSelectionV1,
} from "../../🗺️element-host-registry/🟦️.ts";

const SURFACE = "panel:hosts";
const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;
const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

const HOSTS: readonly (readonly [ComponentKind, unknown])[] = [
  ["canvas-2d", Canvas2dHost],
  ["world-3d", World3dHost],
  ["node-graph", NodeGraphHost],
  ["text-editor", TextEditorHost],
  ["table", TableHost],
  ["paint-2d", Paint2dHost],
  ["tiled-map", TiledMapHost],
  ["board-2d", Board2dHost],
  ["icon-render", IconRenderHost],
  ["ink-canvas", InkCanvasHost],
  ["graph-timeline", GraphTimelineHost],
  ["block-list", BlockListHost],
  ["diff-view", DiffViewHost],
  ["event-feed", EventFeedHost],
];

function surfaceRecord(kind: ComponentKind): UiNodeRecord {
  const component = { type: "surface", kind, docSchema: "probe.surface@1", doc: { bytes: [] }, bindings: [] } as unknown as Component;
  return { id: 1, key: `surface-${kind}`, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: STYLE, activity: "idle", disabled: false, transition: null, accessibility: ACCESSIBILITY, bindings: [], menu: null, children: [] } as unknown as UiNodeRecord;
}

function mountSurface(kind: ComponentKind): ReturnType<typeof render> {
  const store = new UiDocumentStore(SURFACE);
  store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [surfaceRecord(kind)] } as never);
  const element: ReactElement = createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: () => {} } });
  return render(element);
}

function clearRegistry(): void {
  registerElementHostsV1()();
}

describe("🪢️ importing a composition root registers the concrete hosts", () => {
  afterEach(() => cleanup());

  it("has registered every scene kind, the media transport extension and the document selection by the time the shell module is loaded", () => {
    for (const [kind, host] of HOSTS) expect(resolveComponentSceneHost(kind), kind).toBe(host);
    expect(resolveExtensionHost(MEDIA_TRANSPORT_EXTENSION_ID)).toBe(MediaTransportHost);
    expect(leftoverSelectionOverlayV1()).toBe(leftoverWorldSelectionOverlayV1());
  });

  it("mounts the real host of a surface through the Interpreter with no probe in between", () => {
    const view = mountSurface("diff-view");
    expect(view.container.querySelector("[data-probe-host]")).toBeNull();
    expect(view.container.textContent ?? "").not.toMatch(/Unknown component/iu);
    view.unmount();
  });
});

describe("🗺️ the Interpreter reaches element hosts only through its registry", () => {
  afterEach(() => cleanup());

  it("mounts nothing for a kind no one registered and says so visibly, never rendering blank", () => {
    clearRegistry();
    const view = mountSurface("diff-view");
    expect(resolveComponentSceneHost("diff-view")).toBeUndefined();
    expect(view.container.textContent).toMatch(/diff-view/u);
    expect(view.container.querySelector("[data-probe-host]")).toBeNull();
  });

  it("mounts the host a root registered with the node it assembled, and falls back once that registration is retired", () => {
    clearRegistry();
    const seen: ComponentSceneHostProps[] = [];
    const retire = registerComponentSceneHostV1("diff-view", () => (props: ComponentSceneHostProps) => {
      seen.push(props);
      return createElement("div", { "data-probe-host": props.node.componentKind });
    });
    const view = mountSurface("diff-view");
    expect(view.container.querySelector("[data-probe-host]")?.getAttribute("data-probe-host")).toBe("diff-view");
    expect(seen).toHaveLength(1);
    expect(seen[0]!.node.type).toBe("componentScene");
    expect(typeof seen[0]!.onAction).toBe("function");
    view.unmount();
    retire();
    expect(resolveComponentSceneHost("diff-view")).toBeUndefined();
    expect(mountSurface("diff-view").container.querySelector("[data-probe-host]")).toBeNull();
  });

  it("lets the newest registration win and retires only its own", () => {
    clearRegistry();
    const secondHost = () => createElement("b");
    const first = () => () => createElement("i");
    const second = () => secondHost;
    const retireFirst = registerComponentSceneHostV1("block-list", first);
    const retireSecond = registerComponentSceneHostV1("block-list", second);
    retireFirst();
    expect(resolveComponentSceneHost("block-list")).toBe(secondHost);
    retireSecond();
    expect(resolveComponentSceneHost("block-list")).toBeUndefined();
  });

  it("resolves a host at render time, so a registration made before the component exists still mounts it", () => {
    clearRegistry();
    let live: ((props: ComponentSceneHostProps) => ReactElement) | undefined;
    const retire = registerComponentSceneHostV1("event-feed", () => live);
    expect(resolveComponentSceneHost("event-feed")).toBeUndefined();
    live = () => createElement("div", { "data-probe-host": "late" });
    expect(mountSurface("event-feed").container.querySelector("[data-probe-host]")?.getAttribute("data-probe-host")).toBe("late");
    retire();
  });

  it("answers an extension id with the registered host only", () => {
    clearRegistry();
    const probe = () => createElement("div");
    const retire = registerExtensionHostV1((extensionId) => (extensionId === "probe.extension" ? probe : undefined));
    expect(resolveExtensionHost("probe.extension")).toBe(probe);
    expect(resolveExtensionHost("other.extension")).toBeUndefined();
    retire();
    expect(resolveExtensionHost("probe.extension")).toBeUndefined();
  });

  it("has no document selection until a root supplies one, and a retired port reads as no selection", () => {
    clearRegistry();
    expect(leftoverSelectionOverlayV1()).toBeNull();
    expect(leftoverItemSelectedV1("a", ["a"])).toBe(false);
    let listeners = 0;
    const retire = registerLeftoverSelectionPortV1({
      subscribe: () => {
        listeners += 1;
        return () => {
          listeners -= 1;
        };
      },
      overlay: () => ({ ids: ["a"] }),
      itemSelected: (itemId, ids) => ids?.includes(itemId) === true,
    });
    const stop = subscribeLeftoverSelectionV1(() => {});
    expect(listeners).toBe(1);
    expect(leftoverSelectionOverlayV1()?.ids).toEqual(["a"]);
    expect(leftoverItemSelectedV1("a", ["a"])).toBe(true);
    stop();
    expect(listeners).toBe(0);
    retire();
    expect(leftoverSelectionOverlayV1()).toBeNull();
  });
});

describe("🪢️ the element-host composition root registers and retires exactly the concrete hosts", () => {
  afterEach(() => cleanup());

  it("registers each of the fourteen scene kinds with the host module's own component, and retires them all", () => {
    const retire = registerElementHostsV1();
    for (const [kind, host] of HOSTS) expect(resolveComponentSceneHost(kind), kind).toBe(host);
    expect(resolveComponentSceneHost("virtual-file-system" as ComponentKind)).toBeUndefined();
    retire();
    for (const [kind] of HOSTS) expect(resolveComponentSceneHost(kind), kind).toBeUndefined();
  });

  it("registers the media transport extension host and no other extension", () => {
    const retire = registerElementHostsV1();
    expect(resolveExtensionHost(MEDIA_TRANSPORT_EXTENSION_ID)).toBe(MediaTransportHost);
    expect(resolveExtensionHost("unknown.extension")).toBeUndefined();
    retire();
    expect(resolveExtensionHost(MEDIA_TRANSPORT_EXTENSION_ID)).toBeUndefined();
  });

  it("hands the retained tree the world's document selection, subscription and row rule", () => {
    const retire = registerElementHostsV1();
    let notified = 0;
    const stop = subscribeLeftoverSelectionV1(() => {
      notified += 1;
    });
    publishLeftoverWorldSelectionV1({ ids: ["seed-left-001"], hoveredId: null, gumballActive: false, gumballAnchorId: null }, { kind: "document" });
    expect(notified).toBeGreaterThan(0);
    expect(leftoverSelectionOverlayV1()).toBe(leftoverWorldSelectionOverlayV1());
    expect(leftoverSelectionOverlayV1()?.ids).toEqual(["seed-left-001"]);
    expect(leftoverItemSelectedV1("surface/seed-left-001", ["seed-left-001"])).toBe(leftoverTreeItemSelectedV1("surface/seed-left-001", ["seed-left-001"]));
    expect(leftoverItemSelectedV1("surface/seed-left-001", ["seed-left-001"])).toBe(true);
    expect(leftoverItemSelectedV1("other", ["seed-left-001"])).toBe(false);
    stop();
    publishLeftoverWorldSelectionV1(null, { kind: "document" });
    retire();
  });
});
