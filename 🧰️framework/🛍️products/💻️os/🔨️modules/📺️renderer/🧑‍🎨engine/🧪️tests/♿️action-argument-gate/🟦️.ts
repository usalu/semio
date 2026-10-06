
import { createElement } from "react";
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { Tree } from "@semio-tech/ui-react";
import { actionSemanticsForKind } from "@semio-tech/framework";
import { buildActionCategoryTree, buildCommandCategoryTree, syncShellLabelLocale, type ResolvedActionDefinition, type ResolvedCommand } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { actionStageKey } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import fixture from "./🧫️fixtures/🔣️.json";

afterEach(cleanup);



for (const row of fixture.cases) for (const kind of ["action", "command"] as const) {
  it(`${row.id} ${kind} explains the required arguments without dispatching unavailable execution`, () => {
    syncShellLabelLocale(row.locale as "en" | "de");
    const execute = vi.fn();
    const args = ["depth", "segments"].map((id, index) => ({ id, label: row.labels[index]!, schema: { kind: "number" as const, integer: false }, required: true, ...(row.defaults ? { default: index === 0 ? 3 : 2 } : {}) }));
    const action: ResolvedActionDefinition = { id: "fixture", label: row.locale === "en" ? "Shape" : "Form", iconId: "box", kind: "mutation", semantics: actionSemanticsForKind("mutation"), inPalette: true, category: "actions", args };
    const command: ResolvedCommand = { definition: { ...action, category: "actions", keybindings: [] }, address: { owner: "os", commandId: "fixture" } };
    const sections = kind === "action"
      ? buildActionCategoryTree("w", "c", [action], "fixture", { [actionStageKey("w", "fixture")]: row.staged }, false, vi.fn(), vi.fn(), vi.fn(), execute)
      : buildCommandCategoryTree([command], null, { "os:fixture": row.staged }, execute, vi.fn(), vi.fn(), vi.fn()).sections;
    const { container } = render(createElement(Tree, { sections, sortableSections: false }));
    const button = [...container.querySelectorAll("button")].find((entry) => entry.id.endsWith("execute"))!;
    expect(button).toBeDefined();
    const description = button.getAttribute("aria-describedby");
    expect(description ? document.getElementById(description)?.textContent : null).toBe(row.expectedReason);
    const oracle = document.createElement("button");
    oracle.disabled = false;
    if (row.expectedDisabled) oracle.setAttribute("aria-disabled", "true");
    document.body.append(oracle);
    expect([button.disabled, button.getAttribute("aria-disabled")]).toEqual([oracle.disabled, oracle.getAttribute("aria-disabled")]);
    button.focus();
    expect(document.activeElement).toBe(button);
    fireEvent.click(button);
    expect(execute).toHaveBeenCalledTimes(row.expectedDispatch ? 1 : 0);
    oracle.remove();
  });
}
