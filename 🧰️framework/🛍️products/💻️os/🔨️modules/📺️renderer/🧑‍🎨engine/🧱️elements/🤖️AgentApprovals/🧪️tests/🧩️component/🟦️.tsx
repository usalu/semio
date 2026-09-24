// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🤖️AgentApprovals/component.test.tsx
/** @emoji 🧪️ `🤖️AgentApprovals` tests: the pure `parseApprovalSummary` fallback parser plus a
 * render + decision-dispatch test proving the dialog shows capability/diff/risk and that clicking
 * a decision button calls `onDecision` with the right `(approvalId, decision)`. Not wired into the
 * nx `test` target — see `AgentBridge/🧪️component.test.ts`'s header for why, and
 * `.🧬semio/…/📓️terra-P10-report.md` for the direct foreground `vitest run` invocation.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { AgentApprovalAffordance, AgentApprovalsNotice, approvalSecondsRemaining, focusAgentApprovalV1, parseApprovalSummary } from "../../🟦️.tsx";
import { type PendingAgentApproval } from "../../../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(cleanup);

//#region 🔖️ParseSummary
describe("parseApprovalSummary", () => {
  it("parses a structured JSON summary", () => {
    const parsed = parseApprovalSummary(JSON.stringify({ capabilityId: "cad.viewport.translateSelection", diffSummary: "Move 3 elements by (1, 0, 0)", risk: "medium", requestedBy: "agent:demo" }));
    expect(parsed).toEqual({ capabilityId: "cad.viewport.translateSelection", capabilityTitle: null, description: null, artifactKind: null, diffSummary: "Move 3 elements by (1, 0, 0)", risk: "medium", requestedBy: "agent:demo", timeoutMs: null });
  });

  it("falls back to plain text when the summary is not JSON", () => {
    expect(parseApprovalSummary("translate the selection")).toEqual({ capabilityId: null, capabilityTitle: null, description: null, artifactKind: null, diffSummary: "translate the selection", risk: null, requestedBy: null, timeoutMs: null });
  });

  it("falls back to plain text when the summary is JSON but not the expected shape", () => {
    expect(parseApprovalSummary(JSON.stringify([1, 2, 3]))).toEqual({ capabilityId: null, capabilityTitle: null, description: null, artifactKind: null, diffSummary: "[1,2,3]", risk: null, requestedBy: null, timeoutMs: null });
  });

  // 🧾️ The same file the wgpu twin `parse_approval_summary` is asserted against, so the two hosts
  // can never disagree about what the gateway sent.
  const summaryFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🛡️summary/🔣️.json"), "utf8")) as Record<string, { readonly summary: string; readonly parsed: Record<string, unknown> }> & {
    readonly unusableTimeout: { readonly summary: string };
  };

  for (const row of ["rich", "plainText", "legacyWithoutTheNewFields"]) {
    it(`parses the shared fixture row \`${row}\` exactly as the wgpu bank does`, () => {
      expect(parseApprovalSummary(summaryFixture[row]!.summary)).toEqual(summaryFixture[row]!.parsed);
    });
  }

  it("refuses to count down a non-positive timeout", () => {
    expect(parseApprovalSummary(summaryFixture.unusableTimeout.summary).timeoutMs).toBeNull();
  });
});

describe("approvalSecondsRemaining", () => {
  it("counts from the frame's arrival, floors at zero, and answers nothing without a budget", () => {
    expect(approvalSecondsRemaining(120_000, 1_000, 1_000)).toBe(120);
    expect(approvalSecondsRemaining(120_000, 1_000, 61_000)).toBe(60);
    expect(approvalSecondsRemaining(120_000, 1_000, 999_000)).toBe(0);
    expect(approvalSecondsRemaining(null, 1_000, 1_000)).toBeNull();
  });
});
//#endregion 🔖️ParseSummary

//#region 🔖️Render
const structuredApproval: PendingAgentApproval = {
  approvalId: "appr_1",
  summary: JSON.stringify({ capabilityId: "cad.viewport.translateSelection", diffSummary: "Move 3 elements by (1, 0, 0)", risk: "high", requestedBy: "agent:demo" }),
  requestedAtMs: 1_700_000_000_000,
};

/** ⛩️ Renders one approval exactly as the agent conversation does — the ONE affordance per approval
 * (slice U5 retired the modal copy that shadowed it). */
function affordance(approval: PendingAgentApproval, onDecision?: (approvalId: string, decision: "deny" | "once" | "session") => void) {
  return <AgentApprovalAffordance approvalId={approval.approvalId} summary={approval.summary} requestedAtMs={approval.requestedAtMs} state="pending" decision={null} onDecision={onDecision} />;
}

