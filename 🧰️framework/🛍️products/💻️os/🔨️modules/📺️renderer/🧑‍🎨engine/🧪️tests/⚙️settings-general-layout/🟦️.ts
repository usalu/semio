/** ⚙️ Neutral and React oracle for General Tree projection and bottom-panel flow. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { fireEvent, render } from "@testing-library/react";
import { createElement as h, useState } from "react";
import { describe, expect, test } from "vitest";
import { Button } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔘️Button/🟦️.tsx";
import { Input } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx";
import { Stepper } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx";
import { Toggle } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🟦️.tsx";
import { Tree } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const frameworkRoot = join(engineRoot, "..", "..", "..", "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "⚙️settings-general-layout", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "⚙️settings-general-layout", "🔣️.json"), "utf8"));

const bottomGeometry = ({ panel, tabRows, rowHeight, contentInset }: typeof fixture.bottomPanel) => {
  const barHeight = tabRows * rowHeight;
  return {
    content: { x: panel.x + contentInset, y: panel.y + contentInset, w: panel.w - contentInset * 2, h: panel.h - barHeight - contentInset * 2 },
    rootTabY: panel.y + panel.h - rowHeight,
    leafTabY: panel.y + panel.h - tabRows * rowHeight,
    dividerY: panel.y + panel.h - barHeight,
  };
};

const upFlowHeaderRect = ({ sectionRect, headerHeight }: typeof fixture.upFlowDisclosure) => ({
  x: sectionRect.x,
  y: sectionRect.y + sectionRect.h - headerHeight,
  w: sectionRect.w,
  h: Math.min(headerHeight, sectionRect.h),
});

function DriverPublicationHarness() {
  const [driver, setDriver] = useState("compact");
  const [labels, setLabels] = useState("icons");
  const [saveLabel, setSaveLabel] = useState("");
  const [custom, setCustom] = useState(false);
  return h(
    "div",
    null,
    h(Select, { value: driver, onValueChange: setDriver }, h(SelectTrigger, { "aria-label": "framework.settings.driver" }, h(SelectValue)), h(SelectContent, null, h(SelectItem, { value: "default" }, "default"), h(SelectItem, { value: "compact" }, "compact"), h(SelectItem, { value: "custom.studio-driver" }, "custom.studio-driver"))),
    h(Select, { value: labels, onValueChange: setLabels }, h(SelectTrigger, { "aria-label": "framework.settings.driver.labels" }, h(SelectValue)), h(SelectContent, null, h(SelectItem, { value: "icons" }, "icons"), h(SelectItem, { value: "full" }, "full"))),
    h(Input, { "aria-label": "framework.settings.driver.saveLabel", value: saveLabel, onChange: (event) => setSaveLabel(event.currentTarget.value) }),
    h(Button, { "aria-label": "framework.settings.driver.save", disabled: saveLabel.trim() === "", text: "save", onClick: () => { setDriver("custom.studio-driver"); setSaveLabel(""); setCustom(true); } }),
    custom ? h(Button, { "aria-label": "framework.settings.driver.delete", text: "delete", onClick: () => { setDriver("default"); setCustom(false); } }) : null,
  );
}

describe("⚙️ General Settings Tree and bottom-panel flow", () => {
  test("the language-neutral layout fixture satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("an independent upward-flow oracle reserves bottom tabs outside retained content", () => {
    expect(bottomGeometry(fixture.bottomPanel)).toEqual(fixture.bottomPanel.expected);
    const contentBottom = fixture.bottomPanel.expected.content.y + fixture.bottomPanel.expected.content.h;
    expect(contentBottom).toBeLessThanOrEqual(fixture.bottomPanel.expected.leafTabY);
    expect(fixture.bottomPanel.expected.rootTabY).toBeGreaterThan(fixture.bottomPanel.expected.leafTabY);
  });

  test("React and the neutral geometry keep an Up-flow disclosure header below its expanded children", () => {
    const contract = fixture.upFlowDisclosure;
    expect(upFlowHeaderRect(contract)).toEqual(contract.expectedHeaderRect);
    expect(contract.expectedHeaderRect.y).toBeGreaterThanOrEqual(contract.childRect.y + contract.childRect.h);
    const treeSource = readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🧱️elements", "🌳️Tree", "🟦️.tsx"), "utf8");
    const start = treeSource.indexOf('{direction === "up" ? (');
    const branch = treeSource.slice(start, treeSource.indexOf("</Collapsible>", start));
    expect(branch.indexOf("{" + contract.react.order[0] + "}")).toBeLessThan(branch.indexOf("{" + contract.react.order[1] + "}"));
  });

  test("the neutral inline-control contract derives the React value column and per-kind heights from shared spacing", () => {
    const contract = fixture.inlineControls;
    expect(contract.valueColumnUiSpacing * contract.uiSpacingPx).toBe(contract.expectedValueColumnWidthPx);
    for (const vector of contract.cases) {
      const heightSpacing = vector.heightMode === "small" ? contract.smallControlHeightUiSpacing : contract.defaultControlHeightUiSpacing;
      expect(heightSpacing * contract.uiSpacingPx, vector.controlId).toBeCloseTo(vector.expectedHeightPx);
      expect(vector.expectedWidthPx, vector.controlId).toBe(vector.widthMode === "fill" ? contract.expectedValueColumnWidthPx : null);
    }
  });

  test("the mounted React Tree publishes its token-backed value column and each control kind's authored size", () => {
    const contract = fixture.inlineControls;
    const controls: Record<string, ReturnType<typeof h>> = {
      select: h(
        Select,
        { id: "framework.settings.appearance", defaultValue: "system" },
        h(SelectTrigger, { id: "framework.settings.appearance", className: "h-small w-32", size: "sm" }, h(SelectValue)),
        h(SelectContent, null, h(SelectItem, { value: "system" }, "System")),
      ),
      input: h(Input, { id: "framework.settings.driver.saveLabel", className: "h-small w-32", value: "Driver", readOnly: true }),
      numberStepper: h(Stepper, { id: "fixture.settings.numberStepper", value: 1, min: 0, max: 2 }),
      toggle: h(Toggle, { id: "fixture.settings.toggle", icon: "circle", pressed: false }),
      button: h(Button, { id: "framework.settings.resetDock", icon: "rotate-ccw", text: "Reset" }),
    };
    const view = render(
      h(Tree, {
        sections: [
          {
            id: "fixture.settings.controls",
            label: "Controls",
            defaultOpen: true,
            items: contract.cases.map((vector: (typeof contract.cases)[number]) => ({ id: `row.${vector.kind}`, label: vector.kind, control: controls[vector.kind] })),
          },
        ],
      }),
    );

    for (const vector of contract.cases) {
      const row = document.getElementById(`row.${vector.kind}`);
      expect(row, vector.controlId).not.toBeNull();
      const control = row!.querySelector<HTMLElement>("[data-slot='tree-item-control']");
      expect(control, vector.controlId).not.toBeNull();
      expect(control!.parentElement?.style.gridTemplateColumns, vector.controlId).toBe(contract.react.gridTemplateColumns);
      expect(control!.closest<HTMLElement>("[data-slot='tree']")?.style.getPropertyValue("--tree-value-column"), vector.controlId).toBe(contract.react.valueColumn);
      const widthOwner = control!.querySelector<HTMLElement>("[data-detail-panel-control]");
      expect(widthOwner?.dataset.detailPanelControl, vector.controlId).toBe(vector.widthMode === "fill" ? contract.react.fillAttribute : contract.react.fitAttribute);
      const heightOwner =
        vector.kind === "select"
          ? control!.querySelector<HTMLElement>("[data-slot='select-trigger']")
          : vector.kind === "input"
            ? control!.querySelector<HTMLElement>("[data-slot='input']")
            : vector.kind === "numberStepper"
              ? control!.querySelector<HTMLElement>("[data-slot='stepper-group']")
              : vector.kind === "toggle"
                ? control!.querySelector<HTMLElement>("[data-slot='toggle-group']")
                : control!.querySelector<HTMLElement>("[data-slot='button-group']");
      expect(heightOwner?.className.split(/\s+/), vector.controlId).toContain(vector.heightMode === "small" ? "h-small" : "h-medium");
    }
    view.unmount();
  });

  test("React authors the same Tree sections, row controls, and upward panel ownership", () => {
    const panelSource = readFileSync(join(engineRoot, "🧱️elements", "📌️ChromePanels", "🟦️.tsx"), "utf8");
    const panelHost = readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🧱️elements", "🖼️Panel", "🟦️.tsx"), "utf8");
    const reactTarget = readFileSync(join(frameworkRoot, "🔨️modules", "🖱️ui", "🎯️targets", "⚛️react", "🟦️.tsx"), "utf8");
    for (const section of fixture.tree.sections) {
      expect(panelSource).toContain(`id: "${section.id}"`);
      expect(panelSource).toContain(`defaultOpen: ${section.defaultOpen}`);
      for (const id of section.itemIds) {
        const driverAxis = id.match(/^framework\.settings\.driver\.(labels|labelTier|drag|chrome|gumball|tooltips|hotkeys)$/)?.[1];
        if (driverAxis) expect(panelSource).toContain(`driverAxisSelectRow("${driverAxis}"`);
        else expect(panelSource).toContain(id);
      }
    }
    for (const control of fixture.tree.controls) {
      expect(panelSource).toContain(control.rowId);
      expect(panelSource).toContain(control.controlId);
    }
    expect(panelHost).toContain('isBottom ? "flex-col-reverse" : "flex-col"');
    expect(panelHost).toContain(`viewportClassName={isBottom ? "flex min-h-full flex-col ${fixture.stableGeneral.react.viewportJustification}" : undefined}`);
    const anchorStyle = reactTarget.slice(reactTarget.indexOf("export function anchorPositionStyle"), reactTarget.indexOf("export function chromeHostedOpenPanelPositionStyle"));
    expect(anchorStyle).toContain(`${fixture.stableGeneral.react.extentProperty}:`);
    expect(fixture.stableGeneral.react.bondedEdge).toBe("bottom");
    expect(anchorStyle).toContain('style[vertical] = "var(--spacing-single)"');
    expect(anchorStyle).not.toMatch(/\bheight\s*:/);
  });

  test("React commits one appearance value and retires its listbox in the same interaction", () => {
    const commit = fixture.selectCommit;
    const values: string[] = [];
    const view = render(
      h(
        Select,
        { id: commit.controlId, defaultValue: commit.initialValue, onValueChange: (value: string) => values.push(value) },
        h(SelectTrigger, { "aria-label": commit.controlId }, h(SelectValue)),
        h(
          SelectContent,
          null,
          h(SelectItem, { value: commit.initialValue }, "System"),
          h(SelectItem, { value: commit.nextValue }, "Dark"),
        ),
      ),
    );
    fireEvent.click(view.getByRole("combobox", { name: commit.controlId }));
    fireEvent.click(view.getByRole("option", { name: "Dark" }));
    expect(values).toEqual([commit.nextValue]);
    expect(view.queryByRole("listbox")).toBeNull();
    expect(view.getByRole("combobox", { name: commit.controlId }).textContent).toContain("Dark");
    view.unmount();
  });

  test("mounted React preferences expose each host-only or full-refresh publication lane", () => {
    const publication = fixture.retainedPreferencePublication;
    for (const vector of publication.cases) {
      expect(vector.publicationLane).toBe(vector.requiresGuestRefresh ? "settle" : "dispatch");
      const values: string[] = [];
      const view = render(
        h(
          Select,
          { id: vector.controlId, defaultValue: vector.initialValue, onValueChange: (value: string) => values.push(value) },
          h(SelectTrigger, { "aria-label": vector.controlId }, h(SelectValue)),
          h(
            SelectContent,
            null,
            h(SelectItem, { value: vector.initialValue }, vector.initialValue),
            h(SelectItem, { value: vector.nextValue }, vector.nextValue),
          ),
        ),
      );
      fireEvent.click(view.getByRole("combobox", { name: vector.controlId }));
      fireEvent.click(view.getByRole("option", { name: vector.nextValue }));
      expect(values, vector.action).toEqual([vector.nextValue]);
      expect(view.getByRole("combobox", { name: vector.controlId }).textContent, vector.action).toContain(vector.nextValue);
      view.unmount();
    }
  });

  test("mounted React driver controls publish draft, save and reset consequences immediately", () => {
    const workflow = fixture.retainedPreferencePublication.driverWorkflow;
    const view = render(h(DriverPublicationHarness));
    fireEvent.click(view.getByRole("combobox", { name: workflow[0].observations[0].controlId }));
    fireEvent.click(view.getByRole("option", { name: workflow[0].arguments.value }));
    expect(view.getByRole("combobox", { name: workflow[0].observations[0].controlId }).textContent).toContain("full");
    const label = view.getByRole("textbox", { name: workflow[1].observations[0].controlId });
    fireEvent.change(label, { target: { value: workflow[1].arguments.value } });
    expect((label as HTMLInputElement).value).toBe("Studio Driver");
    const save = view.getByRole("button", { name: "framework.settings.driver.save" }) as HTMLButtonElement;
    expect(save.disabled).toBe(false);
    fireEvent.click(save);
    expect(view.getByRole("combobox", { name: "framework.settings.driver" }).textContent).toContain("custom.studio-driver");
    expect((label as HTMLInputElement).value).toBe("");
    expect(save.disabled).toBe(true);
    fireEvent.click(view.getByRole("button", { name: "framework.settings.driver.delete" }));
    expect(view.getByRole("combobox", { name: "framework.settings.driver" }).textContent).toContain("default");
    expect(view.queryByRole("button", { name: "framework.settings.driver.delete" })).toBeNull();
  });

  test("WGPU publishes General directly as Tree rows and uses flow-owned panel geometry", () => {
    const shellSource = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    const start = shellSource.indexOf("pub(crate) fn build_settings_general_ui");
    const end = shellSource.indexOf("fn driver_rows", start);
    const general = shellSource.slice(start, end);
    expect(general).toContain("UiNode::Tree(UiTreeNode");
    expect(general).not.toContain("UiNode::Stack(UiStackNode");
    expect(shellSource).toContain("flowed_anchor_content_rect(anchor.flow().block");
    expect(shellSource).toContain("flowed_anchor_tab_row_y(anchor.flow().block");
    expect(shellSource).toContain("restart_viewport_after_host_reflow");
  });
});
