// #region 🔌️Adapters
import { act } from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { ToggleGroup } from "../../🟦️.tsx";
import cases from "../🔣️.json";
import { ButtonGroup, ButtonGroupItem } from "../../../🔳️ButtonGroup/🟦️.tsx";
import { Toggle } from "../../../🔀️Toggle/🟦️.tsx";
import { Action, ActionGroup, ActionGroupItem } from "../../../⚡️ActionGroup/🟦️.tsx";
import { Navbar } from "../../../🔝️Navbar/🟦️.tsx";
import { Footer } from "../../../🔚️Footer/🟦️.tsx";
import { Ribbon } from "../../../🎀️Ribbon/🟦️.tsx";
import { DragHandle } from "../../../🧱️DragHandle/🟦️.tsx";
import { Button } from "../../../🔘️Button/🟦️.tsx";
import { PanelTabBar } from "../../../🧭️PanelTabBar/🟦️.tsx";
import { chromeControlGroupShellClass, chromeControlGroupSiblingDividerClass, chromeControlItemBaseClass } from "../../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎛️Fixture
const items = [
  { value: "alpha", id: "toggle-alpha", icon: "x" as const, text: "Alpha" },
  { value: "disabled", id: "toggle-disabled", icon: "x" as const, text: "Disabled", disabled: true },
  { value: "beta", id: "toggle-beta", icon: "x" as const, text: "Beta" },
  { value: "gamma", id: "toggle-gamma", icon: "x" as const, text: "Gamma" },
];
// #endregion 🎛️Fixture

