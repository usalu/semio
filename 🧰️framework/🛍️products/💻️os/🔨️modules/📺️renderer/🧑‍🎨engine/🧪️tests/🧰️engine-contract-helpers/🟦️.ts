/** 🧰 Shared fixtures, recorders and descriptor readers of the renderer engine-contract suites. */
import { createElement, type ReactElement } from "react";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { type ValidateFunction } from "ajv";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { type BuiltNode, type Component, type UiNodeRecord, type UiSnapshot, type ActionBinding, type LayoutSpec } from "@semio-tech/framework";
import { UiDocumentStore, type UiInterpreterContext, UiPresenceOverlayContext, type UiPresenceOverlayEntry, uiNodeToTreePanelConfig, interpretUiNode } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

const ownedExports = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(rendererSchema);

export const rendererExport = (exportId: string): ValidateFunction => ownedExports.getSchema(`${rendererSchema.$id}#/$defs/${exportId}`) as ValidateFunction;

export const { computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");

//#region 🔌️jsdom polyfills
// Renderer hosts measure through ResizeObserver; jsdom does not implement it.
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

if (!globalThis.ResizeObserver) globalThis.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;

// Renderer navigation calls scrollIntoView; jsdom does not implement it.
if (!Element.prototype.scrollIntoView) Element.prototype.scrollIntoView = () => {};
//#endregion 🔌️jsdom polyfills

export const noopAction = () => {};

/** 🌳️ Renders a panel body the way the dock does — through the tree's `emptyState`, full-width, never a property-layout control wrapper (ticket 26/09/02/PUZZLE-3D-END-TO-END, the inspection panel "shows nothing" defect). */
export const panelTreePanelHost = (config: ReturnType<typeof uiNodeToTreePanelConfig>): ReactElement => config.emptyState as ReactElement;


//#region 🧪️Contract test fixtures
// 🧬️ MIGRATION (react-tests packet, ticket 26/08/20): helpers for building `UiSnapshot` fixtures
// directly against the semantic contract, mirroring `📃️UiDocumentStore`'s/`Interpreter`'s own inline
// test `leaf`/`snapshot` helpers so fixture shape stays one convention across the package, not a
// second drifting copy.
type ContractNodeSpec = {
  readonly key: string;
  readonly component: Component;
  readonly layout?: LayoutSpec;
  readonly disabled?: boolean;
  readonly bindings?: readonly ActionBinding[];
  readonly children?: readonly ContractNodeSpec[];
};

export const CONTRACT_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };

const CONTRACT_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;

const CONTRACT_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

export function buildContractSnapshot(root: ContractNodeSpec): UiSnapshot {
  const nodes: UiNodeRecord[] = [];
  let nextId = 0;
  const walk = (spec: ContractNodeSpec): number => {
    const id = nextId;
    nextId += 1;
    const children = (spec.children ?? []).map(walk);
    nodes.push({
      id,
      key: spec.key,
      component: spec.component,
      layout: spec.layout ?? CONTRACT_LEAF_LAYOUT,
      style: CONTRACT_STYLE,
      activity: "idle",
      disabled: spec.disabled ?? false,
      transition: null,
      accessibility: CONTRACT_ACCESSIBILITY,
      bindings: [...(spec.bindings ?? [])],
      menu: null,
      children,
    });
    return id;
  };
  const rootId = walk(root);
  return { surface: "test", revision: 0, root: rootId, nodes, layoutEpoch: 0n } as UiSnapshot;
}

export function buildContractNode(spec: ContractNodeSpec): BuiltNode {
  return {
    key: spec.key,
    component: spec.component,
    layout: spec.layout ?? CONTRACT_LEAF_LAYOUT,
    style: CONTRACT_STYLE,
    activity: "idle",
    disabled: spec.disabled ?? false,
    accessibility: CONTRACT_ACCESSIBILITY,
    bindings: [...(spec.bindings ?? [])],
    menu: null,
    children: (spec.children ?? []).map(buildContractNode),
  } as BuiltNode;
}

/** 🌳️ Renders `root` (and its nested `children`) through the real `📃️UiDocumentStore`/`interpretUiNode`
 * production path — never a hand-rolled shadow renderer — optionally under a `UiPresenceOverlayContext`
 * so hover/selection-driven markup (never a document field, per the contract) can be exercised too.
 *
 * 🪲️ PRODUCTION BUG (reported, not fixed — forbidden file): `📃️UiDocumentStore`'s `useUiNode` calls
 * `useSyncExternalStore(subscribe, getSnapshot)` with only two arguments — no `getServerSnapshot` —
 * which React's SSR path (`renderToStaticMarkup`/`renderToString`) throws on ("Missing
 * getServerSnapshot, which is required for server-rendered content"). Every pre-migration test in
 * this describe block used `renderToStaticMarkup` (the old Interpreter had no store/hook to trip
 * this on); this helper uses client-side `render()` instead, which does not hit the SSR path — a
 * test-only workaround, not a fix for the underlying gap. See `📃️UiDocumentStore/🟦️.tsx`'s
 * `useUiNode`/`useUiDocumentRoot`/`useUiDocumentRevision`.
 */
export function renderContractTree(root: ContractNodeSpec, presenceByKey?: Readonly<Record<string, UiPresenceOverlayEntry>>): string {
  const store = new UiDocumentStore("test");
  store.loadSnapshot(buildContractSnapshot(root));
  const context: UiInterpreterContext = { store, onAction: noopAction, onIntent: () => {} };
  const tree = interpretUiNode(store, context);
  const element = presenceByKey ? createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map(Object.entries(presenceByKey)) } }, tree) : (tree as ReactElement);
  const { container } = render(element);
  const markup = container.innerHTML;
  cleanup();
  return markup;
}
//#endregion 🧪️Contract test fixtures
