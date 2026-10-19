import type { InteractionTarget } from "@semio-tech/framework";
/** 🏗️ World3dHost brush meshes and menus: the brush mesh queue and registration, the mesh re-upload, the frame and zoom handlers, the context menu dispatch and the suggestion rows and submenu search. */
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Box3, Vector3 } from "@semio-tech/ui-react";
import {
  createCoalescingActionDispatcher,
  type Puzzle3dBrushMeshPage,
  puzzle3dAnnounceableBrushMeshUrls,
  puzzle3dBrushMeshDigest,
  puzzle3dBrushMeshPages,
  drainPuzzle3dBrushMeshQueue,
  PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES,
  puzzle3dBrushMeshRegistry
} from "../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { suggestionRowsWithFocus, useSuggestionSubmenuSearch } from "../../🎣️suggestion-submenu/🟦️.ts";
import { world3dBoundsRadius, world3dFrameCameraFromBounds, world3dFrameCameraFromInstances } from "../🔭️scene-schema/🟦️.ts";
import { world3dRetainLocalVortexHover } from "../🔗️instance-selection/🟦️.ts";
import { suggestionMenuItems } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { useShellContextMenuFallback, useMapContextMenuSpecs } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import type { WorldCore2 } from "../🧭️fit-and-presence/🟦️.ts";

