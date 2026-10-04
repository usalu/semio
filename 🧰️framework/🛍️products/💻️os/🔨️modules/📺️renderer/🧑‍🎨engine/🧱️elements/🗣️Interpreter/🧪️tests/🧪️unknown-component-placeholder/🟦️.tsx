type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../📃️UiDocumentStore/🟦️.tsx"), "DEFAULT_UI_DOCUMENT_LIMITS" | "UiDocumentStore"> & Pick<typeof import("../../🟦️.tsx"), "UiNodeView" | "accessibilityAriaProps"> & Pick<typeof import("react"), "Profiler">, source: TestSource): Promise<void> {
  const { DEFAULT_UI_DOCUMENT_LIMITS, Profiler, UiDocumentStore, UiNodeView, accessibilityAriaProps } = dependencies;
  type AccessibilitySpec = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").AccessibilitySpec;
  type Component = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").Component;
  type PatchRejection = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").PatchRejection;
  type StyleSpec = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").StyleSpec;
  type UiDocumentLimits = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiDocumentLimits;
  type UiInterpreterContext = import("../../🟦️.tsx").UiInterpreterContext;
  type UiNodeRecord = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiNodeRecord;
  type UiPatch = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiPatch;
  type UiSnapshot = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiSnapshot;

  const { describe, expect, it, vi } = vitest;

  const TEST_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

  function leaf(id: number, key: string, component: Component, children: readonly number[] = []): UiNodeRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
  }

  function snapshot(root: number, nodes: readonly UiNodeRecord[]): UiSnapshot {
    return { surface: "s", revision: 0, root, nodes: [...nodes], layoutEpoch: 0n };
  }

  const noopContext: UiInterpreterContext = { store: new UiDocumentStore("noop"), onAction: () => {}, onIntent: () => {} };

  describe("unknown component placeholder", () => {
    it("renders a visible placeholder and never nothing for an unregistered component type", async () => {
      const { render, cleanup } = await import("@semio-tech/ui-react/test");
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [leaf(0, "root", { type: "future-widget" } as unknown as Component)]));
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      const { container } = render(<UiNodeView store={store} id={0} context={{ ...noopContext, store }} />);
      expect(container.querySelector("[data-unknown-component]")).not.toBeNull();
      expect(container.textContent).toMatch(/Unrecognized component/);
      expect(errorSpy).toHaveBeenCalled();
      errorSpy.mockRestore();
      cleanup();
    });
  });

  describe("per-node render granularity (React level)", () => {
    it("re-renders only the component whose own record changed", async () => {
      const { act, render, cleanup } = await import("@semio-tech/ui-react/test");
      const store = new UiDocumentStore("s");
      // 🧭️ `root` owns `a`/`b` as real document children (required — every node must be reachable
      // from the root or `validateUiDocumentCore` rejects the whole document as `danglingRoot`), but
      // this test deliberately mounts `a`/`b` as two INDEPENDENT top-level `UiNodeView` trees rather
      // than rendering `root` and letting `ContainerView` recurse into them — mounting `root` too
      // would nest `a`/`b` a second time inside it, and `root`'s own `Profiler` would then correctly
      // fire on any descendant commit, which is not what this test measures.
      store.loadSnapshot(
        snapshot(0, [
          leaf(0, "root", { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, [1, 2]),
          leaf(1, "a", { type: "text", value: "A", emphasize: null, dataAttributes: null }),
          leaf(2, "b", { type: "text", value: "B", emphasize: null, dataAttributes: null }),
        ]),
      );

      const aRenders = vi.fn();
      const bRenders = vi.fn();
      const context: UiInterpreterContext = { ...noopContext, store };
      const { unmount } = render(
        <>
          <Profiler id="a" onRender={aRenders}>
            <UiNodeView store={store} id={1} context={context} />
          </Profiler>
          <Profiler id="b" onRender={bRenders}>
            <UiNodeView store={store} id={2} context={context} />
          </Profiler>
        </>,
      );
      aRenders.mockClear();
      bRenders.mockClear();

      act(() => {
        const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 1, component: { type: "text", value: "A changed", emphasize: null, dataAttributes: null } }] });
        expect(result.ok).toBe(true);
      });

      expect(aRenders).toHaveBeenCalledTimes(1);
      expect(bRenders).toHaveBeenCalledTimes(0);
      unmount();
      cleanup();
    });
  });

  //#region CorpusConformance
  /** 🧪️ Consumes the shared conformance corpus (`🧬️contract/🧫️fixtures/🧪️conformance/`, 76 cases) —
   * the load-bearing proof that this React store agrees with the Rust `apply_patch`/`validate_snapshot`
   * the GPU renderer also builds on. For each accept case: loads the snapshot (+ patch, if present)
   * into a real `📃️UiDocumentStore` and asserts the retained tree shape, every node's accessibility
   * fields, and the full set of reachable `ActionId`s (formatted `scope.name@version`, matching
   * `ActionId::Display`) against the fixture's `.expect.json`. For each reject case: asserts
   * `applyPatch` rejects with the exact named `PatchRejection` AND that the store's state is left
   * reference-identical (not just value-equal) to before, mirroring `📃️UiDocumentStore`'s own guarantee. */
  describe("conformance corpus", async () => {
    // 🧭️ Dynamic imports (never static) — this file ships to the browser in production, and
    // `node:fs`/`node:path`/`node:url` must never enter that bundle. `vitest` dead-code
    // elimination strips this whole block (dynamic imports included) from non-test builds.
    const { readFileSync } = await import("node:fs");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");

    const here = dirname(fileURLToPath(source.url));
    const corpusRoot = join(here, "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance");

    type CorpusExpectation = {
      readonly case: string;
      readonly kind: string;
      readonly outcome: "accept" | "reject";
      readonly limits: UiDocumentLimits | null;
      readonly tree?: { readonly root: number; readonly nodeCount: number; readonly shape: readonly { readonly id: number; readonly key: string; readonly type: string; readonly children: readonly number[] }[] };
      readonly accessibility?: readonly { readonly id: number; readonly label: string | null; readonly description: string | null; readonly live: string; readonly shortcut: string | null; readonly hidden: boolean }[];
      readonly actionIds?: readonly string[];
      readonly baseRevision?: number;
      readonly patchRejection?: PatchRejection;
    };

    type CorpusCase = { readonly group: string; readonly name: string; readonly expect: CorpusExpectation; readonly snapshot: UiSnapshot; readonly patch: UiPatch | null };

    function loadCorpus(): readonly CorpusCase[] {
      const cases: CorpusCase[] = [];
      const catalog = JSON.parse(readFileSync(join(corpusRoot, "📇️catalog.json"), "utf8")) as { roles: { snapshot: string; expect: string; patch: string }; groups: Record<string, { patch: boolean; cases: Record<string, string> }> };
      for (const [group, definition] of Object.entries(catalog.groups)) {
        for (const [name, directory] of Object.entries(definition.cases)) {
          const caseDir = join(corpusRoot, group, directory);
          const expectation = JSON.parse(readFileSync(join(caseDir, catalog.roles.expect), "utf8")) as CorpusExpectation;
          const snap = JSON.parse(readFileSync(join(caseDir, catalog.roles.snapshot), "utf8")) as UiSnapshot;
          const patch = definition.patch ? JSON.parse(readFileSync(join(caseDir, catalog.roles.patch), "utf8")) as UiPatch : null;
          cases.push({ group, name, expect: expectation, snapshot: snap, patch });
        }
      }
      return cases;
    }

    /** 🧭️ The corpus's `actionIds`, defined once for every language harness (the Rust contract law reads it the same way): every
     * binding of every node, nodes in ascending id order, duplicates kept — two cells bound to one verb are two reachable actions. */
    /** 🎯️ Every action a record can dispatch: its bindings, then — for a tree/table row — its ONE target's activation and
     * each row action's verb, all bound to that target's scope (the Rust twin is `reachable_action_ids`). */
    function allActionIds(state: ReturnType<InstanceType<typeof UiDocumentStore>["getState"]>): string[] {
      return [...state.nodes.values()].sort((left, right) => left.id - right.id).flatMap((record) => {
        const row = record.component as { type: string; target?: { scope: string; version: number; activation?: string | null } | null; rowActions?: readonly { verb: string }[] };
        const target = row.type === "treeItem" || row.type === "tableRow" ? row.target ?? null : null;
        const verbs = target ? [...(target.activation ? [target.activation] : []), ...(row.rowActions ?? []).map((action) => action.verb)] : [];
        return [...(record.bindings ?? []).map((binding) => `${binding.action.scope}.${binding.action.name}@${binding.action.version}`), ...verbs.map((verb) => `${target!.scope}.${verb}@${target!.version}`)];
      });
    }

    const cases = loadCorpus();
    it("loads all 77 corpus fixtures", () => {
      expect(cases.length).toBe(77);
    });

    for (const testCase of cases) {
      it(`${testCase.group}/${testCase.name} — ${testCase.expect.outcome}`, () => {
        const limits = testCase.expect.limits ?? DEFAULT_UI_DOCUMENT_LIMITS;
        const store = new UiDocumentStore(testCase.snapshot.surface, limits);
        store.loadSnapshot(testCase.snapshot);

        if (!testCase.patch) {
          expect(testCase.expect.outcome).toBe("accept");
        } else if (testCase.expect.outcome === "reject") {
          const before = store.getState();
          const result = store.applyPatch(testCase.patch);
          expect(result.ok).toBe(false);
          if (!result.ok) expect(result.rejection).toEqual(testCase.expect.patchRejection);
          expect(store.getState()).toBe(before);
          return;
        } else {
          const applied = store.applyPatch(testCase.patch);
          expect(applied.ok).toBe(true);
        }

        const state = store.getState();
        if (testCase.expect.tree) {
          expect(state.root).toBe(testCase.expect.tree.root);
          expect(state.nodes.size).toBe(testCase.expect.tree.nodeCount);
          for (const expected of testCase.expect.tree.shape) {
            const record = state.nodes.get(expected.id);
            expect(record, `node ${expected.id} should exist`).toBeDefined();
            expect(record!.key).toBe(expected.key);
            expect(record!.component.type).toBe(expected.type);
            expect([...(record!.children ?? [])]).toEqual(expected.children);
          }
        }
        if (testCase.expect.accessibility) {
          for (const expected of testCase.expect.accessibility) {
            const record = state.nodes.get(expected.id)!;
            const { props: aria } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
            expect(record.accessibility.label ?? null).toBe(expected.label);
            expect(record.accessibility.description ?? null).toBe(expected.description);
            expect(record.accessibility.live ?? "off").toBe(expected.live);
            expect(record.accessibility.shortcut ?? null).toBe(expected.shortcut);
            expect(record.accessibility.hidden ?? false).toBe(expected.hidden);
            if (expected.label) expect(aria["aria-label"]).toBe(expected.label);
            if (expected.hidden) expect(aria["aria-hidden"]).toBe(true);
          }
        }
        expect(allActionIds(state)).toEqual(testCase.expect.actionIds ?? []);
      });
    }

    /** 🎚️ The number-control and recipe cases render their own semantics: one tick per detent, a stepper and
     * number fields at their precision, chips named by their remove verb, a destructive choice described by
     * its consequence — the React half of the wgpu `🧪️conformance-corpus` law over the same snapshots. */
    it("renders the number-control and recipe cases with their own semantics", async () => {
      const { render, cleanup, fireEvent } = await import("@semio-tech/ui-react/test");
      const mount = (name: string) => {
        const testCase = cases.find((candidate) => candidate.name === name)!;
        const store = new UiDocumentStore(testCase.snapshot.surface);
        store.loadSnapshot(testCase.snapshot);
        return render(<UiNodeView store={store} id={testCase.snapshot.root} context={{ ...noopContext, store }} />);
      };
      const slider = mount("slider-with-snaps");
      expect([...slider.container.querySelectorAll<HTMLElement>('[data-slot="slider-tick"]')].map((tick) => tick.dataset.snap)).toEqual(["2.5", "5", "7.5"]);
      cleanup();
      const stepper = mount("stepper-precision");
      expect(stepper.container.querySelector<HTMLInputElement>('[data-stepper-input="true"]')!.value).toBe("2.50");
      cleanup();
      const vector = mount("vector-input");
      const axes = [...vector.container.querySelectorAll<HTMLInputElement>('input[type="number"]')];
      expect(axes.map((axis) => [axis.value, axis.step])).toEqual([["1.3", "0.5"], ["-3.0", "0.5"]]);
      fireEvent.keyDown(axes[0]!, { key: "PageDown" });
      fireEvent.keyDown(axes[1]!, { key: "PageUp" });
      expect(axes.map((axis) => axis.value)).toEqual(["0.0", "0.0"]);
      fireEvent.keyDown(axes[0]!, { key: "PageUp" });
      expect(axes[0]!.value).toBe("5.0");
      cleanup();
      const color = mount("color-input");
      const swatch = color.container.querySelector<HTMLInputElement>('input[type="color"]')!;
      expect([swatch.value, swatch.getAttribute("aria-label")]).toEqual(["#ff8000", "Tint"]);
      const hex = color.container.querySelector<HTMLInputElement>('input[aria-label="Hex"]')!;
      expect(hex.value).toBe("#ff800080");
      const alpha = color.container.querySelector<HTMLElement>('[role="slider"]')!;
      expect([alpha.getAttribute("aria-valuemin"), alpha.getAttribute("aria-valuemax"), alpha.getAttribute("aria-valuenow")]).toEqual(["0", "1", "0.5"]);
      cleanup();
      const rows = mount("tree-row-recipes");
      const named = (label: string) => rows.container.querySelector<HTMLElement>(`[aria-label="${label}"]`);
      for (const label of ["Tint", "Hex", "Alpha", "X", "Y", "Remove Piece 3", "Remove Piece 7", "Use selection", "Add target", "Replay progress"]) expect(named(label), label).not.toBeNull();
      expect([...rows.container.querySelectorAll<HTMLInputElement>('input[type="color"]')].map((swatch) => swatch.value)).toEqual(["#ff8000"]);
      expect(rows.container.querySelector('[role="progressbar"]')?.getAttribute("aria-valuenow")).toBe("3");
      expect(rows.container.textContent).toContain("Replaying 3 of 7");
      const rowOf = (label: string) => named(label)!.closest('[role="treeitem"]')?.getAttribute("aria-label") ?? named(label)!.closest('[role="treeitem"]')?.textContent;
      for (const [label, row] of [["Hex", "Tint"], ["X", "Offset"], ["Remove Piece 3", "Targets"], ["Replay progress", "Replay"]] as const) expect(rowOf(label), label).toContain(row);
      cleanup();
      const references = mount("reference-list");
      expect([...references.container.querySelectorAll("button")].map((button) => button.getAttribute("aria-label"))).toEqual(expect.arrayContaining(["Remove Piece 3", "Remove Piece 7", "Use selection"]));
      cleanup();
      const choices = mount("dialog-choices");
      const overwrite = choices.container.querySelector<HTMLButtonElement>('button[aria-label="Overwrite"]')!;
      const described = (overwrite.getAttribute("aria-describedby") ?? "").split(" ").map((id) => document.getElementById(id)?.textContent ?? "").join(" ");
      expect(described).toContain("Replaces the edited mutations in every alternative that contains them.");
      cleanup();
    });

    /** 🧭️ The G6 number-control cases render every descriptor facet: a degree dial with its detent ticks and display-unit
     * numbers, log-axis ticks, a display factor read back exactly, stepper detents on the page keys, and a hard-bound refusal
     * that keeps the draft, names the bound and dispatches nothing. */
    it("renders the dial, log, display-factor, detent and hard-bound cases with every facet", async () => {
      const { render, cleanup, fireEvent } = await import("@semio-tech/ui-react/test");
      const intents = vi.fn();
      const actions = vi.fn();
      const mount = (name: string) => {
        const testCase = cases.find((candidate) => candidate.name === name)!;
        const store = new UiDocumentStore(testCase.snapshot.surface);
        store.loadSnapshot(testCase.snapshot);
        return render(<UiNodeView store={store} id={testCase.snapshot.root} context={{ store, onAction: actions, onIntent: intents }} />);
      };
      const dial = mount("dial-with-snaps");
      expect(dial.container.querySelector('[data-slot="slider-dial"]')).not.toBeNull();
      expect(dial.container.querySelectorAll('[data-slot="slider-tick"]').length).toBe(5);
      const angle = dial.container.querySelector<HTMLElement>('[role="slider"]')!;
      expect(["aria-valuemin", "aria-valuemax", "aria-valuenow", "aria-valuetext", "aria-label"].map((name) => angle.getAttribute(name))).toEqual(["-180", "180", "90", "90 °", "Angle"]);
      expect(dial.container.querySelector('[data-slot="slider-value"]')!.textContent).toBe("90");
      cleanup();
      const log = mount("log-slider");
      const ticks = [...log.container.querySelectorAll<HTMLElement>('[data-slot="slider-tick"]')].map((tick) => Number.parseFloat(tick.style.left));
      expect(ticks.map((tick) => Number(tick.toFixed(6)))).toEqual([0.25, 0.5, 1, 2, 4].map((snap) => Number(((Math.log(snap / 0.1) / Math.log(100)) * 100).toFixed(6))));
      expect(log.container.querySelector('[data-slot="slider-value"]')!.textContent).toBe("1.00");
      cleanup();
      const heading = mount("display-factor");
      const degrees = heading.container.querySelector<HTMLInputElement>('[data-stepper-input="true"]')!;
      expect([degrees.value, degrees.getAttribute("aria-valuetext")]).toEqual(["45", "45 °"]);
      expect(heading.container.querySelector('[data-slot="stepper-unit"]')!.textContent).toBe("°");
      cleanup();
      const gap = mount("stepper-detents");
      const field = gap.container.querySelector<HTMLInputElement>('[data-stepper-input="true"]')!;
      expect(field.value).toBe("2.0");
      fireEvent.keyDown(field, { key: "PageUp" });
      expect(field.value).toBe("5.0");
      cleanup();
      intents.mockClear();
      actions.mockClear();
      const width = mount("hard-bound-refusal");
      const number = width.container.querySelector<HTMLInputElement>('input[type="number"]')!;
      fireEvent.change(number, { target: { value: "-1" } });
      fireEvent.blur(number);
      expect(width.container.querySelector('[role="alert"]')?.textContent).toBe("Must be greater than 0");
      expect([number.value, number.getAttribute("aria-invalid")]).toEqual(["-1", "true"]);
      fireEvent.change(number, { target: { value: "120" } });
      fireEvent.blur(number);
      expect(width.container.querySelector('[role="alert"]')?.textContent).toBe("Must be at most 100");
      expect(intents).not.toHaveBeenCalled();
      expect(actions).not.toHaveBeenCalled();
      cleanup();
    });

    /** 💬️ The `row-semantics` case (`rowSemantics`, shared with the wgpu renderer law): a property row's description is
     * visible and named by its control through `aria-describedby`; a disabled row action stays focusable with
     * `aria-disabled`, names its reason through `aria-describedby`, shows that same reason as visible text on every
     * `revealReason.on` trigger and hides it on every `revealReason.off` trigger, and activating it dispatches nothing; an
     * option row exposes its choice as `aria-selected` and paints the chosen row selected. */
    it("exposes row descriptions on their controls and keeps disabled row actions focusable with their reason", async () => {
      const { render, cleanup, fireEvent, act } = await import("@semio-tech/ui-react/test");
      const { CHROME_CONTROL_TOOLTIP_DELAY_MS } = await import("@semio-tech/ui-react");
      const intents = vi.fn();
      const actions = vi.fn();
      const testCase = cases.find((candidate) => candidate.name === "row-semantics")!;
      type Reveal = "hover" | "focus" | "press";
      type Conceal = "leave" | "blur" | "escape";
      const semantics = (testCase.expect as unknown as { readonly rowSemantics: { readonly describedControls: readonly { readonly description: string }[]; readonly disabledRowActions: readonly { readonly reason: string; readonly revealReason: { readonly on: readonly Reveal[]; readonly off: readonly Conceal[] } }[]; readonly selectedRows: readonly { readonly row: number; readonly selected: boolean }[] } }).rowSemantics;
      const store = new UiDocumentStore(testCase.snapshot.surface);
      store.loadSnapshot(testCase.snapshot);
      const view = render(<UiNodeView store={store} id={testCase.snapshot.root} context={{ store, onAction: actions, onIntent: intents }} />);
      const describedBy = (element: Element) => (element.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ");
      const control = view.container.querySelector<HTMLElement>('[role="slider"]')!;
      expect(describedBy(control)).toContain(semantics.describedControls[0]!.description);
      expect(view.container.textContent).toContain(semantics.describedControls[0]!.description);
      const edit = [...view.container.querySelectorAll<HTMLElement>('[data-slot="action"]')].find((button) => button.getAttribute("aria-disabled") === "true")!;
      expect([edit.hasAttribute("disabled"), edit.tabIndex >= 0]).toEqual([false, true]);
      expect(describedBy(edit)).toBe(semantics.disabledRowActions[0]!.reason);
      edit.focus();
      expect(document.activeElement).toBe(edit);
      fireEvent.click(edit);
      expect(intents).not.toHaveBeenCalled();
      expect(actions).not.toHaveBeenCalled();
      act(() => edit.blur());
      const disabled = semantics.disabledRowActions[0]!;
      const shown = () => document.querySelector('[data-slot="row-action-reason"][data-revealed]')?.textContent ?? null;
      const reveal: Record<Reveal, () => Promise<void>> = {
        hover: async () => {
          fireEvent.pointerEnter(edit, { pointerType: "mouse" });
          expect(shown(), "a hover reveals only after the tooltip delay").toBeNull();
          await act(async () => {
            await new Promise((resolve) => setTimeout(resolve, CHROME_CONTROL_TOOLTIP_DELAY_MS + 50));
          });
        },
        focus: async () => act(() => edit.focus()),
        press: async () => {
          fireEvent.click(edit);
        },
      };
      const conceal: Record<Conceal, () => void> = {
        leave: () => fireEvent.pointerLeave(edit, { pointerType: "mouse" }),
        blur: () => act(() => edit.blur()),
        escape: () => fireEvent.keyDown(edit, { key: "Escape" }),
      };
      expect([disabled.revealReason.on.length, disabled.revealReason.off.length]).toEqual([3, 3]);
      for (const [index, trigger] of disabled.revealReason.on.entries()) {
        expect(shown(), `hidden before ${trigger}`).toBeNull();
        await reveal[trigger]();
        expect(shown(), `${trigger} shows the reason`).toBe(disabled.reason);
        expect(describedBy(edit), `${trigger} keeps it the description`).toBe(disabled.reason);
        const off = disabled.revealReason.off[index]!;
        conceal[off]();
        expect(shown(), `${off} hides the reason`).toBeNull();
        expect(describedBy(edit), `${off} keeps it the description`).toBe(disabled.reason);
      }
      expect(intents).not.toHaveBeenCalled();
      expect(actions).not.toHaveBeenCalled();
      for (const row of semantics.selectedRows) {
        const record = testCase.snapshot.nodes.find((node) => node.id === row.row)!;
        const label = (record.component as { readonly label: string }).label;
        const item = [...view.container.querySelectorAll<HTMLElement>('[role="treeitem"]')].filter((candidate) => candidate.textContent?.includes(label)).at(-1)!;
        expect([label, item.getAttribute("aria-selected"), item.hasAttribute("data-selected")]).toEqual([label, String(row.selected), row.selected]);
      }
      cleanup();
    });
  });
  //#endregion CorpusConformance

}
