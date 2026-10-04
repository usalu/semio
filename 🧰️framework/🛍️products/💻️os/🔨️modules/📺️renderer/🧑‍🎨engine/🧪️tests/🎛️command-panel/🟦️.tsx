// @vitest-environment jsdom

import React from "react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import { actionSemanticsForKind } from "@semio-tech/framework";
import { Tree } from "@semio-tech/ui-react";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { buildCommandCategoryTree, commandAddressKey, type ResolvedCommand } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import fixture from "../../🧱️elements/🐚️Shell/🧫️fixtures/🎛️command-panel/🔣️.json";
import schema from "../../🧱️elements/🐚️Shell/🧬️schema/🎛️command-panel/🔣️.json";

beforeEach(() => { vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} }); });
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

it("validates the neutral command panel contract", () => {
  const validate = new Ajv2020().compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

for (const locale of fixture.locales as ("en" | "de")[]) {
  const commands: ResolvedCommand[] = fixture.commands.map(row => ({
    address: row.address as ResolvedCommand["address"],
    definition: { id: row.address.commandId, label: row.labels[locale], category: row.category, iconId: "wrench", semantics: actionSemanticsForKind("shell"), kind: "shell", keybindings: [], inPalette: true,
      args: row.args.map(arg => ({ id: arg.id, label: arg.labels[locale], schema: { kind: "string", options: [] }, required: arg.required })),
    },
  }));

  it(`React command rows activate each qualified owner in ${locale}`, () => {
    const onExecute = vi.fn();
    const entries = commands.filter(entry => entry.definition.category === "layout");
    const tree = buildCommandCategoryTree(entries, null, {}, onExecute, vi.fn(), vi.fn(), vi.fn());
    const mounted = render(<Tree sections={tree.sections} />);
    expect(mounted.container.querySelectorAll(`[role="${fixture.rowRole}"]`)).toHaveLength(entries.length);
    for (const entry of entries) {
      fireEvent.click(mounted.getByText(entry.definition.label, { exact: true }));
      expect(onExecute).toHaveBeenLastCalledWith(entry);
    }
    expect(onExecute.mock.calls.map(([entry]) => commandAddressKey(entry.address))).toEqual(fixture.zeroArgumentKeys);
  });

  it(`React command rows expand a staged form and auto-expand its singleton in ${locale}`, () => {
    const onExecute = vi.fn(), onToggle = vi.fn(), onStage = vi.fn(), onReset = vi.fn();
    const entries = commands.filter(entry => entry.definition.category === "appearance");
    const tree = (expanded: string | null, staged = {}) => buildCommandCategoryTree(entries, expanded, staged, onExecute, onToggle, onStage, onReset);
    const initial = tree(null);
    const mounted = render(<Tree sections={initial.sections} />);
    const entry = entries.find(entry => commandAddressKey(entry.address) === fixture.expandKey)!;
    fireEvent.click(mounted.getByText(`${entry.definition.label}…`, { exact: true }));
    expect(onToggle).toHaveBeenLastCalledWith(fixture.expandKey);
    expect(onExecute).not.toHaveBeenCalled();
    const expanded = tree(fixture.expandKey);
    expect(expanded.sections.map(section => section.id)).toEqual(["command.category.appearance.form", "command.category.list"]);
    expect(expanded.sections[0].actions?.map(action => action.id?.split("-").at(-1))).toEqual(fixture.formActions);
    expect(expanded.sections[0].actions?.[0].disabled).toBe(true);
    mounted.rerender(<Tree sections={expanded.sections} />);
    expect(mounted.container.querySelector(`[id="${expanded.sections[0].items?.[0]?.id}.disclosureLabel"]`)?.textContent).toBe(entry.definition.args[0].label);
    const header = () => mounted.container.querySelector(`[id="${expanded.sections[0].id}"]`)!;
    expect(header().getAttribute("aria-expanded")).toBe(String(fixture.formDefaultOpen));
    const arg = () => mounted.container.querySelector(`[id="${entry.definition.args[0].id}"]`)!;
    expect(arg().closest('[data-slot="collapsible-content"]')?.hasAttribute("hidden")).toBe(true);
    fireEvent.click(header());
    expect(arg().closest('[data-slot="collapsible-content"]')?.hasAttribute("hidden")).toBe(false);
    fireEvent.change(arg(), { target: { value: fixture.staged.themeId } });
    expect(onStage).toHaveBeenLastCalledWith(fixture.expandKey, "themeId", fixture.staged.themeId);
    const ready = tree(fixture.expandKey, { [fixture.expandKey]: fixture.staged });
    const execute = ready.sections[0].actions![0], reset = ready.sections[0].actions![1];
    if (execute.kind === "checkbox" || reset.kind === "checkbox") throw new Error("Command form requires button actions");
    expect(execute.disabled).toBe(false);
    mounted.rerender(<Tree sections={ready.sections} />);
    fireEvent.click(mounted.container.querySelector(`[id="${execute.id}"]`)!);
    expect(onExecute).toHaveBeenLastCalledWith(entry, fixture.staged);
    expect(header().getAttribute("aria-expanded")).toBe("true");
    fireEvent.click(mounted.container.querySelector(`[id="${reset.id}"]`)!);
    expect(onReset).toHaveBeenLastCalledWith(fixture.expandKey);
    const singleton = buildCommandCategoryTree(commands.filter(entry => commandAddressKey(entry.address) === fixture.singletonKey), null, {}, onExecute, onToggle, onStage, onReset);
    expect(singleton.sections.map(section => section.id)).toEqual(["command.category.general.form"]);
  });
}
