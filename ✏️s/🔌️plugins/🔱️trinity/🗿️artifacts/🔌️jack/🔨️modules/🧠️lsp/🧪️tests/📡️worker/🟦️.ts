import { beforeEach, describe, expect, test, vi } from "vitest";
import { calls } from "./🪞️session-double/🟦️.ts";

type Listener = (event: { data: unknown }) => void;
const posted: unknown[] = [];
let listener: Listener | undefined;

async function settle(): Promise<void> {
  for (let turn = 0; turn < 20; turn += 1) await Promise.resolve();
  await new Promise(resolve => setTimeout(resolve, 0));
}

describe("jack lsp worker pump", () => {
  beforeEach(async () => {
    posted.length = 0;
    calls.handled.length = 0;
    calls.snapshots.length = 0;
    calls.initialized = 0;
    calls.failing = false;
    vi.resetModules();
    vi.stubGlobal("self", { addEventListener: (_type: string, next: Listener) => { listener = next; }, postMessage: (message: unknown) => { posted.push(message); } });
    await import("../../🟦️.ts");
  });

  test("answers init with ready after the session exists and loads the init snapshot into it", async () => {
    listener!({ data: { operation: "init", snapshotJson: "{}" } });
    await settle();
    expect(posted).toEqual([{ operator: "ready" }]);
    expect(calls.snapshots).toEqual(["{}"]);
    expect(calls.initialized).toBe(1);
  });

  test("forwards requests to the session and posts every reply in order", async () => {
    listener!({ data: { jsonrpc: "2.0", id: 3, method: "textDocument/hover" } });
    await settle();
    expect(calls.handled).toHaveLength(1);
    expect(posted).toEqual([{ jsonrpc: "2.0", id: 3, result: "echo" }, { jsonrpc: "2.0", method: "second" }]);
  });

  test("forwards notifications and keeps the order of consecutive messages", async () => {
    listener!({ data: { jsonrpc: "2.0", method: "textDocument/didOpen", params: {} } });
    listener!({ data: { jsonrpc: "2.0", id: 4, method: "textDocument/hover" } });
    await settle();
    expect(calls.handled.map(text => (JSON.parse(text) as { method: string }).method)).toEqual(["textDocument/didOpen", "textDocument/hover"]);
    expect(calls.initialized).toBe(1);
    expect(posted).toHaveLength(3);
  });

  test("answers a request with an internal error when the session traps", async () => {
    calls.failing = true;
    listener!({ data: { jsonrpc: "2.0", id: 5, method: "textDocument/hover" } });
    listener!({ data: { jsonrpc: "2.0", method: "textDocument/didClose" } });
    await settle();
    expect(posted).toEqual([{ jsonrpc: "2.0", id: 5, error: { code: -32603, message: "Error: wasm trap" } }]);
  });

  test("ignores payloads that are neither init nor JSON-RPC requests and notifications", async () => {
    listener!({ data: "text" });
    listener!({ data: { operation: "other" } });
    listener!({ data: { unrelated: true } });
    listener!({ data: { jsonrpc: "2.0", id: 1, result: null } });
    await settle();
    expect(posted).toEqual([]);
    expect(calls.handled).toEqual([]);
  });
});
