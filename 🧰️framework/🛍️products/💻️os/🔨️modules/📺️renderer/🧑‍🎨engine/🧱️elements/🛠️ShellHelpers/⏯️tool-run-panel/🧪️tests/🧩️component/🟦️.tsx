/** @emoji 🧪️ The framework ToolRun panel the plugin runtime renders for a running run (`🔌️plugin/🧫️fixtures/⏯️tool-run/
 * 🪧️panel-running.json`, pinned by the Rust `tool_run_panel_of_a_running_run_is_the_shell_fixture` law) mounts through the
 * shell's panel-tab path: a labelled run group, a real progressbar, keyboard-reachable buttons that dispatch the run's own
 * actions, and the reveal rule that opens the panel when a run starts. dom-testing-library's role queries and
 * `@testing-library/user-event` keyboard navigation are the third-party oracles. */
import { afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment } from "react";
import { cleanup, render, screen } from "@semio-tech/ui-react/test";
import panelFixture from "../../../../../../../🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json";
import "../../../../🐚️Shell/🟦️.tsx";
import { uiNodeToTreePanelConfig } from "../../../🟦️.tsx";
import { toolRunPanelReveal } from "../../🟦️.ts";

const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧾️ The Rust wire omits defaulted style and accessibility fields; the shell's `BuiltNode` spells them out. */
function hostNode(node: any): any {
  return { ...node, style: { ...STYLE, ...node.style }, accessibility: { ...ACCESSIBILITY, ...node.accessibility }, menu: node.menu ?? null, children: node.children.map(hostNode) };
}

describe("⏯️ framework ToolRun panel", () => {
  afterEach(() => cleanup());

  it("renders a running run's group with its progressbar and dispatches its buttons with the run's identity", async () => {
    const dispatched: any[] = [];
    const body = hostNode(panelFixture);
    const config = uiNodeToTreePanelConfig(body, (action) => dispatched.push(action), "framework.panel.toolRun");
    render(createElement(Fragment, null, config.emptyState));
    expect(document.querySelector('[aria-label="Toy fill"]')?.getAttribute("data-ui-node-key")).toBe("framework.toolRun.1");
    expect(screen.getByRole("progressbar")).toBeTruthy();
    const pause = screen.getByRole("button", { name: "Pause" });
    const disabled = (name: string) => { const button = screen.getByRole("button", { name }) as HTMLButtonElement; return button.disabled || button.getAttribute("aria-disabled") === "true"; };
    expect(["Pause", "Step", "Abort", "Finalize"].map(disabled)).toEqual([false, true, false, false]);
    pause.focus();
    expect(document.activeElement).toBe(pause);
    pause.click();
    screen.getByRole("button", { name: "Abort" }).click();
    const actions = dispatched.map((action) => ({ action: action.action, args: action.args }));
    expect(actions).toEqual([
      { action: "toolRunPause", args: { generation: 0, runId: "1" } },
      { action: "toolRunAbort", args: { generation: 0, runId: "1" } },
    ]);
  });

  it("reveals the panel exactly when a run it has not seen appears", () => {
    const body = hostNode(panelFixture);
    const first = toolRunPanelReveal(new Set(), body);
    expect([[...first.runs], first.added]).toEqual([[1n], [1n]]);
    const again = toolRunPanelReveal(first.runs, body);
    expect(again.added).toEqual([]);
    expect(toolRunPanelReveal(new Set(), undefined).added).toEqual([]);
  });
});
