/** 🪞️ Test double of the wasm package `@semio-tech/trinity-jack-lsp`: records what the worker asks of the session. */
type Calls = { handled: string[]; snapshots: string[]; initialized: number; failing: boolean };
const slot = globalThis as { __jackLspDouble?: Calls };
export const calls: Calls = (slot.__jackLspDouble ??= { handled: [], snapshots: [], initialized: 0, failing: false });

export default async function init(): Promise<void> {
  calls.initialized += 1;
}

export class JackLspSession {
  loadSnapshotJson(json: string): void {
    calls.snapshots.push(json);
  }

  handleMessageJson(message: string): string {
    calls.handled.push(message);
    if (calls.failing) throw new Error("wasm trap");
    const { id } = JSON.parse(message) as { id?: number };
    return JSON.stringify(id === undefined ? [{ jsonrpc: "2.0", method: "textDocument/publishDiagnostics", params: {} }] : [{ jsonrpc: "2.0", id, result: "echo" }, { jsonrpc: "2.0", method: "second" }]);
  }
}
