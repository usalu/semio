import type { Asset } from "@teaching/staffage";
import { drawFigure, type DrawingReply, type DrawingRequest } from "./🟦️.ts";

const assets = new Map<number, Asset>();
globalThis.onmessage = ({ data }: MessageEvent<DrawingRequest>) => {
  if (data.kind === "asset") { assets.set(data.id, data.asset); return; }
  if (data.kind === "release") { assets.delete(data.id); return; }
  let response: DrawingReply;
  try {
    const asset = assets.get(data.id);
    if (!asset) throw new Error("Drawing worker does not hold the requested asset");
    response = { lease: data.lease, drawing: drawFigure(asset, data.options, data.query, data.wrapLoops) };
  } catch (error) {
    response = { lease: data.lease, error: error instanceof Error ? error.message : String(error) };
  }
  globalThis.postMessage(response);
};
