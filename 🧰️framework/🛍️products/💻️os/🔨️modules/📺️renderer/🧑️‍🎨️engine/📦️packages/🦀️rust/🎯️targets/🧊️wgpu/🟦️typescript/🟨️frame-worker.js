var __defProp = Object.defineProperty;
var __returnValue = (v) => v;
function __exportSetter(name, newValue) {
  this[name] = __returnValue.bind(null, newValue);
}
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, {
      get: all[name],
      enumerable: true,
      configurable: true,
      set: __exportSetter.bind(all, name)
    });
};
var __esm = (fn, res) => () => (fn && (res = fn(fn = 0)), res);
var __require = /* @__PURE__ */ ((x) => typeof require !== "undefined" ? require : typeof Proxy !== "undefined" ? new Proxy(x, {
  get: (a, b) => (typeof require !== "undefined" ? require : a)[b]
}) : x)(function(x) {
  if (typeof require !== "undefined")
    return require.apply(this, arguments);
  throw Error('Dynamic require of "' + x + '" is not supported');
});

/* ../../../../../../../../../🔨️modules/🛂️manifest/🤖️generated/🟦️ui-axes.ts */
var SHELL_LOCALES, SHELL_TERMINOLOGIES, isShellLocale = (value) => SHELL_LOCALES.includes(value), isShellTerminology = (value) => SHELL_TERMINOLOGIES.includes(value);
var init__ui_axes = __esm(() => {
  SHELL_LOCALES = ["en", "de"];
  SHELL_TERMINOLOGIES = ["native", "reuse"];
});

/* ../../../../../../../../../🔨️modules/🛂️manifest/🟦️component.ts */
function argControl(def) {
  const schema = def.schema;
  if (!schema)
    return { kind: "text", placeholder: undefined };
  switch (schema.kind) {
    case "string": {
      if (schema.options && schema.options.length > 0) {
        return { kind: "select", options: schema.options };
      }
      const format = schema.format;
      if (format?.kind === "iconId") {
        return { kind: "iconSelect", classifierKind: "icon" };
      }
      if (format?.kind === "artifactKind") {
        return { kind: "artifactKind", roles: format.roles };
      }
      if (format?.kind === "surfaceApp") {
        return { kind: "surfaceApp", roles: format.roles, dialectArg: format.dialectArg };
      }
      return { kind: "text", placeholder: undefined };
    }
    case "number": {
      if (def.presentation?.kind === "slider" || schema.min !== undefined && schema.max !== undefined) {
        return { kind: "slider", min: schema.min ?? 0, max: schema.max ?? 0, step: schema.step, unit: schema.unit };
      }
      return { kind: "number", min: schema.min, max: schema.max, step: schema.step };
    }
    case "boolean":
      return { kind: "toggle" };
    case "vec3":
      return { kind: "vec3" };
    case "array":
    case "object":
    case "any":
    default:
      return { kind: "text", placeholder: undefined };
  }
}
function normalizeAppLabelsOverlay(raw) {
  return {
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
    actionLabels: raw?.actionLabels ?? {},
    utilityLabels: raw?.utilityLabels ?? {},
    exampleLabels: raw?.exampleLabels ?? {},
    actionArgLabels: raw?.actionArgLabels ?? {},
    dialogLabels: raw?.dialogLabels ?? {},
    introductionLabels: raw?.introductionLabels ?? {},
    groupLabels: raw?.groupLabels ?? {}
  };
}
function encodeArtifactKindChoice(choice) {
  return JSON.stringify({
    kindId: choice.kindId,
    schema: choice.schema,
    dialect: choice.dialect,
    label: { en: choice.label.en, de: choice.label.de }
  });
}
function decodeArtifactKindChoice(value) {
  const json = JSON.parse(value);
  if (typeof json.kindId !== "string")
    throw new Error("artifact kind choice missing string field kindId");
  if (typeof json.schema !== "string")
    throw new Error("artifact kind choice missing string field schema");
  const dialect = json.dialect;
  if (typeof dialect?.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") {
    throw new Error("artifact kind choice missing field dialect");
  }
  const label = json.label;
  if (typeof label?.en !== "string")
    throw new Error("artifact kind choice missing string field label.en");
  if (typeof label.de !== "string")
    throw new Error("artifact kind choice missing string field label.de");
  return { kindId: json.kindId, schema: json.schema, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, label: { en: label.en, de: label.de } };
}
function encodeSurfaceAppChoice(choice) {
  return JSON.stringify({ pluginId: choice.app.pluginId, appId: choice.app.appId, role: choice.role });
}
function decodeSurfaceAppChoice(value) {
  const json = JSON.parse(value);
  if (typeof json.pluginId !== "string")
    throw new Error("surface app choice missing string field pluginId");
  if (typeof json.appId !== "string")
    throw new Error("surface app choice missing string field appId");
  if (json.role !== "editor" && json.role !== "viewer")
    throw new Error("surface app choice missing string field role");
  return { app: { pluginId: json.pluginId, appId: json.appId }, role: json.role };
}
function resolveNativeLabel(label) {
  const native = label?.native;
  return { en: native?.en ?? "", de: native?.de ?? "" };
}
function artifactKindChoices(manifests, roles) {
  const byCoordinate = new Map;
  for (const manifest of manifests) {
    for (const raw of manifest.apps) {
      const app = raw;
      if (!roles.includes(app.role) || app.io.documentSchema === "")
        continue;
      const coordinate = `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`;
      if (byCoordinate.has(coordinate))
        continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.documentSchema, dialect: app.dialect, label: resolveNativeLabel(app.label) });
    }
  }
  return [...byCoordinate.keys()].sort().map((coordinate) => byCoordinate.get(coordinate));
}
function panelTabKindId(kind) {
  switch (kind.kind) {
    case "workbenchCategory":
      return "framework.category.workbench";
    case "displayCategory":
      return "framework.category.display";
    case "detailsCategory":
      return "framework.category.details";
    case "settingsCategory":
      return "framework.category.settings";
    case "displayWindows":
      return "framework.display.windows";
    case "displayLayout":
      return "framework.display.layout";
    case "settingsGeneral":
      return "framework.settings.general";
    case "settingsTheme":
      return "framework.settings.theme";
    case "settingsDefaultApps":
      return "framework.settings.defaultApps";
    case "app":
      return kind.id;
  }
}
var CANVAS_HOVER_SOURCE_CANVAS = "canvas", CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu", CANVAS_HOVER_SOURCE_CATALOG = "catalog", CANVAS_HOVER_SOURCE_ARTIFACT = "document", FRAMEWORK_PANEL_TAB_ARTIFACT_ID = "framework.panel.artifact", FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue", FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection", FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL = "Artifact", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection", FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID = "framework.panel.artifact", FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue", FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection", FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters", FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters", FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters", FRAMEWORK_PANEL_TAB_HISTORY_ID = "framework.panel.history", FRAMEWORK_PANEL_TAB_HISTORY_LABEL = "History", FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID = "framework.panel.history", UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed", SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility", SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool", INTERACTION_SELECT_ACTION_ID = "interactionSelect", INTERACTION_HOVER_ACTION_ID = "interactionHover", CLEAR_SELECTION_ACTION_ID = "clearSelection", SELECT_ALL_ACTION_ID = "selectAll", SET_SELECTION_MODE_ACTION_ID = "setSelectionMode", SET_INTERACTION_GRANULARITY_ACTION_ID = "setInteractionGranularity", START_INTRODUCTION_ACTION_ID = "startIntroduction", START_TUTORIAL_ACTION_ID = "startTutorial", RECORD_TUTORIAL_ACTION_ID = "recordTutorial", TUTORIAL_CONVERGE_MS = 600, HISTORY_ACTION_IDS, EMPTY_APP_LABELS_OVERLAY;
var init__component = __esm(() => {
  init__ui_axes();
  HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];
  EMPTY_APP_LABELS_OVERLAY = {
    windowKindLabels: {},
    panelTabLabels: {},
    modeLabels: {},
    actionLabels: {},
    utilityLabels: {},
    exampleLabels: {},
    actionArgLabels: {},
    dialogLabels: {},
    introductionLabels: {},
    groupLabels: {}
  };
  if (import.meta.vitest) {
    const { describe, expect, it } = import.meta.vitest;
    describe("\uD83D\uDD16️HostResolvedArgs", () => {
      const PINNED_ARTIFACT_KIND_CHOICE = {
        kindId: "s.draw.draw",
        schema: "draw.document",
        dialect: { artifactKind: "s.draw.draw", standard: "1", subset: "*" },
        label: { en: "Draw", de: "Zeichnung" }
      };
      const PINNED_ARTIFACT_KIND_CHOICE_JSON = '{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":"s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}';
      it("encodeArtifactKindChoice matches the contract's pinned byte-identical fixture", () => {
        expect(encodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE)).toBe(PINNED_ARTIFACT_KIND_CHOICE_JSON);
      });
      it("decodeArtifactKindChoice inverts the pinned fixture", () => {
        expect(decodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE_JSON)).toEqual(PINNED_ARTIFACT_KIND_CHOICE);
      });
      it("decodeArtifactKindChoice throws naming the missing field", () => {
        expect(() => decodeArtifactKindChoice("{}")).toThrow(/kindId/);
      });
      it("encodeSurfaceAppChoice / decodeSurfaceAppChoice round-trip the frozen shape", () => {
        const choice = { app: { pluginId: "draw", appId: "s.draw.draw@1/*#editor" }, role: "editor" };
        const json = encodeSurfaceAppChoice(choice);
        expect(json).toBe('{"pluginId":"draw","appId":"s.draw.draw@1/*#editor","role":"editor"}');
        expect(decodeSurfaceAppChoice(json)).toEqual(choice);
      });
      it("decodeSurfaceAppChoice throws on an invalid role", () => {
        expect(() => decodeSurfaceAppChoice('{"pluginId":"draw","appId":"a","role":"bogus"}')).toThrow(/role/);
      });
      function fakeManifest(pluginId, apps) {
        return {
          pluginId,
          label: pluginId,
          version: "1.0.0",
          apps: apps.map((app) => ({ role: app.role, dialect: app.dialect, label: { native: app.label ?? { en: app.dialect.artifactKind, de: app.dialect.artifactKind } }, io: { documentSchema: app.documentSchema } })),
          workflows: [],
          examples: []
        };
      }
      it("artifactKindChoices dedupes by dialect coordinate (owner manifest first wins), sorts, and filters by role", () => {
        const drawDialect = { artifactKind: "s.draw.draw", standard: "1", subset: "*" };
        const dagDialect = { artifactKind: "s.dag.dag", standard: "1", subset: "*" };
        const manifests = [
          fakeManifest("draw", [
            { role: "editor", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw", de: "Zeichnung" } },
            { role: "editor", dialect: { artifactKind: "s.draw.empty", standard: "1", subset: "*" }, documentSchema: "" }
          ]),
          fakeManifest("draw-contrib", [{ role: "viewer", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw (fallback)", de: "Zeichnung (fallback)" } }]),
          fakeManifest("dag", [{ role: "editor", dialect: dagDialect, documentSchema: "dag.document", label: { en: "DAG", de: "DAG" } }])
        ];
        const editorOnly = artifactKindChoices(manifests, ["editor"]);
        expect(editorOnly.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
        expect(editorOnly.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });
        const editorAndViewer = artifactKindChoices(manifests, ["editor", "viewer"]);
        expect(editorAndViewer.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
        expect(editorAndViewer.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });
        expect(artifactKindChoices(manifests, ["viewer"]).map((choice) => choice.kindId)).toEqual(["s.draw.draw"]);
      });
    });
  }
});

/* ../../../../../../../../../🔨️modules/🎯️action-bus/🟦️component.ts */
function deriveUtilityNodes(controllerId, utilities, activeUtilityId) {
  const toggle = (utility) => ({
    id: utility.id,
    kind: "toggle",
    iconId: utility.iconId,
    label: utility.label,
    title: utility.label,
    pressed: activeUtilityId === utility.id,
    category: utility.category,
    onChange: { controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { utilityId: utility.id } }
  });
  const nodes = [];
  const groupIndex = new Map;
  for (const utility of utilities) {
    const node = toggle(utility);
    if (utility.group === undefined) {
      nodes.push(node);
      continue;
    }
    const existing = groupIndex.get(utility.group);
    if (existing !== undefined) {
      const collection = nodes[existing];
      collection.children.push(node);
    } else {
      groupIndex.set(utility.group, nodes.length);
      const groupLabel = utility.groupLabel ?? utility.group;
      nodes.push({ id: `group:${utility.group}`, kind: "collection", iconId: utility.iconId, label: groupLabel, title: groupLabel, category: utility.category, children: [node] });
    }
  }
  return nodes.map((node) => node.kind === "collection" && node.children.length === 1 ? node.children[0] : node);
}
function partitionWindowMeasures(measures, activeUtilityId) {
  const general = [];
  const utilityOptions = [];
  for (const measure of measures) {
    if (measure.kind === "group" && measure.activeUtilityId !== undefined) {
      if (measure.activeUtilityId === activeUtilityId)
        utilityOptions.push(...measure.children);
      continue;
    }
    general.push(measure);
  }
  return { general, utilityOptions };
}
function resolveWindowActions(_app, windowKind) {
  const resolved = [];
  const seen = new Set;
  for (const action of windowKind.actions ?? []) {
    if (action && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  return resolved;
}
function resolveModeTools(app, activeModeId) {
  const tools = app?.tools ?? [];
  const mode = app?.modes.find((entry) => entry.id === activeModeId);
  if (!mode)
    return [];
  const resolved = [];
  const seen = new Set;
  for (const ref of mode.tools ?? []) {
    const tool = tools.find((entry) => entry.id === ref);
    if (tool && !seen.has(tool.id)) {
      seen.add(tool.id);
      resolved.push(tool);
    }
  }
  return resolved;
}
var init__component2 = __esm(() => {
  init__component();
});

/* ../../../../../../../../../🔨️modules/🧮️action-argument-resolution/🟦️component.ts */
function effectiveActionArgs(defs, staged, seed) {
  if (defs.length === 0)
    return { ...seed, ...staged };
  const effective = seed ? { ...seed } : {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (!Object.prototype.hasOwnProperty.call(effective, def.id) && def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}
function missingRequiredArgs(defs, effective) {
  return defs.filter((def) => def.required).filter((def) => {
    const value = effective[def.id];
    return value === undefined || value === null || value === "";
  }).map((def) => def.id);
}

/* ../../../../../../../../../🔨️modules/🧬️schema/🟦️component.ts */
class ArtifactSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
}

class ArtifactInferenceRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
}

class AppSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
  get isEmpty() {
    return this.#byId.size === 0;
  }
}
var GRAPHQL_STATE_PREAMBLE, STATE_CLASSES, JSON_SCHEMA_DERIVED_KEY = "x-semio-derived", GRAPHQL_COMPOSITION_PREAMBLE;
var init__component3 = __esm(() => {
  GRAPHQL_STATE_PREAMBLE = `enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }
` + `directive @state(class: StateClass!) on FIELD_DEFINITION
` + "directive @derived on FIELD_DEFINITION";
  STATE_CLASSES = ["artifact", "config", "presence", "transient"];
  GRAPHQL_COMPOSITION_PREAMBLE = `type ArtifactLink { targetId: String! kind: String! }
` + `directive @child(kind: String!) on FIELD_DEFINITION
` + "directive @link(roles: [String!]) on FIELD_DEFINITION";
});

/* ../../../../../../../../../🔨️modules/🖥️platform/🟦️component.ts */
function elementIdSegment(raw) {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch))
      continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}
function windowElementId(kindId) {
  return `framework.window.${elementIdSegment(kindId)}`;
}
function panelTabElementId(tabId) {
  return `framework.panelTab.${tabId}`;
}
function panelTabFirstDraggableElementId(tabId) {
  return `framework.panelTab.${tabId}.firstDraggable`;
}
function resolveUiPresence(presence) {
  return presence ?? DEFAULT_UI_PRESENCE;
}
function uiPresenceShowsSkeleton(presence) {
  const status = resolveUiPresence(presence).status;
  return status === "loading" || status === "waiting";
}
function windowMeasureChromeStatus(measure) {
  if (measure.loading)
    return "loading";
  if (measure.waiting)
    return "waiting";
  return "idle";
}
function builtNode(key, component, children = []) {
  return {
    key,
    component,
    layout: DEFAULT_BUILT_LAYOUT,
    style: DEFAULT_BUILT_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: DEFAULT_BUILT_ACCESSIBILITY,
    bindings: [],
    menu: null,
    children: [...children]
  };
}
function pendingWindowUiNode() {
  return { ...builtNode("pending", { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }), activity: "loading" };
}
function pendingPanelUiNode() {
  return { ...builtNode("pending", { type: "tree", interactionDomain: null }), activity: "loading" };
}
function canvasPickTargetKey(target) {
  return `${target.domain}:${target.id}`;
}
function parseCanvasPickTargetKey(key) {
  const colon = key.indexOf(":");
  if (colon < 0)
    return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}
function sortCanvasPickTargetsGeneralFirst(targets) {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}
function pickMostSpecificCanvasTarget(targets) {
  if (targets.length === 0)
    return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}
function canvasHoverFocusFromTarget(sourceId, target) {
  return { sourceId, target };
}
function createWindowLayout(windowKindId, title, options) {
  return {
    kind: "window",
    windowKindId,
    ...title ? { title } : {},
    ...options?.instanceId ? { instanceId: options.instanceId } : {},
    ...options?.templateId ? { templateId: options.templateId } : {}
  };
}
function createStackLayout(windowKindIds, titles) {
  return {
    root: {
      kind: "stack",
      children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index]))
    }
  };
}
function createDefaultLayout(windowIds, direction = "row", sizes, titles) {
  return {
    root: {
      kind: direction,
      children: windowIds.map((id, index) => ({
        kind: "stack",
        ...sizes?.[index] !== undefined ? { size: sizes[index] } : {},
        children: [createWindowLayout(id, titles?.[index] ?? id)]
      }))
    }
  };
}
function createTabStackLayout(windowIds, titles) {
  return createStackLayout(windowIds, titles);
}
function createNamedLayout(id, label, layout, origin = "builtin", iconId, groupPath) {
  return {
    id,
    label,
    layout,
    origin,
    ...iconId ? { iconId } : {},
    ...groupPath?.length ? { groupPath } : {}
  };
}
function mergeById(base, extension) {
  if (!base?.length && !extension?.length)
    return;
  const merged = new Map;
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}
function mergeNamedLayouts(base, extension) {
  return mergeById(base, extension) ?? [];
}

class Store {
  listeners = new Set;
  disposed = false;
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  notify() {
    if (this.disposed)
      return;
    for (const listener of this.listeners)
      listener();
  }
  dispose() {
    this.disposed = true;
    this.listeners.clear();
  }
}
function emptyOsShellConfig() {
  return { version: 1, preferences: {}, namedLayouts: {}, dockLayouts: { apps: {} }, dockUi: { apps: {} }, windowPanes: { apps: {} } };
}
function validDockSkeleton(value) {
  return Boolean(value) && typeof value === "object" && value.version === 3 && Boolean(value.anchors) && typeof value.anchors === "object";
}
function validDockUiState(value) {
  return Boolean(value) && typeof value === "object" && value.version === 3 && Boolean(value.anchors) && typeof value.anchors === "object";
}
function validWindowPaneUiState(value) {
  return Boolean(value) && typeof value === "object" && value.version === 1 && Boolean(value.windows) && typeof value.windows === "object";
}
function createBrowserStoragePort() {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined")
          localStorage.setItem(key, value);
      } catch {}
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined")
          localStorage.removeItem(key);
      } catch {}
    }
  };
}
function createMemoryStoragePort() {
  const map = new Map;
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    }
  };
}
function createScopedStoragePort(base, namespace) {
  const prefix = `semio.shell.${namespace}.`;
  return {
    get: (key) => base.get(`${prefix}${key}`),
    set: (key, value) => base.set(`${prefix}${key}`, value),
    remove: (key) => base.remove(`${prefix}${key}`)
  };
}
function uiInspectorAllEqual(values) {
  if (values.length <= 1)
    return true;
  const first = values[0];
  for (let index = 1;index < values.length; index += 1) {
    if (values[index] !== first)
      return false;
  }
  return true;
}
function uiInspectorMixedText(values) {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? values[0] ?? "" : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}
function uiInspectorMixedNumber(values) {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? values[0] ?? 0 : Number.NaN, uniform };
}
function uiInspectorMixedSelect(values) {
  return uiInspectorMixedText(values);
}
function uiInspectorMixedToggle(values) {
  const uniform = uiInspectorAllEqual(values);
  return { pressed: uniform ? values[0] ?? false : false, uniform };
}
function uiInspectorMixedSlider(values) {
  return uiInspectorMixedNumber(values);
}
var UI_NAVBAR_ELEMENT_ID = "ui.navbar", UI_FOOTER_ELEMENT_ID = "ui.footer", DEFAULT_UI_PRESENCE, UI_PENDING_PRESENCE, DEFAULT_BUILT_LAYOUT, DEFAULT_BUILT_STYLE, DEFAULT_BUILT_ACCESSIBILITY, OS_SHELL_CONFIG_STORAGE_KEY = "semio.os.config", OsShellConfig, NamedLayoutStore, DockLayoutStore, DockUiStateStore, WindowPaneStateStore;
var init__component4 = __esm(() => {
  init__component();
  DEFAULT_UI_PRESENCE = { state: "normal", status: "idle", hover: false, selected: false, color: null, peers: [] };
  UI_PENDING_PRESENCE = { state: "normal", status: "loading", hover: false, selected: false, color: null, peers: [] };
  DEFAULT_BUILT_LAYOUT = { kind: "leaf", width: "hug", height: "hug" };
  DEFAULT_BUILT_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  DEFAULT_BUILT_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  OsShellConfig = class OsShellConfig extends Store {
    storage;
    constructor(storage) {
      super();
      this.storage = storage;
    }
    getSnapshot() {
      const raw = this.storage.get(OS_SHELL_CONFIG_STORAGE_KEY);
      if (!raw)
        return emptyOsShellConfig();
      try {
        const parsed = JSON.parse(raw);
        if (parsed.version !== 1 || !parsed.preferences || !parsed.namedLayouts || !parsed.dockLayouts?.apps || !parsed.dockUi?.apps || !parsed.windowPanes?.apps)
          return emptyOsShellConfig();
        return parsed;
      } catch {
        return emptyOsShellConfig();
      }
    }
    update(update) {
      const next = update(this.getSnapshot());
      this.storage.set(OS_SHELL_CONFIG_STORAGE_KEY, JSON.stringify(next));
      this.notify();
    }
    getPreference(key) {
      return this.getSnapshot().preferences[key];
    }
    setPreference(key, value) {
      this.update((current) => ({ ...current, preferences: { ...current.preferences, [key]: value } }));
    }
    reset() {
      this.storage.remove(OS_SHELL_CONFIG_STORAGE_KEY);
      this.notify();
    }
  };
  NamedLayoutStore = class NamedLayoutStore extends Store {
    layouts = [];
    config;
    appId;
    constructor(appId, storage) {
      super();
      this.appId = appId;
      this.config = new OsShellConfig(storage);
      this.layouts = this.readPersisted();
    }
    getSnapshot() {
      return this.layouts;
    }
    save(layout) {
      const next = mergeNamedLayouts(this.layouts.filter((entry) => entry.id !== layout.id), [layout]);
      this.layouts = next;
      this.persist();
      this.notify();
    }
    remove(layoutId) {
      const next = this.layouts.filter((entry) => entry.id !== layoutId);
      if (next.length === this.layouts.length)
        return;
      this.layouts = next;
      this.persist();
      this.notify();
    }
    readPersisted() {
      const parsed = this.config.getSnapshot().namedLayouts[this.appId];
      if (!Array.isArray(parsed))
        return [];
      return parsed.filter((entry) => Boolean(entry) && typeof entry === "object" && typeof entry.id === "string" && typeof entry.label === "string" && entry.origin === "user" && Boolean(entry.layout));
    }
    persist() {
      this.config.update((current) => ({ ...current, namedLayouts: { ...current.namedLayouts, [this.appId]: this.layouts } }));
    }
  };
  DockLayoutStore = class DockLayoutStore extends Store {
    config;
    appId;
    constructor(storage, appId) {
      super();
      this.appId = appId;
      this.config = new OsShellConfig(storage);
    }
    getSnapshot() {
      const layouts = this.config.getSnapshot().dockLayouts;
      if (this.appId) {
        const app = layouts.apps[this.appId];
        if (validDockSkeleton(app))
          return app;
      }
      return validDockSkeleton(layouts.os) ? layouts.os : null;
    }
    save(skeleton) {
      this.updateLayer(this.appId, skeleton);
      this.notify();
    }
    saveOs(skeleton) {
      this.updateLayer(undefined, skeleton);
      this.notify();
    }
    reset() {
      this.config.update((current) => {
        const apps = { ...current.dockLayouts.apps };
        if (this.appId)
          delete apps[this.appId];
        return { ...current, dockLayouts: { apps } };
      });
      this.notify();
    }
    updateLayer(appId, skeleton) {
      this.config.update((current) => {
        const apps = { ...current.dockLayouts.apps };
        if (appId) {
          if (skeleton)
            apps[appId] = skeleton;
          else
            delete apps[appId];
          return { ...current, dockLayouts: { ...current.dockLayouts, apps } };
        }
        return { ...current, dockLayouts: skeleton ? { ...current.dockLayouts, os: skeleton } : { apps } };
      });
    }
  };
  DockUiStateStore = class DockUiStateStore extends Store {
    config;
    appId;
    constructor(storage, appId) {
      super();
      this.appId = appId;
      this.config = new OsShellConfig(storage);
    }
    getSnapshot() {
      const dockUi = this.config.getSnapshot().dockUi;
      if (this.appId) {
        const app = dockUi.apps[this.appId];
        if (validDockUiState(app))
          return app;
      }
      return validDockUiState(dockUi.os) ? dockUi.os : null;
    }
    save(state) {
      this.updateLayer(this.appId, state);
      this.notify();
    }
    saveOs(state) {
      this.updateLayer(undefined, state);
      this.notify();
    }
    reset() {
      this.config.update((current) => {
        const apps = { ...current.dockUi.apps };
        if (this.appId)
          delete apps[this.appId];
        return { ...current, dockUi: { apps } };
      });
      this.notify();
    }
    updateLayer(appId, state) {
      this.config.update((current) => {
        const apps = { ...current.dockUi.apps };
        if (appId) {
          if (state)
            apps[appId] = state;
          else
            delete apps[appId];
          return { ...current, dockUi: { ...current.dockUi, apps } };
        }
        return { ...current, dockUi: state ? { ...current.dockUi, os: state } : { apps } };
      });
    }
  };
  WindowPaneStateStore = class WindowPaneStateStore extends Store {
    config;
    appId;
    constructor(storage, appId) {
      super();
      this.appId = appId;
      this.config = new OsShellConfig(storage);
    }
    getSnapshot() {
      const panes = this.config.getSnapshot().windowPanes;
      if (this.appId) {
        const app = panes.apps[this.appId];
        if (validWindowPaneUiState(app))
          return app;
      }
      return validWindowPaneUiState(panes.os) ? panes.os : null;
    }
    save(state) {
      this.updateLayer(this.appId, state);
      this.notify();
    }
    saveOs(state) {
      this.updateLayer(undefined, state);
      this.notify();
    }
    reset() {
      this.config.update((current) => {
        const apps = { ...current.windowPanes.apps };
        if (this.appId)
          delete apps[this.appId];
        return { ...current, windowPanes: { apps } };
      });
      this.notify();
    }
    updateLayer(appId, state) {
      this.config.update((current) => {
        const apps = { ...current.windowPanes.apps };
        if (appId) {
          if (state)
            apps[appId] = state;
          else
            delete apps[appId];
          return { ...current, windowPanes: { ...current.windowPanes, apps } };
        }
        return { ...current, windowPanes: state ? { ...current.windowPanes, os: state } : { apps } };
      });
    }
  };
});

/* ../../../../../../../../../🔨️modules/🔺️mesh/🟦️component.ts */
function contextMenuIsBareSeparator(item) {
  return item.separator === true && item.label === undefined;
}
function contextMenuIsHeader(item) {
  return item.separator === true && item.label !== undefined;
}
function contextMenuIsGroupRow(item) {
  return item.id.startsWith("menu.group.");
}
function contextMenuGroupCategory(item) {
  return item.id.startsWith("menu.group.") ? item.id.slice("menu.group.".length) : item.id;
}
function contextMenuTaxonomyRank(category) {
  const index = RIBBON_PARENT_CATEGORIES.indexOf(category);
  return index === -1 ? RIBBON_PARENT_CATEGORIES.length : index;
}
function contextMenuSeparatorRow(seed) {
  return { id: `separator-organized-${seed}`, separator: true };
}
function contextMenuNormalizeSeparators(items) {
  const out = [];
  for (const item of items) {
    if (contextMenuIsBareSeparator(item) && out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1])) {
      continue;
    }
    out.push(item);
  }
  if (out.length > 0 && contextMenuIsBareSeparator(out[0])) {
    out.shift();
  }
  while (out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1])) {
    out.pop();
  }
  return out;
}
function contextMenuMergeGroupRows(items) {
  const out = [];
  const groupIndex = new Map;
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      const index = groupIndex.get(item.id);
      if (index !== undefined) {
        const children = out[index].children ? [...out[index].children] : [];
        for (const child of item.children ?? []) {
          if (!children.some((existing) => existing.id === child.id)) {
            children.push(child);
          }
        }
        out[index] = { ...out[index], children };
      } else {
        groupIndex.set(item.id, out.length);
        out.push(item);
      }
    } else {
      out.push(item);
    }
  }
  return out;
}
function contextMenuEmitWithinBudget(items) {
  const leavesAndHeaders = [];
  const groupRows = [];
  const destructiveLeaves = [];
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      groupRows.push(item);
    } else if (item.destructive === true) {
      destructiveLeaves.push(item);
    } else {
      leavesAndHeaders.push(item);
    }
  }
  groupRows.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  const out = [...leavesAndHeaders, ...groupRows];
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}
function contextMenuEmitOverBudget(items, categoryOf) {
  function bucketMut(buckets, id) {
    const index = buckets.findIndex((bucket) => bucket.id === id);
    if (index !== -1)
      return index;
    buckets.push({ id, label: undefined, children: [] });
    return buckets.length - 1;
  }
  const primaries = [];
  const existingGroups = [];
  const destructiveLeaves = [];
  const bucketedGroups = [];
  let currentHeaderKey;
  for (const item of items) {
    if (contextMenuIsHeader(item)) {
      currentHeaderKey = item.label;
      continue;
    }
    if (contextMenuIsGroupRow(item)) {
      existingGroups.push(item);
      currentHeaderKey = undefined;
      continue;
    }
    if (item.destructive === true) {
      destructiveLeaves.push(item);
      continue;
    }
    if (currentHeaderKey !== undefined) {
      const slug = currentHeaderKey.toLowerCase().split(/\s+/).join("-");
      const index2 = bucketMut(bucketedGroups, `menu.group.${slug}`);
      bucketedGroups[index2] = { ...bucketedGroups[index2], children: [...bucketedGroups[index2].children ?? [], item] };
      continue;
    }
    if (primaries.length < CONTEXT_MENU_PRIMARY_BUDGET) {
      primaries.push(item);
      continue;
    }
    const category = categoryOf(item.action ?? item.id) ?? "actions";
    const index = bucketMut(bucketedGroups, `menu.group.${category}`);
    bucketedGroups[index] = { ...bucketedGroups[index], children: [...bucketedGroups[index].children ?? [], item] };
  }
  const groups = [...existingGroups, ...bucketedGroups];
  groups.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  let out = [...primaries, ...groups];
  if (out.length > CONTEXT_MENU_ROW_BUDGET) {
    const foldFrom = CONTEXT_MENU_ROW_BUDGET - 1;
    const overflowingGroups = out.slice(foldFrom);
    out = out.slice(0, foldFrom);
    const foldedChildren = [];
    for (const group of overflowingGroups) {
      foldedChildren.push(...group.children ?? []);
    }
    out.push({ id: "menu.group.more", label: undefined, children: foldedChildren });
  }
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}
function organizeContextMenu(items, categoryOf) {
  const mapped = items.map((item) => ({
    ...item,
    children: item.children ? organizeContextMenu(item.children, categoryOf) : item.children
  }));
  const normalized = contextMenuMergeGroupRows(contextMenuNormalizeSeparators(mapped));
  const interactiveCount = normalized.filter((item) => item.separator !== true).length;
  return interactiveCount <= CONTEXT_MENU_ROW_BUDGET ? contextMenuEmitWithinBudget(normalized) : contextMenuEmitOverBudget(normalized, categoryOf);
}
var RIBBON_PARENT_CATEGORIES, CONTEXT_MENU_ROW_BUDGET = 9, CONTEXT_MENU_PRIMARY_BUDGET = 5, nodeGraphActions, textEditorActions, inkCanvasActions;
var init__component5 = __esm(() => {
  RIBBON_PARENT_CATEGORIES = [
    "history",
    "hand",
    "selection",
    "lasso",
    "filter",
    "open",
    "save",
    "transfer",
    "transform",
    "create",
    "view",
    "actions",
    "settings",
    "methods",
    "mode",
    "targets",
    "export",
    "tools",
    "utilities",
    "sync"
  ];
  nodeGraphActions = {
    select: "nodeGraphSelect",
    hover: "nodeGraphHover",
    edit: "nodeGraphEdit",
    viewport: "nodeGraphViewport",
    spotlightCommit: "spotlightCommit"
  };
  textEditorActions = {
    edit: "textEdit",
    select: "textSelect",
    hover: "textHover",
    requestCompletions: "requestCompletions",
    commitRename: "commitRename",
    formatDocument: "formatDocument"
  };
  inkCanvasActions = {
    applyEvents: "inkApplyEvents",
    setSelection: "setSelection",
    setCamera: "setCamera",
    setHover: "setHover"
  };
});

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts */
function orderEnvelopesByLane(envelopes) {
  return envelopes.map((envelope, index) => ({ envelope, index })).sort((left, right) => {
    const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
    return rank !== 0 ? rank : left.index - right.index;
  }).map((entry) => entry.envelope);
}
function createGrantedBudgetTracker(fallback = MAINTENANCE_LANE_DEFAULT_BUDGET) {
  const budgets = new Map;
  return {
    recordGrant(actorId, budget) {
      budgets.set(actorId, budget);
    },
    forget(actorId) {
      budgets.delete(actorId);
    },
    granted(actorId) {
      return budgets.get(actorId) ?? fallback;
    }
  };
}
function interpretShardFrame(frame, tracker) {
  switch (frame.kind) {
    case "Register":
      return { action: "register", actor: frame.actor };
    case "Unregister":
      tracker.forget(frame.actor);
      return { action: "unregister", actor: frame.actor };
    case "Grant":
      tracker.recordGrant(frame.actor, frame.budget);
      return { action: "runEnvelopes", actor: frame.actor, budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };
    case "Envelope":
      return { action: "runEnvelopes", actor: frame.envelope.to, budget: tracker.granted(frame.envelope.to), envelopes: [frame.envelope] };
    default:
      return { action: "unknown", frame };
  }
}
function formatQuotaBreachMessage(breach) {
  return `outstanding effect quota exceeded: ${breach.quota} limit=${breach.limit} actual=${breach.actual}`;
}
function freshHeartbeatState(nowMs) {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}
function graftWorkerStack(actorId, reason, stack, kind, framesBytes) {
  const error = new Error(reason);
  if (stack)
    error.stack = `${stack}
    ↳ main: ${error.stack ?? ""}`;
  console.log(`[DEBUG] program worker ${actorId || "unknown"} error type=${kind ?? "unknown"} framesBytes=${framesBytes ?? "n/a"}`);
  return error;
}

class ShardClient {
  shards = [];
  actorShard = new Map;
  pending = new Map;
  exclusiveIndices;
  heartbeatSabView;
  heartbeatTimeoutMs;
  watchdogIntervalMs;
  now;
  createWorker;
  onShardLost;
  onActorTrap;
  onHostEffect;
  maxOutstandingEffectsPerActor;
  outstandingEffectsByActor = new Map;
  effectReplySeq = 0;
  nextRoundRobin = 0;
  requestSeq = 0;
  watchdogHandle = null;
  constructor(options) {
    if (options.shardCount < 1)
      throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? DEFAULT_HEARTBEAT_TIMEOUT_MS;
    this.watchdogIntervalMs = options.watchdogIntervalMs ?? this.heartbeatTimeoutMs;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    this.onHostEffect = options.onHostEffect;
    this.maxOutstandingEffectsPerActor = options.maxOutstandingEffectsPerActor ?? DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set;
    for (let index = options.shardCount - exclusiveCount;index < options.shardCount; index += 1)
      exclusive.add(index);
    this.exclusiveIndices = exclusive;
    for (let index = 0;index < options.shardCount; index += 1)
      this.shards.push(this.spawnShard(index));
  }
  spawnShard(index) {
    const worker = this.createWorker(index);
    const slot = { index, worker, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set, actorIds: new Set };
    worker.onmessage = (event) => this.handleMessage(slot, event.data);
    worker.onerror = (error) => {
      console.error(`[DEBUG] shard ${index} worker error`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed`));
    };
    if (this.heartbeatSabView)
      worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.heartbeatSabView.buffer });
    return slot;
  }
  handleMessage(slot, message) {
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      this.handleInboundFrame(slot, message.actorId, message.frame);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry)
      return;
    this.pending.delete(message.requestId);
    slot.pendingRequestIds.delete(message.requestId);
    this.recomputeOldestPending(slot);
    if (message.ok)
      entry.resolve(message.value);
    else
      entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
  }
  recomputeOldestPending(slot) {
    let oldest = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      if (oldest === null || entry.startedAtMs < oldest)
        oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }
  failShard(slot, error) {
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      this.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
    for (const actorId of slot.actorIds) {
      this.abortOutstandingEffects(actorId);
      this.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
  }
  rejectActorPending(slot, actorId, error) {
    for (const requestId of [...slot.pendingRequestIds]) {
      const entry = this.pending.get(requestId);
      if (entry?.actorId !== actorId)
        continue;
      this.pending.delete(requestId);
      slot.pendingRequestIds.delete(requestId);
      entry.reject(error);
    }
    this.recomputeOldestPending(slot);
  }
  assignShard(actorId) {
    const existing = this.actorShard.get(actorId);
    if (existing !== undefined)
      return this.shards[existing];
    const roundRobinCount = this.shards.length - this.exclusiveIndices.size;
    let index = this.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.exclusiveIndices.has(index))
      index = (index + 1) % this.shards.length;
    this.nextRoundRobin = (this.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.actorShard.set(actorId, index);
    this.shards[index].actorIds.add(actorId);
    return this.shards[index];
  }
  leaseExclusive(actorId, options) {
    const already = this.actorShard.get(actorId);
    if (already !== undefined && this.exclusiveIndices.has(already))
      return already;
    for (const index of this.exclusiveIndices) {
      const slot = this.shards[index];
      if (slot.actorIds.size === 0 || options?.force) {
        if (already !== undefined)
          this.shards[already].actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`[DEBUG] ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.exclusiveIndices.size} reserved, all leased)`);
  }
  releaseExclusive(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined || !this.exclusiveIndices.has(index))
      return;
    this.shards[index].actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  shardIndexFor(actorId) {
    return this.actorShard.get(actorId);
  }
  nextRequestId() {
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }
  send(slot, message, requestId) {
    if (requestId === null) {
      slot.worker.postMessage(message);
      return Promise.resolve(undefined);
    }
    return new Promise((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve, reject, shardIndex: slot.index, startedAtMs, actorId: "actorId" in message ? message.actorId : "" });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null)
        slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      slot.worker.postMessage(message);
    });
  }
  async activate(actorId, moduleUrl, caps, budget, assets = []) {
    const slot = this.assignShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "activate", requestId, actorId, moduleUrl, caps, budget, assets }, requestId);
  }
  async turn(actorId, events, budget) {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined)
      throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    const slot = this.shards[shardIndex];
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "turn", requestId, actorId, events, budget }, requestId);
  }
  async envelope(shardEnvelope) {
    const slot = this.requireShard(shardEnvelope.to);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }
  async grant(actorId, budget, envelopes) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
  }
  async startJob(actorId, job, jobKind, input) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }
  async stepJob(actorId, job, budget) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }
  cancelJob(actorId, job) {
    const slot = this.requireShard(actorId);
    this.send(slot, { kind: "cancelJob", actorId, job }, null);
  }
  async takeSegmentedDownloadChunk(actorId, instanceId, operationId) {
    if (!Number.isSafeInteger(instanceId) || instanceId < 0 || typeof operationId !== "bigint" || operationId <= 0n || operationId > MAX_SEGMENTED_DOWNLOAD_OPERATION_ID)
      throw new Error("segmented-download-authority-invalid");
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const value = await this.send(slot, { kind: "takeSegmentedDownloadChunk", requestId, actorId, instanceId, operationId }, requestId);
    if (value === undefined || value === null)
      return;
    if (Object.prototype.toString.call(value) !== "[object Uint8Array]")
      throw new Error("segmented-download-transport-type");
    const chunk = value;
    if (chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES)
      throw new Error("segmented-download-transport-limit");
    return chunk;
  }
  async checkpoint(actorId) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }
  async restore(actorId, state) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "restore", requestId, actorId, state }, requestId);
  }
  dispose(actorId) {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined)
      return;
    this.abortOutstandingEffects(actorId);
    const slot = this.shards[shardIndex];
    this.rejectActorPending(slot, actorId, new Error(`ShardClient actor disposed: ${actorId}`));
    slot.worker.postMessage({ kind: "dispose", actorId });
    slot.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  requireShard(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined)
      throw new Error(`[DEBUG] ShardClient: actor ${actorId} is not activated on any shard`);
    return this.shards[index];
  }
  handleInboundFrame(slot, actorId, frame) {
    if (frame.kind !== "Envelope")
      return;
    const payload = frame.envelope.payload;
    if (payload.kind !== "effect-request")
      return;
    const request = payload.payload;
    this.handleEffectRequest(slot, actorId, request.effect, request.requestId, request.params);
  }
  handleEffectRequest(slot, actorId, effect, requestId, params) {
    const outstanding = this.outstandingEffectsByActor.get(actorId) ?? new Map;
    if (outstanding.size >= this.maxOutstandingEffectsPerActor) {
      const breach = { quota: "outstandingRequests", limit: this.maxOutstandingEffectsPerActor, actual: outstanding.size };
      this.replyEffectError(slot, actorId, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.onHostEffect) {
      this.replyEffectError(slot, actorId, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController;
    outstanding.set(requestId, controller);
    this.outstandingEffectsByActor.set(actorId, outstanding);
    this.onHostEffect(actorId, effect, params, controller.signal).then((value) => {
      if (this.settleEffect(actorId, requestId))
        this.replyEffectComplete(slot, actorId, requestId, value);
    }, (error) => {
      if (this.settleEffect(actorId, requestId))
        this.replyEffectError(slot, actorId, requestId, error instanceof Error ? error.message : String(error));
    });
  }
  settleEffect(actorId, requestId) {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding || !outstanding.delete(requestId))
      return false;
    if (outstanding.size === 0)
      this.outstandingEffectsByActor.delete(actorId);
    return true;
  }
  abortOutstandingEffects(actorId) {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding)
      return;
    this.outstandingEffectsByActor.delete(actorId);
    for (const controller of outstanding.values())
      controller.abort();
  }
  postEffectReply(slot, actorId, kind, innerPayload) {
    this.effectReplySeq += 1;
    const frame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } }
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.nextRequestId(), actorId, frame });
  }
  replyEffectComplete(slot, actorId, requestId, value) {
    this.postEffectReply(slot, actorId, "effect-complete", { requestId, value });
  }
  replyEffectError(slot, actorId, requestId, message) {
    this.postEffectReply(slot, actorId, "effect-error", { requestId, message });
  }
  recordHeartbeat(slot, turnSeq, atMs) {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }
  pollHeartbeatSab(nowMs = this.now()) {
    if (!this.heartbeatSabView)
      return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      if (seq !== slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }
  checkHeartbeats(nowMs = this.now()) {
    for (const slot of this.shards) {
      const pendingSince = slot.heartbeat.oldestPendingStartedAtMs;
      if (pendingSince === null)
        continue;
      if (slot.heartbeat.lastHeartbeatAtMs >= pendingSince)
        continue;
      const silentForMs = nowMs - pendingSince;
      if (silentForMs <= this.heartbeatTimeoutMs)
        continue;
      if (nowMs - slot.heartbeat.lastMissCountedAtMs < this.heartbeatTimeoutMs)
        continue;
      slot.heartbeat.missedCount += 1;
      slot.heartbeat.lastMissCountedAtMs = nowMs;
      if (slot.heartbeat.missedCount >= HEARTBEAT_MISSED_LIMIT) {
        const actorIds = [...slot.actorIds];
        this.terminate(slot.index);
        this.rebuild(slot.index);
        this.onShardLost?.(slot.index, actorIds);
      }
    }
  }
  startWatchdog(intervalMs = this.watchdogIntervalMs) {
    if (this.watchdogHandle !== null)
      return;
    this.watchdogHandle = setInterval(() => {
      this.pollHeartbeatSab();
      this.checkHeartbeats();
    }, intervalMs);
  }
  stopWatchdog() {
    if (this.watchdogHandle === null)
      return;
    clearInterval(this.watchdogHandle);
    this.watchdogHandle = null;
  }
  shardMetricsSamples(nowMs = this.now()) {
    return this.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
  terminate(index) {
    const slot = this.shards[index];
    if (!slot)
      throw new Error(`[DEBUG] ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.failShard(slot, new Error(`shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }
  rebuild(index) {
    const old = this.shards[index];
    if (!old)
      throw new Error(`[DEBUG] ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds)
      this.actorShard.delete(actorId);
    this.shards[index] = this.spawnShard(index);
  }
  disposeAll() {
    this.stopWatchdog();
    for (const slot of this.shards) {
      this.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
var MAINTENANCE_LANE_DEFAULT_BUDGET, SHARD_FRAME_VARIANT_FIELDS, SHARD_FRAME_LANE_ORDER, MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID, DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000, HEARTBEAT_MISSED_LIMIT = 3, DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;
var init__shard_client = __esm(() => {
  MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
  SHARD_FRAME_VARIANT_FIELDS = [
    { kind: "Register", fields: ["actor"] },
    { kind: "Unregister", fields: ["actor"] },
    { kind: "Grant", fields: ["actor", "budget", "envelopes"] },
    { kind: "Envelope", fields: ["envelope"] }
  ];
  SHARD_FRAME_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
  MAX_SEGMENTED_DOWNLOAD_OPERATION_ID = (1n << 64n) - 1n;
  if (import.meta.vitest) {
    let harness = function(shardCount = 2, extra) {
      const workers = [];
      let nowMs = 0;
      const client = new ShardClient({
        shardCount,
        createWorker: (index) => {
          const worker = new FakeShardWorker(index);
          workers.push(worker);
          return worker;
        },
        now: () => nowMs,
        ...extra
      });
      return { client, workers, advance: (ms) => nowMs += ms, setNow: (ms) => nowMs = ms };
    }, makeEnvelope = function(to, lane, seq, kind = "wake") {
      return { to, from: { kind: "kernel" }, lane, seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: {} } };
    }, makeEffectRequestFrame = function(actorId, effect, requestId, params) {
      return { kind: "frame", actorId, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
    }, findEffectReply = function(sent, requestId, kind) {
      return sent.find((message) => message.kind === "frame" && message.frame?.kind === "Envelope" && message.frame.envelope?.payload?.kind === kind && message.frame.envelope.payload?.payload?.requestId === requestId);
    }, flushMicrotasks = function() {
      return new Promise((resolve) => setTimeout(resolve, 0));
    };
    const { describe, expect, it, vi } = import.meta.vitest;

    class FakeShardWorker {
      index;
      onmessage = null;
      onerror = null;
      sent = [];
      terminated = false;
      constructor(index) {
        this.index = index;
      }
      postMessage(message) {
        this.sent.push(message);
      }
      terminate() {
        this.terminated = true;
      }
      deliver(message) {
        this.onmessage?.({ data: message });
      }
    }
    const BUDGET = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
    describe("ShardClient activation + turn round-trip", () => {
      it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
        const { client, workers } = harness(2);
        const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
        const activateMsg = workers[0].sent[0];
        expect(activateMsg.kind).toBe("activate");
        workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;
        const turnPromise = client.turn("actor-1", [{ kind: "wake", payload: {} }], BUDGET);
        const turnMsg = workers[0].sent[1];
        expect(turnMsg.kind).toBe("turn");
        workers[0].deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
        await expect(turnPromise).resolves.toEqual({ effects: [] });
      });
      it("rejects turn() for an actor never activated", async () => {
        const { client } = harness(2);
        await expect(client.turn("ghost", [], BUDGET)).rejects.toThrow(/not activated/);
      });
    });
    describe("ShardClient segmented-download transport", () => {
      async function activatedHarness() {
        const state = harness(1);
        const activation = state.client.activate("actor-download", "https://x/plugin.js", [], BUDGET);
        const message = state.workers[0].sent[0];
        state.workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
        await activation;
        return state;
      }
      it("preserves operation identity and last-Some then None ordering", async () => {
        const { client, workers } = await activatedHarness();
        for (const expected of [new Uint8Array([1, 2]), new Uint8Array([3]), undefined]) {
          const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
          const message = workers[0].sent.at(-1);
          expect(message).toMatchObject({ kind: "takeSegmentedDownloadChunk", instanceId: 17, operationId: 91n });
          workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: expected });
          await expect(read).resolves.toEqual(expected);
        }
      });
      it("propagates unknown-operation errors without manufacturing a terminal None", async () => {
        const { client, workers } = await activatedHarness();
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 404n);
        const message = workers[0].sent.at(-1);
        workers[0].deliver({ kind: "result", requestId: message.requestId, ok: false, error: "interactive-job.unknown-segmented-download" });
        await expect(read).rejects.toThrow("interactive-job.unknown-segmented-download");
      });
      it("rejects oversized or empty response items", async () => {
        const { client, workers } = await activatedHarness();
        for (const invalid of [new Uint8Array(4097), new Uint8Array(0)]) {
          const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
          const message = workers[0].sent.at(-1);
          workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: invalid });
          await expect(read).rejects.toThrow("segmented-download-transport-limit");
        }
      });
      it("rejects an in-flight read when actor disposal cancels its transport ownership", async () => {
        const { client } = await activatedHarness();
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        client.dispose("actor-download");
        await expect(read).rejects.toThrow("ShardClient actor disposed");
      });
      it("preserves the complete u64 operation authority through structured clone", async () => {
        const { client, workers } = await activatedHarness();
        for (const operationId of [(1n << 53n) + 1n, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID]) {
          const read = client.takeSegmentedDownloadChunk("actor-download", 17, operationId);
          const message = structuredClone(workers[0].sent.at(-1));
          expect(message.operationId).toBe(operationId);
          workers[0].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
          await expect(read).resolves.toBeUndefined();
        }
      });
      it("rejects zero and overflowing operation authorities", async () => {
        const { client } = await activatedHarness();
        await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 0n)).rejects.toThrow("segmented-download-authority-invalid");
        await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 1n << 64n)).rejects.toThrow("segmented-download-authority-invalid");
      });
    });
    describe("ShardClient actor-id multiplexing", () => {
      it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
        const { client, workers } = harness(1);
        const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
        const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
        const [msgA, msgB] = workers[0].sent;
        workers[0].deliver({ kind: "result", requestId: msgB.requestId, ok: true, value: undefined });
        workers[0].deliver({ kind: "result", requestId: msgA.requestId, ok: true, value: undefined });
        await expect(p1).resolves.toBeUndefined();
        await expect(p2).resolves.toBeUndefined();
        expect(client.shardIndexFor("a")).toBe(0);
        expect(client.shardIndexFor("b")).toBe(0);
      });
      it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
        const { client } = harness(4, { exclusiveShardCount: 1 });
        const indices = ["a", "b", "c"].map((id) => {
          client.activate(id, "https://x/y.js", [], BUDGET);
          return client.shardIndexFor(id);
        });
        expect(new Set(indices).has(3)).toBe(false);
        expect(indices).toEqual([0, 1, 2]);
      });
    });
    describe("ShardClient heartbeat watchdog (postMessage path)", () => {
      it("does not miss while idle (no pending turn)", () => {
        const { client, advance } = harness(1);
        advance(1e5);
        client.checkHeartbeats(1e5);
        expect(true).toBe(true);
      });
      it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
        const lost = [];
        const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
        setNow(0);
        const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
        const activateMsg = workers[0].sent[0];
        workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;
        const originalWorker = workers[0];
        client.turn("stuck", [], BUDGET).catch(() => {});
        advance(1001);
        client.checkHeartbeats();
        expect(lost).toEqual([]);
        expect(originalWorker.terminated).toBe(false);
        advance(1001);
        client.checkHeartbeats();
        advance(1001);
        client.checkHeartbeats();
        expect(originalWorker.terminated).toBe(true);
        expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
        expect(client.shardIndexFor("stuck")).toBeUndefined();
      });
      it("a fresh heartbeat resets the miss count", async () => {
        const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
        setNow(0);
        const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
        await activatePromise;
        client.turn("busy", [], BUDGET);
        advance(1001);
        client.checkHeartbeats();
        advance(1001);
        client.checkHeartbeats();
        workers[0].deliver({ kind: "heartbeat", turnSeq: 7 });
        advance(1001);
        client.checkHeartbeats();
        expect(workers[0].terminated).toBe(false);
      });
    });
    describe("ShardClient SAB heartbeat path", () => {
      it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
        const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
        const view = new Int32Array(sab);
        const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
        setNow(0);
        const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent.find((m) => m.kind === "activate").requestId, ok: true, value: undefined });
        await activatePromise;
        client.turn("sab-actor", [], BUDGET);
        advance(1001);
        client.checkHeartbeats();
        expect(workers[0].terminated).toBe(false);
        Atomics.store(view, 0, 5);
        client.pollHeartbeatSab();
        advance(1001);
        client.checkHeartbeats();
        expect(workers[0].terminated).toBe(false);
      });
    });
    describe("ShardClient leaseExclusive", () => {
      it("moves an actor onto a reserved exclusive shard and back", async () => {
        const { client } = harness(4, { exclusiveShardCount: 2 });
        client.activate("heavy", "https://x/h.js", [], BUDGET);
        expect(client.shardIndexFor("heavy")).toBe(0);
        const exclusiveIndex = client.leaseExclusive("heavy");
        expect([2, 3]).toContain(exclusiveIndex);
        expect(client.shardIndexFor("heavy")).toBe(exclusiveIndex);
        client.releaseExclusive("heavy");
        expect(client.shardIndexFor("heavy")).toBeUndefined();
      });
      it("throws once every exclusive shard is leased and force is not set", () => {
        const { client } = harness(3, { exclusiveShardCount: 1 });
        client.activate("first", "https://x/1.js", [], BUDGET);
        client.leaseExclusive("first");
        client.activate("second", "https://x/2.js", [], BUDGET);
        expect(() => client.leaseExclusive("second")).toThrow(/no free exclusive shard/);
      });
      it("is idempotent for the same actor already leased", () => {
        const { client } = harness(2, { exclusiveShardCount: 1 });
        client.activate("a", "https://x/a.js", [], BUDGET);
        const first = client.leaseExclusive("a");
        const second = client.leaseExclusive("a");
        expect(first).toBe(second);
      });
    });
    describe("ShardClient.startWatchdog / stopWatchdog", () => {
      it("self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds", async () => {
        vi.useFakeTimers();
        try {
          const lost = [];
          const workers = [];
          const client = new ShardClient({
            shardCount: 1,
            createWorker: (index) => {
              const worker = new FakeShardWorker(index);
              workers.push(worker);
              return worker;
            },
            heartbeatTimeoutMs: 1000,
            onShardLost: (index, actorIds) => lost.push({ index, actorIds })
          });
          const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
          const activateMsg = workers[0].sent[0];
          workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
          await activatePromise;
          client.turn("stuck", [], BUDGET).catch(() => {});
          client.startWatchdog(1001);
          vi.advanceTimersByTime(1001);
          expect(lost).toEqual([]);
          vi.advanceTimersByTime(1001);
          expect(lost).toEqual([]);
          vi.advanceTimersByTime(1001);
          expect(workers[0].terminated).toBe(true);
          expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
          expect(client.shardIndexFor("stuck")).toBeUndefined();
          client.stopWatchdog();
          const lostCountAfterStop = lost.length;
          vi.advanceTimersByTime(1e4);
          expect(lost.length).toBe(lostCountAfterStop);
        } finally {
          vi.useRealTimers();
        }
      });
      it("is idempotent to call twice, and stopWatchdog before ever starting is a no-op", () => {
        vi.useFakeTimers();
        try {
          const { client } = harness(1);
          client.startWatchdog(500);
          client.startWatchdog(500);
          client.stopWatchdog();
          client.stopWatchdog();
          expect(true).toBe(true);
        } finally {
          vi.useRealTimers();
        }
      });
    });
    describe("ShardClient failShard clears routing", () => {
      it("clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
        await activatePromise;
        expect(client.shardIndexFor("x")).toBe(0);
        workers[0].onerror?.(new Error("boom"));
        expect(client.shardIndexFor("x")).toBeUndefined();
        await expect(client.turn("x", [], BUDGET)).rejects.toThrow(/not activated/);
      });
    });
    describe("ShardClient terminate/rebuild", () => {
      it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
        const rejection = expect(activatePromise).rejects.toThrow(/terminated/);
        const oldWorker = workers[0];
        const actorIds = client.terminate(0);
        expect(actorIds).toEqual(["x"]);
        await rejection;
        expect(oldWorker.terminated).toBe(true);
        client.rebuild(0);
        expect(workers.length).toBe(2);
        expect(client.shardIndexFor("x")).toBeUndefined();
      });
    });
    describe("ShardClient worker crash", () => {
      it("onerror fails every pending request on that shard", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("crashy", "https://x/c.js", [], BUDGET);
        workers[0].onerror?.(new Error("boom"));
        await expect(activatePromise).rejects.toThrow(/crashed/);
      });
    });
    describe("ShardClient.shardMetricsSamples", () => {
      it("reports zero actors/busyRatio and an infinite heartbeat age for a fresh, never-touched shard", () => {
        const { client } = harness(2);
        const samples = client.shardMetricsSamples(1000);
        expect(samples).toHaveLength(2);
        for (const sample of samples) {
          expect(sample.metrics.actors).toBe(0);
          expect(sample.metrics.busyRatio).toBe(0);
          expect(sample.metrics.heartbeatAgeMs).toBe(Number.POSITIVE_INFINITY);
        }
      });
      it("counts resident actors and in-flight turns as busyRatio, and ages the heartbeat off the injected clock", async () => {
        const { client, workers, setNow } = harness(1);
        setNow(0);
        const activateA = client.activate("a", "https://x/a.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
        await activateA;
        const activateB = client.activate("b", "https://x/b.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[1].requestId, ok: true, value: undefined });
        await activateB;
        workers[0].deliver({ kind: "heartbeat", turnSeq: 1 });
        client.turn("a", [], BUDGET);
        setNow(300);
        const [sample] = client.shardMetricsSamples(300);
        expect(sample.metrics.actors).toBe(2);
        expect(sample.metrics.busyRatio).toBeCloseTo(0.5);
        expect(sample.metrics.heartbeatAgeMs).toBe(300);
      });
    });
    describe("orderEnvelopesByLane", () => {
      it("sorts by Lane priority, not arrival order, stable within a tied lane", () => {
        const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2), makeEnvelope("a", "Maintenance", 3), makeEnvelope("a", "Interactive", 4)];
        const ordered = orderEnvelopesByLane(envelopes);
        expect(ordered.map((envelope) => envelope.seq)).toEqual([2, 4, 1, 3]);
      });
      it("is a no-op for an already-lane-sorted, single-lane batch", () => {
        const envelopes = [makeEnvelope("a", "Interactive", 1), makeEnvelope("a", "Interactive", 2)];
        expect(orderEnvelopesByLane(envelopes).map((envelope) => envelope.seq)).toEqual([1, 2]);
      });
    });
    describe("GrantedBudgetTracker + interpretShardFrame", () => {
      it("a Grant records its budget and hands back envelopes in lane-priority order", () => {
        const tracker = createGrantedBudgetTracker();
        const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
        const grantBudget = { ...BUDGET, fuel: 999 };
        const result = interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes }, tracker);
        expect(result).toEqual({ action: "runEnvelopes", actor: "a", budget: grantBudget, envelopes: [envelopes[1], envelopes[0]] });
        expect(tracker.granted("a")).toBe(grantBudget);
      });
      it("an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant", () => {
        const tracker = createGrantedBudgetTracker();
        const lonelyEnvelope = makeEnvelope("never-granted", "Interactive", 1);
        const result = interpretShardFrame({ kind: "Envelope", envelope: lonelyEnvelope }, tracker);
        expect(result).toEqual({ action: "runEnvelopes", actor: "never-granted", budget: MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [lonelyEnvelope] });
      });
      it("an Envelope AFTER a Grant for the same actor runs under THAT granted budget — proving the old constant no longer influences it", () => {
        const tracker = createGrantedBudgetTracker();
        const grantBudget = { ...BUDGET, fuel: 42 };
        interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes: [] }, tracker);
        const followUp = makeEnvelope("a", "Interactive", 5);
        const result = interpretShardFrame({ kind: "Envelope", envelope: followUp }, tracker);
        expect(result.action).toBe("runEnvelopes");
        expect(result.budget).toBe(grantBudget);
        expect(result.budget).not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET);
      });
      it("Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget", () => {
        const tracker = createGrantedBudgetTracker();
        interpretShardFrame({ kind: "Grant", actor: "a", budget: BUDGET, envelopes: [] }, tracker);
        expect(tracker.granted("a")).toBe(BUDGET);
        expect(interpretShardFrame({ kind: "Register", actor: "a" }, tracker)).toEqual({ action: "register", actor: "a" });
        expect(interpretShardFrame({ kind: "Unregister", actor: "a" }, tracker)).toEqual({ action: "unregister", actor: "a" });
        expect(tracker.granted("a")).toEqual(MAINTENANCE_LANE_DEFAULT_BUDGET);
      });
      it("an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)", () => {
        const tracker = createGrantedBudgetTracker();
        const futureFrame = { kind: "Checkpoint", actor: "a" };
        expect(() => interpretShardFrame(futureFrame, tracker)).not.toThrow();
        expect(interpretShardFrame(futureFrame, tracker)).toEqual({ action: "unknown", frame: futureFrame });
      });
    });
    describe("ShardClient.grant / ShardClient.envelope wire adoption", () => {
      it("grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
        await activatePromise;
        const grantBudget = { ...BUDGET, fuel: 12345 };
        const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
        client.grant("a", grantBudget, envelopes);
        const sent = workers[0].sent[1];
        expect(sent.kind).toBe("frame");
        expect(sent.frame.kind).toBe("Grant");
        const grantFrame = sent.frame;
        expect(grantFrame.budget).toBe(grantBudget);
        expect(grantFrame.envelopes.map((envelope) => envelope.seq)).toEqual([2, 1]);
      });
      it("envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
        workers[0].deliver({ kind: "result", requestId: workers[0].sent[0].requestId, ok: true, value: undefined });
        await activatePromise;
        client.envelope(makeEnvelope("a", "Interactive", 1));
        const sent = workers[0].sent[1];
        expect(sent.kind).toBe("frame");
        expect(sent.frame.kind).toBe("Envelope");
        expect(Object.keys(sent.frame)).toEqual(["kind", "envelope"]);
      });
      it("turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)", async () => {
        const { client, workers } = harness(1);
        const activatePromise = client.activate("legacy", "https://x/legacy.js", [], BUDGET);
        const activateMsg = workers[0].sent[0];
        expect(activateMsg.kind).toBe("activate");
        workers[0].deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;
        const turnPromise = client.turn("legacy", [{ kind: "wake", payload: {} }], BUDGET);
        const turnMsg = workers[0].sent[1];
        expect(turnMsg.kind).toBe("turn");
        workers[0].deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
        await expect(turnPromise).resolves.toEqual({ effects: [] });
      });
    });
    describe("ShardFrame parity with Rust component.rs", () => {
      it("TS ShardFrame variant/field names match the live Rust enum in \uD83D\uDDA5️host/\uD83E\uDDF5️shard/\uD83E\uDD80️component.rs", async () => {
        const { readFileSync } = await import("node:fs");
        const rustUrl = new URL("../../../../\uD83D\uDECD️products/\uD83D\uDCBB️os/\uD83D\uDD28️modules/\uD83D\uDD0C️plugin/\uD83D\uDDA5️host/\uD83E\uDDF5️shard/\uD83E\uDD80️component.rs", import.meta.url);
        const source = readFileSync(rustUrl, "utf8");
        const enumMatch = source.match(/pub enum ShardFrame \{([\s\S]*?)\n\}\s*\n\s*impl ShardFrame/);
        expect(enumMatch).not.toBeNull();
        const body = enumMatch[1].replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
        const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
        const rustVariants = [];
        let match;
        while ((match = variantPattern.exec(body)) !== null) {
          const [, name, structFields, tupleType] = match;
          if (structFields !== undefined) {
            const fields = structFields.split(",").map((part) => part.trim()).filter((part) => part.length > 0).map((part) => part.split(":")[0].trim());
            rustVariants.push({ name, fields });
          } else if (tupleType !== undefined) {
            rustVariants.push({ name, fields: null });
          }
        }
        expect(rustVariants.map((variant) => variant.name)).toEqual(SHARD_FRAME_VARIANT_FIELDS.map((variant) => variant.kind));
        for (const rustVariant of rustVariants) {
          if (rustVariant.fields === null)
            continue;
          const tsVariant = SHARD_FRAME_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name);
          expect(tsVariant.fields).toEqual(rustVariant.fields);
        }
      });
    });
    async function activateActor(client, workers, actorId, shardIndex = 0) {
      const promise = client.activate(actorId, `https://x/${actorId}.js`, [], BUDGET);
      const message = workers[shardIndex].sent.at(-1);
      workers[shardIndex].deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await promise;
    }
    describe("ShardClient host-effect bridge — handler success", () => {
      it("resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker", async () => {
        const { client, workers } = harness(1, { onHostEffect: async (actorId, effect, params) => ({ actorId, effect, params, from: "handler" }) });
        await activateActor(client, workers, "a");
        workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", { url: "https://example.test" }));
        await flushMicrotasks();
        const reply = findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-complete");
        expect(reply).toBeDefined();
        expect(reply?.frame.envelope.payload.payload.value).toEqual({ actorId: "a", effect: "http-fetch", params: { url: "https://example.test" }, from: "handler" });
      });
    });
    describe("ShardClient host-effect bridge — handler error", () => {
      it("a rejected onHostEffect settles as effect-error, never a hang", async () => {
        const { client, workers } = harness(1, { onHostEffect: async () => {
          throw new Error("boom");
        } });
        await activateActor(client, workers, "a");
        workers[0].deliver(makeEffectRequestFrame("a", "blob-read", "a:blob-read:1", { hash: "x" }));
        await flushMicrotasks();
        const reply = findEffectReply(workers[0].sent, "a:blob-read:1", "effect-error");
        expect(reply?.frame.envelope.payload.payload.message).toBe("boom");
      });
    });
    describe("ShardClient host-effect bridge — no handler installed", () => {
      it("fails FAST with an explicit effect-error, synchronously, never a silent hang", async () => {
        const { client, workers } = harness(1);
        await activateActor(client, workers, "a");
        workers[0].deliver(makeEffectRequestFrame("a", "storage-read", "a:storage-read:1", {}));
        const reply = findEffectReply(workers[0].sent, "a:storage-read:1", "effect-error");
        expect(reply?.frame.envelope.payload.payload.message).toBe("no host effect handler installed");
      });
    });
    describe("ShardClient host-effect bridge — backpressure cap", () => {
      it("rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending", async () => {
        const { client, workers } = harness(1, { maxOutstandingEffectsPerActor: 1, onHostEffect: () => new Promise(() => {}) });
        await activateActor(client, workers, "a");
        workers[0].deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:1", {}));
        workers[0].deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:2", {}));
        expect(findEffectReply(workers[0].sent, "a:spawn-job:1", "effect-error")).toBeUndefined();
        expect(findEffectReply(workers[0].sent, "a:spawn-job:1", "effect-complete")).toBeUndefined();
        const reply = findEffectReply(workers[0].sent, "a:spawn-job:2", "effect-error");
        expect(reply?.frame.envelope.payload.payload.message).toMatch(/outstandingRequests.*limit=1.*actual=1/);
      });
    });
    describe("ShardClient host-effect bridge — shard-loss settlement", () => {
      it("terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker", async () => {
        let capturedSignal;
        const { client, workers } = harness(1, {
          onHostEffect: (_actorId, _effect, _params, signal) => new Promise((resolve) => {
            capturedSignal = signal;
            signal.addEventListener("abort", () => resolve("too-late"));
          })
        });
        await activateActor(client, workers, "a");
        workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
        expect(capturedSignal?.aborted).toBe(false);
        client.terminate(0);
        expect(capturedSignal?.aborted).toBe(true);
        const sentBeforeLateResolve = workers[0].sent.length;
        await flushMicrotasks();
        expect(workers[0].sent.length).toBe(sentBeforeLateResolve);
        expect(findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-complete")).toBeUndefined();
        expect(findEffectReply(workers[0].sent, "a:http-fetch:1", "effect-error")).toBeUndefined();
      });
      it("dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's", async () => {
        const signals = {};
        const { client, workers } = harness(1, {
          onHostEffect: (actorId, _effect, _params, signal) => {
            signals[actorId] = signal;
            return new Promise(() => {});
          }
        });
        await activateActor(client, workers, "a");
        await activateActor(client, workers, "b");
        workers[0].deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
        workers[0].deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}));
        client.dispose("a");
        expect(signals.a?.aborted).toBe(true);
        expect(signals.b?.aborted).toBe(false);
      });
    });
  }
});

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts */
function laneRank(lane) {
  return MAILBOX_LANE_ORDER.indexOf(lane);
}
function createBoundedMailbox(capacity) {
  const lanes = MAILBOX_LANE_ORDER.map(() => []);
  let len = 0;
  return {
    enqueue(envelope) {
      const incomingRank = laneRank(envelope.lane);
      if (envelope.coalesce !== undefined) {
        const lane = lanes[incomingRank];
        const existingIndex = lane.findIndex((queued) => queued.coalesce === envelope.coalesce);
        if (existingIndex !== -1) {
          lane[existingIndex] = envelope;
          return { kind: "coalesced" };
        }
      }
      if (len >= capacity) {
        let victimRank = -1;
        for (let rank = MAILBOX_LANE_ORDER.length - 1;rank > incomingRank; rank--) {
          if (lanes[rank].length > 0) {
            victimRank = rank;
            break;
          }
        }
        if (victimRank === -1)
          return { kind: "rejected" };
        lanes[victimRank].shift();
        len -= 1;
        lanes[incomingRank].push(envelope);
        len += 1;
        return { kind: "dropped", lane: MAILBOX_LANE_ORDER[victimRank] };
      }
      lanes[incomingRank].push(envelope);
      len += 1;
      return { kind: "accept" };
    },
    popNext() {
      for (const lane of lanes) {
        if (lane.length > 0) {
          len -= 1;
          return lane.shift();
        }
      }
      return;
    },
    get length() {
      return len;
    },
    get isEmpty() {
      return len === 0;
    }
  };
}
var MAILBOX_LANE_ORDER;
var init__mailbox = __esm(() => {
  MAILBOX_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
  if (import.meta.vitest) {
    const { describe, expect, it } = import.meta.vitest;
    describe("createBoundedMailbox", () => {
      it("overflow is rejected (not a silent drop) when nothing lower-priority exists to evict", () => {
        const mailbox = createBoundedMailbox(2);
        expect(mailbox.enqueue({ lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
        expect(mailbox.enqueue({ lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
        expect(mailbox.enqueue({ lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
        expect(mailbox.length).toBe(2);
      });
      it("coalescing collapses same-key entries latest-wins, preserving queue position", () => {
        const mailbox = createBoundedMailbox(10);
        for (let i = 0;i < 200; i++) {
          const backpressure = mailbox.enqueue({ lane: "Interactive", coalesce: "pointer-move", payload: i });
          expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
        }
        expect(mailbox.length).toBe(1);
        expect(mailbox.popNext()?.payload).toBe(199);
      });
      it("lane priority beats FIFO order on popNext", () => {
        const mailbox = createBoundedMailbox(10);
        mailbox.enqueue({ lane: "Maintenance", payload: "low" });
        mailbox.enqueue({ lane: "Background", payload: "mid" });
        mailbox.enqueue({ lane: "Interactive", payload: "high" });
        expect(mailbox.popNext()?.lane).toBe("Interactive");
        expect(mailbox.popNext()?.lane).toBe("Background");
        expect(mailbox.popNext()?.lane).toBe("Maintenance");
        expect(mailbox.isEmpty).toBe(true);
      });
      it("dropped backpressure reports the evicted lane, admitting the higher-priority incomer", () => {
        const mailbox = createBoundedMailbox(2);
        mailbox.enqueue({ lane: "Maintenance", payload: "a" });
        mailbox.enqueue({ lane: "Background", payload: "b" });
        const backpressure = mailbox.enqueue({ lane: "Interactive", payload: "c" });
        expect(backpressure).toEqual({ kind: "dropped", lane: "Maintenance" });
        expect(mailbox.length).toBe(2);
        expect(mailbox.popNext()?.payload).toBe("c");
        expect(mailbox.popNext()?.payload).toBe("b");
      });
    });
  }
});

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts */
function freshLaneCounts() {
  return { Interactive: 0, UserVisible: 0, Background: 0, Maintenance: 0 };
}

class TurnScheduler {
  mailboxes = new Map;
  laneCounts = new Map;
  busyActors = new Set;
  options;
  pumpScheduled = false;
  constructor(options) {
    this.options = options;
  }
  enqueue(actorId, turn) {
    const mailbox = this.mailboxFor(actorId);
    const backpressure = mailbox.enqueue({ lane: turn.lane, coalesce: turn.coalesce, payload: turn.payload });
    this.applyLaneDelta(actorId, turn.lane, backpressure);
    if (backpressure.kind !== "rejected")
      this.schedulePump();
    return backpressure;
  }
  mailboxFor(actorId) {
    let mailbox = this.mailboxes.get(actorId);
    if (!mailbox) {
      mailbox = createBoundedMailbox(this.options.mailboxCapacity);
      this.mailboxes.set(actorId, mailbox);
      this.laneCounts.set(actorId, freshLaneCounts());
    }
    return mailbox;
  }
  applyLaneDelta(actorId, incomingLane, backpressure) {
    const counts = this.laneCounts.get(actorId);
    if (backpressure.kind === "accept") {
      counts[incomingLane] += 1;
    } else if (backpressure.kind === "dropped") {
      counts[backpressure.lane] -= 1;
      counts[incomingLane] += 1;
    }
  }
  cancelQueued(actorId) {
    const mailbox = this.mailboxes.get(actorId);
    if (!mailbox)
      return 0;
    const counts = this.laneCounts.get(actorId);
    let cancelled = 0;
    let envelope;
    while ((envelope = mailbox.popNext()) !== undefined) {
      counts[envelope.lane] -= 1;
      cancelled += 1;
    }
    return cancelled;
  }
  teardownActor(actorId) {
    const cancelled = this.cancelQueued(actorId);
    this.mailboxes.delete(actorId);
    this.laneCounts.delete(actorId);
    return cancelled;
  }
  isBusy(actorId) {
    return this.busyActors.has(actorId);
  }
  pendingCount(actorId) {
    return this.mailboxes.get(actorId)?.length ?? 0;
  }
  schedulePump() {
    if (this.pumpScheduled)
      return;
    this.pumpScheduled = true;
    queueMicrotask(() => {
      this.pumpScheduled = false;
      this.pump();
    });
  }
  pickNextReadyActor() {
    for (const lane of LANE_ORDER) {
      for (const actorId of this.mailboxes.keys()) {
        if (this.busyActors.has(actorId))
          continue;
        const counts = this.laneCounts.get(actorId);
        if (counts && counts[lane] > 0)
          return actorId;
      }
    }
    return;
  }
  pump() {
    for (;; ) {
      const actorId = this.pickNextReadyActor();
      if (actorId === undefined)
        return;
      const mailbox = this.mailboxes.get(actorId);
      const envelope = mailbox.popNext();
      if (!envelope)
        continue;
      this.laneCounts.get(actorId)[envelope.lane] -= 1;
      this.busyActors.add(actorId);
      const budget = this.options.budgetFor(actorId);
      this.options.runTurn(actorId, envelope.payload, budget).catch((error) => this.options.onTurnError?.(actorId, error)).finally(() => {
        this.busyActors.delete(actorId);
        this.schedulePump();
      });
    }
  }
}
var LANE_ORDER;
var init__turn_scheduler = __esm(() => {
  init__mailbox();
  LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
  if (import.meta.vitest) {
    let deferred = function() {
      let resolve;
      let reject;
      const promise = new Promise((res, rej) => {
        resolve = res;
        reject = rej;
      });
      return { promise, resolve, reject };
    }, harness = function(mailboxCapacity = 10) {
      const order = [];
      const running = new Map;
      const scheduler = new TurnScheduler({
        mailboxCapacity,
        budgetFor: () => {
          return;
        },
        runTurn: (actorId, payload) => {
          order.push({ actorId, payload });
          const { promise, resolve, reject } = deferred();
          running.set(actorId, { resolve, reject });
          return promise;
        }
      });
      return { scheduler, order, settle: (actorId) => running.get(actorId)?.resolve(), fail: (actorId, error) => running.get(actorId)?.reject(error) };
    };
    const { describe, expect, it, vi } = import.meta.vitest;
    const flush = () => new Promise((resolve) => queueMicrotask(() => queueMicrotask(resolve)));
    describe("TurnScheduler lane priority", () => {
      it("dispatches by lane priority, not arrival order, when a batch lands before the first pick", async () => {
        const { scheduler, order } = harness();
        scheduler.enqueue("low", { lane: "Background", payload: "low-1" });
        scheduler.enqueue("high", { lane: "Interactive", payload: "high-1" });
        scheduler.enqueue("mid", { lane: "UserVisible", payload: "mid-1" });
        await flush();
        expect(order.map((entry) => entry.actorId)).toEqual(["high", "mid", "low"]);
      });
    });
    describe("TurnScheduler per-actor ordering under interleaving", () => {
      it("never starts an actor's next turn before its current one settles, even while other actors interleave", async () => {
        const { scheduler, order, settle } = harness();
        scheduler.enqueue("a", { lane: "Interactive", payload: "a-1" });
        await flush();
        expect(order.map((entry) => entry.actorId)).toEqual(["a"]);
        scheduler.enqueue("a", { lane: "Interactive", payload: "a-2" });
        scheduler.enqueue("b", { lane: "Interactive", payload: "b-1" });
        await flush();
        expect(order.map((entry) => entry.actorId)).toEqual(["a", "b"]);
        expect(scheduler.isBusy("a")).toBe(true);
        settle("a");
        await flush();
        expect(order.map((entry) => entry.actorId)).toEqual(["a", "b", "a"]);
        expect(order[2].payload).toBe("a-2");
        settle("b");
        settle("a");
        await flush();
      });
    });
    describe("TurnScheduler coalescing", () => {
      it("collapses a burst of same-key envelopes to one queued turn, never 200 deep", async () => {
        const { scheduler, order, settle } = harness();
        for (let i = 0;i < 200; i++) {
          const backpressure = scheduler.enqueue("pointer", { lane: "Interactive", coalesce: "pointer-move", payload: i });
          expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
        }
        expect(scheduler.pendingCount("pointer")).toBe(1);
        await flush();
        expect(order).toHaveLength(1);
        expect(order[0].payload).toBe(199);
        settle("pointer");
      });
    });
    describe("TurnScheduler backpressure at the cap", () => {
      it("rejected surfaces synchronously at the cap instead of the queue growing past it", () => {
        const { scheduler } = harness(2);
        expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
        expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
        expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
        expect(scheduler.pendingCount("full")).toBe(2);
      });
    });
    describe("TurnScheduler cancellation", () => {
      it("cancels only queued turns, leaving an in-flight one to settle on its own", async () => {
        const { scheduler, order, settle } = harness();
        scheduler.enqueue("x", { lane: "Interactive", payload: "x-1" });
        await flush();
        expect(order.map((e) => e.payload)).toEqual(["x-1"]);
        scheduler.enqueue("x", { lane: "Interactive", payload: "x-2" });
        scheduler.enqueue("x", { lane: "Background", payload: "x-3" });
        expect(scheduler.pendingCount("x")).toBe(2);
        const cancelled = scheduler.cancelQueued("x");
        expect(cancelled).toBe(2);
        expect(scheduler.pendingCount("x")).toBe(0);
        settle("x");
        await flush();
        expect(order.map((e) => e.payload)).toEqual(["x-1"]);
      });
      it("teardownActor cancels queued work and forgets the actor so a later enqueue starts fresh", async () => {
        const { scheduler, order } = harness();
        scheduler.enqueue("y", { lane: "Interactive", payload: "y-1" });
        scheduler.enqueue("y", { lane: "Interactive", payload: "y-2" });
        const cancelled = scheduler.teardownActor("y");
        expect(cancelled).toBe(2);
        expect(scheduler.pendingCount("y")).toBe(0);
        await flush();
        expect(order).toHaveLength(0);
      });
    });
    describe("TurnScheduler onTurnError", () => {
      it("reports a rejected runTurn instead of throwing out of the pump loop, and keeps draining", async () => {
        const errors = [];
        const scheduler = new TurnScheduler({
          mailboxCapacity: 5,
          budgetFor: () => {
            return;
          },
          runTurn: async (actorId, payload) => {
            if (payload === "boom")
              throw new Error("turn failed");
          },
          onTurnError: (actorId, error) => errors.push({ actorId, error })
        });
        scheduler.enqueue("z", { lane: "Interactive", payload: "boom" });
        await flush();
        expect(errors).toHaveLength(1);
        expect(errors[0].actorId).toBe("z");
        expect(scheduler.isBusy("z")).toBe(false);
      });
    });
  }
});

/* ../../../../../../../../../🔨️modules/🎠️kernel/🟦️component.ts */
class OsTransient {
  boxes = new Map;
  maps = new Map;
  sets = new Map;
  weakMaps = new Map;
  box(key, init) {
    let box = this.boxes.get(key);
    if (!box) {
      box = { current: init };
      this.boxes.set(key, box);
    }
    return box;
  }
  map(key) {
    let map = this.maps.get(key);
    if (!map) {
      map = new Map;
      this.maps.set(key, map);
    }
    return map;
  }
  set(key) {
    let set = this.sets.get(key);
    if (!set) {
      set = new Set;
      this.sets.set(key, set);
    }
    return set;
  }
  weakMap(key) {
    let map = this.weakMaps.get(key);
    if (!map) {
      map = new WeakMap;
      this.weakMaps.set(key, map);
    }
    return map;
  }
  reset() {
    this.boxes.clear();
    this.maps.clear();
    this.sets.clear();
    this.weakMaps.clear();
  }
}
function ephemeralBox(key, init) {
  return defaultOsTransient.box(key, init);
}
function ephemeralMap(key) {
  return defaultOsTransient.map(key);
}
function ephemeralSet(key) {
  return defaultOsTransient.set(key);
}
function ephemeralWeakMap(key) {
  return defaultOsTransient.weakMap(key);
}
function createTurnOutcomeBroadcast() {
  const subscribers = new Set;
  return {
    push: (value) => {
      for (const subscriber of subscribers) {
        if (subscriber.resolve) {
          const resolve = subscriber.resolve;
          subscriber.resolve = null;
          resolve({ value, done: false });
        } else {
          subscriber.queue.push(value);
        }
      }
    },
    complete: () => {
      for (const subscriber of subscribers)
        subscriber.resolve?.({ value: undefined, done: true });
      subscribers.clear();
    },
    stream: {
      [Symbol.asyncIterator]() {
        const subscriber = { queue: [], resolve: null };
        subscribers.add(subscriber);
        return {
          next: () => {
            if (subscriber.queue.length > 0)
              return Promise.resolve({ value: subscriber.queue.shift(), done: false });
            return new Promise((resolve) => {
              subscriber.resolve = resolve;
            });
          },
          return: () => {
            subscribers.delete(subscriber);
            return Promise.resolve({ value: undefined, done: true });
          }
        };
      }
    }
  };
}
function buildContributionsJson(loaded) {
  const entries = [];
  for (const entry of loaded) {
    for (const topicContribution of entry.manifest.topicContributions ?? []) {
      entries.push({ pluginId: entry.pluginId, topicContribution });
    }
  }
  return JSON.stringify(entries);
}
function resolveLayoutForMode(app, modeId) {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named)
      return named.layout;
  }
  return app.defaultLayout;
}
function expandPluginRegistry(plugins, primaryPluginId, hostMode = false) {
  if (hostMode || !primaryPluginId)
    return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry]));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  const selected = new Map;
  const queue = [...primaryEntries, ...contributorEntries];
  for (const entry of queue)
    selected.set(entry.pluginId, entry);
  for (let index = 0;index < queue.length; index++) {
    const entry = queue[index];
    for (const dependency of entry.dependencies ?? []) {
      if (selected.has(dependency.pluginId))
        continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry)
        continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}
async function ensureContributorInstance(pluginId, appId, context) {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null)
    return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle)
    return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}
async function resolveExternalSlots(node, context) {
  if (node.component.type === "extension") {
    const [pluginId = "", appId = pluginId] = node.component.extension.split("/");
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
    }
    return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
  }
  if (node.children.length === 0)
    return node;
  const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child, context)));
  return children.every((child, index) => child === node.children[index]) ? node : { ...node, children };
}
function dependsOnToPluginDependencies(dependsOn) {
  return dependsOn?.map((pluginId) => ({ pluginId, version: "*" }));
}
function dialectEquals(a, b) {
  return a.artifactKind === b.artifactKind && a.standard === b.standard && a.subset === b.subset;
}
function dialectCoordinate(dialect) {
  return `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
}
function parseDialectCoordinate(coordinate) {
  const atIndex = coordinate.indexOf("@");
  if (atIndex < 0)
    throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} missing '@'`);
  const kind = coordinate.slice(0, atIndex);
  const rest = coordinate.slice(atIndex + 1);
  const slashIndex = rest.lastIndexOf("/");
  if (slashIndex < 0)
    throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} missing '/'`);
  const standard = rest.slice(0, slashIndex);
  const subset = rest.slice(slashIndex + 1);
  if (kind === "" || standard === "" || subset === "")
    throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} has an empty component`);
  return { artifactKind: kind, standard, subset };
}
function appRefEquals(a, b) {
  return a.pluginId === b.pluginId && a.appId === b.appId;
}
function surfaceAppId(dialect, role) {
  return `${dialectCoordinate(dialect)}#${role}`;
}
function parseSurfaceAppId(id) {
  const hashIndex = id.lastIndexOf("#");
  if (hashIndex < 0)
    throw new Error(`surface id ${JSON.stringify(id)} missing '#'`);
  const coordinate = id.slice(0, hashIndex);
  const roleStr = id.slice(hashIndex + 1);
  const dialect = parseDialectCoordinate(coordinate);
  if (roleStr !== "viewer" && roleStr !== "editor") {
    throw new Error(`surface id ${JSON.stringify(id)}: unknown app role ${JSON.stringify(roleStr)}, expected "viewer" or "editor"`);
  }
  return { dialect, role: roleStr };
}
function surfaceFault(code, message, scope = {}) {
  return { origin: "framework", code, severity: "error", message, scope, retryable: false };
}
function readManifestAppSurface(app) {
  const id = app.id;
  const role = app.role;
  const dialect = app.dialect;
  if (typeof id !== "string")
    return;
  if (role !== "viewer" && role !== "editor")
    return;
  if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string")
    return;
  return { appId: id, role, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset } };
}
function coordinateRoleKey(dialect, role) {
  return `${dialectCoordinate(dialect)}#${role}`;
}

class AppRouter {
  entriesByCoordinateRole;
  dialectsByCoordinate;
  ownerByArtifactKind;
  constructor(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind) {
    this.entriesByCoordinateRole = entriesByCoordinateRole;
    this.dialectsByCoordinate = dialectsByCoordinate;
    this.ownerByArtifactKind = ownerByArtifactKind;
  }
  static build(manifests) {
    const ownerByArtifactKind = new Map;
    const seenRefs = new Set;
    const grouped = new Map;
    const ordered = [...manifests].sort((left, right) => Number((right.artifactKinds?.length ?? 0) > 0) - Number((left.artifactKinds?.length ?? 0) > 0));
    for (const manifest of ordered) {
      for (const kind of manifest.artifactKinds ?? []) {
        if (!ownerByArtifactKind.has(kind.id))
          ownerByArtifactKind.set(kind.id, manifest.pluginId);
      }
      for (const raw of manifest.apps) {
        const surface = readManifestAppSurface(raw);
        if (!surface)
          continue;
        let owner = ownerByArtifactKind.get(surface.dialect.artifactKind);
        if (owner === undefined) {
          owner = manifest.pluginId;
          ownerByArtifactKind.set(surface.dialect.artifactKind, owner);
        }
        if (owner !== manifest.pluginId) {
          const permitted = (manifest.dependencies ?? []).some((dependency) => dependency.pluginId === owner);
          if (!permitted) {
            throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.ContributionNotPermitted, `plugin ${JSON.stringify(manifest.pluginId)} contributes a surface for ${JSON.stringify(dialectCoordinate(surface.dialect))} without depending on owner ${JSON.stringify(owner)}`, { pluginId: manifest.pluginId }));
          }
        }
        const ref = { pluginId: manifest.pluginId, appId: surface.appId };
        const refKey = `${ref.pluginId} ${ref.appId}`;
        if (seenRefs.has(refKey)) {
          throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.Conflict, `AppRef {pluginId: ${JSON.stringify(ref.pluginId)}, appId: ${JSON.stringify(ref.appId)}} registered twice`, { pluginId: ref.pluginId, appId: ref.appId }));
        }
        seenRefs.add(refKey);
        const key = coordinateRoleKey(surface.dialect, surface.role);
        let group = grouped.get(key);
        if (!group) {
          group = { dialect: surface.dialect, role: surface.role, entries: [] };
          grouped.set(key, group);
        }
        group.entries.push(ref);
      }
    }
    const entriesByCoordinateRole = new Map;
    const dialectsByCoordinate = new Map;
    for (const [key, group] of grouped) {
      const owner = ownerByArtifactKind.get(group.dialect.artifactKind);
      const sorted = [...group.entries].sort((a, b) => a.pluginId === b.pluginId ? a.appId.localeCompare(b.appId) : a.pluginId.localeCompare(b.pluginId));
      const ordered2 = owner === undefined ? sorted : [...sorted.filter((ref) => ref.pluginId === owner), ...sorted.filter((ref) => ref.pluginId !== owner)];
      entriesByCoordinateRole.set(key, ordered2);
      dialectsByCoordinate.set(dialectCoordinate(group.dialect), group.dialect);
    }
    return new AppRouter(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind);
  }
  entriesFor(dialect, role) {
    return this.entriesByCoordinateRole.get(coordinateRoleKey(dialect, role)) ?? [];
  }
  ownerPluginId(artifactKind) {
    return this.ownerByArtifactKind.get(artifactKind);
  }
  ownedSurfaceGaps() {
    const gaps = [];
    for (const [coordinate, dialect] of this.dialectsByCoordinate) {
      const owner = this.ownerByArtifactKind.get(dialect.artifactKind);
      if (owner === undefined)
        continue;
      for (const role of ["viewer", "editor"]) {
        if (this.entriesFor(dialect, role).length === 0) {
          gaps.push(surfaceFault(SURFACE_FAULT_CODES.MissingOwnerSurface, `owned subset ${JSON.stringify(coordinate)} has no ${role} surface`, { pluginId: owner }));
        }
      }
    }
    return gaps;
  }
}
function ioFidelityRank(fidelity) {
  switch (fidelity) {
    case "Exact":
      return 3;
    case "Canonical":
      return 2;
    case "Semantic":
      return 1;
    case "Lossy":
      return 0;
  }
}
function ioFidelityFromRank(rank) {
  if (rank >= 3)
    return "Exact";
  if (rank === 2)
    return "Canonical";
  if (rank === 1)
    return "Semantic";
  return "Lossy";
}
function ioConfidenceRank(confidence) {
  switch (confidence) {
    case "None":
      return 0;
    case "Low":
      return 1;
    case "Medium":
      return 2;
    case "High":
      return 3;
  }
}
function ioConfidenceFromRank(rank) {
  if (rank >= 3)
    return "High";
  if (rank === 2)
    return "Medium";
  if (rank === 1)
    return "Low";
  return "None";
}
function ioEntryKey(from, into) {
  return `${dialectCoordinate(from)}->${dialectCoordinate(into)}`;
}

class IoEntryGraph {
  ownerByEntry;
  constructor(ownerByEntry) {
    this.ownerByEntry = ownerByEntry;
  }
  static build(plugins) {
    const ownerByEntry = new Map;
    for (const plugin of plugins) {
      for (const descriptor of plugin.entries) {
        const key = ioEntryKey(descriptor.from, descriptor.into);
        const existing = ownerByEntry.get(key);
        if (existing) {
          if (existing.pluginId !== plugin.pluginId) {
            throw new Error(`io entry route conflict for ${key}: ${JSON.stringify(existing.pluginId)} already owns it; ${JSON.stringify(plugin.pluginId)} cannot replace it`);
          }
          continue;
        }
        ownerByEntry.set(key, { pluginId: plugin.pluginId, descriptor });
      }
    }
    return new IoEntryGraph(ownerByEntry);
  }
  route(from, into, maxHops = 3) {
    const bound = Math.min(maxHops, 3);
    if (bound <= 0)
      throw new Error(`io_routes ${dialectCoordinate(from)} -> ${dialectCoordinate(into)}: max hops clamped to 0`);
    const candidates = [];
    const path = [];
    const visited = new Set([dialectCoordinate(from)]);
    const walk = (current, remainingHops) => {
      if (remainingHops === 0)
        return;
      for (const { descriptor } of this.ownerByEntry.values()) {
        if (!dialectEquals(descriptor.from, current))
          continue;
        const nextCoordinate = dialectCoordinate(descriptor.into);
        if (visited.has(nextCoordinate))
          continue;
        path.push(descriptor);
        if (dialectEquals(descriptor.into, into)) {
          candidates.push([...path]);
        } else {
          visited.add(nextCoordinate);
          walk(descriptor.into, remainingHops - 1);
          visited.delete(nextCoordinate);
        }
        path.pop();
      }
    };
    walk(from, bound);
    if (candidates.length === 0)
      throw new Error(`no io route from ${dialectCoordinate(from)} to ${dialectCoordinate(into)} within ${bound} hops`);
    const rank = (hops) => {
      const minFidelity = Math.min(...hops.map((hop) => ioFidelityRank(hop.fidelity)));
      const joined = hops.map((hop) => dialectCoordinate(hop.into)).join(",");
      return [-minFidelity, hops.length, joined];
    };
    const sorted = [...candidates].sort((a, b) => {
      const [aInverseFidelity, aLength, aJoined] = rank(a);
      const [bInverseFidelity, bLength, bJoined] = rank(b);
      if (aInverseFidelity !== bInverseFidelity)
        return aInverseFidelity - bInverseFidelity;
      if (aLength !== bLength)
        return aLength - bLength;
      return aJoined.localeCompare(bJoined);
    });
    const best = sorted[0];
    const minFidelityRank = Math.min(...best.map((hop) => ioFidelityRank(hop.fidelity)));
    return { hops: best, fidelity: ioFidelityFromRank(minFidelityRank) };
  }
  ownerOf(from, into) {
    return this.ownerByEntry.get(ioEntryKey(from, into))?.pluginId;
  }
  carrierEntries(carrier) {
    const found = [];
    for (const { pluginId, descriptor } of this.ownerByEntry.values()) {
      if (dialectEquals(descriptor.from, carrier) && descriptor.sniffs)
        found.push({ into: descriptor.into, pluginId });
    }
    return found;
  }
}
async function ioRun(graph, callingPluginId, from, into, payload, runHop, maxHops = 3) {
  const route = graph.route(from, into, maxHops);
  const hops = route.hops.map((hop) => {
    const owner = graph.ownerOf(hop.from, hop.into);
    if (owner === undefined)
      throw new Error(`io-run: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} vanished from the graph between resolve and execute`);
    if (owner === callingPluginId) {
      throw new Error(`io-run refused: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} is owned by the calling plugin ${JSON.stringify(callingPluginId)} itself — executing it would re-enter that plugin's own in-flight worker call`);
    }
    return { hop, owner };
  });
  let current = payload;
  for (const { hop, owner } of hops) {
    current = await runHop(owner, hop.from, hop.into, current);
  }
  return current;
}
async function ioIdentify(graph, callingPluginId, carrier, payload, sniffHop) {
  const candidates = graph.carrierEntries(carrier).filter((entry) => entry.pluginId !== callingPluginId);
  const found = [];
  for (const { into, pluginId } of candidates) {
    const confidence = ioConfidenceFromRank(await sniffHop(pluginId, carrier, into, payload));
    if (confidence !== "None")
      found.push([into, confidence]);
  }
  found.sort((a, b) => {
    const rankDiff = ioConfidenceRank(b[1]) - ioConfidenceRank(a[1]);
    if (rankDiff !== 0)
      return rankDiff;
    return dialectCoordinate(a[0]).localeCompare(dialectCoordinate(b[0]));
  });
  return found;
}
function decodeOpeningPreferences(value) {
  if (!value || typeof value !== "object" || !("defaults" in value) || !Array.isArray(value.defaults))
    return;
  const defaults = [];
  for (const raw of value.defaults) {
    if (!raw || typeof raw !== "object")
      return;
    const record = raw;
    const dialect = record.dialect;
    const role = record.role;
    const app = record.app;
    if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string")
      return;
    if (role !== "viewer" && role !== "editor")
      return;
    if (!app || typeof app.pluginId !== "string" || typeof app.appId !== "string")
      return;
    defaults.push({ dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, role, app: { pluginId: app.pluginId, appId: app.appId } });
  }
  return { defaults };
}
function decodeOpeningConfigMutation(value) {
  if (!value || typeof value !== "object")
    return;
  const record = value;
  const dialect = record.dialect;
  const role = record.role;
  if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string")
    return;
  if (role !== "viewer" && role !== "editor")
    return;
  const typedDialect = { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset };
  if (record.mutation === "setDefaultApp") {
    const app = record.app;
    if (!app || typeof app.pluginId !== "string" || typeof app.appId !== "string")
      return;
    return { mutation: "setDefaultApp", dialect: typedDialect, role, app: { pluginId: app.pluginId, appId: app.appId } };
  }
  if (record.mutation === "clearDefaultApp") {
    return { mutation: "clearDefaultApp", dialect: typedDialect, role };
  }
  return;
}
function applyOpeningConfigMutation(base, mutation) {
  const defaults = base.defaults.filter((entry) => !(dialectEquals(entry.dialect, mutation.dialect) && entry.role === mutation.role));
  if (mutation.mutation === "setDefaultApp")
    defaults.push({ dialect: mutation.dialect, role: mutation.role, app: mutation.app });
  return { defaults };
}
function foldOpeningPreferences(ops, base = EMPTY_OPENING_PREFERENCES) {
  return ops.reduce(applyOpeningConfigMutation, base);
}
function resolveOpeningApp(router, dialect, role, prefs) {
  const entries = router.entriesFor(dialect, role);
  const pinned = prefs.defaults.find((entry) => dialectEquals(entry.dialect, dialect) && entry.role === role);
  if (pinned && entries.some((ref) => appRefEquals(ref, pinned.app)))
    return pinned.app;
  const owner = router.ownerPluginId(dialect.artifactKind);
  if (owner !== undefined) {
    const ownerEntry = entries.find((ref) => ref.pluginId === owner);
    if (ownerEntry)
      return ownerEntry;
  }
  const first = entries[0];
  if (first)
    return first;
  throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.UnknownDialect, `no surface registered for ${dialectCoordinate(dialect)}#${role}`, {}));
}
function severityAsU8(severity) {
  return SEVERITY_ORDER.indexOf(severity);
}
function severityFromU8(value) {
  return SEVERITY_ORDER[value];
}
function resolveUiDirtyScope(scope) {
  return scope ?? { kind: "full" };
}
function parseInvocationResponse(raw) {
  try {
    const parsed = JSON.parse(raw);
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.mutations)) {
      return parsed;
    }
  } catch {}
  return EMPTY_INVOCATION_RESPONSE;
}
function mergePolicyAsU8(policy) {
  return MERGE_POLICY_ORDER.indexOf(policy);
}
function mergePolicyFromU8(value) {
  return MERGE_POLICY_ORDER[value];
}
function conflictResolutionAsU8(resolution) {
  return CONFLICT_RESOLUTION_ORDER.indexOf(resolution);
}
function conflictResolutionFromU8(value) {
  return CONFLICT_RESOLUTION_ORDER[value];
}
function relayPluginBackboneOutbound(uri, message) {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}
function pushMainThreadPluginBackboneInbound(uri, messages) {
  const bridge = globalThis;
  const queue = bridge.__semioBackboneInbound ?? new Map;
  queue.set(uri, [...queue.get(uri) ?? [], ...messages]);
  bridge.__semioBackboneInbound = queue;
}
function postPluginBackboneInbound(pluginId, uri, messages) {
  pushMainThreadPluginBackboneInbound(uri, messages);
}
function pluginBackboneDocumentIdFromUri(uri) {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}
function registerPluginBackboneRoute(documentId, relay) {
  pluginBackboneRoutes.set(documentId, relay);
  return () => {
    if (pluginBackboneRoutes.get(documentId) === relay)
      pluginBackboneRoutes.delete(documentId);
  };
}
function intersectCapabilityGrants(granted, requested) {
  const grantedIds = new Set(granted.map((grant) => grant.id));
  return requested.filter((request) => grantedIds.has(request.id));
}
function defaultGuestSlimAssetFetcher(moduleUrl) {
  const vendorUrl = moduleUrl.split(/[?#]/)[0].replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
  return fetch(vendorUrl).then((response) => {
    if (!response.ok)
      throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  }).then((buffer) => [["guestslim-typst-fonts", buffer]]);
}
function clampResidentActors(value) {
  return Math.min(MAX_MAX_RESIDENT_ACTORS, Math.max(MIN_MAX_RESIDENT_ACTORS, Math.round(value)));
}
function defaultMemoryProbe() {
  const nav = globalThis.navigator;
  const perf = globalThis.performance;
  return { deviceMemoryGiB: nav?.deviceMemory, jsHeapSizeLimitBytes: perf?.memory?.jsHeapSizeLimit };
}
function residentActorCapFromMemory(reading, fallback = DEFAULT_MAX_RESIDENT_ACTORS) {
  if (typeof reading.deviceMemoryGiB === "number" && reading.deviceMemoryGiB > 0)
    return clampResidentActors(reading.deviceMemoryGiB * RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB);
  if (typeof reading.jsHeapSizeLimitBytes === "number" && reading.jsHeapSizeLimitBytes > 0)
    return clampResidentActors(reading.jsHeapSizeLimitBytes / BYTES_PER_RESIDENT_ACTOR);
  return fallback;
}

class ActivationRegistry {
  manifests = new Map;
  resident = new Map;
  residencyOrder = [];
  checkpoints = new Map;
  actorPlugin = new Map;
  actorGeneration = new Map;
  extensionsByParent = new Map;
  extensionChildren = new Map;
  shardClient;
  defaultBudget;
  maxResidentActors;
  fetchAssets;
  assetsPromise = null;
  now;
  lastRuntimeMetricsPublishMs = null;
  turnScheduler;
  onTurnResult;
  stopMetricsPublisher;
  metricsBus = new EventTarget;
  constructor(options) {
    this.shardClient = options.shardClient;
    this.defaultBudget = options.defaultBudget;
    this.maxResidentActors = options.maxResidentActors ?? residentActorCapFromMemory((options.memoryProbe ?? defaultMemoryProbe)());
    this.fetchAssets = options.fetchAssets ?? defaultGuestSlimAssetFetcher;
    this.now = options.now ?? (() => Date.now());
    this.onTurnResult = options.onTurnResult ?? (() => {});
    const onTurnError = options.onTurnError ?? ((actorId, error) => console.error(`[DEBUG] ActivationRegistry: turn failed for ${actorId}`, error));
    this.turnScheduler = new TurnScheduler({
      mailboxCapacity: options.turnMailboxCapacity ?? DEFAULT_TURN_MAILBOX_CAPACITY,
      budgetFor: () => this.defaultBudget,
      runTurn: (actorId, payload, budget) => this.runQueuedTurn(actorId, payload, budget),
      onTurnError
    });
    this.stopMetricsPublisher = options.autoStartMetricsPublisher === true ? this.startRuntimeMetricsPublisher((topic, snapshot) => this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot }))) : () => {};
  }
  registerManifest(entry) {
    this.manifests.set(entry.pluginId, entry);
  }
  registerCatalog(catalog) {
    for (const target of catalog.plugins)
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.moduleUrl(target.pluginId, target.wasmOut), caps: [] });
    for (const target of catalog.extensions) {
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut), caps: [] });
      const parentId = target.dependsOn?.[0];
      if (!parentId)
        continue;
      const siblings = this.extensionsByParent.get(parentId) ?? [];
      siblings.push(target.pluginId);
      this.extensionsByParent.set(parentId, siblings);
    }
  }
  manifestFor(pluginId) {
    return this.manifests.get(pluginId);
  }
  loadAssets(moduleUrl) {
    this.assetsPromise ??= this.fetchAssets(moduleUrl).catch((error) => {
      console.warn("[DEBUG] ActivationRegistry: guestSlim asset fetch failed; affected actors render without it", error);
      this.assetsPromise = null;
      return [];
    });
    return this.assetsPromise;
  }
  markResident(actorId, pluginId) {
    this.resident.set(actorId, { actorId, pluginId });
    this.actorPlugin.set(actorId, pluginId);
    this.touch(actorId);
  }
  touch(actorId) {
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    this.residencyOrder.push(actorId);
  }
  async activate(pluginId, actorId, _reason) {
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.activate: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    this.markResident(actorId, pluginId);
    await this.activateExtensionsOf(pluginId, actorId);
  }
  async activateExtensionsOf(pluginId, parentActorId) {
    const extensionIds = this.extensionsByParent.get(pluginId);
    if (!extensionIds || extensionIds.length === 0)
      return;
    const parentCaps = this.manifests.get(pluginId)?.caps ?? [];
    const children = [];
    for (const extensionId of extensionIds) {
      const manifest = this.manifests.get(extensionId);
      if (!manifest) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} has no registered manifest, skipping`);
        continue;
      }
      const childActorId = `${parentActorId}::${extensionId}`;
      try {
        const scopedCaps = intersectCapabilityGrants(parentCaps, manifest.caps);
        const assets = await this.loadAssets(manifest.moduleUrl);
        await this.shardClient.activate(childActorId, manifest.moduleUrl, scopedCaps, this.defaultBudget, assets);
        this.markResident(childActorId, extensionId);
        children.push(childActorId);
      } catch (error) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} failed to activate`, error);
      }
    }
    if (children.length > 0)
      this.extensionChildren.set(parentActorId, children);
  }
  enqueueTurn(actorId, lane, events, options) {
    const generation = this.actorGeneration.get(actorId) ?? 0;
    return this.turnScheduler.enqueue(actorId, { lane, coalesce: options?.coalesce, payload: { events, generation } });
  }
  async runQueuedTurn(actorId, payload, budget) {
    const currentGeneration = this.actorGeneration.get(actorId) ?? 0;
    if (payload.generation !== currentGeneration) {
      console.warn(`[DEBUG] ActivationRegistry: dropping turn for ${actorId} queued against generation ${payload.generation}, now at ${currentGeneration} (restored in between)`);
      return;
    }
    this.touch(actorId);
    const result = await this.shardClient.turn(actorId, payload.events, budget);
    this.onTurnResult(actorId, result);
  }
  async evictForMemoryPressure() {
    while (this.residencyOrder.length >= this.maxResidentActors) {
      await this.suspend(this.residencyOrder[0]);
    }
  }
  async suspend(actorId) {
    if (!this.resident.has(actorId))
      return;
    this.turnScheduler.cancelQueued(actorId);
    await this.suspendExtensionsOf(actorId);
    const checkpoint = await this.shardClient.checkpoint(actorId);
    this.checkpoints.set(actorId, checkpoint);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  async suspendExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children)
      await this.suspend(child);
  }
  async resume(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      throw new Error(`[DEBUG] ActivationRegistry.resume: unknown actor ${actorId} (never activated)`);
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.resume: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    const checkpoint = this.checkpoints.get(actorId);
    if (checkpoint)
      await this.shardClient.restore(actorId, checkpoint);
    this.markResident(actorId, pluginId);
    await this.resumeExtensionsOf(actorId);
  }
  async resumeExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children) {
      if (this.checkpoints.has(child) && !this.resident.has(child))
        await this.resume(child);
    }
  }
  async restoreActor(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      return;
    this.actorGeneration.set(actorId, (this.actorGeneration.get(actorId) ?? 0) + 1);
    this.turnScheduler.cancelQueued(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    await this.resume(actorId);
  }
  async restoreActors(actorIds) {
    await Promise.all(actorIds.map((actorId) => this.restoreActor(actorId).catch((error) => console.error(`[DEBUG] ActivationRegistry.restoreActors: failed to restore ${actorId}`, error))));
  }
  handleShardLost = (_shardIndex, actorIds) => {
    this.restoreActors(actorIds);
  };
  cancel(actorId) {
    if (!this.actorPlugin.has(actorId))
      return;
    const children = this.extensionChildren.get(actorId);
    if (children) {
      for (const child of children)
        this.cancel(child);
      this.extensionChildren.delete(actorId);
    }
    this.turnScheduler.teardownActor(actorId);
    this.actorGeneration.delete(actorId);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    this.checkpoints.delete(actorId);
    this.actorPlugin.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  isResident(actorId) {
    return this.resident.has(actorId);
  }
  dispose() {
    this.stopMetricsPublisher();
  }
  runtimeMetricsActorRows() {
    return [...this.actorPlugin.entries()].map(([actorId, pluginId]) => ({ actorId, pluginId, resident: this.resident.has(actorId), shard: this.shardClient.shardIndexFor(actorId) ?? null }));
  }
  runtimeMetricsSnapshot(sampledAtMs = this.now()) {
    return { actors: this.runtimeMetricsActorRows(), shards: this.shardClient.shardMetricsSamples(sampledAtMs), sampledAtMs };
  }
  startRuntimeMetricsPublisher(sink) {
    const interval = setInterval(() => {
      const nowMs = this.now();
      if (!runtimeMetricsDue(this.lastRuntimeMetricsPublishMs, nowMs))
        return;
      this.lastRuntimeMetricsPublishMs = nowMs;
      sink("os.runtime.metrics", this.runtimeMetricsSnapshot(nowMs));
    }, RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
    return () => clearInterval(interval);
  }
}
function runtimeMetricsDue(lastPublishedMs, nowMs) {
  if (lastPublishedMs === null)
    return true;
  return nowMs - lastPublishedMs >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
}
function createDevPluginSource(registry) {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry]));
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry)
        throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined")
        return () => {};
      const source = new EventSource(PLUGIN_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data));
        } catch (error) {
          console.warn(`[DEBUG] plugin source "dev" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    }
  };
}
function createExtensionSource(catalog) {
  const registry = catalog.extensions.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn)
  }));
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry]));
  return {
    id: "extensions",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry)
        throw new Error(`[DEBUG] plugin source "extensions" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined")
        return () => {};
      const source = new EventSource(EXTENSION_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data));
        } catch (error) {
          console.warn(`[DEBUG] plugin source "extensions" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    }
  };
}
function multiplexPluginSources(...sources) {
  if (sources.length === 0)
    throw new Error("[DEBUG] multiplexPluginSources requires at least one source");
  if (sources.length === 1)
    return sources[0];
  return {
    id: sources.map((source) => source.id).join("+"),
    async list() {
      const merged = new Map;
      for (const entries of await Promise.all(sources.map((source) => source.list()))) {
        for (const entry of entries)
          merged.set(entry.pluginId, entry);
      }
      return [...merged.values()];
    },
    moduleUrl(pluginId, rebuiltAt) {
      for (const source of sources) {
        try {
          return source.moduleUrl(pluginId, rebuiltAt);
        } catch {
          continue;
        }
      }
      throw new Error(`[DEBUG] multiplexed plugin sources have no registry entry for ${pluginId}`);
    },
    subscribe(listener) {
      const unsubscribes = sources.map((source) => source.subscribe(listener));
      return () => {
        for (const unsubscribe of unsubscribes)
          unsubscribe();
      };
    }
  };
}
function findPlaygroundVariant(catalog, playgroundPluginId) {
  return catalog.playgrounds.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}
function resolvePluginRegistryId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.pluginId ?? playgroundPluginId;
}
function resolvePlaygroundDefaultAppId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.app;
}
function resolvePlaygroundBoot(catalog, variant, session) {
  const defaultAppId = resolvePlaygroundDefaultAppId(catalog, variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins, dependencyErrors: [] };
  }
  const registryPluginId = resolvePluginRegistryId(catalog, variant);
  const hostMode = resolvePluginHostConfig(catalog, variant) !== undefined;
  const catalogPlugins = [...catalog.plugins, ...catalog.extensions].map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: target.role === "extension" ? catalog.extensionModuleUrl(target.pluginId, target.wasmOut) : catalog.moduleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn)
  }));
  const expanded = expandPluginRegistry(catalogPlugins, hostMode ? undefined : registryPluginId, hostMode);
  const { order, errors } = orderPluginRegistryEntries(expanded);
  if (errors.length > 0) {
    for (const error of errors)
      console.error(`[DEBUG] resolvePlaygroundBoot(${variant}): ${pluginGraphErrorMessage(error, "en")}`);
  }
  return {
    variant,
    defaultAppId,
    plugins: order,
    dependencyErrors: errors
  };
}
function resolvePluginHostConfig(catalog, playgroundPluginId) {
  const registryId = resolvePluginRegistryId(catalog, playgroundPluginId);
  return catalog.hosts.find((entry) => entry.pluginId === registryId);
}
function parseVersion(raw) {
  if (!raw)
    return null;
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match)
    return null;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]) };
}
function compareVersions(a, b) {
  if (a.major !== b.major)
    return a.major - b.major;
  if (a.minor !== b.minor)
    return a.minor - b.minor;
  return a.patch - b.patch;
}
function parseVersionReq(raw) {
  const trimmed = raw.trim();
  if (trimmed === "*")
    return { kind: "any" };
  const opMatch = /^(=|\^|~|>=)(\d+\.\d+\.\d+)$/.exec(trimmed);
  if (!opMatch)
    return null;
  const version = parseVersion(opMatch[2]);
  if (!version)
    return null;
  switch (opMatch[1]) {
    case "=":
      return { kind: "exact", version };
    case "^":
      return { kind: "caret", version };
    case "~":
      return { kind: "tilde", version };
    case ">=":
      return { kind: "atLeast", version };
    default:
      return null;
  }
}
function versionSatisfies(actual, requirement) {
  const req = parseVersionReq(requirement);
  if (!req)
    return false;
  if (req.kind === "any")
    return true;
  const version = parseVersion(actual);
  if (!version)
    return false;
  if (req.kind === "exact")
    return compareVersions(version, req.version) === 0;
  if (req.kind === "atLeast")
    return compareVersions(version, req.version) >= 0;
  if (req.kind === "tilde") {
    return version.major === req.version.major && version.minor === req.version.minor && version.patch >= req.version.patch;
  }
  if (compareVersions(version, req.version) < 0)
    return false;
  if (req.version.major > 0)
    return version.major === req.version.major;
  if (req.version.minor > 0)
    return version.major === 0 && version.minor === req.version.minor;
  return version.major === 0 && version.minor === 0 && version.patch === req.version.patch;
}
function validatePluginDependencyGraph(nodes) {
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const errors = [];
  for (const node of nodes) {
    for (const dependency of node.dependencies ?? []) {
      const target = byId.get(dependency.pluginId);
      if (!target) {
        errors.push({ code: "transaction.dependency-missing", pluginId: node.pluginId, dependsOn: dependency.pluginId });
        continue;
      }
      if (target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {
        errors.push({ code: "transaction.version-mismatch", pluginId: node.pluginId, dependsOn: dependency.pluginId, required: dependency.version, actual: target.version });
      }
    }
  }
  return errors;
}
function findCycleMembers(byId, leftover) {
  const visiting = new Set;
  const visited = new Set;
  const stack = [];
  let cycle = null;
  function visit(id) {
    if (cycle || !leftover.has(id) || visited.has(id))
      return;
    if (visiting.has(id)) {
      const start = stack.indexOf(id);
      cycle = stack.slice(start);
      return;
    }
    visiting.add(id);
    stack.push(id);
    for (const dependency of byId.get(id)?.dependencies ?? []) {
      if (leftover.has(dependency.pluginId))
        visit(dependency.pluginId);
      if (cycle)
        return;
    }
    stack.pop();
    visiting.delete(id);
    visited.add(id);
  }
  for (const id of [...leftover].sort()) {
    visit(id);
    if (cycle)
      break;
  }
  return cycle ?? [...leftover].sort();
}
function resolvePluginLoadOrder(nodes) {
  const structural = validatePluginDependencyGraph(nodes);
  if (structural.length > 0)
    return { order: [], errors: structural };
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const indegree = new Map;
  const dependents = new Map;
  for (const node of nodes) {
    indegree.set(node.pluginId, indegree.get(node.pluginId) ?? 0);
    for (const dependency of node.dependencies ?? []) {
      indegree.set(node.pluginId, (indegree.get(node.pluginId) ?? 0) + 1);
      const list = dependents.get(dependency.pluginId) ?? [];
      list.push(node.pluginId);
      dependents.set(dependency.pluginId, list);
    }
  }
  const order = [];
  const remaining = new Map(indegree);
  const queue = [...indegree.entries()].filter(([, count]) => count === 0).map(([id]) => id);
  while (queue.length > 0) {
    queue.sort();
    const id = queue.shift();
    order.push(id);
    for (const dependent of dependents.get(id) ?? []) {
      const next = (remaining.get(dependent) ?? 0) - 1;
      remaining.set(dependent, next);
      if (next === 0)
        queue.push(dependent);
    }
  }
  if (order.length === nodes.length)
    return { order, errors: [] };
  const leftover = new Set(nodes.map((node) => node.pluginId).filter((id) => !order.includes(id)));
  return { order: [], errors: [{ code: "transaction.cycle", members: findCycleMembers(byId, leftover) }] };
}
function pluginDependents(nodes, pluginId) {
  return nodes.filter((node) => (node.dependencies ?? []).some((dependency) => dependency.pluginId === pluginId)).map((node) => node.pluginId).sort();
}

class PluginGraph {
  nodes;
  constructor(nodes) {
    this.nodes = nodes;
  }
  validate() {
    return validatePluginDependencyGraph(this.nodes);
  }
  loadOrder() {
    return resolvePluginLoadOrder(this.nodes);
  }
  dependents(pluginId) {
    return pluginDependents(this.nodes, pluginId);
  }
  canUnload(pluginId, loadedIds) {
    return this.dependents(pluginId).every((dependent) => !loadedIds.has(dependent));
  }
}
function orderPluginRegistryEntries(entries) {
  const nodes = entries.map((entry) => ({ pluginId: entry.pluginId, dependencies: entry.dependencies }));
  const { order, errors } = new PluginGraph(nodes).loadOrder();
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  if (errors.length === 0) {
    return { order: order.map((id) => byId.get(id)).filter((entry) => entry !== undefined), errors: [] };
  }
  const blocked = new Set(errors.flatMap((error) => error.code === "transaction.cycle" ? error.members : [error.pluginId]));
  const remaining = entries.filter((entry) => !blocked.has(entry.pluginId));
  const retried = orderPluginRegistryEntries(remaining);
  return { order: retried.order, errors: [...errors, ...retried.errors] };
}
function resolveLocalizedLabel(label, locale) {
  return label[locale] ?? label.en ?? Object.values(label)[0] ?? "";
}
function pluginGraphErrorMessage(error, locale) {
  switch (error.code) {
    case "transaction.dependency-missing":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}", which is not installed.`,
        de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“, welches nicht installiert ist.`
      }, locale);
    case "transaction.version-mismatch":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}" ${error.required}, but ${error.actual} is installed.`,
        de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“ ${error.required}, installiert ist jedoch ${error.actual}.`
      }, locale);
    case "transaction.cycle":
      return resolveLocalizedLabel({
        en: `Plugin dependency cycle: ${error.members.join(" → ")}.`,
        de: `Zyklische Plugin-Abhängigkeit: ${error.members.join(" → ")}.`
      }, locale);
  }
}

class InstanceDirectory {
  byArtifactId = new Map;
  register(artifactId, ref) {
    this.byArtifactId.set(artifactId, ref);
  }
  unregister(artifactId) {
    this.byArtifactId.delete(artifactId);
  }
  resolve(artifactId) {
    return this.byArtifactId.get(artifactId);
  }
  entries() {
    return [...this.byArtifactId.entries()];
  }
}
function stableStringify(value) {
  if (value === null || typeof value !== "object")
    return JSON.stringify(value);
  if (Array.isArray(value))
    return `[${value.map(stableStringify).join(",")}]`;
  const record = value;
  const keys = Object.keys(record).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableStringify(record[key])}`).join(",")}}`;
}

class ConflictCheckedRegistry {
  entries = new Map;
  register(artifactKind, key, ownership, metadata) {
    const compositeKey = `${artifactKind} ${key}`;
    const fingerprint = stableStringify(metadata);
    const existing = this.entries.get(compositeKey);
    if (existing && existing.fingerprint !== fingerprint)
      throw new ArtifactRouterConflictError(artifactKind, key);
    this.entries.set(compositeKey, { ownership, fingerprint });
  }
  resolve(artifactKind, key) {
    return this.entries.get(`${artifactKind} ${key}`)?.ownership;
  }
}

class ArtifactMutationRouter {
  registry = new ConflictCheckedRegistry;
  registerOwner(artifactKind, mutationId) {
    this.registry.register(artifactKind, mutationId, { kind: "owner" }, { kind: "owner", artifactKind, mutationId });
  }
  registerContributed(artifactKind, contributorPluginId, ownerPluginId, metadata, contributorDependsOnOwner) {
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(contributorPluginId, ownerPluginId);
    this.registry.register(artifactKind, metadata.mutationId, { kind: "contributed", pluginId: contributorPluginId }, metadata);
  }
  resolve(artifactKind, mutationId) {
    return this.registry.resolve(artifactKind, mutationId);
  }
}

class ArtifactInferenceRouter {
  registry = new ConflictCheckedRegistry;
  dependsOn = new Map;
  registerOwner(artifactKind, inferenceSchema) {
    this.registry.register(artifactKind, inferenceSchema, { kind: "owner" }, { kind: "owner", artifactKind, inferenceSchema });
  }
  registerContributed(artifactKind, metadata, contributorDependsOnOwner) {
    if (metadata.owner !== metadata.contributor) {
      throw new Error(`[DEBUG] contributed inference owner/contributor mismatch: ${metadata.owner} !== ${metadata.contributor}`);
    }
    if (metadata.artifactKind !== artifactKind) {
      throw new Error(`[DEBUG] contributed inference artifactKind mismatch: ${metadata.artifactKind} !== ${artifactKind}`);
    }
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(metadata.contributor, artifactKind);
    this.registry.register(artifactKind, metadata.inferenceSchema, { kind: "contributed", pluginId: metadata.contributor }, metadata);
    this.dependsOn.set(`${artifactKind} ${metadata.inferenceSchema}`, metadata.dependsOn ?? []);
  }
  resolve(artifactKind, inferenceSchema) {
    return this.registry.resolve(artifactKind, inferenceSchema);
  }
  dependencyOrder() {
    const keys = [...this.dependsOn.keys()];
    const indegree = new Map(keys.map((key) => [key, 0]));
    const dependents = new Map;
    for (const key of keys) {
      for (const dependency of this.dependsOn.get(key) ?? []) {
        if (!indegree.has(dependency))
          continue;
        indegree.set(key, (indegree.get(key) ?? 0) + 1);
        const list = dependents.get(dependency) ?? [];
        list.push(key);
        dependents.set(dependency, list);
      }
    }
    const order = [];
    const remaining = new Map(indegree);
    const queue = keys.filter((key) => (indegree.get(key) ?? 0) === 0);
    while (queue.length > 0) {
      queue.sort();
      const key = queue.shift();
      order.push(key);
      for (const dependent of dependents.get(key) ?? []) {
        const next = (remaining.get(dependent) ?? 0) - 1;
        remaining.set(dependent, next);
        if (next === 0)
          queue.push(dependent);
      }
    }
    if (order.length !== keys.length) {
      const leftover = keys.filter((key) => !order.includes(key)).sort();
      throw new Error(`[DEBUG] ArtifactInferenceRouter.dependencyOrder: cycle among ${leftover.join(", ")}`);
    }
    return order;
  }
}
var defaultOsTransient, SURFACE_FAULT_CODES, CARRIER_BINARY_DIALECT, CARRIER_TEXT_DIALECT, EMPTY_OPENING_PREFERENCES, SEVERITY_ORDER, SemioFaultError, EMPTY_INVOCATION_RESPONSE, DEFAULT_MERGE_POLICY = "Normal", MERGE_POLICY_ORDER, CONFLICT_RESOLUTION_ORDER, MUTATION_APPLY_ERROR_SCHEMA, MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR, pluginBackboneRoutes, DEFAULT_MAX_RESIDENT_ACTORS = 24, MIN_MAX_RESIDENT_ACTORS = 4, MAX_MAX_RESIDENT_ACTORS = 96, RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB = 6, BYTES_PER_RESIDENT_ACTOR, DEFAULT_TURN_MAILBOX_CAPACITY = 32, RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500, PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch", EXTENSION_SOURCE_WATCH_PATH = "/extensions/watch", ArtifactRouterConflictError, ArtifactContributionNotPermittedError;
var init__component6 = __esm(() => {
  init__shard_client();
  init__turn_scheduler();
  defaultOsTransient = new OsTransient;
  if (import.meta.vitest) {
    const { describe, expect, it } = import.meta.vitest;
    describe("createTurnOutcomeBroadcast", () => {
      it("multicasts one pushed value to EVERY live subscriber, not a shared drain-once FIFO", async () => {
        const broadcast = createTurnOutcomeBroadcast();
        const iteratorA = broadcast.stream[Symbol.asyncIterator]();
        const iteratorB = broadcast.stream[Symbol.asyncIterator]();
        broadcast.push({ instanceId: 1, frames: [] });
        const [stepA, stepB] = await Promise.all([iteratorA.next(), iteratorB.next()]);
        expect(stepA).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
        expect(stepB).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
      });
      it("queues a value pushed before next() is called, and delivers queued values in push order", async () => {
        const broadcast = createTurnOutcomeBroadcast();
        const iterator = broadcast.stream[Symbol.asyncIterator]();
        broadcast.push({ instanceId: 2, frames: [] });
        broadcast.push({ instanceId: 3, frames: [] });
        expect(await iterator.next()).toEqual({ value: { instanceId: 2, frames: [] }, done: false });
        expect(await iterator.next()).toEqual({ value: { instanceId: 3, frames: [] }, done: false });
      });
      it("return() unsubscribes immediately — a later push never reaches a next() called after it", async () => {
        const broadcast = createTurnOutcomeBroadcast();
        const iterator = broadcast.stream[Symbol.asyncIterator]();
        expect(await iterator.return?.()).toEqual({ value: undefined, done: true });
        const pending = iterator.next();
        broadcast.push({ instanceId: 4, frames: [] });
        const raceResult = await Promise.race([pending.then(() => "resolved"), Promise.resolve("not-yet")]);
        expect(raceResult).toBe("not-yet");
      });
      it("complete() force-closes every still-live subscriber at once", async () => {
        const broadcast = createTurnOutcomeBroadcast();
        const iteratorA = broadcast.stream[Symbol.asyncIterator]();
        const iteratorB = broadcast.stream[Symbol.asyncIterator]();
        const pendingA = iteratorA.next();
        const pendingB = iteratorB.next();
        broadcast.complete();
        expect(await pendingA).toEqual({ value: undefined, done: true });
        expect(await pendingB).toEqual({ value: undefined, done: true });
      });
    });
  }
  SURFACE_FAULT_CODES = {
    ViewerReadOnly: "viewer.read-only",
    UnknownDialect: "surface.unknown-dialect",
    ContributionNotPermitted: "surface.contribution-not-permitted",
    Conflict: "surface.conflict",
    MissingOwnerSurface: "surface.missing-owner-surface"
  };
  CARRIER_BINARY_DIALECT = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
  CARRIER_TEXT_DIALECT = { artifactKind: "s.stdio.txt", standard: "utf-8", subset: "*" };
  EMPTY_OPENING_PREFERENCES = { defaults: [] };
  SEVERITY_ORDER = ["info", "warning", "error", "fatal"];
  SemioFaultError = class SemioFaultError extends Error {
    fault;
    constructor(fault) {
      super(fault.message);
      this.name = "SemioFaultError";
      this.fault = fault;
    }
  };
  EMPTY_INVOCATION_RESPONSE = {
    output: null,
    mutations: [],
    inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }
  };
  MERGE_POLICY_ORDER = ["LaissezFaire", "Normal", "Vigilant"];
  CONFLICT_RESOLUTION_ORDER = ["accept", "discard"];
  MUTATION_APPLY_ERROR_SCHEMA = {
    type: "object",
    additionalProperties: false,
    required: ["code", "message"],
    properties: {
      code: { type: "string" },
      message: { type: "string" },
      target: { type: "array", items: { type: "string" } }
    }
  };
  MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR = {
    json: '{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}',
    value: {
      code: "mutation.apply.invalid-index",
      message: "index 4 exceeds length 2",
      target: ["slides", "4"]
    }
  };
  globalThis.__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;
  pluginBackboneRoutes = new Map;
  BYTES_PER_RESIDENT_ACTOR = 64 * 1024 * 1024;
  if (import.meta.vitest) {
    let createAutoReplyWorker = function() {
      const worker = {
        postMessage: (message) => {
          const requestId = message.requestId;
          if (requestId)
            queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
        },
        terminate: () => {},
        onmessage: null,
        onerror: null
      };
      return worker;
    }, fakeShardClient = function(shardCount = 1) {
      return new ShardClient({ shardCount, createWorker: () => createAutoReplyWorker() });
    }, catalogWithOneExtension = function() {
      return {
        plugins: [{ pluginId: "p1", wasmOut: "p1.wasm", role: "plugin", contributes: [], consumes: [] }],
        extensions: [{ pluginId: "p1-ext", wasmOut: "p1-ext.wasm", role: "extension", contributes: [], consumes: [], dependsOn: ["p1"] }],
        hosts: [],
        playgrounds: [],
        moduleUrl: (pluginId, wasmOut) => `https://x/${pluginId}/${wasmOut}`,
        extensionModuleUrl: (pluginId, wasmOut) => `https://x/ext/${pluginId}/${wasmOut}`
      };
    };
    const { describe, expect, it, vi } = import.meta.vitest;
    const BUDGET_FIXTURE = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
    async function flushMicrotasks(n = 10) {
      for (let i = 0;i < n; i += 1)
        await Promise.resolve();
    }
    describe("ActivationRegistry.runtimeMetricsActorRows / runtimeMetricsSnapshot", () => {
      it("rows cover both resident and suspended actors, never activated-and-forgotten ones", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 500, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        const rows = registry.runtimeMetricsActorRows();
        expect(rows).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
        await registry.suspend("actor-1");
        const afterSuspend = registry.runtimeMetricsActorRows();
        expect(afterSuspend).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
      });
      it("snapshot combines actor rows with ShardClient.shardMetricsSamples at the given clock reading", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 999, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        const snapshot = registry.runtimeMetricsSnapshot(1000);
        expect(snapshot.sampledAtMs).toBe(1000);
        expect(snapshot.actors).toHaveLength(1);
        expect(snapshot.shards).toEqual(shardClient.shardMetricsSamples(1000));
      });
    });
    describe("runtimeMetricsDue", () => {
      it("gates at the 500ms / 2Hz interval, always due on the first call", () => {
        expect(runtimeMetricsDue(null, 0)).toBe(true);
        expect(runtimeMetricsDue(1000, 1200)).toBe(false);
        expect(runtimeMetricsDue(1000, 1500)).toBe(true);
      });
    });
    describe("ActivationRegistry.startRuntimeMetricsPublisher", () => {
      it("calls the sink with the os.runtime.metrics topic at the 2Hz interval, and stop() cancels it", () => {
        vi.useFakeTimers();
        try {
          const shardClient = fakeShardClient();
          const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
          const calls = [];
          const stop = registry.startRuntimeMetricsPublisher((topic, snapshot) => calls.push({ topic, snapshot }));
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
          expect(calls).toHaveLength(1);
          expect(calls[0].topic).toBe("os.runtime.metrics");
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
          expect(calls).toHaveLength(2);
          stop();
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 3);
          expect(calls).toHaveLength(2);
        } finally {
          vi.useRealTimers();
        }
      });
    });
    describe("ActivationRegistry.cancel", () => {
      it("disposes the worker-side instance and forgets the actor entirely — resume() afterward throws unknown actor", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        expect(registry.isResident("actor-1")).toBe(true);
        registry.cancel("actor-1");
        expect(registry.isResident("actor-1")).toBe(false);
        expect(registry.runtimeMetricsActorRows()).toEqual([]);
        expect(shardClient.shardIndexFor("actor-1")).toBeUndefined();
        await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
      });
      it("is a no-op for an actor this registry never activated", () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        expect(() => registry.cancel("ghost")).not.toThrow();
      });
      it("cancelling a suspended (non-resident but still tracked) actor still forgets it", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        await registry.suspend("actor-1");
        expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
        registry.cancel("actor-1");
        expect(registry.runtimeMetricsActorRows()).toEqual([]);
      });
    });
    describe("intersectCapabilityGrants", () => {
      it("keeps only requested grants the parent's own granted set also carries, matched by id", () => {
        const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
        const granted = [grant("fs.read"), grant("net.fetch")];
        const requested = [grant("fs.read"), grant("fs.admin")];
        expect(intersectCapabilityGrants(granted, requested).map((g) => g.id)).toEqual(["fs.read"]);
      });
      it("is empty when the parent holds nothing, never escalates an ungranted request", () => {
        const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
        expect(intersectCapabilityGrants([], [grant("fs.admin")])).toEqual([]);
      });
    });
    describe("ActivationRegistry extension cascade (registerCatalog)", () => {
      it("activate() cascades to every registered extension of the plugin, under a deterministic child actorId", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerCatalog(catalogWithOneExtension());
        await registry.activate("p1", "actor-1", "manual");
        expect(registry.isResident("actor-1")).toBe(true);
        expect(registry.isResident("actor-1::p1-ext")).toBe(true);
        const rows = registry.runtimeMetricsActorRows();
        expect(rows).toContainEqual({ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 });
        expect(rows).toContainEqual({ actorId: "actor-1::p1-ext", pluginId: "p1-ext", resident: true, shard: 0 });
      });
      it("a plugin with no registered extensions activates with no cascade side effects", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
      });
      it("suspend() cascades leaves-first, resume() cascades parent-first — zero orphans either way", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerCatalog(catalogWithOneExtension());
        await registry.activate("p1", "actor-1", "manual");
        await registry.suspend("actor-1");
        expect(registry.isResident("actor-1")).toBe(false);
        expect(registry.isResident("actor-1::p1-ext")).toBe(false);
        expect(registry.runtimeMetricsActorRows().map((r) => r.actorId).sort()).toEqual(["actor-1", "actor-1::p1-ext"]);
        await registry.resume("actor-1");
        expect(registry.isResident("actor-1")).toBe(true);
        expect(registry.isResident("actor-1::p1-ext")).toBe(true);
      });
      it("cancel() on the parent takes its extension down too — permanently, zero orphans", async () => {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerCatalog(catalogWithOneExtension());
        await registry.activate("p1", "actor-1", "manual");
        registry.cancel("actor-1");
        expect(registry.runtimeMetricsActorRows()).toEqual([]);
        await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
        await expect(registry.resume("actor-1::p1-ext")).rejects.toThrow(/unknown actor/);
      });
      it("scopes an extension's activated caps to the intersection with its parent's own granted set", async () => {
        const shardClient = fakeShardClient();
        const sentCaps = new Map;
        const worker = createAutoReplyWorker();
        const originalPostMessage = worker.postMessage;
        worker.postMessage = (message) => {
          const msg = message;
          if (msg.kind === "activate" && msg.actorId)
            sentCaps.set(msg.actorId, msg.caps ?? []);
          originalPostMessage(message);
        };
        const client = new ShardClient({ shardCount: 1, createWorker: () => worker });
        const registry = new ActivationRegistry({ shardClient: client, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerCatalog(catalogWithOneExtension());
        const grant = (id) => ({ id, token: "t", scope: "s", expiresMs: null });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1/p1.wasm", caps: [grant("fs.read")] });
        registry.registerManifest({ pluginId: "p1-ext", moduleUrl: "https://x/ext/p1-ext/p1-ext.wasm", caps: [grant("fs.read"), grant("fs.admin")] });
        await registry.activate("p1", "actor-1", "manual");
        expect(sentCaps.get("actor-1::p1-ext")?.map((g) => g.id)).toEqual(["fs.read"]);
      });
    });
    describe("ActivationRegistry.enqueueTurn lane priority", () => {
      it("dispatches turns by lane priority end-to-end through the registry, not enqueue order", async () => {
        const shardClient = fakeShardClient();
        const order = [];
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: (actorId) => order.push(actorId) });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "low", "manual");
        await registry.activate("p1", "high", "manual");
        await registry.activate("p1", "mid", "manual");
        registry.enqueueTurn("low", "Background", []);
        registry.enqueueTurn("high", "Interactive", []);
        registry.enqueueTurn("mid", "UserVisible", []);
        await flushMicrotasks();
        expect(order).toEqual(["high", "mid", "low"]);
      });
    });
    describe("ActivationRegistry.suspend cancels queued turns", () => {
      it("a suspended actor's queued turns are cancelled, never delivered", async () => {
        const shardClient = fakeShardClient();
        const delivered = [];
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        registry.enqueueTurn("actor-1", "Interactive", []);
        await registry.suspend("actor-1");
        await flushMicrotasks();
        expect(delivered).toEqual([]);
        await registry.resume("actor-1");
        await flushMicrotasks();
        expect(delivered).toEqual([]);
      });
    });
    describe("ActivationRegistry.handleShardLost / restoreActors", () => {
      it("is a valid ShardClientOptions.onShardLost value", () => {
        let registry;
        const shardClient = new ShardClient({
          shardCount: 1,
          createWorker: () => createAutoReplyWorker(),
          onShardLost: (shardIndex, actorIds) => registry.handleShardLost(shardIndex, actorIds)
        });
        registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        expect(typeof registry.handleShardLost).toBe("function");
      });
      it("restores exactly the actors that were on the lost shard, leaving an actor on a different shard untouched", async () => {
        const shardClient = new ShardClient({ shardCount: 2, exclusiveShardCount: 0, createWorker: () => createAutoReplyWorker() });
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "on-shard-0", "manual");
        await registry.activate("p1", "on-shard-1", "manual");
        expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
        expect(shardClient.shardIndexFor("on-shard-1")).toBe(1);
        const lostActorIds = shardClient.terminate(0);
        shardClient.rebuild(0);
        expect(lostActorIds).toEqual(["on-shard-0"]);
        await registry.restoreActors(lostActorIds);
        expect(registry.isResident("on-shard-0")).toBe(true);
        expect(registry.isResident("on-shard-1")).toBe(true);
        expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
      });
    });
    describe("ActivationRegistry restore ordering", () => {
      it("a restored actor does not receive turns that were queued before the restore, but does receive turns queued after", async () => {
        const shardClient = new ShardClient({ shardCount: 1, createWorker: () => createAutoReplyWorker() });
        const delivered = [];
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        await registry.activate("p1", "actor-1", "manual");
        registry.enqueueTurn("actor-1", "Interactive", []);
        const lostActorIds = shardClient.terminate(0);
        shardClient.rebuild(0);
        await registry.restoreActors(lostActorIds);
        await flushMicrotasks();
        expect(delivered).toEqual([]);
        registry.enqueueTurn("actor-1", "Interactive", []);
        await flushMicrotasks();
        expect(delivered).toEqual(["delivered"]);
      });
    });
    describe("residentActorCapFromMemory", () => {
      it("derives the cap from deviceMemoryGiB when present, clamped to [4, 96]", () => {
        expect(residentActorCapFromMemory({ deviceMemoryGiB: 1 })).toBe(6);
        expect(residentActorCapFromMemory({ deviceMemoryGiB: 16 })).toBe(96);
      });
      it("falls back to jsHeapSizeLimitBytes when deviceMemoryGiB is absent", () => {
        expect(residentActorCapFromMemory({ jsHeapSizeLimitBytes: 256 * 1024 * 1024 })).toBe(4);
      });
      it("falls back to the hardcoded constant when neither signal is present", () => {
        expect(residentActorCapFromMemory({})).toBe(DEFAULT_MAX_RESIDENT_ACTORS);
      });
    });
    describe("ActivationRegistry.maxResidentActors derived from an injected memory probe", () => {
      async function activateAndCountResident(memoryProbe, activationCount) {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], memoryProbe });
        registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
        for (let i = 0;i < activationCount; i += 1)
          await registry.activate("p1", `actor-${i}`, "manual");
        let resident = 0;
        for (let i = 0;i < activationCount; i += 1)
          if (registry.isResident(`actor-${i}`))
            resident += 1;
        return resident;
      }
      it("a small deviceMemoryGiB reading evicts down to its (small) derived cap", async () => {
        const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 1 }), 10);
        expect(resident).toBe(residentActorCapFromMemory({ deviceMemoryGiB: 1 }));
        expect(resident).toBeLessThan(10);
      });
      it("a large deviceMemoryGiB reading keeps every one of the same 10 activations resident", async () => {
        const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 16 }), 10);
        expect(resident).toBe(10);
      });
    });
    describe("ActivationRegistry.metricsBus (autoStartMetricsPublisher)", () => {
      it("publishes os.runtime.metrics as a CustomEvent on metricsBus at the 2Hz interval, driven by the injected clock, and dispose() stops it", () => {
        vi.useFakeTimers();
        try {
          const shardClient = fakeShardClient();
          let simulatedNowMs = 0;
          const registry = new ActivationRegistry({
            shardClient,
            defaultBudget: BUDGET_FIXTURE,
            fetchAssets: async () => [],
            now: () => simulatedNowMs,
            autoStartMetricsPublisher: true
          });
          const received = [];
          registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push(event.detail));
          simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
          expect(received).toHaveLength(1);
          expect(received[0].sampledAtMs).toBe(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
          simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 2;
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
          expect(received).toHaveLength(2);
          registry.dispose();
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 5);
          expect(received).toHaveLength(2);
        } finally {
          vi.useRealTimers();
        }
      });
      it("stays empty (no live interval, no bus traffic) when autoStartMetricsPublisher is left at its default", () => {
        vi.useFakeTimers();
        try {
          const shardClient = fakeShardClient();
          const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
          const received = [];
          registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push(event.detail));
          vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 10);
          expect(received).toEqual([]);
          registry.dispose();
        } finally {
          vi.useRealTimers();
        }
      });
    });
  }
  ArtifactRouterConflictError = class ArtifactRouterConflictError extends Error {
    code = "artifact-router.conflict";
    constructor(artifactKind, key) {
      super(`[DEBUG] router conflict: ${artifactKind}#${key} already registered with different metadata`);
      this.name = "ArtifactRouterConflictError";
    }
  };
  ArtifactContributionNotPermittedError = class ArtifactContributionNotPermittedError extends Error {
    code = "transaction.contribution-not-permitted";
    constructor(contributorPluginId, ownerPluginId) {
      super(`[DEBUG] "${contributorPluginId}" may not contribute onto "${ownerPluginId}"'s artifact kind — not a direct dependency`);
      this.name = "ArtifactContributionNotPermittedError";
    }
  };
  if (import.meta.vitest) {
    const { describe, expect, it } = import.meta.vitest;
    describe("expandPluginRegistry", () => {
      it("includes transitive dependsOn of primary and consume-matched contributors", () => {
        const plugins = [
          { pluginId: "host-plugin", moduleUrl: "a", consumes: ["ext.tag"], dependencies: [{ pluginId: "core", version: "*" }] },
          { pluginId: "core", moduleUrl: "b", dependencies: [{ pluginId: "stdio", version: "*" }] },
          { pluginId: "stdio", moduleUrl: "c" },
          { pluginId: "ext", moduleUrl: "d", contributes: ["ext.tag"], dependencies: [{ pluginId: "flow", version: "*" }] },
          { pluginId: "flow", moduleUrl: "e", dependencies: [{ pluginId: "stdio", version: "*" }] },
          { pluginId: "unrelated", moduleUrl: "f" }
        ];
        const expanded = expandPluginRegistry(plugins, "host-plugin", false);
        const ids = new Set(expanded.map((entry) => entry.pluginId));
        expect(ids.has("host-plugin")).toBe(true);
        expect(ids.has("core")).toBe(true);
        expect(ids.has("stdio")).toBe(true);
        expect(ids.has("ext")).toBe(true);
        expect(ids.has("flow")).toBe(true);
        expect(ids.has("unrelated")).toBe(false);
      });
    });
  }
  if (import.meta.vitest) {
    const { describe, expect, it } = import.meta.vitest;
    describe("IoEntryGraph", () => {
      const binaryRaw = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
      const gif87a = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" };
      const gif89a = { artifactKind: "s.stdio.gif", standard: "89a", subset: "*" };
      const fixturePlugins = [
        { pluginId: "stdio", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true }] },
        {
          pluginId: "gif",
          entries: [
            { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
            { from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }
          ]
        }
      ];
      it("resolves the highest-minimum-fidelity route regardless of registration order", () => {
        const forward = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a);
        const reversed = IoEntryGraph.build([...fixturePlugins].reverse()).route(binaryRaw, gif89a);
        expect(forward).toEqual(reversed);
        expect(forward).toEqual({
          hops: [
            { from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true },
            { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false }
          ],
          fidelity: "Canonical"
        });
      });
      it("respects maxHops, picking the direct (weaker) shortcut when bounded to 1", () => {
        const route = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a, 1);
        expect(route).toEqual({ hops: [{ from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }], fidelity: "Lossy" });
      });
      it("rejects a different plugin claiming an already-owned (from,into) key", () => {
        expect(() => IoEntryGraph.build([...fixturePlugins, { pluginId: "intruder", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Lossy", sniffs: false }] }])).toThrow(/conflict/);
      });
      it("ownerOf reports the registering plugin", () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        expect(graph.ownerOf(binaryRaw, gif87a)).toBe("stdio");
        expect(graph.ownerOf(gif87a, gif89a)).toBe("gif");
        expect(graph.ownerOf(gif89a, binaryRaw)).toBeUndefined();
      });
      it("carrierEntries returns only the sniff-declaring hops whose from is the given carrier", () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        const entries = graph.carrierEntries(binaryRaw);
        expect(entries.map((entry) => ({ into: entry.into, pluginId: entry.pluginId }))).toEqual([
          { into: gif87a, pluginId: "stdio" },
          { into: gif89a, pluginId: "gif" }
        ]);
      });
      it("ioRun executes the whole route hop by hop, feeding each hop's output to the next", async () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        const calls = [];
        const result = await ioRun(graph, "norm", binaryRaw, gif89a, new Uint8Array([1]), (pluginId, from, into, payload) => {
          calls.push(`${pluginId}:${dialectCoordinate(from)}->${dialectCoordinate(into)}`);
          return new Uint8Array([...payload, payload.length]);
        });
        expect(calls).toEqual(["stdio:s.stdio.binary@raw/*->s.stdio.gif@87a/*", "gif:s.stdio.gif@87a/*->s.stdio.gif@89a/*"]);
        expect(Array.from(result)).toEqual([1, 1, 2]);
      });
      it("ioRun refuses the WHOLE route (no partial execution) when the calling plugin owns any hop", async () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        let ran = false;
        await expect(ioRun(graph, "gif", binaryRaw, gif89a, new Uint8Array, () => {
          ran = true;
          return new Uint8Array;
        })).rejects.toThrow(/refused/);
        expect(ran).toBe(false);
      });
      it("ioIdentify fans sniffHop out across carrier entries, skipping the calling plugin's own, sorted by confidence then coordinate", async () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        const results = await ioIdentify(graph, "norm", binaryRaw, new Uint8Array, (pluginId) => pluginId === "stdio" ? 3 : 1);
        expect(results).toEqual([
          [gif87a, "High"],
          [gif89a, "Low"]
        ]);
      });
      it("ioIdentify skips the calling plugin's own carrier entries", async () => {
        const graph = IoEntryGraph.build(fixturePlugins);
        const results = await ioIdentify(graph, "stdio", binaryRaw, new Uint8Array, () => 3);
        expect(results).toEqual([[gif89a, "High"]]);
      });
    });
  }
});

/* ../../../../../../../../../🔨️modules/🔄️machine/🟦️component.ts */
class BitSet {
  #bits;
  constructor(bits = []) {
    this.#bits = new Set(bits);
  }
  set(id) {
    this.#bits.add(id);
  }
  clear(id) {
    this.#bits.delete(id);
  }
  contains(id) {
    return this.#bits.has(id);
  }
  *iterOnes() {
    for (const id of [...this.#bits].sort((a, b) => a - b))
      yield NodeId(id);
  }
  clearAll() {
    this.#bits.clear();
  }
  isEmpty() {
    return this.#bits.size === 0;
  }
  clone() {
    return new BitSet(this.#bits);
  }
  equals(other) {
    if (!(other instanceof BitSet))
      return false;
    if (other.#bits.size !== this.#bits.size)
      return false;
    for (const id of this.#bits)
      if (!other.#bits.has(id))
        return false;
    return true;
  }
}

class Snapshot {
  configuration;
  context;
  status;
  #nodes;
  #history = [];
  constructor(nodes, configuration, context, status = { kind: "running" }) {
    this.#nodes = nodes;
    this.configuration = configuration;
    this.context = context;
    this.status = status;
  }
  matches(stableId) {
    for (const id of this.configuration.iterOnes())
      if (this.#nodes[id].stableId === stableId)
        return true;
    return false;
  }
  historyFor(node) {
    return this.#history.find(([key]) => key === node)?.[1];
  }
  recordHistory(node, value) {
    const entry = this.#history.find(([key]) => key === node);
    if (entry)
      entry[1] = [...value];
    else
      this.#history.push([node, [...value]]);
  }
  historyEntries() {
    return this.#history;
  }
  branchForExploration() {
    const branch = new Snapshot(this.#nodes, this.configuration.clone(), structuredClone(this.context), { kind: "running" });
    for (const [owner, ids] of this.#history)
      branch.recordHistory(owner, ids);
    return branch;
  }
}

class NullInspector {
  observe() {}
}

class TraceInspector {
  entries = [];
  observe(event) {
    if (event.kind === "microstep")
      this.entries.push({ exited: event.exited, entered: event.entered });
  }
}
function isDescendant(nodes, a, ancestor) {
  if (a === ancestor)
    return false;
  let cur = nodes[a].parent;
  while (cur !== undefined) {
    if (cur === ancestor)
      return true;
    cur = nodes[cur].parent;
  }
  return false;
}
function isDescendantOrSelf(nodes, a, ancestor) {
  return a === ancestor || isDescendant(nodes, a, ancestor);
}
function depthOf(nodes, id) {
  let depth = 0;
  let cur = nodes[id].parent;
  while (cur !== undefined) {
    depth += 1;
    cur = nodes[cur].parent;
  }
  return depth;
}
function isCompoundOrParallel(nodes, id) {
  const kind = nodes[id].kind;
  return kind === "compound" || kind === "parallel";
}
function isLeafish(nodes, id) {
  const kind = nodes[id].kind;
  return kind === "atomic" || kind === "final";
}
function computeDomain(nodes, source, targets, kind) {
  if (targets.length === 0)
    return source;
  if (kind === "internal" && isCompoundOrParallel(nodes, source) && targets.every((t) => isDescendant(nodes, t, source)))
    return source;
  let anc = nodes[source].parent;
  while (anc !== undefined) {
    if (isCompoundOrParallel(nodes, anc) && targets.every((t) => isDescendantOrSelf(nodes, t, anc)))
      return anc;
    anc = nodes[anc].parent;
  }
  return ROOT;
}
function resolveEffectiveTargets(nodes, targets, snapshot) {
  const out = [];
  for (const t of targets) {
    const kind = nodes[t].kind;
    if (kind === "historyShallow" || kind === "historyDeep") {
      const recorded = snapshot.historyFor(t);
      if (recorded) {
        for (const r of recorded)
          if (!out.includes(r))
            out.push(r);
      } else {
        const fallback = nodes[t].initial;
        if (fallback !== undefined && !out.includes(fallback))
          out.push(fallback);
      }
    } else if (!out.includes(t)) {
      out.push(t);
    }
  }
  return out;
}
function addDescendantStatesToEnter(nodes, state, snapshot, out) {
  const kind = nodes[state].kind;
  if (kind === "historyShallow" || kind === "historyDeep") {
    for (const r of resolveEffectiveTargets(nodes, [state], snapshot))
      addDescendantStatesToEnter(nodes, r, snapshot, out);
    return;
  }
  if (!out.includes(state))
    out.push(state);
  if (kind === "compound") {
    const initial = nodes[state].initial;
    if (initial !== undefined) {
      addDescendantStatesToEnter(nodes, initial, snapshot, out);
      addAncestorStatesToEnter(nodes, initial, state, snapshot, out);
    }
  } else if (kind === "parallel") {
    for (const child of nodes[state].children) {
      if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
        addDescendantStatesToEnter(nodes, child, snapshot, out);
    }
  }
}
function addAncestorStatesToEnter(nodes, state, stopAt, snapshot, out) {
  let anc = nodes[state].parent;
  while (anc !== undefined && anc !== stopAt) {
    if (!out.includes(anc))
      out.push(anc);
    if (nodes[anc].kind === "parallel") {
      for (const child of nodes[anc].children) {
        if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
          addDescendantStatesToEnter(nodes, child, snapshot, out);
      }
    }
    anc = nodes[anc].parent;
  }
}
function stateDone(nodes, config, node) {
  const kind = nodes[node].kind;
  if (kind === "final")
    return true;
  if (kind === "compound") {
    for (const child of nodes[node].children)
      if (config.contains(child))
        return stateDone(nodes, config, child);
    return false;
  }
  if (kind === "parallel")
    return nodes[node].children.every((c) => stateDone(nodes, config, c));
  return false;
}
function computeDoneNodes(nodes, config) {
  const out = [];
  for (const id of config.iterOnes())
    if (isCompoundOrParallel(nodes, id) && stateDone(nodes, config, id))
      out.push(id);
  return out;
}
function candidatesFor(definition, config, context, event, selector, done) {
  const out = [];
  definition.transitions.forEach((t, i) => {
    if (!config.contains(t.source))
      return;
    const matchesTrigger = selector.kind === "event" && t.trigger.kind === "event" && t.trigger.event === selector.event || selector.kind === "spontaneous" && (t.trigger.kind === "eventless" || t.trigger.kind === "done" && done.includes(t.trigger.node)) || selector.kind === "timer" && t.trigger.kind === "timer" && t.trigger.timer === selector.timer;
    if (!matchesTrigger)
      return;
    if (t.guard !== undefined && !definition.guards[t.guard](context, event))
      return;
    out.push(i);
  });
  return out;
}
function resolveConflicts(nodes, transitions, candidates) {
  const sorted = [...candidates].sort((a, b) => transitions[a].docIndex - transitions[b].docIndex);
  const selected = [];
  outer:
    for (const cand of sorted) {
      const candDomain = computeDomain(nodes, transitions[cand].source, transitions[cand].targets, transitions[cand].kind);
      const toRemove = [];
      for (let i = 0;i < selected.length; i += 1) {
        const sel = selected[i];
        const selDomain = computeDomain(nodes, transitions[sel].source, transitions[sel].targets, transitions[sel].kind);
        if (isDescendantOrSelf(nodes, candDomain, selDomain) || isDescendantOrSelf(nodes, selDomain, candDomain)) {
          if (depthOf(nodes, transitions[cand].source) > depthOf(nodes, transitions[sel].source))
            toRemove.push(i);
          else
            continue outer;
        }
      }
      for (let i = toRemove.length - 1;i >= 0; i -= 1)
        selected.splice(toRemove[i], 1);
      selected.push(cand);
    }
  return selected;
}
function applyTransitions(definition, snapshot, transitionsIdx, event, sink, inspector) {
  const nodes = definition.nodes;
  const exitIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    for (const id of snapshot.configuration.iterOnes())
      if (isDescendant(nodes, id, domain) && !exitIds.includes(id))
        exitIds.push(id);
  }
  exitIds.sort((a, b) => depthOf(nodes, b) - depthOf(nodes, a));
  for (const owner of exitIds) {
    for (const child of nodes[owner].children) {
      const childKind = nodes[child].kind;
      if (childKind === "historyShallow") {
        const activeChild = nodes[owner].children.find((c) => snapshot.configuration.contains(c) && nodes[c].kind !== "historyShallow" && nodes[c].kind !== "historyDeep");
        if (activeChild !== undefined)
          snapshot.recordHistory(child, [activeChild]);
      } else if (childKind === "historyDeep") {
        const leaves = [];
        for (const id of snapshot.configuration.iterOnes())
          if (isDescendant(nodes, id, owner) && isLeafish(nodes, id))
            leaves.push(id);
        snapshot.recordHistory(child, leaves);
      }
    }
  }
  for (const id of exitIds) {
    for (const actionId of nodes[id].exitActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId] of nodes[id].timers)
      sink.push({ kind: "cancelTimer", timer: timerId });
    for (const invokeId of nodes[id].invokes)
      sink.push({ kind: "stopInvoke", invoke: invokeId });
    snapshot.configuration.clear(id);
  }
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    for (const actionId of t.actions)
      definition.actions[actionId](snapshot.context, event, sink);
  }
  const entryIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    const effectiveTargets = resolveEffectiveTargets(nodes, t.targets, snapshot);
    for (const target of effectiveTargets)
      addDescendantStatesToEnter(nodes, target, snapshot, entryIds);
    for (const target of effectiveTargets)
      addAncestorStatesToEnter(nodes, target, domain, snapshot, entryIds);
  }
  entryIds.sort((a, b) => depthOf(nodes, a) - depthOf(nodes, b));
  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of nodes[id].entryActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId, delayMs] of nodes[id].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of nodes[id].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  inspector.observe({ kind: "microstep", exited: exitIds, entered: entryIds });
}
function finalizeStatus(definition, snapshot) {
  if (snapshot.status.kind === "done")
    return;
  if (stateDone(definition.nodes, snapshot.configuration, ROOT) && definition.makeOutput) {
    snapshot.status = { kind: "done", output: definition.makeOutput(snapshot.context) };
  }
}
function runToCompletion(definition, snapshot, seed, sink, inspector) {
  inspector.observe({ kind: "macrostepStart" });
  const queue = seed ? [seed] : [];
  let microsteps = 0;
  for (;; ) {
    if (microsteps >= MICROSTEP_LIMIT)
      break;
    let selected;
    let eventOwned;
    const trigger = queue.shift();
    if (trigger) {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const selector = trigger.selector.kind === "event" ? { kind: "event", event: trigger.selector.event } : { kind: "timer", timer: trigger.selector.timer };
      selected = candidatesFor(definition, snapshot.configuration, snapshot.context, trigger.event, selector, done);
      eventOwned = trigger.event;
    } else {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const spontaneous = candidatesFor(definition, snapshot.configuration, snapshot.context, undefined, { kind: "spontaneous" }, done);
      if (spontaneous.length === 0)
        break;
      selected = spontaneous;
      eventOwned = undefined;
    }
    if (selected.length === 0)
      continue;
    const resolved = resolveConflicts(definition.nodes, definition.transitions, selected);
    microsteps += 1;
    const local = [];
    applyTransitions(definition, snapshot, resolved, eventOwned, local, inspector);
    for (const command of local) {
      if (command.kind === "raise")
        queue.push({ selector: { kind: "event", event: command.event.eventId() }, event: command.event });
      inspector.observe({ kind: "commandIssued", command });
      sink.push(command);
    }
  }
  finalizeStatus(definition, snapshot);
  inspector.observe({ kind: "settled", microsteps });
  return { microsteps };
}
function init(machine, input, sink) {
  const definition = machine.definition;
  const snapshot = new Snapshot(definition.nodes, new BitSet, definition.contextFromInput(input));
  const entryIds = [];
  addDescendantStatesToEnter(definition.nodes, ROOT, snapshot, entryIds);
  entryIds.sort((a, b) => depthOf(definition.nodes, a) - depthOf(definition.nodes, b));
  for (const id of entryIds) {
    snapshot.configuration.set(id);
    for (const actionId of definition.nodes[id].entryActions)
      definition.actions[actionId](snapshot.context, undefined, sink);
    for (const [timerId, delayMs] of definition.nodes[id].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of definition.nodes[id].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  runToCompletion(definition, snapshot, undefined, sink, new NullInspector);
  return snapshot;
}
function macrostep(machine, snapshot, event, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "event", event: event.eventId() }, event }, sink, inspector);
}
function timerElapsed(machine, snapshot, timer, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "timer", timer } }, sink, inspector);
}

class NativeHost {
  #start = Date.now();
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  effects() {
    return this.#effects;
  }
  drainEffects() {
    return this.#effects.splice(0, this.#effects.length);
  }
  startedTasks() {
    return this.#startedTasks;
  }
  dueTimers() {
    const now = this.nowMs();
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.nowMs() + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
  }
  nowMs() {
    return Date.now() - this.#start;
  }
}

class TestHost {
  #clockMs = 0;
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  #cancelledTasks = [];
  effects() {
    return this.#effects;
  }
  startedTasks() {
    return this.#startedTasks;
  }
  cancelledTasks() {
    return this.#cancelledTasks;
  }
  advance(delayMs) {
    this.#clockMs += delayMs;
    const now = this.#clockMs;
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.#clockMs + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
    this.#cancelledTasks.push([actor, invoke]);
  }
  nowMs() {
    return this.#clockMs;
  }
}
function persist(machine, snapshot) {
  const nodes = machine.definition.nodes;
  const states = [...snapshot.configuration.iterOnes()].map((id) => nodes[id].stableId);
  const history = snapshot.historyEntries().map(([owner, ids]) => [nodes[owner].stableId, ids.map((id) => nodes[id].stableId)]);
  return { version: 1, fingerprint: machine.definition.fingerprint, states, history, done: snapshot.status.kind === "done" };
}
function stableIdToNode(nodes, stableId) {
  const idx = nodes.findIndex((n) => n.stableId === stableId);
  return idx === -1 ? undefined : NodeId(idx);
}
function restore(machine, persisted, context, migrations) {
  const definition = machine.definition;
  let current = persisted;
  while (current.fingerprint !== definition.fingerprint) {
    const next = migrations.find((m) => m.sourceFingerprint === current.fingerprint);
    if (!next)
      return { ok: false, error: { kind: "fingerprintMismatch" } };
    current = next.migrate(current);
  }
  const configuration = new BitSet;
  for (const stableId of current.states) {
    const id = stableIdToNode(definition.nodes, stableId);
    if (id === undefined)
      return { ok: false, error: { kind: "unknownStableId", stableId } };
    configuration.set(id);
  }
  const snapshot = new Snapshot(definition.nodes, configuration, context, { kind: "running" });
  for (const [ownerStableId, ids] of current.history) {
    const owner = stableIdToNode(definition.nodes, ownerStableId);
    if (owner === undefined)
      return { ok: false, error: { kind: "unknownStableId", stableId: ownerStableId } };
    const resolved = [];
    for (const stableId of ids) {
      const id = stableIdToNode(definition.nodes, stableId);
      if (id === undefined)
        return { ok: false, error: { kind: "unknownStableId", stableId } };
      resolved.push(id);
    }
    snapshot.recordHistory(owner, resolved);
  }
  return { ok: true, snapshot };
}

class Actor {
  id;
  snapshot;
  mailbox = [];
  constructor(id, snapshot) {
    this.id = id;
    this.snapshot = snapshot;
  }
}

class ActorSystem {
  host;
  #machine;
  #actors = [];
  #nextId = 0;
  constructor(host, machine) {
    this.host = host;
    this.#machine = machine;
  }
  spawnRoot(input) {
    const id = ActorId(this.#nextId);
    this.#nextId += 1;
    const buffer = [];
    const snapshot = init(this.#machine, input, buffer);
    this.#actors.push(new Actor(id, snapshot));
    this.#routeCommands(id, buffer);
    return id;
  }
  snapshot(id) {
    return this.#actors.find((a) => a.id === id)?.snapshot;
  }
  send(to, event) {
    this.#actors.find((a) => a.id === to)?.mailbox.push(event);
  }
  timerElapsed(to, timer) {
    const actor = this.#actors.find((a) => a.id === to);
    if (!actor)
      return;
    const buffer = [];
    const report = timerElapsed(this.#machine, actor.snapshot, timer, buffer, new NullInspector);
    this.#routeCommands(to, buffer);
    return report;
  }
  drain() {
    const reports = [];
    for (;; ) {
      let progressed = false;
      for (const actor of this.#actors) {
        const event = actor.mailbox.shift();
        if (event === undefined)
          continue;
        progressed = true;
        const buffer = [];
        const report = macrostep(this.#machine, actor.snapshot, event, buffer, new NullInspector);
        this.#routeCommands(actor.id, buffer);
        reports.push(report);
      }
      if (!progressed)
        break;
    }
    return reports;
  }
  #routeCommands(actor, commands) {
    const sends = [];
    const found = this.#actors.find((a) => a.id === actor);
    if (found) {
      for (const command of commands) {
        const pair = routeCommand(this.host, found.snapshot, actor, command);
        if (pair)
          sends.push(pair);
      }
    }
    for (const [to, event] of sends)
      this.send(to, event);
  }
}
function routeCommand(host, snapshot, actor, command) {
  switch (command.kind) {
    case "effect":
      host.executeEffect(actor, command.effect);
      return;
    case "raise":
      return;
    case "send":
      return [command.to, command.event];
    case "emit":
      snapshot.status = { kind: "done", output: command.output };
      return;
    case "startInvoke":
      host.startTask(actor, command.invoke);
      return;
    case "stopInvoke":
      host.cancelTask(actor, command.invoke);
      return;
    case "schedule":
      host.schedule(actor, command.timer, command.delayMs);
      return;
    case "cancelTimer":
      host.cancelTimer(actor, command.timer);
      return;
  }
}

class StepInspector {
  entered = [];
  exited = [];
  observe(event) {
    if (event.kind === "microstep") {
      this.exited.push(...event.exited);
      this.entered.push(...event.entered);
    }
  }
}

class MachineStep {
  entered;
  exited;
  active;
  commands;
  report;
  persisted;
  constructor(entered, exited, active, commands, report, persisted) {
    this.entered = entered;
    this.exited = exited;
    this.active = active;
    this.commands = commands;
    this.report = report;
    this.persisted = persisted;
  }
  isActive(stableId) {
    return this.active.includes(stableId);
  }
}
function stableIds(nodes, ids) {
  return ids.map((id) => nodes[id].stableId);
}
function machineStepOf(machine, snapshot, entered, exited, commands, report) {
  const nodes = machine.definition.nodes;
  return new MachineStep(stableIds(nodes, entered), stableIds(nodes, exited), stableIds(nodes, [...snapshot.configuration.iterOnes()]), commands, report, persist(machine, snapshot));
}
function start(machine, input) {
  const commands = [];
  const snapshot = init(machine, input, commands);
  return machineStepOf(machine, snapshot, [], [], commands, { microsteps: 0 });
}
function step(machine, prior, context, event, migrations) {
  const restored = restore(machine, prior, context, migrations);
  if (!restored.ok)
    return restored;
  const commands = [];
  const inspector = new StepInspector;
  const report = macrostep(machine, restored.snapshot, event, commands, inspector);
  return { ok: true, step: machineStepOf(machine, restored.snapshot, inspector.entered, inspector.exited, commands, report) };
}

class Model {
  events;
  constructor(events) {
    this.events = events;
  }
}
function activeStableIds(nodes, snapshot) {
  return [...snapshot.configuration.iterOnes()].map((id) => nodes[id].stableId);
}
function explore(machine, model, input) {
  const nodes = machine.definition.nodes;
  const root = init(machine, input, []);
  const visited = [];
  const frontier = [root];
  const reachedIds = [];
  let snapshot = frontier.pop();
  while (snapshot) {
    if (visited.some((v) => v.equals(snapshot.configuration))) {
      snapshot = frontier.pop();
      continue;
    }
    for (const stable of activeStableIds(nodes, snapshot))
      if (!reachedIds.includes(stable))
        reachedIds.push(stable);
    visited.push(snapshot.configuration.clone());
    for (const event of model.events) {
      const next = snapshot.branchForExploration();
      macrostep(machine, next, event, [], new NullInspector);
      frontier.push(next);
    }
    snapshot = frontier.pop();
  }
  return { visitedConfigurations: visited.length, reachedStableIds: reachedIds };
}
function checkInvariants(snapshot, invariants) {
  const violations = [];
  for (const invariant of invariants) {
    const result = invariant.check(snapshot);
    if (!result.ok)
      violations.push(`${invariant.name}: ${result.error.message}`);
  }
  return violations;
}
function runConformance(machine, input, steps) {
  const nodes = machine.definition.nodes;
  const sink = [];
  const snapshot = init(machine, input, sink);
  for (let index = 0;index < steps.length; index += 1) {
    const step2 = steps[index];
    macrostep(machine, snapshot, step2.event, sink, new NullInspector);
    for (const expected of step2.expectActive) {
      if (!snapshot.matches(expected)) {
        return { ok: false, error: { kind: "violation", message: `conformance step ${index}: expected active state '${expected}', got ${JSON.stringify(activeStableIds(nodes, snapshot))}` } };
      }
    }
  }
  return { ok: true };
}
var NodeId = (value) => value, EventId = (value) => value, TransitionId = (value) => value, GuardId = (value) => value, ActionId = (value) => value, InvokeId = (value) => value, TimerId = (value) => value, ActorId = (value) => value, ROOT, MICROSTEP_LIMIT = 1000;
var init__component7 = __esm(() => {
  ROOT = NodeId(0);
});

/* ../../../../../../../../../📦️packages/🟦️typescript/🟦️glue.ts */
var exports__glue = {};
__export(exports__glue, {
  windowMeasureChromeStatus: () => windowMeasureChromeStatus,
  windowElementId: () => windowElementId,
  waitForEvent: () => waitForEvent,
  versionSatisfies: () => versionSatisfies,
  validatePluginDependencyGraph: () => validatePluginDependencyGraph,
  uiPresenceShowsSkeleton: () => uiPresenceShowsSkeleton,
  uiInspectorMixedToggle: () => uiInspectorMixedToggle,
  uiInspectorMixedText: () => uiInspectorMixedText,
  uiInspectorMixedSlider: () => uiInspectorMixedSlider,
  uiInspectorMixedSelect: () => uiInspectorMixedSelect,
  uiInspectorMixedNumber: () => uiInspectorMixedNumber,
  uiInspectorAllEqual: () => uiInspectorAllEqual,
  timerElapsed: () => timerElapsed,
  textEditorActions: () => textEditorActions,
  surfaceAppId: () => surfaceAppId,
  step: () => step,
  start: () => start,
  sortCanvasPickTargetsGeneralFirst: () => sortCanvasPickTargetsGeneralFirst,
  severityFromU8: () => severityFromU8,
  severityAsU8: () => severityAsU8,
  runtimeMetricsDue: () => runtimeMetricsDue,
  runConformance: () => runConformance,
  routeCommand: () => routeCommand,
  retryWithJitteredBackoff: () => retryWithJitteredBackoff,
  restore: () => restore,
  resolveWindowActions: () => resolveWindowActions,
  resolveUiPresence: () => resolveUiPresence,
  resolveUiDirtyScope: () => resolveUiDirtyScope,
  resolvePluginRegistryId: () => resolvePluginRegistryId,
  resolvePluginLoadOrder: () => resolvePluginLoadOrder,
  resolvePluginHostConfig: () => resolvePluginHostConfig,
  resolvePlaygroundDefaultAppId: () => resolvePlaygroundDefaultAppId,
  resolvePlaygroundBoot: () => resolvePlaygroundBoot,
  resolveOpeningApp: () => resolveOpeningApp,
  resolveModeTools: () => resolveModeTools,
  resolveLayoutForMode: () => resolveLayoutForMode,
  resolveExternalSlots: () => resolveExternalSlots,
  residentActorCapFromMemory: () => residentActorCapFromMemory,
  relayPluginBackboneOutbound: () => relayPluginBackboneOutbound,
  registerPluginBackboneRoute: () => registerPluginBackboneRoute,
  postPluginBackboneInbound: () => postPluginBackboneInbound,
  pluginGraphErrorMessage: () => pluginGraphErrorMessage,
  pluginDependents: () => pluginDependents,
  pickMostSpecificCanvasTarget: () => pickMostSpecificCanvasTarget,
  persist: () => persist,
  pendingWindowUiNode: () => pendingWindowUiNode,
  pendingPanelUiNode: () => pendingPanelUiNode,
  partitionWindowMeasures: () => partitionWindowMeasures,
  parseSurfaceAppId: () => parseSurfaceAppId,
  parseInvocationResponse: () => parseInvocationResponse,
  parseDialectCoordinate: () => parseDialectCoordinate,
  parseCanvasPickTargetKey: () => parseCanvasPickTargetKey,
  panelTabKindId: () => panelTabKindId,
  panelTabFirstDraggableElementId: () => panelTabFirstDraggableElementId,
  panelTabElementId: () => panelTabElementId,
  organizeContextMenu: () => organizeContextMenu,
  orderPluginRegistryEntries: () => orderPluginRegistryEntries,
  normalizeAppLabelsOverlay: () => normalizeAppLabelsOverlay,
  nodeGraphActions: () => nodeGraphActions,
  multiplexPluginSources: () => multiplexPluginSources,
  missingRequiredArgs: () => missingRequiredArgs,
  mergePolicyFromU8: () => mergePolicyFromU8,
  mergePolicyAsU8: () => mergePolicyAsU8,
  mergeNamedLayouts: () => mergeNamedLayouts,
  mergeById: () => mergeById,
  macrostep: () => macrostep,
  latestWins: () => latestWins,
  isShellTerminology: () => isShellTerminology,
  isShellLocale: () => isShellLocale,
  ioRun: () => ioRun,
  ioIdentify: () => ioIdentify,
  ioConfidenceFromRank: () => ioConfidenceFromRank,
  intersectCapabilityGrants: () => intersectCapabilityGrants,
  inkCanvasActions: () => inkCanvasActions,
  init: () => init,
  foldOpeningPreferences: () => foldOpeningPreferences,
  fetchWithTimeout: () => fetchWithTimeout,
  explore: () => explore,
  expandPluginRegistry: () => expandPluginRegistry,
  ephemeralWeakMap: () => ephemeralWeakMap,
  ephemeralSet: () => ephemeralSet,
  ephemeralMap: () => ephemeralMap,
  ephemeralBox: () => ephemeralBox,
  ensureContributorInstance: () => ensureContributorInstance,
  encodeSurfaceAppChoice: () => encodeSurfaceAppChoice,
  encodeArtifactKindChoice: () => encodeArtifactKindChoice,
  effectiveActionArgs: () => effectiveActionArgs,
  dialectCoordinate: () => dialectCoordinate,
  deriveUtilityNodes: () => deriveUtilityNodes,
  defaultOsTransient: () => defaultOsTransient,
  defaultMemoryProbe: () => defaultMemoryProbe,
  decodeSurfaceAppChoice: () => decodeSurfaceAppChoice,
  decodeOpeningPreferences: () => decodeOpeningPreferences,
  decodeOpeningConfigMutation: () => decodeOpeningConfigMutation,
  decodeArtifactKindChoice: () => decodeArtifactKindChoice,
  createWindowLayout: () => createWindowLayout,
  createTurnOutcomeBroadcast: () => createTurnOutcomeBroadcast,
  createTabStackLayout: () => createTabStackLayout,
  createStackLayout: () => createStackLayout,
  createScopedStoragePort: () => createScopedStoragePort,
  createNamedLayout: () => createNamedLayout,
  createMemoryStoragePort: () => createMemoryStoragePort,
  createLeasePool: () => createLeasePool,
  createExtensionSource: () => createExtensionSource,
  createDevPluginSource: () => createDevPluginSource,
  createDefaultLayout: () => createDefaultLayout,
  createBrowserStoragePort: () => createBrowserStoragePort,
  conflictResolutionFromU8: () => conflictResolutionFromU8,
  conflictResolutionAsU8: () => conflictResolutionAsU8,
  checkInvariants: () => checkInvariants,
  canvasPickTargetKey: () => canvasPickTargetKey,
  canvasHoverFocusFromTarget: () => canvasHoverFocusFromTarget,
  buildContributionsJson: () => buildContributionsJson,
  artifactKindChoices: () => artifactKindChoices,
  argControl: () => argControl,
  WindowPaneStateStore: () => WindowPaneStateStore,
  UI_PENDING_PRESENCE: () => UI_PENDING_PRESENCE,
  UI_NAVBAR_ELEMENT_ID: () => UI_NAVBAR_ELEMENT_ID,
  UI_INSPECTOR_MIXED_PLACEHOLDER: () => UI_INSPECTOR_MIXED_PLACEHOLDER,
  UI_FOOTER_ELEMENT_ID: () => UI_FOOTER_ELEMENT_ID,
  TransitionId: () => TransitionId,
  TraceInspector: () => TraceInspector,
  TimerId: () => TimerId,
  TestHost: () => TestHost,
  TUTORIAL_CONVERGE_MS: () => TUTORIAL_CONVERGE_MS,
  Store: () => Store,
  Snapshot: () => Snapshot,
  SemioFaultError: () => SemioFaultError,
  SURFACE_FAULT_CODES: () => SURFACE_FAULT_CODES,
  STATE_CLASSES: () => STATE_CLASSES,
  START_TUTORIAL_ACTION_ID: () => START_TUTORIAL_ACTION_ID,
  START_INTRODUCTION_ACTION_ID: () => START_INTRODUCTION_ACTION_ID,
  SHELL_TERMINOLOGIES: () => SHELL_TERMINOLOGIES,
  SHELL_LOCALES: () => SHELL_LOCALES,
  SET_SELECTION_MODE_ACTION_ID: () => SET_SELECTION_MODE_ACTION_ID,
  SET_INTERACTION_GRANULARITY_ACTION_ID: () => SET_INTERACTION_GRANULARITY_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID: () => SET_ACTIVE_UTILITY_ACTION_ID,
  SET_ACTIVE_TOOL_ACTION_ID: () => SET_ACTIVE_TOOL_ACTION_ID,
  SELECT_ALL_ACTION_ID: () => SELECT_ALL_ACTION_ID,
  RUNTIME_METRICS_PUBLISH_INTERVAL_MS: () => RUNTIME_METRICS_PUBLISH_INTERVAL_MS,
  ROOT: () => ROOT,
  RECORD_TUTORIAL_ACTION_ID: () => RECORD_TUTORIAL_ACTION_ID,
  PluginGraph: () => PluginGraph,
  PLUGIN_SOURCE_WATCH_PATH: () => PLUGIN_SOURCE_WATCH_PATH,
  OsTransient: () => OsTransient,
  OsShellConfig: () => OsShellConfig,
  NullInspector: () => NullInspector,
  NodeId: () => NodeId,
  NativeHost: () => NativeHost,
  NamedLayoutStore: () => NamedLayoutStore,
  Model: () => Model,
  MachineStep: () => MachineStep,
  MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR: () => MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR,
  MUTATION_APPLY_ERROR_SCHEMA: () => MUTATION_APPLY_ERROR_SCHEMA,
  MICROSTEP_LIMIT: () => MICROSTEP_LIMIT,
  JSON_SCHEMA_DERIVED_KEY: () => JSON_SCHEMA_DERIVED_KEY,
  IoEntryGraph: () => IoEntryGraph,
  InvokeId: () => InvokeId,
  InstanceDirectory: () => InstanceDirectory,
  INTERACTION_SELECT_ACTION_ID: () => INTERACTION_SELECT_ACTION_ID,
  INTERACTION_HOVER_ACTION_ID: () => INTERACTION_HOVER_ACTION_ID,
  HISTORY_ACTION_IDS: () => HISTORY_ACTION_IDS,
  GuardId: () => GuardId,
  GRAPHQL_STATE_PREAMBLE: () => GRAPHQL_STATE_PREAMBLE,
  GRAPHQL_COMPOSITION_PREAMBLE: () => GRAPHQL_COMPOSITION_PREAMBLE,
  FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: () => FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID: () => FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: () => FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: () => FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID: () => FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: () => FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_LABEL: () => FRAMEWORK_PANEL_TAB_HISTORY_LABEL,
  FRAMEWORK_PANEL_TAB_HISTORY_ID: () => FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID: () => FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: () => FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID: () => FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: () => FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL: () => FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ID: () => FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
  FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID: () => FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID,
  EventId: () => EventId,
  EXTENSION_SOURCE_WATCH_PATH: () => EXTENSION_SOURCE_WATCH_PATH,
  EMPTY_OPENING_PREFERENCES: () => EMPTY_OPENING_PREFERENCES,
  EMPTY_APP_LABELS_OVERLAY: () => EMPTY_APP_LABELS_OVERLAY,
  DockUiStateStore: () => DockUiStateStore,
  DockLayoutStore: () => DockLayoutStore,
  DEFAULT_MERGE_POLICY: () => DEFAULT_MERGE_POLICY,
  DEFAULT_MAX_RESIDENT_ACTORS: () => DEFAULT_MAX_RESIDENT_ACTORS,
  CLEAR_SELECTION_ACTION_ID: () => CLEAR_SELECTION_ACTION_ID,
  CARRIER_TEXT_DIALECT: () => CARRIER_TEXT_DIALECT,
  CARRIER_BINARY_DIALECT: () => CARRIER_BINARY_DIALECT,
  CANVAS_HOVER_SOURCE_PICK_MENU: () => CANVAS_HOVER_SOURCE_PICK_MENU,
  CANVAS_HOVER_SOURCE_CATALOG: () => CANVAS_HOVER_SOURCE_CATALOG,
  CANVAS_HOVER_SOURCE_CANVAS: () => CANVAS_HOVER_SOURCE_CANVAS,
  CANVAS_HOVER_SOURCE_ARTIFACT: () => CANVAS_HOVER_SOURCE_ARTIFACT,
  BitSet: () => BitSet,
  ArtifactSchemaRegistry: () => ArtifactSchemaRegistry,
  ArtifactRouterConflictError: () => ArtifactRouterConflictError,
  ArtifactMutationRouter: () => ArtifactMutationRouter,
  ArtifactInferenceRouter: () => ArtifactInferenceRouter,
  ArtifactInferenceRegistry: () => ArtifactInferenceRegistry,
  ArtifactContributionNotPermittedError: () => ArtifactContributionNotPermittedError,
  AppSchemaRegistry: () => AppSchemaRegistry,
  AppRouter: () => AppRouter,
  ActorSystem: () => ActorSystem,
  ActorId: () => ActorId,
  ActivationRegistry: () => ActivationRegistry,
  ActionId: () => ActionId
});
function createLeasePool(load, dispose, options) {
  const lingerMs = options?.lingerMs ?? 30000;
  const label = options?.label ?? "resource";
  const entries = new Map;
  function disposeEntry(key, entry) {
    if (entries.get(key) !== entry)
      return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }
  return {
    async acquire(key) {
      let entry = entries.get(key);
      if (!entry) {
        const created = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then((value) => {
          created.settled = value;
        }, () => {
          if (entries.get(key) === created)
            entries.delete(key);
        });
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released)
              return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0)
              return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          }
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key) {
      for (const [entryKey, entry] of key ? [[key, entries.get(key)]] : entries) {
        if (!entry)
          continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped — ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null)
          clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats() {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident"
      }));
    }
  };
}
function abortReason(signal) {
  return signal.reason ?? new Error("retryWithJitteredBackoff: aborted");
}
function abortableDelay(ms, signal) {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(abortReason(signal));
      return;
    }
    const timer = setTimeout(() => {
      cleanup();
      resolve();
    }, ms);
    function cleanup() {
      clearTimeout(timer);
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort() {
      cleanup();
      reject(abortReason(signal));
    }
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
async function retryWithJitteredBackoff(fn, options) {
  const { minMs, maxMs, signal } = options;
  let attempt = 0;
  for (;; ) {
    if (signal?.aborted)
      throw abortReason(signal);
    try {
      return await fn();
    } catch (error) {
      if (signal?.aborted)
        throw abortReason(signal);
      attempt += 1;
      const cap = Math.min(maxMs, minMs * 2 ** attempt);
      const waitMs = cap <= minMs ? minMs : minMs + Math.random() * (cap - minMs);
      try {
        await abortableDelay(waitMs, signal);
      } catch {
        throw signal?.aborted ? abortReason(signal) : error;
      }
    }
  }
}
function latestWins(run) {
  let current = null;
  let queued = null;
  function launch() {
    let promise;
    try {
      promise = Promise.resolve(run());
    } catch (error) {
      promise = Promise.reject(error);
    }
    current = promise;
    const advance = () => {
      if (current === promise)
        current = null;
      if (queued !== null)
        queued = null;
    };
    promise.then(advance, advance);
    return promise;
  }
  return function trigger() {
    if (current === null)
      return launch();
    if (queued === null)
      queued = current.then(launch, launch);
    return queued;
  };
}
async function fetchWithTimeout(url, init2, options) {
  const { timeoutMs, signal: externalSignal } = options;
  if (externalSignal?.aborted)
    throw externalSignal.reason ?? new Error("fetchWithTimeout: aborted");
  const controller = new AbortController;
  const timer = setTimeout(() => controller.abort(new Error(`fetchWithTimeout: timed out after ${timeoutMs}ms`)), timeoutMs);
  function onExternalAbort() {
    controller.abort(externalSignal.reason);
  }
  externalSignal?.addEventListener("abort", onExternalAbort, { once: true });
  try {
    return await fetch(url, { ...init2, signal: controller.signal });
  } finally {
    clearTimeout(timer);
    externalSignal?.removeEventListener("abort", onExternalAbort);
  }
}
function waitForEvent(subscribe, options) {
  const signal = options?.signal;
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
      return;
    }
    let unsubscribe = null;
    function cleanup() {
      unsubscribe?.();
      unsubscribe = null;
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort() {
      cleanup();
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
    }
    unsubscribe = subscribe((value) => {
      cleanup();
      resolve(value);
    });
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
var init__glue = __esm(() => {
  init__component5();
  init__component4();
  init__component6();
  init__component7();
  init__component2();
  init__component3();
  init__component4();
  init__component5();
  init__component();
  init__component6();
  init__component7();
  if (import.meta.vitest) {
    const { describe, expect, it, vi } = import.meta.vitest;
    describe("DockLayoutStore", () => {
      const emptySkeleton = () => ({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
      });
      it("returns null when nothing persisted", () => {
        const store = new DockLayoutStore(createMemoryStoragePort());
        expect(store.getSnapshot()).toBeNull();
      });
      it("app layer wins over os layer when both are set", () => {
        const storage = createMemoryStoragePort();
        const store = new DockLayoutStore(storage, "my-app");
        const osSkeleton = emptySkeleton();
        const appSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
        store.saveOs(osSkeleton);
        store.save(appSkeleton);
        expect(store.getSnapshot()).toEqual(appSkeleton);
      });
      it("falls back to os layer when app layer absent", () => {
        const storage = createMemoryStoragePort();
        const store = new DockLayoutStore(storage, "my-app");
        const osSkeleton = emptySkeleton();
        store.saveOs(osSkeleton);
        expect(store.getSnapshot()).toEqual(osSkeleton);
      });
      it("save(null) removes the app-layer key", () => {
        const storage = createMemoryStoragePort();
        const store = new DockLayoutStore(storage, "my-app");
        store.save(emptySkeleton());
        expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toEqual(emptySkeleton());
        store.save(null);
        expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toBeUndefined();
        expect(store.getSnapshot()).toBeNull();
      });
      it("reset() clears both layers", () => {
        const storage = createMemoryStoragePort();
        const store = new DockLayoutStore(storage, "my-app");
        store.saveOs(emptySkeleton());
        store.save(emptySkeleton());
        store.reset();
        expect(new OsShellConfig(storage).getSnapshot().dockLayouts).toEqual({ apps: {} });
        expect(store.getSnapshot()).toBeNull();
      });
      it("returns null on corrupt JSON rather than throwing", () => {
        const storage = createMemoryStoragePort();
        storage.set("semio.os.config", "{not json");
        const store = new DockLayoutStore(storage);
        expect(() => store.getSnapshot()).not.toThrow();
        expect(store.getSnapshot()).toBeNull();
      });
      it("discards a stale version-1 (corners) blob instead of migrating it", () => {
        const storage = createMemoryStoragePort();
        new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 1, corners: { "top-left": [{ id: "a" }] } }, apps: {} } }));
        const store = new DockLayoutStore(storage);
        expect(store.getSnapshot()).toBeNull();
      });
      it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
        const storage = createMemoryStoragePort();
        new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 2, anchors: {} }, apps: {} } }));
        const store = new DockLayoutStore(storage);
        expect(store.getSnapshot()).toBeNull();
      });
    });
    describe("DockUiStateStore", () => {
      const emptyUiState = () => ({ version: 3, anchors: {} });
      it("returns null when nothing persisted", () => {
        const store = new DockUiStateStore(createMemoryStoragePort());
        expect(store.getSnapshot()).toBeNull();
      });
      it("app layer wins over os layer when both are set", () => {
        const storage = createMemoryStoragePort();
        const store = new DockUiStateStore(storage, "my-app");
        const osState = emptyUiState();
        const appState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
        store.saveOs(osState);
        store.save(appState);
        expect(store.getSnapshot()).toEqual(appState);
      });
      it("falls back to os layer when app layer absent", () => {
        const storage = createMemoryStoragePort();
        const store = new DockUiStateStore(storage, "my-app");
        const osState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.artifact" } };
        store.saveOs(osState);
        expect(store.getSnapshot()).toEqual(osState);
      });
      it("save(null) removes the app-layer key", () => {
        const storage = createMemoryStoragePort();
        const store = new DockUiStateStore(storage, "my-app");
        store.save(emptyUiState());
        expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toEqual(emptyUiState());
        store.save(null);
        expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toBeUndefined();
        expect(store.getSnapshot()).toBeNull();
      });
      it("reset() clears both layers", () => {
        const storage = createMemoryStoragePort();
        const store = new DockUiStateStore(storage, "my-app");
        store.saveOs(emptyUiState());
        store.save(emptyUiState());
        store.reset();
        expect(new OsShellConfig(storage).getSnapshot().dockUi).toEqual({ apps: {} });
        expect(store.getSnapshot()).toBeNull();
      });
      it("returns null on corrupt JSON rather than throwing", () => {
        const storage = createMemoryStoragePort();
        storage.set("semio.os.config", "{not json");
        const store = new DockUiStateStore(storage);
        expect(() => store.getSnapshot()).not.toThrow();
        expect(store.getSnapshot()).toBeNull();
      });
      it("discards a stale version-1 (corners) blob instead of migrating it", () => {
        const storage = createMemoryStoragePort();
        new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 1, corners: {} }, apps: {} } }));
        const store = new DockUiStateStore(storage);
        expect(store.getSnapshot()).toBeNull();
      });
      it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
        const storage = createMemoryStoragePort();
        new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 2, anchors: {} }, apps: {} } }));
        const store = new DockUiStateStore(storage);
        expect(store.getSnapshot()).toBeNull();
      });
      it('keeps dock layout and dock ui as distinct projections for an app literally named "ui"', () => {
        const storage = createMemoryStoragePort();
        new DockLayoutStore(storage, "ui").save({
          version: 3,
          anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
        });
        new DockUiStateStore(storage).saveOs(emptyUiState());
        const config = new OsShellConfig(storage).getSnapshot();
        expect(config.dockLayouts.apps.ui).toBeDefined();
        expect(config.dockUi.os).toEqual(emptyUiState());
      });
    });
    describe("WindowPaneStateStore", () => {
      const emptyPaneState = () => ({ version: 1, windows: {} });
      it("returns null when nothing persisted", () => {
        const store = new WindowPaneStateStore(createMemoryStoragePort());
        expect(store.getSnapshot()).toBeNull();
      });
      it("app layer wins over os layer when both are set", () => {
        const storage = createMemoryStoragePort();
        const store = new WindowPaneStateStore(storage, "my-app");
        const osState = emptyPaneState();
        const appState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
        store.saveOs(osState);
        store.save(appState);
        expect(store.getSnapshot()).toEqual(appState);
      });
      it("falls back to os layer when app layer absent", () => {
        const storage = createMemoryStoragePort();
        const store = new WindowPaneStateStore(storage, "my-app");
        const osState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
        store.saveOs(osState);
        expect(store.getSnapshot()).toEqual(osState);
      });
      it("save(null) removes the app-layer key", () => {
        const storage = createMemoryStoragePort();
        const store = new WindowPaneStateStore(storage, "my-app");
        store.save(emptyPaneState());
        expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toEqual(emptyPaneState());
        store.save(null);
        expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toBeUndefined();
        expect(store.getSnapshot()).toBeNull();
      });
      it("reset() clears both layers", () => {
        const storage = createMemoryStoragePort();
        const store = new WindowPaneStateStore(storage, "my-app");
        store.saveOs(emptyPaneState());
        store.save(emptyPaneState());
        store.reset();
        expect(new OsShellConfig(storage).getSnapshot().windowPanes).toEqual({ apps: {} });
        expect(store.getSnapshot()).toBeNull();
      });
      it("returns null on corrupt JSON rather than throwing", () => {
        const storage = createMemoryStoragePort();
        storage.set("semio.os.config", "{not json");
        const store = new WindowPaneStateStore(storage);
        expect(() => store.getSnapshot()).not.toThrow();
        expect(store.getSnapshot()).toBeNull();
      });
      it("discards a foreign-version blob instead of migrating it", () => {
        const storage = createMemoryStoragePort();
        new OsShellConfig(storage).update((current) => ({ ...current, windowPanes: { os: { version: 2, windows: {} }, apps: {} } }));
        const store = new WindowPaneStateStore(storage);
        expect(store.getSnapshot()).toBeNull();
      });
    });
    describe("OsShellConfig", () => {
      it("consolidates all four persisted shell projections into one config document", () => {
        const values = new Map;
        const storage = {
          get: (key) => values.get(key) ?? null,
          set: (key, value) => void values.set(key, value),
          remove: (key) => void values.delete(key)
        };
        const skeleton = {
          version: 3,
          anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] }
        };
        new NamedLayoutStore("draw", storage).save({ id: "wide", label: "Wide", origin: "user", layout: { root: { kind: "stack", children: [] } } });
        new DockLayoutStore(storage, "draw").save(skeleton);
        new DockUiStateStore(storage, "draw").save({ version: 3, anchors: { "left-middle": { visible: true } } });
        new WindowPaneStateStore(storage, "draw").save({ version: 1, windows: {} });
        new OsShellConfig(storage).setPreference("ui.chrome.locale", "de");
        expect([...values.keys()]).toEqual(["semio.os.config"]);
        const snapshot = new OsShellConfig(storage).getSnapshot();
        expect(snapshot.namedLayouts.draw?.[0]?.id).toBe("wide");
        expect(snapshot.dockLayouts.apps.draw).toEqual(skeleton);
        expect(snapshot.dockUi.apps.draw?.anchors["left-middle"]?.visible).toBe(true);
        expect(snapshot.windowPanes.apps.draw).toEqual({ version: 1, windows: {} });
        expect(snapshot.preferences["ui.chrome.locale"]).toBe("de");
      });
    });
    const SYNTHETIC_PLUGIN_CATALOG = {
      plugins: [
        { pluginId: "alpha", wasmOut: "alpha.wasm", role: "plugin", contributes: [], consumes: [] },
        { pluginId: "beta", wasmOut: "beta.wasm", role: "plugin", contributes: [], consumes: [] }
      ],
      extensions: [{ pluginId: "beta-extension-gamma", wasmOut: "beta_gamma.wasm", role: "extension", contributes: ["beta.module"], consumes: [] }],
      hosts: [{ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" }],
      playgrounds: [
        { variant: "alpha", pluginId: "alpha", aliases: [] },
        { variant: "beta-play", pluginId: "beta", app: "beta-play-app", aliases: ["b", "beta play"] }
      ],
      moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
      extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`
    };
    describe("PlaygroundResolution", () => {
      it("resolves host config from the injected catalog", () => {
        expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "alpha")).toEqual({ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" });
        expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "beta-play")).toBeUndefined();
      });
      it("resolves playground aliases to registry plugin ids", () => {
        expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "b")).toBe("beta");
        expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "beta play")).toBe("beta");
      });
      it("rebuilds program rows when the generated session variant is stale", () => {
        const boot = resolvePlaygroundBoot(SYNTHETIC_PLUGIN_CATALOG, "beta-play", {
          variant: "alpha",
          defaultAppId: "alpha-app",
          plugins: [{ pluginId: "alpha", moduleUrl: "/plugin-modules/alpha/alpha_plugin.js" }]
        });
        expect(boot.variant).toBe("beta-play");
        expect(boot.defaultAppId).toBe("beta-play-app");
        expect(boot.plugins).toEqual([{ pluginId: "beta", moduleUrl: "/plugin-modules/beta/beta.js", contributes: [], consumes: [] }]);
      });
    });
    describe("effectiveActionArgs", () => {
      const TEXT_SCHEMA = { kind: "string", options: [] };
      const textArg = (id, extra = {}) => ({
        id,
        label: id,
        schema: TEXT_SCHEMA,
        required: false,
        ...extra
      });
      it("keeps a seeded arg that is not a declared form field, alongside the form's own staged fields (26/08/16 HUB-SPACES shareSpace regression: spaceId must reach the dispatched descriptor)", () => {
        const defs = [textArg("email")];
        const effective = effectiveActionArgs(defs, { email: "user2@semio.dev" }, { spaceId: "sp-1" });
        expect(effective).toEqual({ spaceId: "sp-1", email: "user2@semio.dev" });
      });
      it("a seed value for a declared field pre-fills it until the form stages its own value (renameSpace's current-name prefill)", () => {
        const defs = [textArg("name")];
        expect(effectiveActionArgs(defs, {}, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "Old Name" });
        expect(effectiveActionArgs(defs, { name: "New Name" }, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "New Name" });
      });
      it("a zero-declared-field confirm dialog passes seed+staged through wholesale (deleteSpace's confirm/cancel shape)", () => {
        expect(effectiveActionArgs([], {}, { spaceId: "sp-1", confirmed: true })).toEqual({ spaceId: "sp-1", confirmed: true });
      });
      it("missingRequiredArgs is unaffected by extra seed keys", () => {
        const defs = [textArg("email", { required: true })];
        const effective = effectiveActionArgs(defs, {}, { spaceId: "sp-1" });
        expect(missingRequiredArgs(defs, effective)).toEqual(["email"]);
      });
    });
    describe("organizeContextMenu", () => {
      const menuLeaf = (id) => ({ id, label: id, action: id });
      const menuDestructive = (id) => ({ ...menuLeaf(id), destructive: true });
      it("keeps a flat within-budget menu as-is, with groups sorted after leaves", () => {
        const items = [menuLeaf("a"), menuLeaf("b"), { id: "menu.group.view", children: [menuLeaf("c")] }];
        expect(organizeContextMenu(items, () => {
          return;
        })).toEqual(items);
      });
      it("shares the Rust fixture's grouped structure for a flat 12-item over-budget menu", () => {
        const items = [
          menuLeaf("primary0"),
          menuLeaf("primary1"),
          menuLeaf("primary2"),
          menuLeaf("primary3"),
          menuLeaf("primary4"),
          menuLeaf("overflow0"),
          menuLeaf("overflow1"),
          menuLeaf("overflow2"),
          menuLeaf("overflow3"),
          menuLeaf("overflow4"),
          menuLeaf("overflow5"),
          menuDestructive("delete")
        ];
        const categoryOf = (id) => id.startsWith("overflow") ? "view" : undefined;
        const organized = organizeContextMenu(items, categoryOf);
        expect(organized.map((item) => item.id)).toEqual([
          "primary0",
          "primary1",
          "primary2",
          "primary3",
          "primary4",
          "menu.group.view",
          "separator-organized-6",
          "delete"
        ]);
        expect(organized[5].children?.map((child) => child.id)).toEqual([
          "overflow0",
          "overflow1",
          "overflow2",
          "overflow3",
          "overflow4",
          "overflow5"
        ]);
        expect(organized[6].separator).toBe(true);
        expect(organized[6].label).toBeUndefined();
        expect(organized[7].destructive).toBe(true);
      });
    });
    describe("PluginSource", () => {
      const registry = [
        { pluginId: "note", moduleUrl: "/plugin-modules/note/note_plugin.js" },
        { pluginId: "s", moduleUrl: "/plugin-modules/s/s_plugin.js" }
      ];
      it("list() returns the registry it was created with", async () => {
        const source = createDevPluginSource(registry);
        expect(source.id).toBe("dev");
        await expect(source.list()).resolves.toEqual(registry);
      });
      it("moduleUrl() passes through unbusted without rebuiltAt", () => {
        const source = createDevPluginSource(registry);
        expect(source.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
      });
      it("moduleUrl() cache-busts with a rebuiltAt query param", () => {
        const source = createDevPluginSource(registry);
        expect(source.moduleUrl("note", 1785789943669)).toBe("/plugin-modules/note/note_plugin.js?v=1785789943669");
      });
      it("moduleUrl() throws for an unknown pluginId", () => {
        const source = createDevPluginSource(registry);
        expect(() => source.moduleUrl("missing")).toThrow(/missing/);
      });
      it("subscribe() is a harmless no-op without a global EventSource (node/vitest)", () => {
        const source = createDevPluginSource(registry);
        const events = [];
        const unsubscribe = source.subscribe((event) => events.push(event));
        expect(() => unsubscribe()).not.toThrow();
        expect(events).toEqual([]);
      });
      it("multiplexPluginSources() merges list() and resolves moduleUrl from the matching child", async () => {
        const catalog = {
          plugins: [],
          extensions: [{ pluginId: "gamma-extension", wasmOut: "gamma.wasm", role: "extension", contributes: [], consumes: [] }],
          hosts: [],
          playgrounds: [],
          moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
          extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`
        };
        const dev = createDevPluginSource(registry);
        const extensions = createExtensionSource(catalog);
        const multiplexed = multiplexPluginSources(dev, extensions);
        expect(multiplexed.id).toBe("dev+extensions");
        const listed = await multiplexed.list();
        expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...catalog.extensions.map((entry) => entry.pluginId)].sort());
        expect(multiplexed.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
        expect(() => multiplexed.moduleUrl("missing")).toThrow(/missing/);
      });
    });
    describe("ephemeralBox", () => {
      it("stores a function-typed init as the current value (not as a lazy factory)", () => {
        const identity = (id) => id;
        const box = ephemeralBox(`test.ephemeralBox.fn.${Math.random()}`, identity);
        expect(typeof box.current).toBe("function");
        expect(box.current("ui.nav.back")).toBe("ui.nav.back");
      });
      it("stores a no-op function init without invoking it", () => {
        let calls = 0;
        const noop = () => {
          calls += 1;
        };
        const box = ephemeralBox(`test.ephemeralBox.noop.${Math.random()}`, noop);
        expect(typeof box.current).toBe("function");
        expect(calls).toBe(0);
        box.current();
        expect(calls).toBe(1);
      });
      it("is owned by an isolatable, resettable OsTransient lane", () => {
        const left = new OsTransient;
        const right = new OsTransient;
        const leftBox = left.box("cursor", { x: 1 });
        leftBox.current.x = 2;
        expect(left.box("cursor", { x: 99 })).toBe(leftBox);
        expect(right.box("cursor", { x: 3 }).current.x).toBe(3);
        const oldMap = left.map("measurements");
        oldMap.set("width", 42);
        left.reset();
        expect(left.map("measurements")).not.toBe(oldMap);
        expect(left.map("measurements").size).toBe(0);
        expect(oldMap.get("width")).toBe(42);
      });
    });
    describe("LeasePool evictNow (hot-swap reload eviction)", () => {
      it("disposes a fully-released key immediately", async () => {
        const disposed = [];
        const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value), { lingerMs: 30000 });
        const lease = await pool.acquire("url-v1");
        lease.release();
        expect(disposed).toEqual([]);
        pool.evictNow("url-v1");
        expect(disposed).toEqual(["value:url-v1"]);
      });
      it("skips (does not throw) a key with an active lease, matching a reload that hasn't released the old handle yet", async () => {
        const disposed = [];
        const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value));
        const lease = await pool.acquire("url-v1");
        expect(() => pool.evictNow("url-v1")).not.toThrow();
        expect(disposed).toEqual([]);
        lease.release();
        pool.evictNow("url-v1");
        expect(disposed).toEqual(["value:url-v1"]);
      });
      it("treats two cache-busted URLs of the same pluginId as independent keys", async () => {
        const disposed = [];
        const pool = createLeasePool((key) => Promise.resolve(`value:${key}`), (value) => disposed.push(value));
        const oldLease = await pool.acquire("note.js?v=1");
        const newLease = await pool.acquire("note.js?v=2");
        oldLease.release();
        pool.evictNow("note.js?v=1");
        expect(disposed).toEqual(["value:note.js?v=1"]);
        newLease.release();
        pool.evictNow("note.js?v=2");
        expect(disposed).toEqual(["value:note.js?v=1", "value:note.js?v=2"]);
      });
    });

    class UnitFlipEvent {
      eventCount = 1;
      eventId() {
        return EventId(0);
      }
      eventName() {
        return "Flip";
      }
    }
    const UNIT_FLIP = new UnitFlipEvent;
    const UNIT_TOGGLE_NODES = [
      { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
      { stableId: "off", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
      { stableId: "on", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 }
    ];
    const UNIT_TOGGLE_TRANSITIONS = [
      { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
      { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 1 }
    ];
    const UNIT_TOGGLE_MACHINE = {
      definition: { id: "unit_toggle", nodes: UNIT_TOGGLE_NODES, transitions: UNIT_TOGGLE_TRANSITIONS, contextFromInput: () => ({ count: 0 }), guards: [], actions: [], fingerprint: 42n, manifestJson: "{}" }
    };
    const TOGGLE_MACHINE = {
      definition: {
        id: "toggle",
        nodes: UNIT_TOGGLE_NODES,
        transitions: [
          { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 0 },
          { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, guard: GuardId(0), targets: [NodeId(1)], kind: "external", actions: [ActionId(0)], docIndex: 1 }
        ],
        contextFromInput: (allow) => ({ count: 0, allow }),
        guards: [(ctx) => ctx.allow],
        actions: [(ctx) => ctx.count += 1],
        fingerprint: 1n,
        manifestJson: "{}"
      }
    };

    class PlayerEvent {
      static IDS = { open: 0, pause: 1, play: 2, stop: 3, resume: 4 };
      static NAMES = ["Open", "Pause", "Play", "Stop", "Resume"];
      eventCount = 5;
      type;
      constructor(type) {
        this.type = type;
      }
      eventId() {
        return EventId(PlayerEvent.IDS[this.type]);
      }
      eventName(id) {
        return PlayerEvent.NAMES[id] ?? "?";
      }
    }
    const PLAYER_MACHINE = {
      definition: {
        id: "player",
        nodes: [
          { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(3)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
          { stableId: "closed", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
          { stableId: "playing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
          { stableId: "open", kind: "compound", parent: NodeId(0), initial: NodeId(2), children: [NodeId(2), NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
          { stableId: "paused", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
          { stableId: "open.history", kind: "historyShallow", parent: NodeId(3), initial: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 }
        ],
        transitions: [
          { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(3)], kind: "external", actions: [], docIndex: 0 },
          { source: NodeId(2), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 1 },
          { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 2 },
          { source: NodeId(3), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
          { source: NodeId(1), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 }
        ],
        contextFromInput: () => ({}),
        guards: [],
        actions: [],
        fingerprint: 2n,
        manifestJson: "{}"
      }
    };

    class RecorderEvent {
      static IDS = { start: 0, audioStop: 1, videoStop: 2 };
      static NAMES = ["Start", "AudioStop", "VideoStop"];
      eventCount = 3;
      type;
      constructor(type) {
        this.type = type;
      }
      eventId() {
        return EventId(RecorderEvent.IDS[this.type]);
      }
      eventName(id) {
        return RecorderEvent.NAMES[id] ?? "?";
      }
    }
    const RECORDER_MACHINE = {
      definition: {
        id: "recorder",
        nodes: [
          { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
          { stableId: "idle", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
          { stableId: "recording", kind: "parallel", parent: NodeId(0), children: [NodeId(3), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
          { stableId: "audio", kind: "compound", parent: NodeId(2), initial: NodeId(4), children: [NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
          { stableId: "audio.capturing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
          { stableId: "audio.done", kind: "final", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
          { stableId: "video", kind: "compound", parent: NodeId(2), initial: NodeId(7), children: [NodeId(7), NodeId(8)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
          { stableId: "video.capturing", kind: "atomic", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
          { stableId: "video.done", kind: "final", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 }
        ],
        transitions: [
          { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
          { source: NodeId(4), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 1 },
          { source: NodeId(7), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(8)], kind: "external", actions: [], docIndex: 2 },
          { source: NodeId(2), trigger: { kind: "done", node: NodeId(2) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 }
        ],
        contextFromInput: () => ({}),
        guards: [],
        actions: [],
        fingerprint: 3n,
        manifestJson: "{}"
      }
    };

    class CheckoutEvent {
      static IDS = { confirm: 0, selectMethod: 1, paymentSucceeded: 2, paymentFailed: 3, retry: 4, cancel: 5, resume: 6, shipDone: 7, invoiceDone: 8 };
      static NAMES = ["Confirm", "SelectMethod", "PaymentSucceeded", "PaymentFailed", "Retry", "Cancel", "Resume", "ShipDone", "InvoiceDone"];
      eventCount = 9;
      type;
      constructor(type) {
        this.type = type;
      }
      eventId() {
        return EventId(CheckoutEvent.IDS[this.type]);
      }
      eventName(id) {
        return CheckoutEvent.NAMES[id] ?? "?";
      }
    }
    const CHECKOUT_MACHINE = {
      definition: {
        id: "checkout",
        nodes: [
          { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2), NodeId(7), NodeId(14)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
          { stableId: "cart", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
          { stableId: "payment", kind: "compound", parent: NodeId(0), initial: NodeId(3), children: [NodeId(3), NodeId(4), NodeId(5), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
          { stableId: "selecting", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
          { stableId: "processing", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [InvokeId(0)], timers: [[TimerId(0), 5000]], docIndex: 4 },
          { stableId: "failed", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
          { stableId: "payment_history", kind: "historyShallow", parent: NodeId(2), initial: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
          { stableId: "fulfilment", kind: "parallel", parent: NodeId(0), children: [NodeId(8), NodeId(11)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
          { stableId: "shipping", kind: "compound", parent: NodeId(7), initial: NodeId(9), children: [NodeId(9), NodeId(10)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
          { stableId: "ship_pending", kind: "atomic", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 9 },
          { stableId: "ship_done", kind: "final", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 10 },
          { stableId: "invoicing", kind: "compound", parent: NodeId(7), initial: NodeId(12), children: [NodeId(12), NodeId(13)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 11 },
          { stableId: "invoice_pending", kind: "atomic", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 12 },
          { stableId: "invoice_done", kind: "final", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 13 },
          { stableId: "done", kind: "final", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 14 }
        ],
        transitions: [
          { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
          { source: NodeId(1), trigger: { kind: "event", event: EventId(6) }, targets: [NodeId(6)], kind: "external", actions: [], docIndex: 1 },
          { source: NodeId(3), trigger: { kind: "event", event: EventId(1) }, guard: GuardId(0), targets: [NodeId(4)], kind: "external", actions: [ActionId(0)], docIndex: 2 },
          { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(7)], kind: "external", actions: [], docIndex: 3 },
          { source: NodeId(4), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
          { source: NodeId(4), trigger: { kind: "event", event: EventId(5) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 5 },
          { source: NodeId(4), trigger: { kind: "timer", timer: TimerId(0) }, targets: [NodeId(5)], kind: "external", actions: [ActionId(1)], docIndex: 6 },
          { source: NodeId(5), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 7 },
          { source: NodeId(9), trigger: { kind: "event", event: EventId(7) }, targets: [NodeId(10)], kind: "external", actions: [], docIndex: 8 },
          { source: NodeId(12), trigger: { kind: "event", event: EventId(8) }, targets: [NodeId(13)], kind: "external", actions: [], docIndex: 9 },
          { source: NodeId(7), trigger: { kind: "done", node: NodeId(7) }, targets: [NodeId(14)], kind: "external", actions: [], docIndex: 10 }
        ],
        contextFromInput: () => ({ attempts: 0, methodSet: false }),
        makeOutput: (ctx) => ({ attempts: ctx.attempts }),
        guards: [(ctx) => ctx.attempts < 3],
        actions: [(ctx) => ctx.methodSet = true, (ctx) => ctx.attempts += 1],
        fingerprint: 100n,
        manifestJson: "{}"
      }
    };
    describe("machine: TestHost", () => {
      it("advance fires due timers only", () => {
        const host = new TestHost;
        host.schedule(ActorId(0), TimerId(0), 100);
        host.schedule(ActorId(0), TimerId(1), 300);
        expect(host.advance(150)).toEqual([[0, 0]]);
        expect(host.advance(200)).toEqual([[0, 1]]);
      });
      it("cancelTimer removes pending", () => {
        const host = new TestHost;
        host.schedule(ActorId(0), TimerId(0), 100);
        host.cancelTimer(ActorId(0), TimerId(0));
        expect(host.advance(200)).toEqual([]);
      });
      it("records effects and task lifecycle", () => {
        const host = new TestHost;
        host.executeEffect(ActorId(0), "audit");
        expect(host.effects()).toEqual([[0, "audit"]]);
        host.startTask(ActorId(0), InvokeId(0));
        expect(host.startedTasks()).toEqual([[0, 0]]);
        host.cancelTask(ActorId(0), InvokeId(0));
        expect(host.startedTasks()).toEqual([]);
        expect(host.cancelledTasks()).toEqual([[0, 0]]);
      });
    });
    describe("machine: TraceInspector", () => {
      it("records one microstep per transition", () => {
        const sink = [];
        const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
        const inspector = new TraceInspector;
        macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        expect(inspector.entries.length).toBe(2);
        expect(inspector.entries[0].exited).toEqual([NodeId(1)]);
        expect(inspector.entries[0].entered).toEqual([NodeId(2)]);
        expect(inspector.entries[1].exited).toEqual([NodeId(2)]);
        expect(inspector.entries[1].entered).toEqual([NodeId(1)]);
      });
    });
    describe("machine: kernel", () => {
      it("flat machine toggles and counts", () => {
        const sink = [];
        const snapshot = init(TOGGLE_MACHINE, true, sink);
        expect(snapshot.matches("off")).toBe(true);
        const inspector = new NullInspector;
        macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        expect(snapshot.matches("on")).toBe(true);
        expect(snapshot.context.count).toBe(1);
        macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        expect(snapshot.matches("off")).toBe(true);
        expect(snapshot.context.count).toBe(2);
      });
      it("guard blocks transition when false", () => {
        const sink = [];
        const snapshot = init(TOGGLE_MACHINE, false, sink);
        const inspector = new NullInspector;
        macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        expect(snapshot.matches("on")).toBe(true);
        macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
        expect(snapshot.matches("on")).toBe(true);
        expect(snapshot.context.count).toBe(1);
      });
      it("hierarchical machine enters default descendant", () => {
        const sink = [];
        const snapshot = init(PLAYER_MACHINE, undefined, sink);
        expect(snapshot.matches("closed")).toBe(true);
        expect(snapshot.matches("open")).toBe(false);
      });
      it("hierarchical machine transitions into compound default", () => {
        const sink = [];
        const snapshot = init(PLAYER_MACHINE, undefined, sink);
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, new NullInspector);
        expect(snapshot.matches("open")).toBe(true);
        expect(snapshot.matches("playing")).toBe(true);
      });
      it("shallow history restores last active child", () => {
        const sink = [];
        const snapshot = init(PLAYER_MACHINE, undefined, sink);
        const inspector = new NullInspector;
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, inspector);
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("pause"), sink, inspector);
        expect(snapshot.matches("paused")).toBe(true);
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("stop"), sink, inspector);
        expect(snapshot.matches("closed")).toBe(true);
        expect(snapshot.matches("open")).toBe(false);
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("resume"), sink, inspector);
        expect(snapshot.matches("open")).toBe(true);
        expect(snapshot.matches("paused")).toBe(true);
        expect(snapshot.matches("playing")).toBe(false);
        macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("play"), sink, inspector);
        expect(snapshot.matches("playing")).toBe(true);
        expect(snapshot.matches("paused")).toBe(false);
      });
      it("parallel regions enter together", () => {
        const sink = [];
        const snapshot = init(RECORDER_MACHINE, undefined, sink);
        macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, new NullInspector);
        expect(snapshot.matches("recording")).toBe(true);
        expect(snapshot.matches("audio.capturing")).toBe(true);
        expect(snapshot.matches("video.capturing")).toBe(true);
      });
      it("parallel done bubbles only once every region finishes", () => {
        const sink = [];
        const snapshot = init(RECORDER_MACHINE, undefined, sink);
        const inspector = new NullInspector;
        macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, inspector);
        macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("audioStop"), sink, inspector);
        expect(snapshot.matches("audio.done")).toBe(true);
        expect(snapshot.matches("recording")).toBe(true);
        macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("videoStop"), sink, inspector);
        expect(snapshot.matches("idle")).toBe(true);
        expect(snapshot.matches("recording")).toBe(false);
      });
    });
    describe("machine: persist/restore", () => {
      it("persist then restore round-trips active state", () => {
        const sink = [];
        const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
        macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, new NullInspector);
        expect(snapshot.matches("on")).toBe(true);
        const persisted = persist(UNIT_TOGGLE_MACHINE, snapshot);
        expect(persisted.fingerprint).toBe(UNIT_TOGGLE_MACHINE.definition.fingerprint);
        expect(persisted.states).toContain("on");
        const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
        expect(restored.ok).toBe(true);
        expect(restored.ok && restored.snapshot.matches("on")).toBe(true);
      });
      it("restore rejects fingerprint mismatch without migration", () => {
        const sink = [];
        const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
        const persisted = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
        const result = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
        expect(result).toEqual({ ok: false, error: { kind: "fingerprintMismatch" } });
      });
      it("restore applies migration chain until fingerprint matches", () => {
        const sink = [];
        const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
        const persisted = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
        const migration = { sourceFingerprint: 9999n, migrate: (s) => ({ ...s, fingerprint: UNIT_TOGGLE_MACHINE.definition.fingerprint }) };
        const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, [migration]);
        expect(restored.ok).toBe(true);
        expect(restored.ok && restored.snapshot.matches("off")).toBe(true);
      });
    });
    describe("machine: ActorSystem", () => {
      it("drains sent events through one macrostep each", () => {
        const system = new ActorSystem(new TestHost, UNIT_TOGGLE_MACHINE);
        const root = system.spawnRoot(undefined);
        expect(system.snapshot(root).matches("off")).toBe(true);
        system.send(root, UNIT_FLIP);
        const reports = system.drain();
        expect(reports.length).toBe(1);
        expect(system.snapshot(root).matches("on")).toBe(true);
        system.send(root, UNIT_FLIP);
        system.drain();
        expect(system.snapshot(root).matches("off")).toBe(true);
        expect(system.snapshot(root).context).toEqual({ count: 0 });
      });
    });
    describe("machine: testing (Model/Coverage/Invariant/Conformance)", () => {
      it("explore reaches both toggle states", () => {
        const model = new Model([UNIT_FLIP]);
        const coverage = explore(UNIT_TOGGLE_MACHINE, model, undefined);
        expect(coverage.reachedStableIds).toContain("off");
        expect(coverage.reachedStableIds).toContain("on");
        expect(coverage.visitedConfigurations).toBe(2);
      });
      it("conformance fixture passes for matching sequence", () => {
        const steps = [
          { event: UNIT_FLIP, expectActive: ["on"] },
          { event: UNIT_FLIP, expectActive: ["off"] }
        ];
        expect(runConformance(UNIT_TOGGLE_MACHINE, undefined, steps).ok).toBe(true);
      });
      it("conformance fixture fails with a descriptive message", () => {
        const steps = [{ event: UNIT_FLIP, expectActive: ["off"] }];
        const result = runConformance(UNIT_TOGGLE_MACHINE, undefined, steps);
        expect(result.ok).toBe(false);
        expect(!result.ok && result.error.message).toContain("step 0");
        expect(!result.ok && result.error.message).toContain("off");
      });
      it("invariant reports violation by name", () => {
        const sink = [];
        const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
        const invariants = [{ name: "never off", check: (s) => s.matches("off") ? { ok: false, error: { kind: "violation", message: "was off" } } : { ok: true } }];
        expect(checkInvariants(snapshot, invariants)).toEqual(["never off: was off"]);
      });
    });
    describe("machine: BitSet", () => {
      it("set/clear/contains", () => {
        const bits = new BitSet;
        expect(bits.contains(NodeId(3))).toBe(false);
        bits.set(NodeId(3));
        expect(bits.contains(NodeId(3))).toBe(true);
        bits.clear(NodeId(3));
        expect(bits.contains(NodeId(3))).toBe(false);
      });
      it("iterOnes ascends regardless of insertion order", () => {
        const bits = new BitSet;
        bits.set(NodeId(100));
        bits.set(NodeId(0));
        bits.set(NodeId(64));
        bits.set(NodeId(63));
        expect([...bits.iterOnes()]).toEqual([0, 63, 64, 100]);
      });
      it("clearAll and isEmpty", () => {
        const bits = new BitSet;
        expect(bits.isEmpty()).toBe(true);
        bits.set(NodeId(5));
        expect(bits.isEmpty()).toBe(false);
        bits.clearAll();
        expect(bits.isEmpty()).toBe(true);
      });
    });
    describe("machine: checkout DSL twin (integration)", () => {
      it("walks cart to receipt", () => {
        const host = new TestHost;
        const system = new ActorSystem(host, CHECKOUT_MACHINE);
        const root = system.spawnRoot(undefined);
        expect(system.snapshot(root).matches("cart")).toBe(true);
        system.send(root, new CheckoutEvent("confirm"));
        system.drain();
        expect(system.snapshot(root).matches("selecting")).toBe(true);
        system.send(root, new CheckoutEvent("selectMethod"));
        system.drain();
        expect(system.snapshot(root).matches("processing")).toBe(true);
        expect(system.snapshot(root).context.methodSet).toBe(true);
        expect(host.startedTasks()).toEqual([[root, 0]]);
        system.send(root, new CheckoutEvent("paymentSucceeded"));
        system.drain();
        expect(system.snapshot(root).matches("ship_pending")).toBe(true);
        expect(system.snapshot(root).matches("invoice_pending")).toBe(true);
        expect(host.cancelledTasks()).toEqual([[root, 0]]);
        system.send(root, new CheckoutEvent("shipDone"));
        system.drain();
        expect(system.snapshot(root).matches("ship_done")).toBe(true);
        expect(system.snapshot(root).matches("invoice_pending")).toBe(true);
        system.send(root, new CheckoutEvent("invoiceDone"));
        system.drain();
        const finalStatus = system.snapshot(root).status;
        expect(finalStatus.kind).toBe("done");
        expect(finalStatus.kind === "done" && finalStatus.output.attempts).toBe(0);
      });
      it("cancel/resume round-trips via shallow history", () => {
        const sink = [];
        const snapshot = init(CHECKOUT_MACHINE, undefined, sink);
        const inspector = new TraceInspector;
        macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("confirm"), sink, inspector);
        macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("selectMethod"), sink, inspector);
        expect(snapshot.matches("processing")).toBe(true);
        macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("cancel"), sink, inspector);
        expect(snapshot.matches("cart")).toBe(true);
        expect(snapshot.matches("payment")).toBe(false);
        macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("resume"), sink, inspector);
        expect(snapshot.matches("processing")).toBe(true);
        expect(snapshot.matches("selecting")).toBe(false);
        expect(inspector.entries.length).toBeGreaterThan(0);
        const fired = timerElapsed(CHECKOUT_MACHINE, snapshot, TimerId(0), sink, inspector);
        expect(fired.microsteps).toBe(1);
        expect(snapshot.matches("failed")).toBe(true);
        expect(snapshot.context.attempts).toBe(1);
        macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("retry"), sink, inspector);
        expect(snapshot.matches("processing")).toBe(true);
        const persisted = persist(CHECKOUT_MACHINE, snapshot);
        expect(persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
        const restored = restore(CHECKOUT_MACHINE, persisted, { ...snapshot.context }, []);
        expect(restored.ok).toBe(true);
        expect(restored.ok && restored.snapshot.matches("processing")).toBe(true);
      });
      it("model coverage reaches every declared state", () => {
        const model = new Model(["confirm", "selectMethod", "paymentSucceeded", "paymentFailed", "retry", "cancel", "resume", "shipDone", "invoiceDone"].map((type) => new CheckoutEvent(type)));
        const coverage = explore(CHECKOUT_MACHINE, model, undefined);
        for (const expected of ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"]) {
          expect(coverage.reachedStableIds).toContain(expected);
        }
      });
      it("start produces a persistable initial configuration", () => {
        const initial = start(CHECKOUT_MACHINE, undefined);
        expect(initial.isActive("cart")).toBe(true);
        expect(initial.entered).toEqual([]);
        expect(initial.persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
      });
      it("step round-trips through persisted state only", () => {
        let carried = start(CHECKOUT_MACHINE, undefined).persisted;
        let context = { attempts: 0, methodSet: false };
        for (const [type, expected] of [
          ["confirm", "selecting"],
          ["selectMethod", "processing"]
        ]) {
          const outcome = step(CHECKOUT_MACHINE, carried, context, new CheckoutEvent(type), []);
          expect(outcome.ok).toBe(true);
          if (!outcome.ok)
            continue;
          expect(outcome.step.isActive(expected)).toBe(true);
          context = { ...context, methodSet: true };
          carried = outcome.step.persisted;
        }
      });
      it("step reports entered and exited states", () => {
        const initial = start(CHECKOUT_MACHINE, undefined).persisted;
        const outcome = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
        expect(outcome.ok).toBe(true);
        if (!outcome.ok)
          return;
        expect(outcome.step.exited).toContain("cart");
        expect(outcome.step.entered).toContain("payment");
        expect(outcome.step.entered).toContain("selecting");
        expect(outcome.step.isActive("cart")).toBe(false);
      });
      it("step with a blocked guard leaves the configuration untouched", () => {
        const initial = start(CHECKOUT_MACHINE, undefined).persisted;
        const confirmed = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
        expect(confirmed.ok).toBe(true);
        if (!confirmed.ok)
          return;
        const blocked = step(CHECKOUT_MACHINE, confirmed.step.persisted, { attempts: 3, methodSet: false }, new CheckoutEvent("selectMethod"), []);
        expect(blocked.ok).toBe(true);
        if (!blocked.ok)
          return;
        expect(blocked.step.isActive("selecting")).toBe(true);
        expect(blocked.step.entered).toEqual([]);
      });
      it("step rejects a persisted snapshot from another machine shape", () => {
        const foreign = { ...start(CHECKOUT_MACHINE, undefined).persisted };
        const mismatched = { ...foreign, fingerprint: foreign.fingerprint ^ 0xffffffffn };
        const outcome = step(CHECKOUT_MACHINE, mismatched, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
        expect(outcome.ok).toBe(false);
      });
    });
    describe("retryWithJitteredBackoff", () => {
      it("keeps every backoff delay within [minMs, maxMs] and varies it across attempts", async () => {
        vi.useFakeTimers();
        try {
          const setTimeoutSpy = vi.spyOn(globalThis, "setTimeout");
          let calls = 0;
          const fn = vi.fn(async () => {
            calls += 1;
            if (calls < 6)
              throw new Error("retry me");
            return "ok";
          });
          const resultPromise = retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 100 });
          for (let i = 0;i < 5; i++) {
            await vi.advanceTimersByTimeAsync(100);
          }
          await expect(resultPromise).resolves.toBe("ok");
          expect(calls).toBe(6);
          const delays = setTimeoutSpy.mock.calls.map(([, ms]) => Number(ms));
          expect(delays.length).toBe(5);
          for (const delay of delays) {
            expect(delay).toBeGreaterThanOrEqual(10);
            expect(delay).toBeLessThanOrEqual(100);
          }
          expect(new Set(delays).size).toBeGreaterThan(1);
        } finally {
          vi.useRealTimers();
        }
      });
      it("rejects promptly on abort mid-wait and makes no further attempt", async () => {
        vi.useFakeTimers();
        try {
          const controller = new AbortController;
          let calls = 0;
          const fn = vi.fn(async () => {
            calls += 1;
            throw new Error("always fails");
          });
          const promise = retryWithJitteredBackoff(fn, { minMs: 50, maxMs: 50, signal: controller.signal });
          await vi.advanceTimersByTimeAsync(0);
          expect(calls).toBe(1);
          controller.abort(new Error("stop"));
          await expect(promise).rejects.toThrow("stop");
          expect(calls).toBe(1);
        } finally {
          vi.useRealTimers();
        }
      });
      it("throws immediately for an already-aborted signal without calling fn", async () => {
        const controller = new AbortController;
        controller.abort(new Error("pre-aborted"));
        const fn = vi.fn(async () => "unreachable");
        await expect(retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 10, signal: controller.signal })).rejects.toThrow("pre-aborted");
        expect(fn).not.toHaveBeenCalled();
      });
    });
    describe("latestWins", () => {
      it("collapses N concurrent calls during an in-flight run into exactly one trailing run, and every caller sees the latest result", async () => {
        let calls = 0;
        const resolvers = [];
        const run = vi.fn(() => {
          calls += 1;
          const callIndex = calls;
          return new Promise((resolve) => resolvers.push(() => resolve(callIndex)));
        });
        const trigger = latestWins(run);
        const first = trigger();
        expect(calls).toBe(1);
        const concurrentA = trigger();
        const concurrentB = trigger();
        const concurrentC = trigger();
        expect(calls).toBe(1);
        resolvers[0](0);
        await new Promise((resolve) => setTimeout(resolve, 0));
        expect(calls).toBe(2);
        resolvers[1](0);
        await expect(first).resolves.toBe(1);
        await expect(concurrentA).resolves.toBe(2);
        await expect(concurrentB).resolves.toBe(2);
        await expect(concurrentC).resolves.toBe(2);
      });
      it("starts a fresh run when called again after everything has settled", async () => {
        let n = 0;
        const trigger = latestWins(async () => {
          n += 1;
          return n;
        });
        await expect(trigger()).resolves.toBe(1);
        await expect(trigger()).resolves.toBe(2);
      });
    });
    describe("fetchWithTimeout", () => {
      it("aborts the underlying fetch when the timeout elapses first", async () => {
        vi.useFakeTimers();
        try {
          const fetchMock = vi.fn((_url, init2) => new Promise((_resolve, reject) => {
            init2?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
          }));
          vi.stubGlobal("fetch", fetchMock);
          const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 50 });
          const assertion = expect(promise).rejects.toThrow();
          await vi.advanceTimersByTimeAsync(50);
          await assertion;
        } finally {
          vi.unstubAllGlobals();
          vi.useRealTimers();
        }
      });
      it("aborts the underlying fetch when the external signal aborts", async () => {
        const controller = new AbortController;
        const fetchMock = vi.fn((_url, init2) => new Promise((_resolve, reject) => {
          init2?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
        }));
        vi.stubGlobal("fetch", fetchMock);
        try {
          const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1e4, signal: controller.signal });
          controller.abort(new Error("caller cancelled"));
          await expect(promise).rejects.toThrow();
        } finally {
          vi.unstubAllGlobals();
        }
      });
      it("clears the timer and removes the abort listener on the success path", async () => {
        vi.useFakeTimers();
        try {
          const clearTimeoutSpy = vi.spyOn(globalThis, "clearTimeout");
          const controller = new AbortController;
          const removeEventListenerSpy = vi.spyOn(controller.signal, "removeEventListener");
          const response = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
          vi.stubGlobal("fetch", vi.fn(async () => response));
          const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1000, signal: controller.signal });
          expect(result).toBe(response);
          expect(clearTimeoutSpy).toHaveBeenCalled();
          expect(removeEventListenerSpy).toHaveBeenCalledWith("abort", expect.any(Function));
          await vi.advanceTimersByTimeAsync(5000);
        } finally {
          vi.unstubAllGlobals();
          vi.useRealTimers();
        }
      });
    });
    describe("waitForEvent", () => {
      it("resolves with the first delivered value and unsubscribes", async () => {
        let handler;
        let unsubscribed = false;
        const subscribe = (h) => {
          handler = h;
          return () => {
            unsubscribed = true;
          };
        };
        const promise = waitForEvent(subscribe);
        expect(unsubscribed).toBe(false);
        handler(42);
        await expect(promise).resolves.toBe(42);
        expect(unsubscribed).toBe(true);
      });
      it("rejects on abort and unsubscribes without ever having received a value", async () => {
        let unsubscribed = false;
        const subscribe = () => () => {
          unsubscribed = true;
        };
        const controller = new AbortController;
        const promise = waitForEvent(subscribe, { signal: controller.signal });
        controller.abort(new Error("cancelled"));
        await expect(promise).rejects.toThrow("cancelled");
        expect(unsubscribed).toBe(true);
      });
      it("rejects immediately for an already-aborted signal, never subscribing", async () => {
        const controller = new AbortController;
        controller.abort(new Error("pre-aborted"));
        let subscribed = false;
        const subscribe = () => {
          subscribed = true;
          return () => {};
        };
        await expect(waitForEvent(subscribe, { signal: controller.signal })).rejects.toThrow("pre-aborted");
        expect(subscribed).toBe(false);
      });
    });
  }
});

// node:url
var exports_url = {};
__export(exports_url, {
  resolveObject: () => urlResolveObject,
  resolve: () => urlResolve,
  parse: () => urlParse,
  format: () => urlFormat,
  default: () => url_default,
  Url: () => Url,
  URLSearchParams: () => URLSearchParams2,
  URL: () => URL2
});
function util_isString(arg) {
  return typeof arg === "string";
}
function util_isObject(arg) {
  return typeof arg === "object" && arg !== null;
}
function util_isNull(arg) {
  return arg === null;
}
function util_isNullOrUndefined(arg) {
  return arg == null;
}
function Url() {
  this.protocol = null, this.slashes = null, this.auth = null, this.host = null, this.port = null, this.hostname = null, this.hash = null, this.search = null, this.query = null, this.pathname = null, this.path = null, this.href = null;
}
function urlParse(url, parseQueryString, slashesDenoteHost) {
  if (url && util_isObject(url) && url instanceof Url)
    return url;
  var u = new Url;
  return u.parse(url, parseQueryString, slashesDenoteHost), u;
}
function urlFormat(obj) {
  if (util_isString(obj))
    obj = urlParse(obj);
  if (!(obj instanceof Url))
    return Url.prototype.format.call(obj);
  return obj.format();
}
function urlResolve(source, relative) {
  return urlParse(source, false, true).resolve(relative);
}
function urlResolveObject(source, relative) {
  if (!source)
    return relative;
  return urlParse(source, false, true).resolveObject(relative);
}
var URL2, URLSearchParams2, protocolPattern, portPattern, simplePathPattern, delims, unwise, autoEscape, nonHostChars, hostEndingChars, hostnameMaxLen = 255, hostnamePartPattern, hostnamePartStart, unsafeProtocol, hostlessProtocol, slashedProtocol, querystring, url_default;
var init_url = __esm(() => {
  ({ URL: URL2, URLSearchParams: URLSearchParams2 } = globalThis);
  protocolPattern = /^([a-z0-9.+-]+:)/i;
  portPattern = /:[0-9]*$/;
  simplePathPattern = /^(\/\/?(?!\/)[^\?\s]*)(\?[^\s]*)?$/;
  delims = ["<", ">", '"', "`", " ", "\r", `
`, "\t"];
  unwise = ["{", "}", "|", "\\", "^", "`"].concat(delims);
  autoEscape = ["'"].concat(unwise);
  nonHostChars = ["%", "/", "?", ";", "#"].concat(autoEscape);
  hostEndingChars = ["/", "?", "#"];
  hostnamePartPattern = /^[+a-z0-9A-Z_-]{0,63}$/;
  hostnamePartStart = /^([+a-z0-9A-Z_-]{0,63})(.*)$/;
  unsafeProtocol = { javascript: true, "javascript:": true };
  hostlessProtocol = { javascript: true, "javascript:": true };
  slashedProtocol = { http: true, https: true, ftp: true, gopher: true, file: true, "http:": true, "https:": true, "ftp:": true, "gopher:": true, "file:": true };
  querystring = { parse(str) {
    var decode = decodeURIComponent;
    return (str + "").replace(/\+/g, " ").split("&").filter(Boolean).reduce(function(obj, item, index) {
      var ref = item.split("="), key = decode(ref[0] || ""), val = decode(ref[1] || ""), prev = obj[key];
      return obj[key] = prev === undefined ? val : [].concat(prev, val), obj;
    }, {});
  }, stringify(obj) {
    var encode = encodeURIComponent;
    return Object.keys(obj || {}).reduce(function(arr, key) {
      return [].concat(obj[key]).forEach(function(v) {
        arr.push(encode(key) + "=" + encode(v));
      }), arr;
    }, []).join("&").replace(/\s/g, "+");
  } };
  Url.prototype.parse = function(url, parseQueryString, slashesDenoteHost) {
    if (!util_isString(url))
      throw TypeError("Parameter 'url' must be a string, not " + typeof url);
    var queryIndex = url.indexOf("?"), splitter = queryIndex !== -1 && queryIndex < url.indexOf("#") ? "?" : "#", uSplit = url.split(splitter), slashRegex = /\\/g;
    uSplit[0] = uSplit[0].replace(slashRegex, "/"), url = uSplit.join(splitter);
    var rest = url;
    if (rest = rest.trim(), !slashesDenoteHost && url.split("#").length === 1) {
      var simplePath = simplePathPattern.exec(rest);
      if (simplePath) {
        if (this.path = rest, this.href = rest, this.pathname = simplePath[1], simplePath[2])
          if (this.search = simplePath[2], parseQueryString)
            this.query = querystring.parse(this.search.substr(1));
          else
            this.query = this.search.substr(1);
        else if (parseQueryString)
          this.search = "", this.query = {};
        return this;
      }
    }
    var proto = protocolPattern.exec(rest);
    if (proto) {
      proto = proto[0];
      var lowerProto = proto.toLowerCase();
      this.protocol = lowerProto, rest = rest.substr(proto.length);
    }
    if (slashesDenoteHost || proto || rest.match(/^\/\/[^@\/]+@[^@\/]+/)) {
      var slashes = rest.substr(0, 2) === "//";
      if (slashes && !(proto && hostlessProtocol[proto]))
        rest = rest.substr(2), this.slashes = true;
    }
    if (!hostlessProtocol[proto] && (slashes || proto && !slashedProtocol[proto])) {
      var hostEnd = -1;
      for (var i = 0;i < hostEndingChars.length; i++) {
        var hec = rest.indexOf(hostEndingChars[i]);
        if (hec !== -1 && (hostEnd === -1 || hec < hostEnd))
          hostEnd = hec;
      }
      var auth, atSign;
      if (hostEnd === -1)
        atSign = rest.lastIndexOf("@");
      else
        atSign = rest.lastIndexOf("@", hostEnd);
      if (atSign !== -1)
        auth = rest.slice(0, atSign), rest = rest.slice(atSign + 1), this.auth = decodeURIComponent(auth);
      hostEnd = -1;
      for (var i = 0;i < nonHostChars.length; i++) {
        var hec = rest.indexOf(nonHostChars[i]);
        if (hec !== -1 && (hostEnd === -1 || hec < hostEnd))
          hostEnd = hec;
      }
      if (hostEnd === -1)
        hostEnd = rest.length;
      this.host = rest.slice(0, hostEnd), rest = rest.slice(hostEnd), this.parseHost(), this.hostname = this.hostname || "";
      var ipv6Hostname = this.hostname[0] === "[" && this.hostname[this.hostname.length - 1] === "]";
      if (!ipv6Hostname) {
        var hostparts = this.hostname.split(/\./);
        for (var i = 0, l = hostparts.length;i < l; i++) {
          var part = hostparts[i];
          if (!part)
            continue;
          if (!part.match(hostnamePartPattern)) {
            var newpart = "";
            for (var j = 0, k = part.length;j < k; j++)
              if (part.charCodeAt(j) > 127)
                newpart += "x";
              else
                newpart += part[j];
            if (!newpart.match(hostnamePartPattern)) {
              var validParts = hostparts.slice(0, i), notHost = hostparts.slice(i + 1), bit = part.match(hostnamePartStart);
              if (bit)
                validParts.push(bit[1]), notHost.unshift(bit[2]);
              if (notHost.length)
                rest = "/" + notHost.join(".") + rest;
              this.hostname = validParts.join(".");
              break;
            }
          }
        }
      }
      if (this.hostname.length > hostnameMaxLen)
        this.hostname = "";
      else
        this.hostname = this.hostname.toLowerCase();
      if (!ipv6Hostname)
        this.hostname = new URL2(`https://${this.hostname}`).hostname;
      var p = this.port ? ":" + this.port : "", h = this.hostname || "";
      if (this.host = h + p, this.href += this.host, ipv6Hostname) {
        if (this.hostname = this.hostname.substr(1, this.hostname.length - 2), rest[0] !== "/")
          rest = "/" + rest;
      }
    }
    if (!unsafeProtocol[lowerProto])
      for (var i = 0, l = autoEscape.length;i < l; i++) {
        var ae = autoEscape[i];
        if (rest.indexOf(ae) === -1)
          continue;
        var esc = encodeURIComponent(ae);
        if (esc === ae)
          esc = escape(ae);
        rest = rest.split(ae).join(esc);
      }
    var hash = rest.indexOf("#");
    if (hash !== -1)
      this.hash = rest.substr(hash), rest = rest.slice(0, hash);
    var qm = rest.indexOf("?");
    if (qm !== -1) {
      if (this.search = rest.substr(qm), this.query = rest.substr(qm + 1), parseQueryString)
        this.query = querystring.parse(this.query);
      rest = rest.slice(0, qm);
    } else if (parseQueryString)
      this.search = "", this.query = {};
    if (rest)
      this.pathname = rest;
    if (slashedProtocol[lowerProto] && this.hostname && !this.pathname)
      this.pathname = "/";
    if (this.pathname || this.search) {
      var p = this.pathname || "", s = this.search || "";
      this.path = p + s;
    }
    return this.href = this.format(), this;
  };
  Url.prototype.format = function() {
    var auth = this.auth || "";
    if (auth)
      auth = encodeURIComponent(auth), auth = auth.replace(/%3A/i, ":"), auth += "@";
    var protocol = this.protocol || "", pathname = this.pathname || "", hash = this.hash || "", host = false, query = "";
    if (this.host)
      host = auth + this.host;
    else if (this.hostname) {
      if (host = auth + (this.hostname.indexOf(":") === -1 ? this.hostname : "[" + this.hostname + "]"), this.port)
        host += ":" + this.port;
    }
    if (this.query && util_isObject(this.query) && Object.keys(this.query).length)
      query = querystring.stringify(this.query);
    var search = this.search || query && "?" + query || "";
    if (protocol && protocol.substr(-1) !== ":")
      protocol += ":";
    if (this.slashes || (!protocol || slashedProtocol[protocol]) && host !== false) {
      if (host = "//" + (host || ""), pathname && pathname.charAt(0) !== "/")
        pathname = "/" + pathname;
    } else if (!host)
      host = "";
    if (hash && hash.charAt(0) !== "#")
      hash = "#" + hash;
    if (search && search.charAt(0) !== "?")
      search = "?" + search;
    return pathname = pathname.replace(/[?#]/g, function(match) {
      return encodeURIComponent(match);
    }), search = search.replace("#", "%23"), protocol + host + pathname + search + hash;
  };
  Url.prototype.resolve = function(relative) {
    return this.resolveObject(urlParse(relative, false, true)).format();
  };
  Url.prototype.resolveObject = function(relative) {
    if (util_isString(relative)) {
      var rel = new Url;
      rel.parse(relative, false, true), relative = rel;
    }
    var result = new Url, tkeys = Object.keys(this);
    for (var tk = 0;tk < tkeys.length; tk++) {
      var tkey = tkeys[tk];
      result[tkey] = this[tkey];
    }
    if (result.hash = relative.hash, relative.href === "")
      return result.href = result.format(), result;
    if (relative.slashes && !relative.protocol) {
      var rkeys = Object.keys(relative);
      for (var rk = 0;rk < rkeys.length; rk++) {
        var rkey = rkeys[rk];
        if (rkey !== "protocol")
          result[rkey] = relative[rkey];
      }
      if (slashedProtocol[result.protocol] && result.hostname && !result.pathname)
        result.path = result.pathname = "/";
      return result.href = result.format(), result;
    }
    if (relative.protocol && relative.protocol !== result.protocol) {
      if (!slashedProtocol[relative.protocol]) {
        var keys = Object.keys(relative);
        for (var v = 0;v < keys.length; v++) {
          var k = keys[v];
          result[k] = relative[k];
        }
        return result.href = result.format(), result;
      }
      if (result.protocol = relative.protocol, !relative.host && !hostlessProtocol[relative.protocol]) {
        var relPath = (relative.pathname || "").split("/");
        while (relPath.length && !(relative.host = relPath.shift()))
          ;
        if (!relative.host)
          relative.host = "";
        if (!relative.hostname)
          relative.hostname = "";
        if (relPath[0] !== "")
          relPath.unshift("");
        if (relPath.length < 2)
          relPath.unshift("");
        result.pathname = relPath.join("/");
      } else
        result.pathname = relative.pathname;
      if (result.search = relative.search, result.query = relative.query, result.host = relative.host || "", result.auth = relative.auth, result.hostname = relative.hostname || relative.host, result.port = relative.port, result.pathname || result.search) {
        var p = result.pathname || "", s = result.search || "";
        result.path = p + s;
      }
      return result.slashes = result.slashes || relative.slashes, result.href = result.format(), result;
    }
    var isSourceAbs = result.pathname && result.pathname.charAt(0) === "/", isRelAbs = relative.host || relative.pathname && relative.pathname.charAt(0) === "/", mustEndAbs = isRelAbs || isSourceAbs || result.host && relative.pathname, removeAllDots = mustEndAbs, srcPath = result.pathname && result.pathname.split("/") || [], relPath = relative.pathname && relative.pathname.split("/") || [], psychotic = result.protocol && !slashedProtocol[result.protocol];
    if (psychotic) {
      if (result.hostname = "", result.port = null, result.host)
        if (srcPath[0] === "")
          srcPath[0] = result.host;
        else
          srcPath.unshift(result.host);
      if (result.host = "", relative.protocol) {
        if (relative.hostname = null, relative.port = null, relative.host)
          if (relPath[0] === "")
            relPath[0] = relative.host;
          else
            relPath.unshift(relative.host);
        relative.host = null;
      }
      mustEndAbs = mustEndAbs && (relPath[0] === "" || srcPath[0] === "");
    }
    if (isRelAbs)
      result.host = relative.host || relative.host === "" ? relative.host : result.host, result.hostname = relative.hostname || relative.hostname === "" ? relative.hostname : result.hostname, result.search = relative.search, result.query = relative.query, srcPath = relPath;
    else if (relPath.length) {
      if (!srcPath)
        srcPath = [];
      srcPath.pop(), srcPath = srcPath.concat(relPath), result.search = relative.search, result.query = relative.query;
    } else if (!util_isNullOrUndefined(relative.search)) {
      if (psychotic) {
        result.hostname = result.host = srcPath.shift();
        var authInHost = result.host && result.host.indexOf("@") > 0 ? result.host.split("@") : false;
        if (authInHost)
          result.auth = authInHost.shift(), result.host = result.hostname = authInHost.shift();
      }
      if (result.search = relative.search, result.query = relative.query, !util_isNull(result.pathname) || !util_isNull(result.search))
        result.path = (result.pathname ? result.pathname : "") + (result.search ? result.search : "");
      return result.href = result.format(), result;
    }
    if (!srcPath.length) {
      if (result.pathname = null, result.search)
        result.path = "/" + result.search;
      else
        result.path = null;
      return result.href = result.format(), result;
    }
    var last = srcPath.slice(-1)[0], hasTrailingSlash = (result.host || relative.host || srcPath.length > 1) && (last === "." || last === "..") || last === "", up = 0;
    for (var i = srcPath.length;i >= 0; i--)
      if (last = srcPath[i], last === ".")
        srcPath.splice(i, 1);
      else if (last === "..")
        srcPath.splice(i, 1), up++;
      else if (up)
        srcPath.splice(i, 1), up--;
    if (!mustEndAbs && !removeAllDots)
      for (;up--; up)
        srcPath.unshift("..");
    if (mustEndAbs && srcPath[0] !== "" && (!srcPath[0] || srcPath[0].charAt(0) !== "/"))
      srcPath.unshift("");
    if (hasTrailingSlash && srcPath.join("/").substr(-1) !== "/")
      srcPath.push("");
    var isAbsolute = srcPath[0] === "" || srcPath[0] && srcPath[0].charAt(0) === "/";
    if (psychotic) {
      result.hostname = result.host = isAbsolute ? "" : srcPath.length ? srcPath.shift() : "";
      var authInHost = result.host && result.host.indexOf("@") > 0 ? result.host.split("@") : false;
      if (authInHost)
        result.auth = authInHost.shift(), result.host = result.hostname = authInHost.shift();
    }
    if (mustEndAbs = mustEndAbs || result.host && srcPath.length, mustEndAbs && !isAbsolute)
      srcPath.unshift("");
    if (!srcPath.length)
      result.pathname = null, result.path = null;
    else
      result.pathname = srcPath.join("/");
    if (!util_isNull(result.pathname) || !util_isNull(result.search))
      result.path = (result.pathname ? result.pathname : "") + (result.search ? result.search : "");
    return result.auth = relative.auth || result.auth, result.slashes = result.slashes || relative.slashes, result.href = result.format(), result;
  };
  Url.prototype.parseHost = function() {
    var host = this.host, port = portPattern.exec(host);
    if (port) {
      if (port = port[0], port !== ":")
        this.port = port.substr(1);
      host = host.substr(0, host.length - port.length);
    }
    if (host)
      this.hostname = host;
  };
  url_default = { parse: urlParse, resolve: urlResolve, resolveObject: urlResolveObject, format: urlFormat, Url, URL: URL2, URLSearchParams: URLSearchParams2 };
});

// node:path
var exports_path = {};
__export(exports_path, {
  sep: () => sep,
  resolve: () => resolve,
  relative: () => relative,
  posix: () => posix,
  parse: () => parse,
  normalize: () => normalize,
  join: () => join,
  isAbsolute: () => isAbsolute,
  format: () => format,
  extname: () => extname,
  dirname: () => dirname,
  delimiter: () => delimiter,
  default: () => path_default,
  basename: () => basename,
  _makeLong: () => _makeLong
});
function assertPath(path) {
  if (typeof path !== "string")
    throw TypeError("Path must be a string. Received " + JSON.stringify(path));
}
function normalizeStringPosix(path, allowAboveRoot) {
  var res = "", lastSegmentLength = 0, lastSlash = -1, dots = 0, code;
  for (var i = 0;i <= path.length; ++i) {
    if (i < path.length)
      code = path.charCodeAt(i);
    else if (code === 47)
      break;
    else
      code = 47;
    if (code === 47) {
      if (lastSlash === i - 1 || dots === 1)
        ;
      else if (lastSlash !== i - 1 && dots === 2) {
        if (res.length < 2 || lastSegmentLength !== 2 || res.charCodeAt(res.length - 1) !== 46 || res.charCodeAt(res.length - 2) !== 46) {
          if (res.length > 2) {
            var lastSlashIndex = res.lastIndexOf("/");
            if (lastSlashIndex !== res.length - 1) {
              if (lastSlashIndex === -1)
                res = "", lastSegmentLength = 0;
              else
                res = res.slice(0, lastSlashIndex), lastSegmentLength = res.length - 1 - res.lastIndexOf("/");
              lastSlash = i, dots = 0;
              continue;
            }
          } else if (res.length === 2 || res.length === 1) {
            res = "", lastSegmentLength = 0, lastSlash = i, dots = 0;
            continue;
          }
        }
        if (allowAboveRoot) {
          if (res.length > 0)
            res += "/..";
          else
            res = "..";
          lastSegmentLength = 2;
        }
      } else {
        if (res.length > 0)
          res += "/" + path.slice(lastSlash + 1, i);
        else
          res = path.slice(lastSlash + 1, i);
        lastSegmentLength = i - lastSlash - 1;
      }
      lastSlash = i, dots = 0;
    } else if (code === 46 && dots !== -1)
      ++dots;
    else
      dots = -1;
  }
  return res;
}
function _format(sep, pathObject) {
  var dir = pathObject.dir || pathObject.root, base = pathObject.base || (pathObject.name || "") + (pathObject.ext || "");
  if (!dir)
    return base;
  if (dir === pathObject.root)
    return dir + base;
  return dir + sep + base;
}
function resolve() {
  var resolvedPath = "", resolvedAbsolute = false, cwd;
  for (var i = arguments.length - 1;i >= -1 && !resolvedAbsolute; i--) {
    var path;
    if (i >= 0)
      path = arguments[i];
    else {
      if (cwd === undefined)
        cwd = process.cwd();
      path = cwd;
    }
    if (assertPath(path), path.length === 0)
      continue;
    resolvedPath = path + "/" + resolvedPath, resolvedAbsolute = path.charCodeAt(0) === 47;
  }
  if (resolvedPath = normalizeStringPosix(resolvedPath, !resolvedAbsolute), resolvedAbsolute)
    if (resolvedPath.length > 0)
      return "/" + resolvedPath;
    else
      return "/";
  else if (resolvedPath.length > 0)
    return resolvedPath;
  else
    return ".";
}
function normalize(path) {
  if (assertPath(path), path.length === 0)
    return ".";
  var isAbsolute = path.charCodeAt(0) === 47, trailingSeparator = path.charCodeAt(path.length - 1) === 47;
  if (path = normalizeStringPosix(path, !isAbsolute), path.length === 0 && !isAbsolute)
    path = ".";
  if (path.length > 0 && trailingSeparator)
    path += "/";
  if (isAbsolute)
    return "/" + path;
  return path;
}
function isAbsolute(path) {
  return assertPath(path), path.length > 0 && path.charCodeAt(0) === 47;
}
function join() {
  if (arguments.length === 0)
    return ".";
  var joined;
  for (var i = 0;i < arguments.length; ++i) {
    var arg = arguments[i];
    if (assertPath(arg), arg.length > 0)
      if (joined === undefined)
        joined = arg;
      else
        joined += "/" + arg;
  }
  if (joined === undefined)
    return ".";
  return normalize(joined);
}
function relative(from, to) {
  if (assertPath(from), assertPath(to), from === to)
    return "";
  if (from = resolve(from), to = resolve(to), from === to)
    return "";
  var fromStart = 1;
  for (;fromStart < from.length; ++fromStart)
    if (from.charCodeAt(fromStart) !== 47)
      break;
  var fromEnd = from.length, fromLen = fromEnd - fromStart, toStart = 1;
  for (;toStart < to.length; ++toStart)
    if (to.charCodeAt(toStart) !== 47)
      break;
  var toEnd = to.length, toLen = toEnd - toStart, length = fromLen < toLen ? fromLen : toLen, lastCommonSep = -1, i = 0;
  for (;i <= length; ++i) {
    if (i === length) {
      if (toLen > length) {
        if (to.charCodeAt(toStart + i) === 47)
          return to.slice(toStart + i + 1);
        else if (i === 0)
          return to.slice(toStart + i);
      } else if (fromLen > length) {
        if (from.charCodeAt(fromStart + i) === 47)
          lastCommonSep = i;
        else if (i === 0)
          lastCommonSep = 0;
      }
      break;
    }
    var fromCode = from.charCodeAt(fromStart + i), toCode = to.charCodeAt(toStart + i);
    if (fromCode !== toCode)
      break;
    else if (fromCode === 47)
      lastCommonSep = i;
  }
  var out = "";
  for (i = fromStart + lastCommonSep + 1;i <= fromEnd; ++i)
    if (i === fromEnd || from.charCodeAt(i) === 47)
      if (out.length === 0)
        out += "..";
      else
        out += "/..";
  if (out.length > 0)
    return out + to.slice(toStart + lastCommonSep);
  else {
    if (toStart += lastCommonSep, to.charCodeAt(toStart) === 47)
      ++toStart;
    return to.slice(toStart);
  }
}
function _makeLong(path) {
  return path;
}
function dirname(path) {
  if (assertPath(path), path.length === 0)
    return ".";
  var code = path.charCodeAt(0), hasRoot = code === 47, end = -1, matchedSlash = true;
  for (var i = path.length - 1;i >= 1; --i)
    if (code = path.charCodeAt(i), code === 47) {
      if (!matchedSlash) {
        end = i;
        break;
      }
    } else
      matchedSlash = false;
  if (end === -1)
    return hasRoot ? "/" : ".";
  if (hasRoot && end === 1)
    return "//";
  return path.slice(0, end);
}
function basename(path, ext) {
  if (ext !== undefined && typeof ext !== "string")
    throw TypeError('"ext" argument must be a string');
  assertPath(path);
  var start2 = 0, end = -1, matchedSlash = true, i;
  if (ext !== undefined && ext.length > 0 && ext.length <= path.length) {
    if (ext.length === path.length && ext === path)
      return "";
    var extIdx = ext.length - 1, firstNonSlashEnd = -1;
    for (i = path.length - 1;i >= 0; --i) {
      var code = path.charCodeAt(i);
      if (code === 47) {
        if (!matchedSlash) {
          start2 = i + 1;
          break;
        }
      } else {
        if (firstNonSlashEnd === -1)
          matchedSlash = false, firstNonSlashEnd = i + 1;
        if (extIdx >= 0)
          if (code === ext.charCodeAt(extIdx)) {
            if (--extIdx === -1)
              end = i;
          } else
            extIdx = -1, end = firstNonSlashEnd;
      }
    }
    if (start2 === end)
      end = firstNonSlashEnd;
    else if (end === -1)
      end = path.length;
    return path.slice(start2, end);
  } else {
    for (i = path.length - 1;i >= 0; --i)
      if (path.charCodeAt(i) === 47) {
        if (!matchedSlash) {
          start2 = i + 1;
          break;
        }
      } else if (end === -1)
        matchedSlash = false, end = i + 1;
    if (end === -1)
      return "";
    return path.slice(start2, end);
  }
}
function extname(path) {
  assertPath(path);
  var startDot = -1, startPart = 0, end = -1, matchedSlash = true, preDotState = 0;
  for (var i = path.length - 1;i >= 0; --i) {
    var code = path.charCodeAt(i);
    if (code === 47) {
      if (!matchedSlash) {
        startPart = i + 1;
        break;
      }
      continue;
    }
    if (end === -1)
      matchedSlash = false, end = i + 1;
    if (code === 46) {
      if (startDot === -1)
        startDot = i;
      else if (preDotState !== 1)
        preDotState = 1;
    } else if (startDot !== -1)
      preDotState = -1;
  }
  if (startDot === -1 || end === -1 || preDotState === 0 || preDotState === 1 && startDot === end - 1 && startDot === startPart + 1)
    return "";
  return path.slice(startDot, end);
}
function format(pathObject) {
  if (pathObject === null || typeof pathObject !== "object")
    throw TypeError('The "pathObject" argument must be of type Object. Received type ' + typeof pathObject);
  return _format("/", pathObject);
}
function parse(path) {
  assertPath(path);
  var ret = { root: "", dir: "", base: "", ext: "", name: "" };
  if (path.length === 0)
    return ret;
  var code = path.charCodeAt(0), isAbsolute2 = code === 47, start2;
  if (isAbsolute2)
    ret.root = "/", start2 = 1;
  else
    start2 = 0;
  var startDot = -1, startPart = 0, end = -1, matchedSlash = true, i = path.length - 1, preDotState = 0;
  for (;i >= start2; --i) {
    if (code = path.charCodeAt(i), code === 47) {
      if (!matchedSlash) {
        startPart = i + 1;
        break;
      }
      continue;
    }
    if (end === -1)
      matchedSlash = false, end = i + 1;
    if (code === 46) {
      if (startDot === -1)
        startDot = i;
      else if (preDotState !== 1)
        preDotState = 1;
    } else if (startDot !== -1)
      preDotState = -1;
  }
  if (startDot === -1 || end === -1 || preDotState === 0 || preDotState === 1 && startDot === end - 1 && startDot === startPart + 1) {
    if (end !== -1)
      if (startPart === 0 && isAbsolute2)
        ret.base = ret.name = path.slice(1, end);
      else
        ret.base = ret.name = path.slice(startPart, end);
  } else {
    if (startPart === 0 && isAbsolute2)
      ret.name = path.slice(1, startDot), ret.base = path.slice(1, end);
    else
      ret.name = path.slice(startPart, startDot), ret.base = path.slice(startPart, end);
    ret.ext = path.slice(startDot, end);
  }
  if (startPart > 0)
    ret.dir = path.slice(0, startPart - 1);
  else if (isAbsolute2)
    ret.dir = "/";
  return ret;
}
var sep = "/", delimiter = ":", posix, path_default;
var init_path = __esm(() => {
  posix = ((p) => (p.posix = p, p))({ resolve, normalize, isAbsolute, join, relative, _makeLong, dirname, basename, extname, format, parse, sep, delimiter, win32: null, posix: null });
  path_default = posix;
});

/* 🟦️typescript/🧵️frame-worker.ts */
init__glue();

/* ../../../../../../🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts */
var PLUGIN_HOST_CONFIGS = [
  { pluginId: "s", landingAppId: "home", hostAppId: "studio" }
];
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "animate", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF9E️animate/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_animate.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:animate.present"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590", coreWasmSha256: "5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590", descriptorSha256: "12a912e82f98d54f405262123150f41035a15234332a1abc971062ac7e973b17" } },
  { pluginId: "architect", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFDB️architect/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_architect.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:data.\uD83C\uDFDB️program"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "2301bc724c96c3f6ea698bc1eba4feb50a0b0b4d1dfdbffa94a912c7e9dab510", coreWasmSha256: "2301bc724c96c3f6ea698bc1eba4feb50a0b0b4d1dfdbffa94a912c7e9dab510", descriptorSha256: "09d0f7320243a4aa38d5c83fa7d0a75ed398756edcb093c848adf515d1c1c4d8" } },
  { pluginId: "block", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDF1️block/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_block.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "cad", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_cad.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.cad"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "d3501088cd8ec6762011a2f7523b313c45c00ad4867041a78443c38572b092e6", coreWasmSha256: "d3501088cd8ec6762011a2f7523b313c45c00ad4867041a78443c38572b092e6", descriptorSha256: "63b5a4ca0a07a7f4f6c98ec19c0c20046a46edbbf51ff0e90072c952e8b147e4" } },
  { pluginId: "dag", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD78️dag/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_dag.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:graph.dag"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "55c9da9026706dbcd47277335eda53abf66e3ecf19fd848280a95b7a531f51e2", coreWasmSha256: "55c9da9026706dbcd47277335eda53abf66e3ecf19fd848280a95b7a531f51e2", descriptorSha256: "53d81f2b0927fbc1383cccb1c989a5fe190fd98ea582786bd6ea1846aea5258d" } },
  { pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_demonstrator.wasm", role: "plugin", capabilities: [], contributes: [], consumes: ["forms.questionKind", "flow.extension", "process.machines"], dependsOn: ["cad", "gis", "procedural", "process", "puzzle", "sourcing", "stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "draw", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD8D️draw/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_draw.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["draw-fsm", "stdio"], activationEvents: ["on-artifact-kind:2d.drawing"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "4bccf647dd64b0d6088e7338a25e7ed1326a412f44660459f0d6c9cab0e79714", coreWasmSha256: "4bccf647dd64b0d6088e7338a25e7ed1326a412f44660459f0d6c9cab0e79714", descriptorSha256: "b9d12f23271b085b41da39d7ba395ea78604cab8006b6b00e1ee39aa5265a1bd" } },
  { pluginId: "energy", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD0B️energy/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_energy.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:data.\uD83D\uDD0B️model"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "1c0f620a5d442096c9683acf7095f470375c8b7efa0821076d8e548b8d706f20", coreWasmSha256: "1c0f620a5d442096c9683acf7095f470375c8b7efa0821076d8e548b8d706f20", descriptorSha256: "383853b475b0308336f8088fe067d27fa2f525b21349d70b080b07aa86ae2ec1" } },
  { pluginId: "fem", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFD7️fem/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_fem.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.fem2d", "on-artifact-kind:computation.fem3d"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13", coreWasmSha256: "924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13", descriptorSha256: "f0c10888f9dc7101c596b0e8b837fcbd439cb031738dd233e767cc8ad59f6fdb" } },
  { pluginId: "flow", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["flow.extension"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.flow"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "b996f5722473bb19e91f3ab4b38cd67bd95cf1852586684e836a260a642eaed2", coreWasmSha256: "b996f5722473bb19e91f3ab4b38cd67bd95cf1852586684e836a260a642eaed2", descriptorSha256: "1996bf86c181d869f1d9839d3b4763146ce18c99da5a3c0cc67470398c10f2d4" } },
  { pluginId: "forms", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCCB️forms/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_forms.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["forms.questionKind"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:form.dictionary"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "a63d0dfc2619a9e7f05ae83c119717989ff8a32667f4771838c5c5599014b152", coreWasmSha256: "a63d0dfc2619a9e7f05ae83c119717989ff8a32667f4771838c5c5599014b152", descriptorSha256: "8e0b3d00eb48790dd1f31070462adaf925fc00cdc8a664c1865366b6589c0d88" } },
  { pluginId: "gis", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0D️gis/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_gis.wasm", role: "plugin", capabilities: ["documents.write", "shell.navigate"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.map"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "3933bfbe8d1b987e336d4331eabd9810439cb2044ca002b0001a62354a05fe63", coreWasmSha256: "3933bfbe8d1b987e336d4331eabd9810439cb2044ca002b0001a62354a05fe63", descriptorSha256: "935d4998777641d0e7df7d8111e1cf6583339aed8e71b09c88efba0dda5a0750" } },
  { pluginId: "imperative", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.imperative"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "32cdff3f114c8390f85c3f7ed928525d25ed52be15b147cbfa58ec64a0e4234f", coreWasmSha256: "32cdff3f114c8390f85c3f7ed928525d25ed52be15b147cbfa58ec64a0e4234f", descriptorSha256: "7dc6bc0885b16f4a552ecdf5e1757da8d336efebcb81b603c87341ae25a66506" } },
  { pluginId: "layout", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCCF️layout/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_layout.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.layout"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "dfde964f079e83c8f8cc67873cd495448be7a06ac8f6776e8585aef4b4f5b0bc", coreWasmSha256: "dfde964f079e83c8f8cc67873cd495448be7a06ac8f6776e8585aef4b4f5b0bc", descriptorSha256: "66358711ac5cd24af7edebf20ba9e40c3a7d96bb9e28ba19bc9d548b62c026db" } },
  { pluginId: "lowpoly", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCA0️lowpoly/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_lowpoly.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["cad", "stdio"], activationEvents: ["on-artifact-kind:3d.lowpoly"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "95f9ac4920995ae69e8807c90be68082694a15b2466910d3cf257476a8940c02", coreWasmSha256: "95f9ac4920995ae69e8807c90be68082694a15b2466910d3cf257476a8940c02", descriptorSha256: "2e2e5e1e43988b270aa356d10fca3608c594faa7b7f6a47b9c1efa93fbb45751" } },
  { pluginId: "mathematical", cratePath: "✏️s/\uD83D\uDD0C️plugins/➗️mathematical/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_mathematical.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.mathematical"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "0b801ea2f23f760c1b8b2b24a7f137af965cc5825da11065cac51cd179b14716", coreWasmSha256: "0b801ea2f23f760c1b8b2b24a7f137af965cc5825da11065cac51cd179b14716", descriptorSha256: "824b2c80a380ac3cebb2c39ec5ff9b95282fb98e6888f6c91293f85e0263b227" } },
  { pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_norm.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["fem", "stdio"], activationEvents: ["on-artifact-kind:computation.norm.din4108", "on-artifact-kind:computation.norm.din16798", "on-artifact-kind:computation.norm.din18599", "on-artifact-kind:computation.norm.en1990", "on-artifact-kind:computation.norm.en1991", "on-artifact-kind:computation.norm.en1992", "on-artifact-kind:computation.norm.en1993", "on-artifact-kind:computation.norm.en1994", "on-artifact-kind:computation.norm.en1995", "on-artifact-kind:computation.norm.en1996", "on-artifact-kind:computation.norm.en1997", "on-artifact-kind:computation.norm.en1998", "on-artifact-kind:computation.norm.en1999", "on-artifact-kind:computation.norm.iso16757", "on-artifact-kind:computation.norm.vdi3805"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "ee09ede9e0a96f42d31342b2e646edfb17b05f3d63b47315148774eb9f99dbfc", coreWasmSha256: "ee09ede9e0a96f42d31342b2e646edfb17b05f3d63b47315148774eb9f99dbfc", descriptorSha256: "dbca604de90af12da82cb423792a4ced55422e75c1f1baee863caf898f0295c3" } },
  { pluginId: "note", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDDD2️note/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_note.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.note"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7", coreWasmSha256: "a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7", descriptorSha256: "1b8c29c800f1fd38f95f6754ec982585b59595a60ddf06fdbbadb6738850a093" } },
  { pluginId: "playbook", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD6️playbook/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_playbook.wasm", role: "plugin", capabilities: [], contributes: [], consumes: ["playbook.blockKind"], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "procedural", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF00️procedural/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_procedural.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["forms.questionKind", "flow.extension"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.procedural", "on-artifact-kind:3d.procedural"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "be2cee9b5207741615ed09eddf257acde85b0acdf58535f56c0d03f8cf91915f", coreWasmSha256: "be2cee9b5207741615ed09eddf257acde85b0acdf58535f56c0d03f8cf91915f", descriptorSha256: "cc84360e8f5e007ca916ea785556baee6f193b4bd78d30d6a359da4fe84d939e" } },
  { pluginId: "process", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_process.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["process.machines"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.process"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "dcb1cf7e073ec7c61fa31d6fe176d1e11424ccb452cc1387a14a442c81efe746", coreWasmSha256: "dcb1cf7e073ec7c61fa31d6fe176d1e11424ccb452cc1387a14a442c81efe746", descriptorSha256: "ba72fb1fdc5e1aa1e47833146de43ea59601e494c812ac4ca45e5e3c746b2628" } },
  { pluginId: "puzzle", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDE9️puzzle/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_puzzle.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "raster", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDDA8️raster/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_raster.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.raster"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6", coreWasmSha256: "9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6", descriptorSha256: "26760a5a3c146b1612a8e8036c877f91a17c13cef425b94a174127df3e33bd94" } },
  { pluginId: "reasoning-mindmap", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCA1️reasoning/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_reasoning_mindmap.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:graph.wires"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "7686a3193c6aeffe74e8e73d76b842112e892e57f9f3aa9ed04d39bc8bc1c2b8", coreWasmSha256: "7686a3193c6aeffe74e8e73d76b842112e892e57f9f3aa9ed04d39bc8bc1c2b8", descriptorSha256: "eb21b2587a19242762803823f748628b1eb1553c783f6281dfee25ac72706f93" } },
  { pluginId: "remodel", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCF8️remodel/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_remodel.wasm", role: "plugin", capabilities: ["documents.write", "ui.dialog"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.remodel"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "77ef3c98d134f1164cdd388911333b0618bcec94fead7c11ad6fdd24abb125b5", coreWasmSha256: "77ef3c98d134f1164cdd388911333b0618bcec94fead7c11ad6fdd24abb125b5", descriptorSha256: "1e1dded5a4979ce72c0ff11f4e12e8336df93784c89c0f53b0ee573b694fbe62" } },
  { pluginId: "s", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDE90️space/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_space.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:space.shome", "on-artifact-kind:space.sspace"], extensionPoints: [], host: { landingAppId: "home", hostAppId: "studio" }, executionMode: "isolated", hashes: { wasmSha256: "762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af", coreWasmSha256: "762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af", descriptorSha256: "df021b9a83bcb48ab858afe4a8f2c2e30d69f8166850ddebb064421109b3fed6" } },
  { pluginId: "sequence", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAC️sequence/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_sequence.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["imperative-control", "imperative-effect", "imperative-math", "imperative-text", "stdio"], activationEvents: ["on-artifact-kind:computation.sequence"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "bbcf24176893beb37e0dcdf36f658f52a62b8a5e48163130cd5f02371b2a6a79", coreWasmSha256: "bbcf24176893beb37e0dcdf36f658f52a62b8a5e48163130cd5f02371b2a6a79", descriptorSha256: "5c5ee126f62f14b60a81d95575c85186db47ec9b7712d0e56d5ba6b2a032088a" } },
  { pluginId: "shooting", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFA5️shooting/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_shooting.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.shooting"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "2e16eed70a875e078501c439d8f05c162163f1193bcaee4f11b41f0b2f2eed01", coreWasmSha256: "2e16eed70a875e078501c439d8f05c162163f1193bcaee4f11b41f0b2f2eed01", descriptorSha256: "ad86c4d9cf0730ae4b512389898962bb9eefd1f631f8543d7fd8143be3276129" } },
  { pluginId: "sourcing", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDEB5️sourcing/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_sourcing.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:catalogue.sourcing"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "a3bf35a836d57f1277b74e4f2c4b2172e2f1c33fb3fc1ce841b1c957b8ed9531", coreWasmSha256: "a3bf35a836d57f1277b74e4f2c4b2172e2f1c33fb3fc1ce841b1c957b8ed9531", descriptorSha256: "6fb94b60b530d27f2a9f2ab4ee507c9736c277396697f5d0998d91ffff6fc252" } },
  { pluginId: "stdio", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDDC4️stdio/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_stdio.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: [], activationEvents: [], extensionPoints: [] },
  { pluginId: "trinity", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD31️trinity/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_trinity.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "vcs", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF3F️vcs/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_vcs.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:vcs.document"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "74771b987f39e483da63efdb21006a3ce511ad5edd1c3bd0de05543bef00d925", coreWasmSha256: "74771b987f39e483da63efdb21006a3ce511ad5edd1c3bd0de05543bef00d925", descriptorSha256: "b702fe11bb1c92bb06226ccce58792ccd37fa01be8313a740e52ea6a48e8329e" } },
  { pluginId: "writer", cratePath: "✏️s/\uD83D\uDD0C️plugins/✒️writer/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_writer.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio", "trinity"], activationEvents: ["on-artifact-kind:text.document"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "6507f654884a1c93e633bfc4cd42b5cebb880925ca4e03ca278bc9ccf191c18e", coreWasmSha256: "6507f654884a1c93e633bfc4cd42b5cebb880925ca4e03ca278bc9ccf191c18e", descriptorSha256: "ced53b5c3f821e2cb2fc847868737e6c695e8e32aa9c2358886df559214e750d" } }
];
var EXTENSION_TARGETS = [
  { pluginId: "cad-extension-aec-building", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83E\uDDE9️extensions/\uD83C\uDFE2️aec-building/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_cad_aec_building.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "af59b52fd8c7f60d5eb1195406a65d4eaf2de59b471fe54ddddd9dd1ec7d70c0", coreWasmSha256: "af59b52fd8c7f60d5eb1195406a65d4eaf2de59b471fe54ddddd9dd1ec7d70c0", descriptorSha256: "4f06e341b211c507f489e3929838512d79015d79a3b8fd97f3c4ef1f3a2ee43e" } },
  { pluginId: "cad-extension-aec-building-energy", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83E\uDDE9️extensions/\uD83D\uDD25️aec-building-energy/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_cad_aec_building_energy.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "e5b2ff618804be66178d53f4f5302d9e08974e7f982a94163e812ddd7c315722", coreWasmSha256: "e5b2ff618804be66178d53f4f5302d9e08974e7f982a94163e812ddd7c315722", descriptorSha256: "c49d812a1ef6056b2a7a38b886a91d5b2ceb3bce0f498d4e2f7fd5e709b8489f" } },
  { pluginId: "cad-extension-aec-building-structure", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83E\uDDE9️extensions/\uD83C\uDFDB️aec-building-structure/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_cad_aec_building_structure.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "ec7281c5e733b921760a7a365660b0d105442b20c9064168b28168f92bc97fc9", coreWasmSha256: "ec7281c5e733b921760a7a365660b0d105442b20c9064168b28168f92bc97fc9", descriptorSha256: "71cd360006a6b82ab5e42bbce01580600441234af040753100200e3f2f736dc4" } },
  { pluginId: "cad-extension-spatial-shape", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83E\uDDE9️extensions/\uD83D\uDCD0️spatial-shape/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_cad_spatial_shape.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "d77ec8ebc85fd286e5cdb3f24d037137461e9293524cbf94d7809a7d58fd98ab", coreWasmSha256: "d77ec8ebc85fd286e5cdb3f24d037137461e9293524cbf94d7809a7d58fd98ab", descriptorSha256: "7919165697487d2cdee0c6e4162a25dc8f6e57d0fa751f6239ed5b6b872de5a5" } },
  { pluginId: "flow-extension-bim", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83C\uDFD7️bim/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_bim.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-brep", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDCD0️brep/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_brep.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow", "stdio"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-dictionary", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDCD6️dictionary/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_dictionary.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-draw", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDD8D️draw/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_draw.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-list", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDCC3️list/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_list.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-logic", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83E\uDDE0️logic/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_logic.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-math", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83E\uDDEE️math/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_math.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-primitive", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDD24️primitive/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_primitive.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-text", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83E\uDDE9️extensions/\uD83D\uDCDD️text/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_flow_extension_text.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "imperative-extension-control", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83E\uDDE9️extensions/\uD83C\uDFAE️control/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative_control.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-effect", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83E\uDDE9️extensions/\uD83D\uDCE3️effect/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative_effect.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-logic", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83E\uDDE9️extensions/\uD83E\uDDE0️logic/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative_logic.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-math", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83E\uDDE9️extensions/\uD83E\uDDEE️math/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative_math.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-text", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83E\uDDE9️extensions/\uD83D\uDCDD️text/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_imperative_text.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "playbook-module-procedural", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD6️playbook/\uD83E\uDDE9️extensions/\uD83C\uDF00️procedural/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_playbook_procedural.wasm", role: "extension", capabilities: ["playbook.blockKind"], contributes: ["playbook.blockKind"], consumes: [], dependsOn: ["playbook"], activationEvents: [], extensionPoints: [], extends: "playbook" },
  { pluginId: "process-extension-concrete", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83E\uDDE9️extensions/\uD83E\uDDF1️concrete/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_process_concrete.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-metal", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83E\uDDE9️extensions/\uD83D\uDD29️metal/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_process_metal.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-robotic", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83E\uDDE9️extensions/\uD83E\uDD16️robotic/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_process_robotic.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-wood", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83E\uDDE9️extensions/\uD83E\uDEB5️wood/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_process_wood.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "sourcing-module-beams", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDEB5️sourcing/\uD83E\uDDE9️extensions/\uD83E\uDEB5️beams/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_sourcing_beams.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" },
  { pluginId: "sourcing-module-slabs", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDEB5️sourcing/\uD83E\uDDE9️extensions/\uD83E\uDDF1️slabs/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_sourcing_slabs.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" },
  { pluginId: "sourcing-module-windows", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDEB5️sourcing/\uD83E\uDDE9️extensions/\uD83E\uDE9F️windows/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", wasmOut: "semio_s_plugin_sourcing_windows.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" }
];
var PROGRAM_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: `/plugin-modules/${target.pluginId}/${target.wasmOut.replace(/\.wasm$/, ".js")}`
}));
var pluginModuleUrl = (pluginId, fileName) => `/plugin-modules/${pluginId}/${fileName.replace(/\.wasm$/, ".js")}`;
var extensionModuleUrl = (extensionId, fileName) => `/extensions/${extensionId}/${fileName.replace(/\.wasm$/, ".js")}`;

/* ../../../../../../🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts */
var PLAYGROUND_BUILD_TARGETS = [
  { variant: "aggregator", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.puzzle3d@1/*#editor", brand: "entwerfen-mit-bestand-aggregator", aliases: ["mit-bestand", "entwerfen-mit-bestand"], ports: { react: 6023, wgpu: 6123 }, examples: [], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83C\uDF31️metabolism/\uD83C\uDFA8️representation", "♻️mit-bestand/\uD83D\uDDBC️asset/\uD83C\uDFDA️abbau-aufbau"], placeholder: "\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83E\uDD7D️mesh/\uD83E\uDDCA️placeholder.glb", filterFromExamples: true }, { kind: "static-dir", route: "/infinite-fixture", root: "\uD83E\uDDF0️framework/\uD83D\uDECD️products/\uD83D\uDCBB️os/\uD83D\uDD28️modules/♾️infinite/\uD83E\uDDEB️fixtures" }] },
  { variant: "animate", pluginId: "animate", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF9E️animate/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6051, wgpu: 6151 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "architect", pluginId: "architect", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFDB️architect/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6090, wgpu: 6190 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "aussuchen", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "sourcing-curate", brand: "entwerfen-mit-bestand-aussuchen", aliases: ["entwerfen-mit-bestand-aussuchen"], ports: { react: 6030, wgpu: 6130 }, examples: [], engines: [], assets: [] },
  { variant: "bearbeiten", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "process3d-play", brand: "entwerfen-mit-bestand-bearbeiten", aliases: ["entwerfen-mit-bestand-bearbeiten"], ports: { react: 6031, wgpu: 6131 }, examples: [], engines: [], assets: [] },
  { variant: "block2d", pluginId: "block", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDF1️block/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.block.block2d@1/*#editor", aliases: ["block 2d"], ports: { react: 6024, wgpu: 6124 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "block3d", pluginId: "block", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDF1️block/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.block.block3d@1/*#editor", aliases: ["block 3d"], ports: { react: 6025, wgpu: 6125 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83C\uDF31️metabolism/\uD83C\uDFA8️representation", "♻️mit-bestand/\uD83D\uDDBC️asset/\uD83C\uDFDA️abbau-aufbau"], placeholder: "\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83E\uDD7D️mesh/\uD83E\uDDCA️placeholder.glb", filterFromExamples: true }] },
  { variant: "block5d", pluginId: "block", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDF1️block/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.block.block5d@1/*#editor", aliases: ["block 5d"], ports: { react: 6026, wgpu: 6126 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "cad", pluginId: "cad", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.cad.cad@1/*#editor", aliases: [], ports: { react: 6020, wgpu: 6120 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [{ kind: "static-dir", route: "/cad-fixture", root: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83D\uDDFF️artifacts/\uD83D\uDCD0️cad/\uD83C\uDFC5️standards/\uD83D\uDD16️1/\uD83E\uDE86️subsets/✳️any/\uD83D\uDCDA️examples/\uD83E\uDDEB️fixtures" }] },
  { variant: "dag", pluginId: "dag", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD78️dag/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6017, wgpu: 6117 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "din16798", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.din16798@1/*#editor", aliases: [], ports: { react: 6092, wgpu: 6192 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "din18599", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.din18599@1/*#editor", aliases: [], ports: { react: 6093, wgpu: 6193 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "din4108", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.din4108@1/*#editor", aliases: [], ports: { react: 6091, wgpu: 6191 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "draw", pluginId: "draw", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD8D️draw/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6064, wgpu: 6164 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1990", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1990@1/*#editor", aliases: [], ports: { react: 6094, wgpu: 6194 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1991", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1991@1/*#editor", aliases: [], ports: { react: 6095, wgpu: 6195 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1992", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1992@1/*#editor", aliases: [], ports: { react: 6096, wgpu: 6196 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1993", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1993@1/*#editor", aliases: [], ports: { react: 6097, wgpu: 6197 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1994", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1994@1/*#editor", aliases: [], ports: { react: 6098, wgpu: 6198 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1995", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1995@1/*#editor", aliases: [], ports: { react: 6099, wgpu: 6199 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1996", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1996@1/*#editor", aliases: [], ports: { react: 6100, wgpu: 6200 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1997", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1997@1/*#editor", aliases: [], ports: { react: 6101, wgpu: 6201 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1998", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1998@1/*#editor", aliases: [], ports: { react: 6102, wgpu: 6202 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "en1999", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.en1999@1/*#editor", aliases: [], ports: { react: 6103, wgpu: 6203 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "fem2d", pluginId: "fem", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFD7️fem/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.fem.fem2d@1/*#editor", aliases: ["fem 2d"], ports: { react: 6086, wgpu: 6186 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "fem3d", pluginId: "fem", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFD7️fem/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.fem.fem3d@1/*#editor", aliases: ["fem 3d"], ports: { react: 6087, wgpu: 6187 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "flow", pluginId: "flow", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0A️flow/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6016, wgpu: 6116 }, examples: ["\uD83C\uDFAC️demo-session"], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDECD️products/\uD83D\uDCBB️os/\uD83D\uDD28️modules/\uD83C\uDF0A️flow/\uD83E\uDEC0️core/pkg"], assets: [] },
  { variant: "forms", pluginId: "forms", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCCB️forms/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6058, wgpu: 6158 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "generator", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.procedural.procedural3d@1/*#editor", brand: "entwerfen-mit-bestand-generator", aliases: ["entwerfen-mit-bestand-generator"], ports: { react: 6027, wgpu: 6127 }, examples: [], engines: [], assets: [] },
  { variant: "gis2d", pluginId: "gis", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0D️gis/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.gis.gismap@1/*#editor", aliases: ["gis 2d"], ports: { react: 6040, wgpu: 6140 }, examples: ["\uD83C\uDFAC️demo-session"], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDFA️surface/\uD83D\uDCE6️packages/\uD83E\uDD80️rust"], assets: [{ kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" }, { kind: "tile-proxy", route: "/vt", upstream: "https://tiles.openfreemap.org/planet", cache: "openfreemap-vt" }] },
  { variant: "gis3d", pluginId: "gis", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF0D️gis/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.gis.gisterrain@1/*#editor", aliases: ["gis 3d"], ports: { react: 6083, wgpu: 6183 }, examples: ["\uD83C\uDFAC️demo-session"], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDFA️surface/\uD83D\uDCE6️packages/\uD83E\uDD80️rust"], assets: [{ kind: "tile-proxy", route: "/dem", upstream: "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png", cache: "terrarium-dem" }] },
  { variant: "imperative", pluginId: "imperative", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCDC️imperative/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6076, wgpu: 6176 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "iso16757", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.iso16757@1/*#editor", aliases: [], ports: { react: 6104, wgpu: 6204 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "koordinator", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.cad.cad@1/*#editor", brand: "entwerfen-mit-bestand-koordinator", aliases: ["entwerfen-mit-bestand-koordinator"], ports: { react: 6028, wgpu: 6128 }, examples: [], engines: [], assets: [{ kind: "static-dir", route: "/cad-fixture", root: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD0️cad/\uD83D\uDDFF️artifacts/\uD83D\uDCD0️cad/\uD83C\uDFC5️standards/\uD83D\uDD16️1/\uD83E\uDE86️subsets/✳️any/\uD83D\uDCDA️examples/\uD83E\uDDEB️fixtures" }] },
  { variant: "layout", pluginId: "layout", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCCF️layout/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6079, wgpu: 6179 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "lowpoly", pluginId: "lowpoly", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCA0️lowpoly/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6078, wgpu: 6178 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "mathematical", pluginId: "mathematical", cratePath: "✏️s/\uD83D\uDD0C️plugins/➗️mathematical/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "mathematical-play", aliases: ["mathematical", "math"], ports: { react: 6084, wgpu: 6184 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "note", pluginId: "note", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDDD2️note/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6080, wgpu: 6180 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "playbook", pluginId: "playbook", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD6️playbook/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6085, wgpu: 6185 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "procedural2d", pluginId: "procedural", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF00️procedural/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "procedural2d-play", aliases: ["procedural 2d"], ports: { react: 6021, wgpu: 6121 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "procedural3d", pluginId: "procedural", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF00️procedural/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "procedural3d-play", aliases: ["procedural 3d"], ports: { react: 6018, wgpu: 6118 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "process3d", pluginId: "process", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFED️process/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "process3d-play", aliases: ["process 3d"], ports: { react: 6022, wgpu: 6122 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "puzzle2d", pluginId: "puzzle", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDE9️puzzle/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.puzzle2d@1/*#editor", aliases: ["2d", "puzzle 2d"], ports: { react: 6012, wgpu: 6112 }, examples: ["\uD83C\uDFAC️demo-session"], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDFA️surface/\uD83D\uDCE6️packages/\uD83E\uDD80️rust"], assets: [] },
  { variant: "puzzle3d", pluginId: "puzzle", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDE9️puzzle/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.puzzle3d@1/*#editor", aliases: ["3d", "puzzle 3d"], ports: { react: 6013, wgpu: 6113 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83C\uDF31️metabolism/\uD83C\uDFA8️representation", "♻️mit-bestand/\uD83D\uDDBC️asset/\uD83C\uDFDA️abbau-aufbau"], placeholder: "\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83E\uDD7D️mesh/\uD83E\uDDCA️placeholder.glb", filterFromExamples: true }, { kind: "static-dir", route: "/infinite-fixture", root: "\uD83E\uDDF0️framework/\uD83D\uDECD️products/\uD83D\uDCBB️os/\uD83D\uDD28️modules/♾️infinite/\uD83E\uDDEB️fixtures" }] },
  { variant: "puzzle5d", pluginId: "puzzle", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDDE9️puzzle/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.puzzle5d@1/*#editor", aliases: ["5d", "puzzle 5d"], ports: { react: 6014, wgpu: 6114 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "raster", pluginId: "raster", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDDA8️raster/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6060, wgpu: 6160 }, examples: ["\uD83C\uDFAC️demo-session"], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDFA️surface/\uD83D\uDCE6️packages/\uD83E\uDD80️rust"], assets: [] },
  { variant: "reasoning-wires", pluginId: "reasoning-mindmap", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCA1️reasoning/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: ["wires"], ports: { react: 6015, wgpu: 6115 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "remodel", pluginId: "remodel", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCF8️remodel/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6063, wgpu: 6163 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "s", pluginId: "s", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDE90️space/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6070, wgpu: 6066 }, userPorts: { react: [6072, 6073], wgpu: [6067, 6068] }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "sequence", pluginId: "sequence", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAC️sequence/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6077, wgpu: 6177 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "shooting", pluginId: "shooting", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFA5️shooting/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6019, wgpu: 6119 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", roots: ["\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83C\uDF31️metabolism/\uD83C\uDFA8️representation", "♻️mit-bestand/\uD83D\uDDBC️asset/\uD83C\uDFDA️abbau-aufbau"], placeholder: "\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDBC️assets/\uD83E\uDD7D️mesh/\uD83E\uDDCA️placeholder.glb", filterFromExamples: true }] },
  { variant: "sourcing", pluginId: "sourcing", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83E\uDEB5️sourcing/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "sourcing-curate", aliases: ["curate"], ports: { react: 6081, wgpu: 6181 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "trinity-jack", pluginId: "trinity", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD31️trinity/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.trinity.jack@1/*#editor", aliases: ["trinity jack"], ports: { react: 6054, wgpu: 6154 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "trinity-rewrite", pluginId: "trinity", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDD31️trinity/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.trinity.rewrite@1/*#editor", aliases: ["trinity rewrite"], ports: { react: 6056, wgpu: 6156 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "vcs", pluginId: "vcs", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDF3F️vcs/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6075, wgpu: 6175 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "vdi3805", pluginId: "norm", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83D\uDCD5️norm/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.norm.vdi3805@1/*#editor", aliases: [], ports: { react: 6105, wgpu: 6205 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] },
  { variant: "verfolgen", pluginId: "demonstrator", cratePath: "✏️s/\uD83D\uDD0C️plugins/\uD83C\uDFAA️demonstrator/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", app: "s.gis.gismap@1/*#editor", brand: "entwerfen-mit-bestand-verfolgen", aliases: ["entwerfen-mit-bestand-verfolgen"], ports: { react: 6032, wgpu: 6132 }, examples: [], engines: ["./\uD83E\uDDF0️framework/\uD83D\uDD28️modules/\uD83D\uDDFA️surface/\uD83D\uDCE6️packages/\uD83E\uDD80️rust"], assets: [{ kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" }, { kind: "tile-proxy", route: "/vt", upstream: "https://tiles.openfreemap.org/planet", cache: "openfreemap-vt" }] },
  { variant: "writer", pluginId: "writer", cratePath: "✏️s/\uD83D\uDD0C️plugins/✒️writer/\uD83D\uDCE6️packages/\uD83E\uDD80️rust", aliases: [], ports: { react: 6062, wgpu: 6162 }, examples: ["\uD83C\uDFAC️demo-session"], engines: [], assets: [] }
];

/* ../../../../../../🔌️plugin/📇️registry/🟦️catalog.ts */
function toCatalogTarget(target) {
  return { pluginId: target.pluginId, wasmOut: target.wasmOut, role: target.role, contributes: target.contributes, consumes: target.consumes, dependsOn: target.dependsOn };
}
function toPlaygroundCatalogTarget(target) {
  return { variant: target.variant, pluginId: target.pluginId, app: target.app, aliases: target.aliases };
}
function buildPluginCatalog() {
  return {
    plugins: PLUGIN_BUILD_TARGETS.map(toCatalogTarget),
    extensions: EXTENSION_TARGETS.map(toCatalogTarget),
    hosts: PLUGIN_HOST_CONFIGS,
    playgrounds: PLAYGROUND_BUILD_TARGETS.map(toPlaygroundCatalogTarget),
    moduleUrl: pluginModuleUrl,
    extensionModuleUrl
  };
}
var PLUGIN_CATALOG = buildPluginCatalog();

/* ../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️layout.ts */
var DIAGRAM_UNIT = 48;
var DIAGRAM_LAYOUT_CODEC_KIND = "diagram-directed-layout-v1";
var DIAGRAM_LAYOUT_INGRESS_ITEMS = 64;
var DIAGRAM_LAYOUT_INGRESS_BYTES = 16 * 1024;
var DIAGRAM_LAYOUT_OUTPUT_ITEMS = 128;
var DIAGRAM_LAYOUT_MAX_INPUT_ITEMS = 65536;
var DIAGRAM_LAYOUT_MAX_ID_CHARACTERS = 512;
var DIAGRAM_LAYOUT_MAX_NODE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4;
var DIAGRAM_LAYOUT_MAX_EDGE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4 * 3;
var DIAGRAM_LAYOUT_MAX_RESERVED_BYTES = 256 * 1024 * 1024;
function diagramLayoutUtf8Bytes(value) {
  let bytes = 0;
  let characters = 0;
  for (let index = 0;index < value.length; index++) {
    characters += 1;
    if (characters > DIAGRAM_LAYOUT_MAX_ID_CHARACTERS)
      throw new Error("Diagram layout id exceeds 512 Unicode characters");
    const code = value.charCodeAt(index);
    if (code <= 127)
      bytes += 1;
    else if (code <= 2047)
      bytes += 2;
    else if (code >= 55296 && code <= 56319 && index + 1 < value.length && value.charCodeAt(index + 1) >= 56320 && value.charCodeAt(index + 1) <= 57343) {
      bytes += 4;
      index += 1;
    } else
      bytes += 3;
  }
  return bytes;
}
function diagramLayoutNodeWireBytes(value) {
  return 64 + diagramLayoutUtf8Bytes(value.id);
}
function diagramLayoutEdgeWireBytes(value) {
  return 64 + diagramLayoutUtf8Bytes(value.id) + diagramLayoutUtf8Bytes(value.source) + diagramLayoutUtf8Bytes(value.target);
}
function diagramLayoutIdentityAdmitted(value, allowEmpty = false) {
  if (typeof value !== "string" || !allowEmpty && value.length === 0)
    return false;
  try {
    diagramLayoutUtf8Bytes(value);
    return true;
  } catch {
    return false;
  }
}
function diagramLayoutCredits(nodeCount, edgeCount) {
  if (!Number.isSafeInteger(nodeCount) || !Number.isSafeInteger(edgeCount) || nodeCount < 0 || edgeCount < 0 || nodeCount + edgeCount > DIAGRAM_LAYOUT_MAX_INPUT_ITEMS)
    return { admitted: false, reason: "items" };
  const inputBytes = nodeCount * DIAGRAM_LAYOUT_MAX_NODE_BYTES + edgeCount * DIAGRAM_LAYOUT_MAX_EDGE_BYTES;
  const outputBytes = nodeCount * 32;
  if (!Number.isSafeInteger(inputBytes) || inputBytes + outputBytes > DIAGRAM_LAYOUT_MAX_RESERVED_BYTES)
    return { admitted: false, reason: "bytes" };
  return { admitted: true, inputBytes, inputItems: nodeCount + edgeCount, outputBytes, outputItems: nodeCount };
}
function asDiagramLayoutSource(values) {
  if ("get" in values && typeof values.get === "function")
    return values;
  const array = values;
  return { get: (index) => array[index], length: array.length };
}
var diagramLayoutLimits = Object.freeze({ maxEdges: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, maxNodes: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, previewNodes: 128 });
var diagramLayoutFrame = Object.freeze({ fuel: 16384, milliseconds: 6 });
var diagramLayoutPageSize = 128;

class DiagramPagedStore {
  capacity;
  directories = new Array(16);
  count = 0;
  pageHighWater = 0;
  constructor(capacity) {
    this.capacity = capacity;
  }
  get length() {
    return this.count;
  }
  get(index) {
    if (index < 0 || index >= this.count)
      return;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    return this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32]?.[index % diagramLayoutPageSize];
  }
  set(index, value) {
    if (index < 0 || index >= this.capacity)
      throw new Error("Diagram layout page capacity exceeded");
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex] ?? (this.directories[directoryIndex] = new Array(32));
    const page = directory[pageIndex % 32] ?? (directory[pageIndex % 32] = new Array(diagramLayoutPageSize));
    this.pageHighWater = Math.max(this.pageHighWater, pageIndex + 1);
    page[index % diagramLayoutPageSize] = value;
    if (index >= this.count)
      this.count = index + 1;
  }
  push(value) {
    const index = this.count;
    this.set(index, value);
    return index;
  }
  pop() {
    if (this.count === 0)
      return;
    const index = --this.count;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    const page = directory?.[pageIndex % 32];
    const value = page?.[index % diagramLayoutPageSize];
    if (page)
      page[index % diagramLayoutPageSize] = undefined;
    if (index % diagramLayoutPageSize === 0 && directory)
      directory[pageIndex % 32] = undefined;
    return value;
  }
  take(index) {
    if (index < 0 || index >= this.count)
      return;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const page = this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32];
    const offset = index % diagramLayoutPageSize;
    const value = page?.[offset];
    if (page)
      page[offset] = undefined;
    return value;
  }
  resetCleared() {
    this.count = 0;
  }
  releaseOnePage() {
    if (this.count > 0) {
      this.pop();
      return false;
    }
    if (this.pageHighWater === 0)
      return true;
    const pageIndex = --this.pageHighWater;
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    if (directory) {
      directory[pageIndex % 32] = undefined;
      if (pageIndex % 32 === 0)
        this.directories[directoryIndex] = undefined;
    }
    return this.pageHighWater === 0;
  }
  releasePageStep() {
    const retained = this.count;
    const limit = Math.max(0, retained - diagramLayoutPageSize);
    while (this.count > limit)
      this.pop();
    if (retained > 0)
      return false;
    return this.releaseOnePage();
  }
}
function finiteLayoutValue(value, fallback) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}
function optionalFiniteLayoutValue(value) {
  return value === undefined || typeof value === "number" && Number.isFinite(value);
}
function resolveLayoutOptions(options) {
  return {
    direction: options.direction ?? "TB",
    nodeHeight: Math.max(1, finiteLayoutValue(options.nodeHeight, DIAGRAM_UNIT)),
    nodeSep: Math.max(0, finiteLayoutValue(options.nodeSep, DIAGRAM_UNIT * 1.04)),
    nodeWidth: Math.max(1, finiteLayoutValue(options.nodeWidth, DIAGRAM_UNIT)),
    rankSep: Math.max(0, finiteLayoutValue(options.rankSep, DIAGRAM_UNIT * 1.67))
  };
}
function nodeLayoutDimension(node, axis, fallback) {
  const measured = node.measured?.[axis];
  const direct = node[axis];
  const style = typeof node.style?.[axis] === "number" ? node.style[axis] : undefined;
  return Math.max(1, finiteLayoutValue(measured, finiteLayoutValue(direct, finiteLayoutValue(style, fallback))));
}
function createLayoutMerge(source) {
  return { left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: new DiagramPagedStore(source.capacity), width: 1 };
}
function stepLayoutMerge(merge, compare) {
  if (merge.source.length < 2 || merge.width >= merge.source.length)
    return true;
  if (merge.left >= merge.source.length) {
    const cleared = merge.source;
    merge.source = merge.target;
    cleared.resetCleared();
    merge.target = cleared;
    merge.width *= 2;
    merge.left = 0;
    merge.leftCursor = 0;
    merge.middle = Math.min(merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.width * 2, merge.source.length);
    return merge.width >= merge.source.length;
  }
  if (merge.leftCursor >= merge.middle && merge.rightCursor >= merge.right) {
    merge.left += merge.width * 2;
    merge.leftCursor = merge.left;
    merge.middle = Math.min(merge.left + merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.left + merge.width * 2, merge.source.length);
    return false;
  }
  if (merge.rightCursor >= merge.right)
    merge.target.push(merge.source.take(merge.leftCursor++));
  else if (merge.leftCursor >= merge.middle)
    merge.target.push(merge.source.take(merge.rightCursor++));
  else
    merge.target.push(merge.source.take(compare(merge.source.get(merge.leftCursor), merge.source.get(merge.rightCursor)) <= 0 ? merge.leftCursor++ : merge.rightCursor++));
  return false;
}
function compareLayoutText(left, right) {
  return left < right ? -1 : left > right ? 1 : 0;
}
function projectLayoutNode(source, x, y) {
  return {
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    connectable: source.connectable,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    dragHandle: source.dragHandle,
    draggable: source.draggable,
    dragging: source.dragging,
    expandParent: source.expandParent,
    extent: source.extent,
    focusable: source.focusable,
    handles: source.handles,
    height: source.height,
    hidden: source.hidden,
    id: source.id,
    initialHeight: source.initialHeight,
    initialWidth: source.initialWidth,
    measured: source.measured,
    origin: source.origin,
    parentId: source.parentId,
    position: { x, y },
    resizing: source.resizing,
    selectable: source.selectable,
    selected: source.selected,
    sourcePosition: source.sourcePosition,
    style: source.style,
    targetPosition: source.targetPosition,
    type: source.type,
    width: source.width,
    zIndex: source.zIndex
  };
}
function projectLayoutEdge(source) {
  return {
    animated: source.animated,
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    focusable: source.focusable,
    hidden: source.hidden,
    id: source.id,
    interactionWidth: source.interactionWidth,
    label: source.label,
    labelBgBorderRadius: source.labelBgBorderRadius,
    labelBgPadding: source.labelBgPadding,
    labelBgStyle: source.labelBgStyle,
    labelShowBg: source.labelShowBg,
    labelStyle: source.labelStyle,
    markerEnd: source.markerEnd,
    markerStart: source.markerStart,
    reconnectable: source.reconnectable,
    selectable: source.selectable,
    selected: source.selected,
    source: source.source,
    sourceHandle: source.sourceHandle,
    style: source.style,
    target: source.target,
    targetHandle: source.targetHandle,
    type: source.type,
    zIndex: source.zIndex
  };
}
function pagedLayoutArray(store, length) {
  const target = [];
  const numericIndex = (property) => {
    if (typeof property !== "string" || !/^(0|[1-9]\d*)$/.test(property))
      return;
    const index = Number(property);
    return Number.isSafeInteger(index) && index < length ? index : undefined;
  };
  return new Proxy(target, {
    get(array, property, receiver) {
      if (property === "length")
        return length;
      const index = numericIndex(property);
      return index === undefined ? Reflect.get(array, property, receiver) : store.get(index);
    },
    getOwnPropertyDescriptor(array, property) {
      const index = numericIndex(property);
      return index === undefined ? Reflect.getOwnPropertyDescriptor(array, property) : { configurable: true, enumerable: true, value: store.get(index), writable: false };
    },
    has(array, property) {
      return numericIndex(property) !== undefined || Reflect.has(array, property);
    }
  });
}

class DiagramLayoutPublication {
  sourceNodes;
  sourceEdges;
  descriptor;
  capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  positions = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  closeStage = "positions";
  expectedPosition = 0;
  expectedSequence = 1;
  outputComplete = false;
  faulted = false;
  terminalRetained = false;
  constructor(sourceNodes, sourceEdges, descriptor) {
    this.sourceNodes = sourceNodes;
    this.sourceEdges = sourceEdges;
    this.descriptor = descriptor;
  }
  readInputPage(cursor, maxItems) {
    try {
      if (this.faulted || !Number.isSafeInteger(cursor) || cursor < 0 || cursor > this.sourceNodes.length + this.sourceEdges.length)
        return this.faultPage();
      const limit = Math.max(1, Math.min(DIAGRAM_LAYOUT_INGRESS_ITEMS, Math.floor(finiteLayoutValue(maxItems, 1))));
      if (cursor < this.sourceNodes.length)
        return this.readNodePage(cursor, limit);
      return this.readEdgePage(cursor - this.sourceNodes.length, limit);
    } catch {
      return this.faultPage();
    }
  }
  acceptOutputPage(page) {
    try {
      if (this.faulted || this.outputComplete || !Number.isSafeInteger(page.itemCount) || !Number.isSafeInteger(page.byteLength) || page.itemCount < 0 || page.itemCount > DIAGRAM_LAYOUT_OUTPUT_ITEMS || page.byteLength < 0 || page.byteLength > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.rejectOutput();
      const payload = page.payload;
      if (!payload || payload.kind !== "positions" || payload.generation !== this.descriptor.generation || payload.sequence !== this.expectedSequence || !Array.isArray(payload.values) || payload.values.length !== page.itemCount || page.byteLength !== page.itemCount * 32 || page.complete !== payload.complete || page.itemCount === 0 && this.sourceNodes.length > this.expectedPosition)
        return this.rejectOutput();
      for (let index = 0;index < payload.values.length; index++) {
        const position = payload.values[index];
        if (!position || position.index !== this.expectedPosition || position.index >= this.sourceNodes.length || !Number.isFinite(position.x) || !Number.isFinite(position.y))
          return this.rejectOutput();
        this.positions.set(position.index, { index: position.index, x: position.x, y: position.y });
        const node = this.capturedNodes?.get(position.index);
        if (node)
          node.position = { x: position.x, y: position.y };
        this.expectedPosition += 1;
      }
      const exactComplete = this.expectedPosition === this.sourceNodes.length;
      if (payload.complete !== exactComplete)
        return this.rejectOutput();
      this.expectedSequence += 1;
      this.outputComplete = payload.complete;
      return true;
    } catch {
      return this.rejectOutput();
    }
  }
  acceptTerminal(terminal) {
    this.terminalRetained = true;
    if (terminal.generation !== this.descriptor.generation || terminal.status !== "complete" || this.faulted || !this.outputComplete || this.expectedPosition !== this.sourceNodes.length || this.capturedNodes?.length !== this.sourceNodes.length || this.capturedEdges?.length !== this.sourceEdges.length) {
      this.faulted = true;
      return;
    }
    const nodes = this.capturedNodes;
    const edges = this.capturedEdges;
    this.capturedNodes = undefined;
    this.capturedEdges = undefined;
    return new DiagramLayoutPublishedResult(nodes, edges, this.sourceNodes.length, this.sourceEdges.length);
  }
  closeStep() {
    if (this.closeStage === "positions") {
      if (!this.positions.releasePageStep())
        return false;
      this.closeStage = "edges";
      return false;
    }
    if (this.closeStage === "edges") {
      if (this.capturedEdges && !this.capturedEdges.releasePageStep())
        return false;
      this.capturedEdges = undefined;
      this.closeStage = "nodes";
      return false;
    }
    if (this.closeStage === "nodes") {
      if (this.capturedNodes && !this.capturedNodes.releasePageStep())
        return false;
      this.capturedNodes = undefined;
      this.closeStage = "terminal";
      return false;
    }
    if (this.closeStage === "terminal") {
      this.terminalRetained = false;
      this.closeStage = "complete";
    }
    return true;
  }
  terminalIsEmpty() {
    return !this.terminalRetained;
  }
  readNodePage(offset, limit) {
    const values = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceNodes.length) {
      const index = offset + values.length;
      const source = this.sourceNodes[index];
      if (!source || typeof source.id !== "string")
        return this.faultPage();
      const value = {
        height: source.height,
        id: source.id,
        index,
        measuredHeight: source.measured?.height,
        measuredWidth: source.measured?.width,
        styleHeight: typeof source.style?.height === "number" ? source.style.height : undefined,
        styleWidth: typeof source.style?.width === "number" ? source.style.width : undefined,
        width: source.width
      };
      let valueBytes;
      try {
        valueBytes = diagramLayoutNodeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.faultPage();
      this.capturedNodes.set(index, projectLayoutNode(source, source.position.x, source.position.y));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceNodes.length && this.sourceEdges.length === 0;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "nodes", offset, values } };
  }
  readEdgePage(offset, limit) {
    const values = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceEdges.length) {
      const index = offset + values.length;
      const source = this.sourceEdges[index];
      if (!source || typeof source.id !== "string" || typeof source.source !== "string" || typeof source.target !== "string")
        return this.faultPage();
      const value = { id: source.id, index, source: source.source, target: source.target };
      let valueBytes;
      try {
        valueBytes = diagramLayoutEdgeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.faultPage();
      this.capturedEdges.set(index, projectLayoutEdge(source));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceEdges.length;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "edges", offset, values } };
  }
  faultPage() {
    this.faulted = true;
    return { byteLength: 0, complete: true, itemCount: 0, payload: { generation: this.descriptor.generation, kind: "seal" } };
  }
  rejectOutput() {
    this.faulted = true;
    return false;
  }
}

class DiagramLayoutPublishedResult {
  nodeStore;
  edgeStore;
  nodes;
  edges;
  constructor(nodeStore, edgeStore, nodeCount, edgeCount) {
    this.nodeStore = nodeStore;
    this.edgeStore = edgeStore;
    this.nodes = pagedLayoutArray(nodeStore, nodeCount);
    this.edges = pagedLayoutArray(edgeStore, edgeCount);
  }
  closeStep() {
    if (!this.nodeStore.releasePageStep())
      return false;
    return this.edgeStore.releasePageStep();
  }
}
class DiagramLayoutOwnedResult {
  nodes;
  edges;
  nodeCount;
  edgeCount;
  constructor(nodes, edges, nodeCount, edgeCount) {
    this.nodes = nodes;
    this.edges = edges;
    this.nodeCount = nodeCount;
    this.edgeCount = edgeCount;
  }
  takeNode(index) {
    return this.nodes.take(index);
  }
  takeEdge(index) {
    return this.edges.take(index);
  }
  closeStep() {
    if (this.nodes.length > 0)
      this.nodes.pop();
    else if (this.edges.length > 0)
      this.edges.pop();
    return this.nodes.length === 0 && this.edges.length === 0;
  }
}

class DiagramLayoutJob {
  generation;
  sourceNodes;
  sourceEdges;
  options;
  nodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  queue = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankCross = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankDepth = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankOffset = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankSpan = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edgeNext = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  previewPositions = new DiagramPagedStore(diagramLayoutLimits.previewNodes);
  layoutX = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  layoutY = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  admittedEdgeCount = 0;
  queueLength = 0;
  previewLength = 0;
  previewWriteCursor = 0;
  resultTaken = false;
  pendingEdge;
  nodeMerge;
  edgeMerge;
  crossingMerge;
  mergeSpares = new Array(9);
  mergeSpareLength = 0;
  stage = "admit-nodes";
  status = "running";
  cursor = 0;
  secondaryCursor = 0;
  queueCursor = 0;
  activeRankNode = -1;
  unresolvedCursor = 0;
  maxRank = 0;
  totalDepth = 0;
  previewSequence = 0;
  faultReason;
  closeStage = "previews";
  closeCursor = 0;
  closeArray = 0;
  closePrepared = false;
  sourceNodeCount;
  sourceEdgeCount;
  constructor(nodes, edges, options = {}, generation = 1) {
    this.generation = generation;
    this.sourceNodes = asDiagramLayoutSource(nodes);
    this.sourceEdges = asDiagramLayoutSource(edges);
    this.sourceNodeCount = nodes.length;
    this.sourceEdgeCount = edges.length;
    this.options = resolveLayoutOptions(options);
    if (nodes.length > diagramLayoutLimits.maxNodes || edges.length > diagramLayoutLimits.maxEdges)
      this.fail("Diagram layout capacity exceeded");
  }
  static fromBatchTest(nodes, edges, options = {}, generation = 1) {
    return new DiagramLayoutJob(nodes, edges, options, generation);
  }
  static fromOwnedPagedSources(nodes, edges, options = {}, generation = 1) {
    return new DiagramLayoutJob(nodes, edges, options, generation);
  }
  takeResult() {
    if (this.status !== "complete" || this.resultTaken)
      return;
    this.resultTaken = true;
    const result = new DiagramLayoutOwnedResult(this.capturedNodes, this.capturedEdges, this.sourceNodeCount, this.sourceEdgeCount);
    this.capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
    this.capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
    return result;
  }
  get reason() {
    return this.faultReason;
  }
  cancel(generation = this.generation) {
    if (generation === this.generation && this.status === "running")
      this.status = "cancelled";
  }
  takePreview() {
    if (this.previewLength === 0)
      return;
    const positions = new Array(this.previewLength);
    for (let index = 0;index < this.previewLength; index++) {
      const sourceIndex = (this.previewWriteCursor - this.previewLength + index + diagramLayoutLimits.previewNodes) % diagramLayoutLimits.previewNodes;
      positions[index] = this.previewPositions.take(sourceIndex);
    }
    this.previewPositions.resetCleared();
    this.previewLength = 0;
    this.previewWriteCursor = 0;
    return { generation: this.generation, positions, sequence: this.previewSequence };
  }
  step(work) {
    const fuel = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    if (work.generation !== this.generation)
      this.cancel();
    if (this.status !== "running" || fuel === 0)
      return { consumed: 0, stage: this.stage, status: this.status };
    let remaining = fuel;
    while (remaining > 0 && this.now() < work.deadline && this.status === "running") {
      remaining -= 1;
      this.stepUnit();
    }
    return { consumed: fuel - remaining, stage: this.stage, status: this.status };
  }
  close(work) {
    this.prepareClose();
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && this.now() < work.deadline && this.closeStage !== "complete") {
      remaining -= 1;
      this.closeUnit();
    }
    return this.closeStage === "complete";
  }
  now() {
    return typeof performance === "undefined" ? Date.now() : performance.now();
  }
  fail(reason) {
    this.faultReason = reason;
    this.status = "fault";
  }
  prepareClose() {
    if (this.closePrepared)
      return;
    this.closePrepared = true;
    for (const merge of [this.nodeMerge, this.edgeMerge, this.crossingMerge]) {
      if (!merge)
        continue;
      this.mergeSpares[this.mergeSpareLength++] = merge.source;
      this.mergeSpares[this.mergeSpareLength++] = merge.target;
    }
  }
  advance(stage) {
    this.stage = stage;
    this.cursor = 0;
    this.secondaryCursor = 0;
  }
  stepUnit() {
    if (this.stage === "admit-nodes")
      this.admitNode();
    else if (this.stage === "sort-nodes")
      this.sortNode();
    else if (this.stage === "index-nodes")
      this.indexNode();
    else if (this.stage === "admit-edges")
      this.admitEdge();
    else if (this.stage === "sort-edges")
      this.sortEdge();
    else if (this.stage === "build-graph")
      this.buildGraph();
    else if (this.stage === "assign-ranks")
      this.assignRank();
    else if (this.stage === "crossing")
      this.accumulateCrossing();
    else if (this.stage === "sort-crossing")
      this.sortCrossing();
    else if (this.stage === "measure-ranks")
      this.measureRank();
    else if (this.stage === "position-ranks")
      this.positionRank();
    else if (this.stage === "coordinates")
      this.coordinateNode();
    else if (this.stage === "project")
      this.projectNode();
    else if (this.stage === "project-edges")
      this.projectEdge();
  }
  admitNode() {
    const source = this.sourceNodes;
    if (this.cursor >= source.length) {
      this.nodeMerge = createLayoutMerge(this.nodes);
      this.sourceNodes = undefined;
      this.advance("sort-nodes");
      return;
    }
    const node = source.get(this.cursor);
    if (!diagramLayoutIdentityAdmitted(node.id)) {
      this.fail("Diagram layout node id is invalid");
      return;
    }
    const sourceIndex = this.cursor++;
    const captured = projectLayoutNode(node, node.position.x, node.position.y);
    this.capturedNodes.set(sourceIndex, captured);
    this.nodes.push({
      barycenterCount: 0,
      barycenterSum: 0,
      cross: 0,
      depth: 0,
      height: nodeLayoutDimension(captured, "height", this.options.nodeHeight),
      id: captured.id,
      indegree: 0,
      order: 0,
      outgoingHead: -1,
      outgoingTail: -1,
      processed: false,
      rank: 0,
      sourceIndex,
      width: nodeLayoutDimension(captured, "width", this.options.nodeWidth),
      x: 0,
      y: 0
    });
  }
  sortNode() {
    if (!this.nodeMerge || stepLayoutMerge(this.nodeMerge, (left, right) => compareLayoutText(left.id, right.id))) {
      if (this.nodeMerge) {
        this.nodes = this.nodeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.nodeMerge.target;
      }
      this.nodeMerge = undefined;
      this.advance("index-nodes");
    }
  }
  indexNode() {
    if (this.cursor >= this.nodes.length) {
      this.advance("admit-edges");
      return;
    }
    const node = this.nodes.get(this.cursor);
    if (this.cursor > 0 && this.nodes.get(this.cursor - 1).id === node.id) {
      this.fail("Duplicate Diagram layout node id");
      return;
    }
    node.order = this.cursor;
    this.cursor += 1;
  }
  admitEdge() {
    const source = this.sourceEdges;
    if (this.cursor >= source.length && !this.pendingEdge) {
      this.edgeMerge = createLayoutMerge(this.edges);
      this.sourceEdges = undefined;
      this.advance("sort-edges");
      return;
    }
    if (!this.pendingEdge) {
      const edge = source.get(this.cursor);
      const inputIndex = this.cursor++;
      if (!edge)
        return;
      if (!diagramLayoutIdentityAdmitted(edge.id, true) || !diagramLayoutIdentityAdmitted(edge.source) || !diagramLayoutIdentityAdmitted(edge.target)) {
        this.fail("Diagram layout edge identity is invalid");
        return;
      }
      const captured = projectLayoutEdge(edge);
      this.capturedEdges.set(inputIndex, captured);
      this.pendingEdge = { captured, inputIndex, sourceLookup: { done: false, high: this.nodes.length - 1, low: 0, value: captured.source } };
    }
    const pending = this.pendingEdge;
    if (!pending.sourceLookup.done) {
      this.stepLayoutLookup(pending.sourceLookup);
      return;
    }
    pending.targetLookup ??= { done: false, high: this.nodes.length - 1, low: 0, value: pending.captured.target };
    if (!pending.targetLookup.done) {
      this.stepLayoutLookup(pending.targetLookup);
      return;
    }
    const sourceIndex = pending.sourceLookup.result;
    const targetIndex = pending.targetLookup.result;
    if (sourceIndex !== undefined && targetIndex !== undefined) {
      const id = typeof pending.captured.id === "string" ? pending.captured.id : `${pending.captured.source}:${pending.captured.target}:${pending.inputIndex}`;
      this.edges.push({ id, source: sourceIndex, sourceId: pending.captured.source, sourceIndex: pending.inputIndex, target: targetIndex, targetId: pending.captured.target });
      this.admittedEdgeCount += 1;
    }
    this.pendingEdge = undefined;
  }
  stepLayoutLookup(lookup) {
    if (lookup.low > lookup.high) {
      lookup.done = true;
      return;
    }
    const middle = Math.floor((lookup.low + lookup.high) / 2);
    const comparison = compareLayoutText(lookup.value, this.nodes.get(middle).id);
    if (comparison === 0) {
      lookup.done = true;
      lookup.result = middle;
    } else if (comparison < 0)
      lookup.high = middle - 1;
    else
      lookup.low = middle + 1;
  }
  sortEdge() {
    if (!this.edgeMerge || stepLayoutMerge(this.edgeMerge, (left, right) => compareLayoutText(left.sourceId, right.sourceId) || compareLayoutText(left.targetId, right.targetId) || compareLayoutText(left.id, right.id))) {
      if (this.edgeMerge) {
        this.edges = this.edgeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.edgeMerge.target;
      }
      this.edgeMerge = undefined;
      this.advance("build-graph");
    }
  }
  buildGraph() {
    if (this.cursor >= this.edges.length) {
      if (this.secondaryCursor < this.nodes.length) {
        const node = this.nodes.get(this.secondaryCursor);
        if (node && node.indegree === 0)
          this.queue.set(this.queueLength++, this.secondaryCursor);
        this.secondaryCursor += 1;
        return;
      }
      this.queueCursor = 0;
      this.activeRankNode = -1;
      this.unresolvedCursor = 0;
      this.advance("assign-ranks");
      return;
    }
    const edgeIndex = this.cursor++;
    const edge = this.edges.get(edgeIndex);
    if (edge.source === edge.target)
      return;
    const source = this.nodes.get(edge.source);
    const target = this.nodes.get(edge.target);
    this.edgeNext.set(edgeIndex, -1);
    if (source.outgoingTail < 0)
      source.outgoingHead = edgeIndex;
    else
      this.edgeNext.set(source.outgoingTail, edgeIndex);
    source.outgoingTail = edgeIndex;
    target.indegree += 1;
  }
  assignRank() {
    if (this.activeRankNode >= 0) {
      const active = this.nodes.get(this.activeRankNode);
      if (this.secondaryCursor >= 0) {
        const edgeIndex = this.secondaryCursor;
        this.secondaryCursor = this.edgeNext.get(edgeIndex) ?? -1;
        const edge = this.edges.get(edgeIndex);
        const target = this.nodes.get(edge.target);
        if (!target.processed) {
          target.rank = Math.max(target.rank, active.rank + 1);
          target.indegree = Math.max(0, target.indegree - 1);
          if (target.indegree === 0)
            this.queue.set(this.queueLength++, edge.target);
        }
        return;
      }
      active.processed = true;
      this.activeRankNode = -1;
      this.secondaryCursor = -1;
      return;
    }
    if (this.queueCursor < this.queueLength) {
      const candidate = this.queue.get(this.queueCursor++);
      if (this.nodes.get(candidate).processed)
        return;
      this.activeRankNode = candidate;
      const active = this.nodes.get(candidate);
      this.secondaryCursor = active.outgoingHead;
      this.maxRank = Math.max(this.maxRank, active.rank);
      return;
    }
    if (this.unresolvedCursor < this.nodes.length) {
      const candidate = this.unresolvedCursor++;
      if (this.nodes.get(candidate).processed)
        return;
      this.nodes.get(candidate).indegree = 0;
      this.queue.set(this.queueLength++, candidate);
      return;
    }
    this.advance("crossing");
  }
  accumulateCrossing() {
    if (this.cursor >= this.edges.length) {
      this.crossingMerge = createLayoutMerge(this.nodes);
      this.advance("sort-crossing");
      return;
    }
    const edge = this.edges.get(this.cursor++);
    const source = this.nodes.get(edge.source);
    const target = this.nodes.get(edge.target);
    if (source.rank < target.rank) {
      target.barycenterCount += 1;
      target.barycenterSum += source.order;
    }
  }
  sortCrossing() {
    const compare = (left, right) => {
      if (left.rank !== right.rank)
        return left.rank - right.rank;
      const leftBarycenter = left.barycenterCount === 0 ? left.order : left.barycenterSum / left.barycenterCount;
      const rightBarycenter = right.barycenterCount === 0 ? right.order : right.barycenterSum / right.barycenterCount;
      return leftBarycenter - rightBarycenter || compareLayoutText(left.id, right.id);
    };
    if (!this.crossingMerge || stepLayoutMerge(this.crossingMerge, compare)) {
      if (this.crossingMerge) {
        this.nodes = this.crossingMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.crossingMerge.target;
      }
      this.crossingMerge = undefined;
      this.advance("measure-ranks");
    }
  }
  measureRank() {
    if (this.cursor >= this.nodes.length) {
      this.advance("position-ranks");
      return;
    }
    const node = this.nodes.get(this.cursor++);
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const rank = node.rank;
    const span = this.rankSpan.get(rank);
    this.rankSpan.set(rank, (span ?? 0) + (span === undefined ? 0 : this.options.nodeSep) + crossSize);
    this.rankDepth.set(rank, Math.max(this.rankDepth.get(rank) ?? 0, depthSize));
  }
  positionRank() {
    if (this.cursor > this.maxRank) {
      this.totalDepth = this.maxRank < 0 ? 0 : (this.rankOffset.get(this.maxRank) ?? 0) + (this.rankDepth.get(this.maxRank) ?? 0);
      this.advance("coordinates");
      return;
    }
    const rank = this.cursor++;
    this.rankOffset.set(rank, rank === 0 ? 0 : (this.rankOffset.get(rank - 1) ?? 0) + (this.rankDepth.get(rank - 1) ?? 0) + this.options.rankSep);
    this.rankCross.set(rank, -(this.rankSpan.get(rank) ?? 0) / 2);
  }
  coordinateNode() {
    if (this.cursor >= this.nodes.length) {
      this.advance("project");
      return;
    }
    const node = this.nodes.get(this.cursor++);
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const cross = (this.rankCross.get(node.rank) ?? 0) + crossSize / 2;
    const forwardDepth = (this.rankOffset.get(node.rank) ?? 0) + depthSize / 2;
    const depth = this.options.direction === "BT" || this.options.direction === "RL" ? this.totalDepth - forwardDepth : forwardDepth;
    this.rankCross.set(node.rank, cross + crossSize / 2 + this.options.nodeSep);
    node.cross = cross;
    node.depth = depth;
    node.x = horizontal ? depth - node.width / 2 : cross - node.width / 2;
    node.y = horizontal ? cross - node.height / 2 : depth - node.height / 2;
    this.layoutX.set(node.sourceIndex, node.x);
    this.layoutY.set(node.sourceIndex, node.y);
  }
  projectNode() {
    if (this.cursor >= this.sourceNodeCount) {
      this.advance("project-edges");
      return;
    }
    const sourceNode = this.capturedNodes.get(this.cursor++);
    if (!sourceNode)
      return;
    const sourceIndex = this.cursor - 1;
    const x = this.layoutX.get(sourceIndex);
    const y = this.layoutY.get(sourceIndex);
    if (x === undefined || y === undefined)
      return;
    sourceNode.position = { x, y };
    this.previewPositions.set(this.previewWriteCursor, { index: sourceIndex, x, y });
    this.previewWriteCursor = (this.previewWriteCursor + 1) % diagramLayoutLimits.previewNodes;
    this.previewLength = Math.min(diagramLayoutLimits.previewNodes, this.previewLength + 1);
    this.previewSequence += 1;
  }
  projectEdge() {
    if (this.cursor >= this.sourceEdgeCount) {
      this.stage = "complete";
      this.status = "complete";
      return;
    }
    this.capturedEdges.get(this.cursor++);
  }
  closeUnit() {
    if (this.closeStage === "previews") {
      if (this.previewLength > 0)
        this.previewPositions.take(--this.previewLength);
      else if (!this.resultTaken && this.capturedNodes.length > 0)
        this.capturedNodes.pop();
      else if (!this.resultTaken && this.capturedEdges.length > 0)
        this.capturedEdges.pop();
      else
        this.closeStage = "edges";
      return;
    }
    if (this.closeStage === "edges") {
      if (this.edges.length > 0)
        this.edges.pop();
      else
        this.closeStage = "nodes";
      return;
    }
    if (this.closeStage === "nodes") {
      const node = this.nodes.get(this.nodes.length - 1);
      if (!node) {
        this.closeStage = "spares";
        return;
      }
      this.nodes.pop();
      return;
    }
    if (this.closeStage === "spares") {
      if (this.closeCursor >= this.mergeSpareLength) {
        this.closeCursor = 0;
        this.closeStage = "indices";
        return;
      }
      if (this.mergeSpares[this.closeCursor].releaseOnePage())
        this.closeCursor += 1;
      return;
    }
    if (this.closeStage === "indices") {
      const store = this.closeIndexStore();
      if (!store) {
        this.closeCursor = 0;
        this.closeArray = 0;
        this.closeStage = "captures";
        return;
      }
      if (store.length > 0)
        store.pop();
      else
        this.closeArray += 1;
      return;
    }
    if (this.closeStage === "captures") {
      this.closeStage = "scalars";
      return;
    }
    if (this.closeStage === "scalars") {
      this.sourceNodes = undefined;
      this.sourceEdges = undefined;
      this.nodeMerge = undefined;
      this.edgeMerge = undefined;
      this.crossingMerge = undefined;
      this.closeStage = "complete";
    }
  }
  closeIndexStore() {
    if (this.closeArray === 0)
      return this.queue;
    if (this.closeArray === 1)
      return this.rankCross;
    if (this.closeArray === 2)
      return this.rankDepth;
    if (this.closeArray === 3)
      return this.rankOffset;
    if (this.closeArray === 4)
      return this.rankSpan;
    if (this.closeArray === 5)
      return this.edgeNext;
    if (this.closeArray === 6)
      return this.layoutX;
    if (this.closeArray === 7)
      return this.layoutY;
    return;
  }
}

class DiagramLayoutWireJob {
  descriptor;
  nodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  nodeReceived = 0;
  edgeReceived = 0;
  job;
  owned;
  resultCursor = 0;
  sequence = 0;
  emptyResultPublished = false;
  cancelled = false;
  ingesting = false;
  faultReason;
  constructor(descriptor) {
    this.descriptor = descriptor;
    const credits = diagramLayoutCredits(descriptor.nodeCount, descriptor.edgeCount);
    if (descriptor.kind !== DIAGRAM_LAYOUT_CODEC_KIND || !Number.isSafeInteger(descriptor.generation) || descriptor.generation < 0 || !Number.isSafeInteger(descriptor.nodeCount) || !Number.isSafeInteger(descriptor.edgeCount) || descriptor.nodeCount < 0 || descriptor.edgeCount < 0 || descriptor.nodeCount > diagramLayoutLimits.maxNodes || descriptor.edgeCount > diagramLayoutLimits.maxEdges || !credits.admitted)
      this.faultReason = "Diagram layout descriptor capacity is invalid";
  }
  get status() {
    if (this.faultReason)
      return "fault";
    if (this.cancelled)
      return "cancelled";
    if (!this.job)
      return "running";
    return this.job.step({ deadline: 0, fuel: 0, generation: this.descriptor.generation }).status;
  }
  get reason() {
    return this.faultReason ?? this.job?.reason;
  }
  ingest(page) {
    if (this.cancelled || this.faultReason || this.ingesting)
      return false;
    this.ingesting = true;
    try {
      if (!page || typeof page !== "object" || Array.isArray(page))
        return this.failIngress("Diagram layout ingress page is invalid");
      const candidate = page;
      const generation = candidate.generation;
      const kind = candidate.kind;
      if (!Number.isSafeInteger(generation) || generation !== this.descriptor.generation)
        return this.failIngress("Diagram layout ingress generation is invalid");
      if (kind === "seal")
        return this.sealIngress();
      if (kind !== "nodes" && kind !== "edges")
        return this.failIngress("Diagram layout ingress kind is invalid");
      if (this.job)
        return false;
      const offset = candidate.offset;
      const bytes = candidate.bytes;
      const complete = candidate.complete;
      const values = candidate.values;
      if (!Number.isSafeInteger(offset) || offset < 0 || !Number.isSafeInteger(bytes) || bytes < 0 || bytes > DIAGRAM_LAYOUT_INGRESS_BYTES || complete !== undefined && typeof complete !== "boolean" || !Array.isArray(values) || values.length > DIAGRAM_LAYOUT_INGRESS_ITEMS)
        return this.failIngress("Diagram layout ingress page exceeds its item or byte cap");
      const capturedNodes = kind === "nodes" ? this.captureNodes(values, offset, bytes) : undefined;
      const capturedEdges = kind === "edges" ? this.captureEdges(values, offset, bytes) : undefined;
      if (kind === "nodes" && !capturedNodes || kind === "edges" && !capturedEdges)
        return false;
      const nextNodeReceived = this.nodeReceived + (capturedNodes?.length ?? 0);
      const nextEdgeReceived = this.edgeReceived + (capturedEdges?.length ?? 0);
      const ingressComplete = nextNodeReceived === this.descriptor.nodeCount && nextEdgeReceived === this.descriptor.edgeCount;
      if (complete && !ingressComplete)
        return this.failIngress("Diagram layout ingress completed before its declared counts");
      if (this.cancelled || this.faultReason)
        return false;
      for (let index = 0;index < (capturedNodes?.length ?? 0); index++)
        this.nodes.set(this.nodeReceived + index, capturedNodes[index]);
      for (let index = 0;index < (capturedEdges?.length ?? 0); index++)
        this.edges.set(this.edgeReceived + index, capturedEdges[index]);
      this.nodeReceived = nextNodeReceived;
      this.edgeReceived = nextEdgeReceived;
      if (ingressComplete)
        this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
      return true;
    } catch {
      return this.failIngress("Diagram layout ingress value is invalid");
    } finally {
      this.ingesting = false;
    }
  }
  cancel(generation = this.descriptor.generation) {
    if (generation !== this.descriptor.generation)
      return;
    this.cancelled = true;
    this.job?.cancel(generation);
  }
  step(work) {
    if (this.faultReason)
      return { consumed: 0, stage: "complete", status: "fault" };
    if (this.cancelled && !this.job)
      return { consumed: 0, stage: "complete", status: "cancelled" };
    if (!this.job)
      return { consumed: 0, stage: "admit-nodes", status: "running" };
    const result = this.job.step(work);
    if (result.status === "fault")
      this.faultReason = this.job.reason;
    if (result.status === "complete" && this.resultCursor < this.descriptor.nodeCount)
      return { ...result, status: "running" };
    return result;
  }
  takePreviewPage() {
    return;
  }
  takeResultPage() {
    if (!this.job || this.status !== "complete")
      return;
    this.owned ??= this.job.takeResult();
    if (!this.owned)
      return;
    if (this.owned.nodeCount === 0) {
      if (this.emptyResultPublished)
        return;
      this.emptyResultPublished = true;
      this.sequence += 1;
      return { complete: true, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values: [] };
    }
    if (this.resultCursor >= this.owned.nodeCount)
      return;
    const count = Math.min(DIAGRAM_LAYOUT_OUTPUT_ITEMS, this.owned.nodeCount - this.resultCursor);
    const values = new Array(count);
    for (let index = 0;index < count; index++) {
      const sourceIndex = this.resultCursor + index;
      const node = this.owned.takeNode(sourceIndex);
      values[index] = { index: sourceIndex, x: node.position.x, y: node.position.y };
    }
    this.resultCursor += count;
    this.sequence += 1;
    return { complete: this.resultCursor === this.owned.nodeCount, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values };
  }
  close(work) {
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && (typeof performance === "undefined" ? Date.now() : performance.now()) < work.deadline) {
      remaining -= 1;
      if (this.job && !this.job.close({ deadline: work.deadline, fuel: 1 }))
        continue;
      if (this.owned && !this.owned.closeStep())
        continue;
      if (this.nodes.length > 0) {
        this.nodes.pop();
        continue;
      }
      if (this.edges.length > 0) {
        this.edges.pop();
        continue;
      }
      return true;
    }
    return false;
  }
  terminal() {
    const status = this.status;
    if (status === "running")
      return;
    if (status === "complete" && (this.resultCursor < this.descriptor.nodeCount || this.descriptor.nodeCount === 0 && !this.emptyResultPublished))
      return;
    if (status === "fault")
      return { generation: this.descriptor.generation, kind: "terminal", reason: this.reason ?? "Diagram layout fault", status };
    return { generation: this.descriptor.generation, kind: "terminal", status };
  }
  captureNodes(values, offset, declaredBytes) {
    if (offset !== this.nodeReceived || offset + values.length > this.descriptor.nodeCount)
      return this.failCapture("Diagram node ingress offset is invalid");
    let bytes = 0;
    const captured = new Array(values.length);
    for (let index = 0;index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source))
        return this.failCapture("Diagram node ingress value is invalid");
      const candidate = source;
      const value = {
        height: candidate.height,
        id: candidate.id,
        index: candidate.index,
        measuredHeight: candidate.measuredHeight,
        measuredWidth: candidate.measuredWidth,
        styleHeight: candidate.styleHeight,
        styleWidth: candidate.styleWidth,
        width: candidate.width
      };
      if (!Number.isSafeInteger(value.index) || value.index !== offset + index || typeof value.id !== "string" || value.id.length === 0 || !optionalFiniteLayoutValue(value.height) || !optionalFiniteLayoutValue(value.measuredHeight) || !optionalFiniteLayoutValue(value.measuredWidth) || !optionalFiniteLayoutValue(value.styleHeight) || !optionalFiniteLayoutValue(value.styleWidth) || !optionalFiniteLayoutValue(value.width))
        return this.failCapture("Diagram node ingress value is invalid");
      bytes += diagramLayoutNodeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.failCapture("Diagram node ingress exceeds its byte cap");
      if (!value.id)
        return this.failCapture("Diagram node id is empty");
      captured[index] = {
        data: {},
        height: value.height,
        id: value.id,
        measured: { height: value.measuredHeight, width: value.measuredWidth },
        position: { x: 0, y: 0 },
        style: { height: value.styleHeight, width: value.styleWidth },
        width: value.width
      };
    }
    if (bytes !== declaredBytes)
      return this.failCapture("Diagram node ingress byte accounting is invalid");
    return captured;
  }
  captureEdges(values, offset, declaredBytes) {
    if (offset !== this.edgeReceived || offset + values.length > this.descriptor.edgeCount)
      return this.failCapture("Diagram edge ingress offset is invalid");
    let bytes = 0;
    const captured = new Array(values.length);
    for (let index = 0;index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source))
        return this.failCapture("Diagram edge ingress value is invalid");
      const candidate = source;
      const value = { id: candidate.id, index: candidate.index, source: candidate.source, target: candidate.target };
      if (!Number.isSafeInteger(value.index) || value.index !== offset + index || typeof value.id !== "string" || typeof value.source !== "string" || typeof value.target !== "string" || value.source.length === 0 || value.target.length === 0)
        return this.failCapture("Diagram edge ingress value is invalid");
      bytes += diagramLayoutEdgeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.failCapture("Diagram edge ingress exceeds its byte cap");
      captured[index] = { id: value.id, source: value.source, target: value.target };
    }
    if (bytes !== declaredBytes)
      return this.failCapture("Diagram edge ingress byte accounting is invalid");
    return captured;
  }
  sealIngress() {
    if (this.job)
      return true;
    if (this.nodeReceived !== this.descriptor.nodeCount || this.edgeReceived !== this.descriptor.edgeCount)
      return this.failIngress("Diagram layout ingress was not complete");
    this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
    return true;
  }
  failCapture(reason) {
    this.faultReason = reason;
    return;
  }
  failIngress(reason) {
    this.faultReason = reason;
    return false;
  }
}
function createDiagramLayoutWorkerJob(descriptor) {
  return new DiagramLayoutWireJob(descriptor);
}

/* 🟦️typescript/🧵️browser-interactive-job-port.ts */
var INTERACTIVE_JOB_SLOT_CAPACITY = 16;
var INTERACTIVE_JOB_INPUT_ITEM_CAPACITY = 65536;
var INTERACTIVE_JOB_INPUT_BYTE_CAPACITY = 256 * 1024 * 1024;
var INTERACTIVE_JOB_PAGE_ITEM_CAPACITY = 128;
var INTERACTIVE_JOB_PAGE_BYTE_CAPACITY = 16 * 1024;
var INTERACTIVE_JOB_UI_BUDGET_MS = 2;
var INTERACTIVE_JOB_OBSERVER_CAPACITY = 32;
var INTERACTIVE_JOB_PORT_ITEM_CAPACITY = 262144;
var INTERACTIVE_JOB_PORT_BYTE_CAPACITY = 256 * 1024 * 1024;

class BrowserInteractiveJobPort {
  lifecycle;
  send;
  quarantineConsumer;
  schedule;
  status = "unavailable";
  slots = new Array(INTERACTIVE_JOB_SLOT_CAPACITY);
  closeCursor = 0;
  closeScheduled = false;
  reservedItems = 0;
  reservedBytes = 0;
  observers = new Array(INTERACTIVE_JOB_OBSERVER_CAPACITY);
  observerCursor = 0;
  observerNotifyScheduled = false;
  statusRevision = 0;
  statusSnapshot = { status: "unavailable", revision: 0 };
  now;
  constructor(lifecycle, send, now, quarantineConsumer, schedule = (callback) => setTimeout(callback, 0)) {
    this.lifecycle = lifecycle;
    this.send = send;
    this.quarantineConsumer = quarantineConsumer;
    this.schedule = schedule;
    this.now = now;
  }
  ready() {
    if (this.status === "unavailable") {
      this.status = "ready";
      this.publishStatus();
    }
  }
  getSnapshot() {
    return this.statusSnapshot;
  }
  observeConsumerTurn(site, durationMs) {
    if (durationMs < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${durationMs.toFixed(3)} ms`);
    return false;
  }
  subscribe(listener) {
    const slot = this.observers.findIndex((entry) => entry === undefined);
    if (slot < 0)
      throw new Error(`interactive job observer slots exceeded ${INTERACTIVE_JOB_OBSERVER_CAPACITY}`);
    this.observers[slot] = listener;
    return () => {
      this.observers[slot] = undefined;
    };
  }
  submit(descriptor, consumer) {
    if (this.status !== "ready" || descriptor.kind.length === 0 || descriptor.kind.length > 64)
      return;
    if (!admittedCount(descriptor.operation) || !admittedCount(descriptor.generation) || !admittedCount(descriptor.inputItems) || !admittedCount(descriptor.inputBytes) || !admittedCount(descriptor.outputItems) || !admittedCount(descriptor.outputBytes) || !admittedCount(descriptor.inputPageItems) || !admittedCount(descriptor.outputPageItems) || !admittedCount(descriptor.pageBytes))
      return;
    if (descriptor.inputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.inputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY || descriptor.outputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.outputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY)
      return;
    if (descriptor.inputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.outputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.pageBytes > INTERACTIVE_JOB_PAGE_BYTE_CAPACITY)
      return;
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY)
      return;
    if (this.slots.some((slot) => slot?.descriptor.operation === descriptor.operation))
      return;
    const index = this.slots.findIndex((slot) => slot === undefined);
    if (index < 0)
      return;
    this.slots[index] = { descriptor, consumer, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, closing: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    try {
      this.send({ kind: "job-submit", lifecycle: this.lifecycle, descriptor });
    } catch {
      this.slots[index] = undefined;
      this.reservedItems -= reservedItems;
      this.reservedBytes -= reservedBytes;
      return;
    }
    return { operation: descriptor.operation, generation: descriptor.generation, cancel: () => this.cancel(descriptor.operation, descriptor.generation) };
  }
  receive(message) {
    if (!message.kind.startsWith("job-"))
      return false;
    if (message.lifecycle !== this.lifecycle || this.status !== "ready")
      return true;
    if (!admittedCount(message.operation) || !admittedCount(message.generation)) {
      this.quarantine("interactive job message identity was invalid");
      return true;
    }
    const index = this.slots.findIndex((slot2) => slot2?.descriptor.operation === message.operation);
    if (index < 0)
      return true;
    const slot = this.slots[index];
    if (message.generation > slot.descriptor.generation) {
      this.quarantine(`interactive job returned future generation ${message.generation}`);
      return true;
    }
    if (message.generation < slot.descriptor.generation)
      return true;
    if (slot.closing)
      return true;
    if (message.kind === "job-input-pull") {
      if (!admittedCount(message.cursor) || message.cursor !== slot.inputCursor || !admittedCount(message.maxItems) || message.maxItems === 0 || message.maxItems > slot.descriptor.inputPageItems) {
        this.quarantine("interactive job pull exceeded fixed credits");
        return true;
      }
      const startedAt2 = this.now();
      let page;
      try {
        page = slot.consumer.readInputPage(message.cursor, Math.min(message.maxItems, slot.descriptor.inputPageItems));
      } catch (error) {
        this.quarantine(`input consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "input consumer"))
        return true;
      if (!this.admitPage(slot, page, true))
        return true;
      slot.inputCursor += page.itemCount;
      try {
        this.send({ kind: "job-input-page", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: message.cursor, page });
      } catch (error) {
        this.quarantine(`input page transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      return true;
    }
    if (message.kind === "job-output-page") {
      if (!this.admitPage(slot, message.page, false))
        return true;
      const startedAt2 = this.now();
      try {
        slot.consumer.onOutputPage(message.page);
      } catch (error) {
        this.quarantine(`output consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "output consumer"))
        return true;
      return true;
    }
    if (message.status !== "complete" && message.status !== "cancelled" && message.status !== "fault") {
      this.quarantine("interactive job returned invalid terminal status");
      return true;
    }
    const terminal = { operation: message.operation, generation: message.generation, status: message.status, ...message.detail === undefined ? {} : { detail: message.detail } };
    const startedAt = this.now();
    try {
      slot.consumer.onTerminal(terminal);
    } catch (error) {
      this.quarantine(`terminal consumer threw: ${error instanceof Error ? error.message : String(error)}`);
      slot.closing = true;
      this.scheduleClose();
      return true;
    }
    slot.closing = true;
    if (!this.observe(startedAt, "terminal consumer"))
      return true;
    this.scheduleClose();
    return true;
  }
  close() {
    if (this.status === "closed")
      return;
    this.status = "closed";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  closeStep() {
    if (this.status !== "closed" && this.status !== "quarantined")
      return false;
    return this.drainClosingStep();
  }
  drainClosingStep() {
    while (this.closeCursor < this.slots.length && (!this.slots[this.closeCursor] || !this.slots[this.closeCursor].closing))
      this.closeCursor++;
    if (this.closeCursor === this.slots.length)
      return true;
    const slot = this.slots[this.closeCursor];
    const startedAt = this.now();
    let complete = false;
    try {
      complete = slot.consumer.closeStep();
      if (complete)
        complete = slot.consumer.terminalIsEmpty();
    } catch (error) {
      this.quarantine(`consumer close threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    if (!this.observe(startedAt, "consumer close"))
      return false;
    if (complete) {
      this.releaseSlot(this.closeCursor);
      this.closeCursor++;
    }
    return false;
  }
  quarantineFromOwner() {
    if (this.status === "closed")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  cancel(operation, generation) {
    if (this.status !== "ready")
      return false;
    const slot = this.slots.find((candidate) => candidate?.descriptor.operation === operation);
    if (!slot || slot.descriptor.generation !== generation)
      return false;
    try {
      this.send({ kind: "job-cancel", lifecycle: this.lifecycle, operation, generation });
    } catch (error) {
      this.quarantine(`cancel transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    return true;
  }
  admitPage(slot, page, input) {
    const pageItemLimit = input ? slot.descriptor.inputPageItems : slot.descriptor.outputPageItems;
    if (!admittedCount(page.itemCount) || !admittedCount(page.byteLength) || typeof page.complete !== "boolean" || page.itemCount === 0 && !page.complete || page.itemCount > pageItemLimit || page.byteLength > slot.descriptor.pageBytes) {
      this.quarantine("interactive job page exceeded fixed credits");
      return false;
    }
    const items = (input ? slot.inputItems : slot.outputItems) + page.itemCount;
    const bytes = (input ? slot.inputBytes : slot.outputBytes) + page.byteLength;
    const itemLimit = input ? slot.descriptor.inputItems : slot.descriptor.outputItems;
    const byteLimit = input ? slot.descriptor.inputBytes : slot.descriptor.outputBytes;
    if (items > itemLimit || bytes > byteLimit) {
      this.quarantine("interactive job aggregate credits exhausted");
      return false;
    }
    if (page.complete && items !== itemLimit || !page.complete && items >= itemLimit) {
      this.quarantine("interactive job page completion violated declared item credits");
      return false;
    }
    if (input) {
      slot.inputItems = items;
      slot.inputBytes = bytes;
    } else {
      slot.outputItems = items;
      slot.outputBytes = bytes;
    }
    return true;
  }
  observe(startedAt, site) {
    const duration = this.now() - startedAt;
    if (duration < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${duration.toFixed(3)} ms`);
    return false;
  }
  quarantine(detail) {
    if (this.status !== "ready")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
    this.quarantineConsumer(detail);
  }
  notifyObservers() {
    this.observerCursor = 0;
    if (this.observerNotifyScheduled)
      return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  publishStatus() {
    this.statusRevision += 1;
    this.statusSnapshot = { status: this.status, revision: this.statusRevision };
    this.notifyObservers();
  }
  notifyOneObserver() {
    this.observerNotifyScheduled = false;
    while (this.observerCursor < this.observers.length && !this.observers[this.observerCursor])
      this.observerCursor++;
    if (this.observerCursor === this.observers.length)
      return;
    const observer = this.observers[this.observerCursor++];
    const startedAt = this.now();
    try {
      observer();
    } catch (error) {
      this.quarantine(`status observer threw: ${error instanceof Error ? error.message : String(error)}`);
      return;
    }
    if (!this.observe(startedAt, "status observer"))
      return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  releaseSlot(index) {
    const slot = this.slots[index];
    if (!slot)
      return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }
  scheduleClose() {
    if (this.closeScheduled)
      return;
    this.closeScheduled = true;
    this.schedule(() => {
      this.closeScheduled = false;
      this.closeCursor = 0;
      if (!this.drainClosingStep())
        this.scheduleClose();
    });
  }
}
function admittedCount(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

/* 🟦️typescript/🧵️interactive-job-registry.ts */
var DIAGRAM_DESCRIPTOR = {
  kind: DIAGRAM_LAYOUT_CODEC_KIND,
  inputPageItems: 64,
  outputPageItems: 128,
  pageBytes: 16 * 1024,
  create(descriptor) {
    const payload = descriptor.payload;
    if (payload.kind !== DIAGRAM_LAYOUT_CODEC_KIND || payload.generation !== descriptor.generation)
      return;
    return new DiagramInteractiveWorkerJob(createDiagramLayoutWorkerJob(payload), descriptor.generation);
  }
};
var INTERACTIVE_WORKER_DESCRIPTORS = Object.freeze([DIAGRAM_DESCRIPTOR]);

class DiagramInteractiveWorkerJob {
  job;
  generation;
  constructor(job, generation) {
    this.job = job;
    this.generation = generation;
  }
  acceptInput(payload) {
    return this.job.ingest(payload);
  }
  cancel() {
    this.job.cancel(this.generation);
  }
  close(step2) {
    return this.job.close({ deadline: step2.deadlineMs, fuel: step2.fuel });
  }
  step(step2) {
    return this.job.step({ deadline: step2.deadlineMs, fuel: step2.fuel, generation: this.generation }).status;
  }
  takeOutput() {
    const page = this.job.takePreviewPage() ?? this.job.takeResultPage();
    if (!page)
      return;
    return { itemCount: page.values.length, byteLength: page.values.length * 32, payload: page, complete: page.complete };
  }
  terminal() {
    const terminal = this.job.terminal();
    if (!terminal)
      return;
    return terminal.status === "fault" ? { status: "fault", detail: terminal.reason } : { status: terminal.status };
  }
}

class InteractiveWorkerScheduler {
  lifecycle;
  descriptors;
  post;
  schedule;
  now;
  fault;
  slots = new Array(INTERACTIVE_JOB_SLOT_CAPACITY);
  cursor = 0;
  scheduled = false;
  closed = false;
  closeCursor = 0;
  reservedItems = 0;
  reservedBytes = 0;
  constructor(lifecycle, descriptors, post, schedule, now, fault) {
    this.lifecycle = lifecycle;
    this.descriptors = descriptors;
    this.post = post;
    this.schedule = schedule;
    this.now = now;
    this.fault = fault;
  }
  receive(message) {
    try {
      return this.receiveOwned(message);
    } catch (error) {
      return this.protocolFault(`interactive job callback threw: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  receiveOwned(message) {
    if (!message.kind.startsWith("job-"))
      return false;
    if (this.closed || message.lifecycle !== this.lifecycle)
      return true;
    if (message.kind === "job-submit")
      return this.submit(message.descriptor);
    if (!admittedCount2(message.operation) || !admittedCount2(message.generation))
      return this.protocolFault("interactive job message identity was invalid");
    const index = this.find(message.operation);
    if (index < 0)
      return true;
    const slot = this.slots[index];
    if (message.generation > slot.descriptor.generation)
      return this.protocolFault("interactive job future generation");
    if (message.generation < slot.descriptor.generation)
      return true;
    if (message.kind === "job-cancel") {
      slot.job.cancel();
      slot.phase = "closing";
      this.scheduleRun();
      return true;
    }
    if (slot.phase !== "ingress" || message.cursor !== slot.inputCursor)
      return this.protocolFault("interactive job ingress cursor mismatch");
    if (!admitPage(message.page, slot.descriptor.inputPageItems, slot.descriptor.pageBytes) || message.page.itemCount === 0 && !message.page.complete)
      return this.protocolFault("interactive job input page exceeded fixed credits");
    const items = slot.inputItems + message.page.itemCount;
    const bytes = slot.inputBytes + message.page.byteLength;
    if (items > slot.descriptor.inputItems || bytes > slot.descriptor.inputBytes)
      return this.protocolFault("interactive job input credits exhausted");
    if (!slot.job.acceptInput(message.page.payload))
      return this.protocolFault("interactive job rejected input ownership");
    slot.inputItems = items;
    slot.inputBytes = bytes;
    slot.inputCursor += message.page.itemCount;
    if (message.page.complete) {
      slot.phase = "running";
      this.scheduleRun();
    } else {
      this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: slot.inputCursor, maxItems: slot.descriptor.inputPageItems });
    }
    return true;
  }
  close() {
    if (this.closed)
      return;
    this.closed = true;
    this.closeCursor = 0;
  }
  closeStep() {
    try {
      return this.closeOwnedStep();
    } catch (error) {
      this.protocolFault(`interactive job close callback threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
  }
  closeOwnedStep() {
    for (let scanned = 0;scanned < this.slots.length; scanned++) {
      const index = (this.closeCursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot)
        continue;
      if (slot.phase !== "closing") {
        slot.job.cancel();
        slot.phase = "closing";
        this.closeCursor = (index + 1) % this.slots.length;
        return false;
      }
      if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 }))
        this.releaseSlot(index);
      this.closeCursor = (index + 1) % this.slots.length;
      return false;
    }
    return true;
  }
  submit(descriptor) {
    if (!admitDescriptor(descriptor) || this.find(descriptor.operation) >= 0) {
      this.postTerminal(descriptor, "fault", "interactive job descriptor unavailable or saturated");
      return true;
    }
    const index = this.slots.findIndex((slot) => slot === undefined);
    const factory = this.descriptors.find((candidate) => candidate.kind === descriptor.kind);
    if (factory && (descriptor.inputPageItems !== factory.inputPageItems || descriptor.outputPageItems !== factory.outputPageItems || descriptor.pageBytes !== factory.pageBytes)) {
      this.postTerminal(descriptor, "fault", "interactive job kind limits do not match the static registry");
      return true;
    }
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY) {
      this.postTerminal(descriptor, "fault", "interactive job process credits saturated");
      return true;
    }
    const job = factory?.create(descriptor);
    if (index < 0 || !job) {
      this.postTerminal(descriptor, "fault", "interactive job kind unavailable or slots saturated");
      return true;
    }
    this.slots[index] = { descriptor, job, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, phase: "ingress", afterPublish: "running", terminalSent: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, cursor: 0, maxItems: descriptor.inputPageItems });
    return true;
  }
  scheduleRun() {
    if (this.scheduled || this.closed)
      return;
    this.scheduled = true;
    this.schedule(() => {
      this.scheduled = false;
      const startedAt = this.now();
      try {
        this.runOne();
      } catch (error) {
        this.protocolFault(`interactive job Worker callback threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      if (this.now() - startedAt >= 8)
        this.protocolFault("interactive job Worker turn exceeded budget");
    });
  }
  runOne() {
    if (this.closed)
      return;
    for (let scanned = 0;scanned < this.slots.length; scanned++) {
      const index = (this.cursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot || slot.phase === "ingress")
        continue;
      this.cursor = (index + 1) % this.slots.length;
      if (slot.phase === "running") {
        const status = slot.job.step({ deadlineMs: this.now() + 6, fuel: 16384 });
        if (status !== "running" && status !== "complete" && status !== "cancelled" && status !== "fault")
          return void this.protocolFault("interactive job returned invalid step status");
        if (status === "running" || status === "complete") {
          slot.phase = "publishing";
          slot.afterPublish = status === "running" ? "running" : "closing";
        } else
          slot.phase = "closing";
        this.scheduleRun();
        return;
      }
      if (slot.phase === "publishing") {
        const page = slot.job.takeOutput();
        if (!page)
          slot.phase = slot.afterPublish;
        else {
          if (!admitPage(page, slot.descriptor.outputPageItems, slot.descriptor.pageBytes))
            return void this.protocolFault("interactive job output page exceeded fixed credits");
          slot.outputItems += page.itemCount;
          slot.outputBytes += page.byteLength;
          if (slot.outputItems > slot.descriptor.outputItems || slot.outputBytes > slot.descriptor.outputBytes)
            return void this.protocolFault("interactive job output credits exhausted");
          this.post({ kind: "job-output-page", lifecycle: this.lifecycle, operation: slot.descriptor.operation, generation: slot.descriptor.generation, page });
          if (page.complete)
            slot.phase = "closing";
          else if (slot.afterPublish === "running")
            slot.phase = "running";
        }
        this.scheduleRun();
        return;
      }
      if (slot.phase === "closing") {
        const terminal = slot.job.terminal() ?? { status: "cancelled" };
        if (terminal.status !== "complete" && terminal.status !== "cancelled" && terminal.status !== "fault")
          return void this.protocolFault("interactive job returned invalid terminal status");
        if (!slot.terminalSent) {
          this.postTerminal(slot.descriptor, terminal.status, terminal.detail);
          slot.terminalSent = true;
          this.scheduleRun();
          return;
        }
        if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 }))
          this.releaseSlot(index);
      }
      this.scheduleRun();
      return;
    }
  }
  find(operation) {
    return this.slots.findIndex((slot) => slot?.descriptor.operation === operation);
  }
  postTerminal(descriptor, status, detail) {
    this.post({ kind: "job-terminal", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, status, ...detail === undefined ? {} : { detail } });
  }
  releaseSlot(index) {
    const slot = this.slots[index];
    if (!slot)
      return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }
  protocolFault(detail) {
    this.close();
    try {
      this.fault(detail);
    } catch {}
    return true;
  }
}
function admitDescriptor(descriptor) {
  return descriptor.kind.length > 0 && descriptor.kind.length <= 64 && admittedCount2(descriptor.operation) && admittedCount2(descriptor.generation) && admittedCount2(descriptor.inputItems) && admittedCount2(descriptor.inputBytes) && admittedCount2(descriptor.outputItems) && admittedCount2(descriptor.outputBytes) && admittedCount2(descriptor.inputPageItems) && admittedCount2(descriptor.outputPageItems) && admittedCount2(descriptor.pageBytes) && descriptor.inputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.outputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.inputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.outputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.inputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.outputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.pageBytes <= INTERACTIVE_JOB_PAGE_BYTE_CAPACITY;
}
function admitPage(page, itemCapacity, byteCapacity) {
  return admittedCount2(page.itemCount) && admittedCount2(page.byteLength) && typeof page.complete === "boolean" && page.itemCount <= itemCapacity && page.byteLength <= byteCapacity;
}
function admittedCount2(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

/* 🟦️typescript/🐚️plugin-bridge.ts */
init__glue();

/* ../../../../../../../🟦️component.ts */
init__glue();

/* ../../../../../../../../../🔨️modules/📡️replication/🟦️component.ts */
//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧫️fixtures/wire/`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.
function mutationEnvelopeToWire(envelope, timestamp, codec) {
  const packPayload = (value) => Array.from(codec.encode(value));
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
    dependencies: [...envelope.deps ?? []],
    diff: { schema: envelope.diff.schemaId, payload: packPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: packPayload(envelope.inverse.inverseDiff.payload) },
    timestamp
  };
}
function mutationEnvelopeFromWire(envelope, codec) {
  const decodePayload = (bytes) => codec.decode(new Uint8Array(bytes));
  const payload = decodePayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && "sequenceNumber" in payload ? Number(payload.sequenceNumber) : 0;
  return {
    id: envelope.mutation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: "",
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.mutation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: decodePayload(envelope.inverse.payload) },
      baseVersion: Number.isFinite(sequenceNumber) ? Math.max(0, sequenceNumber) : 0,
      dependencies: [],
      undoPolicy: "exactBaseOnly"
    }
  };
}
function writeVarintU64(out, value) {
  let remaining = value;
  for (;; ) {
    const byte = remaining & 127;
    remaining = Math.floor(remaining / 128);
    if (remaining === 0) {
      out.push(byte);
      return;
    }
    out.push(byte | 128);
  }
}
function readVarintU64(bytes, pos) {
  let result = 0;
  let shift = 1;
  for (let i = 0;i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined)
      throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result += (byte & 127) * shift;
    if ((byte & 128) === 0)
      return result;
    shift *= 128;
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}
function writeStr(out, value) {
  const bytes = new TextEncoder().encode(value);
  writeVarintU64(out, bytes.length);
  for (const byte of bytes)
    out.push(byte);
}
function readStr(bytes, pos) {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len)
    throw new Error("wire str: truncated");
  pos[0] += len;
  return new TextDecoder().decode(slice);
}
function writeBytes(out, value) {
  writeVarintU64(out, value.length);
  for (const byte of value)
    out.push(byte);
}
function readBytes(bytes, pos) {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len)
    throw new Error("wire bytes: truncated");
  pos[0] += len;
  return Array.from(slice);
}
function writeHash32(out, value) {
  if (value.length !== 32)
    throw new Error("wire hash32: expected 32 bytes");
  for (const byte of value)
    out.push(byte);
}
function readHash32(bytes, pos) {
  const slice = bytes.subarray(pos[0], pos[0] + 32);
  if (slice.length !== 32)
    throw new Error("wire hash32: truncated");
  pos[0] += 32;
  return Array.from(slice);
}
function writeBool(out, value) {
  out.push(value ? 1 : 0);
}
function readBool(bytes, pos) {
  const byte = bytes[pos[0]];
  if (byte === undefined)
    throw new Error("wire bool: truncated");
  pos[0] += 1;
  return byte !== 0;
}
function writeF64(out, value) {
  const buffer = new ArrayBuffer(8);
  new DataView(buffer).setFloat64(0, value, true);
  for (const byte of new Uint8Array(buffer))
    out.push(byte);
}
function readF64(bytes, pos) {
  const slice = bytes.subarray(pos[0], pos[0] + 8);
  if (slice.length !== 8)
    throw new Error("wire f64: truncated");
  pos[0] += 8;
  return new DataView(slice.buffer, slice.byteOffset, 8).getFloat64(0, true);
}
function writeVecBytes(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writeBytes(out, value);
}
function readVecBytes(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result = [];
  for (let i = 0;i < count; i++)
    result.push(readBytes(bytes, pos));
  return result;
}
function presencePresent(value) {
  return value !== undefined && value !== null;
}
function encodePresencePeer(peer) {
  const out = [];
  writeStr(out, peer.actor);
  let flags = 0;
  if (presencePresent(peer.label))
    flags |= 1 << 0;
  if (presencePresent(peer.presencePack))
    flags |= 1 << 1;
  if (presencePresent(peer.userId))
    flags |= 1 << 2;
  if (presencePresent(peer.role))
    flags |= 1 << 3;
  if (presencePresent(peer.dragGhostJson))
    flags |= 1 << 4;
  if (presencePresent(peer.interaction))
    flags |= 1 << 5;
  if (presencePresent(peer.color))
    flags |= 1 << 6;
  if (presencePresent(peer.surface))
    flags |= 1 << 7;
  if (peer.views.length > 0)
    flags |= 1 << 8;
  if (presencePresent(peer.ui))
    flags |= 1 << 9;
  writeVarintU64(out, flags);
  writeVarintU64(out, peer.connectedAtMs ?? 0);
  if (presencePresent(peer.label))
    writeStr(out, peer.label);
  if (presencePresent(peer.presencePack))
    writeBytes(out, peer.presencePack);
  if (presencePresent(peer.userId))
    writeStr(out, peer.userId);
  if (presencePresent(peer.role))
    writeStr(out, peer.role);
  if (presencePresent(peer.dragGhostJson))
    writeStr(out, peer.dragGhostJson);
  if (presencePresent(peer.interaction))
    writePresenceInteraction(out, peer.interaction);
  if (presencePresent(peer.color))
    out.push(peer.color);
  if (presencePresent(peer.surface))
    writeStr(out, peer.surface);
  if (peer.views.length > 0)
    writeVecPresenceWindowView(out, peer.views);
  if (presencePresent(peer.ui))
    writePresenceUi(out, peer.ui);
  return out;
}
function decodePresencePeer(bytes, pos) {
  const actor = readStr(bytes, pos);
  const flags = readVarintU64(bytes, pos);
  if (flags >> 10 !== 0)
    throw new Error(`presence peer flags: unknown flag bits set: ${flags.toString(16)}`);
  const connectedAtMs = readVarintU64(bytes, pos);
  const label = flags & 1 << 0 ? readStr(bytes, pos) : undefined;
  const presencePack = flags & 1 << 1 ? readBytes(bytes, pos) : undefined;
  const userId = flags & 1 << 2 ? readStr(bytes, pos) : undefined;
  const role = flags & 1 << 3 ? readStr(bytes, pos) : undefined;
  const dragGhostJson = flags & 1 << 4 ? readStr(bytes, pos) : undefined;
  const interaction = flags & 1 << 5 ? readPresenceInteraction(bytes, pos) : undefined;
  const color = flags & 1 << 6 ? readU8(bytes, pos) : undefined;
  const surface = flags & 1 << 7 ? readStr(bytes, pos) : undefined;
  const views = flags & 1 << 8 ? readVecPresenceWindowView(bytes, pos) : [];
  const ui = flags & 1 << 9 ? readPresenceUi(bytes, pos) : undefined;
  return { actor, connectedAtMs, label, presencePack, userId, role, dragGhostJson, interaction, color, surface, views, ui };
}
function readU8(bytes, pos) {
  const byte = bytes[pos[0]];
  if (byte === undefined)
    throw new Error("presence peer color: truncated");
  pos[0] += 1;
  return byte;
}
function writePresenceInteraction(out, interaction) {
  writeStr(out, interaction.app_id);
  writeVarintU64(out, interaction.domains.length);
  for (const domain of interaction.domains) {
    writeStr(out, domain.domain);
    writeStr(out, domain.granularity);
    writeVecStr(out, domain.selected);
    writeVecStr(out, domain.hovered);
  }
}
function readPresenceInteraction(bytes, pos) {
  const app_id = readStr(bytes, pos);
  const count = Number(readVarintU64(bytes, pos));
  const domains = [];
  for (let index = 0;index < count; index += 1) {
    domains.push({ domain: readStr(bytes, pos), granularity: readStr(bytes, pos), selected: readVecStr(bytes, pos), hovered: readVecStr(bytes, pos) });
  }
  return { app_id, domains };
}
function writePresenceViewKind(out, kind) {
  if (kind.kind === "canvas") {
    out.push(0);
    writeF64(out, kind.x);
    writeF64(out, kind.y);
    writeF64(out, kind.zoom);
  } else if (kind.kind === "orbit") {
    out.push(1);
    for (const value of [...kind.position, ...kind.target, ...kind.up])
      writeF64(out, value);
    writeF64(out, kind.fov);
  } else {
    out.push(2);
    writeF64(out, kind.lng);
    writeF64(out, kind.lat);
    writeF64(out, kind.zoom);
    writeF64(out, kind.bearing);
    writeF64(out, kind.pitch);
  }
}
function readPresenceViewKind(bytes, pos) {
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("presence view kind tag: truncated");
  pos[0] += 1;
  if (tag === 0)
    return { kind: "canvas", x: readF64(bytes, pos), y: readF64(bytes, pos), zoom: readF64(bytes, pos) };
  if (tag === 1) {
    const read3 = () => [readF64(bytes, pos), readF64(bytes, pos), readF64(bytes, pos)];
    const position = read3();
    const target = read3();
    const up = read3();
    return { kind: "orbit", position, target, up, fov: readF64(bytes, pos) };
  }
  if (tag === 2)
    return { kind: "geo", lng: readF64(bytes, pos), lat: readF64(bytes, pos), zoom: readF64(bytes, pos), bearing: readF64(bytes, pos), pitch: readF64(bytes, pos) };
  throw new Error(`presence view kind tag: unknown tag ${tag}`);
}
function writePresenceWindowView(out, view) {
  writeStr(out, view.windowId);
  writeStr(out, view.space);
  writePresenceViewKind(out, view.kind);
  writeF64(out, view.size[0]);
  writeF64(out, view.size[1]);
  writeBool(out, presencePresent(view.pointer));
  if (presencePresent(view.pointer))
    for (const value of view.pointer)
      writeF64(out, value);
}
function readPresenceWindowView(bytes, pos) {
  const windowId = readStr(bytes, pos);
  const space = readStr(bytes, pos);
  const kind = readPresenceViewKind(bytes, pos);
  const size = [readF64(bytes, pos), readF64(bytes, pos)];
  const pointer = readBool(bytes, pos) ? [readF64(bytes, pos), readF64(bytes, pos), readF64(bytes, pos)] : undefined;
  return { windowId, space, kind, size, pointer };
}
function writeVecPresenceWindowView(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writePresenceWindowView(out, value);
}
function readVecPresenceWindowView(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result = [];
  for (let i = 0;i < count; i++)
    result.push(readPresenceWindowView(bytes, pos));
  return result;
}
function writePresenceUi(out, ui) {
  writeOptStr(out, ui.hoveredPath ?? null);
  writeOptStr(out, ui.focusedPath ?? null);
  writeOptStr(out, ui.pressedPath ?? null);
}
function readPresenceUi(bytes, pos) {
  const hoveredPath = readOptStr(bytes, pos) ?? undefined;
  const focusedPath = readOptStr(bytes, pos) ?? undefined;
  const pressedPath = readOptStr(bytes, pos) ?? undefined;
  return { hoveredPath, focusedPath, pressedPath };
}
function writeOptStr(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    writeStr(out, value);
}
function readOptStr(bytes, pos) {
  return readBool(bytes, pos) ? readStr(bytes, pos) : null;
}
function writeOptBytes(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    writeBytes(out, value);
}
function readOptBytes(bytes, pos) {
  return readBool(bytes, pos) ? readBytes(bytes, pos) : null;
}
function writeOptFrontier(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    encodeFrontier(out, value);
}
function readOptFrontier(bytes, pos) {
  return readBool(bytes, pos) ? decodeFrontier(bytes, pos) : null;
}
function writeVecStr(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writeStr(out, value);
}
function readVecStr(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result = [];
  for (let i = 0;i < count; i++)
    result.push(readStr(bytes, pos));
  return result;
}
function writeVecEnvelope(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    encodeEnvelope(out, value);
}
function readVecEnvelope(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result = [];
  for (let i = 0;i < count; i++)
    result.push(decodeEnvelope(bytes, pos));
  return result;
}
function encodeHlc(out, hlc) {
  writeVarintU64(out, hlc.actor);
  writeVarintU64(out, hlc.physical_ms);
  writeVarintU64(out, hlc.logical);
}
function decodeHlc(bytes, pos) {
  const actor = readVarintU64(bytes, pos);
  const physical_ms = readVarintU64(bytes, pos);
  const logical = readVarintU64(bytes, pos);
  return { actor, physical_ms, logical };
}
function encodeEnvelope(out, envelope) {
  writeStr(out, envelope.mutation_id);
  writeStr(out, envelope.document_id);
  writeStr(out, envelope.actor);
  writeVecStr(out, envelope.dependencies);
  writeStr(out, envelope.diff.schema);
  writeBytes(out, envelope.diff.payload);
  writeStr(out, envelope.inverse.schema);
  writeBytes(out, envelope.inverse.payload);
  encodeHlc(out, envelope.timestamp);
}
function decodeEnvelope(bytes, pos) {
  const mutation_id = readStr(bytes, pos);
  const document_id = readStr(bytes, pos);
  const actor = readStr(bytes, pos);
  const dependencies = readVecStr(bytes, pos);
  const diffSchema = readStr(bytes, pos);
  const diffPayload = readBytes(bytes, pos);
  const inverseSchema = readStr(bytes, pos);
  const inversePayload = readBytes(bytes, pos);
  const timestamp = decodeHlc(bytes, pos);
  return { mutation_id, document_id, actor, dependencies, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp };
}
function encodeFrontier(out, frontier) {
  writeStr(out, frontier.document_id);
  writeVarintU64(out, frontier.head_edit_ordinal);
  writeStr(out, frontier.head_edit_id);
  writeVarintU64(out, frontier.last_commit_seq);
  writeHash32(out, frontier.chain_hash);
}
function decodeFrontier(bytes, pos) {
  const document_id = readStr(bytes, pos);
  const head_edit_ordinal = readVarintU64(bytes, pos);
  const head_edit_id = readStr(bytes, pos);
  const last_commit_seq = readVarintU64(bytes, pos);
  const chain_hash = readHash32(bytes, pos);
  return { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
}
function encodeBootstrap(out, bootstrap) {
  if (bootstrap === "None") {
    out.push(0);
    return;
  }
  if (bootstrap === "Tail") {
    out.push(2);
    return;
  }
  out.push(1);
  writeHash32(out, bootstrap.Snapshot.pack_hash);
  writeOptBytes(out, bootstrap.Snapshot.inline);
}
function decodeBootstrap(bytes, pos) {
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("wire bootstrap tag: truncated");
  pos[0] += 1;
  if (tag === 0)
    return "None";
  if (tag === 2)
    return "Tail";
  if (tag === 1)
    return { Snapshot: { pack_hash: readHash32(bytes, pos), inline: readOptBytes(bytes, pos) } };
  throw new Error(`wire bootstrap tag: unknown tag ${tag}`);
}
function encodeApplyOutcome(out, outcome) {
  if (outcome === "Accepted") {
    out.push(0);
    return;
  }
  if ("Transformed" in outcome) {
    out.push(1);
    encodeEnvelope(out, outcome.Transformed.envelope);
    return;
  }
  out.push(2);
  writeStr(out, outcome.Rejected.reason);
  writeBytes(out, outcome.Rejected.messages);
}
function decodeApplyOutcome(bytes, pos) {
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("wire apply-outcome tag: truncated");
  pos[0] += 1;
  if (tag === 0)
    return "Accepted";
  if (tag === 1)
    return { Transformed: { envelope: decodeEnvelope(bytes, pos) } };
  if (tag === 2)
    return { Rejected: { reason: readStr(bytes, pos), messages: readBytes(bytes, pos) } };
  throw new Error(`wire apply-outcome tag: unknown tag ${tag}`);
}
function encodeAckStage(out, stage) {
  if (stage === "Received") {
    out.push(0);
    return;
  }
  if (stage === "Persisted") {
    out.push(1);
    return;
  }
  out.push(2);
  encodeApplyOutcome(out, stage.Applied.outcome);
}
function decodeAckStage(bytes, pos) {
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("wire ack-stage tag: truncated");
  pos[0] += 1;
  if (tag === 0)
    return "Received";
  if (tag === 1)
    return "Persisted";
  if (tag === 2)
    return { Applied: { outcome: decodeApplyOutcome(bytes, pos) } };
  throw new Error(`wire ack-stage tag: unknown tag ${tag}`);
}
function writeVecAckStage(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    encodeAckStage(out, value);
}
function readVecAckStage(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result = [];
  for (let i = 0;i < count; i++)
    result.push(decodeAckStage(bytes, pos));
  return result;
}
var WIRE_LANE_BYTES = { command: 0, preview: 1 };
var WIRE_BYTE_LANES = ["command", "preview"];
function encodeClientFrame(frame, lane) {
  const out = [WIRE_LANE_BYTES[lane]];
  if (frame === "Bye") {
    out.push(6);
    return new Uint8Array(out);
  }
  if ("Hello" in frame) {
    out.push(0);
    const hello = frame.Hello;
    writeVarintU64(out, hello.wire_version);
    writeVarintU64(out, hello.protocol_version);
    writeStr(out, hello.schema);
    writeHash32(out, hello.pack_schema_hash);
    writeStr(out, hello.actor);
    writeOptStr(out, hello.token);
    writeOptStr(out, hello.resume_token);
    writeOptFrontier(out, hello.frontier);
  } else if ("Commands" in frame) {
    out.push(1);
    writeVarintU64(out, frame.Commands.batch_id);
    writeVecEnvelope(out, frame.Commands.envelopes);
  } else if ("FrontierAdvertise" in frame) {
    out.push(2);
    encodeFrontier(out, frame.FrontierAdvertise.frontier);
  } else if ("PreviewPublish" in frame) {
    out.push(3);
    writeStr(out, frame.PreviewPublish.key);
    writeVarintU64(out, frame.PreviewPublish.seq);
    writeBytes(out, frame.PreviewPublish.payload);
  } else if ("Presence" in frame) {
    out.push(4);
    writeBytes(out, frame.Presence.peer);
  } else if ("CreditGrant" in frame) {
    out.push(5);
    writeVarintU64(out, frame.CreditGrant.n);
  } else {
    throw new Error("encodeClientFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}
function decodeClientFrame(bytes) {
  if (bytes.length === 0)
    throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined)
    throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("wire client-frame tag: truncated");
  pos[0] += 1;
  let frame;
  switch (tag) {
    case 0: {
      const wire_version = readVarintU64(bytes, pos);
      const protocol_version = readVarintU64(bytes, pos);
      const schema = readStr(bytes, pos);
      const pack_schema_hash = readHash32(bytes, pos);
      const actor = readStr(bytes, pos);
      const token = readOptStr(bytes, pos);
      const resume_token = readOptStr(bytes, pos);
      const frontier = readOptFrontier(bytes, pos);
      frame = { Hello: { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier } };
      break;
    }
    case 1:
      frame = { Commands: { batch_id: readVarintU64(bytes, pos), envelopes: readVecEnvelope(bytes, pos) } };
      break;
    case 2:
      frame = { FrontierAdvertise: { frontier: decodeFrontier(bytes, pos) } };
      break;
    case 3:
      frame = { PreviewPublish: { key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 4:
      frame = { Presence: { peer: readBytes(bytes, pos) } };
      break;
    case 5:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 6:
      frame = "Bye";
      break;
    default:
      throw new Error(`wire client-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}
function encodeServerFrame(frame, lane) {
  const out = [WIRE_LANE_BYTES[lane]];
  if ("Welcome" in frame) {
    out.push(0);
    writeStr(out, frame.Welcome.session_id);
    writeStr(out, frame.Welcome.resume_token);
    encodeFrontier(out, frame.Welcome.server_frontier);
    encodeBootstrap(out, frame.Welcome.bootstrap);
  } else if ("SnapshotChunk" in frame) {
    out.push(1);
    writeVarintU64(out, frame.SnapshotChunk.seq);
    writeBytes(out, frame.SnapshotChunk.bytes);
  } else if ("SnapshotDone" in frame) {
    out.push(2);
    writeVarintU64(out, frame.SnapshotDone.seq_count);
  } else if ("Commands" in frame) {
    out.push(3);
    writeVecEnvelope(out, frame.Commands.envelopes);
    writeStr(out, frame.Commands.origin);
    encodeFrontier(out, frame.Commands.frontier);
  } else if ("Ack" in frame) {
    out.push(4);
    writeVarintU64(out, frame.Ack.batch_id);
    writeVecAckStage(out, frame.Ack.stages);
    encodeFrontier(out, frame.Ack.frontier);
  } else if ("Preview" in frame) {
    out.push(5);
    writeStr(out, frame.Preview.actor);
    writeStr(out, frame.Preview.key);
    writeVarintU64(out, frame.Preview.seq);
    writeBytes(out, frame.Preview.payload);
  } else if ("Presence" in frame) {
    out.push(6);
    writeVecBytes(out, frame.Presence.peers);
  } else if ("CreditGrant" in frame) {
    out.push(7);
    writeVarintU64(out, frame.CreditGrant.n);
  } else if ("Error" in frame) {
    out.push(8);
    writeStr(out, frame.Error.code);
    writeStr(out, frame.Error.message);
  } else if ("Session" in frame) {
    out.push(9);
    writeStr(out, frame.Session.actor);
    out.push(frame.Session.color);
  } else {
    throw new Error("encodeServerFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}
function decodeServerFrame(bytes) {
  if (bytes.length === 0)
    throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined)
    throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined)
    throw new Error("wire server-frame tag: truncated");
  pos[0] += 1;
  let frame;
  switch (tag) {
    case 0:
      frame = { Welcome: { session_id: readStr(bytes, pos), resume_token: readStr(bytes, pos), server_frontier: decodeFrontier(bytes, pos), bootstrap: decodeBootstrap(bytes, pos) } };
      break;
    case 1:
      frame = { SnapshotChunk: { seq: readVarintU64(bytes, pos), bytes: readBytes(bytes, pos) } };
      break;
    case 2:
      frame = { SnapshotDone: { seq_count: readVarintU64(bytes, pos) } };
      break;
    case 3:
      frame = { Commands: { envelopes: readVecEnvelope(bytes, pos), origin: readStr(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 4:
      frame = { Ack: { batch_id: readVarintU64(bytes, pos), stages: readVecAckStage(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 5:
      frame = { Preview: { actor: readStr(bytes, pos), key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 6:
      frame = { Presence: { peers: readVecBytes(bytes, pos) } };
      break;
    case 7:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 8:
      frame = { Error: { code: readStr(bytes, pos), message: readStr(bytes, pos) } };
      break;
    case 9:
      frame = { Session: { actor: readStr(bytes, pos), color: readU8(bytes, pos) } };
      break;
    default:
      throw new Error(`wire server-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("wire fixtures", () => {
    it("decodes the Rust-generated binary wire fixtures byte-identically", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const fixturesDir = join2(dirname2(fileURLToPath(import.meta.url)), "\uD83E\uDDEB️fixtures/wire");
      function loadClient(name) {
        const bytes = new Uint8Array(readFileSync(join2(fixturesDir, name)));
        const decoded = decodeClientFrame(bytes);
        expect(encodeClientFrame(decoded.frame, decoded.lane)).toEqual(bytes);
        return decoded;
      }
      function loadServer(name) {
        const bytes = new Uint8Array(readFileSync(join2(fixturesDir, name)));
        const decoded = decodeServerFrame(bytes);
        expect(encodeServerFrame(decoded.frame, decoded.lane)).toEqual(bytes);
        return decoded;
      }
      function assertOpBinaryPayload(payload) {
        expect(payload.length).toBeGreaterThan(0);
        expect(payload[0]).toBe(1);
      }
      const hello = loadClient("\uD83D\uDCE6️client-hello.bin");
      expect(hello.lane).toBe("command");
      if (typeof hello.frame === "string" || !("Hello" in hello.frame))
        throw new Error("expected a Hello frame");
      expect(hello.frame.Hello.schema).toBe("demo/v1");
      expect(hello.frame.Hello.actor).toBe("actor-1");
      const commands = loadClient("\uD83D\uDCE6️client-commands.bin");
      if (typeof commands.frame === "string" || !("Commands" in commands.frame))
        throw new Error("expected a Commands frame");
      expect(commands.frame.Commands.envelopes).toHaveLength(1);
      assertOpBinaryPayload(commands.frame.Commands.envelopes[0]?.diff.payload ?? []);
      const frontierAdvertise = loadClient("\uD83D\uDCE6️client-frontier-advertise.bin");
      if (typeof frontierAdvertise.frame === "string" || !("FrontierAdvertise" in frontierAdvertise.frame))
        throw new Error("expected a FrontierAdvertise frame");
      const previewPublish = loadClient("\uD83D\uDCE6️client-preview-publish.bin");
      if (typeof previewPublish.frame === "string" || !("PreviewPublish" in previewPublish.frame))
        throw new Error("expected a PreviewPublish frame");
      expect(previewPublish.frame.PreviewPublish.key).toBe("cursor");
      const presence = loadClient("\uD83D\uDCE6️client-presence.bin");
      if (typeof presence.frame === "string" || !("Presence" in presence.frame))
        throw new Error("expected a Presence frame");
      const peer = decodePresencePeer(new Uint8Array(presence.frame.Presence.peer), [0]);
      expect(peer.actor).toBe("actor-1");
      expect(peer.label).toBe("Ada");
      expect(peer.userId).toBe("user-9");
      expect(peer.role).toBe("owner");
      expect(peer.connectedAtMs).toBe(1700000000000);
      expect(peer.color).toBe(5);
      expect(peer.surface).toBe("s.space.home@1/*#editor");
      expect(peer.views).toHaveLength(2);
      expect(peer.views[0]).toEqual({ windowId: "w1", space: "world", kind: { kind: "orbit", position: [1, 2, 3], target: [0, 0, 0], up: [0, 1, 0], fov: 45 }, size: [1024, 768], pointer: [0.5, 0.5, 0.5] });
      expect(peer.views[1]).toEqual({ windowId: "w2", space: "canvas", kind: { kind: "canvas", x: 12.5, y: -4, zoom: 1 }, size: [800, 600], pointer: undefined });
      expect(peer.ui).toEqual({ hoveredPath: "row[2]#t1", focusedPath: undefined, pressedPath: undefined });
      expect(peer.interaction?.app_id).toBe("space");
      expect(peer.interaction?.domains).toEqual([
        { domain: "outline", granularity: "task", selected: ["t1", "t2"], hovered: [] },
        { domain: "board", granularity: "card", selected: [], hovered: ["c1"] },
        { domain: "canvas", granularity: "node", selected: ["n9"], hovered: ["n9", "n10"] }
      ]);
      expect(encodePresencePeer(peer)).toEqual(Array.from(new Uint8Array(presence.frame.Presence.peer)));
      const creditGrant = loadClient("\uD83D\uDCE6️client-credit-grant.bin");
      if (typeof creditGrant.frame === "string" || !("CreditGrant" in creditGrant.frame))
        throw new Error("expected a CreditGrant frame");
      expect(creditGrant.frame.CreditGrant.n).toBe(16);
      const bye = loadClient("\uD83D\uDCE6️client-bye.bin");
      expect(bye.frame).toBe("Bye");
      const welcomeTail = loadServer("\uD83D\uDCE6️server-welcome-tail.bin");
      if (typeof welcomeTail.frame === "string" || !("Welcome" in welcomeTail.frame))
        throw new Error("expected a Welcome frame");
      expect(welcomeTail.frame.Welcome.resume_token).toBe("resume-1");
      expect(welcomeTail.frame.Welcome.bootstrap).toBe("Tail");
      const welcomeSnapshot = loadServer("\uD83D\uDCE6️server-welcome-snapshot-inline.bin");
      if (typeof welcomeSnapshot.frame === "string" || !("Welcome" in welcomeSnapshot.frame))
        throw new Error("expected a Welcome frame");
      if (welcomeSnapshot.frame.Welcome.bootstrap === "None" || welcomeSnapshot.frame.Welcome.bootstrap === "Tail" || !("Snapshot" in welcomeSnapshot.frame.Welcome.bootstrap))
        throw new Error("expected a Snapshot bootstrap");
      expect(welcomeSnapshot.frame.Welcome.bootstrap.Snapshot.inline).toEqual([9, 9, 9]);
      const snapshotChunk = loadServer("\uD83D\uDCE6️server-snapshot-chunk.bin");
      if (typeof snapshotChunk.frame === "string" || !("SnapshotChunk" in snapshotChunk.frame))
        throw new Error("expected a SnapshotChunk frame");
      expect(snapshotChunk.frame.SnapshotChunk.bytes).toEqual([1, 2, 3, 4]);
      const snapshotDone = loadServer("\uD83D\uDCE6️server-snapshot-done.bin");
      if (typeof snapshotDone.frame === "string" || !("SnapshotDone" in snapshotDone.frame))
        throw new Error("expected a SnapshotDone frame");
      expect(snapshotDone.frame.SnapshotDone.seq_count).toBe(4);
      const serverCommands = loadServer("\uD83D\uDCE6️server-commands.bin");
      if (typeof serverCommands.frame === "string" || !("Commands" in serverCommands.frame))
        throw new Error("expected a Commands frame");
      expect(serverCommands.frame.Commands.envelopes).toHaveLength(1);
      const ackAccepted = loadServer("\uD83D\uDCE6️server-ack-accepted.bin");
      if (typeof ackAccepted.frame === "string" || !("Ack" in ackAccepted.frame))
        throw new Error("expected an Ack frame");
      expect(ackAccepted.frame.Ack.batch_id).toBe(1);
      expect(ackAccepted.frame.Ack.stages).toHaveLength(3);
      const ackTransformed = loadServer("\uD83D\uDCE6️server-ack-transformed.bin");
      if (typeof ackTransformed.frame === "string" || !("Ack" in ackTransformed.frame))
        throw new Error("expected an Ack frame");
      expect(ackTransformed.frame.Ack.batch_id).toBe(2);
      const ackRejected = loadServer("\uD83D\uDCE6️server-ack-rejected.bin");
      if (typeof ackRejected.frame === "string" || !("Ack" in ackRejected.frame))
        throw new Error("expected an Ack frame");
      expect(ackRejected.frame.Ack.batch_id).toBe(3);
      const rejectedStage = ackRejected.frame.Ack.stages.find((stage) => typeof stage !== "string" && ("Applied" in stage));
      if (typeof rejectedStage === "string" || rejectedStage === undefined || !("Applied" in rejectedStage) || typeof rejectedStage.Applied.outcome === "string" || !("Rejected" in rejectedStage.Applied.outcome))
        throw new Error("expected a rejected apply outcome");
      expect(rejectedStage.Applied.outcome.Rejected.messages).toEqual([1, 2, 3]);
      const preview = loadServer("\uD83D\uDCE6️server-preview.bin");
      if (typeof preview.frame === "string" || !("Preview" in preview.frame))
        throw new Error("expected a Preview frame");
      expect(preview.frame.Preview.key).toBe("cursor");
      const serverPresence = loadServer("\uD83D\uDCE6️server-presence.bin");
      if (typeof serverPresence.frame === "string" || !("Presence" in serverPresence.frame))
        throw new Error("expected a Presence frame");
      expect(serverPresence.frame.Presence.peers).toHaveLength(2);
      expect(JSON.parse(new TextDecoder().decode(new Uint8Array(serverPresence.frame.Presence.peers[0])))).toEqual({ id: "a" });
      expect(decodePresencePeer(new Uint8Array(serverPresence.frame.Presence.peers[1]), [0])).toEqual(peer);
      const creditGrantServer = loadServer("\uD83D\uDCE6️server-credit-grant.bin");
      if (typeof creditGrantServer.frame === "string" || !("CreditGrant" in creditGrantServer.frame))
        throw new Error("expected a CreditGrant frame");
      expect(creditGrantServer.frame.CreditGrant.n).toBe(32);
      const error = loadServer("\uD83D\uDCE6️server-error.bin");
      if (typeof error.frame === "string" || !("Error" in error.frame))
        throw new Error("expected an Error frame");
      expect(error.frame.Error.code).toBe("rejected");
      const session = loadServer("\uD83D\uDCE6️server-session.bin");
      if (typeof session.frame === "string" || !("Session" in session.frame))
        throw new Error("expected a Session frame");
      expect(session.frame.Session.actor).toBe("actor-1");
      expect(session.frame.Session.color).toBe(5);
    });
  });
}
/* ../../../../../../📇️directory/🟦️component.ts */
function emptyDirectoryReadModel() {
  return { spaces: new Map, cursor: 0, users: new Map };
}
function upsertMember(space, users, userId, role, updatedAtMs) {
  const user = users.get(userId);
  const existing = space.members.find((member) => member.userId === userId);
  if (existing) {
    existing.role = role;
  } else {
    space.members.push({ userId, email: user?.email ?? "", displayName: user?.displayName ?? "", role });
  }
  space.view.memberCount = space.members.length;
  space.view.updatedAtMs = updatedAtMs;
}
function fold(model, event) {
  if (event.seq <= model.cursor)
    return model;
  const spaces = new Map(model.spaces);
  const users = new Map(model.users);
  const next = { spaces, cursor: event.seq, users };
  const body = event.body;
  const withSpace = (spaceId, mutate) => {
    const existing = spaces.get(spaceId);
    if (!existing)
      return;
    const copy = { view: { ...existing.view }, members: existing.members.map((member) => ({ ...member })) };
    mutate(copy);
    spaces.set(spaceId, copy);
  };
  switch (body.kind) {
    case "user.created":
      users.set(body.userId, { id: body.userId, email: body.email, displayName: body.displayName, createdAtMs: event.recordedAtMs });
      break;
    case "space.created":
      spaces.set(body.spaceId, {
        view: {
          id: body.spaceId,
          name: body.name,
          kind: body.spaceKind,
          visibility: body.visibility,
          ownerUserId: body.ownerUserId,
          memberCount: 0,
          documentCount: 0,
          activeConnections: 0,
          createdAtMs: event.recordedAtMs,
          updatedAtMs: event.recordedAtMs
        },
        members: []
      });
      break;
    case "space.renamed":
      withSpace(body.spaceId, (space) => {
        space.view.name = body.name;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.visibility-changed":
      withSpace(body.spaceId, (space) => {
        space.view.visibility = body.visibility;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.archived":
      withSpace(body.spaceId, (space) => {
        space.view.kind = "archive";
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "space.deleted":
      spaces.delete(body.spaceId);
      break;
    case "member.upserted":
      withSpace(body.spaceId, (space) => upsertMember(space, users, body.userId, body.role, event.recordedAtMs));
      break;
    case "member.removed":
      withSpace(body.spaceId, (space) => {
        space.members = space.members.filter((member) => member.userId !== body.userId);
        space.view.memberCount = space.members.length;
        space.view.updatedAtMs = event.recordedAtMs;
      });
      break;
    case "invite.redeemed":
      withSpace(body.spaceId, (space) => upsertMember(space, users, body.userId, body.role, event.recordedAtMs));
      break;
  }
  return next;
}
function foldAll(model, events) {
  return events.reduce(fold, model);
}

/* ../../../../../../../🟦️component.ts */
var replicationPackCodec = { encode: encodePackValue, decode: decodePackValue };
var FRAMEWORK_SYNC_CONTROLLER_ID = "framework.sync";
var BACKBONE_ENDPOINT_PATH = "/semio-backbone";
function backboneKindFromUri(uri) {
  if (uri.startsWith("file://"))
    return "file";
  if (uri.startsWith("folder://"))
    return "folder";
  if (uri.startsWith("remote://"))
    return "remote";
  return "unknown";
}
function parseRemoteBackboneUri(uri) {
  if (!uri.startsWith("remote://"))
    return null;
  const rest = uri.slice("remote://".length);
  const firstSlash = rest.indexOf("/");
  if (firstSlash <= 0)
    return null;
  const secondSlash = rest.indexOf("/", firstSlash + 1);
  if (secondSlash <= 0)
    return null;
  return { hostPort: rest.slice(0, firstSlash), spaceId: rest.slice(firstSlash + 1, secondSlash), documentId: rest.slice(secondSlash + 1) };
}
function buildRemoteBackboneUri(hostPort, spaceId, documentId) {
  return `remote://${hostPort}/${spaceId}/${documentId}`;
}
function buildFileBackboneUri(path) {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `file://${normalized}`;
}
function buildFolderBackboneUri(path) {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `folder://${normalized}`;
}
function remoteEnvelopeUrl(remote) {
  return `http://${remote.hostPort}/spaces/${encodeURIComponent(remote.spaceId)}/documents/${encodeURIComponent(remote.documentId)}/envelope`;
}
function encodeDocumentPackBytes(pack, spr) {
  const out = [];
  writeVarintU64(out, pack.length);
  for (const byte of pack)
    out.push(byte);
  for (const byte of spr)
    out.push(byte);
  return new Uint8Array(out);
}
function decodeDocumentPackBytes(bytes) {
  const pos = [0];
  const packLen = readVarintU64(bytes, pos);
  const packEnd = pos[0] + packLen;
  if (packEnd > bytes.length)
    throw new Error("document pack bytes truncated");
  const pack = bytes.subarray(pos[0], packEnd);
  pos[0] = packEnd;
  return { pack, spr: bytes.subarray(pos[0]) };
}
function encodeDocumentPackBundle(snapshot, spr = new Uint8Array) {
  return encodeDocumentPackBytes(encodePackValue(snapshot), spr);
}
function decodeDocumentPackSnapshot(bundle) {
  const { pack } = decodeDocumentPackBytes(bundle);
  return decodePackValue(pack);
}
var BACKBONE_OCTET_STREAM = "application/octet-stream";
var BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS = 1e4;
var BACKBONE_ENVELOPE_RETRY_MIN_MS = 500;
var BACKBONE_ENVELOPE_RETRY_MAX_MS = 5000;
var BACKBONE_ENVELOPE_RETRY_WINDOW_MS = 15000;

class BackboneEnvelopeResponseError extends Error {
}
async function readBackboneEnvelopeOnce(uri, signal) {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote)
      return null;
    const response2 = await fetchWithTimeout(remoteEnvelopeUrl(remote), undefined, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal });
    if (response2.status === 404)
      return null;
    if (!response2.ok)
      throw new BackboneEnvelopeResponseError(`remote backbone read failed (${response2.status})`);
    return new Uint8Array(await response2.arrayBuffer());
  }
  const response = await fetchWithTimeout(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, undefined, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal });
  if (response.status === 404)
    return null;
  if (!response.ok)
    throw new BackboneEnvelopeResponseError(`backbone read failed (${response.status})`);
  return new Uint8Array(await response.arrayBuffer());
}
async function readBackboneEnvelope(uri, signal) {
  const retryAbort = new AbortController;
  if (signal?.aborted)
    retryAbort.abort(signal.reason);
  const onCallerAbort = () => retryAbort.abort(signal.reason);
  signal?.addEventListener("abort", onCallerAbort, { once: true });
  const windowTimer = setTimeout(() => retryAbort.abort(new Error(`backbone read: retry window exceeded after ${BACKBONE_ENVELOPE_RETRY_WINDOW_MS}ms`)), BACKBONE_ENVELOPE_RETRY_WINDOW_MS);
  try {
    return await retryWithJitteredBackoff(async () => {
      try {
        return await readBackboneEnvelopeOnce(uri, retryAbort.signal);
      } catch (error) {
        if (error instanceof BackboneEnvelopeResponseError && !retryAbort.signal.aborted)
          retryAbort.abort(error);
        throw error;
      }
    }, { minMs: BACKBONE_ENVELOPE_RETRY_MIN_MS, maxMs: BACKBONE_ENVELOPE_RETRY_MAX_MS, signal: retryAbort.signal });
  } finally {
    clearTimeout(windowTimer);
    signal?.removeEventListener("abort", onCallerAbort);
  }
}
async function writeBackboneEnvelope(uri, bundle, signal) {
  const body = Uint8Array.from(bundle).buffer;
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote)
      throw new Error(`invalid remote backbone uri: ${uri}`);
    const response2 = await fetchWithTimeout(remoteEnvelopeUrl(remote), { method: "PUT", headers: { "content-type": BACKBONE_OCTET_STREAM }, body }, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal });
    if (!response2.ok)
      throw new Error(`remote backbone write failed (${response2.status})`);
    return;
  }
  const response = await fetchWithTimeout(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, { method: "PUT", headers: { "content-type": BACKBONE_OCTET_STREAM }, body }, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal });
  if (!response.ok)
    throw new Error(`backbone write failed (${response.status})`);
}
if (import.meta.vitest) {
  const { afterEach, describe, expect, it, vi } = import.meta.vitest;
  describe("backbone envelope io", () => {
    const originalFetch = globalThis.fetch;
    afterEach(() => {
      globalThis.fetch = originalFetch;
      vi.useRealTimers();
    });
    it("readBackboneEnvelope retries a transient transport failure and then succeeds, with no real sleep", async () => {
      vi.useFakeTimers();
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        if (calls < 3)
          throw new Error("connection refused");
        return { ok: true, status: 200, arrayBuffer: async () => new Uint8Array([1, 2, 3]).buffer };
      });
      const promise = readBackboneEnvelope("folder:///doc");
      await vi.runAllTimersAsync();
      const result = await promise;
      expect(calls).toBe(3);
      expect(Array.from(result ?? [])).toEqual([1, 2, 3]);
    });
    it("readBackboneEnvelope does NOT retry a definitive non-404 server response", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 500, arrayBuffer: async () => new ArrayBuffer(0) };
      });
      await expect(readBackboneEnvelope("folder:///doc")).rejects.toThrow("backbone read failed (500)");
      expect(calls).toBe(1);
    });
    it("readBackboneEnvelope gives up after its retry window instead of hanging forever", async () => {
      vi.useFakeTimers();
      globalThis.fetch = vi.fn(async () => {
        throw new Error("connection refused");
      });
      const promise = readBackboneEnvelope("folder:///doc");
      let settled = false;
      promise.then(() => settled = true, () => settled = true);
      await vi.advanceTimersByTimeAsync(BACKBONE_ENVELOPE_RETRY_WINDOW_MS + 1000);
      expect(settled).toBe(true);
      await expect(promise).rejects.toThrow();
    });
    it("readBackboneEnvelope returns null on 404 without retrying", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 404, arrayBuffer: async () => new ArrayBuffer(0) };
      });
      const result = await readBackboneEnvelope("folder:///doc");
      expect(result).toBeNull();
      expect(calls).toBe(1);
    });
    it("writeBackboneEnvelope does NOT retry on transport failure (duplicate-write safety)", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        throw new Error("connection refused");
      });
      await expect(writeBackboneEnvelope("folder:///doc", new Uint8Array([1]))).rejects.toThrow("connection refused");
      expect(calls).toBe(1);
    });
    it("writeBackboneEnvelope propagates an external abort promptly with no leaked timer", async () => {
      const controller = new AbortController;
      globalThis.fetch = vi.fn((_url, init2) => {
        return new Promise((_resolve, reject) => {
          init2?.signal?.addEventListener("abort", () => reject(init2.signal.reason ?? new Error("aborted")));
        });
      });
      const promise = writeBackboneEnvelope("folder:///doc", new Uint8Array([1]), controller.signal);
      controller.abort(new Error("caller cancelled"));
      await expect(promise).rejects.toThrow("caller cancelled");
    });
  });
}
function encodeBackboneMessage(message) {
  const out = [];
  if (message.kind === "snapshot") {
    out.push(0);
    writeBytes(out, Array.from(message.pack));
    writeBytes(out, Array.from(message.spr));
  } else if (message.kind === "mutations") {
    out.push(1);
    writeVecEnvelope(out, message.envelopes);
  } else {
    out.push(2);
    writeVecStr(out, message.opIds);
  }
  return new Uint8Array(out);
}
function decodeBackboneMessage(bytes) {
  if (bytes.length === 0)
    throw new Error("backbone message: empty");
  const tag = bytes[0];
  const pos = [1];
  if (tag === 0) {
    const pack = new Uint8Array(readBytes(bytes, pos));
    const spr = new Uint8Array(readBytes(bytes, pos));
    return { kind: "snapshot", pack, spr };
  }
  if (tag === 1) {
    return { kind: "mutations", envelopes: readVecEnvelope(bytes, pos) };
  }
  if (tag === 2) {
    return { kind: "ack", opIds: readVecStr(bytes, pos) };
  }
  throw new Error(`backbone message: unknown tag ${tag}`);
}
function applyBackboneMessage(storedBundle, messageBytes) {
  const message = decodeBackboneMessage(messageBytes);
  if (message.kind === "snapshot")
    return encodeDocumentPackBytes(message.pack, message.spr);
  if (message.kind === "mutations") {
    if (storedBundle == null)
      throw new Error("cannot append operations before a snapshot exists");
    throw new Error("backbone operations apply requires native store — ingest envelopes through the sync actor");
  }
  throw new Error(`unsupported backbone message kind: ${message.kind}`);
}
function buildFrameworkSyncUtilities(activeUri) {
  const activeKind = activeUri ? backboneKindFromUri(activeUri) : null;
  const pressed = (kind) => activeKind === kind;
  return [
    { id: "framework.sync.file", kind: "toggle", iconId: "file-json", label: "File", category: "sync", pressed: pressed("file"), order: 0, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFile" },
    { id: "framework.sync.folder", kind: "toggle", iconId: "folder", label: "Folder", category: "sync", pressed: pressed("folder"), order: 1, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFolder" },
    { id: "framework.sync.remote", kind: "toggle", iconId: "cloud", label: "Remote", category: "sync", pressed: pressed("remote"), order: 2, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectRemote" }
  ];
}
function mediaFlowTopologicalNodeOrder(graph) {
  const adjacency = new Map;
  for (const edge of graph.edges) {
    const targets = adjacency.get(edge.sourceNodeId) ?? [];
    targets.push(edge.targetNodeId);
    adjacency.set(edge.sourceNodeId, targets);
  }
  const visited = new Set;
  const order = [];
  const dfs = (nodeId) => {
    if (visited.has(nodeId))
      return;
    visited.add(nodeId);
    for (const next of adjacency.get(nodeId) ?? [])
      dfs(next);
    order.push(nodeId);
  };
  for (const node of graph.nodes)
    dfs(node.id);
  order.reverse();
  return order;
}
function planWorkflow(graph, dirtyInstanceIds) {
  const nodeById = new Map(graph.nodes.map((node) => [node.id, node]));
  const edgesBySource = new Map;
  for (const edge of graph.edges) {
    const edges = edgesBySource.get(edge.sourceNodeId) ?? [];
    edges.push(edge);
    edgesBySource.set(edge.sourceNodeId, edges);
  }
  const order = mediaFlowTopologicalNodeOrder(graph);
  const dirty = new Set(dirtyInstanceIds);
  const deliveries = [];
  for (const nodeId of order) {
    const node = nodeById.get(nodeId);
    if (!node || !dirty.has(node.instanceId))
      continue;
    for (const edge of edgesBySource.get(nodeId) ?? []) {
      const targetNode = nodeById.get(edge.targetNodeId);
      if (!targetNode)
        continue;
      deliveries.push({
        edgeId: edge.id,
        producerInstanceId: node.instanceId,
        producerPortId: edge.sourcePortId,
        consumerInstanceId: targetNode.instanceId,
        consumerPortId: edge.targetPortId
      });
      dirty.add(targetNode.instanceId);
    }
  }
  return deliveries;
}
var JSON_BRIDGE_FIELD_ID = 1;
var PACK_TAG_FALSE = 1;
var PACK_TAG_TRUE = 2;
var PACK_TAG_F64 = 5;
var PACK_TAG_STR = 6;
var PACK_TAG_STR_INLINE = 7;
var PACK_TAG_LIST = 12;
var PACK_TAG_MAP = 16;
var PACK_TAG_VALUE = 17;
var PACK_TAG_NULL = 18;
function packPushBytes(out, bytes) {
  for (let index = 0;index < bytes.length; index++)
    out.push(bytes[index]);
}
function packByteCompare(a, b) {
  const encoder = new TextEncoder;
  const ab = encoder.encode(a);
  const bb = encoder.encode(b);
  const len = Math.min(ab.length, bb.length);
  for (let index = 0;index < len; index++) {
    const diff = ab[index] - bb[index];
    if (diff !== 0)
      return diff;
  }
  return ab.length - bb.length;
}
function packCollectStrings(value, counts) {
  if (typeof value === "string") {
    counts.set(value, (counts.get(value) ?? 0) + 1);
    return;
  }
  if (Array.isArray(value)) {
    for (const item of value)
      packCollectStrings(item, counts);
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value))
      packCollectStrings(item, counts);
  }
}
function packBuildSymbols(value) {
  const counts = new Map;
  packCollectStrings(value, counts);
  const encoder = new TextEncoder;
  const symbols = [];
  for (const [text, count] of counts)
    if (encoder.encode(text).length <= 128 || count >= 2)
      symbols.push(text);
  symbols.sort(packByteCompare);
  return symbols;
}
function packEncodeString(text, symbolIndex, out) {
  const index = symbolIndex.get(text);
  if (index !== undefined) {
    out.push(PACK_TAG_STR);
    writeVarintU64(out, index);
    return;
  }
  packEncodeStringInline(text, out);
}
function packEncodeStringInline(text, out) {
  const bytes = new TextEncoder().encode(text);
  out.push(PACK_TAG_STR_INLINE);
  writeVarintU64(out, bytes.length);
  packPushBytes(out, bytes);
}
function packDecodeString(bytes, symbols, pos) {
  const tag = bytes[pos[0]];
  pos[0] += 1;
  if (tag === PACK_TAG_STR) {
    const index = readVarintU64(bytes, pos);
    const symbol = symbols[index];
    if (symbol === undefined)
      throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
    return symbol;
  }
  if (tag === PACK_TAG_STR_INLINE) {
    const len = readVarintU64(bytes, pos);
    const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
    pos[0] += len;
    return text;
  }
  throw new Error(`decodePackValue: expected a string tag, found 0x${tag.toString(16)}`);
}
function packEncodeValue(value, symbolIndex, out) {
  if (value === null || value === undefined) {
    out.push(PACK_TAG_NULL);
    return;
  }
  if (typeof value === "boolean") {
    out.push(value ? PACK_TAG_TRUE : PACK_TAG_FALSE);
    return;
  }
  if (typeof value === "number") {
    out.push(PACK_TAG_F64);
    writeF64(out, value === 0 ? 0 : value);
    return;
  }
  if (typeof value === "string") {
    packEncodeString(value, symbolIndex, out);
    return;
  }
  if (Array.isArray(value)) {
    out.push(PACK_TAG_LIST);
    writeVarintU64(out, value.length);
    for (const item of value)
      packEncodeValue(item, symbolIndex, out);
    return;
  }
  if (typeof value === "object") {
    out.push(PACK_TAG_MAP);
    const entries = Object.entries(value).sort((a, b) => packByteCompare(a[0], b[0]));
    writeVarintU64(out, entries.length);
    for (const [key, entryValue] of entries) {
      packEncodeStringInline(key, out);
      packEncodeValue(entryValue, symbolIndex, out);
    }
    return;
  }
  throw new Error(`encodePackValue: unsupported JSON value of type ${typeof value}`);
}
function packDecodeValue(bytes, symbols, pos) {
  const tag = bytes[pos[0]];
  pos[0] += 1;
  switch (tag) {
    case PACK_TAG_NULL:
      return null;
    case PACK_TAG_FALSE:
      return false;
    case PACK_TAG_TRUE:
      return true;
    case PACK_TAG_F64:
      return readF64(bytes, pos);
    case PACK_TAG_STR: {
      const index = readVarintU64(bytes, pos);
      const symbol = symbols[index];
      if (symbol === undefined)
        throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
      return symbol;
    }
    case PACK_TAG_STR_INLINE: {
      const len = readVarintU64(bytes, pos);
      const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
      pos[0] += len;
      return text;
    }
    case PACK_TAG_LIST: {
      const count = readVarintU64(bytes, pos);
      const items = [];
      for (let i = 0;i < count; i++)
        items.push(packDecodeValue(bytes, symbols, pos));
      return items;
    }
    case PACK_TAG_MAP: {
      const count = readVarintU64(bytes, pos);
      const entries = {};
      for (let i = 0;i < count; i++) {
        const key = packDecodeString(bytes, symbols, pos);
        entries[key] = packDecodeValue(bytes, symbols, pos);
      }
      return entries;
    }
    default:
      throw new Error(`decodePackValue: unrecognized dsl value tag 0x${tag.toString(16)}`);
  }
}
function encodePackValue(value) {
  const symbols = packBuildSymbols(value);
  const symbolIndex = new Map(symbols.map((symbol, index) => [symbol, index]));
  const encoder = new TextEncoder;
  const out = [];
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
  }
  writeVarintU64(out, 1);
  writeVarintU64(out, JSON_BRIDGE_FIELD_ID);
  out.push(PACK_TAG_VALUE);
  packEncodeValue(value, symbolIndex, out);
  return new Uint8Array(out);
}
function decodePackValue(bytes) {
  const pos = [0];
  const decoder = new TextDecoder;
  const symbolCount = readVarintU64(bytes, pos);
  const symbols = [];
  for (let i = 0;i < symbolCount; i++) {
    const len = readVarintU64(bytes, pos);
    symbols.push(decoder.decode(bytes.subarray(pos[0], pos[0] + len)));
    pos[0] += len;
  }
  const fieldCount = readVarintU64(bytes, pos);
  let result = null;
  for (let i = 0;i < fieldCount; i++) {
    const fieldId = readVarintU64(bytes, pos);
    const outerTag = bytes[pos[0]];
    pos[0] += 1;
    if (outerTag !== PACK_TAG_VALUE)
      throw new Error(`decodePackValue: unexpected field tag 0x${outerTag.toString(16)} for field ${fieldId}`);
    const value = packDecodeValue(bytes, symbols, pos);
    if (fieldId === JSON_BRIDGE_FIELD_ID)
      result = value;
  }
  return result;
}
function writeOptU64(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    writeVarintU64(out, value);
}
function readOptU64(bytes, pos) {
  return readBool(bytes, pos) ? readVarintU64(bytes, pos) : null;
}
function writeOptU8(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    out.push(value);
}
function readOptU8(bytes, pos) {
  if (!readBool(bytes, pos))
    return null;
  const byte = bytes[pos[0]];
  pos[0] += 1;
  return byte;
}
function writeChildPackEntry(out, entry) {
  writeStr(out, entry.slot);
  writeStr(out, entry.child_id);
  writeStr(out, entry.dialect);
  writeBytes(out, entry.envelope_pack);
}
function readChildPackEntry(bytes, pos) {
  return { slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };
}
function writeVecChildPackEntry(out, entries) {
  writeVarintU64(out, entries.length);
  for (const entry of entries)
    writeChildPackEntry(out, entry);
}
function readVecChildPackEntry(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
var APP_COMMAND_TAGS = {
  ConfigCommand: 0,
  Command: 1,
  CommandText: 2,
  ContextMenu: 3,
  ArtifactCommand: 4,
  ApplyEnvelopes: 5,
  LoadDocument: 6,
  ReadDocument: 7,
  LoadConfig: 8,
  ReadConfig: 9,
  MediaIn: 10,
  MediaOut: 11,
  MediaFingerprint: 12,
  PureCommand: 13,
  LoadChildren: 14,
  ReadChildren: 15,
  ReadHistory: 16,
  transactionPrepare: 17,
  transactionCommit: 18,
  transactionRollback: 19,
  transactionUndo: 20,
  transactionRedo: 21,
  openArtifact: 22,
  setDefaultApp: 23,
  clearDefaultApp: 24,
  setMergePolicy: 25,
  resolveConflict: 26,
  readConflicts: 27,
  presence: 28
};
var APP_FRAME_TAGS = {
  Done: 0,
  Invocation: 1,
  DocumentChanged: 2,
  Document: 3,
  Config: 4,
  ConfigChanged: 5,
  ContextMenu: 6,
  Media: 7,
  MediaFingerprint: 8,
  Error: 9,
  Emit: 10,
  Draft: 11,
  Children: 12,
  Ephemeral: 13,
  HistorySnapshot: 14,
  transactionProposal: 15,
  transactionPrepared: 16,
  transactionCommitted: 17,
  transactionRolledBack: 18,
  MergeReport: 19,
  Conflicts: 20,
  UiPatch: 21,
  UiSnapshotEnd: 22
};
function encodeAppCommand(cmd) {
  const out = [];
  if ("ConfigCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ConfigCommand);
    writeVarintU64(out, cmd.ConfigCommand.seq);
    writeBytes(out, cmd.ConfigCommand.command);
  } else if ("Command" in cmd) {
    out.push(APP_COMMAND_TAGS.Command);
    writeVarintU64(out, cmd.Command.seq);
    writeBytes(out, cmd.Command.command);
    writeBytes(out, cmd.Command.view_state);
  } else if ("CommandText" in cmd) {
    out.push(APP_COMMAND_TAGS.CommandText);
    writeVarintU64(out, cmd.CommandText.seq);
    writeStr(out, cmd.CommandText.line);
  } else if ("ContextMenu" in cmd) {
    out.push(APP_COMMAND_TAGS.ContextMenu);
    writeVarintU64(out, cmd.ContextMenu.seq);
    writeBytes(out, cmd.ContextMenu.request);
  } else if ("ArtifactCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ArtifactCommand);
    writeVarintU64(out, cmd.ArtifactCommand.seq);
    writeBytes(out, cmd.ArtifactCommand.command);
  } else if ("ApplyEnvelopes" in cmd) {
    out.push(APP_COMMAND_TAGS.ApplyEnvelopes);
    writeVarintU64(out, cmd.ApplyEnvelopes.seq);
    writeVecEnvelope(out, cmd.ApplyEnvelopes.envelopes.map((envelope, index) => mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: 0, logical: index + 1 }, replicationPackCodec)));
  } else if ("LoadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadDocument);
    writeVarintU64(out, cmd.LoadDocument.seq);
    writeBytes(out, cmd.LoadDocument.pack);
    writeBytes(out, cmd.LoadDocument.spr);
  } else if ("ReadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocument);
    writeVarintU64(out, cmd.ReadDocument.seq);
  } else if ("LoadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadConfig);
    writeVarintU64(out, cmd.LoadConfig.seq);
    writeBytes(out, cmd.LoadConfig.pack);
    writeBytes(out, cmd.LoadConfig.spr);
  } else if ("ReadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadConfig);
    writeVarintU64(out, cmd.ReadConfig.seq);
  } else if ("MediaIn" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaIn);
    writeVarintU64(out, cmd.MediaIn.seq);
    writeStr(out, cmd.MediaIn.port);
    writeBytes(out, cmd.MediaIn.descriptor);
    writeBytes(out, cmd.MediaIn.data);
  } else if ("MediaOut" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaOut);
    writeVarintU64(out, cmd.MediaOut.seq);
    writeStr(out, cmd.MediaOut.port);
    writeBytes(out, cmd.MediaOut.request);
  } else if ("MediaFingerprint" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaFingerprint);
    writeVarintU64(out, cmd.MediaFingerprint.seq);
    writeStr(out, cmd.MediaFingerprint.port);
  } else if ("PureCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.PureCommand);
    writeVarintU64(out, cmd.PureCommand.seq);
    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.document);
    writeBytes(out, cmd.PureCommand.document_spr);
    writeBytes(out, cmd.PureCommand.config);
    writeBytes(out, cmd.PureCommand.config_spr);
    writeBytes(out, cmd.PureCommand.draft);
    writeBytes(out, cmd.PureCommand.draft_spr);
  } else if ("LoadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadChildren);
    writeVarintU64(out, cmd.LoadChildren.seq);
    writeVecChildPackEntry(out, cmd.LoadChildren.entries);
  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
  } else if ("ReadHistory" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadHistory);
    writeVarintU64(out, cmd.ReadHistory.seq);
  } else if ("transactionPrepare" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionPrepare);
    writeVarintU64(out, cmd.transactionPrepare.seq);
    writeStr(out, cmd.transactionPrepare.txn_id);
    writeStr(out, cmd.transactionPrepare.mutation_id);
    writeBytes(out, cmd.transactionPrepare.payload);
    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);
    writeStr(out, cmd.transactionPrepare.label);
    writeBytes(out, cmd.transactionPrepare.origin);
  } else if ("transactionCommit" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionCommit);
    writeVarintU64(out, cmd.transactionCommit.seq);
    writeStr(out, cmd.transactionCommit.txn_id);
  } else if ("transactionRollback" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRollback);
    writeVarintU64(out, cmd.transactionRollback.seq);
    writeStr(out, cmd.transactionRollback.txn_id);
  } else if ("transactionUndo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionUndo);
    writeVarintU64(out, cmd.transactionUndo.seq);
    writeStr(out, cmd.transactionUndo.group_id);
  } else if ("transactionRedo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRedo);
    writeVarintU64(out, cmd.transactionRedo.seq);
    writeStr(out, cmd.transactionRedo.group_id);
  } else if ("openArtifact" in cmd) {
    out.push(APP_COMMAND_TAGS.openArtifact);
    writeVarintU64(out, cmd.openArtifact.seq);
    writeStr(out, cmd.openArtifact.artifact_ref);
    out.push(cmd.openArtifact.role);
    writeStr(out, cmd.openArtifact.plugin_id);
    writeStr(out, cmd.openArtifact.app_id);
  } else if ("setDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.setDefaultApp);
    writeVarintU64(out, cmd.setDefaultApp.seq);
    writeStr(out, cmd.setDefaultApp.artifact_kind);
    writeStr(out, cmd.setDefaultApp.standard);
    writeStr(out, cmd.setDefaultApp.subset);
    out.push(cmd.setDefaultApp.role);
    writeStr(out, cmd.setDefaultApp.plugin_id);
    writeStr(out, cmd.setDefaultApp.app_id);
  } else if ("clearDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.clearDefaultApp);
    writeVarintU64(out, cmd.clearDefaultApp.seq);
    writeStr(out, cmd.clearDefaultApp.artifact_kind);
    writeStr(out, cmd.clearDefaultApp.standard);
    writeStr(out, cmd.clearDefaultApp.subset);
    out.push(cmd.clearDefaultApp.role);
  } else if ("setMergePolicy" in cmd) {
    out.push(APP_COMMAND_TAGS.setMergePolicy);
    writeVarintU64(out, cmd.setMergePolicy.seq);
    out.push(cmd.setMergePolicy.policy);
  } else if ("resolveConflict" in cmd) {
    out.push(APP_COMMAND_TAGS.resolveConflict);
    writeVarintU64(out, cmd.resolveConflict.seq);
    writeStr(out, cmd.resolveConflict.conflict_id);
    out.push(cmd.resolveConflict.resolution);
  } else if ("readConflicts" in cmd) {
    out.push(APP_COMMAND_TAGS.readConflicts);
    writeVarintU64(out, cmd.readConflicts.seq);
  } else if ("presence" in cmd) {
    out.push(APP_COMMAND_TAGS.presence);
    writeVarintU64(out, cmd.presence.seq);
    writeOptU8(out, cmd.presence.own_color);
    writeVecBytes(out, cmd.presence.peers);
  } else {
    throw new Error("encodeAppCommand: unrecognized command variant");
  }
  return new Uint8Array(out);
}
function decodeAppCommand(bytes) {
  if (bytes.length === 0)
    throw new Error("decodeAppCommand: empty frame");
  const pos = [1];
  switch (bytes[0]) {
    case APP_COMMAND_TAGS.ConfigCommand:
      return { ConfigCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.Command: {
      const seq = readVarintU64(bytes, pos);
      const command = readBytes(bytes, pos);
      const view_state = readBytes(bytes, pos);
      return { Command: { seq, command, view_state } };
    }
    case APP_COMMAND_TAGS.CommandText:
      return { CommandText: { seq: readVarintU64(bytes, pos), line: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.ContextMenu:
      return { ContextMenu: { seq: readVarintU64(bytes, pos), request: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ArtifactCommand:
      return { ArtifactCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ApplyEnvelopes: {
      const seq = readVarintU64(bytes, pos);
      const wire = readVecEnvelope(bytes, pos);
      return { ApplyEnvelopes: { seq, envelopes: wire.map((envelope) => mutationEnvelopeFromWire(envelope, replicationPackCodec)) } };
    }
    case APP_COMMAND_TAGS.LoadDocument: {
      const seq = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      return { LoadDocument: { seq, pack, spr } };
    }
    case APP_COMMAND_TAGS.ReadDocument:
      return { ReadDocument: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadConfig: {
      const seq = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      return { LoadConfig: { seq, pack, spr } };
    }
    case APP_COMMAND_TAGS.ReadConfig:
      return { ReadConfig: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.MediaIn: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { MediaIn: { seq, port, descriptor, data } };
    }
    case APP_COMMAND_TAGS.MediaOut: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const request = readBytes(bytes, pos);
      return { MediaOut: { seq, port, request } };
    }
    case APP_COMMAND_TAGS.MediaFingerprint:
      return { MediaFingerprint: { seq: readVarintU64(bytes, pos), port: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.PureCommand:
      return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), document: readBytes(bytes, pos), document_spr: readBytes(bytes, pos), config: readBytes(bytes, pos), config_spr: readBytes(bytes, pos), draft: readBytes(bytes, pos), draft_spr: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadChildren:
      return { LoadChildren: { seq: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadChildren:
      return { ReadChildren: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadHistory:
      return { ReadHistory: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionPrepare: {
      const seq = readVarintU64(bytes, pos);
      const txn_id = readStr(bytes, pos);
      const mutation_id = readStr(bytes, pos);
      const payload = readBytes(bytes, pos);
      const prepared_ops = readVecBytes(bytes, pos);
      const label = readStr(bytes, pos);
      const origin = readBytes(bytes, pos);
      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } };
    }
    case APP_COMMAND_TAGS.transactionCommit:
      return { transactionCommit: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRollback:
      return { transactionRollback: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionUndo:
      return { transactionUndo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRedo:
      return { transactionRedo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.openArtifact: {
      const seq = readVarintU64(bytes, pos);
      const artifact_ref = readStr(bytes, pos);
      const role = bytes[pos[0]];
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { openArtifact: { seq, artifact_ref, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.setDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]];
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { setDefaultApp: { seq, artifact_kind, standard, subset, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.clearDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]];
      pos[0] += 1;
      return { clearDefaultApp: { seq, artifact_kind, standard, subset, role } };
    }
    case APP_COMMAND_TAGS.setMergePolicy: {
      const seq = readVarintU64(bytes, pos);
      const policy = bytes[pos[0]];
      pos[0] += 1;
      return { setMergePolicy: { seq, policy } };
    }
    case APP_COMMAND_TAGS.resolveConflict: {
      const seq = readVarintU64(bytes, pos);
      const conflict_id = readStr(bytes, pos);
      const resolution = bytes[pos[0]];
      pos[0] += 1;
      return { resolveConflict: { seq, conflict_id, resolution } };
    }
    case APP_COMMAND_TAGS.readConflicts:
      return { readConflicts: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.presence: {
      const seq = readVarintU64(bytes, pos);
      const own_color = readOptU8(bytes, pos);
      const peers = readVecBytes(bytes, pos);
      return { presence: { seq, own_color, peers } };
    }
    default:
      throw new Error(`decodeAppCommand: unknown tag ${bytes[0]}`);
  }
}
function encodeAppFrame(frame) {
  const out = [];
  if ("Done" in frame) {
    out.push(APP_FRAME_TAGS.Done);
    writeVarintU64(out, frame.Done.in_reply_to);
  } else if ("Invocation" in frame) {
    out.push(APP_FRAME_TAGS.Invocation);
    writeVarintU64(out, frame.Invocation.in_reply_to);
    writeBytes(out, frame.Invocation.output);
    writeBytes(out, frame.Invocation.diagnostics);
    writeBytes(out, frame.Invocation.ui_scope);
    writeBytes(out, frame.Invocation.history_patch);
    writeBytes(out, frame.Invocation.messages);
  } else if ("DocumentChanged" in frame) {
    out.push(APP_FRAME_TAGS.DocumentChanged);
    writeVecBytes(out, frame.DocumentChanged.envelopes);
    writeStr(out, frame.DocumentChanged.origin);
  } else if ("Document" in frame) {
    out.push(APP_FRAME_TAGS.Document);
    writeVarintU64(out, frame.Document.in_reply_to);
    writeBytes(out, frame.Document.pack);
    writeBytes(out, frame.Document.spr);
    writeStr(out, frame.Document.ops);
  } else if ("Config" in frame) {
    out.push(APP_FRAME_TAGS.Config);
    writeVarintU64(out, frame.Config.in_reply_to);
    writeBytes(out, frame.Config.pack);
    writeBytes(out, frame.Config.spr);
    writeStr(out, frame.Config.ops);
  } else if ("ConfigChanged" in frame) {
    out.push(APP_FRAME_TAGS.ConfigChanged);
    writeVecBytes(out, frame.ConfigChanged.envelopes);
    writeStr(out, frame.ConfigChanged.origin);
  } else if ("ContextMenu" in frame) {
    out.push(APP_FRAME_TAGS.ContextMenu);
    writeVarintU64(out, frame.ContextMenu.in_reply_to);
    writeBytes(out, frame.ContextMenu.items);
  } else if ("Media" in frame) {
    out.push(APP_FRAME_TAGS.Media);
    writeVarintU64(out, frame.Media.in_reply_to);
    writeStr(out, frame.Media.port);
    writeBytes(out, frame.Media.descriptor);
    writeBytes(out, frame.Media.data);
  } else if ("MediaFingerprint" in frame) {
    out.push(APP_FRAME_TAGS.MediaFingerprint);
    writeVarintU64(out, frame.MediaFingerprint.in_reply_to);
    writeStr(out, frame.MediaFingerprint.port);
    writeBytes(out, frame.MediaFingerprint.fingerprint);
  } else if ("Error" in frame) {
    out.push(APP_FRAME_TAGS.Error);
    writeOptU64(out, frame.Error.in_reply_to);
    writeBytes(out, frame.Error.fault);
    writeBytes(out, frame.Error.report);
  } else if ("Emit" in frame) {
    out.push(APP_FRAME_TAGS.Emit);
    writeVarintU64(out, frame.Emit.in_reply_to);
    writeBytes(out, frame.Emit.document_ops);
    writeBytes(out, frame.Emit.config_ops);
    writeBytes(out, frame.Emit.draft_ops);
    writeBytes(out, frame.Emit.output);
    writeBytes(out, frame.Emit.diagnostics);
  } else if ("Draft" in frame) {
    out.push(APP_FRAME_TAGS.Draft);
    writeVarintU64(out, frame.Draft.in_reply_to);
    writeBytes(out, frame.Draft.pack);
    writeBytes(out, frame.Draft.spr);
    writeStr(out, frame.Draft.ops);
  } else if ("Children" in frame) {
    out.push(APP_FRAME_TAGS.Children);
    writeVarintU64(out, frame.Children.in_reply_to);
    writeVecChildPackEntry(out, frame.Children.entries);
  } else if ("Ephemeral" in frame) {
    out.push(APP_FRAME_TAGS.Ephemeral);
    writeBytes(out, frame.Ephemeral.presence);
    writeVarintU64(out, frame.Ephemeral.presence_generation);
    writeVarintU64(out, frame.Ephemeral.transient_generation);
    writeBytes(out, frame.Ephemeral.interaction);
  } else if ("HistorySnapshot" in frame) {
    out.push(APP_FRAME_TAGS.HistorySnapshot);
    writeVarintU64(out, frame.HistorySnapshot.in_reply_to);
    writeBytes(out, frame.HistorySnapshot.history_patch);
  } else if ("transactionProposal" in frame) {
    out.push(APP_FRAME_TAGS.transactionProposal);
    writeVarintU64(out, frame.transactionProposal.in_reply_to);
    writeStr(out, frame.transactionProposal.proposal_id);
    writeVecBytes(out, frame.transactionProposal.local_ops);
    writeStr(out, frame.transactionProposal.description);
    writeStr(out, frame.transactionProposal.coalesce_key);
    writeVecBytes(out, frame.transactionProposal.foreign);
  } else if ("transactionPrepared" in frame) {
    out.push(APP_FRAME_TAGS.transactionPrepared);
    writeStr(out, frame.transactionPrepared.txn_id);
    writeVecBytes(out, frame.transactionPrepared.foreign);
    writeBytes(out, frame.transactionPrepared.rejection);
  } else if ("transactionCommitted" in frame) {
    out.push(APP_FRAME_TAGS.transactionCommitted);
    writeStr(out, frame.transactionCommitted.txn_id);
    writeStr(out, frame.transactionCommitted.edit_id);
  } else if ("transactionRolledBack" in frame) {
    out.push(APP_FRAME_TAGS.transactionRolledBack);
    writeStr(out, frame.transactionRolledBack.txn_id);
  } else if ("MergeReport" in frame) {
    out.push(APP_FRAME_TAGS.MergeReport);
    writeOptU64(out, frame.MergeReport.in_reply_to);
    writeBytes(out, frame.MergeReport.report);
  } else if ("Conflicts" in frame) {
    out.push(APP_FRAME_TAGS.Conflicts);
    writeOptU64(out, frame.Conflicts.in_reply_to);
    writeBytes(out, frame.Conflicts.conflicts);
  } else if ("UiPatch" in frame) {
    out.push(APP_FRAME_TAGS.UiPatch);
    writeOptU64(out, frame.UiPatch.in_reply_to);
    writeStr(out, frame.UiPatch.surface);
    writeStr(out, frame.UiPatch.kind);
    writeVarintU64(out, frame.UiPatch.revision);
    writeVarintU64(out, frame.UiPatch.base_revision);
    writeBytes(out, frame.UiPatch.ops);
  } else if ("UiSnapshotEnd" in frame) {
    out.push(APP_FRAME_TAGS.UiSnapshotEnd);
    writeVarintU64(out, frame.UiSnapshotEnd.revision);
  } else {
    throw new Error("encodeAppFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}
function decodeAppFrame(bytes) {
  if (bytes.length === 0)
    throw new Error("decodeAppFrame: empty frame");
  const pos = [1];
  switch (bytes[0]) {
    case APP_FRAME_TAGS.Done:
      return { Done: { in_reply_to: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.Invocation: {
      const in_reply_to = readVarintU64(bytes, pos);
      const output = readBytes(bytes, pos);
      const diagnostics = readBytes(bytes, pos);
      const ui_scope = readBytes(bytes, pos);
      const history_patch = readBytes(bytes, pos);
      const messages = readBytes(bytes, pos);
      return { Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch, messages } };
    }
    case APP_FRAME_TAGS.DocumentChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { DocumentChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.Document: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Document: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.Config: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Config: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.ConfigChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { ConfigChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.ContextMenu:
      return { ContextMenu: { in_reply_to: readVarintU64(bytes, pos), items: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Media: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { Media: { in_reply_to, port, descriptor, data } };
    }
    case APP_FRAME_TAGS.MediaFingerprint: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const fingerprint = readBytes(bytes, pos);
      return { MediaFingerprint: { in_reply_to, port, fingerprint } };
    }
    case APP_FRAME_TAGS.Error: {
      const in_reply_to = readOptU64(bytes, pos);
      const fault = readBytes(bytes, pos);
      const report = readBytes(bytes, pos);
      return { Error: { in_reply_to, fault, report } };
    }
    case APP_FRAME_TAGS.Emit:
      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Draft:
      return { Draft: { in_reply_to: readVarintU64(bytes, pos), pack: readBytes(bytes, pos), spr: readBytes(bytes, pos), ops: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.Children:
      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.Ephemeral:
      return {
        Ephemeral: { presence: readBytes(bytes, pos), presence_generation: readVarintU64(bytes, pos), transient_generation: readVarintU64(bytes, pos), interaction: readBytes(bytes, pos) }
      };
    case APP_FRAME_TAGS.HistorySnapshot:
      return { HistorySnapshot: { in_reply_to: readVarintU64(bytes, pos), history_patch: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.transactionProposal: {
      const in_reply_to = readVarintU64(bytes, pos);
      const proposal_id = readStr(bytes, pos);
      const local_ops = readVecBytes(bytes, pos);
      const description = readStr(bytes, pos);
      const coalesce_key = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      return { transactionProposal: { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } };
    }
    case APP_FRAME_TAGS.transactionPrepared: {
      const txn_id = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      const rejection = readBytes(bytes, pos);
      return { transactionPrepared: { txn_id, foreign, rejection } };
    }
    case APP_FRAME_TAGS.transactionCommitted:
      return { transactionCommitted: { txn_id: readStr(bytes, pos), edit_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.transactionRolledBack:
      return { transactionRolledBack: { txn_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.MergeReport:
      return { MergeReport: { in_reply_to: readOptU64(bytes, pos), report: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Conflicts:
      return { Conflicts: { in_reply_to: readOptU64(bytes, pos), conflicts: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.UiPatch: {
      const in_reply_to = readOptU64(bytes, pos);
      const surface = readStr(bytes, pos);
      const kind = readStr(bytes, pos);
      const revision = readVarintU64(bytes, pos);
      const base_revision = readVarintU64(bytes, pos);
      const ops = readBytes(bytes, pos);
      return { UiPatch: { in_reply_to, surface, kind, revision, base_revision, ops } };
    }
    case APP_FRAME_TAGS.UiSnapshotEnd:
      return { UiSnapshotEnd: { revision: readVarintU64(bytes, pos) } };
    default:
      throw new Error(`decodeAppFrame: unknown tag ${bytes[0]}`);
  }
}
function decodeFaultFromWire(faultBytes, decodePackValue2) {
  try {
    const raw = decodePackValue2(new Uint8Array(faultBytes));
    if (!raw || typeof raw !== "object" || !("message" in raw))
      return null;
    return raw;
  } catch {
    return null;
  }
}
function faultDisplayMessage(faultBytes, decodePackValue2) {
  const fault = decodeFaultFromWire(faultBytes, decodePackValue2);
  if (!fault)
    return "unknown fault";
  const code = typeof fault.code === "string" ? fault.code : String(fault.code);
  return `${code}: ${fault.message}`;
}
function decodeDispatchReportFromWire(reportBytes, decodePackValue2) {
  if (reportBytes.length === 0)
    return null;
  try {
    return decodePackValue2(new Uint8Array(reportBytes));
  } catch {
    return null;
  }
}
function faultMessages(reportBytes, decodePackValue2) {
  return decodeDispatchReportFromWire(reportBytes, decodePackValue2)?.messages ?? [];
}
function decodeMergeReportFromWire(reportBytes, decodePackValue2) {
  if (reportBytes.length === 0)
    return null;
  try {
    return decodePackValue2(new Uint8Array(reportBytes));
  } catch {
    return null;
  }
}
function decodeConflictsFromWire(conflictsBytes, decodePackValue2) {
  if (conflictsBytes.length === 0)
    return [];
  try {
    return decodePackValue2(new Uint8Array(conflictsBytes));
  } catch {
    return [];
  }
}
var APP_CHANNEL_VERSION = 12;

class AppChannelClient {
  seq = 0;
  handle;
  instanceId;
  appId;
  actor;
  outcomeIterator;
  pending = [];
  cachedPack = null;
  cachedSpr = null;
  constructor(handle, instanceId, appId, actor = "local") {
    this.handle = handle;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
    this.pumpOutcomes();
  }
  async pumpOutcomes() {
    for (;; ) {
      const step2 = await this.outcomeIterator.next();
      if (step2.done)
        return;
      const outcome = step2.value;
      if (outcome.instanceId !== this.instanceId)
        continue;
      const waiter = this.pending.shift();
      if (!waiter)
        continue;
      if ("error" in outcome) {
        waiter.reject(outcome.error);
        continue;
      }
      const frames = outcome.frames.map(decodeAppFrame);
      this.captureDocumentFrames(frames);
      waiter.resolve(frames);
    }
  }
  dispose() {
    this.outcomeIterator.return?.();
  }
  nextSeq() {
    this.seq += 1;
    return this.seq;
  }
  captureDocumentFrames(frames) {
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }
  documentPack() {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack, spr: this.cachedSpr } : null;
  }
  sendCommand(command) {
    return new Promise((resolve2, reject) => {
      this.pending.push({ resolve: resolve2, reject });
      this.handle.enqueue(this.instanceId, [encodeAppCommand(command)]);
    });
  }
  async command(commandBytes, viewState) {
    return this.sendCommand({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewState)) }
    });
  }
  async configure(config) {
    return this.sendCommand({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }
  async readDocument() {
    return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } });
  }
  async loadDocument(pack, spr) {
    this.cachedPack = pack;
    this.cachedSpr = spr;
    return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });
  }
  async readHistory() {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }
  async openArtifact(artifactRef, role, pluginId = "", appId = "") {
    return this.sendCommand({ openArtifact: { seq: this.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }
  async setDefaultApp(artifactKind, standard, subset, role, pluginId, appId) {
    return this.sendCommand({ setDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }
  async clearDefaultApp(artifactKind, standard, subset, role) {
    return this.sendCommand({ clearDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }
  async contextMenu(request) {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) }
    });
    const errorFrame = frames.find((frame) => ("Error" in frame));
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    }
    const menuFrame = frames.find((frame) => ("ContextMenu" in frame) && frame.ContextMenu.in_reply_to === seq);
    if (!menuFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }
  async applyEnvelopes(envelopes) {
    return this.sendCommand({ ApplyEnvelopes: { seq: this.nextSeq(), envelopes } });
  }
  async setMergePolicy(policy) {
    return this.sendCommand({ setMergePolicy: { seq: this.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }
  async resolveConflict(conflictId, resolution) {
    return this.sendCommand({ resolveConflict: { seq: this.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }
  async readConflicts() {
    return this.sendCommand({ readConflicts: { seq: this.nextSeq() } });
  }
  async pushPresence(ownColor, peers) {
    return this.sendCommand({ presence: { seq: this.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  async transactionPrepareOwner(txnId, mutationId, payload) {
    return this.sendCommand({
      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], label: "", origin: [] }
    });
  }
  async transactionPreparePlanned(txnId, preparedOps, label, origin) {
    return this.sendCommand({
      transactionPrepare: {
        seq: this.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        label,
        origin: Array.from(origin)
      }
    });
  }
  async transactionCommit(txnId) {
    return this.sendCommand({ transactionCommit: { seq: this.nextSeq(), txn_id: txnId } });
  }
  async transactionRollback(txnId) {
    return this.sendCommand({ transactionRollback: { seq: this.nextSeq(), txn_id: txnId } });
  }
  async transactionUndo(groupId) {
    return this.sendCommand({ transactionUndo: { seq: this.nextSeq(), group_id: groupId } });
  }
  async transactionRedo(groupId) {
    return this.sendCommand({ transactionRedo: { seq: this.nextSeq(), group_id: groupId } });
  }
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("@semio-tech/framework-os backbone", () => {
    it("classifies backbone uri kinds", () => {
      expect(backboneKindFromUri("file:///tmp/a.json")).toBe("file");
      expect(backboneKindFromUri("folder:///tmp")).toBe("folder");
      expect(backboneKindFromUri("remote://host:1234/doc-1")).toBe("remote");
      expect(backboneKindFromUri("other://x")).toBe("unknown");
    });
    it("builds and parses backbone uris", () => {
      expect(buildFileBackboneUri("tmp/a.json")).toBe("file:///tmp/a.json");
      expect(buildFolderBackboneUri("tmp")).toBe("folder:///tmp");
      expect(buildRemoteBackboneUri("localhost:1234", "studio-1", "doc-1")).toBe("remote://localhost:1234/studio-1/doc-1");
      expect(parseRemoteBackboneUri("remote://localhost:1234/studio-1/doc-1")).toEqual({ hostPort: "localhost:1234", spaceId: "studio-1", documentId: "doc-1" });
      expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toBeNull();
      expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
    });
    it("packs and unpacks document bundles", () => {
      const bundle = encodeDocumentPackBundle({ nodes: [] });
      expect(decodeDocumentPackSnapshot(bundle)).toEqual({ nodes: [] });
    });
    it("round-trips backbone snapshot messages", () => {
      const message = { kind: "snapshot", pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) };
      const round = decodeBackboneMessage(encodeBackboneMessage(message));
      expect(round.kind).toBe("snapshot");
      if (round.kind !== "snapshot")
        return;
      expect(Array.from(round.pack)).toEqual([1, 2]);
      expect(Array.from(round.spr)).toEqual([3]);
    });
    it("applies a snapshot backbone message by overwriting the stored bundle", () => {
      const snapshot = encodeBackboneMessage({ kind: "snapshot", pack: new Uint8Array([9]), spr: new Uint8Array });
      const result = applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array([1]), new Uint8Array), snapshot);
      expect(decodeDocumentPackBytes(result).pack).toEqual(new Uint8Array([9]));
    });
    it("throws when applying operations without native store", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: [] });
      expect(() => applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array, new Uint8Array), message)).toThrow("native store");
    });
    it("throws when applying operations before a snapshot exists", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: [] });
      expect(() => applyBackboneMessage(null, message)).toThrow("cannot append operations before a snapshot exists");
    });
    it("throws on an unknown backbone message tag", () => {
      expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("unknown tag");
    });
    it("builds sync utilities reflecting the active backbone kind", () => {
      const utilities = buildFrameworkSyncUtilities("folder:///tmp");
      expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
      expect(utilities.find((utility) => utility.id === "framework.sync.folder")?.pressed).toBe(true);
      expect(utilities.find((utility) => utility.id === "framework.sync.file")?.pressed).toBe(false);
    });
  });
  describe("@semio-tech/framework-os workflow", () => {
    const mediaContract = () => ({ kindId: "2d.drawing", mediaType: { class: "data", form: "value" }, wire: { kind: "document", schema: "2d.drawing" } });
    const mediaNode = (id, instanceId) => ({
      id,
      instanceId,
      x: 0,
      y: 0,
      width: 160,
      height: 72,
      inputs: [{ id: `${instanceId}:in`, artifactKind: "2d.drawing", direction: "in" }],
      outputs: [{ id: `${instanceId}:out`, artifactKind: "2d.drawing", direction: "out" }]
    });
    it("plans a single delivery across one dirty edge", () => {
      const graph = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }]
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
    });
    it("plans a chain in topological order when only the root is dirty", () => {
      const graph = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2"), mediaNode("node-3", "app-3")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() },
          { id: "edge-bc", sourceNodeId: "node-2", sourcePortId: "app-2:out", targetNodeId: "node-3", targetPortId: "app-3:in", contract: mediaContract() }
        ]
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries.map((delivery) => delivery.edgeId)).toEqual(["edge-ab", "edge-bc"]);
    });
    it("plans a diamond with one delivery per incoming edge", () => {
      const graph = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-a"), mediaNode("node-2", "app-b"), mediaNode("node-3", "app-c"), mediaNode("node-4", "app-d")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-2", targetPortId: "app-b:in", contract: mediaContract() },
          { id: "edge-ac", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-3", targetPortId: "app-c:in", contract: mediaContract() },
          { id: "edge-bd", sourceNodeId: "node-2", sourcePortId: "app-b:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
          { id: "edge-cd", sourceNodeId: "node-3", sourcePortId: "app-c:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() }
        ]
      };
      const deliveries = planWorkflow(graph, new Set(["app-a"]));
      const edgeIds = deliveries.map((delivery) => delivery.edgeId);
      expect(edgeIds).toHaveLength(4);
      expect(edgeIds.indexOf("edge-bd")).toBeGreaterThan(edgeIds.indexOf("edge-ab"));
      expect(edgeIds.indexOf("edge-cd")).toBeGreaterThan(edgeIds.indexOf("edge-ac"));
    });
    it("plans nothing when no instance is dirty", () => {
      const graph = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }]
      };
      expect(planWorkflow(graph, new Set)).toEqual([]);
    });
    it("plans nothing for a dirty node with no outgoing edges", () => {
      const graph = { schema: "os.workflow", nodes: [mediaNode("node-1", "app-1")], edges: [] };
      expect(planWorkflow(graph, new Set(["app-1"]))).toEqual([]);
    });
    it("matches the Rust plan_workflow across shared fixtures decoded via wasm", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath, pathToFileURL } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const fixturesDir = join2(here, "\uD83E\uDDEB️fixtures");
      const rsPkgDir = join2(here, "\uD83D\uDDA5️host", "\uD83D\uDCE6️packages", "\uD83E\uDD80️rust", "pkg");
      const wasmModule = await import(pathToFileURL(join2(rsPkgDir, "semio_framework_os.js")).href);
      await wasmModule.default({ module_or_path: new Uint8Array(readFileSync(join2(rsPkgDir, "semio_framework_os_bg.wasm"))) });
      const dslFiles = readdirSync(fixturesDir).filter((file) => file.endsWith(".dsl"));
      expect(dslFiles.length).toBeGreaterThanOrEqual(5);
      for (const dslFile of dslFiles) {
        const dslText = readFileSync(join2(fixturesDir, dslFile), "utf8");
        const spkFile = dslFile.replace(/^🗣️?/, "\uD83D\uDCE6️").replace(/\.dsl$/, ".spk");
        const spkBytes = new Uint8Array(readFileSync(join2(fixturesDir, spkFile)));
        const viaDsl = wasmModule.parseWorkflowFixtureDsl(dslText);
        const viaPack = wasmModule.decodeWorkflowFixturePack(spkBytes);
        expect(viaDsl).toEqual(viaPack);
        const deliveries = planWorkflow(viaDsl.graph, new Set(viaDsl.dirtyInstanceIds));
        expect(deliveries).toEqual(viaDsl.expectedDeliveries);
      }
    });
  });
  describe("@semio-tech/framework-os PackValueCodec", () => {
    function bytesToHex(bytes) {
      return Array.from(bytes).map((byte) => byte.toString(16).padStart(2, "0")).join("");
    }
    function hexToBytes(hex) {
      const out = new Uint8Array(hex.length / 2);
      for (let i = 0;i < out.length; i++)
        out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
      return out;
    }
    const packValueFixtures = [
      ["null", null, "0001011112"],
      ["bool_true", true, "0001011102"],
      ["bool_false", false, "0001011101"],
      ["int_zero", 0, "00010111050000000000000000"],
      ["int_negative_one", -1, "0001011105000000000000f0bf"],
      ["float_pi", 3.14, "00010111051f85eb51b81e0940"],
      ["float_whole_number", 2, "00010111050000000000000040"],
      ["string_empty", "", "01000101110600"],
      ["string_escapes", `hello
world with "quotes"`, "011968656c6c6f0a776f726c642077697468202271756f746573220101110600"],
      ["array_empty", [], "000101110c00"],
      ["array_ints", [1, 2, 3], "000101110c0305000000000000f03f050000000000000040050000000000000840"],
      ["object_empty", {}, "000101111000"],
      ["object_mixed", { a: 1, b: [true, null] }, "00010111100207016105000000000000f03f0701620c020212"],
      [
        "nested_deep",
        { a: { b: { c: [1, 2, { d: "leaf" }] } } },
        "01046c6561660101111001070161100107016210010701630c0305000000000000f03f05000000000000004010010701640600"
      ]
    ];
    it.each(packValueFixtures)("decodes real Rust encode_wire_value bytes for %s", (_name, expected, hex) => {
      expect(decodePackValue(hexToBytes(hex))).toEqual(expected);
    });
    it.each(packValueFixtures)("encodes byte-exact against real Rust encode_wire_value output for %s", (_name, value, hex) => {
      expect(bytesToHex(encodePackValue(value))).toBe(hex);
    });
    it.each(packValueFixtures)("round-trips %s through encodePackValue/decodePackValue", (_name, value) => {
      expect(decodePackValue(encodePackValue(value))).toEqual(value);
    });
  });
  describe("@semio-tech/framework-os AppChannelCodec", () => {
    it("round-trips the app-typed presence pack through the document-presence wire", () => {
      const peer = {
        actor: "actor-1",
        connectedAtMs: 42,
        label: "One",
        presencePack: [1, 2, 3],
        color: 4,
        surface: "s.space.home@1/*#editor",
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 1, y: 2, zoom: 1.5 }, size: [800, 600], pointer: [10, 20, 0] }],
        ui: { hoveredPath: "row[0]#a" }
      };
      expect(decodePresencePeer(new Uint8Array(encodePresencePeer(peer)), [0])).toEqual(peer);
    });
    const sampleCommands = [
      { ConfigCommand: { seq: 1, command: [4, 5] } },
      { Command: { seq: 2, command: [1], view_state: [2, 3] } },
      { CommandText: { seq: 3, line: "move 1 2" } },
      { ContextMenu: { seq: 5, request: [9, 9] } },
      { ArtifactCommand: { seq: 6, command: [7] } },
      { ApplyEnvelopes: { seq: 7, envelopes: [] } },
      { LoadDocument: { seq: 8, pack: [1, 2, 3], spr: [4, 5, 6] } },
      { ReadDocument: { seq: 9 } },
      { LoadConfig: { seq: 10, pack: [1], spr: [2] } },
      { ReadConfig: { seq: 11 } },
      { MediaIn: { seq: 14, port: "in-1", descriptor: [1], data: [2, 3] } },
      { MediaOut: { seq: 15, port: "out-1", request: [4] } },
      { MediaFingerprint: { seq: 16, port: "fp-1" } },
      { PureCommand: { seq: 17, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } },
      { LoadChildren: { seq: 18, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { ReadChildren: { seq: 19 } },
      { ReadHistory: { seq: 20 } },
      { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], label: "", origin: [] } },
      { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "step-1", origin: [9] } },
      { transactionCommit: { seq: 23, txn_id: "txn-1" } },
      { transactionRollback: { seq: 24, txn_id: "txn-1" } },
      { transactionUndo: { seq: 25, group_id: "grp-1" } },
      { transactionRedo: { seq: 26, group_id: "grp-1" } },
      { openArtifact: { seq: 27, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
      { openArtifact: { seq: 28, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { setDefaultApp: { seq: 29, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { clearDefaultApp: { seq: 30, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
      { setMergePolicy: { seq: 31, policy: 1 } },
      { resolveConflict: { seq: 32, conflict_id: "conflict-1", resolution: 0 } },
      { readConflicts: { seq: 33 } },
      { presence: { seq: 34, own_color: 3, peers: [[1, 2], [9]] } },
      { presence: { seq: 35, own_color: null, peers: [] } }
    ];
    const sampleFrames = [
      { Done: { in_reply_to: 1 } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
      { DocumentChanged: { envelopes: [[1, 2]], origin: "remote" } },
      { Document: { in_reply_to: 6, pack: [1, 2], spr: [3, 4], ops: "op-log" } },
      { ContextMenu: { in_reply_to: 7, items: [1, 2, 3] } },
      { Media: { in_reply_to: 8, port: "out-1", descriptor: [1], data: [2] } },
      { MediaFingerprint: { in_reply_to: 9, port: "fp-1", fingerprint: [1, 2, 3, 4] } },
      { Error: { in_reply_to: 10, fault: [1, 2, 3], report: [6] } },
      { Error: { in_reply_to: null, fault: [4, 5], report: [] } },
      { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5] } },
      { Draft: { in_reply_to: 12, pack: [1], spr: [2], ops: "d" } },
      { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } },
      { HistorySnapshot: { in_reply_to: 14, history_patch: [1] } },
      { transactionProposal: { in_reply_to: 15, proposal_id: "prop-1", local_ops: [[1]], description: "move", coalesce_key: "k-1", foreign: [[2, 3]] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [[1]], rejection: [] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [], rejection: [1, 2] } },
      { transactionCommitted: { txn_id: "txn-1", edit_id: "edit-1" } },
      { transactionRolledBack: { txn_id: "txn-1" } },
      { MergeReport: { in_reply_to: 16, report: [1, 2] } },
      { MergeReport: { in_reply_to: null, report: [] } },
      { Conflicts: { in_reply_to: 17, conflicts: [3] } },
      { Conflicts: { in_reply_to: null, conflicts: [] } },
      { UiPatch: { in_reply_to: 18, surface: "1:body", kind: "window", revision: 2, base_revision: 1, ops: [3] } },
      { UiPatch: { in_reply_to: null, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
      { UiSnapshotEnd: { revision: 5 } }
    ];
    it.each(sampleCommands.map((cmd) => [cmd]))("round-trips AppCommand %j", (cmd) => {
      expect(decodeAppCommand(encodeAppCommand(cmd))).toEqual(cmd);
    });
    it.each(sampleFrames.map((frame) => [frame]))("round-trips AppFrame %j", (frame) => {
      expect(decodeAppFrame(encodeAppFrame(frame))).toEqual(frame);
    });
    it("tags every AppCommand variant per the agreed contract order (ConfigCommand=0 ... presence=28)", () => {
      expect(encodeAppCommand({ ConfigCommand: { seq: 0, command: [] } })[0]).toBe(0);
      expect(encodeAppCommand({ Command: { seq: 0, command: [], view_state: [] } })[0]).toBe(1);
      expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);
      expect(encodeAppCommand({ ReadHistory: { seq: 0 } })[0]).toBe(16);
      expect(encodeAppCommand({ transactionPrepare: { seq: 0, txn_id: "", mutation_id: "", payload: [], prepared_ops: [], label: "", origin: [] } })[0]).toBe(17);
      expect(encodeAppCommand({ transactionCommit: { seq: 0, txn_id: "" } })[0]).toBe(18);
      expect(encodeAppCommand({ transactionRollback: { seq: 0, txn_id: "" } })[0]).toBe(19);
      expect(encodeAppCommand({ transactionUndo: { seq: 0, group_id: "" } })[0]).toBe(20);
      expect(encodeAppCommand({ transactionRedo: { seq: 0, group_id: "" } })[0]).toBe(21);
      expect(encodeAppCommand({ openArtifact: { seq: 0, artifact_ref: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(22);
      expect(encodeAppCommand({ setDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(23);
      expect(encodeAppCommand({ clearDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0 } })[0]).toBe(24);
      expect(encodeAppCommand({ setMergePolicy: { seq: 0, policy: 0 } })[0]).toBe(25);
      expect(encodeAppCommand({ resolveConflict: { seq: 0, conflict_id: "", resolution: 0 } })[0]).toBe(26);
      expect(encodeAppCommand({ readConflicts: { seq: 0 } })[0]).toBe(27);
      expect(encodeAppCommand({ presence: { seq: 0, own_color: null, peers: [] } })[0]).toBe(28);
    });
    it("tags every AppFrame variant per the agreed contract order (Done=0 ... UiSnapshotEnd=22)", () => {
      expect(encodeAppFrame({ Done: { in_reply_to: 0 } })[0]).toBe(0);
      expect(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } })[0]).toBe(1);
      expect(encodeAppFrame({ Error: { in_reply_to: null, fault: [], report: [] } })[0]).toBe(9);
      expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [] } })[0]).toBe(13);
      expect(encodeAppFrame({ HistorySnapshot: { in_reply_to: 0, history_patch: [] } })[0]).toBe(14);
      expect(encodeAppFrame({ transactionProposal: { in_reply_to: 0, proposal_id: "", local_ops: [], description: "", coalesce_key: "", foreign: [] } })[0]).toBe(15);
      expect(encodeAppFrame({ transactionPrepared: { txn_id: "", foreign: [], rejection: [] } })[0]).toBe(16);
      expect(encodeAppFrame({ transactionCommitted: { txn_id: "", edit_id: "" } })[0]).toBe(17);
      expect(encodeAppFrame({ transactionRolledBack: { txn_id: "" } })[0]).toBe(18);
      expect(encodeAppFrame({ MergeReport: { in_reply_to: null, report: [] } })[0]).toBe(19);
      expect(encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: [] } })[0]).toBe(20);
      expect(encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "", kind: "", revision: 0, base_revision: 0, ops: [] } })[0]).toBe(21);
      expect(encodeAppFrame({ UiSnapshotEnd: { revision: 0 } })[0]).toBe(22);
    });
    it("matches protocol_channel's own golden hex fixture corpus, byte-for-byte", () => {
      const commandFixtures = [
        ["ConfigCommand", { ConfigCommand: { seq: 1, command: [9] } }],
        ["Command", { Command: { seq: 1, command: [1], view_state: [] } }],
        ["CommandText", { CommandText: { seq: 1, line: "go" } }],
        ["ContextMenu", { ContextMenu: { seq: 1, request: [1] } }],
        ["ArtifactCommand", { ArtifactCommand: { seq: 1, command: [1] } }],
        ["ApplyEnvelopes", { ApplyEnvelopes: { seq: 1, envelopes: [] } }],
        ["LoadDocument", { LoadDocument: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadDocument", { ReadDocument: { seq: 1 } }],
        ["LoadConfig", { LoadConfig: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadConfig", { ReadConfig: { seq: 1 } }],
        ["MediaIn", { MediaIn: { seq: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaOut", { MediaOut: { seq: 1, port: "p", request: [1] } }],
        ["MediaFingerprint", { MediaFingerprint: { seq: 1, port: "p" } }],
        ["PureCommand", { PureCommand: { seq: 1, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } }],
        ["LoadChildren", { LoadChildren: { seq: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["ReadChildren", { ReadChildren: { seq: 1 } }],
        ["ReadHistory", { ReadHistory: { seq: 1 } }]
      ];
      const commandGoldenHex = {
        ConfigCommand: "00010109",
        Command: "0101010100",
        CommandText: "020102676f",
        ContextMenu: "03010101",
        ArtifactCommand: "04010101",
        ApplyEnvelopes: "050100",
        LoadDocument: "060101010102",
        ReadDocument: "0701",
        LoadConfig: "080101010102",
        ReadConfig: "0901",
        MediaIn: "0a01017001010102",
        MediaOut: "0b0101700101",
        MediaFingerprint: "0c010170",
        PureCommand: "0d010101010201030104010501060107",
        LoadChildren: "0e01010173016301640101",
        ReadChildren: "0f01",
        ReadHistory: "1001"
      };
      const frameFixtures = [
        ["Done", { Done: { in_reply_to: 1 } }],
        ["Invocation", { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } }],
        ["DocumentChanged", { DocumentChanged: { envelopes: [], origin: "o" } }],
        ["Document", { Document: { in_reply_to: 1, pack: [1], spr: [2], ops: "o" } }],
        ["Config", { Config: { in_reply_to: 1, pack: [1], spr: [2], ops: "c" } }],
        ["ConfigChanged", { ConfigChanged: { envelopes: [], origin: "o" } }],
        ["ContextMenu", { ContextMenu: { in_reply_to: 1, items: [1] } }],
        ["Media", { Media: { in_reply_to: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaFingerprint", { MediaFingerprint: { in_reply_to: 1, port: "p", fingerprint: [1] } }],
        ["Error", { Error: { in_reply_to: null, fault: [99], report: [] } }],
        ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [] } }],
        ["Draft", { Draft: { in_reply_to: 1, pack: [1], spr: [2], ops: "d" } }],
        ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["Ephemeral", { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } }],
        ["HistorySnapshot", { HistorySnapshot: { in_reply_to: 1, history_patch: [1] } }],
        ["UiPatch", { UiPatch: { in_reply_to: 1, surface: "1:body", kind: "window", revision: 3, base_revision: 2, ops: [9] } }],
        ["UiSnapshotEnd", { UiSnapshotEnd: { revision: 6 } }]
      ];
      const frameGoldenHex = {
        Done: "0001",
        Invocation: "0101010100000000",
        DocumentChanged: "0200016f",
        Document: "030101010102016f",
        Config: "0401010101020163",
        ConfigChanged: "0500016f",
        ContextMenu: "06010101",
        Media: "0701017001010102",
        MediaFingerprint: "080101700101",
        Error: "0900016300",
        Emit: "0a0101010000010200",
        Draft: "0b01010101020164",
        Children: "0c01010173016301640101",
        Ephemeral: "0d020102030400",
        HistorySnapshot: "0e010101",
        UiPatch: "15010106313a626f64790677696e646f7703020109",
        UiSnapshotEnd: "1606"
      };
      const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      for (const [label, value] of commandFixtures)
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandGoldenHex[label]);
      for (const [label, value] of frameFixtures)
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameGoldenHex[label]);
    });
    it("pins APP_CHANNEL_VERSION against the shared cross-language channel version", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const pin = JSON.parse(readFileSync(join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCE1️channel", "channel-version.json"), "utf8"));
      expect(APP_CHANNEL_VERSION).toBe(pin.channelVersion);
    });
    it("matches the shared cross-language transaction fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const channelFixturesDir = join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCE1️channel");
      const commandVectors = JSON.parse(readFileSync(join2(channelFixturesDir, "app-command-transaction.json"), "utf8"));
      const frameVectors = JSON.parse(readFileSync(join2(channelFixturesDir, "app-frame-transaction.json"), "utf8"));
      const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      const commandCases = {
        TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], label: "", origin: [] } },
        TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "l", origin: [9] } },
        TransactionCommit: { transactionCommit: { seq: 3, txn_id: "t" } },
        TransactionRollback: { transactionRollback: { seq: 4, txn_id: "t" } },
        TransactionUndo: { transactionUndo: { seq: 5, group_id: "g" } },
        TransactionRedo: { transactionRedo: { seq: 6, group_id: "g" } }
      };
      const frameCases = {
        TransactionProposal: { transactionProposal: { in_reply_to: 1, proposal_id: "p", local_ops: [[1]], description: "d", coalesce_key: "k", foreign: [] } },
        TransactionPrepared: { transactionPrepared: { txn_id: "t", foreign: [[1]], rejection: [] } },
        TransactionCommitted: { transactionCommitted: { txn_id: "t", edit_id: "e" } },
        TransactionRolledBack: { transactionRolledBack: { txn_id: "t" } }
      };
      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label], "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label], "hex")))).toEqual(value);
      }
    });
    it("matches the shared cross-language opening fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const channelFixturesDir = join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCE1️channel");
      const commandVectors = JSON.parse(readFileSync(join2(channelFixturesDir, "app-command-opening.json"), "utf8"));
      const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      const commandCases = {
        OpenArtifactResolve: { openArtifact: { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
        OpenArtifactExplicit: { openArtifact: { seq: 2, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        SetDefaultApp: { setDefaultApp: { seq: 3, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        ClearDefaultApp: { clearDefaultApp: { seq: 4, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } }
      };
      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label], "hex")))).toEqual(value);
      }
    });
    it("matches the shared cross-language merge fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const channelFixturesDir = join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCE1️channel");
      const commandVectors = JSON.parse(readFileSync(join2(channelFixturesDir, "app-command-merge.json"), "utf8"));
      const frameVectors = JSON.parse(readFileSync(join2(channelFixturesDir, "app-frame-merge.json"), "utf8"));
      const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      const commandCases = {
        SetMergePolicy: { setMergePolicy: { seq: 5, policy: 1 } },
        ResolveConflict: { resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } },
        ReadConflicts: { readConflicts: { seq: 7 } }
      };
      const frameCases = {
        MergeReport: { MergeReport: { in_reply_to: 1, report: [1] } },
        Conflicts: { Conflicts: { in_reply_to: null, conflicts: [2] } },
        Invocation: { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
        Error: { Error: { in_reply_to: null, fault: [99], report: [7] } }
      };
      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label], "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label], "hex")))).toEqual(value);
      }
    });
  });
  describe("@semio-tech/framework-os AppChannelClient", () => {
    function fakeHandle(reply) {
      const broadcast = createTurnOutcomeBroadcast();
      return {
        enqueue: (instanceId, events) => {
          const commands = events.map(decodeAppCommand);
          const frames = reply(instanceId, commands).map(encodeAppFrame);
          broadcast.push({ instanceId, frames });
        },
        outcomes: broadcast.stream
      };
    }
    it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
      const seqsSeen = [];
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "Command" in cmd)
          seqsSeen.push(cmd.Command.seq);
        return [
          { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } },
          { UiPatch: { in_reply_to: seqsSeen.at(-1) ?? 0, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } }
        ];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
      const second = await client.command(new Uint8Array([3]), { cursor: 1 });
      expect(seqsSeen).toEqual([1, 2]);
      expect(first).toHaveLength(2);
      expect(second).toHaveLength(2);
    });
    it("configure()/readDocument()/loadDocument() frame the right AppCommand variant", async () => {
      const seen = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      await client.configure({ locale: "en" });
      await client.readDocument();
      await client.loadDocument(new Uint8Array([1]), new Uint8Array([2]));
      expect(seen[0]).toEqual({ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } });
      expect(seen[1]).toEqual({ ReadDocument: { seq: 2 } });
      expect(seen[2]).toEqual({ LoadDocument: { seq: 3, pack: [1], spr: [2] } });
    });
    it("caches the document pack from loadDocument()'s own arguments before any reply arrives", async () => {
      const handle = fakeHandle(() => [{ Done: { in_reply_to: 1 } }]);
      const client = new AppChannelClient(handle, 1, "app.demo");
      expect(client.documentPack()).toBeNull();
      await client.loadDocument(new Uint8Array([1, 2]), new Uint8Array([3]));
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
    });
    it("caches the document pack from every AppFrame::Document reply, most recent wins", async () => {
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "ReadDocument" in cmd) {
          return [{ Document: { in_reply_to: cmd.ReadDocument.seq, pack: [9, 9], spr: [8], ops: "" } }];
        }
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      await client.readDocument();
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
    });
    it("transactionPrepareOwner()/transactionPreparePlanned()/transactionCommit()/transactionRollback()/transactionUndo()/transactionRedo() frame the right AppCommand variant", async () => {
      const seen = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      await client.transactionPrepareOwner("txn-1", "s.doc#kind", new Uint8Array([1]));
      await client.transactionPreparePlanned("txn-1", [new Uint8Array([2]), new Uint8Array([3])], "duplicate", new Uint8Array([4]));
      await client.transactionCommit("txn-1");
      await client.transactionRollback("txn-1");
      await client.transactionUndo("grp-1");
      await client.transactionRedo("grp-1");
      expect(seen[0]).toEqual({ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.doc#kind", payload: [1], prepared_ops: [], label: "", origin: [] } });
      expect(seen[1]).toEqual({ transactionPrepare: { seq: 2, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[2], [3]], label: "duplicate", origin: [4] } });
      expect(seen[2]).toEqual({ transactionCommit: { seq: 3, txn_id: "txn-1" } });
      expect(seen[3]).toEqual({ transactionRollback: { seq: 4, txn_id: "txn-1" } });
      expect(seen[4]).toEqual({ transactionUndo: { seq: 5, group_id: "grp-1" } });
      expect(seen[5]).toEqual({ transactionRedo: { seq: 6, group_id: "grp-1" } });
    });
    it("setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const commandVectors = JSON.parse(readFileSync(join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCE1️channel", "app-command-merge.json"), "utf8"));
      const hex = (bytes) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      const seen = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.setMergePolicy("Normal");
      await client.resolveConflict("conflict-1", "accept");
      await client.readConflicts();
      expect(seen[4]).toEqual({ setMergePolicy: { seq: 5, policy: 1 } });
      expect(seen[5]).toEqual({ resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } });
      expect(seen[6]).toEqual({ readConflicts: { seq: 7 } });
      expect(hex(encodeAppCommand(seen[4])), "AppCommand::SetMergePolicy").toBe(commandVectors.SetMergePolicy);
      expect(hex(encodeAppCommand(seen[5])), "AppCommand::ResolveConflict").toBe(commandVectors.ResolveConflict);
      expect(hex(encodeAppCommand(seen[6])), "AppCommand::ReadConflicts").toBe(commandVectors.ReadConflicts);
    });
    it("command() surfaces unsolicited MergeReport/Conflicts frames and the extended Invocation.messages/Error.report fields verbatim", async () => {
      const handle = fakeHandle(() => [
        { Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
        { MergeReport: { in_reply_to: null, report: [1] } },
        { Conflicts: { in_reply_to: null, conflicts: [2] } }
      ]);
      const client = new AppChannelClient(handle, 1, "app.demo");
      const frames = await client.command(new Uint8Array([1]), {});
      expect(frames).toHaveLength(3);
      const invocation = frames.find((frame) => ("Invocation" in frame));
      const mergeReport = frames.find((frame) => ("MergeReport" in frame));
      const conflicts = frames.find((frame) => ("Conflicts" in frame));
      expect(invocation?.Invocation.messages).toEqual([9]);
      expect(mergeReport?.MergeReport.report).toEqual([1]);
      expect(conflicts?.Conflicts.conflicts).toEqual([2]);
    });
    it("faultMessages()/decodeDispatchReportFromWire()/decodeMergeReportFromWire()/decodeConflictsFromWire() decode the frozen TS report shapes", () => {
      const dispatchReport = {
        policy: "Vigilant",
        worst: "warning",
        messages: [{ level: "warning", code: "mutation.clamped", message: "value clamped to range" }]
      };
      const reportBytes = Array.from(encodePackValue(dispatchReport));
      expect(decodeDispatchReportFromWire(reportBytes, decodePackValue)).toEqual(dispatchReport);
      expect(faultMessages(reportBytes, decodePackValue)).toEqual(dispatchReport.messages);
      expect(faultMessages([], decodePackValue)).toEqual([]);
      const mergeReport = {
        policy: "Normal",
        accepted: true,
        insertionIndex: 3,
        replayed: [{ edit_id: "e1", messages: [{ level: "info", code: "mutation.cascade", message: "cascaded" }] }],
        worst: "info",
        conflict: null
      };
      expect(decodeMergeReportFromWire(Array.from(encodePackValue(mergeReport)), decodePackValue)).toEqual(mergeReport);
      expect(decodeMergeReportFromWire([], decodePackValue)).toBeNull();
      const conflicts = [
        {
          id: "conflict-abc",
          kind: { kind: "degraded", edit_ids: ["e1"] },
          status: "open",
          messages: [{ level: "error", code: "mutation.target-missing", message: "target missing" }],
          actors: ["actor-1"],
          timestamp: { actor: 1, physical_ms: 100, logical: 0 }
        }
      ];
      expect(decodeConflictsFromWire(Array.from(encodePackValue(conflicts)), decodePackValue)).toEqual(conflicts);
      expect(decodeConflictsFromWire([], decodePackValue)).toEqual([]);
    });
  });
  describe("@semio-tech/framework PluginGraph", () => {
    it("validates a graph with every dependency present and version-satisfying", async () => {
      const { validatePluginDependencyGraph: validatePluginDependencyGraph2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      expect(validatePluginDependencyGraph2([
        { pluginId: "a", version: "1.2.3" },
        { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "^1.0.0" }] }
      ])).toEqual([]);
    });
    it("reports a missing dependency", async () => {
      const { validatePluginDependencyGraph: validatePluginDependencyGraph2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      expect(validatePluginDependencyGraph2([{ pluginId: "b", dependencies: [{ pluginId: "missing", version: "*" }] }])).toEqual([
        { code: "transaction.dependency-missing", pluginId: "b", dependsOn: "missing" }
      ]);
    });
    it("reports a version mismatch", async () => {
      const { validatePluginDependencyGraph: validatePluginDependencyGraph2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      expect(validatePluginDependencyGraph2([
        { pluginId: "a", version: "2.0.0" },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "^1.0.0" }] }
      ])).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "^1.0.0", actual: "2.0.0" }]);
    });
    it("resolves a diamond load order deterministically, tie-broken lexicographically", async () => {
      const { resolvePluginLoadOrder: resolvePluginLoadOrder2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const result = resolvePluginLoadOrder2([
        {
          pluginId: "d",
          dependencies: [
            { pluginId: "b", version: "*" },
            { pluginId: "c", version: "*" }
          ]
        },
        { pluginId: "c", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a" }
      ]);
      expect(result.errors).toEqual([]);
      expect(result.order).toEqual(["a", "b", "c", "d"]);
    });
    it("names every member of a cycle", async () => {
      const { resolvePluginLoadOrder: resolvePluginLoadOrder2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const result = resolvePluginLoadOrder2([
        { pluginId: "a", dependencies: [{ pluginId: "b", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] }
      ]);
      expect(result.order).toEqual([]);
      expect(result.errors).toEqual([{ code: "transaction.cycle", members: ["a", "b"] }]);
    });
    it("versionSatisfies matches the frozen grammar (*, =, ^, ~, >=), including caret's leading-zero tiers", async () => {
      const { versionSatisfies: versionSatisfies2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      expect(versionSatisfies2("1.2.3", "*")).toBe(true);
      expect(versionSatisfies2("1.2.3", "=1.2.3")).toBe(true);
      expect(versionSatisfies2("1.2.4", "=1.2.3")).toBe(false);
      expect(versionSatisfies2("1.9.0", "^1.2.3")).toBe(true);
      expect(versionSatisfies2("2.0.0", "^1.2.3")).toBe(false);
      expect(versionSatisfies2("0.2.9", "^0.2.3")).toBe(true);
      expect(versionSatisfies2("0.3.0", "^0.2.3")).toBe(false);
      expect(versionSatisfies2("0.0.9", "^0.0.3")).toBe(false);
      expect(versionSatisfies2("0.0.3", "^0.0.3")).toBe(true);
      expect(versionSatisfies2("1.2.9", "~1.2.3")).toBe(true);
      expect(versionSatisfies2("1.3.0", "~1.2.3")).toBe(false);
      expect(versionSatisfies2("1.2.3", ">=1.2.3")).toBe(true);
      expect(versionSatisfies2("9.9.9", ">=1.2.3")).toBe(true);
      expect(versionSatisfies2("1.2.2", ">=1.2.3")).toBe(false);
    });
    it("orderPluginRegistryEntries drops only the blocked entries, dependency-orders the rest", async () => {
      const { orderPluginRegistryEntries: orderPluginRegistryEntries2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const result = orderPluginRegistryEntries2([
        { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a", moduleUrl: "a.js" },
        { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing", version: "*" }] }
      ]);
      expect(result.order.map((entry) => entry.pluginId)).toEqual(["a", "b"]);
      expect(result.errors).toEqual([{ code: "transaction.dependency-missing", pluginId: "broken", dependsOn: "missing" }]);
    });
    it("pluginGraphErrorMessage renders a real English and a real German message", async () => {
      const { pluginGraphErrorMessage: pluginGraphErrorMessage2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const error = { code: "transaction.dependency-missing", pluginId: "b", dependsOn: "a" };
      expect(pluginGraphErrorMessage2(error, "en")).toContain("needs");
      expect(pluginGraphErrorMessage2(error, "de")).toContain("benötigt");
    });
    it("PluginGraph.canUnload refuses while a loaded dependent exists, allows once it's gone", async () => {
      const { PluginGraph: PluginGraph2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const graph = new PluginGraph2([{ pluginId: "a" }, { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] }]);
      expect(graph.canUnload("a", new Set(["a", "b"]))).toBe(false);
      expect(graph.canUnload("a", new Set(["a"]))).toBe(true);
    });
  });
  describe("@semio-tech/framework InstanceDirectory and ArtifactRouters", () => {
    it("InstanceDirectory registers, resolves, and unregisters", async () => {
      const { InstanceDirectory: InstanceDirectory2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const directory = new InstanceDirectory2;
      directory.register("artifact-1", { pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      expect(directory.resolve("artifact-1")).toEqual({ pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      directory.unregister("artifact-1");
      expect(directory.resolve("artifact-1")).toBeUndefined();
    });
    it("ArtifactMutationRouter accepts a byte-identical re-registration, rejects a conflicting one", async () => {
      const { ArtifactMutationRouter: ArtifactMutationRouter2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const router = new ArtifactMutationRouter2;
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      expect(router.resolve("s.cad.model", "s.cad#add-wall")).toEqual({ kind: "owner" });
      expect(() => router.registerContributed("s.cad.model", "aec-building", "cad", { mutationId: "s.cad#add-wall", semantics: { verb: "add", entity: "wall", kind: "add-wall", record: "Wall" }, schemaVersion: 1, algorithmVersion: 1 }, true)).toThrow(/conflict/);
    });
    it("ArtifactMutationRouter.registerContributed rejects a contributor that doesn't depend on the owner", async () => {
      const { ArtifactMutationRouter: ArtifactMutationRouter2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const router = new ArtifactMutationRouter2;
      expect(() => router.registerContributed("s.cad.model", "aec-building", "cad", { mutationId: "s.cad#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 }, false)).toThrow(/not a direct dependency/);
    });
    it("ArtifactInferenceRouter enforces owner === contributor and orders the depends_on DAG", async () => {
      const { ArtifactInferenceRouter: ArtifactInferenceRouter2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const router = new ArtifactInferenceRouter2;
      router.registerContributed("s.cad.model", {
        owner: "aec-building",
        artifactKind: "s.cad.model",
        artifactSchema: "s.cad.model",
        artifactSchemaVersion: 1,
        documentSchema: "s.cad",
        documentSchemaVersion: 1,
        inferenceSchema: "s.aec-building.load-path",
        inferenceSchemaVersion: 1,
        algorithmVersion: 1,
        policyVersion: 1,
        contributor: "aec-building",
        dependsOn: []
      }, true);
      expect(router.resolve("s.cad.model", "s.aec-building.load-path")).toEqual({ kind: "contributed", pluginId: "aec-building" });
      expect(router.dependencyOrder()).toEqual(["s.cad.model s.aec-building.load-path"]);
    });
    it("ArtifactInferenceRouter.registerContributed rejects owner !== contributor", async () => {
      const { ArtifactInferenceRouter: ArtifactInferenceRouter2 } = await Promise.resolve().then(() => (init__glue(), exports__glue));
      const router = new ArtifactInferenceRouter2;
      expect(() => router.registerContributed("s.cad.model", {
        owner: "someone-else",
        artifactKind: "s.cad.model",
        artifactSchema: "s.cad.model",
        artifactSchemaVersion: 1,
        documentSchema: "s.cad",
        documentSchemaVersion: 1,
        inferenceSchema: "s.aec-building.load-path",
        inferenceSchemaVersion: 1,
        algorithmVersion: 1,
        policyVersion: 1,
        contributor: "aec-building",
        dependsOn: []
      }, true)).toThrow(/owner\/contributor mismatch/);
    });
  });
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("@semio-tech/framework-os directory", () => {
    const loadFixtureEvents = async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await Promise.resolve().then(() => (init_url(), exports_url));
      const { dirname: dirname2, join: join2 } = await Promise.resolve().then(() => (init_path(), exports_path));
      const here = dirname2(fileURLToPath(import.meta.url));
      const raw = readFileSync(join2(here, "\uD83E\uDDEB️fixtures", "\uD83D\uDCC7️directory", "\uD83E\uDDFE️events.json"), "utf8");
      return JSON.parse(raw).events;
    };
    it("folds the golden fixture into the expected projection (parity with the Rust twin)", async () => {
      const events = await loadFixtureEvents();
      const model = foldAll(emptyDirectoryReadModel(), events);
      expect(model.cursor).toBe(16);
      expect(model.spaces.size).toBe(1);
      expect(model.spaces.has("sp-atelier-amara")).toBe(false);
      const studio = model.spaces.get("sp-studio-fabrication");
      expect(studio?.view.name).toBe("Fabrication Studio");
      expect(studio?.view.visibility).toBe("public");
      expect(studio?.view.kind).toBe("archive");
      expect(studio?.view.memberCount).toBe(2);
      const roles = (studio?.members ?? []).map((member) => [member.userId, member.role]).sort();
      expect(roles).toEqual([
        ["u-amara", "spectator"],
        ["u-devon", "spectator"]
      ]);
      const devon = studio?.members.find((member) => member.userId === "u-devon");
      expect(devon?.email).toBe("devon@semio.dev");
    });
    it("is idempotent on replay", async () => {
      const events = await loadFixtureEvents();
      const once = foldAll(emptyDirectoryReadModel(), events);
      const twice = foldAll(once, events);
      expect(twice).toEqual(once);
    });
  });
}
var HUB_RECONNECT_MIN_MS = 500;
var HUB_RECONNECT_MAX_MS = 30000;
var HUB_HEALTHY_RESET_MS = HUB_RECONNECT_MAX_MS;

class DirectoryHttpError extends Error {
  status;
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}
var DIRECTORY_HTTP_TIMEOUT_MS = 1e4;

class DirectoryClient {
  baseUrl;
  token;
  constructor(baseUrl, token) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.token = token;
  }
  headers(json) {
    const headers = {};
    if (json)
      headers["content-type"] = "application/json";
    if (this.token)
      headers.authorization = `Bearer ${this.token}`;
    return headers;
  }
  async getJson(path, options) {
    const response = await fetchWithTimeout(`${this.baseUrl}${path}`, { headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok)
      throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    return await response.json();
  }
  async postJson(path, body, options) {
    const response = await fetchWithTimeout(`${this.baseUrl}${path}`, { method: "POST", headers: this.headers(true), body: JSON.stringify(body) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok)
      throw new DirectoryHttpError(response.status, `directory: POST ${path} failed (${response.status})`);
    return await response.json();
  }
  async mintSession(email, options) {
    const body = await this.postJson("/auth/sessions", { email }, options);
    this.token = body.token;
    return { token: body.token, userId: body.user_id };
  }
  async me(options) {
    try {
      return await this.getJson("/auth/sessions/me", options);
    } catch (error) {
      if (error instanceof DirectoryHttpError && error.status === 401)
        return null;
      throw error;
    }
  }
  async spaces(options) {
    return this.getJson("/directory/spaces", options);
  }
  async space(id, options) {
    return this.getJson(`/directory/spaces/${encodeURIComponent(id)}`, options);
  }
  async command(command, options) {
    return this.postJson("/directory/commands", command, options);
  }
  async events(since, options) {
    return this.getJson(`/directory/events?since=${encodeURIComponent(String(since))}`, options);
  }
  stream(since, onMessage) {
    const abort = new AbortController;
    let socket = null;
    let lastSeq = since;
    let healthy = false;
    const wsUrl = () => {
      const wsBase = this.baseUrl.replace(/^http/, "ws");
      const query = new URLSearchParams;
      if (this.token)
        query.set("token", this.token);
      query.set("since", String(lastSeq));
      return `${wsBase}/directory/ws?${query.toString()}`;
    };
    const connectOnce = () => new Promise((resolve2, reject) => {
      if (abort.signal.aborted) {
        reject(abort.signal.reason ?? new Error("directory stream: closed"));
        return;
      }
      let ws;
      try {
        ws = new WebSocket(wsUrl());
      } catch (error) {
        reject(error);
        return;
      }
      socket = ws;
      const onAbort = () => ws.close();
      abort.signal.addEventListener("abort", onAbort, { once: true });
      let healthyTimer = null;
      ws.onopen = () => {
        healthyTimer = setTimeout(() => {
          healthy = true;
        }, HUB_HEALTHY_RESET_MS);
      };
      ws.onmessage = (event) => {
        try {
          const data = event.data;
          const message = JSON.parse(String(data));
          if (message.kind === "event")
            lastSeq = Math.max(lastSeq, message.event.seq);
          if (message.kind === "heartbeat")
            lastSeq = Math.max(lastSeq, message.headSeq);
          onMessage(message);
        } catch {}
      };
      ws.onclose = () => {
        abort.signal.removeEventListener("abort", onAbort);
        if (healthyTimer != null)
          clearTimeout(healthyTimer);
        if (socket === ws)
          socket = null;
        if (abort.signal.aborted || healthy) {
          resolve2();
          return;
        }
        reject(new Error("directory stream: socket closed"));
      };
      ws.onerror = () => {
        try {
          ws.close();
        } catch {}
      };
    });
    async function runCycles() {
      let primeNextCycle = false;
      for (;; ) {
        healthy = false;
        let primed = !primeNextCycle;
        const fn = () => {
          if (!primed) {
            primed = true;
            return Promise.reject(new Error("directory stream: healthy-reset pause"));
          }
          return connectOnce();
        };
        try {
          await retryWithJitteredBackoff(fn, { minMs: HUB_RECONNECT_MIN_MS, maxMs: HUB_RECONNECT_MAX_MS, signal: abort.signal });
        } catch {
          return;
        }
        if (abort.signal.aborted)
          return;
        primeNextCycle = healthy;
      }
    }
    runCycles();
    return {
      close: () => {
        abort.abort();
        socket?.close();
      }
    };
  }
}
if (import.meta.vitest) {
  let sampleDirectoryEvent = function(seq) {
    return {
      seq,
      id: `evt-${seq}`,
      hlc: { physicalMs: seq, logical: 0 },
      actor: { kind: "user", id: "u-1" },
      body: { kind: "space.renamed", spaceId: "sp-1", name: `space ${seq}` },
      recordedAtMs: seq
    };
  };
  const { describe, expect, it, vi } = import.meta.vitest;

  class FakeDirectoryWebSocket {
    static instances = [];
    url;
    readyState = 0;
    onopen = null;
    onmessage = null;
    onclose = null;
    onerror = null;
    constructor(url) {
      this.url = url;
      FakeDirectoryWebSocket.instances.push(this);
    }
    send() {}
    close() {
      this.readyState = 3;
    }
    triggerOpen() {
      this.readyState = 1;
      this.onopen?.();
    }
    triggerMessage(message) {
      this.onmessage?.({ data: JSON.stringify(message) });
    }
    triggerClose() {
      this.readyState = 3;
      this.onclose?.();
    }
  }
  describe("DirectoryClient.stream", () => {
    it("replays then goes live with no gap and no duplicate", () => {
      FakeDirectoryWebSocket.instances = [];
      globalThis.WebSocket = FakeDirectoryWebSocket;
      const received = [];
      const client = new DirectoryClient("http://hub.test", "tok-1");
      const handle = client.stream(0, (message) => received.push(message));
      const socket = FakeDirectoryWebSocket.instances[0];
      expect(socket.url).toBe("ws://hub.test/directory/ws?token=tok-1&since=0");
      socket.triggerOpen();
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(1) });
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(2) });
      socket.triggerMessage({ kind: "heartbeat", headSeq: 2 });
      expect(received.map((message) => message.kind)).toEqual(["event", "event", "heartbeat"]);
      expect(received).toHaveLength(3);
      handle.close();
    });
    it("reconnects resuming from the last seen seq, with jittered backoff within bounds", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        globalThis.WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0);
        const client = new DirectoryClient("http://hub.test", "tok-1");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0];
        first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(7) });
        first.triggerClose();
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        const second = FakeDirectoryWebSocket.instances[1];
        expect(second.url).toBe("ws://hub.test/directory/ws?token=tok-1&since=7");
        randomSpy.mockReturnValue(1);
        second.triggerClose();
        await Promise.resolve();
        const attempt2Cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** 2);
        await vi.advanceTimersByTimeAsync(attempt2Cap - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(3);
        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });
    it("never throws into the caller on a malformed frame", () => {
      FakeDirectoryWebSocket.instances = [];
      globalThis.WebSocket = FakeDirectoryWebSocket;
      const received = [];
      const client = new DirectoryClient("http://hub.test");
      const handle = client.stream(0, (message) => received.push(message));
      const socket = FakeDirectoryWebSocket.instances[0];
      expect(() => socket.onmessage?.({ data: "not json" })).not.toThrow();
      socket.triggerMessage({ kind: "heartbeat", headSeq: 0 });
      expect(received).toHaveLength(1);
      handle.close();
    });
    it("close() stops the reconnect loop — no further socket is ever opened", () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        globalThis.WebSocket = FakeDirectoryWebSocket;
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0];
        handle.close();
        first.triggerClose();
        vi.advanceTimersByTime(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
      }
    });
    it("(a) a drop after sustained health resets the backoff — reconnects near HUB_RECONNECT_MIN_MS, not at an escalated delay", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        globalThis.WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0);
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0];
        first.triggerOpen();
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS);
        first.triggerClose();
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });
    it("(b) rapid accept-then-drop cycling never crosses the health threshold — backoff keeps escalating, never resets", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        globalThis.WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(1);
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        let instanceCount = 1;
        for (let attempt = 1;attempt <= 3; attempt++) {
          const socket = FakeDirectoryWebSocket.instances[instanceCount - 1];
          socket.triggerOpen();
          socket.triggerClose();
          await Promise.resolve();
          const cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** attempt);
          await vi.advanceTimersByTimeAsync(cap - 1);
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
          await vi.advanceTimersByTimeAsync(1);
          instanceCount += 1;
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
        }
        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });
    it("(c) close() during a healthy-but-not-yet-reset connection cancels promptly, clears the health timer, and never reconnects", async () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        globalThis.WebSocket = FakeDirectoryWebSocket;
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0];
        first.triggerOpen();
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS / 2);
        handle.close();
        first.triggerClose();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        expect(vi.getTimerCount()).toBe(0);
      } finally {
        vi.useRealTimers();
      }
    });
  });
  describe("DirectoryClient http (getJson/postJson timeout + abort)", () => {
    const originalFetch = globalThis.fetch;
    it("a hung server rejects at the timeout instead of hanging the caller forever", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url, init2) => {
          return new Promise((_resolve, reject) => {
            init2?.signal?.addEventListener("abort", () => reject(init2.signal.reason ?? new Error("aborted")));
          });
        });
        const client = new DirectoryClient("http://hub.test");
        const promise = client.me();
        let settled = false;
        promise.then(() => settled = true, () => settled = true);
        await vi.advanceTimersByTimeAsync(DIRECTORY_HTTP_TIMEOUT_MS + 1000);
        expect(settled).toBe(true);
        await expect(promise).rejects.toThrow();
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });
    it("an external abort cancels promptly, without ever waiting out the timeout", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url, init2) => {
          return new Promise((_resolve, reject) => {
            init2?.signal?.addEventListener("abort", () => reject(init2.signal.reason ?? new Error("aborted")));
          });
        });
        const client = new DirectoryClient("http://hub.test");
        const controller = new AbortController;
        const promise = client.spaces({ signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        await expect(promise).rejects.toThrow("caller cancelled");
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });
    it("still resolves normally when the server answers promptly", async () => {
      globalThis.fetch = vi.fn(async () => ({ ok: true, status: 200, json: async () => [] }));
      try {
        const client = new DirectoryClient("http://hub.test");
        await expect(client.spaces()).resolves.toEqual([]);
      } finally {
        globalThis.fetch = originalFetch;
      }
    });
  });
}
/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-runtime.ts */
init__shard_client();
var SHARD_WORKER_URL = "/plugin-modules/_shard/\uD83D\uDFE8️shard-worker.js";
var DEFAULT_SHARD_BUDGET = { fuel: 50000000, wallMs: 100, memoryBytes: 256 * 1024 * 1024, uiNodes: 20000, mailboxLen: 64, maxEffects: 64, maxPatchBytes: 1 << 20 };
function poolConcurrency() {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}
function buildShardClientOptions(overrides) {
  return {
    shardCount: poolConcurrency(),
    createWorker: () => new Worker(SHARD_WORKER_URL, { type: "module" }),
    ...overrides
  };
}
function createPooledActorRuntime(options) {
  const shardClient = new ShardClient(buildShardClientOptions(options));
  shardClient.startWatchdog();
  return { shardClient };
}

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts */
function coerceWireBytes(raw) {
  if (raw instanceof Uint8Array)
    return raw;
  if (Array.isArray(raw))
    return Uint8Array.from(raw);
  if (raw && typeof raw === "object") {
    const record = raw;
    if (record.kind === "bytes" && Array.isArray(record.value))
      return Uint8Array.from(record.value);
    if (Array.isArray(record.data))
      return Uint8Array.from(record.data);
  }
  if (typeof raw === "string") {
    const binary = atob(raw);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0;i < binary.length; i++)
      bytes[i] = binary.charCodeAt(i);
    return bytes;
  }
  throw new Error(`[DEBUG] coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.slice(0, 120)}`);
}
function coerceTurnResult(raw) {
  const record = raw && typeof raw === "object" ? raw : {};
  const uiPatches = Array.isArray(record.uiPatches) ? record.uiPatches : [];
  const effects = Array.isArray(record.effects) ? record.effects : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  return { uiPatches, effects, nextWake };
}
function shellFrameBytes(effect, instanceId) {
  if (effect.tag !== "send-message")
    return null;
  const val = effect.val ?? {};
  if (!val.target || val.target.tag !== "shell")
    return null;
  if (Number(val.target.val) !== instanceId)
    return null;
  if (val.payload === undefined)
    return null;
  return coerceWireBytes(val.payload);
}
function decodeWirePatchOps(ops, decodePackValue2) {
  const decoded = [];
  for (const op of ops) {
    const val = op.val ?? {};
    const path = Array.isArray(val.path) ? val.path : [];
    switch (op.tag) {
      case "replace":
        decoded.push({ kind: "Replace", path, node: decodePackValue2(coerceWireBytes(val.node)) });
        break;
      case "insert-child":
        decoded.push({ kind: "InsertChild", path, index: Number(val.index ?? 0), node: decodePackValue2(coerceWireBytes(val.node)) });
        break;
      case "remove-child":
        decoded.push({ kind: "RemoveChild", path, index: Number(val.index ?? 0) });
        break;
      case "set-props":
        decoded.push({ kind: "SetProps", path, props: val.props !== undefined ? decodePackValue2(coerceWireBytes(val.props)) : undefined });
        break;
      default:
        break;
    }
  }
  return decoded;
}
function applyUiPatchToRetained(previous, patch) {
  let node = previous?.node ?? null;
  let sawFullReplace = false;
  for (const op of patch.ops) {
    if (op.kind === "Replace" && op.path.length === 0) {
      node = op.node;
      sawFullReplace = true;
    } else {
      return { surface: previous, desynced: true };
    }
  }
  if (!sawFullReplace && previous && patch.baseRevision !== previous.revision)
    return { surface: previous, desynced: true };
  return { surface: node !== null ? { revision: patch.revision, node } : previous, desynced: false };
}
function wireEffectToFriendly(effect, decodePackValue2) {
  const val = effect.val ?? {};
  const str = (key) => String(val[key] ?? "");
  const num = (key) => Number(val[key] ?? 0);
  const packField = (key) => val[key] !== undefined ? decodePackValue2(coerceWireBytes(val[key])) : undefined;
  switch (effect.tag) {
    case "request-sync":
      return "requestSync";
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: str("kind"), params: packField("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "spawn-plugin-instance":
      return { spawnPluginInstance: { req: num("req"), pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId, label: val.label, documentJson: val.documentJson } };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId } };
    default:
      console.warn(`[DEBUG] wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion`);
      return null;
  }
}

/* 🟦️typescript/🐚️plugin-bridge.ts */
var pooledRuntime = null;
function getShardClient() {
  pooledRuntime ??= createPooledActorRuntime({
    onActorTrap: (actorId, message) => console.error(`[DEBUG] wgpu plugin-bridge: actor ${actorId} trapped: ${message}`),
    onShardLost: (shardIndex, actorIds) => {
      console.error(`[DEBUG] wgpu plugin-bridge: shard ${shardIndex} lost, restoring actors: ${actorIds.join(", ")}`);
      getActivationRegistry().handleShardLost(shardIndex, actorIds);
    }
  });
  return pooledRuntime.shardClient;
}
var sharedActivationRegistry = null;
function getActivationRegistry() {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}
var actorTurnChains = new Map;
function submitTurn(actorId, events) {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => {
    return;
  });
  const next = previousSettled.then(() => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET));
  actorTurnChains.set(actorId, next);
  return next.then(coerceTurnResult);
}
var retainedWindowByActor = new Map;
function applyRetainedWindowPatches(actorId, uiPatches) {
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? [], decodePackValue);
    const previous = retainedWindowByActor.get(actorId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      console.warn(`[DEBUG] plugin-bridge: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — keeping the previously retained body`);
      continue;
    }
    if (surface)
      retainedWindowByActor.set(actorId, surface);
  }
}
async function performRender(actorId, instanceId) {
  const result = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: 0 } } }]);
  if (result.uiPatches.length > 0)
    applyRetainedWindowPatches(actorId, result.uiPatches);
  return retainedWindowByActor.get(actorId)?.node ?? null;
}
var pendingTurnEffects = new Map;
var nextGlobalInstanceId = 1;
async function performInvocation(client, instanceId, invocation, viewState) {
  const frames = await client.command(encodePackValue(invocation), viewState);
  let output = null;
  let diagnostics = [];
  let uiScope;
  let historyPatch;
  for (const frame of frames) {
    if ("Invocation" in frame) {
      output = decodePackValue(new Uint8Array(frame.Invocation.output));
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? decodedDiagnostics : [];
      uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope));
      const decodedHistoryPatch = decodePackValue(new Uint8Array(frame.Invocation.history_patch));
      historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? decodedHistoryPatch : undefined;
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault)
        throw new SemioFaultError(fault);
      throw new Error(`invocation failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const requestedEffects = leftover.map((effect) => wireEffectToFriendly(effect, decodePackValue)).filter((effect) => effect !== null);
  return { output, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics, requestedEffects, events: [], uiScope, historyPatch };
}
async function fetchDescriptorManifest(pluginId, moduleUrl, signal) {
  const descriptorUrl = moduleUrl.replace(/\/[^/]+$/, "/\uD83D\uDD23️descriptor.json");
  try {
    const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
    if (response.ok) {
      const descriptor = await response.json();
      if (descriptor.manifest)
        return descriptor.manifest;
    }
  } catch (error) {
    if (signal?.aborted)
      throw error;
    console.warn(`[DEBUG] fetchDescriptorManifest: ${descriptorUrl} unreachable — using an empty manifest`, error);
  }
  console.warn(`[DEBUG] fetchDescriptorManifest: no descriptor for ${pluginId} yet — loading with an empty manifest, no eager instantiation`);
  return { pluginId, label: pluginId, version: "", apps: [], workflows: [], examples: [] };
}
async function loadPluginModule(pluginId, moduleUrl, signal) {
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl, signal);
  const shardClient = getShardClient();
  const actorIdByInstance = new Map;
  const channelByInstance = new Map;
  let eventSeq = 0;
  const requireActorId = (instanceId) => {
    const actorId = actorIdByInstance.get(instanceId);
    if (!actorId)
      throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const requireChannel = (instanceId) => {
    const client = channelByInstance.get(instanceId);
    if (!client)
      throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  const turnOutcomes = createTurnOutcomeBroadcast();
  const runQueuedTurn = async (instanceId, events) => {
    try {
      const actorId = requireActorId(instanceId);
      const shardEvents = events.map((frame) => {
        eventSeq += 1;
        return { kind: "app-command", payload: { instance: instanceId, seq: eventSeq, command: Array.from(frame) } };
      });
      const result = await submitTurn(actorId, shardEvents);
      const outFrames = [];
      const leftover = [];
      for (const effect of result.effects) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame)
          outFrames.push(frame);
        else
          leftover.push(effect);
      }
      pendingTurnEffects.set(instanceId, leftover);
      if (result.uiPatches.length > 0)
        applyRetainedWindowPatches(actorId, result.uiPatches);
      turnOutcomes.push({ instanceId, frames: outFrames });
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };
  const channelHandle = {
    enqueue: (instanceId, events) => {
      runQueuedTurn(instanceId, events);
    },
    outcomes: turnOutcomes.stream
  };
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      await registry.activate(pluginId, actorId, "manual");
      eventSeq += 1;
      await submitTurn(actorId, [{ kind: "instance-open", payload: { instance: instanceId, appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) } }]);
      channelByInstance.set(instanceId, new AppChannelClient(channelHandle, instanceId, appId, "local"));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId)
        return;
      channelByInstance.get(instanceId)?.dispose();
      actorIdByInstance.delete(instanceId);
      channelByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      shardClient.dispose(actorId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), viewState),
    render: (instanceId) => performRender(requireActorId(instanceId), instanceId),
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    dispose: () => {
      for (const instanceId of channelByInstance.keys())
        channelByInstance.get(instanceId)?.dispose();
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
      channelByInstance.clear();
      turnOutcomes.complete();
    }
  };
}
function viewStateFromContextJson(contextJson) {
  try {
    const parsed = JSON.parse(contextJson);
    return parsed && typeof parsed === "object" && "viewState" in parsed ? parsed.viewState : parsed;
  } catch {
    return;
  }
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, contextJson) => handle.handleAction(instanceId, actionJson, viewStateFromContextJson(contextJson)).then((result) => JSON.stringify(result)),
    handleCommand: (instanceId, commandJson, contextJson) => handle.handleCommand(instanceId, commandJson, viewStateFromContextJson(contextJson)).then((result) => JSON.stringify(result)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items))
  };
}

/* 🟦️typescript/🧵️frame-worker.ts */
var WORKER_STEP_BUDGET_MS = 8;
var BOOT_HEARTBEAT_MS = 2;
var PLUGIN_BOOT_CAPACITY = 32;
var PLUGIN_MANIFEST_CODE_UNIT_CAPACITY = 64 * 1024;
function ownedStep(stage, callback) {
  const startedAt = performance.now();
  const value = callback();
  const duration = performance.now() - startedAt;
  if (duration >= WORKER_STEP_BUDGET_MS)
    throw new Error(`worker-boot-step-overrun: ${stage} took ${duration.toFixed(3)} ms`);
  return value;
}
async function monitoredSuspension(stage, operation) {
  let lastBeat = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
  }, BOOT_HEARTBEAT_MS);
  try {
    const result = await ownedStep(`${stage}:start`, operation);
    await new Promise((resolve2) => setTimeout(resolve2, 0));
    if (closed || closing)
      throw new Error(`worker-boot-cancelled: ${stage}`);
    if (maximumBlockMs >= WORKER_STEP_BUDGET_MS)
      throw new Error(`worker-boot-step-overrun: ${stage} blocked the Worker for ${maximumBlockMs.toFixed(3)} ms`);
    return result;
  } finally {
    clearInterval(heartbeat);
  }
}
async function macrotask() {
  await new Promise((resolve2) => setTimeout(resolve2, 0));
  if (closed || closing)
    throw new Error("worker-boot-cancelled");
}
var scope = self;
var lifecycle = 0;
var runtime;
var interactiveJobs;
var closed = false;
var closing = false;
var failed = false;
var quarantined;
var lastFrame = { cursor: "default", fullscreen: null };
var pendingFault;
var runtimeCloseComplete = false;
var jobsCloseComplete = false;
var closeOwner = "runtime";
scope.onmessage = (event) => void receive(event.data);
async function receive(message) {
  if (message.kind === "boot") {
    await boot(message);
    return;
  }
  if (message.lifecycle !== lifecycle)
    return;
  if (message.kind === "close") {
    if (closed || closing)
      return;
    beginClose();
    return;
  }
  if (closed || closing || failed || quarantined)
    return;
  if (message.kind === "job-submit" || message.kind === "job-input-page" || message.kind === "job-cancel") {
    if (!interactiveJobs) {
      fault("interactive-job-not-ready", "interactive job arrived before Worker boot completed");
      return;
    }
    const startedAt2 = performance.now();
    interactiveJobs.receive(message);
    const duration = performance.now() - startedAt2;
    if (duration >= WORKER_STEP_BUDGET_MS)
      fault("interactive-job-overrun", `interactive job admission turn took ${duration.toFixed(3)} ms`);
    return;
  }
  if (!runtime) {
    fault("worker-not-booted", "frame batch arrived before renderer boot completed");
    return;
  }
  const startedAt = performance.now();
  try {
    runtime.enqueueBatch(JSON.stringify({ replaceable: message.replaceable, lossless: message.lossless }), message.generation);
    const result = JSON.parse(runtime.tick(message.timestampMs, message.sequence, message.generation));
    const duration = performance.now() - startedAt;
    lastFrame = { cursor: result.cursor, fullscreen: result.fullscreen };
    if (result.quarantined || duration >= WORKER_STEP_BUDGET_MS)
      quarantined = { code: result.faultCode ?? "worker-step-overrun", detail: result.faultDetail ?? `frame step took ${duration.toFixed(3)} ms` };
    post({ kind: "frame", lifecycle, sequence: message.sequence, generation: message.generation, cursor: result.cursor, fullscreen: result.fullscreen, requestFrame: result.requestFrame, progress: result.progress, workerDurationMs: duration, quarantined: quarantined !== undefined, faultCode: quarantined?.code, faultDetail: quarantined?.detail });
    if (quarantined)
      requestFault(quarantined.code, quarantined.detail);
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
  }
}
async function closeRuntime() {
  for (;; ) {
    const startedAt = performance.now();
    if (closeOwner === "runtime" && !runtimeCloseComplete) {
      runtimeCloseComplete = runtime ? runtime.closeStep() : true;
      closeOwner = "jobs";
    } else if (!jobsCloseComplete) {
      jobsCloseComplete = interactiveJobs ? interactiveJobs.closeStep() : true;
      closeOwner = "runtime";
    } else if (!runtimeCloseComplete) {
      closeOwner = "runtime";
    }
    if (performance.now() - startedAt >= WORKER_STEP_BUDGET_MS) {
      pendingFault ??= { code: "worker-close-overrun", detail: "Worker close turn exceeded the Worker budget" };
    }
    if (runtimeCloseComplete && jobsCloseComplete)
      break;
    await new Promise((resolve2) => setTimeout(resolve2, 0));
  }
  if (pendingFault)
    post({ kind: "fault", lifecycle, code: pendingFault.code, detail: pendingFault.detail });
  post({ kind: "closed", lifecycle });
  closed = true;
  scope.close();
}
function beginClose() {
  if (closed || closing)
    return;
  closing = true;
  failed = pendingFault !== undefined;
  runtimeCloseComplete = runtime === undefined;
  jobsCloseComplete = interactiveJobs === undefined;
  interactiveJobs?.close();
  closeRuntime();
}
async function boot(message) {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  lifecycle = message.lifecycle;
  try {
    progress("renderer-module", 0.05);
    const bindings = await monitoredSuspension("renderer-module", () => import(message.bindingsModuleUrl));
    if (bindings.default) {
      progress("wasm-instance", 0.15);
      await monitoredSuspension("wasm-instance", () => bindings.default(message.bindingsWasmUrl));
    }
    if (!bindings.semioWgpuWorkerBootstrap)
      throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      bindings.semioWgpuSetAppRole?.(message.appRole);
      if (message.hub)
        bindings.semioWgpuSetHubEnv?.(message.hub.hubUrl, message.hub.user, message.hub.dataDir);
    });
    progress("plugin-graph", 0.25);
    const bootPlan = ownedStep("plugin-graph", () => resolvePlaygroundBoot(PLUGIN_CATALOG, message.pluginVariant));
    if (bootPlan.plugins.length > PLUGIN_BOOT_CAPACITY)
      throw new Error(`plugin-credits: boot plan exceeds ${PLUGIN_BOOT_CAPACITY} plugins`);
    for (const error of bootPlan.dependencyErrors)
      progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins = [];
    for (let index = 0;index < bootPlan.plugins.length; index++) {
      const target = bootPlan.plugins[index];
      progress(`plugin:${target.pluginId}`, 0.3 + 0.3 * (index / Math.max(1, bootPlan.plugins.length)));
      await macrotask();
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl));
      ownedStep(`plugin-manifest:${target.pluginId}`, () => {
        const manifest = JSON.stringify(module.manifest);
        if (manifest.length > PLUGIN_MANIFEST_CODE_UNIT_CAPACITY)
          throw new Error(`plugin-manifest-credits: ${target.pluginId} exceeds ${PLUGIN_MANIFEST_CODE_UNIT_CAPACITY} code units`);
      });
      plugins.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, handle: pluginHandleForBridge(module) })));
    }
    if (plugins.length === 0)
      throw new Error(`no wasm plugin modules found for variant ${message.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => bindings.semioWgpuWorkerBootstrap(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => post({ kind: "wake", lifecycle })));
    while (true) {
      await macrotask();
      const step2 = ownedStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()));
      progress(step2.stage, 0.65 + step2.progress * 0.3);
      if (step2.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell());
        continue;
      }
      if (step2.complete)
        break;
    }
    runtime = ownedStep("renderer-finish", () => bootstrap.finish());
    interactiveJobs = ownedStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)));
    progress("ready", 1);
    post({ kind: "booted", lifecycle });
  } catch (error) {
    fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}
function progress(stage, value) {
  if (!closed && !closing && !failed)
    post({ kind: "boot-progress", lifecycle, stage, progress: value });
}
function post(message) {
  scope.postMessage(message);
}
function fault(code, detail) {
  requestFault(code, detail);
}
function requestFault(code, detail) {
  if (closed || pendingFault)
    return;
  pendingFault = { code, detail };
  failed = true;
  beginClose();
}
