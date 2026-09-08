import * as React from "react";
import { createMemoryStoragePort } from "@semio-tech/framework";
import Ajv from "ajv";
import { computeAccessibleDescription, computeAccessibleName } from "dom-accessibility-api";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { uiI18n } from "../../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx";
import { UiKeybindingsProvider } from "../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { UIDialog, type UIDialogProps } from "../🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../🔽️Select/🟦️.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "../../🗨️Popover/🟦️.tsx";
import { createShellScope, ShellScopeProvider } from "../../🐚️ShellScope/🟦️.tsx";
import fixture from "../🧫️fixtures/♿️modal/🔣️.json";
import uiSchema from "../../../🧬️schema/🔣️.json";

const localized = (key: "title" | "description" | "field" | "kind" | "submit" | "cancel") => ({ native: Object.fromEntries(fixture.cases.map(row => [row.locale, row[key]])) });
const definition: UIDialogProps["dialog"] = {
  id: "createArtifact", title: localized("title"), body: localized("description"),
  args: [
    { id: "name", label: localized("field"), required: true, schema: { kind: "string", options: [] } },
    { id: "kindChoice", label: localized("kind"), required: true, default: "map", schema: { kind: "string", options: [{ value: "map", label: { native: { en: "Map", de: "Karte" } } }] } },
  ],
  submitAction: "createArtifact", submitLabel: localized("submit"), cancelAction: "cancelArtifact", cancelLabel: localized("cancel"),
};
const renderField: UIDialogProps["renderField"] = (def, value, change, field) => def.id === "kindChoice"
  ? <select id={field?.id} aria-labelledby={field?.labelledBy} required={field?.required} value={String(value ?? "")} onChange={event => change(event.target.value)}><option value="map">Map</option></select>
  : <input id={field?.id} aria-labelledby={field?.labelledBy} required={field?.required} value={String(value ?? "")} onChange={event => change(event.target.value)} />;

