import { FlowServer, type FlowMessage } from "./🟦️.ts";

const server = new FlowServer((reply) => globalThis.postMessage(reply));
globalThis.onmessage = ({ data }: MessageEvent<FlowMessage>) => server.post(data);
