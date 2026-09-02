// #region 🔌️Adapters
import { fireEvent, render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "../🟦️.tsx";
// #endregion 🔌️Adapters

// #region ↕️CollapsibleMatrix
describe("Collapsible", () => {
  it("updates uncontrolled state while keeping content mounted and associated", () => {
    const onOpenChange = vi.fn();
    const { getByRole, getByTestId } = render(
      <Collapsible defaultOpen onOpenChange={onOpenChange}>
        <CollapsibleTrigger>Branch</CollapsibleTrigger>
        <CollapsibleContent data-testid="content">Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button") as HTMLButtonElement;
    const content = getByTestId("content") as HTMLDivElement;

    expect(trigger.type).toBe("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(content.id);
    expect(trigger.getAttribute("data-state")).toBe("open");
    expect(content.hidden).toBe(false);
    expect(content.getAttribute("data-state")).toBe("open");

    fireEvent.click(trigger);

    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("data-state")).toBe("closed");
    expect(content.hidden).toBe(true);
    expect(content.textContent).toBe("Leaf");
  });

  it("repeats controlled proposals during parent lag without mutating rendered state", () => {
    const onOpenChange = vi.fn();
    const { getByRole, getByTestId, rerender } = render(
      <Collapsible open={false} onOpenChange={onOpenChange}>
        <CollapsibleTrigger>Branch</CollapsibleTrigger>
        <CollapsibleContent data-testid="content">Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");
    const content = getByTestId("content") as HTMLDivElement;

    fireEvent.click(trigger);
    fireEvent.click(trigger);
    fireEvent.keyDown(trigger, { key: "Enter" });
    fireEvent.click(trigger);
    expect(onOpenChange).toHaveBeenNthCalledWith(1, true);
    expect(onOpenChange).toHaveBeenNthCalledWith(2, true);
    expect(onOpenChange).toHaveBeenNthCalledWith(3, true);
    expect(onOpenChange).toHaveBeenCalledTimes(3);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(content.hidden).toBe(true);

    rerender(
      <Collapsible open onOpenChange={onOpenChange}>
        <CollapsibleTrigger>Branch</CollapsibleTrigger>
        <CollapsibleContent data-testid="content">Leaf</CollapsibleContent>
      </Collapsible>,
    );
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(content.hidden).toBe(false);

    fireEvent.click(trigger);
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(onOpenChange).toHaveBeenCalledTimes(4);
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
  });

  it("protects owned state and association attributes from host overrides", () => {
    const forcedVisibility = { hidden: true } as React.HTMLAttributes<HTMLDivElement>;
    const { getByRole, getByTestId } = render(
      <Collapsible data-slot="host-root" data-state="host-state" defaultOpen>
        <CollapsibleTrigger aria-controls="host-content" aria-expanded>
          Branch
        </CollapsibleTrigger>
        <CollapsibleContent {...forcedVisibility} data-testid="content" data-slot="host-content" data-state="host-state" id="host-content">
          Leaf
        </CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");
    const content = getByTestId("content") as HTMLDivElement;

    expect(trigger.closest('[data-slot="collapsible"]')?.getAttribute("data-state")).toBe("open");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(content.id);
    expect(content.id).not.toBe("host-content");
    expect(content.getAttribute("data-slot")).toBe("collapsible-content");
    expect(content.getAttribute("data-state")).toBe("open");
    expect(content.hidden).toBe(false);
  });

  it("activates a non-native asChild trigger once for Enter and once for Space", () => {
    const onOpenChange = vi.fn();
    const { getByRole } = render(
      <Collapsible onOpenChange={onOpenChange}>
        <CollapsibleTrigger asChild>
          <div>Branch</div>
        </CollapsibleTrigger>
        <CollapsibleContent>Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");

    fireEvent.keyDown(trigger, { key: "Enter" });
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenLastCalledWith(true);

    fireEvent.keyDown(trigger, { key: " " });
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    fireEvent.keyUp(trigger, { key: " " });
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
    expect(onOpenChange).toHaveBeenCalledTimes(2);
  });

  it("lets native button keyboard activation toggle only through its synthesized click", () => {
    const onOpenChange = vi.fn();
    const { getByRole } = render(
      <Collapsible onOpenChange={onOpenChange}>
        <CollapsibleTrigger>Branch</CollapsibleTrigger>
        <CollapsibleContent>Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");

    fireEvent.keyDown(trigger, { key: "Enter" });
    fireEvent.click(trigger);
    fireEvent.keyUp(trigger, { key: "Enter" });
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(onOpenChange).toHaveBeenCalledTimes(1);

    fireEvent.keyDown(trigger, { key: " " });
    fireEvent.keyUp(trigger, { key: " " });
    fireEvent.click(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(onOpenChange).toHaveBeenCalledTimes(2);
  });

  it("composes an asChild trigger into one host with child-first events and refs", () => {
    const calls: string[] = [];
    const childRef = React.createRef<HTMLDivElement>();
    const triggerRef = React.createRef<HTMLElement>();
    const { getByRole } = render(
      <Collapsible>
        <CollapsibleTrigger asChild ref={triggerRef} onClick={() => calls.push("trigger")}>
          <div ref={childRef} onClick={() => calls.push("child")}>
            Branch
          </div>
        </CollapsibleTrigger>
        <CollapsibleContent>Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");

    fireEvent.click(trigger);

    expect(calls).toEqual(["child", "trigger"]);
    expect(triggerRef.current).toBe(trigger);
    expect(childRef.current).toBe(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
  });

  it("honors asChild event cancellation", () => {
    const onClick = vi.fn();
    const { getByRole } = render(
      <Collapsible>
        <CollapsibleTrigger asChild onClick={onClick}>
          <div onClick={(event) => event.preventDefault()}>Branch</div>
        </CollapsibleTrigger>
        <CollapsibleContent>Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button");

    expect(fireEvent.click(trigger)).toBe(false);
    expect(onClick).not.toHaveBeenCalled();
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
  });

  it("suppresses pointer and keyboard activation while disabled", () => {
    const onOpenChange = vi.fn();
    const { getByRole, getByTestId } = render(
      <Collapsible disabled onOpenChange={onOpenChange}>
        <CollapsibleTrigger>Branch</CollapsibleTrigger>
        <CollapsibleContent data-testid="content">Leaf</CollapsibleContent>
      </Collapsible>,
    );
    const trigger = getByRole("button") as HTMLButtonElement;

    expect(trigger.disabled).toBe(true);
    expect(trigger.getAttribute("aria-disabled")).toBe("true");
    fireEvent.click(trigger);
    fireEvent.keyDown(trigger, { key: "Enter" });
    fireEvent.keyDown(trigger, { key: " " });
    fireEvent.keyUp(trigger, { key: " " });
    expect(onOpenChange).not.toHaveBeenCalled();
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect((getByTestId("content") as HTMLDivElement).hidden).toBe(true);
  });

  it("owns trigger-level and asChild disabled pointer, keyboard, and tab state", () => {
    const nativeOnClick = vi.fn();
    const nativeOnOpenChange = vi.fn();
    const childOnClick = vi.fn();
    const slottedOnClick = vi.fn();
    const slottedOnKeyDown = vi.fn();
    const slottedOnOpenChange = vi.fn();
    const { getByRole } = render(
      <>
        <Collapsible onOpenChange={nativeOnOpenChange}>
          <CollapsibleTrigger disabled onClick={nativeOnClick}>
            Native
          </CollapsibleTrigger>
          <CollapsibleContent>Native leaf</CollapsibleContent>
        </Collapsible>
        <Collapsible onOpenChange={slottedOnOpenChange}>
          <CollapsibleTrigger asChild disabled onClick={slottedOnClick} onKeyDown={slottedOnKeyDown}>
            <a href="#leaf" onClick={childOnClick}>
              Slotted
            </a>
          </CollapsibleTrigger>
          <CollapsibleContent>Slotted leaf</CollapsibleContent>
        </Collapsible>
      </>,
    );
    const native = getByRole("button", { name: "Native" }) as HTMLButtonElement;
    const slotted = getByRole("button", { name: "Slotted" });

    expect(native.disabled).toBe(true);
    expect(native.getAttribute("aria-disabled")).toBe("true");
    expect(slotted.getAttribute("aria-disabled")).toBe("true");
    expect(slotted.getAttribute("data-disabled")).toBe("");
    expect(slotted.getAttribute("tabindex")).toBe("-1");
    expect(fireEvent.click(slotted)).toBe(false);
    expect(fireEvent.keyDown(slotted, { key: "Enter" })).toBe(false);
    expect(fireEvent.keyDown(slotted, { key: " " })).toBe(false);
    expect(fireEvent.keyUp(slotted, { key: " " })).toBe(false);
    fireEvent.click(native);

    expect(childOnClick).toHaveBeenCalledTimes(1);
    expect(nativeOnClick).not.toHaveBeenCalled();
    expect(slottedOnClick).not.toHaveBeenCalled();
    expect(slottedOnKeyDown).not.toHaveBeenCalled();
    expect(nativeOnOpenChange).not.toHaveBeenCalled();
    expect(slottedOnOpenChange).not.toHaveBeenCalled();
    expect(slotted.getAttribute("aria-expanded")).toBe("false");
  });

  it("generates stable unique associations and forwards root and content refs", () => {
    const firstRootRef = React.createRef<HTMLDivElement>();
    const firstContentRef = React.createRef<HTMLDivElement>();
    const { getAllByRole, getAllByTestId, rerender } = render(
      <>
        <Collapsible ref={firstRootRef}>
          <CollapsibleTrigger>First</CollapsibleTrigger>
          <CollapsibleContent ref={firstContentRef} data-testid="content">
            First leaf
          </CollapsibleContent>
        </Collapsible>
        <Collapsible>
          <CollapsibleTrigger>Second</CollapsibleTrigger>
          <CollapsibleContent data-testid="content">Second leaf</CollapsibleContent>
        </Collapsible>
      </>,
    );
    const triggers = getAllByRole("button");
    const contents = getAllByTestId("content");
    const ids = triggers.map((trigger) => trigger.getAttribute("aria-controls"));

    expect(ids[0]).toBe(contents[0]?.id);
    expect(ids[1]).toBe(contents[1]?.id);
    expect(ids[0]).not.toBe(ids[1]);
    expect(firstRootRef.current?.getAttribute("data-slot")).toBe("collapsible");
    expect(firstContentRef.current).toBe(contents[0]);

    rerender(
      <>
        <Collapsible ref={firstRootRef}>
          <CollapsibleTrigger>First updated</CollapsibleTrigger>
          <CollapsibleContent ref={firstContentRef} data-testid="content">
            First leaf
          </CollapsibleContent>
        </Collapsible>
        <Collapsible>
          <CollapsibleTrigger>Second updated</CollapsibleTrigger>
          <CollapsibleContent data-testid="content">Second leaf</CollapsibleContent>
        </Collapsible>
      </>,
    );
    expect(getAllByRole("button").map((trigger) => trigger.getAttribute("aria-controls"))).toEqual(ids);
  });
});
// #endregion ↕️CollapsibleMatrix