describe("AgentApprovalAffordance", () => {
  it("shows the capability, diff summary, requester and risk for a pending approval", () => {
    render(affordance(structuredApproval, vi.fn()));
    expect(screen.getByText("cad.viewport.translateSelection")).toBeTruthy();
    expect(screen.getByText(/Move 3 elements by/)).toBeTruthy();
    expect(screen.getByText(/agent:demo/)).toBeTruthy();
    expect(screen.getByText("High")).toBeTruthy();
  });

  // 📝️ The shared fixture's `affordance` rows are the lines BOTH hosts show — the wgpu overlay's
  // `approval_row_lines` asserts the very same list, so the two affordances cannot drift apart.
  const sharedFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🛡️summary/🔣️.json"), "utf8")) as Record<string, { readonly summary: string }> & {
    readonly affordance: readonly { readonly row: string; readonly secondsLeft: number | null; readonly en: readonly string[] }[];
  };
  for (const { row, secondsLeft, en } of sharedFixture.affordance) {
    it(`paints the shared affordance lines of \`${row}\` at ${secondsLeft ?? "no"} seconds left`, () => {
      const timeoutMs = parseApprovalSummary(sharedFixture[row]!.summary).timeoutMs;
      const requestedAtMs = timeoutMs === null || secondsLeft === null ? Date.now() : Date.now() - (timeoutMs - secondsLeft * 1000);
      render(affordance({ approvalId: `fixture_${row}`, summary: sharedFixture[row]!.summary, requestedAtMs }, vi.fn()));
      const section = document.querySelector(`[data-semio-agent-approval-id='fixture_${row}']`);
      expect([...(section?.querySelectorAll(":scope > p") ?? [])].map((line) => line.textContent?.replace(/\s+/gu, " ").trim())).toEqual(en);
    });
  }

  it("falls back to the raw summary text for a plain-text approval", () => {
    render(affordance({ approvalId: "appr_2", summary: "translate the selection", requestedAtMs: 1_700_000_000_000 }, vi.fn()));
    expect(document.querySelector("[data-semio-agent-approval-id='appr_2']")?.textContent).toContain("translate the selection");
  });

  it("dispatches the right decision for the right approval when a decision button is clicked", () => {
    const onDecision = vi.fn();
    render(affordance(structuredApproval, onDecision));
    fireEvent.click(screen.getByRole("button", { name: /Approve Once/ }));
    expect(onDecision).toHaveBeenCalledTimes(1);
    expect(onDecision).toHaveBeenCalledWith("appr_1", "once");
  });

  it("dispatches deny and approve-for-session decisions from their own buttons", () => {
    const onDecision = vi.fn();
    render(affordance(structuredApproval, onDecision));
    fireEvent.click(screen.getByRole("button", { name: /^Deny/ }));
    fireEvent.click(screen.getByRole("button", { name: /Approve for Session/ }));
    expect(onDecision).toHaveBeenNthCalledWith(1, "appr_1", "deny");
    expect(onDecision).toHaveBeenNthCalledWith(2, "appr_1", "session");
  });

  it("addresses every decision by the one id scheme both hosts share, and names the capability it decides", () => {
    render(affordance(structuredApproval, vi.fn()));
    for (const decision of ["deny", "once", "session"]) {
      const button = document.getElementById(`framework.approvals.${decision}.appr_1`);
      expect(button?.getAttribute("aria-label")).toContain("cad.viewport.translateSelection");
    }
  });

  it("offers no decision without a decision path, and a resolved approval shows how it was decided", () => {
    render(affordance(structuredApproval));
    expect(document.getElementById("framework.approvals.once.appr_1")).toBeNull();
    cleanup();
    render(<AgentApprovalAffordance approvalId="appr_1" summary={structuredApproval.summary} requestedAtMs={0} state="resolved" decision="once" onDecision={vi.fn()} />);
    expect(document.getElementById("framework.approvals.once.appr_1")).toBeNull();
    expect(document.querySelector("[data-semio-agent-approval-decision]")?.textContent).toBe("Approved once");
  });

  it("takes keyboard focus onto the affordance group, never onto a decision", () => {
    render(affordance(structuredApproval, vi.fn()));
    expect(focusAgentApprovalV1("appr_1")).toBe(true);
    expect(document.activeElement?.getAttribute("data-semio-agent-approval-id")).toBe("appr_1");
    expect(focusAgentApprovalV1("appr_missing")).toBe(false);
  });
});

describe("AgentApprovalsNotice", () => {
  it("renders nothing while no approval waits", () => {
    const { container } = render(<AgentApprovalsNotice approvals={[]} onReview={vi.fn()} />);
    expect(container.querySelector("[data-semio-agent-approvals-waiting]")).toBeNull();
  });

  it("counts waiting approvals and reviews the oldest one", () => {
    const onReview = vi.fn();
    const second: PendingAgentApproval = { approvalId: "appr_2", summary: "second request", requestedAtMs: 1_700_000_000_001 };
    render(<AgentApprovalsNotice approvals={[structuredApproval, second]} onReview={onReview} />);
    expect(screen.getByRole("status").getAttribute("aria-label")).toContain("2 agent approvals are waiting");
    fireEvent.click(screen.getByRole("button", { name: /Review/ }));
    expect(onReview).toHaveBeenCalledWith("appr_1");
  });
});
//#endregion 🔖️Render