// #region 🎛️ToggleGroupMatrix
describe("ToggleGroup", () => {
  it("owns uncontrolled single selection and exact pressed state", () => {
    const changes = vi.fn();
    const { getByRole } = render(<ToggleGroup defaultValue="alpha" onValueChange={changes} items={items} />);
    const alpha = getByRole("button", { name: "Alpha" });
    const beta = getByRole("button", { name: "Beta" });
    expect(alpha.getAttribute("aria-pressed")).toBe("true");
    expect(alpha.getAttribute("data-state")).toBe("on");
    fireEvent.click(beta);
    expect(changes).toHaveBeenCalledWith("beta");
    expect(alpha.getAttribute("aria-pressed")).toBe("false");
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith("");
    expect(beta.getAttribute("aria-pressed")).toBe("false");
  });

  it("is a radio group under radio semantics: one checked option, arrows move the choice, and the chosen one never presses off", () => {
    const changes = vi.fn();
    const { getByRole, getAllByRole, queryAllByRole } = render(<ToggleGroup semantics="radio" defaultValue="alpha" onValueChange={changes} items={items} aria-label="Axis" />);
    const group = getByRole("radiogroup", { name: "Axis" });
    const radios = getAllByRole("radio");
    expect([group.getAttribute("data-slot"), radios.map((radio) => [radio.getAttribute("aria-checked"), radio.hasAttribute("aria-pressed")]), queryAllByRole("button")]).toEqual(["toggle-group", [["true", false], ["false", false], ["false", false], ["false", false]], []]);
    const [alpha, , beta, gamma] = radios as [HTMLElement, HTMLElement, HTMLElement, HTMLElement];
    fireEvent.click(alpha);
    expect(changes, "the chosen option cannot be pressed off").not.toHaveBeenCalled();
    act(() => alpha.focus());
    fireEvent.keyDown(alpha, { key: "ArrowRight" });
    expect([document.activeElement, changes.mock.calls, beta.getAttribute("aria-checked"), alpha.getAttribute("aria-checked")], "an arrow moves the choice with the focus, past a disabled option").toEqual([beta, [["beta"]], "true", "false"]);
    fireEvent.keyDown(beta, { key: "End" });
    expect([document.activeElement, changes.mock.calls.at(-1), gamma.getAttribute("aria-checked")]).toEqual([gamma, ["gamma"], "true"]);
    fireEvent.click(beta);
    expect([changes.mock.calls.at(-1), radios.map((radio) => radio.tabIndex)], "a press chooses, and only the chosen option is a Tab stop").toEqual([["beta"], [-1, -1, 0, -1]]);
  });

  it("emits controlled single lag proposals without duplicate callbacks or local mutation", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(<ToggleGroup value="alpha" onValueChange={changes} items={items} />);
    const beta = getByRole("button", { name: "Beta" });
    fireEvent.click(beta);
    fireEvent.click(beta);
    fireEvent.click(beta);
    expect(changes.mock.calls).toEqual([["beta"], ["beta"], ["beta"]]);
    expect(beta.getAttribute("aria-pressed")).toBe("false");
    rerender(<ToggleGroup value="beta" onValueChange={changes} items={items} />);
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith("");
  });

  it("owns uncontrolled and controlled multiple arrays", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(<ToggleGroup kind="multiple" defaultValue={["alpha"]} onValueChange={changes} items={items} />);
    const beta = getByRole("button", { name: "Beta" });
    fireEvent.click(beta);
    expect(changes).toHaveBeenLastCalledWith(["alpha", "beta"]);
    expect(beta.getAttribute("aria-pressed")).toBe("true");
    rerender(<ToggleGroup kind="multiple" value={[]} onValueChange={changes} items={items} />);
    fireEvent.click(beta);
    fireEvent.click(beta);
    expect(changes.mock.calls.slice(-2)).toEqual([[["beta"]], [["beta"]]]);
    expect(beta.getAttribute("aria-pressed")).toBe("false");
  });

  it("suppresses group/item disabled activation and owns generated distinct IDs across groups", () => {
    const changes = vi.fn();
    const { container, rerender } = render(
      <div>
        <ToggleGroup
          id="group-one"
          onValueChange={changes}
          items={[
            { value: "alpha", icon: "x", text: "Alpha" },
            { value: "beta", icon: "x", text: "Beta" },
          ]}
        />
        <ToggleGroup
          id="group-two"
          items={[
            { value: "alpha", icon: "x", text: "Other Alpha" },
            { value: "beta", icon: "x", text: "Other Beta" },
          ]}
        />
      </div>,
    );
    const alpha = container.querySelector<HTMLButtonElement>("#group-one-alpha")!;
    const beta = container.querySelector<HTMLButtonElement>("#group-one-beta")!;
    expect(alpha.id).toBe("group-one-alpha");
    expect(beta.id).toBe("group-one-beta");
    expect(container.querySelector<HTMLButtonElement>("#group-two-alpha")?.id).toBe("group-two-alpha");
    expect(new Set(Array.from(container.querySelectorAll<HTMLButtonElement>('[data-slot="toggle-group-item"]')).map((item) => item.id)).size).toBe(4);
    rerender(
      <ToggleGroup
        id="group-one"
        disabled
        onValueChange={changes}
        items={[
          { value: "alpha", icon: "x", text: "Alpha" },
          { value: "beta", icon: "x", text: "Beta" },
        ]}
      />,
    );
    const disabledAlpha = container.querySelector<HTMLButtonElement>("#group-one-alpha")!;
    fireEvent.click(disabledAlpha);
    expect(disabledAlpha.disabled).toBe(true);
    expect(disabledAlpha.tabIndex).toBe(-1);
    expect(changes).not.toHaveBeenCalled();
  });

  it("roves horizontally with RTL, skips disabled items, and honors loop false", () => {
    const { getByRole, rerender } = render(<ToggleGroup value="alpha" dir="rtl" items={items} />);
    const alpha = getByRole("button", { name: "Alpha" });
    const beta = getByRole("button", { name: "Beta" });
    const gamma = getByRole("button", { name: "Gamma" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(beta);
    fireEvent.keyDown(beta, { key: "End" });
    expect(document.activeElement).toBe(gamma);
    rerender(<ToggleGroup value="alpha" dir="ltr" loop={false} items={items} />);
    const ltrAlpha = getByRole("button", { name: "Alpha" });
    ltrAlpha.focus();
    fireEvent.keyDown(ltrAlpha, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(ltrAlpha);
  });

  it("roves vertically and keeps the action as a keyboard-focusable sibling", () => {
    const changes = vi.fn();
    const action = vi.fn();
    const { getByRole } = render(
      <ToggleGroup
        orientation="vertical"
        onValueChange={changes}
        items={[
          {
            value: "alpha",
            icon: "x",
            text: "Alpha",
            action: (
              <button type="button" onClick={action}>
                Action
              </button>
            ),
          },
          { value: "beta", icon: "x", text: "Beta" },
        ]}
      />,
    );
    const alpha = document.querySelector<HTMLButtonElement>('[data-toggle-value="alpha"]')!;
    const beta = getByRole("button", { name: "Beta" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowDown" });
    expect(document.activeElement).toBe(beta);
    const actionButton = getByRole("button", { name: "Action" });
    expect(alpha.contains(actionButton)).toBe(false);
    expect(document.querySelector("button button, button a, button input")).toBeNull();
    actionButton.focus();
    expect(document.activeElement).toBe(actionButton);
    fireEvent.keyDown(actionButton, { key: "Enter" });
    fireEvent.keyUp(actionButton, { key: "Enter" });
    act(() => actionButton.click());
    expect(action).toHaveBeenCalledTimes(1);
    expect(changes).not.toHaveBeenCalled();
  });

  it("keeps chrome group cells stretchable without clipping overflow", () => {
    expect(chromeControlItemBaseClass).toContain("h-full");
    expect(chromeControlItemBaseClass).toContain("overflow-visible");
    expect(chromeControlGroupShellClass).toContain("items-stretch");
  });

  it("paints a logical-end divider between sibling toggle items", () => {
    const markup = renderToStaticMarkup(
      <ToggleGroup
        items={[
          { value: "a", icon: "check" },
          { value: "b", icon: "x" },
        ]}
      />,
    );
    expect(markup).toContain(chromeControlGroupSiblingDividerClass);
  });

  it("renders toggle group items without aspect-square", () => {
    const labeledMarkup = renderToStaticMarkup(
      <ToggleGroup
        items={[
          { value: "a", icon: "check", text: "A" },
          { value: "b", icon: "x", text: "B" },
        ]}
      />,
    );
    expect(labeledMarkup).not.toContain("aspect-square");

    const iconOnlyMarkup = renderToStaticMarkup(
      <ToggleGroup
        items={[
          { value: "a", icon: "check" },
          { value: "b", icon: "x" },
        ]}
      />,
    );
    expect(iconOnlyMarkup).not.toContain("aspect-square");
    expect(iconOnlyMarkup).toContain("min-w-medium");
  });
});
// #endregion 🎛️ToggleGroupMatrix

/** 📏️ Declarative ownership stays identical in server markup and the interactive target. */
describe("shared control border ownership", () => {
  const fixtures = {
    buttons: <ButtonGroup><ButtonGroupItem icon="x" text="First" variant="outline" /><ButtonGroupItem icon="x" text="Second" /></ButtonGroup>,
    toggle: <Toggle icon="x" text="Toggle" />,
    "toggle-actions": <ToggleGroup items={[{ value: "a", icon: "x", text: "First", action: <Action icon="x" text="Secondary" /> }, { value: "b", icon: "x", text: "Second" }]} />,
    actions: <ActionGroup><ActionGroupItem icon="x" text="First" /><ActionGroupItem icon="x" text="Second" /></ActionGroup>,
    navbar: <Navbar items={[]} showFullscreenToggle={false} label="Navigation" />,
    footer: <Footer items={[]} />,
  };
  for (const fixture of cases) {
    it(`declares only the painted edges of ${fixture.name}`, () => {
      const element = fixtures[fixture.name as keyof typeof fixtures];
      const server = document.createElement("div");
      server.innerHTML = renderToStaticMarkup(element);
      const { container } = render(element);
      for (const root of [server, container]) {
        const frame = root.querySelector(`[data-slot="${fixture.frame}"]`)!;
        expect(frame.hasAttribute("data-shared-border-frame")).toBe(true);
        expect(frame.classList.contains("relative")).toBe(true);
        expect(frame.getAttribute("data-shared-border-edges")).toBe(fixture.edges);
        expect(frame.querySelectorAll(":scope > [data-shared-border-cell]").length).toBe(fixture.cells);
        expect(root.querySelector("[data-shared-border-attached]")).toBeNull();
      }
    });
  }

  it("keeps an outlined button's only outer frame on the group", () => {
    const { container } = render(fixtures.buttons);
    expect(container.querySelectorAll("[data-shared-border-frame]").length).toBe(1);
    for (const button of container.querySelectorAll("button")) expect(button.classList.contains("border")).toBe(false);
  });

  it("forwards a standalone button's focus and disabled semantics through one registered frame", () => {
    const onClick = vi.fn();
    const { container, getByRole } = render(<Button icon="x" text="Standalone" disabled onClick={onClick} />);
    const button = getByRole("button", { name: "Standalone" });
    expect(container.querySelectorAll("[data-shared-border-frame]").length).toBe(1);
    expect(button.hasAttribute("data-shared-border-cell")).toBe(true);
    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("registers panel tab rows without replacing chip geometry or pressed semantics", () => {
    const Icon = () => <svg />;
    const markup = renderToStaticMarkup(<PanelTabBar variant="chrome" activePath={["one"]} onActivePathChange={() => {}} tabs={[{ kind: "leaf", id: "one", name: "One", icon: Icon, trees: [] }, { kind: "leaf", id: "two", name: "Two", icon: Icon, trees: [] }]} />);
    const root = document.createElement("div");
    root.innerHTML = markup;
    const row = root.querySelector('[data-slot="panel-tabs"]')!;
    expect(row.hasAttribute("data-shared-border-group")).toBe(true);
    expect(row.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(row.querySelectorAll("[data-shared-border-cell]").length).toBe(2);
    expect(root.querySelectorAll('[aria-pressed="true"]').length).toBe(1);
    expect(root.querySelector("[data-shared-border-attached]")).toBeNull();
  });

  it("keeps action borders and native semantics independent of toggle selection", () => {
    const { container, getByRole } = render(fixtures["toggle-actions"]);
    expect(container.querySelectorAll('[data-slot="toggle-group"] > [data-shared-border-cell]').length).toBe(2);
    const secondary = getByRole("button", { name: "Secondary" });
    expect(secondary.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(getByRole("button", { name: "First" }).contains(secondary)).toBe(false);
  });

  it("declares vertical shell cells without turning their actions into vertical siblings", () => {
    const { container } = render(<ToggleGroup orientation="vertical" items={[{ value: "a", icon: "x", action: <Action icon="x" text="Action" /> }, { value: "b", icon: "x" }]} />);
    const group = container.querySelector('[data-slot="toggle-group"]')!;
    expect(group.getAttribute("data-orientation")).toBe("vertical");
    expect(group.classList.contains("flex-col")).toBe(true);
    expect(group.querySelectorAll(":scope > [data-shared-border-cell]").length).toBe(2);
  });

  it("registers gapped ribbon rows as attachment groups, never painted frames", () => {
    const { container } = render(<Ribbon direction="down" rows={[{ key: "a", content: <DragHandle /> }]} />);
    const ribbon = container.querySelector('[data-slot="ribbon"]')!;
    expect(ribbon.hasAttribute("data-shared-border-group")).toBe(true);
    expect(ribbon.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(container.querySelector('[data-slot="drag-handle"]')?.hasAttribute("data-shared-border-cell")).toBe(false);
  });
});
