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
import { afterEach, describe, expect, it, vi } from "vitest";
import { AgentChatPanel } from "../../🟦️.tsx";
import { type AgentConversationEntry } from "../../../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
function toolCall(id: string, state: "running" | "cancelling" | "ok" | "failed"): AgentConversationEntry {
  return { kind: "toolCall", id, toolName: "inference_run", args: "{}", state, summary: state === "running" || state === "cancelling" ? null : "done", atMs: 0 };
}

const IDLE_PRESENCE = { active: false, label: "", invocationId: null } as const;
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
});
//#endregion 🔖️CancelAffordance
