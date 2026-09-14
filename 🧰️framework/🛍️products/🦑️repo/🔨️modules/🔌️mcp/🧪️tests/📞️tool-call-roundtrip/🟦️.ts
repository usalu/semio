//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { CallToolRequestSchema, GetPromptRequestSchema, ListPromptsRequestSchema, ListResourcesRequestSchema, ListToolsRequestSchema, ReadResourceRequestSchema } from "@modelcontextprotocol/sdk/types.js";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

type Vector = { name: string; ready: boolean; request: string; response: string };

/** 📞️ Starts the reference SDK server carrying the same `echo` tool, `repo://goals` resource and `review` prompt. */
async function referenceCallServer(): Promise<(request: unknown) => Promise<Record<string, unknown>>> {
  const server = new Server({ name: "repo", version: "1.0.0" }, { capabilities: { tools: {}, resources: {}, prompts: {} }, instructions: "owned" });
  server.setRequestHandler(ListToolsRequestSchema, () => ({ tools: [{ name: "echo", inputSchema: { type: "object" as const } }] }));
  server.setRequestHandler(CallToolRequestSchema, (request) => {
    if (request.params.name !== "echo") throw new Error("tool not found");
    return { content: [{ type: "text" as const, text: String((request.params.arguments ?? {}).text ?? "") }] };
  });
  server.setRequestHandler(ListResourcesRequestSchema, () => ({ resources: [{ uri: "repo://goals", name: "goals", mimeType: "application/json" }] }));
  server.setRequestHandler(ReadResourceRequestSchema, () => ({ contents: [{ uri: "repo://goals", mimeType: "application/json", text: "[]" }] }));
  server.setRequestHandler(ListPromptsRequestSchema, () => ({ prompts: [{ name: "review", arguments: [{ name: "scope", required: true }] }] }));
  server.setRequestHandler(GetPromptRequestSchema, (request) => {
    const scope = String((request.params.arguments ?? {}).scope ?? "");
    return { description: `Review ${scope}`, messages: [{ role: "user" as const, content: { type: "text" as const, text: scope } }] };
  });
  const [clientSide, serverSide] = InMemoryTransport.createLinkedPair();
  await server.connect(serverSide);
  await clientSide.start();
  return (request: unknown) =>
    new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("reference server did not answer")), 5000);
      clientSide.onmessage = (message: unknown) => {
        clearTimeout(timer);
        resolve(message as Record<string, unknown>);
      };
      void clientSide.send(request as never).catch(reject);
    });
}

function vectors(ctx: AdapterContext): Vector[] {
  return (JSON.parse(readFileSync(ctx.fixture("shared://2️⃣g2-contract.json"), "utf8")) as { vectors: Vector[] }).vectors;
}

/** 📞️ Dispatches one vector against a freshly initialized reference server. */
async function dispatch(vector: Vector): Promise<Record<string, unknown>> {
  const send = await referenceCallServer();
  if (vector.ready) await send({ jsonrpc: "2.0", id: "init", method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "test", version: "1" } } });
  return send(JSON.parse(vector.request));
}

//#endregion 🧭️Oracle

//#region 🧭️Adapter

/** 🟦️ TypeScript oracle for the golden call vectors, backed by `@modelcontextprotocol/sdk`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "tool-resource-prompt-roundtrip": {
      oracle: async (ctx) => {
        const rows = [];
        for (const vector of vectors(ctx).filter((vector) => ["tool", "resource", "prompt"].includes(vector.name))) {
          const envelope = await dispatch(vector);
          rows.push({ name: vector.name, result: envelope.result ?? null });
        }
        return { projection: { vectors: rows } };
      },
    },
    "protocol-error-vectors": {
      oracle: async (ctx) => {
        const rows = [];
        for (const vector of vectors(ctx).filter((vector) => vector.name === "unknown-method")) {
          const envelope = await dispatch(vector);
          rows.push({ name: vector.name, code: Number(((envelope.error ?? {}) as Record<string, unknown>).code ?? 0) });
        }
        return { projection: { vectors: rows } };
      },
    },
  },
});

//#endregion 🧭️Adapter
