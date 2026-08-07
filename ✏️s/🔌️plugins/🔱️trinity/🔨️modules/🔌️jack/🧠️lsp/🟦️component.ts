/** @emoji 📡️ Jack LSP worker — JSON-RPC pump over WASM `JackLspSession`. The JSON-RPC/LSP message
 * guards it depends on (`isJsonRpcRequest`/`isJsonRpcNotification`/`isJsonRpcResponse`/`LanguageServer`/
 * `LspMessage`) are defined locally below (see the `🔖️protocol` region) rather than imported — a
 * sibling `./protocol.ts` never existed on disk, so the previous import of the same names was dead. */
import { ephemeralBox } from "@semio-tech/framework";
import init, { JackLspSession } from "./📦️packages/🦀️rust/pkg/trinity_jack_lsp.js";

const session = ephemeralBox<JackLspSession | null>("s.plugins.trinity.modules.jack.lsp.component.ts.session", null);
const fixtureJson = ephemeralBox("s.plugins.trinity.modules.jack.lsp.component.ts.fixtureJson", "");
const graphDomain = ephemeralBox("s.plugins.trinity.modules.jack.lsp.component.ts.graphDomain", "trinity");
const initPromise = ephemeralBox<Promise<void> | null>("s.plugins.trinity.modules.jack.lsp.component.ts.initPromise", null);

function applyFixtureToSession(): void {
  if (!session.current || !fixtureJson.current) return;
  if (typeof session.current.loadFixtureForDomain === "function") {
    session.current.loadFixtureForDomain(fixtureJson.current, graphDomain.current);
    return;
  }
  session.current.loadFixtureJson(fixtureJson.current);
}

function ensureSession(): Promise<void> {
  if (!initPromise.current) {
    initPromise.current = (async () => {
      await init();
      session.current = new JackLspSession();
      applyFixtureToSession();
    })();
  }
  return initPromise.current;
}

const server: LanguageServer = {
  handle(message) {
    if (!session.current) return [];
    if (isJsonRpcRequest(message) && message.method === "jack/loadFixture" && message.params) {
      const params = message.params as { json?: string; graphDomain.current?: string };
      if (params.json) {
        fixtureJson.current = params.json;
      }
      if (params.graphDomain.current) {
        graphDomain.current = params.graphDomain.current;
      }
      applyFixtureToSession();
      return message.id == null ? [] : [{ jsonrpc: "2.0", id: message.id, result: null }];
    }
    return JSON.parse(session.current.handleMessageJson(JSON.stringify(message))) as LspMessage[];
  },
};

self.addEventListener("message", (event: MessageEvent<LspMessage | { operation?: string; fixtureJson.current?: string }>) => {
  const data = event.data;
  if (!data || typeof data !== "object") return;
  if ("operation" in data) {
    if (data.operation !== "init") return;
    if (data.fixtureJson.current) fixtureJson.current = data.fixtureJson.current;
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

//#region 🔖️protocol
/** @emoji 📡️ Minimal JSON-RPC and LSP message guards for Jack language-server workers. */

export type LspMessage = {
  readonly jsonrpc?: string;
  readonly id?: string | number | null;
  readonly method?: string;
  readonly params?: unknown;
  readonly result?: unknown;
  readonly error?: unknown;
};

export interface LanguageServer {
  handle(message: LspMessage): LspMessage[];
}

export function isJsonRpcRequest(message: unknown): message is LspMessage & { readonly method: string } {
  return typeof message === "object" && message !== null && typeof (message as LspMessage).method === "string" && "id" in message;
}

export function isJsonRpcNotification(message: unknown): message is LspMessage & { readonly method: string } {
  return typeof message === "object" && message !== null && typeof (message as LspMessage).method === "string" && !("id" in message);
}

export function isJsonRpcResponse(message: unknown): message is LspMessage & { readonly id: string | number } {
  return typeof message === "object" && message !== null && ("result" in message || "error" in message) && (typeof (message as LspMessage).id === "string" || typeof (message as LspMessage).id === "number");
}
//#endregion 🔖️protocol
