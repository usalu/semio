/** 📋 Renderer types, owned declarative controls, external slots and declarative forms parity. */
import { createElement } from "react";
import { parseResolvedPluginViewState } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import sliderPresentationFixture from "../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎚️slider-presentation/🔣️.json" with { type: "json" };
import actionSemanticsFixture from "../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/⚖️action-semantics.json";
import presenceOverlayFixture from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/👥️presence-overlay.json";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { actionSemanticsForKind } from "@semio-tech/framework";
import { renderUiControl } from "../../🧱️elements/🗣️Interpreter/🎛️declarative-controls/🟦️.tsx";
import { appBreadcrumb, appWindowLabel, UiDocumentStore, type UiInterpreterContext, UiPresenceOverlayContext, type UiPresenceOverlayEntry, resolveAppBreadcrumb, flattenPanelTabLeaves, interpretUiNode, dagOverlayLabelFill, dagOverlayLabelFillHex } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { CONTRACT_LEAF_LAYOUT, buildContractSnapshot, noopAction, renderContractTree } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer types", () => {
  it("matches native action-semantics defaults without claiming migrated interactivity", () => {
    
    
    const actual = (["mutation", "view", "interaction", "history", "clipboard", "shell"] as const).map((kind) => ({ kind, semantics: actionSemanticsForKind(kind) }));
    expect(actual).toEqual(actionSemanticsFixture);
    
    
  });

  it("keeps window tabs concise while retaining the app fallback", () => {
    const app = {
      id: "puzzle3d-play",
      label: "Puzzle 3D",
      breadcrumb: ["semio", "puzzle", "3d"],
      terminologyBreadcrumbs: { reuse: ["Entwerfen mit Bestand", "Aggregator"] },
      controllerId: "puzzle3d-play",
      modes: [],
      windowKinds: [],
      panelTabs: [],
      keybindings: [],
    };
    expect(appBreadcrumb(app.breadcrumb)).toBe("semio · puzzle · 3d");
    expect(appWindowLabel(app, "native", "Flow")).toBe("Flow");
    expect(appWindowLabel(app, "native", "Preview")).toBe("Preview");
    expect(appWindowLabel(app, "native", "")).toBe("Puzzle 3D");
    expect(appWindowLabel(app, "reuse", "")).toBe("Aggregator");
    expect(resolveAppBreadcrumb(app, "native")).toEqual(["semio", "puzzle", "3d"]);
    expect(resolveAppBreadcrumb(app, "reuse")).toEqual(["Entwerfen mit Bestand", "Aggregator"]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "reuse"))).toBe("Entwerfen mit Bestand · Aggregator");
  });

  it("survives an app whose manifest declares no breadcrumb at all", () => {
    // 🛡️ `AppDefinition.breadcrumb` is OPTIONAL, so an app may legitimately ship without one — and
    // `appBreadcrumb` runs inside `FrameworkOsShellInner`'s RENDER. Before this guard, one such app
    // threw `Cannot read properties of undefined (reading 'join')` and took the whole shell down
    // with it; in a multi-pane host (the demonstrator) that killed all six panes at once and left
    // the page blank. A nameless title is the correct degradation, never a dead host.
    const app: Parameters<typeof resolveAppBreadcrumb>[0] = { breadcrumb: [], terminologyBreadcrumbs: {} };
    expect(resolveAppBreadcrumb(app, "native")).toEqual([]);
    expect(appBreadcrumb(resolveAppBreadcrumb(app, "native"))).toBe("");
    expect(appBreadcrumb(undefined)).toBe("");
  });

  it("flattens a recursive panelTabs tree to its leaves, depth-first", () => {
    const tabs = [
      { id: "framework.panel.artifact", label: "Artifact", group: "workbench", bodyKey: "doc" },
      {
        id: "framework.panel.catalogue",
        label: "Catalogue",
        group: "workbench",
        children: [
          { id: "framework.panel.catalogue.words", label: "Words", group: "workbench", bodyKey: "words" },
          { id: "framework.panel.catalogue.headings", label: "Headings", group: "workbench", bodyKey: "headings" },
        ],
      },
    ];
    const leaves = flattenPanelTabLeaves(tabs);
    expect(leaves.map((tab) => tab.id)).toEqual(["framework.panel.artifact", "framework.panel.catalogue.words", "framework.panel.catalogue.headings"]);
    expect(leaves.every((tab) => Boolean(tab.bodyKey))).toBe(true);
  });

  it("accepts component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "draw.play.composite",
      controllerId: "draw-play",
      componentKind: "canvas-2d",
      canvas2d: {
        cameraX: 0,
        cameraY: 0,
        zoom: 1,
        layersJson: "[]",
      },
    };
    expect(node.componentKind).toBe("canvas-2d");
  });

  it("accepts graph-timeline component scene nodes", () => {
    const node = {
      type: "componentScene",
      surfaceId: "vcs.play.history",
      controllerId: "vcs-play",
      componentKind: "graph-timeline",
      graphTimeline: {
        columnsJson: "[]",
      },
    };
    expect(node.componentKind).toBe("graph-timeline");
  });
});

