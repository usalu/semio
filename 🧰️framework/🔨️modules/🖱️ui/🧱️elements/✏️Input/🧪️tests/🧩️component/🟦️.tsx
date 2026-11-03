/** ⌨️ Shared lazy fields compose consumer callbacks and cancel drafts without dispatching. */
import * as React from "react";
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Input } from "../../🟦️.tsx";
import { Textarea } from "../../../🔤️Textarea/🟦️.tsx";
import law from "../🧫️fixtures/⌨️draft-lifecycle/🔣️.json";
import defaults from "../🧫️fixtures/⌨️default-value/🔣️.json";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

for (const row of defaults.cases) it(`matches the native React input lifecycle for ${row.name}`, async () => {
  const warning = vi.spyOn(console, "error").mockImplementation(() => {});
  const snapshots: string[][] = [];
  for (const Field of ["input", Input] as const) {
    const change = vi.fn();
    const draw = (defaultValue = row.defaultValue) => <form><Field id="default-field" type={row.type} defaultValue={defaultValue} onChange={change} /></form>;
    const view = render(draw());
    const field = view.container.querySelector("input")!;
    const values = [field.value];
    act(() => field.focus());
    fireEvent.change(field, { target: { value: row.draft } });
    expect(change).toHaveBeenCalledTimes(1);
    values.push(field.value);
    view.rerender(draw(row.nextDefault));
    values.push(field.value);
    act(() => field.blur());
    await act(async () => view.container.querySelector("form")!.reset());
    values.push(field.value);
    expect(values).toEqual([row.initial, row.draft, row.draft, row.reset]);
    snapshots.push(values);
    view.unmount();
  }
  expect(snapshots[1]).toEqual(snapshots[0]);
  expect(warning).not.toHaveBeenCalled();
});

for (const row of defaults.lazy) it(`retains uncontrolled lazy commits and cancels on ${row.key}`, () => {
  const commit = vi.fn();
  const view = render(<Input id="lazy-default" lazy defaultValue="Original" onLazyChange={commit} />);
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Accepted" } });
  if (row.key === "blur") act(() => field.blur());
  else fireEvent.keyDown(field, { key: row.key });
  expect(field.value).toBe(row.accepted);
  expect(commit.mock.calls.map(([value]) => value)).toEqual(row.commits);
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Discarded" } });
  fireEvent.keyDown(field, { key: "Escape" });
  expect(field.value).toBe(row.accepted);
  expect(commit.mock.calls.map(([value]) => value)).toEqual(row.commits);
});

it("keeps controlled values authoritative even when a default is supplied", () => {
  const warning = vi.spyOn(console, "error").mockImplementation(() => {});
  const change = vi.fn();
  const view = render(<Input id="controlled" value="Controlled" defaultValue="Ignored" onChange={change} />);
  const field = view.container.querySelector("input")!;
  fireEvent.change(field, { target: { value: "Rejected" } });
  expect(change).toHaveBeenCalledTimes(1);
  expect(field.value).toBe("Controlled");
  view.rerender(<Input id="controlled" value="Updated" defaultValue="Ignored" onChange={change} />);
  expect(field.value).toBe("Updated");
  expect(warning).not.toHaveBeenCalled();
});

it("keeps a controlled lazy draft until blur and cancels to the latest external value", () => {
  const commit = vi.fn();
  const draw = (value: string) => <Input id="controlled-lazy" lazy value={value} onLazyChange={commit} />;
  const view = render(draw("Original"));
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Draft" } });
  view.rerender(draw("Updated"));
  expect(field.value).toBe("Draft");
  fireEvent.keyDown(field, { key: "Escape" });
  expect(field.value).toBe("Updated");
  expect(commit).not.toHaveBeenCalled();
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Submitted" } });
  act(() => field.blur());
  expect(commit).toHaveBeenCalledExactlyOnceWith("Submitted");
  expect(field.value).toBe("Updated");
});

