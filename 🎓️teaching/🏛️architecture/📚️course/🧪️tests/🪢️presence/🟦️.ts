import { EventEmitter, once } from "node:events";
import { createRequire } from "node:module";
import { createServer, type Socket } from "node:net";
import type { AddressInfo } from "node:net";
import { expect, it } from "vitest";
import cases from "../../🧫️fixtures/🪢️presence/🔣️.json" with { type: "json" };
import { presenceUpgradeStatus, waitForPresenceSocket } from "../../🚀️deploy/🩺️docker-image-check/🟦️.ts";

type OracleSocket = EventEmitter & { close(): void; terminate(): void };
type OracleResponse = { statusCode?: number; destroy(): void };
type OracleServer = EventEmitter & { address(): AddressInfo; readonly clients: Set<OracleSocket>; close(callback: () => void): void };
/** 🧩 The installed `ws` client. Vitest rewrites a bare `ws` specifier into a socket whose handshake deadline never fires. */
function loadWsOracle(): { WebSocket: new (url: string, options: { handshakeTimeout: number; signal: AbortSignal }) => OracleSocket; WebSocketServer: new (options: { host: string; port: number }) => OracleServer } {
  const nodeRequire = createRequire(import.meta.url);
  return nodeRequire(nodeRequire.resolve("ws/package.json").replace(/package\.json$/u, "index.js"));
}
const { WebSocket: Ws, WebSocketServer } = loadWsOracle();

for (const row of cases.cases) it(`settles the ${row.id} presence handshake like ws`, async () => {
  const connections = new Set<Socket>();
  const server = row.server === "open" ? new WebSocketServer({ host: "127.0.0.1", port: 0 }) : createServer(socket => {
    connections.add(socket);
    socket.once("close", () => connections.delete(socket));
    if (row.server === "refuse") socket.end("HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n");
  }).listen(0, "127.0.0.1");
  await once(server, "listening");
  const origin = `ws://127.0.0.1:${(server.address() as AddressInfo).port}`;
  const outcome = (error: unknown): string => error instanceof Error && error.name === "AbortError" ? "aborted" : /timed out|handshake.*within/iu.test(String(error)) ? "timed-out" : "failed";
  let oracleUpgrade: number | string | undefined;
  const run = async (oracle: boolean): Promise<string> => {
    const controller = new AbortController();
    if (row.abort === "before") controller.abort();
    const timer = row.abort === "later" ? setTimeout(() => controller.abort(), 10) : undefined;
    let socket: WebSocket | OracleSocket | undefined;
    try {
      if (oracle) return await new Promise<string>(resolve => {
        const client = new Ws(origin, { handshakeTimeout: row.timeoutMs, signal: controller.signal });
        socket = client;
        client.once("open", () => { oracleUpgrade = 101; resolve("opened"); });
        client.once("unexpected-response", (_: unknown, response: OracleResponse) => { oracleUpgrade = response.statusCode; response.destroy(); client.terminate(); resolve("failed"); });
        client.once("error", (error: Error) => { oracleUpgrade ??= outcome(error); resolve(outcome(error)); });
      });
      const client = new WebSocket(origin);
      socket = client;
      await waitForPresenceSocket(client, controller.signal, row.timeoutMs);
      return "opened";
    } catch (error) { return outcome(error); }
    finally { clearTimeout(timer); socket?.close(); }
  };
  try {
    expect(await run(false)).toBe(row.expected);
    expect(await run(true)).toBe(row.expected);
    const controller = new AbortController();
    if (row.abort === "before") controller.abort();
    const timer = row.abort === "later" ? setTimeout(() => controller.abort(), 10) : undefined;
    try {
      const result = await presenceUpgradeStatus(origin, { origin: "https://foreign.example" }, false, controller.signal, row.timeoutMs).catch(outcome);
      expect(result).toBe(row.upgrade);
      expect(result).toBe(oracleUpgrade);
    } finally { clearTimeout(timer); }
  } finally {
    for (const socket of connections) socket.destroy();
    if (server instanceof WebSocketServer) for (const socket of server.clients) socket.terminate();
    await new Promise<void>(resolve => server.close(() => resolve()));
  }
}, 5000);

import "./👆️swipe-progress/🟦️.ts";
