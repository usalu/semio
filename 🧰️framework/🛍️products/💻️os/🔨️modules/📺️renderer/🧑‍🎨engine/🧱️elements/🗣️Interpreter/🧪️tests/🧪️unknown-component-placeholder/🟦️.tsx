type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { DEFAULT_UI_DOCUMENT_LIMITS, Profiler, UiDocumentStore, UiNodeView, accessibilityAriaProps } = dependencies;
  type AccessibilitySpec = any;
  type Component = any;
  type PatchRejection = any;
  type StyleSpec = any;
  type UiDocumentLimits = any;
  type UiInterpreterContext = any;
  type UiNodeRecord = any;
  type UiPatch = any;
  type UiSnapshot = any;

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
  /** 🧪️ Consumes the shared conformance corpus (`🧬️contract/📚️examples/🧪️conformance/`, 62 cases) —
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
    const corpusRoot = join(here, "../../../../../../../🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance");

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

    function allActionIds(state: ReturnType<UiDocumentStore["getState"]>): string[] {
      const ids = new Set<string>();
      for (const record of state.nodes.values()) {
        for (const binding of record.bindings ?? []) ids.add(`${binding.action.scope}.${binding.action.name}@${binding.action.version}`);
      }
      return [...ids].sort();
    }

    const cases = loadCorpus();
    it("loads all 62 corpus fixtures", () => {
      expect(cases.length).toBe(62);
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
        if (testCase.expect.actionIds) {
          expect(allActionIds(state)).toEqual([...testCase.expect.actionIds].sort());
        }
      });
    }
  });
  //#endregion CorpusConformance

}
