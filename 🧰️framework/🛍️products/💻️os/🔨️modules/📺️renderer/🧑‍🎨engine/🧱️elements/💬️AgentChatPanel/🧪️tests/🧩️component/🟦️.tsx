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
import { setUiLocale } from "@semio-tech/ui-react";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { AgentChatPanel } from "../../🟦️.tsx";
import { type AgentConversationEntry } from "../../../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
function toolCall(id: string, state: "running" | "cancelling" | "ok" | "failed"): AgentConversationEntry {
  return { kind: "toolCall", id, toolName: "inference_run", args: "{}", state, summary: state === "running" || state === "cancelling" ? null : "done", atMs: 0 };
}

const IDLE_PRESENCE = { active: false, label: "", invocationId: null } as const;
const here = dirname(fileURLToPath(import.meta.url));
const chatInputFixture = JSON.parse(readFileSync(join(here, "../../../../🧫️fixtures/💬️chat-input-accessibility/🔣️.json"), "utf8")) as {
  readonly contractVersion: 1;
  readonly cases: readonly {
    readonly locale: "en" | "de";
    readonly bridge: "open" | "closed";
    readonly accessibleName: string;
    readonly placeholder: string;
    readonly enabled: boolean;
    readonly enterAction: "sendChatDraft" | null;
  }[];
};
const chatInputSchema = JSON.parse(readFileSync(join(here, "../../../../🧬️schema/💬️chat-input-accessibility/🔣️.json"), "utf8"));
const cancellationFixture = JSON.parse(readFileSync(join(here, "../../../🔗️AgentBridge/🧫️fixtures/🛑️cancellation/🔣️.json"), "utf8")) as {
  readonly invocation: { readonly id: string; readonly toolName: string; readonly arguments: string };
  readonly openCancellation: { readonly state: "cancelling"; readonly cancelControl: false };
  readonly terminalResult: { readonly summary: string; readonly state: "failed"; readonly cancelControl: false };
};
//#endregion 🔖️Fixtures

//#region 🔖️CancelAffordance
afterEach(cleanup);
beforeEach(async () => {
  await setUiLocale("en");
});

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

//#region ♿️ComposerAccessibility
describe("AgentChatPanel composer accessibility", () => {
  it("validates the language-neutral two-locale, two-bridge contract", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(chatInputSchema);
    expect(validate(chatInputFixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...chatInputFixture, unknown: true })).toBe(false);
    expect(new Set(chatInputFixture.cases.map(({ locale, bridge }) => `${locale}:${bridge}`))).toEqual(new Set(["en:open", "en:closed", "de:open", "de:closed"]));
  });

  it("keeps the localized accessible name separate from placeholder and bridge liveness", async () => {
    for (const row of chatInputFixture.cases) {
      await setUiLocale(row.locale);
      const onSendMessage = vi.fn(() => true);
      const view = render(<AgentChatPanel status={row.bridge} presence={IDLE_PRESENCE} conversation={[]} onSendMessage={onSendMessage} />);
      const composer = screen.getByRole("textbox", { name: row.accessibleName });
      expect(composer.getAttribute("placeholder")).toBe(row.placeholder);
      expect(row.accessibleName).not.toBe(row.placeholder);
      expect(composer.hasAttribute("disabled")).toBe(!row.enabled);
      fireEvent.change(composer, { target: { value: "  inspect the scene  " } });
      fireEvent.keyDown(composer, { key: "Enter", shiftKey: false });
      if (row.enterAction === "sendChatDraft") expect(onSendMessage).toHaveBeenCalledWith("inspect the scene");
      else expect(onSendMessage).not.toHaveBeenCalled();
      view.unmount();
    }
  });
});
//#endregion ♿️ComposerAccessibility

//#region 💬️AgentReply
/** 🧪️ Ticket `26/09/18` slice AC1, audit `📓️g19-ai-user-experience-audit.md` gap 1: until now a
 * user could watch an agent act and type at it, but never read a word from it — the panel had no
 * row kind for the agent's own prose. These laws cover the row's identity, its streaming state, its
 * accessibility (a live region, announced not interrupted) and that both shipped locales name the
 * speaker rather than leaving a wire identifier on screen. */
function agentReply(id: string, text: string, state: "streaming" | "complete", inReplyTo: string | null = null): AgentConversationEntry {
  return { kind: "agentMessage", id, text, state, inReplyTo, atMs: 0 };
}

