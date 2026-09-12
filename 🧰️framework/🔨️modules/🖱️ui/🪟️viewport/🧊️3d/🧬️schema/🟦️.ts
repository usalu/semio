import { viewportNumber, viewportRecord, viewportZoom } from "../../🟦️.ts";

/** 🌐️ Orbit navigation; projection and authored cameras have separate owners. */
export interface Viewport3dOrbit { position: [number, number, number]; target: [number, number, number]; zoom: number; up?: [number, number, number] }

function vector(value: unknown): [number, number, number] {
  if (!Array.isArray(value) || value.length !== 3) throw new TypeError("Expected three viewport coordinates");
  return [viewportNumber(value[0]), viewportNumber(value[1]), viewportNumber(value[2])];
}

/** 📥️ Admits the closed orbit navigation schema emitted by renderer gestures. */
export function parseViewport3dOrbit(value: unknown): Viewport3dOrbit {
  const record = viewportRecord(value, ["position", "target", "zoom"], ["up"]);
  return { position: vector(record.position), target: vector(record.target), zoom: viewportZoom(record.zoom), ...(Object.hasOwn(record, "up") ? { up: vector(record.up) } : {}) };
}
