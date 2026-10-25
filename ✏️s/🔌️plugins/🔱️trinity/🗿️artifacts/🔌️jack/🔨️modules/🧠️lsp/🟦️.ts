/** 📡️ Jack language-server worker: pumps JSON-RPC between its host and the wasm `JackLspSession`; the message guards live in `./🔖️protocol/🟦️.ts`. */
import { ephemeralBox } from "@semio-tech/framework";
import init, { JackLspSession } from "@semio-tech/trinity-jack-lsp";
import { type LspMessage, isJsonRpcNotification, isJsonRpcRequest } from "./🔖️protocol/🟦️.ts";
import protocol from "./🔖️protocol/🔣️.json" with { type: "json" };

const session = ephemeralBox<JackLspSession | null>("s.plugins.trinity.artifacts.jack.modules.lsp.worker.session", null);
const snapshotJson = ephemeralBox("s.plugins.trinity.artifacts.jack.modules.lsp.worker.snapshotJson", "");
const initPromise = ephemeralBox<Promise<void> | null>("s.plugins.trinity.artifacts.jack.modules.lsp.worker.initPromise", null);

function ensureSession(): Promise<void> {
  if (!initPromise.current) {
    initPromise.current = (async () => {
      await init();
      session.current = new JackLspSession();
      if (snapshotJson.current) session.current.loadSnapshotJson(snapshotJson.current);
    })();
  }
  return initPromise.current;
}

function reply(message: LspMessage): void {
  const answers = JSON.parse(session.current!.handleMessageJson(JSON.stringify(message))) as LspMessage[];
  for (const answer of answers) self.postMessage(answer);
}

self.addEventListener("message", (event: MessageEvent<LspMessage | { operation?: string; snapshotJson?: string }>) => {
  const data = event.data;
  if (!data || typeof data !== "object") return;
  if ("operation" in data) {
    if (data.operation !== "init") return;
    if (data.snapshotJson) snapshotJson.current = data.snapshotJson;
    void ensureSession().then(() => self.postMessage({ operator: "ready" }));
    return;
  }
  if (!isJsonRpcRequest(data) && !isJsonRpcNotification(data)) return;
  void ensureSession().then(() => reply(data)).catch(error => {
    if (isJsonRpcRequest(data)) self.postMessage({ jsonrpc: "2.0", id: data.id, error: { code: protocol.errorCodes.internalError, message: String(error) } });
  });
});
