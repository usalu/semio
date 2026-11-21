/** 🧾️ The interaction ledger, hover and context menu requests, the pointer-up gesture and the selection and hover publication.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowInteractionPublication` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { decodeDagChannelsJson, decodeDagHoverJson, decodeDagSelectionJson } from "@semio-tech/framework/infinite/board/io/text/dag-input";
import { useCallback, useMemo, useRef } from "../../⚛️react-port/🟦️.ts";
import { flowJsonText, nodeGraphInputDecodeControl, projectedFlowPortCode, publishNodeGraphHover, publishNodeGraphSelection } from "../🧰️graph-core/🟦️.ts";
import { readObservedFlowTask, syncFlowSessionStructureFromScene } from "../🔄️flow-session-sync/🟦️.ts";
import { createNodeGraphInteractionLedger } from "../🫱️interaction-publication/🟦️.ts";
import type { FlowCoreAfterFlowCanvasSurface } from "../📏️flow-canvas-surface/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowInteractionPublication = FlowCoreAfterFlowCanvasSurface & ReturnType<typeof useFlowInteractionPublication>;

export function useFlowInteractionPublication(core: FlowCoreAfterFlowCanvasSurface) {
  const {
    sessionRef, schedulerRef, appCatalogueRef, sceneRef, dispatch, windowInstanceId, endGesture, paintOverlays, renderFlow
  } = core;
  /** 🧾️ What this surface last told the plugin about selection and hover — see
   * {@link createNodeGraphInteractionLedger}. */
  const interactionLedger = useMemo(() => createNodeGraphInteractionLedger(windowInstanceId), [windowInstanceId]);
  const hoverRequestRef=useRef(0);
  const contextMenuRequestRef=useRef(0);

  const handleGesturePointerUp = useCallback(() => {
    endGesture("gesture");
    const session = sessionRef.current;
    if (session) {
      syncFlowSessionStructureFromScene(session, sceneRef.current, appCatalogueRef.current, true);
      renderFlow();
      paintOverlays();
    }
  }, [endGesture, paintOverlays, renderFlow]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const request=++hoverRequestRef.current;
    const sceneHoverRevision = interactionLedger.sceneHoverRevision;
    void Promise.all([
      readObservedFlowTask(session, "selectionDomainsJson:interaction", session.selectionDomainsJson()),
      readObservedFlowTask(session, "selectedChannelsJson:interaction", session.selectedChannelsJson()),
      readObservedFlowTask(session, "hoveredWidgetId:interaction", session.hoveredWidgetId()),
      readObservedFlowTask(session, "hoveredChannelJson:interaction", session.hoveredChannelJson()),
    ]).then(async ([domainsValue, selectedChannelValue, hoveredValue, channelValue]) => {
      const admission=nodeGraphInputDecodeControl(()=>sessionRef.current===session&&request===hoverRequestRef.current,()=>schedulerRef.current?.invalidate());
      const domains=await decodeDagSelectionJson(flowJsonText(domainsValue),admission);
      const channels = await decodeDagChannelsJson(flowJsonText(selectedChannelValue), admission);
      const nodes = sceneRef.current.nodes;
      const selection = { nodeIds: domains.nodes, edgeIds: domains.edges, handleIds: channels.map(channel => channel.widgetId + "@" + projectedFlowPortCode(nodes, channel)) };
      const hover=await decodeDagHoverJson(flowJsonText(channelValue),admission);
      const hovered = hover.channel?.widgetId ?? (typeof hoveredValue === "string" ? hoveredValue : undefined);
      if(sessionRef.current!==session||request!==hoverRequestRef.current)return;
      const portId = hover.channel && projectedFlowPortCode(sceneRef.current.nodes, hover.channel);
      const edgeId=hover.edge ?? undefined;
      const selectionDue=interactionLedger.publishSelection(selection);
      const hoverDue = interactionLedger.publishHover({ hoveredId: hovered, portId, edgeId }, sceneHoverRevision);
      if (selectionDue) publishNodeGraphSelection(dispatch, sceneRef.current.interactionDomain, selection);
      if (hoverDue) publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hovered, portId, edgeId);
    }).catch(() => {});
    paintOverlays();
  }, [dispatch, interactionLedger, paintOverlays]);

  return { interactionLedger, hoverRequestRef, contextMenuRequestRef, handleGesturePointerUp, emitInteractionState };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
