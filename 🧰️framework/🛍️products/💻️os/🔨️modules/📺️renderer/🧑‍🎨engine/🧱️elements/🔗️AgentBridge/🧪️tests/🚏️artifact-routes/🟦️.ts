/** 🔗️ Artifact routes: 16 tests covering shell channel payload codec; round-trips base64 at every length class and refuses malformed text; decodes every gateway→shell command in the shared fixture; and more. */
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/🚏️artifact-routes.test.ts
/** 🚏️ AgentBridge artifact routes: the shell channel payload codec, the app command answer, the live artifact route and the artifact-bytes route of a live editor instance. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";
import { type AgentBridgeConfig, useAgentBridge } from "../../🟦️.tsx";
import { decodeShellToGateway, encodeGatewayToShell, type ShellToGateway } from "@semio-tech/framework/mcp/bridge";
import { answerAgentAppCommand, NO_ARTIFACT_ROUTE_MESSAGE } from "../../🟦️.tsx";
import { shellAppFault } from "@semio-tech/framework/mcp/channel";
import { decodeChannelBase64, decodeShellAppCommand, encodeChannelBase64, shellAppFrameToJson, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";
import { channelAppPayloadsFixtureUrl } from "@semio-tech/framework-os-mcp/testing/fixtures";
// #endregion 🔌️Adapters

//#region 🔖️LiveArtifactRoute
/** 🧪️ Ticket `26/09/18` slice LB1: the LIVE artifact route — the seam that makes an MCP client's
 * `action_invoke`/`history_undo`/`transaction_*` execute in THIS shell instead of in the gateway's
 * own headless interpreter. Two banks, one fixture file: these laws read the same
 * `🐚️channel/🧫️fixtures/🗿️app-payloads.json` the Rust side's own tests assert against, so a drift in
 * either codec is red here AND there, never a dropped mutation at runtime. */
const appPayloadFixtures = JSON.parse(readFileSync(channelAppPayloadsFixtureUrl, "utf8")) as readonly {
  readonly direction: "gateway_to_shell" | "shell_to_gateway";
  readonly kind: string;
  readonly payload: Record<string, unknown>;
}[];

const appPayload = (kind: string): Record<string, unknown> => {
  const row = appPayloadFixtures.find((entry) => entry.kind === kind);
  if (!row) throw new Error(`fixture \`${kind}\` exists`);
  return row.payload;
};

const appPayloadBytes = (kind: string): Uint8Array => new TextEncoder().encode(JSON.stringify(appPayload(kind)));

describe("shell channel payload codec", () => {
  it("round-trips base64 at every length class and refuses malformed text", () => {
    for (let length = 0; length <= 64; length += 1) {
      const bytes = Uint8Array.from({ length }, (_unused, index) => (index * 7) % 251);
      expect(Array.from(decodeChannelBase64(encodeChannelBase64(bytes)) ?? [])).toEqual(Array.from(bytes));
    }
    expect(decodeChannelBase64("AQI")).toBeNull();
    expect(decodeChannelBase64("A===")).toBeNull();
    expect(decodeChannelBase64("AQ=D")).toBeNull();
    expect(decodeChannelBase64("AQ!D")).toBeNull();
  });

  it("decodes every gateway→shell command in the shared fixture", () => {
    expect(decodeShellAppCommand(appPayloadBytes("readHistory"))).toEqual({ kind: "readHistory" });
    expect(decodeShellAppCommand(appPayloadBytes("readArtifact"))).toEqual({ kind: "readArtifact" });
    expect(decodeShellAppCommand(appPayloadBytes("pureCommand"))).toEqual({ kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: { text: "hello" } });
    const prepare = decodeShellAppCommand(appPayloadBytes("transactionPrepare"));
    expect(prepare.kind).toBe("transactionPrepare");
    expect(prepare.kind === "transactionPrepare" && Array.from(prepare.ops.document[0]!)).toEqual([1, 2, 3]);
    expect(prepare.kind === "transactionPrepare" && prepare.origin).toEqual({ kind: "agent", principal: "agent:local", invocationId: "inv_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionCommit"))).toEqual({ kind: "transactionCommit", txnId: "txn_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionRollback"))).toEqual({ kind: "transactionRollback", txnId: "txn_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionUndo"))).toEqual({ kind: "transactionUndo", groupId: "edit_7" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionRedo"))).toEqual({ kind: "transactionRedo", groupId: "edit_7" });
    const exported = decodeShellAppCommand(appPayloadBytes("exportMedia"));
    expect(exported.kind === "exportMedia" && Array.from(exported.document)).toEqual([1, 2, 3, 4]);
  });

  it("encodes every shell→gateway frame back to the shared fixture, byte for byte", () => {
    const frames: readonly ShellAppFrameV1[] = [
      { kind: "historySnapshot", artifactId: "note-1", headEditId: "edit_7", cursor: "7" },
      { kind: "emit", ops: { document: [Uint8Array.from([1, 2, 3])], config: [], draft: [] }, warnings: [] },
      { kind: "transactionPrepared", txnId: "txn_1" },
      { kind: "transactionCommitted", txnId: "txn_1", editId: "edit_8" },
      { kind: "transactionRolledBack", txnId: "txn_1" },
      { kind: "transactionUndone", groupId: "edit_8" },
      { kind: "transactionRedone", groupId: "edit_8" },
      { kind: "artifact", pack: Uint8Array.from([1, 2, 3, 4]), spr: Uint8Array.from([5, 6]) },
      { kind: "exported", port: "pdf", descriptor: Uint8Array.from([1]), data: Uint8Array.from([2, 3]) },
      { kind: "error", code: "app.command.rejected", message: "the document is read-only for this actor" },
    ];
    for (const frame of frames) expect(shellAppFrameToJson(frame)).toEqual(appPayload(frame.kind));
    expect(frames.length).toBe(appPayloadFixtures.filter((row) => row.direction === "shell_to_gateway").length);
  });

  it("refuses a payload from another codec version and an unknown kind, by name", () => {
    const future = new TextEncoder().encode(JSON.stringify({ ...appPayload("transactionCommit"), version: 99 }));
    expect(() => decodeShellAppCommand(future)).toThrow(/99/);
    const unknown = new TextEncoder().encode(JSON.stringify({ ...appPayload("transactionCommit"), kind: "teleport" }));
    expect(() => decodeShellAppCommand(unknown)).toThrow(/teleport/);
  });
});

