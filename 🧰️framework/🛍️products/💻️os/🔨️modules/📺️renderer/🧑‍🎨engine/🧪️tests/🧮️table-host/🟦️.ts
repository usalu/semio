/** 🧮 Table host rows, actions and stepper cells. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { DEFAULT_UI_DRIVER, UiDriverProvider } from "@semio-tech/ui-react";
import { tableStepperKeyboardFixture } from "@semio-tech/framework/ui/testing/fixtures";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { TableHost, tableStepperClampedDelta, tableStepperKeyDelta } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it("renders table host with ui-react table", () => {
    const markup = renderToStaticMarkup(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.catalogue",
          controllerId: "s-play",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
            rowsJson: JSON.stringify([{ label: "Draw" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-table-host");
    expect(markup).toContain("Draw");
  });

  it("stamps a row's own id onto the rendered row's data-row-id attribute", () => {
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "col0", label: "Name" }]),
            rowsJson: JSON.stringify([{ id: "space:abc", col0: { kind: "text", value: "Atelier" } }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(container.querySelector('[data-row-id="space:abc"]')).not.toBeNull();
  });

  it("dispatches a row action button's own ActionDescriptor, unmodified, on click", () => {
    const onAction = vi.fn();
    const { container } = render(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.space.home",
          controllerId: "s-home",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([
              { id: "col0", label: "Name" },
              { id: "actions", label: "" },
            ]),
            rowsJson: JSON.stringify([
              {
                id: "space:abc",
                col0: { kind: "text", value: "Atelier" },
                actions: { kind: "buttons", buttons: [{ iconId: "trash-2", label: "delete", action: { controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } } }] },
              },
            ]),
          },
        },
        onAction,
      }),
    );
    const button = container.querySelector('[data-row-id="space:abc"] button');
    if (!button) throw new Error("row action button not found");
    fireEvent.click(button);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "s-home", action: "deleteSpace", args: { spaceId: "abc" } });
  });

  const stepperTableNode = (value: number) => ({
    type: "componentScene" as const,
    surfaceId: "window:sourcing-pool",
    controllerId: "sourcing-curation",
    componentKind: "table" as const,
    table: {
      columnsJson: JSON.stringify([
        { id: "name", label: "Name" },
        { id: "curated", label: "Curated" },
      ]),
      rowsJson: JSON.stringify([
        {
          id: "beam-glulam-gl24h",
          name: { kind: "text", value: "Glulam GL24h" },
          curated: { kind: "stepper", value, min: 0, max: 3, step: 1, action: { controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h" } } },
        },
      ]),
    },
  });

  const renderStepperTable = (value: number, onAction: (action: unknown) => void) =>
    render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: stepperTableNode(value) as never, onAction: onAction as never })));

  it("renders a stepper cell as a spinbutton with decrement, live value and increment controls", () => {
    const { container } = renderStepperTable(1, vi.fn());
    const readout = container.querySelector<HTMLInputElement>('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]');
    expect(readout?.getAttribute("role")).toBe("spinbutton");
    expect(readout?.getAttribute("aria-valuenow")).toBe("1");
    expect(readout?.getAttribute("aria-valuemin")).toBe("0");
    expect(readout?.getAttribute("aria-valuemax")).toBe("3");
    expect(readout?.getAttribute("aria-label")).toBe("Curated");
    expect(container.querySelector('[data-stepper-control="decrement"]')).not.toBeNull();
    expect(container.querySelector('[data-stepper-control="increment"]')).not.toBeNull();
    expect(container.querySelector('[data-slot="table-stepper"]')?.getAttribute("data-stepper-for")).toBe("window:sourcing-pool.beam-glulam-gl24h.curated");
  });

  it("dispatches the cell's own descriptor with the wgpu host's {delta} patch on increment and decrement", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(1, onAction);
    fireEvent.click(container.querySelector('[data-stepper-control="increment"]')!);
    expect(onAction).toHaveBeenLastCalledWith({ controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h", delta: 1 } });
    fireEvent.click(container.querySelector('[data-stepper-control="decrement"]')!);
    expect(onAction).toHaveBeenLastCalledWith({ controllerId: "sourcing-curation", action: "curationSetCount", args: { objectId: "beam-glulam-gl24h", delta: -1 } });
  });

  it("disables the stepper control that would leave the cell's min/max bounds", () => {
    const atMin = renderStepperTable(0, vi.fn());
    expect(atMin.container.querySelector<HTMLButtonElement>('[data-stepper-control="decrement"]')?.disabled).toBe(true);
    expect(atMin.container.querySelector<HTMLButtonElement>('[data-stepper-control="increment"]')?.disabled).toBe(false);
    const atMax = renderStepperTable(3, vi.fn());
    expect(atMax.container.querySelector<HTMLButtonElement>('[data-stepper-control="decrement"]')?.disabled).toBe(false);
    expect(atMax.container.querySelector<HTMLButtonElement>('[data-stepper-control="increment"]')?.disabled).toBe(true);
  });

  it("steps a stepper cell from the keyboard: arrows by one step, Home and End onto the bounds", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(1, onAction);
    const readout = container.querySelector('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]')!;
    fireEvent.keyDown(readout, { key: "ArrowUp" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: 1 } }));
    fireEvent.keyDown(readout, { key: "ArrowDown" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -1 } }));
    fireEvent.keyDown(readout, { key: "End" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: 2 } }));
    fireEvent.keyDown(readout, { key: "Home" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -1 } }));
    onAction.mockClear();
    fireEvent.keyDown(readout, { key: "a" });
    expect(onAction).not.toHaveBeenCalled();
  });

  it("never dispatches a keyboard step that would cross a bound", () => {
    const onAction = vi.fn();
    const { container } = renderStepperTable(3, onAction);
    const readout = container.querySelector('[id="window:sourcing-pool.beam-glulam-gl24h.curated"]')!;
    fireEvent.keyDown(readout, { key: "PageUp" });
    expect(onAction).not.toHaveBeenCalled();
    fireEvent.keyDown(readout, { key: "PageDown" });
    expect(onAction).toHaveBeenLastCalledWith(expect.objectContaining({ args: { objectId: "beam-glulam-gl24h", delta: -3 } }));
  });

  it("keeps the stepper buttons on the button-group slots the touch stylesheet sizes at phone width", () => {
    const { container } = renderStepperTable(1, vi.fn());
    const increment = container.querySelector('[data-stepper-control="increment"]');
    expect(increment?.getAttribute("data-slot")).toBe("button-group-item");
    expect(increment?.closest('[data-slot="button-group"]')).not.toBeNull();
    expect(increment?.getAttribute("aria-label")).toBe("Increase Curated");
    expect(container.querySelector('[data-stepper-control="decrement"]')?.getAttribute("aria-label")).toBe("Decrease Curated");
  });

  it("tableStepperKeyDelta and tableStepperClampedDelta agree with the wgpu segment contract", () => {
    const cell = { value: 2, min: 0, max: 5, step: 0.5 };
    expect(tableStepperKeyDelta("ArrowUp", cell)).toBe(0.5);
    expect(tableStepperKeyDelta("ArrowDown", cell)).toBe(-0.5);
    expect(tableStepperKeyDelta("PageUp", cell)).toBe(5);
    expect(tableStepperKeyDelta("Home", cell)).toBe(-2);
    expect(tableStepperKeyDelta("End", cell)).toBe(3);
    expect(tableStepperKeyDelta("Enter", cell)).toBe(0);
    expect(tableStepperClampedDelta(5, cell)).toBe(3);
    expect(tableStepperClampedDelta(-9, cell)).toBe(-2);
    expect(tableStepperClampedDelta(1, { value: 5, min: 0, max: 5 })).toBe(0);
  });

  it("matches the shared Table stepper keyboard fixture through the actual React spinbutton", () => {
    const { cell, action } = tableStepperKeyboardFixture;
    for (const keyboardCase of tableStepperKeyboardFixture.cases) {
      const onAction = vi.fn();
      const node = {
        type: "componentScene" as const,
        surfaceId: "window:sourcing-pool",
        controllerId: action.controllerId,
        componentKind: "table" as const,
        table: {
          columnsJson: JSON.stringify([{ id: cell.columnId, label: cell.columnLabel }]),
          rowsJson: JSON.stringify([{ id: cell.rowId, [cell.columnId]: { kind: "stepper", value: keyboardCase.value, min: cell.min, max: cell.max, step: cell.step, action } }]),
        },
      };
      const mounted = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: node as never, onAction: onAction as never })));
      const readout = mounted.container.querySelector(`[id="${node.surfaceId}.${cell.rowId}.${cell.columnId}"]`);
      if (!readout) throw new Error(`missing ${keyboardCase.id} spinbutton`);
      const propagated = fireEvent.keyDown(readout, { key: keyboardCase.key });
      expect(!propagated, keyboardCase.id).toBe(keyboardCase.consumed);
      if (keyboardCase.expectedDelta === null) {
        expect(onAction, keyboardCase.id).not.toHaveBeenCalled();
      } else {
        expect(onAction, keyboardCase.id).toHaveBeenCalledOnce();
        expect(onAction, keyboardCase.id).toHaveBeenCalledWith({ ...action, args: { ...action.args, delta: keyboardCase.expectedDelta } });
      }
      mounted.unmount();
    }
  });
});
