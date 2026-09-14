//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { Readable, Writable } from "node:stream";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { ListPromptsRequestSchema, ListResourcesRequestSchema, ListToolsRequestSchema } from "@modelcontextprotocol/sdk/types.js";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

/** 📡️ A raw JSON-RPC channel to a reference server: `send` awaits one reply, `notify` expects none. */
export type ReferenceChannel = Readonly<{ send: (request: unknown) => Promise<Record<string, unknown>>; notify: (notification: unknown) => Promise<void> }>;

/** 🤝️ Starts a reference MCP server from Anthropic's TypeScript SDK and returns a raw JSON-RPC channel to it. */
export async function referenceServer(): Promise<ReferenceChannel> {
  const [clientSide, serverSide] = InMemoryTransport.createLinkedPair();
  await referenceServerInstance().connect(serverSide);
  await clientSide.start();
  return {
    send: (request: unknown) =>
      new Promise((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error("reference server did not answer")), 5000);
        clientSide.onmessage = (message: unknown) => {
          clearTimeout(timer);
          resolve(message as Record<string, unknown>);
        };
        void clientSide.send(request as never).catch(reject);
      }),
    notify: (notification: unknown) => clientSide.send(notification as never),
  };
}

/** 🤝️ Performs the fixture's handshake against the reference server and returns the raw envelope. */
async function handshake(ctx: AdapterContext): Promise<{ envelope: Record<string, unknown>; requested: string }> {
  const fixture = JSON.parse(readFileSync(ctx.fixture("shared://🤝️initialize-lenient.json"), "utf8")) as Record<string, unknown>;
  const channel = await referenceServer();
  const envelope = await channel.send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: fixture.requestedProtocolVersion, capabilities: fixture.capabilities, clientInfo: fixture.clientInfo } });
  return { envelope, requested: String(fixture.requestedProtocolVersion) };
}

/** 🔁️ Delivers the initialized notification before the initialize reply, then pings the reference server. */
async function pipelinedHandshake(ctx: AdapterContext): Promise<{ initializeAccepted: boolean; pingAnswered: boolean }> {
  const fixture = JSON.parse(readFileSync(ctx.fixture("shared://🤝️initialize-lenient.json"), "utf8")) as Record<string, unknown>;
  const channel = await referenceServer();
  await channel.notify({ jsonrpc: "2.0", method: "notifications/initialized" });
  const initialized = await channel.send({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: fixture.requestedProtocolVersion, capabilities: fixture.capabilities, clientInfo: fixture.clientInfo } });
  const ping = await channel.send({ jsonrpc: "2.0", id: 2, method: "ping" });
  return { initializeAccepted: initialized.error === undefined, pingAnswered: ping.error === undefined };
}

/** 🏭️ A reference server built on the same tool, resource and prompt surface the other scenarios use. */
function referenceServerInstance(): Server {
  const server = new Server({ name: "repo", version: "1.0.0" }, { capabilities: { tools: {}, resources: {}, prompts: {} }, instructions: "Use repository tools and resources through their owned schemas." });
  server.setRequestHandler(ListToolsRequestSchema, () => ({ tools: [] }));
  server.setRequestHandler(ListResourcesRequestSchema, () => ({ resources: [] }));
  server.setRequestHandler(ListPromptsRequestSchema, () => ({ prompts: [] }));
  return server;
}

/** 🚰️ Feeds the reference server one burst over a standard-input stream that ENDS immediately, then
 * collects every line it wrote — the `printf … | server` shape a short-lived client produces. */
async function burstThenEOF(ctx: AdapterContext): Promise<Record<string, unknown>[]> {
  const fixture = JSON.parse(readFileSync(ctx.fixture("shared://🤝️initialize-lenient.json"), "utf8")) as Record<string, unknown>;
  const burst = [
    JSON.stringify({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: fixture.requestedProtocolVersion, capabilities: fixture.capabilities, clientInfo: fixture.clientInfo } }),
    JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} }),
    JSON.stringify({ jsonrpc: "2.0", id: 2, method: "tools/list", params: {} }),
    "",
  ].join("\n");
  const written: string[] = [];
  const output = new Writable({
    write(chunk: Buffer, _encoding, callback) {
      written.push(chunk.toString("utf8"));
      callback();
    },
  });
  const input = Readable.from([Buffer.from(burst, "utf8")]);
  await referenceServerInstance().connect(new StdioServerTransport(input, output));
  const replies = (): Record<string, unknown>[] =>
    written
      .join("")
      .split("\n")
      .filter((line) => line.trim() !== "")
      .map((line) => JSON.parse(line) as Record<string, unknown>);
  for (let attempt = 0; attempt < 200 && replies().length < 2; attempt += 1) await new Promise((resolve) => setTimeout(resolve, 25));
  return replies();
}

/** 🧾️ Reduces the replies to the ids answered, whether the request that followed the initialized
 * notification carries a result, and how many replies refused an uninitialized session. */
function pipelinedBurstProjection(replies: Record<string, unknown>[]): { answered: string[]; followingRequestServed: boolean; notInitializedRefusals: number } {
  const answered = replies.map((reply) => JSON.stringify(reply.id)).sort();
  const notInitializedRefusals = replies.filter((reply) => ((reply.error ?? {}) as Record<string, unknown>).code === -32002).length;
  const followingRequestServed = replies.some((reply) => JSON.stringify(reply.id) === "2" && reply.result !== undefined);
  return { answered, followingRequestServed, notInitializedRefusals };
}

/** 🧾️ Reduces the replies to the request ids answered and the closed-session refusals among them. */
function burstProjection(replies: Record<string, unknown>[]): { answered: string[]; sessionClosedRefusals: number } {
  const answered = replies.map((reply) => JSON.stringify(reply.id)).sort();
  const sessionClosedRefusals = replies.filter((reply) => ((reply.error ?? {}) as Record<string, unknown>).code === -32004).length;
  return { answered, sessionClosedRefusals };
}

//#endregion 🧭️Oracle

//#region 🧭️Adapter

/** 🟦️ TypeScript oracle for the MCP handshake, backed by `@modelcontextprotocol/sdk`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "initialize-accepts-unknown-members": {
      oracle: async (ctx) => {
        const { envelope } = await handshake(ctx);
        const result = (envelope.result ?? {}) as Record<string, unknown>;
        const capabilities = Object.keys((result.capabilities ?? {}) as Record<string, unknown>).sort();
        return { projection: { accepted: envelope.error === undefined, serverVersion: String(((result.serverInfo ?? {}) as Record<string, unknown>).version ?? ""), capabilities } };
      },
    },
    "initialize-and-initialized-pipelined": {
      oracle: async (ctx) => ({ projection: await pipelinedHandshake(ctx) }),
    },
    "pipelined-burst-serves-requests-after-initialized": {
      oracle: async (ctx) => ({ projection: pipelinedBurstProjection(await burstThenEOF(ctx)) }),
    },
    "eof-after-burst-completes-queued-requests": {
      oracle: async (ctx) => ({ projection: burstProjection(await burstThenEOF(ctx)) }),
    },
    "initialize-echoes-a-supported-version": {
      oracle: async (ctx) => {
        const { envelope, requested } = await handshake(ctx);
        const result = (envelope.result ?? {}) as Record<string, unknown>;
        return { projection: { protocolVersionEchoed: result.protocolVersion === requested } };
      },
    },
  },
});

//#endregion 🧭️Adapter
