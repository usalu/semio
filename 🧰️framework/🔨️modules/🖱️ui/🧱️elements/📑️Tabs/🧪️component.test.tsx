// #region 🔌️Adapters
import * as React from "react";
import { renderToString } from "react-dom/server";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 📑️Fixture
interface TabsFixtureProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  orientation?: "horizontal" | "vertical";
  dir?: "ltr" | "rtl";
  activationMode?: "automatic" | "manual";
}

/** 🧪️ Renders the complete owned parts contract. */
function TabsFixture(props: TabsFixtureProps) {
  return (
    <Tabs {...props}>
      <TabsList>
        <TabsTrigger value="alpha">Alpha</TabsTrigger>
        <TabsTrigger value="disabled" disabled>
          Disabled
        </TabsTrigger>
        <TabsTrigger value="beta" id="explicit-beta">
          Beta
        </TabsTrigger>
        <TabsTrigger value="gamma">Gamma</TabsTrigger>
      </TabsList>
      <TabsContent value="alpha">Alpha panel</TabsContent>
      <TabsContent value="disabled">Disabled panel</TabsContent>
      <TabsContent value="beta">Beta panel</TabsContent>
      <TabsContent value="gamma">Gamma panel</TabsContent>
    </Tabs>
  );
}

const adversarialValues = ["/", "-2F", "%2F", "\0", "😀", "e\u0301", "é"];

/** 🧬️ Renders collision-prone values with the same logical values in independent roots. */
function AdversarialTabs({ group }: { readonly group: string }) {
  return (
    <Tabs defaultValue={adversarialValues[0]}>
      <TabsList>
        {adversarialValues.map((value, index) => (
          <TabsTrigger key={value} value={value}>{`${group} ${index}`}</TabsTrigger>
        ))}
      </TabsList>
      {adversarialValues.map((value, index) => (
        <TabsContent key={value} value={value}>{`${group} panel ${index}`}</TabsContent>
      ))}
    </Tabs>
  );
}
// #endregion 📑️Fixture

