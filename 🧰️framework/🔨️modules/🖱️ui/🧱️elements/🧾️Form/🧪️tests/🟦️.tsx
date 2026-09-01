// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Form } from "../🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🧾️FormMatrix
describe("Form", () => {
  it("keeps native form ownership, uncancelled Enter, and submission", () => {
    const onSubmit = vi.fn((event: React.FormEvent<HTMLFormElement>) => event.preventDefault());
    const ref = React.createRef<HTMLFormElement>();
    const { getByRole } = render(
      <Form ref={ref} onSubmit={onSubmit}>
        <input name="query" aria-label="Query" />
        <button type="submit">Submit</button>
      </Form>,
    );
    const input = getByRole("textbox") as HTMLInputElement;
    const submit = getByRole("button") as HTMLButtonElement;

    expect(ref.current).toBeInstanceOf(HTMLFormElement);
    expect(input.form).toBe(ref.current);
    expect(fireEvent.keyDown(input, { key: "Enter", code: "Enter" })).toBe(true);
    ref.current!.requestSubmit(submit);
    expect(onSubmit).toHaveBeenCalledOnce();
  });
});
// #endregion 🧾️FormMatrix