describe("owned declarative controls", () => {
  it("renders and dispatches a panel input through the Interpreter export", () => {
    const onAction = vi.fn();
    const view = render(renderUiControl({ type: "input", id: "name", inputKind: "text", value: "before", onChange: { controllerId: "test", action: "rename", args: { retained: true } } }, onAction, "panel.name"));
    const input = view.container.querySelector<HTMLInputElement>('input[data-ui-path="panel.name"]')!;
    expect(input.getAttribute("data-ui-path")).toBe("panel.name");
    fireEvent.change(input, { target: { value: "after" } });
    expect(onAction).toHaveBeenCalledWith({ controllerId: "test", action: "rename", args: { retained: true, value: "after" } });
    view.unmount();
  });

  it("dispatches a declarative select through the owned listbox", () => {
    const onAction = vi.fn();
    const { getByRole } = render(
      renderUiControl(
        {
          type: "select",
          id: "mode",
          value: "alpha",
          items: [
            { value: "alpha", label: "Alpha" },
            { value: "beta", label: "Beta" },
          ],
          onChange: { controllerId: "test", action: "mode", args: { retained: true } },
        },
        onAction,
        "panel.mode",
      ),
    );
    const trigger = getByRole("combobox");
    expect(trigger.textContent).toContain("Alpha");
    fireEvent.click(trigger);
    const beta = document.querySelector<HTMLElement>('[role="option"][data-value="beta"]')!;
    expect(beta.textContent).toContain("Beta");
    fireEvent.click(beta);
    expect(onAction).toHaveBeenCalledWith({ controllerId: "test", action: "mode", args: { retained: true, value: "beta" } });
  });

  it("keeps a declarative slider's numeric readout, external unit sibling, and spoken value distinct", () => {
    const law = sliderPresentationFixture.unit;
    for (const placement of law.placements) {
      const control = renderUiControl(
        {
          type: "slider",
          id: `distance.${placement.id}`,
          value: law.value,
          min: 0,
          max: 10,
          step: 0.1,
          unit: law.unit,
          onChange: { controllerId: "test", action: "distance" },
        },
        () => {},
        `panel.${placement.id}`,
      );
      const view = render(createElement("div", placement.kind === "treeControl" ? { role: "treeitem", dir: placement.inline } : { dir: placement.inline }, control));
      const thumb = view.getByRole("slider");
      const slider = view.container.querySelector<HTMLElement>('[data-slot="slider"]')!;
      const wrapper = slider.closest<HTMLElement>(".gap-single")!;
      const external = wrapper.lastElementChild as HTMLElement;
      expect(wrapper.className).toContain("gap-single");
      expect(wrapper.children).toHaveLength(2);
      expect(wrapper.firstElementChild?.contains(slider)).toBe(true);
      expect(thumb.getAttribute("aria-valuetext")).toBe(law.accessibleValueText);
      expect(view.container.querySelector('[data-slot="slider-value"]')?.textContent).toBe(law.internalReadout);
      expect(external.textContent).toBe(law.externalReadout);
      expect(external.className).toContain("text-muted-foreground");
      expect(external.className).toContain("shrink-0");
      view.unmount();
    }
  });
});

