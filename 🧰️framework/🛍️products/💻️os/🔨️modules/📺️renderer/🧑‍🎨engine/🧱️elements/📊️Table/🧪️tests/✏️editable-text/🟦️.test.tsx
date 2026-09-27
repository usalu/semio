import Ajv2020 from "ajv/dist/2020";
import React from "react";
import { describe, expect, test, vi } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import fixture from "../../🧫️fixtures/✏️editable-text/🔣️.json";
import schema from "../../🧬️schema/✏️editable-text/🔣️.json";
import { reconcileTableEditableText, TableEditableTextCell, tableEditableTextAction, type TableCellRecord } from "../../🟦️.tsx";

const cell = fixture.cell as Extract<TableCellRecord, { kind: "editableText" }>;

describe("table editable text cells", () => {
  test("matches the language-neutral schema and merges multiline Unicode into the owned action", () => {
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture.cell), JSON.stringify(validate.errors)).toBe(true);
    expect(tableEditableTextAction(cell, fixture.replacement)).toEqual(fixture.expectedAction);
    const dirty = { base: cell.value, draft: fixture.replacement, dirty: true, conflicted: false } as const;
    expect(reconcileTableEditableText(dirty, cell.value)).toEqual(dirty);
    expect(reconcileTableEditableText(dirty, `${cell.value}!`)).toEqual({ ...dirty, conflicted: true });
    const acceptance = fixture.acceptance;
    const submitted = { base: acceptance.base, draft: acceptance.draft, dirty: true, conflicted: false } as const;
    expect(reconcileTableEditableText(submitted, acceptance.acceptedPersisted)).toEqual({ base: acceptance.acceptedPersisted, draft: acceptance.acceptedPersisted, dirty: false, conflicted: false });
    expect(reconcileTableEditableText(submitted, acceptance.refusedPersisted)).toEqual(submitted);
    expect(reconcileTableEditableText(submitted, acceptance.collaboratorPersisted)).toEqual({ ...submitted, conflicted: true });
  });

  test("commits on Enter and blur, inserts multiline drafts, and cancels with Escape", () => {
    const onAction = vi.fn();
    const view = render(<TableEditableTextCell cell={cell} id="cell" columnLabel="Value" onAction={onAction} />);
    const input = view.getByRole("textbox", { name: "Value" });
    fireEvent.change(input, { target: { value: fixture.replacement } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onAction).toHaveBeenLastCalledWith(fixture.expectedAction);

    fireEvent.change(input, { target: { value: fixture.keyboard.afterShiftEnter } });
    fireEvent.blur(input);
    expect(onAction).toHaveBeenLastCalledWith(tableEditableTextAction(cell, fixture.keyboard.afterShiftEnter));

    fireEvent.change(input, { target: { value: "cancelled Ω" } });
    fireEvent.keyDown(input, { key: "Escape" });
    expect((input as HTMLTextAreaElement).value).toBe(cell.value);
    expect(onAction).toHaveBeenCalledTimes(2);
    cleanup();
  });
});