// #region 📑️TabsMatrix
describe("Tabs", () => {
  it("owns uncontrolled selection, associations, visibility, state, and distinct group IDs", () => {
    const changes = vi.fn();
    const { getAllByRole, getByRole } = render(
      <div>
        <TabsFixture defaultValue="alpha" onValueChange={changes} />
        <TabsFixture defaultValue="alpha" />
      </div>,
    );
    const tabs = getAllByRole("tab");
    const firstAlpha = tabs[0] as HTMLButtonElement;
    const firstBeta = tabs[2] as HTMLButtonElement;
    const secondAlpha = tabs[4] as HTMLButtonElement;

    expect(firstAlpha.getAttribute("aria-selected")).toBe("true");
    expect(firstAlpha.tabIndex).toBe(0);
    const firstAlphaPanel = getAllByRole("tabpanel", { name: "Alpha" })[0] as HTMLDivElement;
    expect(firstAlphaPanel.hidden).toBe(false);
    expect(firstAlpha.getAttribute("aria-controls")).toBe(firstAlphaPanel.id);
    expect(firstAlpha.id).not.toBe(secondAlpha.id);
    expect(firstBeta.id).toBe("explicit-beta");
    fireEvent.click(firstBeta);
    expect(changes).toHaveBeenCalledWith("beta");
    expect(firstBeta.getAttribute("aria-selected")).toBe("true");
    expect(getAllByRole("tabpanel", { name: "Beta" })[0]?.hidden).toBe(false);
    expect(firstAlphaPanel.isConnected).toBe(false);
    expect(document.querySelector('[role="tabpanel"][data-state="inactive"]')).toBeNull();
  });

  it("keeps adversarial Unicode associations injective across multiple roots", () => {
    const { getAllByRole } = render(
      <div>
        <AdversarialTabs group="First" />
        <AdversarialTabs group="Second" />
      </div>,
    );
    const triggers = getAllByRole("tab") as HTMLButtonElement[];
    const triggerIds = triggers.map((trigger) => trigger.id);
    const contentIds = triggers.map((trigger) => trigger.getAttribute("aria-controls"));
    expect(new Set(triggerIds).size).toBe(triggerIds.length);
    expect(new Set(contentIds).size).toBe(contentIds.length);

    for (const trigger of triggers.slice(0, adversarialValues.length)) {
      fireEvent.click(trigger);
      const panel = document.getElementById(trigger.getAttribute("aria-controls")!);
      expect(panel?.getAttribute("aria-labelledby")).toBe(trigger.id);
      expect(document.querySelectorAll('[role="tabpanel"]')).toHaveLength(2);
    }
  });

  it("hydrates server-rendered generated associations without changing IDs", () => {
    const element = <AdversarialTabs group="Hydrated" />;
    const container = document.createElement("div");
    container.innerHTML = renderToString(element);
    document.body.append(container);
    const before = Array.from(container.querySelectorAll<HTMLElement>("[id]")).map((part) => part.id);
    const errors = vi.spyOn(console, "error").mockImplementation(() => undefined);
    const hydrated = render(element, { container, hydrate: true });
    const after = Array.from(container.querySelectorAll<HTMLElement>("[id]")).map((part) => part.id);
    expect(after).toEqual(before);
    expect(errors).not.toHaveBeenCalled();
    hydrated.unmount();
    errors.mockRestore();
    container.remove();
  });

  it("unmounts inactive descendants before mounting the next panel", () => {
    const lifecycle: string[] = [];
    function Probe({ name }: { readonly name: string }) {
      React.useEffect(() => {
        lifecycle.push(`mount:${name}`);
        return () => lifecycle.push(`cleanup:${name}`);
      }, [name]);
      return <span>{name}</span>;
    }
    const { getByRole } = render(
      <Tabs defaultValue="alpha">
        <TabsList>
          <TabsTrigger value="alpha">Alpha</TabsTrigger>
          <TabsTrigger value="beta">Beta</TabsTrigger>
        </TabsList>
        <TabsContent value="alpha">
          <Probe name="alpha" />
        </TabsContent>
        <TabsContent value="beta">
          <Probe name="beta" />
        </TabsContent>
      </Tabs>,
    );
    expect(lifecycle).toEqual(["mount:alpha"]);
    fireEvent.click(getByRole("tab", { name: "Beta" }));
    expect(lifecycle).toEqual(["mount:alpha", "cleanup:alpha", "mount:beta"]);
  });

  it("emits every controlled-lag proposal once without optimistic state", () => {
    const changes = vi.fn();
    const { getByRole, rerender } = render(<TabsFixture value="alpha" onValueChange={changes} />);
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });

    fireEvent.click(beta);
    fireEvent.click(beta);
    fireEvent.click(beta);
    expect(changes.mock.calls).toEqual([["beta"], ["beta"], ["beta"]]);
    expect(alpha.getAttribute("aria-selected")).toBe("true");
    expect(beta.getAttribute("aria-selected")).toBe("false");
    rerender(<TabsFixture value="beta" onValueChange={changes} />);
    expect(beta.getAttribute("aria-selected")).toBe("true");
    fireEvent.click(alpha);
    expect(changes).toHaveBeenLastCalledWith("alpha");
  });

  it("skips disabled tabs and automatically activates horizontal LTR and RTL focus moves", () => {
    const { getByRole, rerender } = render(<TabsFixture defaultValue="alpha" />);
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });
    const disabled = getByRole("tab", { name: "Disabled" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowRight" });
    expect(document.activeElement).toBe(beta);
    expect(beta.getAttribute("aria-selected")).toBe("true");
    expect((disabled as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(disabled);
    expect(beta.getAttribute("aria-selected")).toBe("true");

    rerender(<TabsFixture defaultValue="alpha" dir="rtl" />);
    const rtlAlpha = getByRole("tab", { name: "Alpha" });
    const rtlBeta = getByRole("tab", { name: "Beta" });
    rtlAlpha.focus();
    fireEvent.keyDown(rtlAlpha, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(rtlBeta);
  });

  it("supports vertical Home/End and manual activation", () => {
    const changes = vi.fn();
    const { getByRole } = render(<TabsFixture value="alpha" onValueChange={changes} orientation="vertical" activationMode="manual" />);
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });
    const gamma = getByRole("tab", { name: "Gamma" });
    alpha.focus();
    fireEvent.keyDown(alpha, { key: "ArrowDown" });
    expect(document.activeElement).toBe(beta);
    expect(changes).not.toHaveBeenCalled();
    fireEvent.keyDown(beta, { key: "Enter" });
    expect(changes).toHaveBeenCalledWith("beta");
    fireEvent.keyDown(beta, { key: "End" });
    expect(document.activeElement).toBe(gamma);
    fireEvent.keyDown(gamma, { key: "Home" });
    expect(document.activeElement).toBe(alpha);
  });
});
// #endregion 📑️TabsMatrix
