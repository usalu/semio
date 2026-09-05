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
import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { AgentApprovals, parseApprovalSummary } from "./🟦️.tsx";
import { type PendingAgentApproval } from "../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(cleanup);

//#region 🔖️ParseSummary
describe("parseApprovalSummary", () => {
  it("parses a structured JSON summary", () => {
    const parsed = parseApprovalSummary(JSON.stringify({ capabilityId: "cad.viewport.translateSelection", diffSummary: "Move 3 elements by (1, 0, 0)", risk: "medium", requestedBy: "agent:demo" }));
    expect(parsed).toEqual({ capabilityId: "cad.viewport.translateSelection", diffSummary: "Move 3 elements by (1, 0, 0)", risk: "medium", requestedBy: "agent:demo" });
  });

  it("falls back to plain text when the summary is not JSON", () => {
    expect(parseApprovalSummary("translate the selection")).toEqual({ capabilityId: null, diffSummary: "translate the selection", risk: null, requestedBy: null });
  });

  it("falls back to plain text when the summary is JSON but not the expected shape", () => {
    expect(parseApprovalSummary(JSON.stringify([1, 2, 3]))).toEqual({ capabilityId: null, diffSummary: "[1,2,3]", risk: null, requestedBy: null });
  });
});
//#endregion 🔖️ParseSummary

//#region 🔖️Render
const structuredApproval: PendingAgentApproval = {
  approvalId: "appr_1",
  summary: JSON.stringify({ capabilityId: "cad.viewport.translateSelection", diffSummary: "Move 3 elements by (1, 0, 0)", risk: "high", requestedBy: "agent:demo" }),
  requestedAtMs: 1_700_000_000_000,
};

describe("AgentApprovals", () => {
  it("renders nothing open when there are no pending approvals", () => {
    render(<AgentApprovals approvals={[]} onDecision={vi.fn()} />);
    expect(screen.queryByText("Agent Approvals")).toBeNull();
  });

  it("shows the capability, diff summary, requester and risk for a pending approval", () => {
    render(<AgentApprovals approvals={[structuredApproval]} onDecision={vi.fn()} />);
    expect(screen.getByText("Agent Approvals")).toBeTruthy();
    expect(screen.getByText("cad.viewport.translateSelection")).toBeTruthy();
    expect(screen.getByText(/Move 3 elements by/)).toBeTruthy();
    expect(screen.getByText("agent:demo")).toBeTruthy();
    expect(screen.getByText("High")).toBeTruthy();
  });

  it("falls back to the raw summary text for a plain-text approval", () => {
    render(<AgentApprovals approvals={[{ approvalId: "appr_2", summary: "translate the selection", requestedAtMs: 1_700_000_000_000 }]} onDecision={vi.fn()} />);
    expect(screen.getByText("translate the selection")).toBeTruthy();
  });

  it("dispatches the right decision for the right approval when a decision button is clicked", () => {
    const onDecision = vi.fn();
    render(<AgentApprovals approvals={[structuredApproval]} onDecision={onDecision} />);
    fireEvent.click(screen.getByRole("button", { name: /Approve Once/ }));
    expect(onDecision).toHaveBeenCalledTimes(1);
    expect(onDecision).toHaveBeenCalledWith("appr_1", "once");
  });

  it("dispatches deny and approve-for-session decisions from their own buttons", () => {
    const onDecision = vi.fn();
    render(<AgentApprovals approvals={[structuredApproval]} onDecision={onDecision} />);
    fireEvent.click(screen.getByRole("button", { name: /^Deny/ }));
    fireEvent.click(screen.getByRole("button", { name: /Approve for Session/ }));
    expect(onDecision).toHaveBeenNthCalledWith(1, "appr_1", "deny");
    expect(onDecision).toHaveBeenNthCalledWith(2, "appr_1", "session");
  });

  it("lists every pending approval, each with its own decision buttons", () => {
    const second: PendingAgentApproval = { approvalId: "appr_2", summary: "second request", requestedAtMs: 1_700_000_000_001 };
    render(<AgentApprovals approvals={[structuredApproval, second]} onDecision={vi.fn()} />);
    expect(screen.getAllByRole("button", { name: /Approve Once/ })).toHaveLength(2);
  });
});
//#endregion 🔖️Render
