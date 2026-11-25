import { FlowServer } from "./🟦️.ts";
import { flowWireIn, type FlowWire } from "./wire.ts";

const server = new FlowServer((reply) => globalThis.postMessage(reply));
globalThis.onmessage = ({ data }: MessageEvent<FlowWire>) => server.post(flowWireIn(data));