describe("framework external slots", () => {
  it("preserves explicitly owned host extensions before contributor resolution at any depth", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧩️host-slots/🔣️.json");
    
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv();
    
    const ownedIds = new Set(fixture.hostExtensionIds);
    for (const sample of fixture.cases) {
      expect(ownedIds.has(sample.id)).toBe(sample.retained);
      const node: any = {
        key: sample.id, component: { type: "extension", extension: sample.id, props: sample.props }, layout: CONTRACT_LEAF_LAYOUT,
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
        accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
      };
      const errors: string[] = [];
      const context = { plugins: new Map(), contributorInstances: new Map(), contributorPasses: new Map(), viewState: { locale: "de", terminology: "native" } as any, ownerId: "app:document:window", hostExtensions: new Set(fixture.hostExtensionIds), onError: (id: string) => errors.push(id) };
      const direct = await resolveExternalSlots(node, context);
      const nested = await resolveExternalSlots({ ...node, key: "outer", component: { type: "text", value: "parent", emphasize: null, dataAttributes: null }, children: [node] }, context);
      for (const result of [direct, nested.children[0]]) {
        if (sample.retained) expect(result).toBe(node);
        else expect(result.component.type).toBe("text");
      }
      expect(errors).toEqual(sample.retained ? [] : [sample.id, sample.id]);
      expect(context.contributorInstances.size).toBe(0);
    }
  });

  it("admits isolated extension render inputs against the neutral schema", async () => {
    const { default: fixture } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🧫️fixtures/🧩️extension-input/🔣️.json");
    const { default: schema } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    for (const row of fixture.cases) {
      expect(validate(row.context), JSON.stringify(validate.errors)).toBe(true);
      expect(parseResolvedPluginViewState(row.context)).toEqual(row.context);
    }
    for (const length of [fixture.capacityChars, fixture.capacityChars + 1]) {
      const context = { locale: "en", terminology: "native", extensionInputJson: "x".repeat(length) };
      expect(validate(context)).toBe(length === fixture.capacityChars);
      if (length === fixture.capacityChars) expect(parseResolvedPluginViewState(context)).toEqual(context);
      else expect(() => parseResolvedPluginViewState(context)).toThrow();
    }
  });

  it("renders canonical contributor bodies with isolated instances and current inputs", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧩️external-slots/🔣️.json");
    const created: string[] = [];
    const destroyed: number[] = [];
    const requests: { instanceId: number; request: any }[] = [];
    const node = (key: string, component: any): any => ({
      key, component, layout: CONTRACT_LEAF_LAYOUT,
      style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
      activity: "idle", disabled: false,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [], menu: null, children: [],
    });
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async (appId: string) => { created.push(appId); await Promise.resolve(); return created.length; },
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async (instanceId: number, request: any) => {
        requests.push({ instanceId, request });
        const props = JSON.parse(request.viewState.extensionInputJson);
        return { windows: [{ key: law.windowId, hash: "rendered", value: node("parameters", { type: "text", value: props.paramsJson, emphasize: null, dataAttributes: null }) }] };
      },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const slots = law.slots.map((slot) => node(slot.key, { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: slot.props }));
    const first = await resolveExternalSlots(slots[0], { ...context, ownerId: `${law.ownerId}:a` });
    const second = await resolveExternalSlots(slots[1], { ...context, ownerId: `${law.ownerId}:b` });
    expect(created).toEqual([law.appId, law.appId]);
    expect(first.children[0].component).toMatchObject({ type: "text", value: law.slots[0].props.paramsJson });
    expect(second.children[0].component).toMatchObject({ type: "text", value: law.slots[1].props.paramsJson });
    expect(first.key).toBe(slots[0].key);
    expect(requests.map(({ instanceId }) => instanceId)).toEqual([1, 2]);
    for (const { request } of requests) {
      expect(request.windows).toEqual([{ key: law.windowId, bodyKey: law.bodyKey }]);
      expect(request.viewState.windowInstances).toEqual([{ id: law.windowId, windowKindId: law.windowKindId }]);
      expect(request.viewState.locale).toBe(law.viewState.locale);
      expect(request.viewState.activeModeId).toBe(law.modeId);
      expect(parseResolvedPluginViewState(request.viewState)).toEqual(request.viewState);
    }
    const changed = { ...slots[0], component: { ...slots[0].component, props: law.slots[1].props } };
    const current = await resolveExternalSlots(changed, { ...context, ownerId: `${law.ownerId}:a` });
    expect(created).toHaveLength(2);
    expect(requests.at(-1)?.instanceId).toBe(1);
    expect(current.children[0].component).toMatchObject({ type: "text", value: law.slots[1].props.paramsJson });
    await resolveExternalSlots(node("empty", { type: "text", value: "Empty", emphasize: null, dataAttributes: null }), { ...context, ownerId: `${law.ownerId}:a` });
    expect(destroyed).toEqual([1]);
    expect(context.contributorInstances.size).toBe(1);
  });

  it("shares pending creations and retires the exact detached contributor", async () => {
    const { ensureContributorInstance, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (id: number) => void;
    const destroyed: number[] = [];
    let created = 0;
    const handle = {
      manifest: { apps: [] },
      createApp: async () => { created++; return new Promise<number>((resolve) => { release = resolve; }); },
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => ({}),
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const first = ensureContributorInstance(law.pluginId, law.appId, [law.slots[0].key], context)!;
    const second = ensureContributorInstance(law.pluginId, law.appId, [law.slots[0].key], context)!;
    expect(first).toBe(second);
    await Promise.resolve();
    expect(created).toBe(1);
    const retirement = retireContributorInstances(context.contributorInstances);
    expect(context.contributorInstances.size).toBe(0);
    release(17);
    await retirement;
    expect(destroyed).toEqual([17]);
  });

  it("does not render or recreate an extension after its parent retires during creation", async () => {
    const { resolveExternalSlots, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (id: number) => void;
    let refreshes = 0;
    const destroyed: number[] = [];
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async () => new Promise<number>((resolve) => { release = resolve; }),
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => { refreshes++; return {}; },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId };
    const slot: any = { key: law.slots[0].key, component: { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: law.slots[0].props }, children: [] };
    const rendering = resolveExternalSlots(slot, context);
    await Promise.resolve();
    const retiring = retireContributorInstances(context.contributorInstances);
    release(23);
    await Promise.all([rendering, retiring]);
    expect(refreshes).toBe(0);
    expect(destroyed).toEqual([23]);
    expect(context.contributorInstances.size).toBe(0);
  });

  it("discards effects from a refresh that finishes after retirement", async () => {
    const { resolveExternalSlots, retireContributorInstances } = await import("@semio-tech/framework");
    const { default: law } = await import("../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧩️external-slots/🔣️.json");
    let release!: (response: any) => void;
    let started!: () => void;
    const refreshing = new Promise<void>((resolve) => { started = resolve; });
    const effects: unknown[] = [];
    const destroyed: number[] = [];
    const handle = {
      manifest: { apps: [{ id: law.appId, controllerId: law.controllerId, defaultModeId: law.modeId, modes: [{ id: law.modeId }], windowKinds: [{ id: law.windowKindId, bodyKey: law.bodyKey }] }] },
      createApp: async () => 24,
      destroyApp: async (id: number) => { destroyed.push(id); },
      refreshUi: async () => { started(); return new Promise<any>((resolve) => { release = resolve; }); },
    };
    const context = { plugins: new Map([[law.pluginId, handle]]), contributorInstances: new Map(), contributorPasses: new Map(), viewState: law.viewState, ownerId: law.ownerId, onEffects: (...args: unknown[]) => { effects.push(args); } };
    const slot: any = { key: law.slots[0].key, component: { type: "extension", extension: `${law.pluginId}/${law.appId}`, props: law.slots[0].props }, children: [] };
    const rendering = resolveExternalSlots(slot, context);
    await refreshing;
    const retiring = retireContributorInstances(context.contributorInstances);
    release({ requestedEffects: [{ type: "diagnostic", message: "late extension work" }], windows: [] });
    await Promise.all([rendering, retiring]);
    expect(effects).toEqual([]);
    expect(destroyed).toEqual([24]);
  });

  it("renders external slot fallback text when unresolved", () => {
    // 🧬️ MIGRATION: `Component::Extension` (the old `ExternalSlot`) collapses `pluginId`/`appId`/
    // `bodyKey` into one opaque `extension` address string — see `ExtensionProps`'s own doc.
    const markup = renderContractTree({ key: "missing-module", component: { type: "extension", extension: "missing-module", props: {} } });
    expect(markup).toContain("Extension unavailable: missing-module");
  });
});

