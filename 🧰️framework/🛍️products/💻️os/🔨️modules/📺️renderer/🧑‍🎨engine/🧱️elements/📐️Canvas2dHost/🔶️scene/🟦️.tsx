/** 🔶️ The scene phase of Canvas2dHost. */
import { useCallback, useContext, useEffect, useMemo, useRef, useState, type PointerEvent as ReactPointerEvent } from "react";
import { useLabel } from "@semio-tech/ui-react";
import { WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { useLocalPresenceActorIdV1, usePresenceRuntimeKeyV1 } from "../../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, clearLocalPresenceWindowViewV1, publishLocalPresenceWindowViewV1 } from "../../👕️canvas-presence/🟦️.ts";
import { type CanvasCamera, screenToWorldLogical } from "../📷️camera-math/🟦️.ts";
import type { CanvasLayerRecord } from "../🖍️layer-paint/🟦️.ts";
import { JsonLayersCanvasSession } from "../🗂️layers-session/🟦️.ts";
import { CAMERA_SYNC_DEBOUNCE_MS } from "../🟢️shared/🟦️.tsx";

/** 🔷 Builds scene, windowInstanceId, emptySceneLabel, initialCamera of Canvas2dHost. */
export function useSceneState({ node, onAction }: { readonly node: import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83D\uDFE6\uFE0F").UiComponentSceneNode; readonly onAction: (action: import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDEC2\uFE0Fmanifest/\uD83D\uDFE6\uFE0F").ActionDescriptor, lifecycle?: import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83D\uDFE6\uFE0F").ActionDispatchLifecycleV1) => void | Promise<unknown> }) {
  const scene = node.canvas2d;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const initialCamera = useMemo(() => ({ x: scene?.cameraX ?? 0, y: scene?.cameraY ?? 0, zoom: scene?.zoom ?? 1 }), [scene?.cameraX, scene?.cameraY, scene?.zoom]);
  const cameraRef = useRef<CanvasCamera>(initialCamera);
  const framingRef = useRef(scene?.framing);
  framingRef.current = scene?.framing;
  const sessionRef = useRef<JsonLayersCanvasSession | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const cameraSyncTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dragOverStateRef = useRef<{ x: number; y: number; time: number } | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const [viewportSize, setViewportSize] = useState({ width: 0, height: 0 });
  const [presenceCamera, setPresenceCamera] = useState<CanvasCamera>(initialCamera);
  const presenceRuntimeKey = usePresenceRuntimeKeyV1();
  const presenceWindowId = windowInstanceId ?? node.surfaceId ?? "canvas";
  const localPresenceActor = useLocalPresenceActorIdV1(presenceRuntimeKey);
  const presencePointerRef = useRef<readonly [number, number] | null>(null);
  const presencePublishAtRef = useRef(0);
  /** 👕️ Publishes this window's camera and the pointer in WORLD coordinates for the next presence beat (throttled), so a
   * peer paints the cursor at the same drawing position whatever its own camera. */
  const publishCanvasPresenceView = useCallback((force: boolean) => {
    const now = Date.now();
    if (!force && now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const rect = containerRef.current?.getBoundingClientRect();
    const camera = cameraRef.current;
    const pointer = presencePointerRef.current;
    if (!presenceRuntimeKey) return;
    publishLocalPresenceWindowViewV1(presenceRuntimeKey, presenceWindowId, {
      windowId: presenceWindowId,
      space: "canvas",
      kind: { kind: "canvas", x: camera.x, y: camera.y, zoom: camera.zoom },
      size: [rect?.width ?? 0, rect?.height ?? 0],
      ...(pointer ? { pointer: [pointer[0], pointer[1], 0] as const } : {}),
    });
  }, [presenceWindowId, presenceRuntimeKey]);
  useEffect(() => {
    const timer = setTimeout(() => publishCanvasPresenceView(true), PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS);
    return () => clearTimeout(timer);
  }, [publishCanvasPresenceView, presenceCamera]);
  useEffect(() => presenceRuntimeKey ? () => clearLocalPresenceWindowViewV1(presenceWindowId, presenceRuntimeKey) : undefined, [presenceWindowId, presenceRuntimeKey]);
  const onPresencePointerMove = useCallback((event: ReactPointerEvent<HTMLDivElement>) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;
    const world = screenToWorldLogical(event.clientX - rect.left, event.clientY - rect.top, cameraRef.current, rect.width, rect.height);
    presencePointerRef.current = [world.x, world.y];
    publishCanvasPresenceView(false);
  }, [publishCanvasPresenceView]);
  const onPresencePointerLeave = useCallback(() => {
    presencePointerRef.current = null;
    publishCanvasPresenceView(true);
  }, [publishCanvasPresenceView]);
  const activeUtility = useMemo(() => {
    try {
      const layers = JSON.parse(scene?.layersJson ?? "[]") as readonly { readonly role?: string; readonly utility?: string }[];
      return layers.find((layer) => layer.role === "meta")?.utility;
    } catch {
      return undefined;
    }
  }, [scene?.layersJson]);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.canvas");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void | Promise<unknown> => {
      const ownedArgs = { surfaceId: node.surfaceId, ...args };
      return onAction({ controllerId: node.controllerId, action, args: ownedArgs });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const sessionFactory = useMemo(() => {
    return () => {
      const session = new JsonLayersCanvasSession(
        () => layersJsonRef.current ?? "[]",
        cameraRef.current,
        (next) => {
          cameraRef.current = next;
          sessionRef.current?.updateCamera(next);
          setPresenceCamera(next);
          publishCanvasPresenceView(false);
          if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
          cameraSyncTimeoutRef.current = setTimeout(() => dispatch("setCamera", { camera: next }), CAMERA_SYNC_DEBOUNCE_MS);
        },
        (action, args) => dispatch(action, args),
        () => framingRef.current,
      );
      sessionRef.current = session;
      return session;
    };
  }, [dispatch, publishCanvasPresenceView]);

  useEffect(() => {
    return () => {
      if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
      cameraSyncTimeoutRef.current = null;
    };
  }, [dispatch]);

  useEffect(() => {
    sessionRef.current?.syncLayersJson();
  }, [scene?.layersJson, scene?.framing]);

  const layersJsonRef = useRef(scene?.layersJson);
  layersJsonRef.current = scene?.layersJson;
  const layersCacheRef = useRef<{ readonly json: string; readonly layers: readonly CanvasLayerRecord[] } | null>(null);

  /** 🐢️ `layersJson` can be sizeable and the resolver is called every animation frame while a
   * demonstration targets this surface — cache the parse, keyed by the source string's identity, so a
   * re-render with the same layers never re-parses. */
  const readLayers = useCallback((): readonly CanvasLayerRecord[] => {
    const json = layersJsonRef.current ?? "[]";
    const cached = layersCacheRef.current;
    if (cached && cached.json === json) return cached.layers;
    let layers: readonly CanvasLayerRecord[] = [];
    try {
      const parsed = JSON.parse(json) as unknown;
      layers = Array.isArray(parsed) ? (parsed as CanvasLayerRecord[]) : [];
    } catch {
      layers = [];
    }
    layersCacheRef.current = { json, layers };
    return layers;
  }, []);
  return { windowInstanceId, containerRef, cameraRef, readLayers, dragOverStateRef, dispatch, mapContextMenu, shellContextMenuFallback, setContextMenu, setViewportSize, scene, emptySceneLabel, onPresencePointerMove, onPresencePointerLeave, sessionFactory, activeUtility, viewportSize, presenceRuntimeKey, presenceWindowId, localPresenceActor, presenceCamera, contextMenuTitleLabel, contextMenu };
}
