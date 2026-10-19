import type { PanelTabNode } from "@semio-tech/ui-react/source";
import { createMemoryStoragePort } from "../../../🔨️modules/🗄️storage-port/🟦️.ts";
import paneOwnerFixture from "../../🧫️fixtures/🪪️window-surface-owner/🔣️.json" with { type: "json" };
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents8Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Canvas, Mode, Pane, Panel, React, Search, ShellScopeProvider, UiMobileProvider, Window, childElementId, createMemoryStoragePort, createShellScope, engagementActionTokenEquals, normalizeEngagementActionText, reactHostPort, singleTreeLeaf, uiDataLabel, windowMeasuresDefaultWidthPx } = dependencies;
  type PanelTabNode = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");
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

    it("Mode Escape aborts active window search", async () => {
      const aborted: string[] = [];
      const Harness = () => (
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
                    search: {
                      input: { value: "Box", placeholder: uiDataLabel("Action"), onChange: () => {}, onAbort: () => aborted.push("abort") },
                    },
                    children: <div data-testid="window-body">Body</div>,
                  },
                ]}
                layout={{ kind: "stack", children: [{ kind: "window", id: "engagement-window" }] }}
              />
            </div>
          )}
        </TestShellRoot>
      );
      const { container } = render(<Harness />);
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      await waitFor(() => expect(screen.getByPlaceholderText("Action")).toBeTruthy());
      fireEvent.keyDown(container.querySelector('[data-slot="mode"]')!, { key: "Escape", bubbles: true });
      expect(aborted).toEqual(["abort"]);
    });

    it("Window keeps engagement options reachable while the Actions pane is folded", () => {
      const pressed: string[] = [];
      const { container } = render(
        <Window
          id="quick-actions-window"
          active
          engagement={{
            options: [{ id: "shell-menu.action.openAddObjectDialog", label: uiDataLabel("Add Object…"), icon: { kind: "text", text: "+" }, onPress: () => pressed.push("open") }],
          }}
        >
          <div>Body</div>
        </Window>,
      );
      const quick = container.querySelector('[data-slot="window-engagement-quick-actions"]') as HTMLElement;
      const trigger = container.querySelector('[id="shell-menu.action.openAddObjectDialog"]') as HTMLElement;
      expect(quick).toBeTruthy();
      expect(trigger).toBeTruthy();
      expect(container.querySelector('[data-slot="window-engagement-body"]')).toBeNull();
      fireEvent.click(trigger);
      expect(pressed).toEqual(["open"]);
    });

    it("Window shows engagement and search as folded strips by default, same U-cutout surface as window options", () => {
      const { container } = render(
        <Window id="engagement-window" active engagement={{ sessionActive: true, status: [{ id: "s", content: "Idle" }] }} search={{ sessionActive: true, input: { value: "Box", placeholder: uiDataLabel("Action") } }}>
          <div>Body</div>
        </Window>,
      );
      const engagementZone = container.querySelector('[data-slot="window-engagement-zone"]') as HTMLElement;
      const searchZone = container.querySelector('[data-slot="window-search-zone"]') as HTMLElement;
      expect(engagementZone).toBeTruthy();
      expect(engagementZone.getAttribute("data-level")).toBe("pane");
      expect(engagementZone.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(searchZone).toBeTruthy();
      expect(searchZone.getAttribute("data-level")).toBe("pane");
      expect(searchZone.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')!);
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      expect(screen.getByText("Idle")).toBeTruthy();
      expect(screen.getByPlaceholderText("Action")).toBeTruthy();
      expect(container.querySelector('[data-slot="window-engagement-body"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-search-body"]')).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      expect(screen.getByText("Idle")).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')!);
      expect(screen.queryByText("Idle")).toBeNull();
    });

    it("Window merges the ad-hoc actionPane into the top-left Actions pane below the active engagement, sharing one toggle", () => {
      const { container } = render(
        <Window id="merged-window" active engagement={{ status: [{ id: "s", content: "Idle" }] }} actionPane={<div data-testid="adhoc-actions">Extrude…</div>}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="window-action-pane-overlay"]')).toBeNull();
      expect(container.querySelector('[data-testid="adhoc-actions"]')).toBeNull();
      const toggle = container.querySelector('[id="framework.window.mergedWindow.engagement.toggle"]') as HTMLElement;
      expect(toggle).toBeTruthy();
      fireEvent.click(toggle);
      expect(screen.getByText("Idle")).toBeTruthy();
      const actions = screen.getByTestId("adhoc-actions");
      expect(actions).toBeTruthy();
      const body = container.querySelector('[data-slot="window-engagement-body"]');
      expect(body?.contains(actions)).toBe(true);
      fireEvent.click(container.querySelector('[id="framework.window.mergedWindow.engagement.toggle"]')!);
      expect(container.querySelector('[data-testid="adhoc-actions"]')).toBeNull();
    });

    it("Window shows the merged Actions pane for an actionPane-only window with no active engagement", () => {
      const { container } = render(
        <Window id="actions-only-window" active actionPane={<div data-testid="adhoc-actions">Flatten</div>}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-engagement-overlay"]');
      expect(overlay).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.actionsOnlyWindow.engagement.toggle"]')!);
      expect(screen.getByTestId("adhoc-actions")).toBeTruthy();
    });

    it("Window pane inputs keep the concrete owner through activation and typing", async () => {
      const { default: Ajv2020 } = await import("ajv/dist/2020.js");
      for (const entry of paneOwnerFixture.cases) {
        const activations: string[] = [];
        const changes: { window: string; value: string }[] = [];
        const field = <input aria-label="Pane owner input" onChange={(event) => changes.push({ window: entry.window, value: event.target.value })} />;
        const { container, unmount } = render(
          <Window
            id={entry.window}
            onActivate={() => activations.push(entry.window)}
            measures={entry.kind === "measures" ? field : undefined}
            actionPane={entry.kind === "actions" ? field : undefined}
            search={entry.kind === "search" ? { input: { id: "pane-owner-input", value: "", placeholder: uiDataLabel("Pane owner input"), onChange: (value: string) => changes.push({ window: entry.window, value }) } } : undefined}
          >{entry.kind === "body" ? field : <div>Canvas</div>}</Window>
        );
        try {
          const toggleParts = entry.kind === "measures" ? ["measures", "unfold"] : entry.kind === "actions" ? ["engagement", "toggle"] : ["search", "toggle"];
          if (entry.kind !== "body") {
            const toggle = container.querySelector(`[id="${childElementId("framework.window", entry.window, ...toggleParts)}"]`);
            expect(toggle).toBeTruthy();
            fireEvent.click(toggle!);
          }
          const input = await waitFor(() => {
            const input = container.querySelector("input") as HTMLInputElement | null;
            expect(input).toBeTruthy();
            return input!;
          });
          activations.length = 0;
          fireEvent.pointerDown(input, { button: 0, bubbles: true });
          input.focus();
          expect(document.activeElement).toBe(input);
          fireEvent.change(input, { target: { value: paneOwnerFixture.typed } });
          expect(activations, entry.kind).toEqual([entry.window]);
          expect(changes, entry.kind).toEqual([{ window: entry.window, value: paneOwnerFixture.typed }]);
        } finally {
          unmount();
        }
      }
    });

    it("Window focus follows the shared pane sequence and ignores a late sibling blur", () => {
      const owner = paneOwnerFixture.cases[0].window;
      const activations: string[] = [];
      const field = (kind: string) => <input data-pane-kind={kind} aria-label={kind} />;
      const view = render(<Window id={owner} onActivate={() => activations.push(owner)}
        measures={field("measures")} actionPane={field("actions")}
        search={{ input: { id: "sequence-search", value: "", placeholder: uiDataLabel("sequence-search") } }}
      >{field("body")}</Window>);
      try {
        fireEvent.click(view.container.querySelector(`[id="${childElementId("framework.window", owner, "engagement", "toggle")}"]`)!);
        fireEvent.click(view.container.querySelector(`[id="${childElementId("framework.window", owner, "search", "toggle")}"]`)!);
        fireEvent.click(view.container.querySelector(`[id="${childElementId("framework.window", owner, "measures", "unfold")}"]`)!);
        const inputFor = (kind: string) => view.container.querySelector(kind === "search" ? 'input[placeholder="sequence-search"]' : `input[data-pane-kind="${kind}"]`) as HTMLInputElement;
        let previous: HTMLInputElement | undefined;
        for (const kind of paneOwnerFixture.focusSequence) {
          const input = inputFor(kind);
          expect(input, kind).toBeTruthy();
          fireEvent.pointerDown(input, { button: 0, bubbles: true });
          input.focus();
          expect(document.activeElement, kind).toBe(input);
          if (previous) expect(document.activeElement).not.toBe(previous);
          previous = input;
        }
        fireEvent.blur(inputFor(paneOwnerFixture.lateBlur));
        expect(document.activeElement).toBe(previous);
        expect(activations).toEqual(paneOwnerFixture.focusSequence.map(() => owner));
      } finally {
        view.unmount();
      }
    });

    it("Panel input focus leaves the active Window unchanged and retires on hide", async () => {
      const activations: string[] = [];
      const changes: string[] = [];
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({
        id: paneOwnerFixture.panel.surface, icon: StubIcon, name: paneOwnerFixture.panel.label,
        tree: { sections: [{ id: "panel-owner-section", label: "", items: [{
          id: "panel-owner-row", label: "", control: <input aria-label="Panel owner input" onChange={(event) => changes.push(event.target.value)} />
        }] }] },
      })];
      const content = (visible: boolean) => <>
        <Window id={paneOwnerFixture.previousWindow} active onActivate={() => activations.push(paneOwnerFixture.previousWindow)}><div>Canvas</div></Window>
        <Panel anchor="bottom-right" visible={visible} tabs={tabs} activeTabPath={[paneOwnerFixture.panel.surface]} />
      </>;
      const view = render(content(true));
      try {
        const input = view.container.querySelector('input[aria-label="Panel owner input"]') as HTMLInputElement;
        expect(input).toBeTruthy();
        fireEvent.pointerDown(input, { button: 0, bubbles: true });
        input.focus();
        expect(document.activeElement).toBe(input);
        fireEvent.change(input, { target: { value: paneOwnerFixture.typed } });
        expect(changes).toEqual([paneOwnerFixture.typed]);
        expect(activations).toEqual([]);
        view.rerender(content(false));
        expect(document.activeElement).not.toBe(input);
        expect(view.container.contains(input)).toBe(false);
      } finally {
        view.unmount();
      }
    });

    it("Window reveals search on button click and activates on click", async () => {
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <Window id="engagement-window" active search={{ input: { id: "search-input", value, placeholder: uiDataLabel("Action"), onChange: setValue } }}>
            <div data-testid="window-body">Body</div>
          </Window>
        );
      };
      const { container } = render(<Harness />);
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.search.toggle"]') as HTMLElement;
      expect(toggleBtn).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.click(toggleBtn);
      const activeField = await waitFor(() => {
        const next = screen.getByPlaceholderText("Action") as HTMLInputElement;
        expect(document.activeElement).toBe(next);
        return next;
      });
      expect(activeField.tabIndex).toBe(0);
      expect(document.querySelector('[data-slot="search"]')?.getAttribute("data-active")).toBe("true");
      fireEvent.change(activeField, { target: { value: "b" } });
      await waitFor(() => {
        expect(activeField.value).toBe("b");
      });
    });

    it("Search onChange carries the typed line verbatim, spaces and arguments included", () => {
      const changed: string[] = [];
      render(<Search input={{ value: "", onChange: (next: string) => changed.push(next), placeholder: uiDataLabel("Action") }} />);
      const field = screen.getByPlaceholderText("Action");
      fireEvent.change(field, { target: { value: "set height" } });
      fireEvent.change(field, { target: { value: "move 50 25" } });
      expect(changed).toEqual(["set height", "move 50 25"]);
    });

    it("normalizeEngagementActionText strips separators and PascalCases action NAMES (never the typed line)", () => {
      expect(normalizeEngagementActionText("set height 5")).toBe("SetHeight5");
      expect(normalizeEngagementActionText("b ")).toBe("B");
      expect(normalizeEngagementActionText("box")).toBe("Box");
      expect(normalizeEngagementActionText("SetHeight")).toBe("SetHeight");
    });

    it("normalizeEngagementActionText preserves decimal points inside numbers", () => {
      expect(normalizeEngagementActionText("set height 3.5")).toBe("SetHeight3.5");
      expect(normalizeEngagementActionText("3.5")).toBe("3.5");
      expect(normalizeEngagementActionText("0.25")).toBe("0.25");
      expect(normalizeEngagementActionText("dist 12.75")).toBe("Dist12.75");
    });

    it("engagementActionTokenEquals matches tokens regardless of casing", () => {
      expect(engagementActionTokenEquals("brush", "Brush")).toBe(true);
      expect(engagementActionTokenEquals("SELECT", "select")).toBe(true);
      expect(engagementActionTokenEquals("box", "sphere")).toBe(false);
    });

    it("Search keeps an argument line verbatim through change, Enter, and a session Space", async () => {
      const submitted: string[] = [];
      const Harness = ({ sessionActive }: { readonly sessionActive: boolean }) => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <Search
            active
            sessionActive={sessionActive}
            input={{
              id: "search-input",
              value,
              placeholder: uiDataLabel("Action"),
              onChange: setValue,
              onSubmit: (next: string) => submitted.push(next),
            }}
          />
        );
      };
      const { unmount } = render(<Harness sessionActive={false} />);
      const field = screen.getByPlaceholderText("Action") as HTMLInputElement;
      fireEvent.change(field, { target: { value: "move 50 25" } });
      await waitFor(() => expect(field.value).toBe("move 50 25"));
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(submitted).toEqual(["move 50 25"]));
      unmount();
      render(<Harness sessionActive />);
      const stepField = screen.getByPlaceholderText("Action") as HTMLInputElement;
      fireEvent.change(stepField, { target: { value: "rotate 45 " } });
      fireEvent.keyDown(stepField, { key: " " });
      await waitFor(() => expect(submitted).toEqual(["move 50 25", "rotate 45"]));
    });

    it("Window anchors engagement in a top overlay when active", () => {
      const { container } = render(
        <Window id="engagement-window" active engagement={{ status: [{ id: "s", content: "Idle" }] }}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-engagement-overlay"]') as HTMLElement;
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]') as HTMLElement;
      expect(overlay).toBeTruthy();
      expect(overlay.style.top).toBe("var(--spacing-single)");
      expect(overlay.style.left).toBe("var(--spacing-single)");
      expect(overlay.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(overlay.getAttribute("data-folded")).toBe("true");
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(toggleBtn);
      expect(overlay.getAttribute("data-folded")).toBeNull();
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(screen.getByText("Idle")).toBeTruthy();
    });

    it("Window search action row spans the sized search zone", () => {
      const { container } = render(
        <Window id="engagement-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-search-overlay"]') as HTMLElement;
      const zone = container.querySelector('[data-slot="window-search-zone"]') as HTMLElement;
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.search.toggle"]') as HTMLElement;
      fireEvent.click(toggleBtn);
      expect(zone.className).toContain("w-full");
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      const row = container.querySelector('[data-slot="search-row"]');
      const inputRoot = container.querySelector('[data-slot="search-input"] [data-slot="input-root"]');
      expect(row?.className).toContain("w-full");
      expect(inputRoot?.className).toContain("w-full");
    });

    it("Window and ModeDock controls render with labels", () => {
      const { container } = render(
        <Window id="labeled-window" onOpenInNewWindow={() => {}} onMaximize={() => {}} onClose={() => {}} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );

      const newWindowBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.external"]');
      expect(newWindowBtn?.textContent?.trim()).toBe("New Window");

      const closeBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.close"]');
      expect(closeBtn?.textContent?.trim()).toBe("Close");

      const maximizeBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.maximize"]');
      expect(maximizeBtn?.textContent?.trim()).toBe("Focus");

      const unfoldBtn = container.querySelector('[id="framework.window.labeledWindow.measures.unfold"]');
      expect(unfoldBtn?.textContent?.trim()).toBe("Window Options");

      if (unfoldBtn) fireEvent.click(unfoldBtn);

      const foldToggle = container.querySelector('[id="framework.window.labeledWindow.measures.fold"]');
      expect(foldToggle?.textContent?.trim()).toBe("Window Options");
    });

    it("Window maximize renders as Unfocus when onMinimize is provided", () => {
      const { container } = render(
        <Window id="labeled-window-min" onMinimize={() => {}}>
          <div>Body</div>
        </Window>,
      );
      const maximizeBtn = container.querySelector('[id="framework.window.labeledWindowMin.windowControls.maximize"]');
      expect(maximizeBtn?.textContent?.trim()).toBe("Unfocus");
    });

    it("Window drops the Focus control on mobile — windows always take the full space — but keeps Close", () => {
      const { container } = render(
        <UiMobileProvider mobile>
          <Window id="mobile-window" onOpenInNewWindow={() => {}} onMaximize={() => {}} onClose={() => {}}>
            <div>Body</div>
          </Window>
        </UiMobileProvider>,
      );
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.maximize"]')).toBeNull();
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.external"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.close"]')).toBeTruthy();
    });

    it("Window mints no window-control group when it is handed no window control", () => {
      const { container } = render(
        <Window id="uncontrolled-window">
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[id="framework.window.uncontrolledWindow.windowControls"]')).toBeNull();
      expect([...container.querySelectorAll("[id]")].filter((element) => element.id.includes(".windowControls"))).toHaveLength(0);
    });

    it("Window mints the group again as soon as one control is handed to it", () => {
      const { container } = render(
        <Window id="controlled-window" onClose={() => {}}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[id="framework.window.controlledWindow.windowControls"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.controlledWindow.windowControls.close"]')).toBeTruthy();
    });

    it("Window search pane uses the same default width as panels when unfolded", () => {
      const { container } = render(
        <Window id="layout-window" active search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div data-testid="window-body">Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-search-overlay"]') as HTMLElement;
      fireEvent.click(container.querySelector('[id="framework.window.layoutWindow.search.toggle"]')!);
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(windowMeasuresDefaultWidthPx).toBe(300);
    });

    it("Window engagement and measures overlays pass pointer hits through to the canvas body", () => {
      const bodyDown = vi.fn();
      const { container } = render(
        <Window id="canvas-window" active fill engagement={{ options: [{ id: "opt-a", label: uiDataLabel("Alpha"), icon: "circle-dot", onPress: () => {} }] }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div data-testid="window-body" className="size-full" onPointerDown={bodyDown}>
            Body
          </div>
        </Window>,
      );
      const body = container.querySelector('[data-testid="window-body"]') as HTMLElement;
      const bodyRect = body.getBoundingClientRect();
      fireEvent.pointerDown(body, { clientX: bodyRect.right - 8, clientY: bodyRect.top + bodyRect.height * 0.55, bubbles: true });
      expect(bodyDown).toHaveBeenCalledTimes(1);
      fireEvent.pointerDown(body, { clientX: bodyRect.left + 12, clientY: bodyRect.top + 12, bubbles: true });
      expect(bodyDown).toHaveBeenCalledTimes(2);
    });

    it("Window keeps engagement and search pane toggles visible while inactive, like window options", () => {
      const { container } = render(
        <Window id="engagement-window" engagement={{ status: [{ id: "s", content: "Idle" }] }} search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="window"]')?.getAttribute("data-active")).toBeNull();
      expect(container.querySelector('[data-slot="window-engagement-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-search-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="utility-bar-overlay"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.measures.unfold"]')).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')!);
      expect(screen.getByText("Idle")).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      expect(screen.getByPlaceholderText("Action")).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.engagement"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.search"]')).toBeTruthy();
    });
});
}
