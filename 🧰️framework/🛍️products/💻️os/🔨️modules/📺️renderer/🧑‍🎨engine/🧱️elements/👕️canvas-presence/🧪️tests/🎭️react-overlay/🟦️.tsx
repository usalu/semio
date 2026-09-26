// #region 🧲️Header
/** @emoji 🎭️ React's canvas presence overlay, rendered to the DOM from the shared canvas-presence fixture (`🧫️fixtures/
 * 👕️canvas-presence`, the wgpu twin's corpus): with the domain the board's app DECLARES (block's `handle`) it paints
 * exactly the fixture's peer marks and cursor; with any other domain — the literal `layer` Board2dHost used to pass — it
 * paints no mark at all (🎫️ 26/09/23 C10, WG8 relay). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import type { PresencePeerInput } from "@semio-tech/framework-replication";
import { CanvasPresenceOverlayV1 } from "../../🟦️.tsx";
import { clearArtifactPresenceRosterV1, publishArtifactPresenceRosterV1 } from "../../🟦️.ts";
import fixture from "../../../../🧫️fixtures/👕️canvas-presence/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type Mark = { readonly actor: string; readonly color: number; readonly domain: string; readonly id: string; readonly mark: string; readonly path: string };
type PaintCase = {
  readonly id: string;
  readonly windowId: string;
  readonly bounds: readonly [number, number, number, number];
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly myActor: string;
  readonly localColor: number;
  readonly scenePath: string;
  readonly roster: readonly PresencePeerInput[];
  readonly expected: { readonly cursors: readonly { readonly actor: string; readonly cursorPath: string }[]; readonly marks: readonly Mark[] };
};
const cases = (fixture as unknown as { readonly paint: readonly PaintCase[] }).paint;
const RUNTIME = "canvas-presence-react-overlay";

function paint(row: PaintCase, domain: string | undefined) {
  publishArtifactPresenceRosterV1(RUNTIME, row.roster as never);
  const view = render(
    createElement(CanvasPresenceOverlayV1, {
      runtimeKey: RUNTIME,
      windowId: row.windowId,
      space: "canvas",
      myActor: row.myActor,
      locale: "en",
      localColor: row.localColor,
      localCanvas: row.camera,
      localSizePx: [row.bounds[2], row.bounds[3]],
      domain,
      scenePath: row.scenePath,
    }),
  );
  const marks = [...view.container.querySelectorAll("[data-peer-marks]")].map((element) => ({ actor: element.getAttribute("data-peer-actor"), color: Number(element.getAttribute("data-peer-color")), mark: element.getAttribute("data-peer-mark"), path: element.getAttribute("data-ui-path") }));
  const cursors = [...view.container.querySelectorAll("[data-peer-cursor]")].map((element) => ({ actor: element.getAttribute("data-peer-actor"), path: element.getAttribute("data-ui-path") }));
  return { marks, cursors };
}

const byPath = <T extends { readonly path: string | null; readonly actor: string | null }>(rows: readonly T[]): T[] => [...rows].sort((left, right) => `${left.path}|${left.actor}`.localeCompare(`${right.path}|${right.actor}`));

describe("👕️ React canvas presence overlay", () => {
  afterEach(() => {
    cleanup();
    clearArtifactPresenceRosterV1(RUNTIME);
  });

  it("paints every fixture mark and cursor in the domain the app declares", () => {
    for (const row of cases) {
      const domain = row.expected.marks[0]?.domain;
      expect(domain, row.id).toBeDefined();
      const painted = paint(row, domain);
      expect(byPath(painted.marks), row.id).toEqual(byPath(row.expected.marks.map((mark) => ({ actor: mark.actor, color: mark.color, mark: mark.mark, path: mark.path }))));
      expect(byPath(painted.cursors), row.id).toEqual(byPath(row.expected.cursors.map((cursor) => ({ actor: cursor.actor, path: cursor.cursorPath }))));
    }
  });

  it("paints no mark for a domain the app does not declare", () => {
    for (const row of cases) {
      expect(paint(row, "layer").marks, row.id).toEqual([]);
      cleanup();
      expect(paint(row, undefined).marks, row.id).toEqual([]);
    }
  });
});
//#endregion 🧪️Laws