// #region World3dHost
/** 🏗️ Runs the brush mesh queue and builds the world context menu and suggestion rows. */
export function useWorldBrushMeshAndMenus(core: WorldCore2) {
  const { adoptViewportCamera, autoFitBounds, brushMode, cameraState, contextMenu, dispatch, dispatchSettled, hostRef, instances, instancesGroupRef, interaction, meshes, node, noteUserMovedCamera, onAction, selection, setConnectDragHoverPosition, setConnectDragSource, setSuggestionFocus, setVortexPointerArm, suggestionMenuCheckingPlacementLabel, suggestionMenuNoPlacementLabel, suggestionMenuOwnsThisWindow, windowInstanceId } = core;
  // 🥽️ One GLB's collision geometry never fitted a single retained command: the shared puzzle contract
  // admits 8 192 raw JSON bytes and the Concrete Forest mesh alone encoded to 64 KB, so every upload is
  // a page run (`puzzle3dBrushMeshPages`) — the plugin stages the pages
  // under `(url, digest)` and installs the mesh when the last one lands. A mesh THIS GUEST already holds
  // is re-announced by `{url, digest}` alone, which it serves from its own content-addressed store
  // instead of taking the bytes again.
  //
  // 🚚️ Which meshes that is is `puzzle3dBrushMeshRegistry`'s to say, not this component's: the claim is
  // scoped to the guest instantiation that justified it, confirmed only when a run's LAST page goes out,
  // and voided by a fallen `meshResidency`. The claim used to live in a bare page-lifetime `Map` written
  // at enqueue time, so every activation after a guest restart re-announced seven identities the guest
  // held nothing for and the brush utility silently kept no collision geometry until a full reload.
  //
  // ⏳️ The run is a BACK-PRESSURED stream, never a burst. A macrotask drain (`setTimeout(drain, 0)`) put
  // every page of every mesh into the actor's one command queue within a couple of hundred milliseconds
  // and then left them there: browser-measured 2026-09-12 on wasm #47, 202 pages enqueued in 21 s, 40 of
  // them settled over the next 420 s, and ONE user click that landed mid-run waited 44.3 s behind 15
  // pages (`📓️2026-09-12-wave-B22-brush-mesh-upload.md`). Awaiting each page's own settle before the
  // next is queued bounds what a user action can ever queue behind at ONE page, whatever the run's
  // length — `onAction` settles on the dispatched action's typed-operation completion, which is exactly
  // the "this page is in" signal the old fire-and-forget shape threw away.
  //
  // 🪢️ And a run is only ever paged ONCE PER DIGEST. Every `dist/mesh/*.glb` in this repo is the same
  // 771 728-byte capsule, so a scene placing several object kinds paged byte-identical geometry once per
  // mesh id — 72 commands each. A digest this page already put into THIS guest is announced by
  // `{url, digest}` alone and the guest aliases its own derived page onto the new id
  // (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`); a guest that cannot serve it answers
  // on `meshReuploadUrls` and the claim is re-driven, so an optimistic announcement is self-healing.
  const brushMeshQueueRef = useRef<Puzzle3dBrushMeshPage[]>([]);
  const brushMeshRunsRef = useRef(new Map<string, string>());
  const brushMeshDrainingRef = useRef(false);
  const brushMeshLiveRef = useRef(true);
  const [brushMeshRevisions, setBrushMeshRevisions] = useState<{ readonly generation: number; readonly urls: Readonly<Record<string, number>> }>({ generation: 0, urls: {} });
  /** 🥽️ `dispatch`'s awaitable twin for the one lane that needs back pressure — same envelope, but the
   * caller can wait for the page to be IN before it queues the next one. */
  const dispatchBrushMesh = useCallback(
    (args: Record<string, unknown>): Promise<unknown> =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action: "registerBrushMesh",
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );
  const drainBrushMeshQueue = useCallback(async () => {
    if (brushMeshDrainingRef.current) return;
    brushMeshDrainingRef.current = true;
    try {
      await drainPuzzle3dBrushMeshQueue(
        brushMeshQueueRef.current,
        {
          holdsDigest: (digest) => puzzle3dBrushMeshRegistry.holdsDigest(digest),
          mayAlias: (url) => puzzle3dBrushMeshRegistry.mayAlias(url),
          alias: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.alias(url, digest);
          },
          confirm: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.confirm(url, digest);
          },
        },
        dispatchBrushMesh,
        () => brushMeshLiveRef.current,
      );
    } finally {
      brushMeshDrainingRef.current = false;
    }
  }, [dispatchBrushMesh]);
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      const digest = puzzle3dBrushMeshDigest(positions, indices);
      if (puzzle3dBrushMeshRegistry.holds(url, digest)) {
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (puzzle3dBrushMeshRegistry.mayAlias(url) && puzzle3dBrushMeshRegistry.holdsDigest(digest)) {
        puzzle3dBrushMeshRegistry.alias(url, digest);
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (brushMeshRunsRef.current.get(url) === digest) return;
      const pages = puzzle3dBrushMeshPages(url, node.surfaceId, positions, indices);
      if (pages.length === 0 || brushMeshQueueRef.current.length + pages.length > PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES) return;
      brushMeshRunsRef.current.set(url, digest);
      brushMeshQueueRef.current.push(...pages);
      void drainBrushMeshQueue();
    },
    [dispatchBrushMesh, drainBrushMeshQueue, node.surfaceId],
  );
  useEffect(
    () => () => {
      brushMeshLiveRef.current = false;
      for (const page of brushMeshQueueRef.current.splice(0)) {
        brushMeshRunsRef.current.delete(page.url);
        puzzle3dBrushMeshRegistry.forget(page.url);
      }
    },
    [],
  );
  // 🚚️ The guest's two published facts about what it actually holds, folded in before anything is
  // announced: a residency that fell proves a restarted guest (its mesh store is not part of any
  // checkpoint), which voids EVERY claim this page holds, and `meshReuploadUrls` names identities an
  // announcement already sent is waiting on bytes for. Either way the answer is the same — bump the
  // revision the mounted `BrushMeshRegistrar`s carry, so they re-announce their already-loaded GLB
  // and, with the claim gone, take the page path instead of the id-only one.
  const meshResidency = interaction.meshResidency;
  const meshReuploadUrls = interaction.meshReuploadUrls;
  useEffect(() => {
    if (meshResidency === undefined) return;
    const restarted = puzzle3dBrushMeshRegistry.observeResidency(meshResidency);
    const claimed = (meshReuploadUrls ?? []).filter((url) => puzzle3dBrushMeshRegistry.claimReupload(url));
    if (!restarted && claimed.length === 0) return;
    setBrushMeshRevisions((previous) => {
      const urls: Record<string, number> = { ...previous.urls };
      for (const url of claimed) urls[url] = (previous.urls[url] ?? 0) + 1;
      return { generation: previous.generation + (restarted ? 1 : 0), urls };
    });
  }, [meshResidency, meshReuploadUrls]);

  // 🥽️ Every mesh the scene lane names — placed kinds and catalogue kinds alike — is uploaded for collision, so
  // the brush suggestions run tests a candidate against its real geometry.
  //
  // 🚪️ …but only for a guest that OWNS that lane — see `puzzle3dAnnounceableBrushMeshUrls`.
  const brushMeshUrls = useMemo(() => puzzle3dAnnounceableBrushMeshUrls(meshResidency, meshes), [meshResidency, meshes]);

  const handleFrameVisibleInstances = useCallback(() => {
    noteUserMovedCamera();
    const aspect = hostRef.current ? Math.max(hostRef.current.clientWidth, 1) / Math.max(hostRef.current.clientHeight, 1) : 1;
    if (autoFitBounds) {
      const [minimum, maximum] = autoFitBounds;
      const center: [number, number, number] = [((minimum[0] ?? 0) + (maximum[0] ?? 0)) * 0.5, ((minimum[1] ?? 0) + (maximum[1] ?? 0)) * 0.5, ((minimum[2] ?? 0) + (maximum[2] ?? 0)) * 0.5];
      adoptViewportCamera(world3dFrameCameraFromBounds(center, world3dBoundsRadius(minimum, maximum), cameraState, undefined, aspect), true);
      return;
    }
    const group = instancesGroupRef.current;
    if (group) {
      const box = new Box3().setFromObject(group);
      if (!box.isEmpty()) {
        const center = box.getCenter(new Vector3());
        const radius = world3dBoundsRadius([box.min.x, box.min.y, box.min.z], [box.max.x, box.max.y, box.max.z]);
        if (radius > 1e-6) {
          adoptViewportCamera(world3dFrameCameraFromBounds([center.x, center.y, center.z], radius, cameraState, undefined, aspect), true);
          return;
        }
      }
    }
    adoptViewportCamera(world3dFrameCameraFromInstances(instances, cameraState), true);
  }, [adoptViewportCamera, autoFitBounds, noteUserMovedCamera, cameraState, instances]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) {
      handleFrameVisibleInstances();
      return;
    }
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) {
      handleFrameVisibleInstances();
      return;
    }
    let centerX = 0;
    let centerY = 0;
    let centerZ = 0;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      centerX += position[0];
      centerY += position[1];
      centerZ += position[2];
    }
    const count = selected.length;
    centerX /= count;
    centerY /= count;
    centerZ /= count;
    let maxDistance = 1;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const dx = position[0] - centerX;
      const dy = position[1] - centerY;
      const dz = position[2] - centerZ;
      maxDistance = Math.max(maxDistance, Math.hypot(dx, dy, dz));
    }
    const distance = maxDistance * 3 + 2;
    adoptViewportCamera(
      {
        position: [centerX + distance * 0.6, centerY - distance * 0.6, centerZ + distance * 0.5],
        target: [centerX, centerY, centerZ],
        zoom: cameraState.zoom,
        up: cameraState.up,
        projection: cameraState.projection,
      },
      true,
    );
  }, [adoptViewportCamera, cameraState.projection, cameraState.up, cameraState.zoom, handleFrameVisibleInstances, instances, selection.ids]);

  const handleWorldMenuDispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (action === "zoomToSelection") {
        handleZoomToSelection();
        return;
      }
      dispatch(action, args);
    },
    [dispatch, handleZoomToSelection],
  );

  const mapWorldContextMenuSpecs = useMapContextMenuSpecs(handleWorldMenuDispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const mapSuggestionContextMenuSpecs = useMapContextMenuSpecs((action, args) => {
    if (action === "acceptSuggestion") {
      setVortexPointerArm(null);
      setConnectDragSource(null);
      setConnectDragHoverPosition(null);
    }
    dispatch(action, args);
  });
  useSuggestionSubmenuSearch(contextMenu?.suggestTarget ?? null, { x: contextMenu?.x ?? 0, y: contextMenu?.y ?? 0 }, windowInstanceId ?? undefined, dispatch, () => setSuggestionFocus(null));
  useEffect(() => {
    if (!suggestionMenuOwnsThisWindow) setSuggestionFocus(null);
  }, [suggestionMenuOwnsThisWindow]);
  /** 📂️ The candidate rows one suggestion menu lists — the context menu's "suggest" submenu and the floating
   * popup share them. A submenu whose search has not echoed back yet lists the pending row. */
  const suggestionRows = useMemo(() => {
    const menu = interaction.suggestionMenu?.open && suggestionMenuOwnsThisWindow ? interaction.suggestionMenu : null;
    const listed = menu ?? { pending: true, candidates: [] };
    const rows = mapSuggestionContextMenuSpecs(
      suggestionMenuItems(listed, interaction.brushCandidateIndex ?? 0, { checkingPlacement: suggestionMenuCheckingPlacementLabel, noPlacement: suggestionMenuNoPlacementLabel }),
    );
    return suggestionRowsWithFocus(rows, listed, setSuggestionFocus);
  }, [interaction.brushCandidateIndex, interaction.suggestionMenu, mapSuggestionContextMenuSpecs, suggestionMenuCheckingPlacementLabel, suggestionMenuNoPlacementLabel, suggestionMenuOwnsThisWindow]);

  const hoveredVortexFullIdRef = useRef<string | null>(null);
  /** 🎯️ The interaction target this pane's OWN raycast last resolved under the pointer — see
   * {@link hoveredInteractionTargetV1}. Null whenever the pointer left every instance. */
  const hoveredInstanceInteractionTargetRef = useRef<InteractionTarget | null>(null);
  const altHeldRef = useRef(false);
  useEffect(() => {
    hoveredVortexFullIdRef.current = world3dRetainLocalVortexHover(hoveredVortexFullIdRef.current, interaction.hoveredVortexFullId);
  }, [interaction.hoveredVortexFullId]);
  /** 🎣️ The armed brush points the read-only brush suggestions run at the vortex under the pointer, and at
   * nothing once it leaves every vortex or the brush is disarmed. Only a CHANGE travels, through one
   * single-flight lane (at most one `targetBrushSuggestions` outstanding plus the latest follow-up), so a hover
   * storm never queues guest turns; the command only writes the target, and the run's own trace lane paints
   * every candidate it tests. */
  const brushSuggestionsTarget = brushMode ? interaction.hoveredVortexFullId ?? null : null;
  const brushSuggestionsTargetSentRef = useRef<string | null>(null);
  const sendBrushSuggestionsTarget = useMemo(() => createCoalescingActionDispatcher<string | null>((fullId) => dispatchSettled("targetBrushSuggestions", fullId ? { fullId } : {})), [dispatchSettled]);
  useEffect(() => {
    if (brushSuggestionsTargetSentRef.current === brushSuggestionsTarget) return;
    brushSuggestionsTargetSentRef.current = brushSuggestionsTarget;
    sendBrushSuggestionsTarget(brushSuggestionsTarget);
  }, [brushSuggestionsTarget, sendBrushSuggestionsTarget]);
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Alt" || event.key === "AltGraph") altHeldRef.current = event.type === "keydown";
    };
    window.addEventListener("keydown", onKey, true);
    window.addEventListener("keyup", onKey, true);
    window.addEventListener("blur", () => {
      altHeldRef.current = false;
    });
    return () => {
      window.removeEventListener("keydown", onKey, true);
      window.removeEventListener("keyup", onKey, true);
    };
  }, []);
  return { altHeldRef, brushMeshRevisions, brushMeshUrls, handleRegisterBrushMesh, hoveredInstanceInteractionTargetRef, hoveredVortexFullIdRef, mapWorldContextMenuSpecs, shellContextMenuFallback, suggestionRows };
}

export type WorldCore3 = WorldCore2 & ReturnType<typeof useWorldBrushMeshAndMenus>;
// #endregion World3dHost
