// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/💬️AgentChatPanel/tests/component.tsx
/** @emoji 🧪️ `💬️AgentChatPanel` cancel affordance — ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1, audit `📓️g5-ux-completeness-audit.md`
 * ranked item 2. The panel used to render a `running` tool call with no way to stop it although the
 * gateway's cancel path already existed. These laws cover which rows offer the control, what it
 * dispatches, and that it is reachable by name rather than by icon alone.
 *
 * Registered in `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`'s `engineTestSuites`.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it, vi } from "vitest";
import { AgentChatPanel } from "../../🟦️.tsx";
import { type AgentConversationEntry } from "../../../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
function toolCall(id: string, state: "running" | "cancelling" | "ok" | "failed"): AgentConversationEntry {
  return { kind: "toolCall", id, toolName: "inference_run", args: "{}", state, summary: state === "running" || state === "cancelling" ? null : "done", atMs: 0 };
}

const IDLE_PRESENCE = { active: false, label: "", invocationId: null } as const;
const here = dirname(fileURLToPath(import.meta.url));
const cancellationFixture = JSON.parse(readFileSync(join(here, "../../../🔗️AgentBridge/🧫️fixtures/🛑️cancellation/🔣️.json"), "utf8")) as {
  readonly invocation: { readonly id: string; readonly toolName: string; readonly arguments: string };
  readonly openCancellation: { readonly state: "cancelling"; readonly cancelControl: false };
  readonly terminalResult: { readonly summary: string; readonly state: "failed"; readonly cancelControl: false };
};
//#endregion 🔖️Fixtures

//#region 🔖️CancelAffordance
afterEach(cleanup);

describe("AgentChatPanel cancel affordance", () => {
  it("offers cancel only on a tool call still reported as running", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[toolCall("inv_1", "running"), toolCall("inv_2", "ok"), toolCall("inv_3", "failed"), toolCall("inv_4", "cancelling")]} onSendMessage={() => true} onCancelToolCall={() => true} />);
    const controls = document.querySelectorAll("[data-semio-agent-chat-cancel]");
    expect(controls.length).toBe(1);
    expect(controls[0]!.getAttribute("data-semio-agent-chat-cancel")).toBe("inv_1");
  });

  it("offers nothing when no cancel path is attached, rather than a control that does nothing", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[toolCall("inv_1", "running")]} onSendMessage={() => true} />);
    expect(document.querySelector("[data-semio-agent-chat-cancel]")).toBeNull();
  });

  it("dispatches the row's own invocation id, and names the tool in the accessible label (not colour or icon alone)", () => {
    const onCancelToolCall = vi.fn(() => true);
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[toolCall("inv_9", "running")]} onSendMessage={() => true} onCancelToolCall={onCancelToolCall} />);
    const control = screen.getByRole("button", { name: /inference_run/ });
    fireEvent.click(control);
    expect(onCancelToolCall).toHaveBeenCalledWith("inv_9");
  });

  it("shows a cancelling row as asked-to-stop, never as stopped — cancellation is cooperative", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[toolCall("inv_1", "cancelling")]} onSendMessage={() => true} onCancelToolCall={() => true} />);
    const row = document.querySelector("[data-semio-agent-chat-entry='toolCall']");
    expect(row!.getAttribute("data-agent-chat-state")).toBe("cancelling");
    expect(row!.textContent).toContain("Cancelling…");
  });

  it("matches the shared neutral invocation lifecycle through the actual React panel", () => {
    const onCancelToolCall = vi.fn(() => true);
    const running: AgentConversationEntry = {
      kind: "toolCall",
      id: cancellationFixture.invocation.id,
      toolName: cancellationFixture.invocation.toolName,
      args: cancellationFixture.invocation.arguments,
      state: "running",
      summary: null,
      atMs: 0,
    };
    const view = render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[running]} onSendMessage={() => true} onCancelToolCall={onCancelToolCall} />);
    fireEvent.click(screen.getByRole("button", { name: new RegExp(cancellationFixture.invocation.toolName) }));
    expect(onCancelToolCall).toHaveBeenCalledWith(cancellationFixture.invocation.id);

    const cancelling = { ...running, state: cancellationFixture.openCancellation.state } satisfies AgentConversationEntry;
    view.rerender(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[cancelling]} onSendMessage={() => true} onCancelToolCall={onCancelToolCall} />);
    expect(document.querySelector(`[data-semio-agent-chat-cancel='${cancellationFixture.invocation.id}']`)).toBeNull();
    expect(document.querySelector("[data-semio-agent-chat-entry='toolCall']")!.getAttribute("data-agent-chat-state")).toBe(cancellationFixture.openCancellation.state);

    const settled = { ...running, state: cancellationFixture.terminalResult.state, summary: cancellationFixture.terminalResult.summary } satisfies AgentConversationEntry;
    view.rerender(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[settled]} onSendMessage={() => true} onCancelToolCall={onCancelToolCall} />);
    const row = document.querySelector("[data-semio-agent-chat-entry='toolCall']")!;
    expect(row.getAttribute("data-agent-chat-state")).toBe(cancellationFixture.terminalResult.state);
    expect(row.textContent).toContain(cancellationFixture.terminalResult.summary);
    expect(document.querySelector("[data-semio-agent-chat-cancel]")).toBeNull();
  });
});
//#endregion 🔖️CancelAffordance
