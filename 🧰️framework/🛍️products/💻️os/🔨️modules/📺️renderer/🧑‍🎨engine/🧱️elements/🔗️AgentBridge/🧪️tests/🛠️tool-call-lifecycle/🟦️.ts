/** 🔗️ Tool call lifecycle: 6 tests covering useAgentBridge cancelToolCall; matches the shared cancellation lifecycle through the real React hook; useAgentBridge agentReply; and more. */
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/🛠️tool-call-lifecycle.test.ts
/** 🛠️ AgentBridge tool-call lifecycle through the real hook over a fake socket: tool-call cancellation, the agent's free-text reply turns and the approval withdrawal replay. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { type AgentBridgeConfig, useAgentBridge } from "../../🟦️.tsx";
import { decodeShellToGateway, encodeGatewayToShell, type GatewayToShell, type ShellToGateway } from "@semio-tech/framework/mcp/bridge";
import { policyApprovalWithdrawalFixtureUrl } from "@semio-tech/framework/os/modules/mcp/testing/fixtures";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));
const cancellationFixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🛑️cancellation/🔣️.json"), "utf8")) as {
  readonly invocation: { readonly id: string; readonly toolName: string; readonly arguments: string };
  readonly openCancellation: { readonly accepted: boolean; readonly outbound: ShellToGateway; readonly state: string };
  readonly closedCancellation: { readonly accepted: boolean; readonly outboundCount: number; readonly state: string };
  readonly terminalResult: { readonly ok: boolean; readonly summary: string; readonly state: string };
};

//#region 🔖️CancelToolCall
/** 🧪️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit ranked item 2): the shell
 * can now ask the gateway to stop a tool call it is showing as running. Drives the REAL hook over a
 * fake `WebSocket` (the one seam a jsdom test can inject), feeds a real `agentToolCall` frame in
 * through `socket.onmessage`, and asserts the exact `agentCancel` bytes that leave. */