describe("answerAgentAppCommand", () => {
  it("refuses immediately when no host mounted an artifact route, instead of going silent", async () => {
    const answer = await answerAgentAppCommand({ seq: 7n, instanceId: "inst-1", command: { kind: "readHistory" } }, undefined);
    expect(answer.variant).toBe("appFrames");
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(7n);
    expect(answer.variant === "appFrames" && answer.instanceId).toBe("inst-1");
    const decoded = JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
    expect(decoded.kind).toBe("error");
    expect(decoded.code).toBe("plugin.unavailable");
    expect(String(decoded.message)).toBe(NO_ARTIFACT_ROUTE_MESSAGE);
  });

  it("turns a throwing host handler into a named error frame on the same correlation id", async () => {
    const answer = await answerAgentAppCommand({ seq: 9n, instanceId: "inst-2", command: { kind: "transactionCommit", txnId: "txn_1" } }, () => {
      throw new Error("the active window rejected the dispatch");
    });
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(9n);
    const decoded = JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
    expect(decoded.code).toBe("channel.not-wired");
    expect(decoded.message).toBe("the active window rejected the dispatch");
  });
});

describe("useAgentBridge live artifact route", () => {
  it("declares relayAppCommands only with a handler, publishes its instance census, and answers an inbound appCommand", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.route.proof" };
    const instances = [{ pluginId: "note", appId: "note", instanceId: "inst-1", artifactRef: "note-1", windowIds: ["note-composite"] }];
    const seen: string[] = [];
    const hook = renderHook(() =>
      useAgentBridge({
        config,
        instances,
        onAppCommand: (request) => {
          seen.push(`${request.instanceId}:${request.command.kind}`);
          return [{ kind: "transactionCommitted", txnId: "txn_1", editId: "edit_8" }];
        },
      }),
    );
    try {
      act(() => live?.onopen?.());
      const hello = sent.find((frame) => frame.variant === "hello");
      expect(hello?.variant === "hello" && hello.flags.relayAppCommands).toBe(true);

      const welcome = encodeGatewayToShell({ variant: "welcome", bridgeVersion: 1, connection: "conn_1", principal: "agent:local" });
      act(() => live?.onmessage?.({ data: welcome.buffer.slice(welcome.byteOffset, welcome.byteOffset + welcome.byteLength) as ArrayBuffer }));
      const census = sent.find((frame) => frame.variant === "instances");
      expect(census?.variant === "instances" && census.entries).toEqual(instances);

      const command = encodeGatewayToShell({ variant: "appCommand", seq: 11n, instanceId: "inst-1", command: appPayloadBytes("transactionCommit") });
      await act(async () => {
        live?.onmessage?.({ data: command.buffer.slice(command.byteOffset, command.byteOffset + command.byteLength) as ArrayBuffer });
        await Promise.resolve();
        await Promise.resolve();
      });
      expect(seen).toEqual(["inst-1:transactionCommit"]);
      const reply = sent.find((frame) => frame.variant === "appFrames");
      expect(reply?.variant === "appFrames" && reply.inReplyTo).toBe(11n);
      const decoded = JSON.parse(new TextDecoder().decode((reply as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
      expect(decoded).toEqual(appPayload("transactionCommitted"));
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });

  it("does not claim the relay when no handler is mounted, so the gateway keeps its headless workspace", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onopen: (() => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onopen: (() => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const hook = renderHook(() => useAgentBridge({ config: { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.norelay.proof" } }));
    try {
      act(() => live?.onopen?.());
      const hello = sent.find((frame) => frame.variant === "hello");
      expect(hello?.variant === "hello" && hello.flags.relayAppCommands).toBe(false);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🔖️LiveArtifactRoute

//#region 🔖️ArtifactBytesRoute
/** 🧪️ Slice LB1 §11: an agent's `ReadArtifact`/`ExportMedia` against a live editor instance. The
 * host handler is the seam `🏛️ShellHost` fills from the plugin handle's own `readAppDocumentPack`;
 * these laws pin BOTH outcomes the route promises — real bytes for a plugin with a document port,
 * and a refusal that NAMES the missing port for one without. Never silence: a dropped answer costs
 * the agent its whole wall budget and tells it nothing. */
describe("agent artifact-bytes route", () => {
  const decodeFrame = (answer: ShellToGateway): Record<string, unknown> => JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;

  it("answers a ReadArtifact for a live instance with the document's own pack and spr bytes", async () => {
    const pack = Uint8Array.from([1, 2, 3, 4]);
    const spr = Uint8Array.from([5, 6]);
    const answer = await answerAgentAppCommand({ seq: 21n, instanceId: "inst-1", command: { kind: "readArtifact" } }, async () => {
      const document: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null = { pack, spr };
      if (!document) return [shellAppFault("plugin.unavailable", "no document")];
      return [{ kind: "artifact", pack: document.pack, spr: document.spr }];
    });
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(21n);
    const decoded = decodeFrame(answer);
    expect(decoded.kind).toBe("artifact");
    expect(decoded.pack).toBe(encodeChannelBase64(pack));
    expect(decoded.spr).toBe(encodeChannelBase64(spr));
    expect(decoded).toEqual(appPayload("artifact"));
  });

  it("refuses by name when the plugin exposes no document port, and when it holds no document", async () => {
    const noPort = await answerAgentAppCommand({ seq: 22n, instanceId: "inst-1", command: { kind: "readArtifact" } }, () => [
      shellAppFault("plugin.unavailable", "plugin `space` exposes no document port in this shell — an agent needing a genesis read must resolve a headless context (`--folder`/`--hub`)"),
    ]);
    const refused = decodeFrame(noPort);
    expect(refused.kind).toBe("error");
    expect(refused.code).toBe("plugin.unavailable");
    expect(String(refused.message)).toContain("no document port");
    expect(String(refused.message)).toContain("space");

    const noDocument = await answerAgentAppCommand({ seq: 23n, instanceId: "inst-1", command: { kind: "readArtifact" } }, () => [shellAppFault("plugin.unavailable", "plugin `note` instance 3 holds no document to read")]);
    expect(String(decodeFrame(noDocument).message)).toContain("holds no document");
  });

  it("refuses an ExportMedia by naming the missing OUT port, never a silent or empty export", async () => {
    const answer = await answerAgentAppCommand({ seq: 24n, instanceId: "inst-1", command: { kind: "exportMedia", port: "pdf", document: new Uint8Array(), documentSpr: new Uint8Array() } }, () => [
      shellAppFault("plugin.unavailable", "this shell has no media OUT port for `pdf` — `PluginWasmHandle` exposes none, and the UI's export path hands the human a download rather than returning bytes; `artifact_export` needs a headless context (`--folder`/`--hub`)"),
    ]);
    const decoded = decodeFrame(answer);
    expect(decoded.kind).toBe("error");
    expect(String(decoded.message)).toContain("media OUT port for `pdf`");
    expect(answer.variant === "appFrames" && answer.instanceId).toBe("inst-1");
  });

  it("encodes a real exported frame back to the shared fixture, so the day a port exists the wire is already pinned", () => {
    expect(shellAppFrameToJson({ kind: "exported", port: "pdf", descriptor: Uint8Array.from([1]), data: Uint8Array.from([2, 3]) })).toEqual(appPayload("exported"));
  });
});
//#endregion 🔖️ArtifactBytesRoute
