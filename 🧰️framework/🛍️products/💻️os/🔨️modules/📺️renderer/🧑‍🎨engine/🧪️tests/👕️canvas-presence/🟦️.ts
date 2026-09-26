/** @emoji 👕️ The canvas-presence contract (`🧬️schema/👕️canvas-presence`) held against React's own board presence:
 * `Board2dHost`'s `puzzle2dScreenToWorld` publishes the fixture's views, `peersForWindow` + the canvas overlay math paint
 * its cursors, viewports and marks exactly as `CanvasPresenceOverlayV1` does, and `PEER_OVERLAY_LABELS` names them.
 * gl-matrix's `mat2d` inverse of the board transform is the third-party oracle of the publish half. The wgpu twin
 * (`🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu`) passes the same fixture in `🧪️tests/🔬️wgpu-unit`. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { mat2d, vec2 } from "gl-matrix";
import { describe, expect, test } from "vitest";
import {
  canvasPeerViewportRect,
  canvasPointToScreen,
  PEER_OVERLAY_LABELS,
  peerOverlayPath,
  peersForWindow,
  type PresencePeerInput,
} from "@semio-tech/framework-replication";
import { puzzle2dScreenToWorld } from "../../🧱️elements/🖥️Board2dHost/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "👕️canvas-presence", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "👕️canvas-presence", "🔣️.json"), "utf8"));

type Rect = readonly [number, number, number, number];
type Camera = { readonly x: number; readonly y: number; readonly zoom: number };
type PublishCase = { readonly id: string; readonly windowId: string; readonly bounds: Rect; readonly camera: Camera; readonly pointer: readonly [number, number] | null; readonly expected: unknown };
type PaintCase = {
  readonly id: string;
  readonly windowId: string;
  readonly bounds: Rect;
  readonly camera: Camera;
  readonly myActor: string;
  readonly localColor: number;
  readonly scenePath: string;
  readonly roster: readonly PresencePeerInput[];
  readonly expected: unknown;
};

const inside = (bounds: Rect, point: readonly [number, number]): boolean => point[0] >= bounds[0] && point[1] >= bounds[1] && point[0] < bounds[0] + bounds[2] && point[1] < bounds[1] + bounds[3];

const reactView = (row: PublishCase) => {
  const [bx, by, w, h] = row.bounds;
  const world = row.pointer && inside(row.bounds, row.pointer) ? puzzle2dScreenToWorld(JSON.stringify(row.camera), { w, h }, { x: row.pointer[0] - bx, y: row.pointer[1] - by }) : null;
  return {
    windowId: row.windowId,
    space: "canvas",
    kind: { kind: "canvas", x: row.camera.x, y: row.camera.y, zoom: row.camera.zoom },
    size: [w, h],
    ...(world ? { pointer: [world.x, world.y, 0] } : {}),
  };
};

const glMatrixWorld = (row: PublishCase): readonly [number, number] | null => {
  if (!row.pointer || !inside(row.bounds, row.pointer)) return null;
  const [bx, by, w, h] = row.bounds;
  const toScreen = mat2d.create();
  mat2d.translate(toScreen, toScreen, [w / 2, h / 2]);
  mat2d.scale(toScreen, toScreen, [row.camera.zoom, row.camera.zoom]);
  mat2d.translate(toScreen, toScreen, [-row.camera.x, -row.camera.y]);
  const toWorld = mat2d.invert(mat2d.create(), toScreen);
  if (!toWorld) return null;
  const world = vec2.transformMat2d(vec2.create(), [row.pointer[0] - bx, row.pointer[1] - by], toWorld);
  return [world[0], world[1]];
};

const reactOverlays = (row: PaintCase) => {
  const [bx, by, w, h] = row.bounds;
  const spec = peersForWindow(row.roster, row.windowId, "canvas", undefined, row.myActor, row.localColor);
  const cursors = spec.artifactPeers.flatMap((peer, index) => {
    if (peer.view.kind !== "canvas" || !peer.pointer) return [];
    const at = canvasPointToScreen(row.camera, [w, h], [peer.pointer[0], peer.pointer[1]]);
    const viewport = canvasPeerViewportRect(peer.view, peer.size, row.camera, [w, h]);
    return [
      {
        actor: peer.actor,
        color: peer.color ?? 0,
        at: [at[0] + bx, at[1] + by],
        viewport: [viewport[0] + bx, viewport[1] + by, viewport[2], viewport[3]],
        chip: peer.activeTool ? PEER_OVERLAY_LABELS.en.tool(peer.label, peer.activeTool) : peer.label,
        cursorPath: peerOverlayPath(row.scenePath, "Cursor", index, peer.actor),
        viewportPath: peerOverlayPath(row.scenePath, "Camera", index, peer.actor),
      },
    ];
  });
  const marks = Object.entries(spec.marks)
    .sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
    .flatMap(([domain, byId]) =>
      Object.entries(byId).flatMap(([id, list], index) =>
        list.map((mark) => ({
          actor: mark.actor,
          color: mark.color ?? 0,
          domain,
          id,
          mark: mark.selected ? "selection" : "hover",
          chip: mark.label.slice(0, 2).toUpperCase(),
          path: peerOverlayPath(row.scenePath, "Marks", index, `${domain}:${id}`),
        })),
      ),
    );
  return { cursors, marks };
};

describe("👕️ canvas presence of a board window", () => {
  test("the fixture satisfies its shared schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("React's board publishes every fixture view", () => {
    for (const row of fixture.publish as PublishCase[]) expect(reactView(row), row.id).toEqual(row.expected);
  });

  test("gl-matrix's inverse board transform is the third-party oracle of every published pointer", () => {
    for (const row of fixture.publish as PublishCase[]) {
      const expected = (row.expected as { readonly pointer?: readonly [number, number, number] }).pointer;
      const oracle = glMatrixWorld(row);
      if (!expected) {
        expect(oracle, row.id).toBeNull();
        continue;
      }
      expect(oracle, row.id).not.toBeNull();
      expect(oracle![0], row.id).toBeCloseTo(expected[0], 9);
      expect(oracle![1], row.id).toBeCloseTo(expected[1], 9);
    }
  });

  test("React's overlay derivation paints every fixture cursor, viewport and mark", () => {
    for (const row of fixture.paint as PaintCase[]) expect(reactOverlays(row), row.id).toEqual(row.expected);
  });

  test("React's peer overlay labels name every overlay in both tongues", () => {
    for (const row of fixture.labels as Array<{ locale: "en" | "de"; name: string; tool: string; cursor: string; viewport: string; selection: string; hover: string; chip: string }>) {
      const labels = PEER_OVERLAY_LABELS[row.locale];
      expect([labels.cursor(row.name), labels.viewport(row.name), labels.selection(row.name), labels.hover(row.name), labels.tool(row.name, row.tool)]).toEqual([row.cursor, row.viewport, row.selection, row.hover, row.chip]);
    }
  });
});
