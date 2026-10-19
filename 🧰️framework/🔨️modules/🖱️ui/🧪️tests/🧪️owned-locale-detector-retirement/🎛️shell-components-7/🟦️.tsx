import { createMemoryStoragePort } from "../../../🔨️modules/🗄️storage-port/🟦️.ts";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents7Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Canvas, Engagement, Input, Mode, React, Scrollable, Search, ShellScopeProvider, Window, applySearchSpaceAction, createDOMEventBinding, createMemoryStoragePort, createShellScope, installElementsSurfaceBrowserDefaultSuppression, isUiTypingTarget, reactHostPort, readScrollerContentOverflows, routeWindowSearchEscape, routeWindowSearchSpace, searchSpaceConfirmsLine, shouldRouteKeysToWindowSearch, uiDataLabel } = dependencies;
  type SearchSpec = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, createEvent, waitFor } = await import("@testing-library/react");
  /** 🐚️ Test-only stand-in for `FrameworkOsShell`'s own scope wiring — wraps a render-prop's output in
     * a `ShellScopeProvider` whose root points at the wrapper div, so `useShellKeydown` consumers
     * (`Window`, `Mode`) route document keydowns the same way they do inside a real mounted shell.
     * `renderChildren` is a FUNCTION, not a plain `ReactNode`: `rootRef.current` is still `null` on the
     * initial render (refs attach during commit, after render finishes), so this needs a forced second
     * render — mirroring `FrameworkOsShell`'s own `bumpAfterRootAttach` — to ever see the populated root.
     * A plain `children` prop would defeat that: React treats an unchanged child *element* (the common
     * "pass `children` through" optimization) as a bailout and never re-invokes it, so a component like
     * `Mode` with no unrelated state of its own to force a second render would never re-read the ref.
     * Calling a function instead produces a fresh element every render, matching how `FrameworkOsShell`
     * really works — its inner tree is authored directly in its own body, not threaded through as an
     * opaque prop. */
    function TestShellRoot({ children }: { readonly children: () => React.ReactNode }): React.ReactElement {
      const [scope] = reactHostPort.useState(() => createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" }));
      const [, bump] = reactHostPort.useState(0);
      const setRoot = reactHostPort.useCallback(
        (node: HTMLDivElement | null) => {
          scope.rootRef.current = node;
          bump((n) => n + 1);
        },
        [scope],
      );
      return (
        <div ref={setRoot}>
          <ShellScopeProvider scope={scope}>{children()}</ShellScopeProvider>
        </div>
      );
    }

describe("Shell components", () => {

    it("Search Space and Enter activate inline suggestion without opening possibles list", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action") }}
          possibles={[
            { id: "primitive.box", label: "Box", detail: "b", onSelect: () => selected.push("primitive.box") },
            { id: "primitive.sphere", label: "Sphere", detail: "s", onSelect: () => selected.push("primitive.sphere") },
            { id: "puzzle3d.utility.fill", label: "Fill", detail: "f", onSelect: () => selected.push("puzzle3d.utility.fill") },
          ]}
        />,
      );
      const field = screen.getByPlaceholderText("Action");
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.change(field, { target: { value: "f" } });
      await waitFor(() => expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("ill"));
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(selected).toEqual(["puzzle3d.utility.fill"]));
      fireEvent.change(field, { target: { value: "Sp" } });
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(selected).toEqual(["puzzle3d.utility.fill", "primitive.sphere"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Search shows inline completion while typing and possibles list only on chevron", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action") }}
          possibles={[
            { id: "primitive.box", label: "Box", detail: "b", onSelect: () => selected.push("primitive.box") },
            { id: "primitive.sphere", label: "Sphere", detail: "s", onSelect: () => selected.push("primitive.sphere") },
          ]}
        />,
      );
      const field = screen.getByPlaceholderText("Action");
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.change(field, { target: { value: "b" } });
      await waitFor(() => {
        expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("ox");
        expect(document.querySelector('[data-slot="search-inline-completion"]')?.querySelector(".font-semibold")?.textContent).toBe("B");
      });
      fireEvent.change(field, { target: { value: "Sp" } });
      await waitFor(() => {
        expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("here");
        expect(document.querySelector('[data-slot="search-inline-completion"]')?.textContent).toContain("Sphere");
      });
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      await waitFor(() => expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeTruthy());
      expect(document.querySelector('[data-value="primitive.sphere"]')).toBeTruthy();
      fireEvent.change(field, { target: { value: "sph" } });
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere"]));
      fireEvent.change(field, { target: { value: "" } });
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere", "primitive.box"]));
      fireEvent.change(field, { target: { value: "" } });
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const sphereRow = document.querySelector('[data-slot="command-item"][data-value="primitive.sphere"]');
      expect(sphereRow).toBeTruthy();
      fireEvent.click(sphereRow!);
      await waitFor(() => expect(selected).toEqual(["primitive.sphere", "primitive.box", "primitive.sphere"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Search suggestion click selects the picked row when popover keeps input focus", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action"), value: "Extr", onChange: () => {} }}
          possibles={[
            { id: "feature.extrudeWire", label: "ExtrudeWire", detail: "e", onSelect: () => selected.push("feature.extrudeWire") },
            { id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e", onSelect: () => selected.push("surface.extrudeCrv") },
          ]}
        />,
      );
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const popover = await waitFor(() => document.querySelector('[data-slot="search-autocomplete"]')!);
      const crvRow = document.querySelector('[data-slot="command-item"][data-value="surface.extrudeCrv"]')!;
      const crvLabel = crvRow.querySelector("span")?.firstChild;
      expect(crvLabel).toBeTruthy();
      fireEvent.pointerDown(popover, { pointerId: 1, pointerType: "mouse", buttons: 1 });
      fireEvent.pointerDown(crvLabel!, { pointerId: 1, pointerType: "mouse", buttons: 1 });
      fireEvent.click(crvRow);
      await waitFor(() => expect(selected).toEqual(["surface.extrudeCrv"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Engagement renders toggleGroup and select controls", () => {
      const { container } = render(
        <Engagement
          sessionActive
          control={{
            kind: "toggleGroup",
            id: "engagement-utility-group",
            label: uiDataLabel("Utility"),
            value: "brush",
            options: [
              { id: "select", label: "Select", icon: "mouse-pointer" },
              { id: "brush", label: "Brush", icon: "paintbrush" },
            ],
          }}
          controls={[
            {
              kind: "select",
              id: "engagement-placement",
              label: uiDataLabel("Placement"),
              value: "a",
              items: [
                { id: "a", value: "a", label: "A" },
                { id: "b", value: "b", label: "B" },
              ],
            },
          ]}
        />,
      );
      expect(container.querySelector('[data-control-kind="toggleGroup"]')).toBeTruthy();
      expect(container.querySelector('[data-control-kind="select"]')).toBeTruthy();
    });

    it("applySearchSpaceAction submits empty draft during sessionActive", () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      const input = {
        onSubmit: (value: string) => submitted.push(value),
        onRepeatLast: () => repeated.push("last"),
      };
      expect(applySearchSpaceAction(input, "", true)).toBe(true);
      expect(submitted).toEqual([""]);
      expect(repeated).toEqual([]);
      submitted.length = 0;
      expect(applySearchSpaceAction(input, "", false)).toBe(true);
      expect(repeated).toEqual(["last"]);
      expect(submitted).toEqual([]);
    });

    it("routeWindowSearchSpace calls onRepeatLast for empty action", () => {
      const repeated: string[] = [];
      const search = { input: { value: "", onRepeatLast: () => repeated.push("last") } };
      const body = document.createElement("div");
      expect(
        routeWindowSearchSpace(search, {
          key: " ",
          ctrlKey: false,
          metaKey: false,
          altKey: false,
          defaultPrevented: false,
          isComposing: false,
          target: body,
        }),
      ).toBe(true);
      expect(repeated).toEqual(["last"]);
    });

    it("routeWindowSearchSpace calls onSubmit for non-empty action", () => {
      const submitted: string[] = [];
      const search = {
        input: {
          value: "Box",
          onSubmit: (value: string) => submitted.push(value),
          onRepeatLast: () => submitted.push("last"),
        },
      };
      const body = document.createElement("div");
      expect(
        routeWindowSearchSpace(search, {
          key: " ",
          ctrlKey: false,
          metaKey: false,
          altKey: false,
          defaultPrevented: false,
          isComposing: false,
          target: body,
        }),
      ).toBe(true);
      expect(submitted).toEqual(["Box"]);
    });

    it("Search Space with a draft submits the step during a session and types a separator when idle", async () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      const Harness = ({ sessionActive }: { readonly sessionActive: boolean }) => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <Search
            active
            sessionActive={sessionActive}
            input={{
              value,
              placeholder: uiDataLabel("Action"),
              onChange: setValue,
              onSubmit: (draft: string) => submitted.push(draft),
              onRepeatLast: () => repeated.push("last"),
            }}
          />
        );
      };
      const { unmount } = render(<Harness sessionActive />);
      const field = await screen.findByPlaceholderText("Action");
      fireEvent.change(field, { target: { value: "5" } });
      fireEvent.keyDown(field, { key: " " });
      expect(submitted).toEqual(["5"]);
      expect(repeated).toEqual([]);
      unmount();
      render(<Harness sessionActive={false} />);
      const idle = await screen.findByPlaceholderText("Action");
      fireEvent.change(idle, { target: { value: "move" } });
      const spaceEvent = createEvent.keyDown(idle, { key: " " });
      fireEvent(idle, spaceEvent);
      expect(spaceEvent.defaultPrevented).toBe(false);
      expect(submitted).toEqual(["5"]);
      expect(repeated).toEqual([]);
    });

    it("searchSpaceConfirmsLine confirms an empty line or a live session and types a separator otherwise", () => {
      expect(searchSpaceConfirmsLine("", false)).toBe(true);
      expect(searchSpaceConfirmsLine("", true)).toBe(true);
      expect(searchSpaceConfirmsLine("5", true)).toBe(true);
      expect(searchSpaceConfirmsLine("move", false)).toBe(false);
      expect(searchSpaceConfirmsLine("move 50", false)).toBe(false);
    });

    it("Search Space with empty draft calls onRepeatLast instead of onSubmit", async () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      render(
        <Search
          active
          input={{
            placeholder: uiDataLabel("Action"),
            onSubmit: () => submitted.push("submit"),
            onRepeatLast: () => repeated.push("last"),
          }}
        />,
      );
      const field = await screen.findByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: " " });
      expect(repeated).toEqual(["last"]);
      expect(submitted).toEqual([]);
    });

    it("Mode routes printable keys to the active window search", async () => {
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <TestShellRoot>
            {() => (
              <div className="h-layout-preview-md w-layout-floating-menu-lg">
                <Mode
                  activeWindowId="engagement-window"
                  windows={[
                    {
                      id: "engagement-window",
                      iconId: "search",
                      title: uiDataLabel("Viewport"),
                      active: true,
                      search: { input: { id: "search-input", value, placeholder: uiDataLabel("Action"), onChange: setValue } },
                      children: <div data-testid="window-body">Body</div>,
                    },
                  ]}
                  layout={{ kind: "stack", children: [{ kind: "window", id: "engagement-window" }] }}
                />
              </div>
            )}
          </TestShellRoot>
        );
      };
      const { container } = render(<Harness />);
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.keyDown(container.querySelector('[data-slot="mode"]')!, { key: "b", bubbles: true });
      await waitFor(() => {
        const typedField = screen.getByPlaceholderText("Action") as HTMLInputElement;
        expect(typedField.value).toBe("b");
        expect(typedField.tabIndex).toBe(0);
        expect(document.querySelector('[data-slot="search"]')?.getAttribute("data-active")).toBe("true");
      });
    });

    it("installElementsSurfaceBrowserDefaultSuppression blocks the native context menu and leaves Tab focus traversal to the browser", () => {
      const bindings = createDOMEventBinding();
      installElementsSurfaceBrowserDefaultSuppression(bindings);
      const panel = document.createElement("div");
      document.body.appendChild(panel);
      const contextEvent = new MouseEvent("contextmenu", { bubbles: true, cancelable: true });
      const contextPrevent = vi.spyOn(contextEvent, "preventDefault");
      panel.dispatchEvent(contextEvent);
      expect(contextPrevent).toHaveBeenCalled();
      const tabEvent = new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true });
      const tabPrevent = vi.spyOn(tabEvent, "preventDefault");
      panel.dispatchEvent(tabEvent);
      expect(tabPrevent).not.toHaveBeenCalled();
      const input = document.createElement("input");
      document.body.appendChild(input);
      input.focus();
      const tabInField = new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true });
      const tabInFieldPrevent = vi.spyOn(tabInField, "preventDefault");
      input.dispatchEvent(tabInField);
      expect(tabInFieldPrevent).not.toHaveBeenCalled();
      bindings.dispose();
      panel.remove();
      input.remove();
    });

    it("installElementsSurfaceBrowserDefaultSuppression disables browser defaults on focused form controls", () => {
      const bindings = createDOMEventBinding();
      installElementsSurfaceBrowserDefaultSuppression(bindings);
      const input = document.createElement("input");
      document.body.appendChild(input);
      input.focus();
      expect(input.autocomplete).toBe("off");
      expect(input.spellcheck).toBe(false);
      expect(input.getAttribute("autocorrect")).toBe("off");
      expect(input.dataset.uiBrowserDefaults).toBe("true");
      bindings.dispose();
      input.remove();
    });

    it("isUiTypingTarget treats text inputs, collapsed fields, and command inputs as typing targets", () => {
      const text = document.createElement("input");
      text.type = "text";
      expect(isUiTypingTarget(text)).toBe(true);
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      expect(isUiTypingTarget(checkbox)).toBe(false);
      const collapsed = document.createElement("div");
      collapsed.setAttribute("data-slot", "input");
      collapsed.setAttribute("data-collapsed", "true");
      expect(isUiTypingTarget(collapsed)).toBe(true);
      const command = document.createElement("input");
      command.setAttribute("data-slot", "command-input");
      expect(isUiTypingTarget(command)).toBe(true);
      const writer = document.createElement("div");
      writer.setAttribute("role", "textbox");
      writer.tabIndex = 0;
      expect(isUiTypingTarget(writer)).toBe(true);
      expect(shouldRouteKeysToWindowSearch(writer)).toBe(false);
      expect(shouldRouteKeysToWindowSearch(text)).toBe(false);
    });

    it("Window keeps bodies edgeless and chrome-aware scroll hosts preserve the first content", () => {
      const { container } = render(
        <Window id="chrome-aware-window" active search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div>LOD</div>}>
          <div data-window-content-layout="chrome-aware" className="flex min-h-0 flex-1 flex-col">
            <Scrollable className="h-40">
              <div style={{ height: 240 }}>Line one</div>
            </Scrollable>
          </div>
        </Window>,
      );
      const body = container.querySelector('[data-slot="window-body"]');
      expect(body?.className).not.toContain("has-[[data-window-content-layout=chrome-aware]]:pt-");
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      expect(scroller?.className).toContain("scroll-padding-top:var(--window-content-dead-line)");
      expect(scroller.scrollTop).toBe(0);
    });

    it("Window edgeless bodies do not apply a dead-line scroll offset", () => {
      const { container } = render(
        <Window id="edgeless-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <div data-window-content-layout="edgeless" className="h-40">
            <Scrollable className="h-full">
              <div style={{ height: 240 }}>Canvas</div>
            </Scrollable>
          </div>
        </Window>,
      );
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      expect(scroller.scrollTop).toBe(0);
    });

    it("Window dead-line hosts skip offset when content fits", () => {
      const { container } = render(
        <Window id="fits-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <Scrollable className="h-40">
            <div style={{ height: 40 }}>Short</div>
          </Scrollable>
        </Window>,
      );
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      Object.defineProperty(scroller, "clientHeight", { value: 160, configurable: true });
      Object.defineProperty(scroller, "scrollHeight", { value: 160, configurable: true });
      expect(readScrollerContentOverflows(scroller)).toBe(false);
    });

    it("Search does not steal focus from another input when it becomes active", async () => {
      const Harness = ({ active }: { active: boolean }) => (
        <>
          <Input id="other-input" placeholder="Other" />
          <Search active={active} input={{ placeholder: uiDataLabel("Action") }} />
        </>
      );
      const { rerender } = render(<Harness active={false} />);
      const other = screen.getByPlaceholderText("Other") as HTMLInputElement;
      other.focus();
      rerender(<Harness active />);
      await waitFor(() => expect(document.activeElement).toBe(other));
    });

    it("Search Escape calls onAbort when possibles list is closed", () => {
      const aborted: string[] = [];
      render(<Search active input={{ placeholder: uiDataLabel("Action"), onAbort: () => aborted.push("abort") }} possibles={[{ id: "a", label: "A", onSelect: () => {} }]} />);
      const field = screen.getByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual(["abort"]);
    });

    it("Search Escape closes possibles list before onAbort", () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const aborted: string[] = [];
      render(<Search active input={{ placeholder: uiDataLabel("Action"), onAbort: () => aborted.push("abort") }} possibles={[{ id: "a", label: "A", onSelect: () => {} }]} />);
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const field = screen.getByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual([]);
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual(["abort"]);
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("routeWindowSearchEscape calls onAbort when sessionActive without visible chrome", () => {
      const aborted: string[] = [];
      const search: SearchSpec = {
        sessionActive: true,
        input: { value: "", placeholder: uiDataLabel("Brush"), onAbort: () => aborted.push("abort") },
      };
      const handled = routeWindowSearchEscape(search, { key: "Escape", defaultPrevented: false, isComposing: false, target: document.body }, { chromeVisible: false, actionActive: false });
      expect(handled).toBe(true);
      expect(aborted).toEqual(["abort"]);
    });
});
}
