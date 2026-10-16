/** 🖼️ The retained surface views: carrier epoch, paged surface, accessibility shell and the surface view.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { useCallback, useSyncExternalStore, type ReactNode } from "react";
import { type SceneLaneRef, sceneFromLanes } from "@semio-tech/framework";
import { UiDocumentStore, useUiDocumentRevision } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type Component, type UiNodeRecord } from "@semio-tech/framework";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { renderComponentSceneHost, surfaceKindSceneLanes } from "../🔌️surface-bridge/🟦️.tsx";
import { surfaceLaneTexts } from "../🚚️surface-scene-lanes/🟦️.ts";
import { accessibilityAriaProps } from "../📐️layout-accessibility/🟦️.tsx";
import { dispatchTrigger } from "../⚡️action-dispatch/🟦️.tsx";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
/** 🚚️ A surface whose scene declares out-of-doc payload lanes (`world-3d`, `canvas-2d`, `board-2d`). Its `doc.bytes` carry only
 * the spine; the lanes are retained text-leaf subtrees hanging off this very node, so this view — and
 * only this view — subscribes to the whole document's revision: a lane leaf changing does NOT change
 * this node's own record, and a tree larger than one reconcile page arrives across several patches. */
function surfaceCarrierEpoch(store: UiDocumentStore, record: UiNodeRecord): string {
  const state = store.getState();
  const stack = [...(record.children ?? [])];
  const parts: string[] = [];
  while (stack.length > 0) {
    const id = stack.pop()!;
    const child = state.nodes.get(id);
    if (!child) continue;
    parts.push(`${id}:${String(child.key)}:${child.component.type === "text" && typeof child.component.value === "string" ? child.component.value.length : 0}`);
    for (const nested of child.children ?? []) stack.push(nested);
  }
  return parts.join("|");
}

function useSurfaceCarrierEpoch(store: UiDocumentStore, record: UiNodeRecord): string {
  const childKey = (record.children ?? []).join(",");
  return useSyncExternalStore(
    (onChange) => {
      const state = store.getState();
      const ids: number[] = [];
      const stack = [...(record.children ?? [])];
      while (stack.length > 0) {
        const id = stack.pop()!;
        ids.push(id);
        const child = state.nodes.get(id);
        for (const nested of child?.children ?? []) stack.push(nested);
      }
      const unsubs = ids.map((id) => store.subscribeNode(id)(onChange));
      return () => {
        for (const unsub of unsubs) unsub();
      };
    },
    () => surfaceCarrierEpoch(store, record),
    () => surfaceCarrierEpoch(store, record),
  );
}

function PagedSurfaceView({ record, component, context }: { readonly record: UiNodeRecord; readonly component: Extract<Component, { type: "surface" }>; readonly context: UiInterpreterContext }) {
  const store = context.store;
  const revision = useUiDocumentRevision(store);
  const carrierEpoch = useSurfaceCarrierEpoch(store, record);
  const lanes = surfaceKindSceneLanes(component.kind) ?? [];
  const assemble = useCallback(
    (spine: Record<string, unknown>): Record<string, unknown> => {
      void revision;
      void carrierEpoch;
      const declared = Array.isArray(spine.lanes) ? (spine.lanes as readonly SceneLaneRef[]) : [];
      return sceneFromLanes(spine, surfaceLaneTexts(record, store.getState(), declared, lanes), lanes);
    },
    [record, store, revision, carrierEpoch, lanes],
  );
  return <>{renderComponentSceneHost(record, component, context.onAction, store.getState().surface, context.requestContextMenu, assemble)}</>;
}

/**
 * ♿️ The accessible door to a canvas.
 *
 * Every other component view reaches its ARIA for free, because it renders a real HTML element that
 * already carries the role and takes focus. A surface renders a scene host that paints into a
 * `<canvas>` — an opaque texture with no accessible children at all — so without this wrapper the
 * node-graph and World3d surfaces were unnamed, unfocusable and silent: the `AccessibilitySpec` the
 * contract carries on the record was simply dropped on the floor (`renderComponentSceneHost` never
 * received it), which an accessibility-tree probe on 6018 confirmed — three canvases, no `role`, no
 * `aria-label`, no `tabindex`.
 *
 * `role="application"` is the ARIA contract for "this widget handles its own arrow/Enter keys", which
 * is exactly what a scene host does, and it is the role `accessibility_role` already implies for
 * `Component::Surface` on every renderer. `tabIndex` puts the canvas in the Tab order, matching the
 * contract's own `accessibility_is_focusable`, so React and the wgpu `EventRouter` agree about who is
 * reachable. The layout classes mirror what every scene host's own root already sets
 * (`relative h-full min-h-0 w-full`), so inserting this element changes no geometry.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs
 */
function SurfaceAccessibilityShell({ record, context, children }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext; readonly children: ReactNode }) {
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  const activateBinding = (record.bindings ?? []).find((binding) => binding.trigger === "activate");
  return (
    <div
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      data-ui-surface-shell=""
      data-window-content-layout={record.component.type === "surface" && record.component.kind === "canvas-2d" ? "edgeless" : undefined}
      role="application"
      tabIndex={record.disabled ? -1 : 0}
      aria-disabled={record.disabled || undefined}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      className="focus-visible:ring-primary relative h-full min-h-0 w-full min-w-0 outline-none focus-visible:ring-2"
      onKeyDown={
        activateBinding && !record.disabled
          ? (event) => {
              if (event.key !== "Enter" && event.key !== " ") return;
              event.preventDefault();
              dispatchTrigger(context, record, "activate");
            }
          : undefined
      }
      {...aria}
    >
      {describedBy}
      {children}
    </div>
  );
}

export function SurfaceView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "surface" }>;
  // 🚚️ EVERY scene kind that publishes lanes comes through `PagedSurfaceView`, which puts them back;
  // the membership test is {@link SURFACE_KIND_SCENE_LANES} itself, never a list repeated here.
  const body = surfaceKindSceneLanes(component.kind) ? <PagedSurfaceView record={record} component={component} context={context} /> : <>{renderComponentSceneHost(record, component, context.onAction, context.store.getState().surface, context.requestContextMenu)}</>;
  return (
    <SurfaceAccessibilityShell record={record} context={context}>
      {body}
    </SurfaceAccessibilityShell>
  );
}
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
