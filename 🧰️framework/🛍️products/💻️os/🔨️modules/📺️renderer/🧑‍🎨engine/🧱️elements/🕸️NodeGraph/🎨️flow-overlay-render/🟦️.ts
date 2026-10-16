/** 🎨️ The coalesced overlay paint and the coalesced frame presentation.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowOverlayAndRender` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { decodeDagHoverJson } from "@semio-tech/framework/infinite/board/io/text/dag-input";
import { useCallback, useRef } from "react";
import { hopTrace } from "@semio-tech/framework/trace";
import { flowBoolean, flowJsonText, nodeGraphInputDecodeControl } from "../🧰️graph-core/🟦️.ts";
import {
  computeDagMarqueeOverlay,
  paintDagLabelOverlays,
  parseDagMinimapWidgetCursor,
  parseDagNodeIdArray,
  parseDagPreselectJson,
  parseDagSelectionUnionBoundsScreen
} from "../🖼️dag-overlays/🟦️.tsx";
import { observeFlowTask, readObservedFlowTask, sameOverlayValue } from "../🔄️flow-session-sync/🟦️.ts";
import { flowSurfaceRenderAllowed, flowUnpresentablePresentation } from "../🧷️flow-canvas-contract/🟦️.ts";
import type { FlowCoreAfterFlowDispatchAndGestures } from "../🎬️flow-gestures/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowOverlayAndRender = FlowCoreAfterFlowDispatchAndGestures & ReturnType<typeof useFlowOverlayAndRender>;

export function useFlowOverlayAndRender(core: FlowCoreAfterFlowDispatchAndGestures) {
  const {
    surfaceId, sessionRef, schedulerRef, surfaceReadyRef, drewOnceRef, gpuCanvasRef, labelCanvasRef, containerRef,
    replaceUnpresentableCanvasRef, setWireRefusal, setSelectionBounds, setMarquee, setLabelStateJson, setSliderStateJson,
    setContainerSize, overlayRequestRef, sceneRef
  } = core;
  /** 🏷️ Paints the label/slider/selection overlay, and COALESCES instead of pre-empting — the same
   * law `renderFlow` below carries, on the other canvas of this surface.
   *
   * Each pass reads eleven session queries through `observeFlowTask`, which keeps one task per feature
   * key and cancels the previous. A second pass starting while the first is in flight therefore
   * cancelled the first pass's own reads, its `Promise.all` rejected, and the overlay was NOT
   * painted. Since every invalidation calls this, the overlay painted exactly ONCE per session:
   * measured live as three `fillText` calls in a 60 s run, showing the node-graph's pre-sync picture
   * (the default two-widget fixture) for the whole session while the guest already held all seven.
   *
   * An in-flight pass is therefore left alone and a request arriving during one is re-issued when it
   * settles: at most one extra pass, never a dropped one. */
  const overlayInFlightRef = useRef(false);
  const overlayDirtyRef = useRef(false);
  const paintOverlaysRef = useRef<() => void>(() => {});
  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    if (overlayInFlightRef.current) {
      overlayDirtyRef.current = true;
      return;
    }
    overlayInFlightRef.current = true;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    const request = ++overlayRequestRef.current;
    void Promise.all([
      readObservedFlowTask(session, "labelOverlayPaintStateJson", session.labelOverlayPaintStateJson()),
      readObservedFlowTask(session, "selectedWidgetIds", session.selectedWidgetIds()),
      readObservedFlowTask(session, "preselectWidgetIdsJson", session.preselectWidgetIdsJson()),
      readObservedFlowTask(session, "previewOffWidgetIds", session.previewOffWidgetIds()),
      readObservedFlowTask(session, "hoveredWidgetId:overlay", session.hoveredWidgetId()),
      readObservedFlowTask(session, "sliderOverlayStateJson", session.sliderOverlayStateJson()),
      readObservedFlowTask(session, "selectionUnionBoundsScreenJson", session.selectionUnionBoundsScreenJson()),
      readObservedFlowTask(session, "selectionPreviewPointsJson", session.selectionPreviewPointsJson()),
      readObservedFlowTask(session, "selectionPreviewCrossing", session.selectionPreviewCrossing()),
      readObservedFlowTask(session, "selectionPreviewMethod", session.selectionPreviewMethod()),
      readObservedFlowTask(session, "hoveredChannelJson:overlay", session.hoveredChannelJson()),
    ]).then(async ([labelValue, selectedValue, preselectValue, dimmedValue, hoveredValue, sliderValue, boundsValue, pointsValue, crossingValue, methodValue, channelValue]) => {
      if (request !== overlayRequestRef.current || sessionRef.current !== session || labelCanvasRef.current !== labelCanvas) return;
      const labelJson = flowJsonText(labelValue);
      setLabelStateJson((prev) => (prev === labelJson ? prev : labelJson));
      const minimapCursor = parseDagMinimapWidgetCursor(labelJson);
      if (gpuCanvasRef.current) {
        gpuCanvasRef.current.style.cursor = minimapCursor ?? "";
      }
      const selectedIds = parseDagNodeIdArray(flowJsonText(selectedValue));
      const preselect = parseDagPreselectJson(flowJsonText(preselectValue));
      const dimmedIds = parseDagNodeIdArray(flowJsonText(dimmedValue));
      paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: typeof hoveredValue === "string" ? hoveredValue : null,
        selectedIds,
        preselect,
        dimmedIds,
        highlightIds: sceneRef.current.highlighted ?? [],
      });
      const nextSliderJson = flowJsonText(sliderValue);
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
      const nextBounds = parseDagSelectionUnionBoundsScreen(flowJsonText(boundsValue));
      setSelectionBounds((prev) => (sameOverlayValue(prev, nextBounds) ? prev : nextBounds));
      const nextMarquee = computeDagMarqueeOverlay(flowJsonText(pointsValue), flowBoolean(crossingValue), typeof methodValue === "string" ? methodValue : undefined);
      setMarquee((prev) => (sameOverlayValue(prev, nextMarquee) ? prev : nextMarquee));
      const hover=await decodeDagHoverJson(flowJsonText(channelValue),nodeGraphInputDecodeControl(()=>request===overlayRequestRef.current&&sessionRef.current===session&&labelCanvasRef.current===labelCanvas,()=>schedulerRef.current?.invalidate()));
      if(request!==overlayRequestRef.current||sessionRef.current!==session||labelCanvasRef.current!==labelCanvas)return;
      const nextRefusal=hover.refusal;
      setWireRefusal((prev) => (sameOverlayValue(prev, nextRefusal) ? prev : nextRefusal));
    })
      .catch(() => {})
      .finally(() => {
        overlayInFlightRef.current = false;
        if (!overlayDirtyRef.current) return;
        overlayDirtyRef.current = false;
        paintOverlaysRef.current();
      });
  }, []);
  paintOverlaysRef.current = paintOverlays;

  /** 🖼️ Presents one frame, and coalesces instead of pre-empting.
   *
   * `observeFlowTask` keeps ONE task per feature key and cancels the previous one, which is right for
   * a query whose answer is superseded — and wrong for the paint. A render reply is what actually puts
   * pixels on the canvas, and a resize has just cleared the backing store; cancelling the in-flight
   * render leaves the canvas blank until the NEXT render completes uncancelled, so a burst of
   * invalidations (mount, ResizeObserver, scene sync, theme sync — all of which arrive together)
   * repeatedly emptied a canvas the engine had in fact painted. Measured live: `render_frame` ran on
   * every invalidation (`[TRACE] dag draw lod=detail zoom=1.784` repeating) while `renderFlowCanvas`
   * ran exactly once for the whole session. So an in-flight render is left alone and a request that
   * arrives during one is remembered and re-issued when it settles — at most one extra frame, never a
   * dropped one. */
  const renderInFlightRef = useRef(false);
  const renderDirtyRef = useRef(false);
  const renderFlowRef = useRef<() => void>(() => {});
  const renderFlow = useCallback(() => {
    const session = sessionRef.current;
    const canvas = gpuCanvasRef.current;
    if (!session || !canvas || !flowSurfaceRenderAllowed(surfaceReadyRef.current)) return;
    if (renderInFlightRef.current) {
      renderDirtyRef.current = true;
      return;
    }
    renderInFlightRef.current = true;
    const closePaint = hopTrace.open("surface.paint", { surfaceId });
    const task = session.renderCanvas(canvas);
    observeFlowTask(session, "renderCanvas", task, () => {
      if (drewOnceRef.current) return;
      drewOnceRef.current = true;
    });
    void task.result
      .then((state) => {
        if ((state as { presentation?: string } | undefined)?.presentation === flowUnpresentablePresentation) replaceUnpresentableCanvasRef.current();
      })
      .catch(() => {})
      .finally(() => {
        closePaint();
        renderInFlightRef.current = false;
        if (!renderDirtyRef.current) return;
        renderDirtyRef.current = false;
        renderFlowRef.current();
      });
  }, [surfaceId]);
  renderFlowRef.current = renderFlow;

  return { paintOverlays, renderFlow };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
