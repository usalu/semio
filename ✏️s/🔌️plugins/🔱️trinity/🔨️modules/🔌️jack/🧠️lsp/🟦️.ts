/** 📡️ Jack LSP worker: JSON-RPC pump over WASM `JackLspSession`; the message guards live in `./🔖️protocol/🟦️.ts`. */
import { ephemeralBox } from "@semio-tech/framework";
import { type LanguageServer, type LspMessage, isJsonRpcNotification, isJsonRpcRequest, isJsonRpcResponse } from "./🔖️protocol/🟦️.ts";
import init, { JackLspSession } from "./📦️packages/🦀️rust/pkg/trinity_jack_lsp.js";

const session = ephemeralBox<JackLspSession | null>("s.plugins.trinity.modules.jack.lsp.component.ts.session", null);
const snapshotJson = ephemeralBox("s.plugins.trinity.modules.jack.lsp.component.ts.snapshotJson", "");
const graphDomain = ephemeralBox("s.plugins.trinity.modules.jack.lsp.component.ts.graphDomain", "trinity");
const initPromise = ephemeralBox<Promise<void> | null>("s.plugins.trinity.modules.jack.lsp.component.ts.initPromise", null);

function applySnapshotToSession(): void {
  if (!session.current || !snapshotJson.current) return;
  if (typeof session.current.loadSnapshotForDomain === "function") {
    session.current.loadSnapshotForDomain(snapshotJson.current, graphDomain.current);
    return;
  }
  session.current.loadSnapshotJson(snapshotJson.current);
}

function ensureSession(): Promise<void> {
  if (!initPromise.current) {
    initPromise.current = (async () => {
      await init();
      session.current = new JackLspSession();
      applySnapshotToSession();
    })();
  }
  return initPromise.current;
}

const server: LanguageServer = {
  handle(message) {
    if (!session.current) return [];
    if (isJsonRpcRequest(message) && message.method === "jack/loadSnapshot" && message.params) {
      const params = message.params as { json?: string; graphDomain?: string };
      if (params.json) {
        snapshotJson.current = params.json;
      }
      if (params.graphDomain) {
        graphDomain.current = params.graphDomain;
      }
      applySnapshotToSession();
      return message.id == null ? [] : [{ jsonrpc: "2.0", id: message.id, result: null }];
    }
    return JSON.parse(session.current.handleMessageJson(JSON.stringify(message))) as LspMessage[];
  },
};

self.addEventListener("message", (event: MessageEvent<LspMessage | { operation?: string; snapshotJson?: string }>) => {
  const data = event.data;
  if (!data || typeof data !== "object") return;
  if ("operation" in data) {
    if (data.operation !== "init") return;
    if (data.snapshotJson) snapshotJson.current = data.snapshotJson;
    void ensureSession().then(() => {
      self.postMessage({ operator: "ready" });
    });
    return;
  }
  if (!isJsonRpcRequest(data) && !isJsonRpcNotification(data) && !isJsonRpcResponse(data)) return;
  void ensureSession().then(() => {
    for (const reply of server.handle(data)) {
      self.postMessage(reply);
    }
  });
});