it("honors canceled resets and restores an uncontrolled lazy default without committing", async () => {
  const commit = vi.fn();
  let cancel = true;
  const view = render(<form onReset={(event) => { if (cancel) event.preventDefault(); }}><Input id="reset-lazy" lazy defaultValue="Original" onLazyChange={commit} /></form>);
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Draft" } });
  await act(async () => view.container.querySelector("form")!.reset());
  expect(field.value).toBe("Draft");
  cancel = false;
  await act(async () => view.container.querySelector("form")!.reset());
  expect(field.value).toBe("Original");
  expect(commit).not.toHaveBeenCalled();
});

it("preserves defaults and edits across collapsed labeled input mounts and resets", async () => {
  const view = render(<form><Input id="collapsed-default" showLabel defaultValue={240} type="number" /></form>);
  const collapsed = () => view.container.querySelector<HTMLElement>('[data-collapsed="true"]')!;
  expect(collapsed().textContent).toBe("240");
  fireEvent.click(collapsed());
  const field = view.container.querySelector("input")!;
  expect(field.value).toBe("240");
  fireEvent.change(field, { target: { value: "320" } });
  act(() => field.blur());
  expect(collapsed().textContent).toBe("320");
  await act(async () => view.container.querySelector("form")!.reset());
  expect(collapsed().textContent).toBe("240");
  fireEvent.click(collapsed());
  expect(view.container.querySelector("input")!.value).toBe("240");
});

it("resets focused numbers when the new default already equals component state", async () => {
  const draw = (defaultValue: number) => <form><Input id="focused-reset" type="number" defaultValue={defaultValue} /></form>;
  const view = render(draw(240));
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "320" } });
  view.rerender(draw(320));
  await act(async () => view.container.querySelector("form")!.reset());
  expect(field.value).toBe("320");
});

it("supports associated forms and resets even after a lazy commit", async () => {
  const commit = vi.fn();
  const view = render(<><form id="associated" /><Input id="associated-field" form="associated" lazy defaultValue="Original" onLazyChange={commit} /></>);
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Accepted" } });
  fireEvent.keyDown(field, { key: "Enter" });
  expect(field.value).toBe("Accepted");
  await act(async () => view.container.querySelector("form")!.reset());
  expect(field.value).toBe("Original");
  expect(commit).toHaveBeenCalledExactlyOnceWith("Accepted");
});

it("does not commit a composing Enter or Escape", () => {
  const commit = vi.fn();
  const view = render(<Input id="composing" lazy defaultValue="Original" onLazyChange={commit} />);
  const field = view.container.querySelector("input")!;
  act(() => field.focus());
  fireEvent.change(field, { target: { value: "Draft" } });
  for (const key of ["Enter", "Escape"]) fireEvent.keyDown(field, { key, isComposing: true });
  expect(field.value).toBe("Draft");
  expect(document.activeElement).toBe(field);
  expect(commit).not.toHaveBeenCalled();
});

for (const row of law.cases) it(`composes ${row.kind} callbacks and commits once on ${row.key}`, () => {
  const commit = vi.fn();
  const focus = vi.fn();
  const blur = vi.fn();
  const key = vi.fn();
  const Field = row.kind === "longText" ? Textarea : Input;
  const view = render(<Field id="lazy-field" lazy value={row.base} onLazyChange={commit} onFocus={focus} onBlur={blur} onKeyDown={key} />);
  const field = view.container.querySelector<HTMLInputElement | HTMLTextAreaElement>("input,textarea")!;
  field.focus();
  fireEvent.change(field, { target: { value: row.draft } });
  fireEvent.keyDown(field, { key: row.key });
  expect(commit.mock.calls.map(([value]) => value)).toEqual(row.commits);
  expect(focus).toHaveBeenCalledTimes(1);
  expect(blur).toHaveBeenCalledTimes(1);
  expect(key).toHaveBeenCalledTimes(1);
  if (row.key === "Escape") expect(field.value).toBe(row.base);
});