describe("UIDialog accessibility", () => {
  it("isolates and dismisses dialogs within each owning Shell without blocking a sibling Shell", async () => {
    expect(new Ajv({ strict: true }).addSchema(uiSchema).getSchema(`${uiSchema.$id}#/$defs/UIDialogModalFixture`)!(fixture)).toBe(true);
    await uiI18n.changeLanguage("en");
    const bodySibling = document.createElement("button");
    bodySibling.textContent = "Page action";
    const rootA = document.createElement("section"), appA = document.createElement("div"), portalA = document.createElement("div");
    const rootB = document.createElement("section"), appB = document.createElement("div"), portalB = document.createElement("div");
    rootA.style.position = rootB.style.position = "relative";
    portalA.style.position = portalB.style.position = "absolute";
    rootA.append(appA, portalA);
    rootB.append(appB, portalB);
    document.body.append(bodySibling, rootA, rootB);
    const scopeA = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en", shellId: "dialog-shell-a" });
    const scopeB = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en", shellId: "dialog-shell-b" });
    scopeA.rootRef.current = rootA;
    scopeA.portalLayerRef.current = portalA;
    scopeB.rootRef.current = rootB;
    scopeB.portalLayerRef.current = portalB;
    const cancelA = vi.fn(), cancelB = vi.fn(), siblingAction = vi.fn();
    const scopedRenderField: UIDialogProps["renderField"] = (def, value, change, field) => def.id !== "kindChoice" ? renderField(def, value, change, field) : <Select value={String(value)} onValueChange={change}><SelectTrigger id={field.id} aria-labelledby={field.labelledBy}><SelectValue /></SelectTrigger><SelectContent><SelectItem value="map">Map</SelectItem><SelectItem value="terrain">Terrain</SelectItem></SelectContent></Select>;
    const viewA = render(<ShellScopeProvider scope={scopeA}><UIDialog dialog={definition} renderField={scopedRenderField} onCancel={cancelA} onSubmit={vi.fn()} /></ShellScopeProvider>, { container: appA, baseElement: rootA });
    const viewB = render(<ShellScopeProvider scope={scopeB}><button type="button" onClick={siblingAction}>Sibling action</button></ShellScopeProvider>, { container: appB, baseElement: rootB });
    try {
      const dialogA = rootA.querySelector<HTMLElement>('[role="dialog"]')!;
      expect(dialogA.getAttribute("aria-modal")).toBe(fixture.scoped.modal);
      expect(appA.hasAttribute("inert")).toBe(fixture.scoped.ownAppIsolated);
      expect(rootB.hasAttribute("inert")).toBe(fixture.scoped.siblingShellIsolated);
      expect(bodySibling.hasAttribute("inert")).toBe(fixture.scoped.bodySiblingIsolated);
      expect(rootA.querySelector<HTMLElement>('[data-slot="dialog-overlay"]')?.style.position).toBe(fixture.scoped.position);
      fireEvent.click(rootA.querySelector<HTMLElement>('[role="combobox"]')!);
      const pickerA = rootA.querySelector<HTMLElement>('[role="listbox"]')!;
      expect(pickerA.style.position).toBe(fixture.scoped.position);
      fireEvent.click(viewB.getByRole("button", { name: "Sibling action" }));
      expect(siblingAction).toHaveBeenCalledTimes(1);
      expect(rootA.querySelector('[role="listbox"]')).not.toBeNull();
      fireEvent.keyDown(viewB.getByRole("button", { name: "Sibling action" }), { key: "Escape" });
      expect(cancelA).toHaveBeenCalledTimes(fixture.scoped.foreignEscapeCancels);
      viewB.rerender(<ShellScopeProvider scope={scopeB}><UIDialog dialog={definition} renderField={scopedRenderField} onCancel={cancelB} onSubmit={vi.fn()} /></ShellScopeProvider>);
      fireEvent.click(rootB.querySelector<HTMLElement>('[role="combobox"]')!);
      const pickerB = rootB.querySelector<HTMLElement>('[role="listbox"]')!;
      expect(rootA.querySelector('[role="listbox"]')).not.toBeNull();
      fireEvent.keyDown(pickerB, { key: "Escape" });
      expect(rootB.querySelector('[role="listbox"]')).toBeNull();
      expect(rootA.querySelector('[role="listbox"]')).not.toBeNull();
      fireEvent.keyDown(pickerA, { key: "Escape" });
      expect(rootA.querySelector('[role="listbox"]')).toBeNull();
      const fieldB = rootB.querySelector<HTMLInputElement>('input[required]')!;
      fireEvent.keyDown(fieldB, { key: "Escape" });
      expect(cancelB).toHaveBeenCalledTimes(fixture.scoped.ownEscapeCancels);
      expect(cancelA).toHaveBeenCalledTimes(fixture.scoped.foreignEscapeCancels);
      const fieldA = rootA.querySelector<HTMLInputElement>('input[required]')!;
      fireEvent.keyDown(fieldA, { key: "Escape" });
      expect(cancelA).toHaveBeenCalledTimes(fixture.scoped.ownEscapeCancels);
    } finally {
      viewA.unmount();
      viewB.unmount();
      expect(appA.hasAttribute("inert")).toBe(false);
      expect(appB.hasAttribute("inert")).toBe(false);
      bodySibling.remove();
      rootA.remove();
      rootB.remove();
    }
  });

  it("retains fields and actions inside their owning Shell automation and tutorial scope", async () => {
    const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "de", shellId: "dialog-owner" });
    await scope.i18n.changeLanguage("de");
    const root = document.createElement("div");
    const app = document.createElement("div");
    const portal = document.createElement("div");
    root.append(app, portal);
    document.body.append(root);
    scope.rootRef.current = root;
    scope.portalLayerRef.current = portal;
    const view = render(<ShellScopeProvider scope={scope}><UIDialog dialog={definition} renderField={renderField} onCancel={vi.fn()} onSubmit={vi.fn()} /></ShellScopeProvider>, { container: app, baseElement: root });
    try {
      expect(scope.query("#kindChoice")).toBe(view.getByRole("combobox", { name: "Art" }));
      expect(scope.query('[id="ui.dialog.submit"]')).toBe(view.getByRole("button", { name: "Erstellen" }));
      expect(portal.querySelector('[data-slot="dialog-portal"]')).not.toBeNull();
      expect(root.getAttribute("inert")).toBeNull();
    } finally {
      view.unmount();
      root.remove();
    }
  });

  it("keeps a nested owned editor popover inside modal isolation and dismisses the child first", async () => {
    await uiI18n.changeLanguage("en");
    const cancel = vi.fn();
    const view = render(<UIDialog dialog={definition} onCancel={cancel} onSubmit={vi.fn()} renderField={(def, value, change, field) => def.id !== "name" ? renderField(def, value, change, field) : <Popover><PopoverTrigger aria-labelledby={field.labelledBy}>Edit</PopoverTrigger><PopoverContent aria-label="Name editor"><input aria-label="Edited name" value={String(value ?? "")} onChange={event => change(event.target.value)} /></PopoverContent></Popover>} />);
    const trigger = view.getByRole("button", { name: "Name" });
    fireEvent.click(trigger);
    const field = view.getByRole("textbox", { name: "Edited name" });
    expect(document.activeElement).toBe(field);
    fireEvent.keyDown(field, { key: "Escape" });
    expect(cancel).not.toHaveBeenCalled();
    expect(view.queryByRole("dialog", { name: "Name editor" })).toBeNull();
    expect(document.activeElement).toBe(trigger);
    fireEvent.keyDown(trigger, { key: "Escape" });
    expect(cancel).toHaveBeenCalledTimes(1);
  });

  it("keeps a nested owned kind picker focusable and dismisses it before its dialog", async () => {
    await uiI18n.changeLanguage("en");
    const cancel = vi.fn();
    const submit = vi.fn();
    const view = render(<UIDialog dialog={definition} seedArgs={{ name: "Map C" }} onCancel={cancel} onSubmit={submit} renderField={(def, value, change, field) => def.id !== "kindChoice" ? renderField(def, value, change, field) : <Select value={String(value)} onValueChange={change}><SelectTrigger id={field.id} aria-labelledby={field.labelledBy}><SelectValue /></SelectTrigger><SelectContent><SelectItem value="map">Map</SelectItem><SelectItem value="terrain">Terrain</SelectItem></SelectContent></Select>} />);
    const picker = view.getByRole("combobox", { name: "Kind" });
    fireEvent.click(picker);
    const options = view.getByRole("listbox");
    expect(document.activeElement).toBe(options);
    fireEvent.keyDown(options, { key: "Escape" });
    expect(cancel).not.toHaveBeenCalled();
    expect(view.queryByRole("listbox")).toBeNull();
    expect(document.activeElement).toBe(picker);
    fireEvent.click(picker);
    fireEvent.click(view.getByRole("option", { name: "Terrain" }));
    expect(cancel).not.toHaveBeenCalled();
    expect(submit).not.toHaveBeenCalled();
    fireEvent.click(view.getByRole("button", { name: "Create" }));
    expect(submit).toHaveBeenLastCalledWith({ name: "Map C", kindChoice: "terrain" });
    fireEvent.keyDown(picker, { key: "Escape" });
    expect(cancel).toHaveBeenCalledTimes(1);
    console.log("[DEBUG] UIDialog accessibility: owned-picker focus=1 child-first-escape=1 selection=terrain");
  });

  it.each(fixture.cases)("implements the neutral modal contract in $locale with an independent accessibility oracle", async row => {
    expect(new Ajv({ strict: true }).addSchema(uiSchema).getSchema(`${uiSchema.$id}#/$defs/UIDialogModalFixture`)!(fixture)).toBe(true);
    await uiI18n.changeLanguage(row.locale);
    const opener = document.createElement("button");
    opener.textContent = "Opener";
    document.body.append(opener);
    opener.focus();
    const cancel = vi.fn();
    const submit = vi.fn();
    const view = render(<UIDialog dialog={definition} renderField={renderField} onCancel={cancel} onSubmit={submit} />);
    const modal = view.getByRole(fixture.expected.role, { name: row.title });
    expect(modal.getAttribute("aria-modal")).toBe(fixture.expected.modal);
    expect(computeAccessibleName(modal)).toBe(row.title);
    expect(computeAccessibleDescription(modal)).toBe(row.description);
    expect(view.container.getAttribute("aria-hidden")).toBe(fixture.expected.isolated);
    expect(view.container.hasAttribute("inert")).toBe(true);
    const field = view.getByRole("textbox", { name: row.field });
    expect(computeAccessibleName(field)).toBe(row.field);
    expect(computeAccessibleName(view.getByRole("combobox", { name: row.kind }))).toBe(row.kind);
    expect(field.getAttribute("required")).not.toBeNull();
    expect(document.activeElement).toBe(field);
    fireEvent.keyDown(field, { key: "Enter" });
    expect(submit).toHaveBeenCalledTimes(fixture.expected.requiredMissingSubmits);
    fireEvent.change(field, { target: { value: "Map A" } });
    const submitButton = view.getByRole("button", { name: row.submit });
    submitButton.focus();
    fireEvent.keyDown(submitButton, { key: "Tab" });
    expect(document.activeElement).toBe(field);
    fireEvent.keyDown(field, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(submitButton);
    const cancelButton = view.getByRole("button", { name: row.cancel });
    cancelButton.focus();
    fireEvent.keyDown(cancelButton, { key: "Enter" });
    expect(submit).toHaveBeenCalledTimes(fixture.expected.buttonEnterSubmits);
    field.focus();
    fireEvent.keyDown(field, { key: "Enter" });
    expect(submit).toHaveBeenCalledTimes(fixture.expected.validSubmits);
    expect(submit).toHaveBeenLastCalledWith({ name: "Map A", kindChoice: "map" });
    fireEvent.keyDown(field, { key: "Escape" });
    expect(cancel).toHaveBeenCalledTimes(fixture.expected.escapeCancels);
    view.unmount();
    expect(document.activeElement === opener).toBe(fixture.expected.focusReturns);
    expect(view.container.hasAttribute("inert")).toBe(false);
    opener.remove();
    console.log(`[DEBUG] UIDialog accessibility: locale=${row.locale} modal=1 named-fields=2 exact-escape=1 focus-return=1`);
  });

  it("keeps remapped chords local to the modal and routes outside dismissal once", async () => {
    await uiI18n.changeLanguage("en");
    const cancel = vi.fn();
    const submit = vi.fn();
    const view = render(<UiKeybindingsProvider bindings={new Map([["ui.dialog.cancel", "alt+x"], ["ui.dialog.submit", "ctrl+enter"]])}><UIDialog dialog={definition} seedArgs={{ name: "Map B" }} renderField={renderField} onCancel={cancel} onSubmit={submit} /></UiKeybindingsProvider>);
    fireEvent.keyDown(window, { key: "Enter", ctrlKey: true });
    expect(submit).not.toHaveBeenCalled();
    const field = view.getByRole("textbox", { name: "Name" });
    fireEvent.keyDown(field, { key: "Escape" });
    expect(cancel).not.toHaveBeenCalled();
    fireEvent.keyDown(field, { key: "Enter", ctrlKey: true });
    expect(submit).toHaveBeenCalledTimes(1);
    fireEvent.keyDown(field, { key: "x", altKey: true });
    expect(cancel).toHaveBeenCalledTimes(1);
    cancel.mockClear();
    fireEvent.pointerDown(document.querySelector('[data-slot="dialog-overlay"]')!);
    expect(cancel).toHaveBeenCalledTimes(fixture.expected.outsideCancels);
  });
});
