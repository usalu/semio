import { drawingBench, type DrawingRequest } from "./🟦️.ts";

const answer = drawingBench();
globalThis.onmessage = ({ data }: MessageEvent<DrawingRequest>) => {
  const reply = answer(data);
  if (reply !== null) globalThis.postMessage(reply);
};
