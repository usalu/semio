import React from "react";
import { WasmCanvas } from "@semio-tech/framework/canvas/react-renderer";
import { CanvasPickMenu, cn, ContextMenuController, SelectionMarquee, surfaceClass, type CanvasPickTarget } from "@semio-tech/ui-react";
import { type ActionDescriptor, type ContextMenuItemSpec, type NodeGraphScene, type PluginContextMenuRequest, type PluginContextMenuSurfaceTarget } from "@semio-tech/framework";
import { openSurfaceContextMenu } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { parseSelectionDomainsFromSession, selectionGroupsFromDomains } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { type GraphKeyboardPort, graphPinchWheelPlan } from "../🧰️graph-core/🟦️.ts";
import { alignModeToDag, GraphSliderOverlays, SelectionAlignChrome } from "../🖼️dag-overlays/🟦️.tsx";
import { useWindowInstanceIdState } from "./🔶️window-instance-id/🟦️.tsx";
import { useDispatchGraphEditsState } from "./🔸️dispatch-graph-edits/🟦️.tsx";

//#region 🔖️NodeGraphHost
//#region WasmGraphSurface
export function WasmGraphSurface({
  scene,
  surfaceId,
  controllerId,
  editable,
  requestContextMenu,
  onAction,
  keyboardPort,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly keyboardPort: React.MutableRefObject<GraphKeyboardPort | null>;
}) {
  const { sessionRef, paintOverlays, emitInteractionState, dispatch, containerRef, sceneRef, windowInstanceId, mapContextMenu, shellContextMenuFallback, setContextMenu, sessionFactory, onSessionReady, labelCanvasRef, selectionBounds, marquee, gestureRecognizer, pinchLogScaleRef, sliderStateJson, overlaySize, sliderLane, contextMenuTitleLabel, contextMenu } = useWindowInstanceIdState({ scene, onAction, controllerId, surfaceId });
  const { pickInteraction, dispatchGraphEdits } = useDispatchGraphEditsState({ keyboardPort, sessionRef, paintOverlays, emitInteractionState, dispatch, containerRef, sceneRef });
  return (
    <div
      ref={containerRef}
      className={cn("relative h-full w-full", surfaceClass)}
      data-level="base"
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const session = sessionRef.current;
          const container = containerRef.current;
          let hits: NonNullable<PluginContextMenuSurfaceTarget["hits"]> = [];
          let domains = { nodes: [] as string[], edges: [] as string[], handles: [] as string[] };
          if (session && container) {
            const rect = container.getBoundingClientRect();
            const sx = event.clientX - rect.left;
            const sy = event.clientY - rect.top;
            try {
              const targets = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
              hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
            } catch {
              hits = [];
            }
            try {
              domains = parseSelectionDomainsFromSession(session.selectionDomainsJson?.() ?? session.selectedNodeIdsJson());
            } catch {
              domains = { nodes: [], edges: [], handles: [] };
            }
          }
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph", args: null },
              surface: { surfaceId, kind: "nodeGraph", hits, selection: selectionGroupsFromDomains(domains) },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            mapContextMenu,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
        })();
      }}
      onPointerUp={emitInteractionState}
    >
      <WasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      {selectionBounds ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} /> : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30 touch-none"
        data-gesture-surface="dag"
        onPointerDown={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinchBegin") {
            const session = sessionRef.current;
            pinchLogScaleRef.current = 0;
            pickInteraction.onCanvasPointerLeave();
            for (const tracked of gestureRecognizer.pointers) event.currentTarget.setPointerCapture?.(tracked.pointerId);
            session?.pointerCancelScreen?.();
            session?.renderFrame();
            paintOverlays();
            return;
          }
          if (verdict.kind !== "single") return;
          if (!editable) return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session?.pointerDownScreen) return;
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinch") {
            if (!session?.wheelScreen) return;
            const plan = graphPinchWheelPlan(verdict.step, pinchLogScaleRef.current);
            pinchLogScaleRef.current = plan.pendingLogScale;
            if (plan.calls.length === 0) return;
            for (const call of plan.calls) session.wheelScreen(call.sx, call.sy, call.deltaX, call.deltaY, call.zoomGesture);
            session.renderFrame();
            paintOverlays();
            return;
          }
          if (verdict.kind !== "single") return;
          if (!session?.pointerMoveScreen) return;
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") {
            pinchLogScaleRef.current = 0;
            emitInteractionState();
          }
          if (verdict.kind !== "single") return;
          if (!session?.pointerUpScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          emitInteractionState();
          dispatchGraphEdits();
        }}
        onPointerCancel={(event) => {
          const session = sessionRef.current;
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") {
            pinchLogScaleRef.current = 0;
            emitInteractionState();
          }
          if (verdict.kind !== "single") return;
          pickInteraction.onCanvasPointerLeave();
          if (!session?.pointerCancelScreen) return;
          session.pointerCancelScreen();
          session.renderFrame();
          paintOverlays();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session?.wheelScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          emitInteractionState();
        }}
      />
      {selectionBounds && editable ? (
        <SelectionAlignChrome
          bounds={selectionBounds}
          onAlign={(mode) => {
            const session = sessionRef.current;
            if (!session?.alignSelection) return;
            session.alignSelection(alignModeToDag(mode));
            dispatchGraphEdits();
            session.renderFrame();
            emitInteractionState();
          }}
        />
      ) : null}
      <GraphSliderOverlays
        scopeId={JSON.stringify([windowInstanceId, controllerId, surfaceId])}
        stateJson={sliderStateJson}
        logicalW={overlaySize.w}
        logicalH={overlaySize.h}
        editable={editable}
        onSliderChange={(widgetId, value) => sliderLane(widgetId).offer(value)}
        onSliderCommit={(widgetId, value) => sliderLane(widgetId).commit(value)}
        onSliderAbort={(widgetId, reason) => sliderLane(widgetId).abort(reason)}
      />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}

//#endregion WasmGraphSurface
//#endregion 🔖️NodeGraphHost
