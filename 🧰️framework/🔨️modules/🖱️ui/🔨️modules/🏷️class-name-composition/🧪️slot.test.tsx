// #region 🔌️Adapters
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ButtonGroupItem } from "../../🧱️elements/🎛️ButtonGroup/🟦️component.tsx";
import { Slot } from "./🟦️slot.tsx";
// #endregion 🔌️Adapters

// #region 🪆️SingleChildSlotMatrix
describe("Slot", () => {
  it("requires exactly one valid React element", () => {
    expect(() => render(React.createElement(Slot, null, "text"))).toThrow("Slot requires exactly one valid React element child.");
    expect(() => render(React.createElement(Slot, null, [<span key="first" />, <span key="second" />]))).toThrow("Slot requires exactly one valid React element child.");
  });

  it("merges class, style, refs, and child-first handlers", () => {
    const calls: string[] = [];
    const childRef = React.createRef<HTMLButtonElement>();
    const wrapperRef = React.createRef<HTMLElement>();
    const { getByRole } = render(
      <Slot ref={wrapperRef} className="wrapper-token" style={{ color: "red", padding: 2 }} onClick={() => calls.push("wrapper")}>
        <button ref={childRef} className="child-token" style={{ color: "blue" }} onClick={() => calls.push("child")}>
          Press
        </button>
      </Slot>,
    );
    const button = getByRole("button") as HTMLButtonElement;

    fireEvent.click(button);

    expect(button.className).toBe("wrapper-token child-token");
    expect(button.style.color).toBe("blue");
    expect(button.style.padding).toBe("2px");
    expect(childRef.current).toBe(button);
    expect(wrapperRef.current).toBe(button);
    expect(calls).toEqual(["child", "wrapper"]);
  });

  it("does not call the wrapper handler after the child prevents the default", () => {
    const wrapperHandler = vi.fn();
    const { getByRole } = render(
      <Slot onClick={wrapperHandler}>
        <button onClick={(event) => event.preventDefault()}>Cancel</button>
      </Slot>,
    );

    expect(fireEvent.click(getByRole("button"))).toBe(false);
    expect(wrapperHandler).not.toHaveBeenCalled();
  });
});

describe("ButtonGroupItem asChild", () => {
  it("keeps one host element and appends owned decorations", () => {
    const { getByRole } = render(
      <ButtonGroupItem asChild icon="x" className="wrapper-token">
        <a href="/target" className="child-token">
          Navigate
        </a>
      </ButtonGroupItem>,
    );
    const link = getByRole("link");

    expect(link.getAttribute("data-slot")).toBe("button-group-item");
    expect(link.className).toContain("wrapper-token");
    expect(link.className).toContain("child-token");
    expect(link.textContent).toContain("Navigate");
    expect(link.querySelector('[data-icon="x"]')).toBeTruthy();
  });
});
// #endregion 🪆️SingleChildSlotMatrix
