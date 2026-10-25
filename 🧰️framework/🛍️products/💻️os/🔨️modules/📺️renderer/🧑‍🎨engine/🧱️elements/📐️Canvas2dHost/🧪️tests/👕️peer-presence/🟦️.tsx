/** 👕️ Mounted Canvas2d peer-presence law (collab STEP 14, draw): the real host publishes the author's pointer in WORLD
 * coordinates with its camera (`🧫️fixtures/👕️peer-presence`, derived independently by the fixture's own formula), and the
 * canvas presence overlay it mounts paints a peer published at that drawing position under the author's screen point for the
 * same camera, and at the forward-mapped point (`@semio-tech/framework-replication` `canvasPointToScreen`) for another. */
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { canvasPointToScreen } from "@semio-tech/framework/replication";
import fixture from "../🧫️fixtures/👕️peer-presence/🔣️.json" with { type: "json" };
import { clearArtifactPresenceRosterV1, clearLocalPresenceWindowViewV1, collectLocalPresenceWindowViewsV1, publishArtifactPresenceRosterV1, publishLocalPresenceActorV1 } from "../../../👕️canvas-presence/🟦️.ts";
import { LocalDocumentOwnerContext, LocalDocumentOwnerRegistryV1 } from "../../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";

vi.mock("@semio-tech/canvas-react-renderer", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const React = await import("react");
  return {
    ...actual,
    WasmCanvas: ({ className, sessionFactory }: { readonly className?: string; readonly sessionFactory: () => { setSize(width: number, height: number, ratio: number): void; dispose?(): void } }) => {
      const session = React.useMemo(sessionFactory, [sessionFactory]);
      React.useEffect(() => () => session.dispose?.(), [session]);
      return createElement("canvas", { className, "data-testid": "canvas-input" });
    },
  };
});

import { Canvas2dHost } from "../../🟦️.tsx";

type Row = (typeof fixture.rows)[number];

let observedSize: readonly number[] = [0, 0];
const owners = new LocalDocumentOwnerRegistryV1();
vi.stubGlobal(
  "ResizeObserver",
  class {
    constructor(private readonly callback: (entries: readonly { readonly contentRect: { readonly width: number; readonly height: number } }[]) => void) {}
    observe(): void {
      this.callback([{ contentRect: { width: observedSize[0]!, height: observedSize[1]! } }]);
    }
    unobserve(): void {}
    disconnect(): void {}
  },
);

function mount(row: Row, camera: { readonly x: number; readonly y: number; readonly zoom: number } = row.camera, runtimeKey = "local") {
  observedSize = row.size;
  const view = render(
    createElement(LocalDocumentOwnerContext.Provider, { value: owners.acquire({ pluginId: "fem", appId: "fem2d", sessionInstanceId: 1, runtimeKey, clientInstanceId: runtimeKey }) }, createElement(Canvas2dHost, {
      node: { type: "componentScene", surfaceId: fixture.surfaceId, controllerId: "draw", componentKind: "canvas-2d", canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: "[]" } },
      onAction: () => Promise.resolve(),
    } as never)),
  );
  const host = view.container.querySelector(".semio-canvas-2d-host") as HTMLDivElement;
  host.getBoundingClientRect = () => ({ x: 0, y: 0, left: 0, top: 0, right: row.size[0], bottom: row.size[1], width: row.size[0], height: row.size[1], toJSON: () => ({}) }) as DOMRect;
  return { view, host };
}

afterEach(() => {
  clearLocalPresenceWindowViewV1(fixture.surfaceId, "local");
  owners.retireAll();
  clearArtifactPresenceRosterV1("local");
  cleanup();
});

