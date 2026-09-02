// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentPresence/component.test.tsx
/** @emoji 🧪️ `AgentPresence` tests: the pure `agentPresenceTone` state machine plus a render
 * smoke test asserting the accessible `role="status"` name and visible text for each bridge
 * status/presence combination. Not wired into the nx `test` target — see
 * `AgentBridge/🧪️component.test.ts`'s header for why, and
 * `.🧬semio/…/📓️terra-P10-report.md` for the direct foreground `vitest run` invocation.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { AgentPresence, agentPresenceTone } from "./🟦️.tsx";
import { type AgentBridgePresence } from "../AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(cleanup);

const idle: AgentBridgePresence = { active: false, label: "", invocationId: null };
const working: AgentBridgePresence = { active: true, label: "translateSelection", invocationId: "inv-1" };

describe("agentPresenceTone", () => {
  it("is disconnected when disabled", () => expect(agentPresenceTone("disabled", idle)).toBe("disconnected"));
  it("is disconnected when closed", () => expect(agentPresenceTone("closed", idle)).toBe("disconnected"));
  it("is connecting while connecting", () => expect(agentPresenceTone("connecting", idle)).toBe("connecting"));
  it("is connecting while reconnecting", () => expect(agentPresenceTone("reconnecting", idle)).toBe("connecting"));
  it("is connected when open and idle", () => expect(agentPresenceTone("open", idle)).toBe("connected"));
  it("is working when open and active", () => expect(agentPresenceTone("open", working)).toBe("working"));
});

describe("AgentPresence", () => {
  it("renders a status role with an accessible name", () => {
    render(<AgentPresence status="open" presence={idle} />);
    expect(screen.getByRole("status", { name: "Agent status" })).toBeTruthy();
  });

  it("shows the idle label when connected but not active", () => {
    render(<AgentPresence status="open" presence={idle} />);
    expect(screen.getByRole("status").textContent).toBe("Agent idle");
  });

  it("shows the working label with the invocation label interpolated when active", () => {
    render(<AgentPresence status="open" presence={working} />);
    expect(screen.getByRole("status").textContent).toBe("Agent working: translateSelection");
  });

  it("shows the disconnected label when disabled", () => {
    render(<AgentPresence status="disabled" presence={idle} />);
    expect(screen.getByRole("status").textContent).toBe("Agent disconnected");
  });

  it("shows the connecting label while connecting", () => {
    render(<AgentPresence status="connecting" presence={idle} />);
    expect(screen.getByRole("status").textContent).toBe("Connecting to agent…");
  });
});
