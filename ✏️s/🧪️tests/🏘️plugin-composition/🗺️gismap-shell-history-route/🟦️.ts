/** 🗺️ The Shell undo arbitration agrees with the GIS map approval-history corpus; the law needs the plugin corpus and the renderer Shell, so it lives where plugins may meet. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { shellHistoryUndoRouteV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx";

const corpus = new URL("../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🧫️fixtures/↩️gis-map-approval-history-v1/🔣️.json", import.meta.url);

test("Shell history arbitration routes undo exactly as the neutral approval-history corpus", () => {
  const { ordinaryHistory } = JSON.parse(readFileSync(corpus, "utf8"));
  expect({
    remoteNewerThanLocal: shellHistoryUndoRouteV1({ phase: "available", canUndo: true, order: 2 }, { canUndo: true, order: 1 }),
    localNewerThanRemote: shellHistoryUndoRouteV1({ phase: "available", canUndo: true, order: 1 }, { canUndo: true, order: 2 }),
    remoteSubmitting: shellHistoryUndoRouteV1({ phase: "submitting", canUndo: false, order: 2 }, { canUndo: true, order: 1 }),
    noRemote: shellHistoryUndoRouteV1(null, { canUndo: true, order: 1 }),
  }).toEqual(ordinaryHistory);
});