describe("Canvas2dHost peer presence", () => {
  it("keeps mounted same-id windows, camera publications and peer paint on their exact document owners", () => {
    const row = fixture.rows[0]!;
    const documents = ["mounted-presence-a", "mounted-presence-b"];
    for (const [index, document] of documents.entries()) {
      publishLocalPresenceActorV1(document, `self-${index}`);
      publishArtifactPresenceRosterV1(document, [{ actor: `peer-${index}`, label: `Peer ${index}`, color: index + 2, connectedAtMs: 1, views: [{ windowId: fixture.surfaceId, space: "canvas", kind: { kind: "canvas", ...row.camera }, size: row.size as [number, number], pointer: [row.world[0], row.world[1], 0] }] }] as never);
    }
    const left = mount(row, row.camera, documents[0]);
    const right = mount(row, fixture.peerCamera, documents[1]);
    fireEvent.pointerMove(left.host, { clientX: row.client[0], clientY: row.client[1] });
    fireEvent.pointerMove(right.host, { clientX: row.client[0], clientY: row.client[1] });
    expect(collectLocalPresenceWindowViewsV1(documents[0])[0]?.kind).toEqual({ kind: "canvas", ...row.camera });
    expect(collectLocalPresenceWindowViewsV1(documents[1])[0]?.kind).toEqual({ kind: "canvas", ...fixture.peerCamera });
    expect(left.view.container.querySelector('[data-peer-actor="peer-0"]')).not.toBeNull();
    expect(left.view.container.querySelector('[data-peer-actor="peer-1"]')).toBeNull();
    expect(right.view.container.querySelector('[data-peer-actor="peer-1"]')).not.toBeNull();
    expect(right.view.container.querySelector('[data-peer-actor="peer-0"]')).toBeNull();
    left.view.unmount();
    expect(collectLocalPresenceWindowViewsV1(documents[0])).toEqual([]);
    expect(collectLocalPresenceWindowViewsV1(documents[1])).toHaveLength(1);
    act(() => owners.retireAll());
    expect(collectLocalPresenceWindowViewsV1(documents[1])).toEqual([]);
    expect(right.view.container.querySelector('[data-peer-actor="peer-1"]')).toBeNull();
    right.view.unmount();
    for (const document of documents) { clearArtifactPresenceRosterV1(document); publishLocalPresenceActorV1(document, null); }
  });

  it("publishes the author's pointer at the fixture's drawing position with the author's camera", () => {
    for (const row of fixture.rows) {
      const { host, view } = mount(row);
      fireEvent.pointerMove(host, { clientX: row.client[0], clientY: row.client[1] });
      const published = collectLocalPresenceWindowViewsV1("local").find((candidate) => candidate.windowId === fixture.surfaceId);
      expect(published?.kind, row.id).toEqual({ kind: "canvas", ...row.camera });
      expect(published?.size, row.id).toEqual(row.size);
      expect(published?.pointer?.[0], row.id).toBeCloseTo(row.world[0], 9);
      expect(published?.pointer?.[1], row.id).toBeCloseTo(row.world[1], 9);
      host.dispatchEvent(new MouseEvent("pointerout", { bubbles: true, relatedTarget: document.body }));
      expect(collectLocalPresenceWindowViewsV1("local").find((candidate) => candidate.windowId === fixture.surfaceId)?.pointer, row.id).toBeUndefined();
      view.unmount();
      clearLocalPresenceWindowViewV1(fixture.surfaceId, "local");
    }
  });

  it("paints a peer's pointer at its drawing position for the viewer's own camera", () => {
    publishLocalPresenceActorV1("local", "hub.v1.viewer");
    for (const row of fixture.rows) {
      for (const camera of [row.camera, fixture.peerCamera]) {
        publishArtifactPresenceRosterV1("local", [{ actor: "hub.v1.author", label: "Author", color: 3, connectedAtMs: 1, views: [{ windowId: fixture.surfaceId, space: "canvas", kind: { kind: "canvas", ...row.camera }, size: row.size as [number, number], pointer: [row.world[0], row.world[1], 0] }] }] as never);
        const { view } = mount(row, camera);
        const cursor = view.container.querySelector('[data-peer-cursor][data-peer-actor="hub.v1.author"]') as HTMLElement | null;
        expect(cursor, `${row.id} ${JSON.stringify(camera)}`).not.toBeNull();
        const [x, y] = camera === row.camera ? row.client : canvasPointToScreen(camera, row.size as [number, number], [row.world[0], row.world[1]]);
        expect(Number.parseFloat(cursor!.style.left), row.id).toBeCloseTo(x, 6);
        expect(Number.parseFloat(cursor!.style.top), row.id).toBeCloseTo(y, 6);
        view.unmount();
      }
    }
  });
});
