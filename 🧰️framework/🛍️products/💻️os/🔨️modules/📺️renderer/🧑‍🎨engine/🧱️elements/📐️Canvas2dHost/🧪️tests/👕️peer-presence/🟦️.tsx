/** @emoji 👕️ Mounted Canvas2d peer-presence law (collab STEP 14, draw): the real host publishes the author's pointer in WORLD
 * coordinates with its camera (`🧫️fixtures/👕️peer-presence`, derived independently by the fixture's own formula), and the
 * canvas presence overlay it mounts paints a peer published at that drawing position under the author's screen point for the
 * same camera, and at the forward-mapped point (`@semio-tech/framework-replication` `canvasPointToScreen`) for another. */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { canvasPointToScreen } from "@semio-tech/framework-replication";
import fixture from "../../🧫️fixtures/👕️peer-presence/🔣️.json" with { type: "json" };
import { clearArtifactPresenceRosterV1, clearLocalPresenceWindowViewV1, collectLocalPresenceWindowViewsV1, publishArtifactPresenceRosterV1, publishLocalPresenceActorV1 } from "../../../👕️canvas-presence/🟦️.ts";

vi.mock("@semio-tech/infinite-canvas-react-renderer", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const React = await import("react");
  return {
    ...actual,
    GraphWasmCanvas: ({ className, sessionFactory }: { readonly className?: string; readonly sessionFactory: () => { setSize(width: number, height: number, ratio: number): void; dispose?(): void } }) => {
      const session = React.useMemo(sessionFactory, [sessionFactory]);
      React.useEffect(() => () => session.dispose?.(), [session]);
      return createElement("canvas", { className, "data-testid": "canvas-input" });
    },
  };
});

import { Canvas2dHost } from "../../🟦️.tsx";

type Row = (typeof fixture.rows)[number];

let observedSize: readonly number[] = [0, 0];
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

function mount(row: Row, camera: { readonly x: number; readonly y: number; readonly zoom: number } = row.camera) {
  observedSize = row.size;
  const view = render(
    createElement(Canvas2dHost, {
      node: { type: "componentScene", surfaceId: fixture.surfaceId, controllerId: "draw", componentKind: "canvas-2d", canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: "[]" } },
      onAction: () => Promise.resolve(),
    } as never),
  );
  const host = view.container.querySelector(".semio-canvas-2d-host") as HTMLDivElement;
  host.getBoundingClientRect = () => ({ x: 0, y: 0, left: 0, top: 0, right: row.size[0], bottom: row.size[1], width: row.size[0], height: row.size[1], toJSON: () => ({}) }) as DOMRect;
  return { view, host };
}

afterEach(() => {
  clearLocalPresenceWindowViewV1(fixture.surfaceId);
  clearArtifactPresenceRosterV1("local");
  cleanup();
});

describe("Canvas2dHost peer presence", () => {
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
      clearLocalPresenceWindowViewV1(fixture.surfaceId);
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
