/** ⌨️ Shared lazy fields compose consumer callbacks and cancel drafts without dispatching. */
import * as React from "react";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { Input } from "../../🟦️.tsx";
import { Textarea } from "../../../🔤️Textarea/🟦️.tsx";
import law from "../../🧫️fixtures/⌨️draft-lifecycle/🔣️.json";

afterEach(cleanup);


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
