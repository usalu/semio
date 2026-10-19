import { beforeEach, describe, expect, test, vi } from "vitest";

const handled: string[] = [];
const loaded: string[] = [];
vi.mock("../../📦️packages/🦀️rust/pkg/trinity_jack_lsp.js", () => ({
  default: async () => undefined,
  JackLspSession: class {
    loadSnapshotForDomain(json: string, domain: string): void { loaded.push(`${domain}:${json}`); }
    loadSnapshotJson(json: string): void { loaded.push(json); }
    handleMessageJson(message: string): string {
      handled.push(message);
      return JSON.stringify([{ jsonrpc: "2.0", id: (JSON.parse(message) as { id: number }).id, result: "echo" }]);
    }
  },
}));

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
    handled.length = 0;
    loaded.length = 0;
    vi.resetModules();
    vi.stubGlobal("self", { addEventListener: (_type: string, next: Listener) => { listener = next; }, postMessage: (message: unknown) => { posted.push(message); } });
    await import("../../🟦️.ts");
  });

  test("answers init with ready after the session exists", async () => {
    listener!({ data: { operation: "init", snapshotJson: "{}" } });
    await settle();
    expect(posted).toEqual([{ operator: "ready" }]);
    expect(loaded).toEqual(["trinity:{}"]);
  });

  test("forwards requests to the session and posts every reply", async () => {
    listener!({ data: { jsonrpc: "2.0", id: 3, method: "textDocument/hover" } });
    await settle();
    expect(handled).toHaveLength(1);
    expect(posted).toEqual([{ jsonrpc: "2.0", id: 3, result: "echo" }]);
  });

  test("loads a snapshot for a graph domain without consulting the session", async () => {
    listener!({ data: { jsonrpc: "2.0", id: 4, method: "jack/loadSnapshot", params: { json: "{\"a\":1}", graphDomain: "other" } } });
    await settle();
    expect(handled).toEqual([]);
    expect(loaded).toEqual(["other:{\"a\":1}"]);
    expect(posted).toEqual([{ jsonrpc: "2.0", id: 4, result: null }]);
  });

  test("ignores payloads that are neither init nor JSON-RPC", async () => {
    listener!({ data: "text" });
    listener!({ data: { operation: "other" } });
    listener!({ data: { unrelated: true } });
    await settle();
    expect(posted).toEqual([]);
    expect(handled).toEqual([]);
  });
});