describe("declarative forms parity", () => {
  it("renders declarative text with appearance-aware foreground", () => {
    const markup = renderContractTree({ key: "text", component: { type: "text", value: "Hello flow", emphasize: null, dataAttributes: null } });
    expect(markup).toContain("text-foreground");
    expect(markup).toContain("Hello flow");
    const emphasized = renderContractTree({ key: "text", component: { type: "text", value: "Emphasized", emphasize: true, dataAttributes: null } });
    expect(emphasized).toContain("text-foreground");
    expect(emphasized).toContain("font-semibold");
  });

  it("dag overlay label fills resolve to Canvas2D-safe hex for appearance", () => {
    const chrome = { selectedIds: new Set<string>(["sel"]), highlightedIds: new Set<string>(["hi"]) };
    expect(dagOverlayLabelFill("plain", false, null, chrome)).toBe("var(--color-muted-foreground)");
    expect(dagOverlayLabelFill("sel", false, null, chrome)).toBe("var(--color-foreground)");
    const muted = dagOverlayLabelFillHex("plain", false, null, chrome);
    const selected = dagOverlayLabelFillHex("sel", false, null, chrome);
    const highlighted = dagOverlayLabelFillHex("hi", false, null, chrome);
    const hovered = dagOverlayLabelFillHex("plain", false, "plain", chrome);
    const ghost = dagOverlayLabelFillHex("ghost", true, null, chrome);
    const dimmed = dagOverlayLabelFillHex("plain", false, null, chrome, ["plain"]);
    for (const hex of [muted, selected, highlighted, hovered, ghost, dimmed]) {
      expect(hex).toMatch(/^#[0-9a-f]{6}$/iu);
      expect(hex).not.toBe("#000000");
    }
    expect(selected).toBe(hovered);
    expect(highlighted).toBe(ghost);
  });

  it("renders field description, required marker and inline error", () => {
    // 🧬️ MIGRATION: the old `field`/`input` `UiNode` pair collapses into one `Component::Container`
    // (`role: "field"`) whose single child IS the input — `ContainerProps`'s own doc.
    const markup = renderContractTree({
      key: "forms-try.name",
      component: { type: "container", role: "field", label: "Name", description: "Your full name", required: true, error: "Name is required", defaultOpen: null, dropOverlay: null },
      children: [{ key: "forms-try.name.input", component: { type: "input", kind: "text", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: null, precision: null, snaps: [] } }],
    });
    expect(markup).toContain("Your full name");
    expect(markup).toContain("Name is required");
    expect(markup).toContain("*");
    expect(markup).toContain('data-slot="field-error"');
  });

  it("renders slider unit readout", () => {
    const markup = renderContractTree({ key: "forms-try.volume.slider", component: { type: "slider", value: 60, min: 0, max: 100, step: 5, unit: "%", snaps: [] } });
    expect(markup).toContain("60 %");
  });

  it("renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 3, step: 1, uniform: true, min: null, max: null, precision: null } });
    expect(markup).toContain('data-slot="stepper-group"');
    expect(markup).toContain('data-slot="stepper-minus"');
    expect(markup).toContain('data-slot="stepper-plus"');
    expect(markup).not.toContain("border-border");
  });

  it("shows the mixed-values placeholder on a non-uniform numberStepper", () => {
    const markup = renderContractTree({ key: "forms-try.height.stepper", component: { type: "numberStepper", value: 0, step: 1, uniform: false, min: null, max: null, precision: null } });
    expect(markup).toContain('data-mixed="true"');
  });

  it("renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers)", () => {
    // 🧬️ MIGRATION: `group`/`field` both collapse into `Component::Container` (`role: "group"` /
    // `role: "field"`) — the old `child: Box<UiNode>` singular is simply `children[0]` on the record.
    const markup = renderContractTree({
      key: "puzzle3d-play-inspector.object.origin",
      component: { type: "container", role: "group", label: "Origin", description: null, required: null, error: null, defaultOpen: true, dropOverlay: null },
      children: [
        {
          key: "puzzle3d-play-inspector.object.origin.x",
          component: { type: "container", role: "field", label: "X", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.x.stepper", component: { type: "numberStepper", value: 1, step: 0.1, uniform: true, min: null, max: null, precision: null } }],
        },
        {
          key: "puzzle3d-play-inspector.object.origin.y",
          component: { type: "container", role: "field", label: "Y", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
          children: [{ key: "puzzle3d-play-inspector.object.origin.y.stepper", component: { type: "numberStepper", value: 2, step: 0.1, uniform: true, min: null, max: null, precision: null } }],
        },
      ],
    });
    expect(markup).toContain(">Origin</h2>");
    expect(markup).toContain(">X</label>");
    expect(markup).toContain(">Y</label>");
    expect(markup).toContain('data-slot="stepper-group"');
  });

  // 🧬️ MIGRATION: `LayoutSpec`'s stack `gap`/`padding` are now a closed `SpaceToken` enum resolved to
  // inline CSS via `spaceTokenRem` (Interpreter's own `layoutSpecStyle`/`LayoutAndStyle` region) —
  // the new architecture's real replacement for "never a hardcoded raw rem" is a renderer-neutral
  // token resolved to CSS at read time, not the pre-migration Tailwind-gap-class scheme this test
  // used to assert (`not.toContain("style=")` is no longer true BY DESIGN, not a regression — see
  // `react-renderer` packet's own decisions doc). Rewritten to assert the token resolves through that
  // closed scale (never an arbitrary raw number), while separators still avoid the raw
  // `border-border` utility class.
  it("resolves stack gap/padding through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border", () => {
    const markup = renderContractTree({
      key: "forms-blueprint.section.q1",
      component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: { kind: "stack", axis: "vertical", gap: "xs", padding: { all: "none" }, align: "start", justify: "start", grow: false, wrap: false },
      children: [
        { key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } },
        { key: "sep", component: { type: "separator" } },
      ],
    });
    expect(markup).toMatch(/gap:\s*calc\(1 \* var\(--ui-spacing\)\)/);
    expect(markup).not.toContain("border-border");
  });

  it("passes number bounds and file accept to inputs", () => {
    const numberMarkup = renderContractTree({ key: "forms-try.age.input", component: { type: "input", kind: "number", value: "28", placeholder: null, commit: null, min: 13, max: 120, step: 1, accept: null, precision: null, snaps: [] } });
    expect(numberMarkup).toContain('min="13"');
    expect(numberMarkup).toContain('max="120"');
    const fileMarkup = renderContractTree({ key: "forms-try.resume.input", component: { type: "input", kind: "file", value: "", placeholder: null, commit: null, min: null, max: null, step: null, accept: ".pdf,.doc", precision: null, snaps: [] } });
    expect(fileMarkup).toContain('accept=".pdf,.doc"');
  });

  it("disables gated wizard buttons", () => {
    // 🧬️ MIGRATION: `disabled` moved off the component (`ButtonProps` no longer carries it) onto the
    // record itself (`record.disabled` — `ButtonView`'s own `disabled={record.disabled}`); `action`
    // moved to the record's `bindings`, keyed by `Trigger::Activate`.
    const markup = renderContractTree({
      key: "forms-try.next",
      component: { type: "button", icon: "chevron-right", label: "Next" },
      disabled: true,
      bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "nextStep", version: 1 }, args: null, capability: null }],
    });
    expect(markup).toContain("disabled");
  });

  it("renders selectable builder cards with selection ring", () => {
    // 🧬️ MIGRATION: `selected` is no longer a document field — presence (hover/selection) is a
    // separate `UiPresenceOverlayContext` channel keyed by `UiNodeRecord.key`, fed from
    // `PresenceUpdate` wire messages, never part of the retained document (`PresenceOverlay`
    // region's own doc: "presence changes at input frequency and must not touch a document
    // revision"). `data-ui-path` (a tree-position string) is gone too — the record's own stable
    // `data-ui-node-id` is the only per-node DOM handle now.
    const markup = renderContractTree(
      {
        key: "forms-blueprint.card.q1",
        component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
        bindings: [{ trigger: "activate", action: { scope: "forms-play", name: "setSelection", version: 1 }, args: null, capability: null }],
        children: [{ key: "text", component: { type: "text", value: "text · q1", emphasize: null, dataAttributes: null } }],
      },
      { "forms-blueprint.card.q1": { selected: true } },
    );
    expect(markup).toContain('data-ui-node-id="0"');
    expect(markup).toContain('role="button"');
    expect(markup).toContain("ring-primary");
  });

  it("applies separate presence fixtures without replacing the retained document", () => {
    // 🧬️ One owner for this fixture: `framework.ui.contract`'s `ContractFixture` export (ticket
    // 26/09/08 `📋️cross-partition-requests.md` row 145). `framework.ui` no longer restates it as
    // `PresenceOverlayFixture`; this consumer compiles the owning scope's export by its `$id`.
    
    
    
    
    const store = new UiDocumentStore("document");
    store.loadSnapshot(buildContractSnapshot({ key: "item:根,1", component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null } }));
    const before = store.getState();
    const context: UiInterpreterContext = { store, onAction: noopAction, onIntent: () => {} };
    const mounted = render(createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map() } }, interpretUiNode(store, context)));
    for (const row of presenceOverlayFixture.cases) {
      const own: UiPresenceOverlayEntry = JSON.parse(JSON.stringify(row.update.own));
      mounted.rerender(createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map([[row.update.nodeKey, own]]) } }, interpretUiNode(store, context)));
      expect(mounted.container.innerHTML.includes("ring-primary")).toBe(row.expected.selected);
      expect(mounted.container.innerHTML.includes("outline-primary/50")).toBe(row.expected.hovered);
      expect(own.previewed ?? false).toBe(row.expected.previewed);
      expect(store.getState()).toBe(before);
      expect(store.getRevisionSnapshot()).toBe(before.revision);
    }
  });

  it("renders image nodes from url sources", () => {
    const markup = renderContractTree({ key: "forms-try.avatar.image", component: { type: "image", src: "https://example.com/avatar.png", alt: "Avatar" } });
    expect(markup).toContain('src="https://example.com/avatar.png"');
    expect(markup).toContain('alt="Avatar"');
  });

  // 🪦️ MIGRATION, deleted (not rewritten): `declarativeTreeDragController` — a standalone pure
  // function taking a whole tree `UiNode` + a dispatch callback and returning a
  // `TreeDragAndDropController` — was deliberately not ported forward (react-renderer packet's own
  // decisions doc; the barrel's in-file migration comment says so too). Drag/drop for a `tree`
  // component is now wired INSIDE `🟦️Interpreter`'s own `TreeView` (built from the record's own `drop`
  // `ActionBinding`, dispatched through `dispatchTrigger`/`emitIntent`), not a separately-importable
  // factory this file's OWNS can call in isolation — there is no equivalent unit boundary left.
  // See this packet's report for a production-bug flag this deletion surfaced: `TreeView`'s current
  // `handleDrop` (`🟦️Interpreter/🟦️.tsx`) calls `dispatchTrigger(context, record, "drop")`
  // with NO input payload at all, discarding the drop event's target/payload/position entirely —
  // this test's old assertion (`args: { kind, targetId, dropPosition }`) has no successor to assert
  // against today.
});