describe("AgentChatPanel agent reply", () => {
  it("renders the agent's own words as their own row, addressable by reply id", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[agentReply("rep_1", "Widening that wall means the 300 mm variant.", "complete", "msg_1")]} onSendMessage={() => true} />);
    const row = document.querySelector("[data-semio-agent-chat-entry='agentMessage']")!;
    expect(row.getAttribute("data-agent-chat-state")).toBe("complete");
    expect(row.id).toBe("framework.chat.entry.agentMessage.rep_1");
    expect(row.textContent).toContain("Widening that wall means the 300 mm variant.");
    const prose = document.querySelector("[data-semio-agent-chat-reply='rep_1']")!;
    expect(prose.getAttribute("data-semio-agent-chat-reply-to")).toBe("msg_1");
    expect(prose.getAttribute("aria-live")).toBe("polite");
    expect(prose.getAttribute("aria-busy")).toBe("false");
  });

  it("says a turn is still arriving while it streams, and stops saying so once it is complete", () => {
    const view = render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[agentReply("rep_2", "Checking the", "streaming")]} onSendMessage={() => true} />);
    const streaming = document.querySelector("[data-semio-agent-chat-entry='agentMessage']")!;
    expect(streaming.getAttribute("data-agent-chat-state")).toBe("streaming");
    expect(streaming.textContent).toContain("Still writing…");
    expect(document.querySelector("[data-semio-agent-chat-reply='rep_2']")!.getAttribute("aria-busy")).toBe("true");

    view.rerender(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[agentReply("rep_2", "Checking the catalog — one moment.", "complete")]} onSendMessage={() => true} />);
    const settled = document.querySelector("[data-semio-agent-chat-entry='agentMessage']")!;
    expect(settled.getAttribute("data-agent-chat-state")).toBe("complete");
    expect(settled.textContent).not.toContain("Still writing…");
    expect(settled.textContent).toContain("Checking the catalog — one moment.");
  });

  it("names the speaker in both shipped locales and never falls back to the wire identifier", async () => {
    for (const [locale, role, streaming] of [
      ["en", "Agent", "Still writing…"],
      ["de", "Agent", "Schreibt noch…"],
    ] as const) {
      await setUiLocale(locale);
      const view = render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[agentReply("rep_3", "…", "streaming")]} onSendMessage={() => true} />);
      const row = document.querySelector("[data-semio-agent-chat-entry='agentMessage']")!;
      expect(row.textContent).toContain(role);
      expect(row.textContent).toContain(streaming);
      expect(row.textContent).not.toContain("agentMessage");
      view.unmount();
    }
    await setUiLocale("en");
  });

  it("offers no cancel and no approval control on a prose row — there is nothing to stop or decide", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[agentReply("rep_4", "Done.", "complete")]} onSendMessage={() => true} onCancelToolCall={() => true} onResolveApproval={() => undefined} />);
    expect(document.querySelector("[data-semio-agent-chat-cancel]")).toBeNull();
    expect(document.querySelector("[data-semio-agent-approval-id]")).toBeNull();
  });
});
//#endregion 💬️AgentReply

//#region ⛩️UnifiedApproval
/** ⛩️ Slice U5: an approval's transcript row IS its one affordance — the same `framework.approvals.<decision>.<id>`
 * controls the wgpu twin uses, decided in place, no modal copy beside it. */
describe("AgentChatPanel approval affordance", () => {
  const approval = (state: "pending" | "resolved" | "withdrawn", decision: "deny" | "once" | "session" | null = null, withdrawal: "cancelled" | "timed_out" | "superseded" | null = null): AgentConversationEntry => ({
    kind: "approval",
    id: "appr_9",
    summary: JSON.stringify({ capabilityId: "note.editor.deleteSelection", capabilityTitle: "Delete selection", diffSummary: "Remove 2 blocks", risk: "high", requestedBy: "agent:claude", timeoutMs: 120_000 }),
    state,
    decision,
    withdrawal,
    atMs: Date.now(),
  });

  it("renders the pending approval once, with its countdown, and decides it in place", () => {
    const onResolveApproval = vi.fn();
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[approval("pending")]} onSendMessage={() => true} onResolveApproval={onResolveApproval} />);
    expect(document.querySelectorAll("[data-semio-agent-approval-id='appr_9']")).toHaveLength(1);
    expect(document.querySelector("[data-semio-agent-approval-countdown]")?.getAttribute("data-semio-agent-approval-countdown")).toBe("120");
    fireEvent.click(document.getElementById("framework.approvals.deny.appr_9")!);
    expect(onResolveApproval).toHaveBeenCalledWith("appr_9", "deny");
  });

  it("keeps a resolved approval as a record without decision controls", () => {
    render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[approval("resolved", "session")]} onSendMessage={() => true} onResolveApproval={vi.fn()} />);
    expect(document.getElementById("framework.approvals.session.appr_9")).toBeNull();
    expect(document.querySelector("[data-semio-agent-approval-state]")?.getAttribute("data-semio-agent-approval-state")).toBe("session");
  });

  it("retires a withdrawn approval: no countdown, no decision, and it says why, in en and de", async () => {
    const words = {
      en: { cancelled: "the agent's request was cancelled", timed_out: "nobody decided in time", superseded: "moved to your newer window" },
      de: { cancelled: "die Anfrage des Agenten wurde abgebrochen", timed_out: "niemand hat rechtzeitig entschieden", superseded: "in dein neueres Fenster verschoben" },
    } as const;
    for (const locale of ["en", "de"] as const) {
      await setUiLocale(locale);
      for (const reason of ["cancelled", "timed_out", "superseded"] as const) {
        render(<AgentChatPanel status="open" presence={IDLE_PRESENCE} conversation={[approval("withdrawn", null, reason)]} onSendMessage={() => true} onResolveApproval={vi.fn()} />);
        const affordance = document.querySelector("[data-semio-agent-approval-id='appr_9']");
        expect(affordance?.getAttribute("data-semio-agent-approval-state")).toBe("withdrawn");
        expect(affordance?.getAttribute("data-semio-agent-approval-withdrawal")).toBe(reason);
        expect(document.querySelector("[data-semio-agent-approval-countdown]")).toBeNull();
        for (const decision of ["deny", "once", "session"]) expect(document.getElementById(`framework.approvals.${decision}.appr_9`)).toBeNull();
        expect(affordance?.querySelector("[role='status']")?.textContent).toContain(words[locale][reason]);
        cleanup();
      }
    }
    await setUiLocale("en");
  });
});
//#endregion ⛩️UnifiedApproval