describe("useAgentBridge cancelToolCall", () => {
  it("matches the shared cancellation lifecycle through the real React hook", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.cancel.proof" };
    const hook = renderHook(() => useAgentBridge({ config }));
    try {
      const toolCall: GatewayToShell = { variant: "agentToolCall", invocationId: cancellationFixture.invocation.id, toolName: cancellationFixture.invocation.toolName, arguments: cancellationFixture.invocation.arguments };
      const wire = encodeGatewayToShell(toolCall);
      act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
      expect(hook.result.current.conversation.map((entry) => entry.id)).toEqual([cancellationFixture.invocation.id]);

      let accepted = false;
      act(() => {
        accepted = hook.result.current.cancelToolCall(cancellationFixture.invocation.id);
      });
      expect(accepted).toBe(cancellationFixture.openCancellation.accepted);
      expect(sent.at(-1)).toEqual(cancellationFixture.openCancellation.outbound);
      const entry = hook.result.current.conversation[0]!;
      expect(entry.kind === "toolCall" && entry.state).toBe(cancellationFixture.openCancellation.state);

      (live as unknown as { readyState: number }).readyState = 3;
      const beforeClosedCancel = sent.length;
      let refused = true;
      act(() => {
        refused = hook.result.current.cancelToolCall(cancellationFixture.invocation.id);
      });
      expect(refused).toBe(cancellationFixture.closedCancellation.accepted);
      expect(sent.length - beforeClosedCancel).toBe(cancellationFixture.closedCancellation.outboundCount);
      const cancelling = hook.result.current.conversation[0]!;
      expect(cancelling.kind === "toolCall" && cancelling.state).toBe(cancellationFixture.closedCancellation.state);

      const terminal: GatewayToShell = {
        variant: "agentToolResult",
        invocationId: cancellationFixture.invocation.id,
        toolName: cancellationFixture.invocation.toolName,
        ok: cancellationFixture.terminalResult.ok,
        summary: cancellationFixture.terminalResult.summary,
      };
      const terminalWire = encodeGatewayToShell(terminal);
      act(() => live?.onmessage?.({ data: terminalWire.buffer.slice(terminalWire.byteOffset, terminalWire.byteOffset + terminalWire.byteLength) as ArrayBuffer }));
      const settled = hook.result.current.conversation[0]!;
      expect(settled.kind === "toolCall" && settled.state).toBe(cancellationFixture.terminalResult.state);
      expect(settled.kind === "toolCall" && settled.summary).toBe(cancellationFixture.terminalResult.summary);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🔖️CancelToolCall

//#region 💬️AgentReply
/** 🧪️ Ticket `26/09/18` slice AC1: the agent's own free-text turn, through the real hook. One turn
 * is ONE row however many chunks it arrives in, the row says it is still arriving until the chunk
 * marked `complete` lands, and a second turn is a second row — nothing here is synthesised. */
describe("useAgentBridge agentReply", () => {
  it("folds every chunk of one turn into one row and settles it on the complete chunk", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(): void {}
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.reply.proof" };
    const hook = renderHook(() => useAgentBridge({ config }));
    const deliver = (frame: GatewayToShell): void => {
      const wire = encodeGatewayToShell(frame);
      act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
    };
    try {
      deliver({ variant: "agentReply", replyId: "rep_1", inReplyTo: "msg_1", text: "Widening that wall means", complete: false });
      let entry = hook.result.current.conversation[0]!;
      expect(hook.result.current.conversation).toHaveLength(1);
      expect(entry.kind === "agentMessage" && entry.state).toBe("streaming");
      expect(entry.kind === "agentMessage" && entry.inReplyTo).toBe("msg_1");

      deliver({ variant: "agentReply", replyId: "rep_1", inReplyTo: null, text: " the 300 mm variant.", complete: true });
      expect(hook.result.current.conversation).toHaveLength(1);
      entry = hook.result.current.conversation[0]!;
      expect(entry.kind === "agentMessage" && entry.text).toBe("Widening that wall means the 300 mm variant.");
      expect(entry.kind === "agentMessage" && entry.state).toBe("complete");

      deliver({ variant: "agentReply", replyId: "rep_2", inReplyTo: null, text: "Anything else?", complete: true });
      expect(hook.result.current.conversation.map((row) => row.id)).toEqual(["rep_1", "rep_2"]);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 💬️AgentReply

//#region 🪦️ApprovalWithdrawal
/** 🪦️ Replays `🌉️mcp/🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json`'s `retiredAffordances` through the
 * real hook over a fake socket: the same frames the gateway sends end every approval as pending,
 * resolved or withdrawn-with-its-reason, and a withdrawn one leaves nothing waiting. */
const withdrawalLaw = JSON.parse(readFileSync(policyApprovalWithdrawalFixtureUrl, "utf8")) as {
  readonly retiredAffordances: readonly { readonly name: string; readonly frames: readonly Record<string, string>[]; readonly state: string; readonly withdrawal: string | null; readonly pending: number }[];
};

describe("useAgentBridge approval withdrawal", () => {
  for (const row of withdrawalLaw.retiredAffordances) {
    it(`${row.name}: the approval ends ${row.state}${row.withdrawal ? ` (${row.withdrawal})` : ""}`, async () => {
      const { renderHook, act } = await import("@testing-library/react");
      let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
      class Socket {
        static OPEN = 1;
        readyState = 1;
        onopen: (() => void) | null = null;
        onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
        onerror: (() => void) | null = null;
        onclose: (() => void) | null = null;
        binaryType = "arraybuffer";
        constructor() {
          live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
        }
        send(): void {}
        close(): void {}
      }
      vi.stubGlobal("WebSocket", Socket);
      const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.withdrawal.proof" };
      const hook = renderHook(() => useAgentBridge({ config }));
      try {
        for (const frame of row.frames) {
          const full = { approvalId: "appr_law", ...(frame.variant === "approvalRequested" ? { summary: "{}" } : {}), ...frame } as GatewayToShell;
          const wire = encodeGatewayToShell(full);
          act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
        }
        const entry = hook.result.current.conversation.find((candidate) => candidate.id === "appr_law");
        expect(entry?.kind).toBe("approval");
        expect(entry?.kind === "approval" && entry.state).toBe(row.state);
        expect(entry?.kind === "approval" && entry.withdrawal).toBe(row.withdrawal);
        expect(hook.result.current.pendingApprovals).toHaveLength(row.pending);
      } finally {
        hook.unmount();
        vi.unstubAllGlobals();
      }
    });
  }
});
//#endregion 🪦️ApprovalWithdrawal
