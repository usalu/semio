// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/World3dHost/component.tsx
/** 🌐️ `World3dHost` — the 3D world viewport scene host: mesh/instance parsing, point-cloud and vortex-marker
 * layers, catalogue-drop and selection-preview stores, instance chrome, the transform gumball, and the full R3F
 * `World3dHost` component mounted inside a Mode window. The component body is split into seven hooks that
 * share one typed `core` bag (`🌍️scene-lanes`, `🧭️fit-and-presence`, `🏗️brush-mesh-and-menus`,
 * `👇️hover-and-placement`, `🔀️paint-and-projection`, `🎛️marquee-and-gumball`, `👆️pointer-and-drop`); this
 * file keeps the scene-less fallback and the JSX. */
// #endregion 🧲️Header

import { Suspense } from "react";

// #region 🔌️Adapters
import { cn, ContextMenuController, glassClass, IconShotFrame, SelectionMarquee, sunPositionFromAzimuthElevation, chromePanelSafeAreaStyle, windowChromeClearedTopOffset } from "@semio-tech/ui-react";
import { toolRunTraceDataAttributes } from "./⏯️tool-run-trace/🟦️.tsx";
import { World3dAnnotationOverlay, World3dAnnotationProjector, World3dScalarLegendView, World3dSectionClip, world3dModellingDataAttributes, world3dModellingStrings } from "./📏️modelling/🟦️.tsx";
import { type ComponentSceneHostProps, type InteractionTarget,
  type MergeMode
} from "@semio-tech/framework";
import { DEFAULT_LOD_GRID_FACTOR, DEFAULT_MANUAL_LOD, WorldCanvas, WorldLodBridge, WorldOrbitGated, WorldOrbitViewControls, WorldOrbitViewSnapGateProvider, WorldProjectionRig, WorldReferenceLayer, WorldVolumeLayer } from "@semio-tech/infinite-world-r3f";

import { WorldTerrainLayer } from "../🗺️WorldTerrainLayer/🟦️.tsx";
import { world3dMarqueeOverlayShape } from "../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { NOTE_WORLD_NAVIGATION_ACTION_ID } from "../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";

import { suggestionSubmenuTarget, withSuggestionSubmenu } from "../🎣️suggestion-submenu/🟦️.ts";
import { CanvasPresenceOverlayV1, usePresenceRuntimeKeyV1 } from "../👕️canvas-presence/🟦️.tsx";

// #endregion 🔌️Adapters

import { isTransparentWorldBackground, WORLD3D_FRAME_BOUNDS_MARGIN, world3dCameraDomJson, world3dSunDomJson } from "./🔭️scene-schema/🟦️.ts";
import { WorldAcceptedFrameDiagnostics, WorldAutoFit, WorldProjectionContentFrame } from "./🎨️mesh-render/🟦️.tsx";
import { BrushMeshRegistrar } from "./🧩️glb-surface/🟦️.tsx";
import { resolveWorldContextMenuTarget, world3dContextMenuSurfaceV1, world3dSuggestionsAltHeld, world3dSuggestionsGestureArmed, world3dSuggestionsGestureConsumesContextMenu, worldInstancePickBlocked, worldSurfaceGuestSelectionDomV1, worldSurfaceSelectionDomV1 } from "./🔗️instance-selection/🟦️.ts";
import { isWorldTransformGumballMode } from "./🧮️gumball-math/🟦️.ts";
import { WorldInstancesLayer } from "./🧱️instance-layers/🟦️.tsx";
import { WorldPointCloudLayer } from "./🧱️instance-layers/☁️point-cloud-layer/🟦️.tsx";
import { WorldVortexMarkers } from "./🧱️instance-layers/🌪️vortex-markers/🟦️.tsx";
import { CameraRefBridge, EngagementPreviewLayer, IntroductionWorldResolverBridge, RaycasterPickTuning, WorldAttractionLines, WorldComputeStatusPane, WorldConnectRubberBand, WorldGumballHitStamp, WorldSurfaceStatusPane, WorldVortexHitStamp, WorldVoxelPreviewBox } from "./🎯️marquee-and-bridges/🟦️.tsx";
import { CatalogueDropGhost, CatalogueDropPreviewInvalidate } from "./🗂️catalogue-drop/🟦️.tsx";
import { WorldOrbitProjectionSwitchPane } from "./🪟️window-view-store/🟦️.tsx";
import { openSurfaceContextMenu } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";

import { WorldToolRunTrace } from "./🛤️tool-run-trace-lane/🟦️.tsx";

import { useWorldSceneLanes } from "./🌍️scene-lanes/🟦️.ts";
import { useWorldFitAndPresence } from "./🧭️fit-and-presence/🟦️.ts";
import { useWorldBrushMeshAndMenus } from "./🏗️brush-mesh-and-menus/🟦️.ts";
import { useWorldHoverAndPlacement } from "./👇️hover-and-placement/🟦️.ts";
import { useWorldPaintAndProjection } from "./🔀️paint-and-projection/🟦️.ts";
import { useWorldMarqueeAndGumball } from "./🎛️marquee-and-gumball/🟦️.ts";
import { useWorldPointerAndDrop, type WorldCore7 } from "./👆️pointer-and-drop/🟦️.ts";

//#region 🔖️World3dHost
//#region World3dHost
/** 🚚️ `node.world3d` arrives ASSEMBLED. Its payload fields (`instancesJson`, `vorticesJson`, … — see
 * `WORLD3D_SCENE_LANES`) no longer ride inside the surface doc's 32 KiB `UiFixedBytes` blob, which
 * cannot page and refused a 57 KB Nakagin world outright; each is published as its own retained,
 * individually paged text carrier beside the surface node and reattached by the Interpreter's
 * `PagedSurfaceView` before this host ever sees it (ticket 26/09/02 wave P). Every `*Json` field is
 * therefore still a plain string here, and the per-lane `useMemo`s below stay the incremental seam:
 * a lane whose content did not change keeps its identical string, so its parse never re-runs. The three
 * modelling lanes (`annotations`, `scalarField`, `modellingOptions`) arrive already typed (`encoding: "json"`)
 * and keep their identity by lane hash (`useStableWorld3dLane`). */
export function World3dHost(props: ComponentSceneHostProps) {
  const core = {} as WorldCore7;
  Object.assign(core, useWorldSceneLanes(props));
  Object.assign(core, useWorldFitAndPresence(core));
  Object.assign(core, useWorldBrushMeshAndMenus(core));
  Object.assign(core, useWorldHoverAndPlacement(core));
  Object.assign(core, useWorldPaintAndProjection(core));
  Object.assign(core, useWorldMarqueeAndGumball(core));
  Object.assign(core, useWorldPointerAndDrop(core));
  const { activeUtility, altHeldRef, publishWorldPresenceView, annotationLayer, annotationStore, attractions, autoFitBounds, autoFitKey, autoFitUserMoved, brushMeshRevisions, brushMeshUrls, brushMode, cameraRef, cameraSeedKey, cameraState, catalogueDropPreview, colors, computeStatus, connectDragHoverPosition, connectDragSource, contentBounds, contextMenu, contextMenuOpenEpochRef, contextMenuTitleLabel, dismissContextMenu, dispatch, emptySceneLabel, engagementPreview, environment, externalPendingProjectionSpec, faceDragSession, fit, fitProjectionContent, frame, geometryRevisionRef, gridFactor, gridSnapEnabled, gumballDragActive, gumballLiveDispatch, gumballTransformPreviewSourceId, handleAutoFitCameraChange, handleBrushPlace, handleCameraChange, handleCameraNavigate, handleComponentHover, handleConnectDragDrop, handleConnectDragHover, handleEmptyClick, handleFaceDragStart, handleGizmoCameraChange, handleGumballDrag, handleGumballDragEnd, handleGumballDragStart, handleInstancePointerDown, handleInstancePointerMove, handlePaintAt, handlePointerDown, handlePointerMove, handlePointerUp, handleProjectionContentFrame, handleProjectionKindChange, handleReferenceHover, handleReferenceSelect, handleRegisterBrushMesh, handleSuggestionClose, handleTargetVolumeRelocate, handleTargetVolumeSelect, handleVortexHover, handleVortexPointerArm, handleVortexPointerMove, handleVortexPointerUp, handleVortexSelect, handleWorldOrbitRightPointerDown, handleWorldPick, heatmap, hostRef, hoveredVortexFullIdRef, instances, instancesGroupRef, interaction, interactionDomainId, localPresenceActor, lod, lodRef, mapWorldContextMenuSpecs, marqueeCoverage, marqueeDown, marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, meshStylePalette, meshVisuals, meshes, method, modellingOptions, node, onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop, overlayRailRef, overlayRailSafeArea, paintMode, pendingProjectionSpec, pendingProjectionSpecRef, persistentSelectionMode, presenceObjectWorldPositions, presenceWindowId, previewAttractions, previewVortices, projectionContentFrameMounted, projectionFramePending, publishHoverPaint, referenceHoveredId, referenceSelectedIds, references, releaseLocalHover, requestContextMenu, scalarField, scene, sceneCamera, sectionCapHex, sectionExtent, selection, selectionMode, setContextMenu, setExternalPendingProjectionSpec, setGumballDragActive, setPendingProjectionSpec, sharedGumballTransformPreview, shellContextMenuFallback, shellScope, subElementPaint, subscribeLocalHoverRelease, suggestionFocus, suggestionMenuTitleLabel, suggestionPopupOwnsThisWindow, suggestionRows, suggestionsRightDownDispatchedAtRef, targetVolumeSelectedIds, targetVolumes, toolRunTrace, visibleSelectionPreview, volumeBrushMode, volumeGumballConfig, volumeLayersInteractive, vortices, voxelHoverOrigin, windowInstanceId, worldOrbitConstraints, worldProjectionSpec , presenceRuntimeKey } = core;

  if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={hostRef}
      className="semio-world-3d-host relative h-full min-h-0 w-full touch-none overflow-hidden"
      data-surface-id={node.surfaceId}
      data-window-instance-id={windowInstanceId || undefined}
      data-selection-json={JSON.stringify(worldSurfaceSelectionDomV1(selection, interaction))}
      data-guest-selection-json={JSON.stringify(worldSurfaceGuestSelectionDomV1(scene.selectionJson))}
      data-orbit-view-gizmo=""
      data-puzzle3d-fixture-drag-active={catalogueDropPreview ? "" : undefined}
      data-meshes-json={scene.meshesJson ?? undefined}
      data-world-geometry-revision={geometryRevisionRef.current.revision}
      data-instances-json={scene.instancesJson ?? undefined}
      data-target-volumes-json={scene.targetVolumesJson ?? undefined}
      data-engagement-preview-json={scene.engagementPreviewJson ?? undefined}
      data-camera-json={world3dCameraDomJson(sceneCamera)}
      data-viewport-camera-json={world3dCameraDomJson(cameraState)}
      data-vortices-json={scene.vorticesJson ?? undefined}
      data-suggestion-menu-json={interaction.suggestionMenu ? JSON.stringify(interaction.suggestionMenu) : ""}
      data-suggestion-focus={suggestionFocus === null ? undefined : String(suggestionFocus)}
      data-interaction-json={JSON.stringify(interaction)}
      data-status-json={scene.statusJson ?? undefined}
      data-sun-json={world3dSunDomJson(environment)}
      {...world3dModellingDataAttributes({ pickFilter: modellingOptions?.pickFilter, section: modellingOptions?.section, scalarField, scalarStatus: heatmap.status, annotationCount: annotationLayer?.items.length })}
      {...toolRunTraceDataAttributes(toolRunTrace.store)}
      onContextMenu={(event) => {
        const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
        const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
        if (world3dSuggestionsGestureConsumesContextMenu(alt) || brushArmed) {
          event.preventDefault();
          event.stopPropagation();
          if (performance.now() - suggestionsRightDownDispatchedAtRef.current < 80) return;
          if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
            dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
          }
          return;
        }
        if (!requestContextMenu) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        event.preventDefault();
        event.stopPropagation();
        const epoch = contextMenuOpenEpochRef.current + 1;
        contextMenuOpenEpochRef.current = epoch;
        void (async () => {
          const surface = world3dContextMenuSurfaceV1(target, selection);
          const suggest = { target: null as string | null };
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "world3d", args: null },
              surface: { surfaceId: node.surfaceId, kind: "world3d", hits: [...surface.hits], selection: surface.selection.map((group) => ({ domain: group.domain, ids: [...group.ids] })) },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => {
              suggest.target = suggestionSubmenuTarget(specs);
              return mapWorldContextMenuSpecs(specs);
            },
            shellContextMenuFallback,
          );
          if (contextMenuOpenEpochRef.current !== epoch) return;
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu, suggestTarget: suggest.target });
        })();
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      onPointerLeave={() => { releaseLocalHover(); publishWorldPresenceView(null); }}
      onDragEnter={onCatalogueDragEnter}
      onDragLeave={onCatalogueDragLeave}
      onDragOver={onCatalogueDragOver}
      onDrop={onCatalogueDrop}
    >
      <WorldCanvas
        className="h-full w-full"
        cameraUp={(cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1]}
        cameraFov={cameraState.fov}
        background={environment && !isTransparentWorldBackground(environment.background) ? environment.background : undefined}
        gl={environment && isTransparentWorldBackground(environment.background) ? { antialias: true, alpha: true, stencil: true } : undefined}
        shadows={environment?.shadow?.enabled === true ? true : undefined}
        onPointerMissed={handleEmptyClick}
        overlay={
          <>
            {frame ? <IconShotFrame width={frame.width} height={frame.height} shape={frame.shape === "ellipse" ? "ellipse" : "rectangle"} badge={frame.badge !== false} background={frame.background} /> : null}
            {annotationLayer ? <World3dAnnotationOverlay layer={annotationLayer} store={annotationStore} locale={shellScope?.i18n.language} /> : null}
            {scalarField && heatmap.status === "applied" ? <World3dScalarLegendView field={scalarField} locale={shellScope?.i18n.language} className={cn("pointer-events-none absolute bottom-2 left-2 z-40 rounded px-single py-half text-xs shadow-sm", glassClass)} /> : null}
            {scalarField && heatmap.status === "mismatch" ? (
              <div role="status" data-slot="world-scalar-field-mismatch" className={cn("pointer-events-none absolute bottom-2 left-2 z-40 rounded px-single py-half text-xs shadow-sm", glassClass)}>
                {world3dModellingStrings(shellScope?.i18n.language).mismatch}
              </div>
            ) : null}
            <WorldOrbitProjectionSwitchPane spec={worldProjectionSpec} onSpecChange={handleProjectionKindChange} windowElementSegment={windowInstanceId ?? node.surfaceId} />
            <div
              ref={overlayRailRef}
              data-slot="world-view-overlay-rail"
              data-safe-area-block={overlayRailSafeArea.blockPx || undefined}
              data-safe-area-inline={overlayRailSafeArea.inlinePx || undefined}
              className="pointer-events-none absolute z-40 flex flex-col items-end gap-single"
              style={chromePanelSafeAreaStyle("top-right", overlayRailSafeArea, { block: windowChromeClearedTopOffset })}
            >
              <WorldSurfaceStatusPane visuals={meshVisuals} locale={shellScope?.i18n.language} glassClass={glassClass} />
              <WorldComputeStatusPane status={computeStatus} glassClass={glassClass} locale={shellScope?.i18n.language} onCancel={() => dispatch(computeStatus.cancelAction, computeStatus.cancelArgs)} />
            </div>
            <CanvasPresenceOverlayV1
              runtimeKey={presenceRuntimeKey}
              windowId={presenceWindowId}
              space="world"
              myActor={localPresenceActor ?? ""}
              locale={shellScope?.i18n.language}
              localOrbit={{
                position: cameraState.position as [number, number, number],
                target: cameraState.target as [number, number, number],
                up: (cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1],
                fov: cameraState.fov ?? 45,
              }}
              localSizePx={hostRef.current ? [hostRef.current.clientWidth, hostRef.current.clientHeight] : [1, 1]}
              objectWorldPositions={presenceObjectWorldPositions}
              scenePath={`world/${presenceWindowId}`}
              domain={interactionDomainId}
              selectedIds={selection.ids}
            />
          </>
        }
      >
        <WorldOrbitViewSnapGateProvider>
          <WorldProjectionRig spec={worldProjectionSpec} state={cameraState} seedKey={cameraSeedKey} pendingSpec={pendingProjectionSpec} />
          {projectionContentFrameMounted ? (
            <WorldProjectionContentFrame
              enabled={fitProjectionContent || projectionFramePending}
              spec={pendingProjectionSpecRef.current ?? worldProjectionSpec}
              bounds={contentBounds}
              fov={cameraState.fov}
              onFramed={handleProjectionContentFrame}
            />
          ) : null}
          <WorldOrbitGated
            controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}
            onCamera={handleCameraChange}
            onCameraNavigate={handleCameraNavigate}
            zoom={cameraState.zoom}
            projection={cameraState.explicitProjection ? cameraState.projection : undefined}
            constraints={worldOrbitConstraints}
            onRightPointerDown={handleWorldOrbitRightPointerDown}
            onNavigationGestures={(gestures) => dispatch(NOTE_WORLD_NAVIGATION_ACTION_ID, { windowId: windowInstanceId ?? node.surfaceId, gestures })}
          />
          <WorldOrbitViewControls
            projectionSpec={worldProjectionSpec}
            externalPendingSpec={externalPendingProjectionSpec}
            onExternalPendingSpecClear={() => setExternalPendingProjectionSpec(null)}
            onCameraChange={handleGizmoCameraChange}
            onPendingSpecChange={setPendingProjectionSpec}
          />
          <WorldLodBridge
            lodRef={lodRef}
            distanceReference={100}
            gridFactor={lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR}
            gridSnapEnabled={lod.gridSnapEnabled ?? false}
            showLodGrid={lod.showLodGrid ?? true}
            automaticLod={lod.automaticLod ?? true}
            depthVariableLod={lod.depthVariableLod ?? false}
            manualLod={lod.manualLod ?? DEFAULT_MANUAL_LOD}
            gridDatum={[0, 0, 0]}
          >
            <WorldAcceptedFrameDiagnostics
              surfaceId={node.surfaceId}
              windowInstanceId={windowInstanceId}
              projection={worldProjectionSpec}
              cameraTarget={cameraState.target as [number, number, number]}
              contentBounds={contentBounds}
              references={references}
              lod={lod}
            />
            <ambientLight color={environment?.ambient?.color ?? "#ffffff"} intensity={environment?.ambient?.intensity ?? 1.15} />
            {environment?.sun?.enabled === true ? (
              <directionalLight
                color={environment.sun?.color ?? "#ffffff"}
                intensity={environment.sun?.intensity ?? 0.85}
                position={sunPositionFromAzimuthElevation(environment.sun?.azimuth ?? 45, environment.sun?.elevation ?? 35)}
                castShadow={environment.shadow?.enabled === true}
              />
            ) : (
              <>
                <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
                <directionalLight position={[12, 18, 10]} intensity={2.4} />
                <directionalLight position={[-14, -10, 6]} intensity={1.2} />
                <directionalLight position={[0, 0, -16]} intensity={0.75} />
              </>
            )}
            {fit?.enabled ? (
              <WorldAutoFit
                groupRef={instancesGroupRef}
                fitKey={autoFitKey}
                padding={fit.padding ?? WORLD3D_FRAME_BOUNDS_MARGIN}
                camera={cameraState}
                bounds={autoFitBounds}
                userMoved={autoFitUserMoved}
                onFitted={handleAutoFitCameraChange}
              />
            ) : null}
            <CameraRefBridge cameraRef={cameraRef} />
            <CatalogueDropPreviewInvalidate preview={catalogueDropPreview} />
            <RaycasterPickTuning />
            <WorldVortexHitStamp vortices={previewVortices} hostRef={hostRef} />
            <WorldGumballHitStamp target={selection.gumballTarget} active={Boolean(selection.gumballActive) && isWorldTransformGumballMode(selection.transformMode)} hostRef={hostRef} />
            {windowInstanceId ? (
              <IntroductionWorldResolverBridge windowInstanceId={windowInstanceId} vortices={vortices} instances={instances} attractions={attractions} />
            ) : null}
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} revision={brushMeshRevisions.generation + (brushMeshRevisions.urls[url] ?? 0)} onRegister={handleRegisterBrushMesh} />
              </Suspense>
            ))}
            <WorldTerrainLayer terrainJson={scene?.terrainJson} cameraPosition={cameraState.position} cameraTarget={cameraState.target} />
            <WorldPointCloudLayer pointsJson={scene?.pointsJson} />
            <group ref={instancesGroupRef}>
              <WorldInstancesLayer
                meshVisuals={meshVisuals}
                instances={instances}
                meshes={meshes}
                selection={selection}
                persistentSelectionMode={persistentSelectionMode}
                palette={meshStylePalette}
                projectionSpec={worldProjectionSpec}
                onInstancePointerDown={handleInstancePointerDown}
                onInstancePointerMove={handleInstancePointerMove}
                onLocalHoverRelease={subscribeLocalHoverRelease}
                onHoverPaint={publishHoverPaint}
                onWorldPick={handleWorldPick}
                onComponentHover={handleComponentHover}
                onPaintAt={paintMode ? handlePaintAt : undefined}
                gumballDragActive={gumballDragActive}
                controllerId={node.controllerId}
                gumballPreviewSourceId={gumballTransformPreviewSourceId}
                sharedGumballPreview={sharedGumballTransformPreview}
                onGumballDraggingChanged={setGumballDragActive}
                gumballLiveDispatch={gumballLiveDispatch}
                onGumballDragStart={handleGumballDragStart}
                onGumballDrag={handleGumballDrag}
                onGumballDragEnd={handleGumballDragEnd}
                onFaceDragStart={handleFaceDragStart}
                mergedComponentIds={visibleSelectionPreview.mergedComponentIds}
                mergedInstanceIds={visibleSelectionPreview.mergedInstanceIds}
                blockPick={worldInstancePickBlocked(activeUtility)}
                environment={environment}
                subElementPaint={subElementPaint}
              />
            </group>
            {annotationLayer ? <World3dAnnotationProjector layer={annotationLayer} store={annotationStore} /> : null}
            {modellingOptions?.section ? <World3dSectionClip section={modellingOptions.section} groupRef={instancesGroupRef} capHex={sectionCapHex} extent={sectionExtent} /> : null}
            <WorldVortexMarkers
              vortices={previewVortices}
              palette={meshStylePalette}
              brushMode={brushMode}
              selectionMode={selectionMode}
              connectSourceFullId={connectDragSource?.fullId}
              onHover={handleVortexHover}
              onVortexSelect={handleVortexSelect}
              onBrushPlace={handleBrushPlace}
              onVortexPointerArm={handleVortexPointerArm}
              onVortexPointerMove={handleVortexPointerMove}
              onVortexPointerUp={handleVortexPointerUp}
              onConnectDragHover={handleConnectDragHover}
              onConnectDragDrop={handleConnectDragDrop}
            />
            {connectDragSource && connectDragHoverPosition ? <WorldConnectRubberBand from={connectDragSource.position} to={connectDragHoverPosition} /> : null}
            <WorldAttractionLines attractions={previewAttractions} />
            {catalogueDropPreview ? <CatalogueDropGhost preview={catalogueDropPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            <WorldToolRunTrace lane={scene.toolRunTrace} meshes={meshes} trace={toolRunTrace} focus={suggestionFocus} />
            {engagementPreview.length > 0 ? <EngagementPreviewLayer items={engagementPreview} color={colors.hover} /> : null}
            <WorldVolumeLayer
              volumes={targetVolumes
                .filter((volume) => !volume.hidden)
                .map((volume) => ({
                  id: volume.id,
                  origin: volume.origin as [number, number, number],
                  orientation: volume.orientation as [number, number, number, number] | undefined,
                  scale: volume.scale,
                  color: volume.color,
                  hidden: volume.hidden,
                  locked: volume.locked,
                }))}
              selectedIds={targetVolumeSelectedIds}
              interactive={volumeLayersInteractive}
              gumballConfig={volumeGumballConfig}
              relocateActive={activeUtility === "transform"}
              translationSnap={gridSnapEnabled ? gridFactor : undefined}
              onSelect={handleTargetVolumeSelect}
              onRelocate={handleTargetVolumeRelocate}
            />
            {volumeBrushMode && voxelHoverOrigin ? <WorldVoxelPreviewBox origin={voxelHoverOrigin} dims={interaction.voxelDims ?? [1, 1, 1]} gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} /> : null}
            <WorldReferenceLayer
              references={references
                .filter((reference) => !reference.hidden)
                .map((reference) => ({
                  id: reference.id,
                  source: { url: reference.url, mediaKind: "image" as const },
                  origin: reference.origin as [number, number, number],
                  widthWorld: reference.widthWorld,
                  locked: reference.locked,
                  opacity: reference.opacity,
                }))}
              selectedIds={referenceSelectedIds}
              hoveredId={referenceHoveredId}
              onSelect={(id) => handleReferenceSelect(id)}
              onHover={handleReferenceHover}
            />
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </WorldCanvas>
      {marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "polygon" ? (
        <SelectionMarquee className="z-50" coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
      ) : marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "rect" ? (
          <SelectionMarquee
            className="z-50"
            coverage={marqueeCoverage}
            shape="rect"
            rect={{
              x: Math.min(marqueeStart.x, marqueeEnd.x),
              y: Math.min(marqueeStart.y, marqueeEnd.y),
              width: Math.abs(marqueeEnd.x - marqueeStart.x),
              height: Math.abs(marqueeEnd.y - marqueeStart.y),
            }}
          />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null && (contextMenu.items?.length ?? 0) > 0 && !suggestionPopupOwnsThisWindow}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.suggestTarget ? withSuggestionSubmenu(contextMenu.items, suggestionRows) : (contextMenu?.items ?? [])}
        onOpenChange={(open) => {
          if (!open) dismissContextMenu();
        }}
      />
      {suggestionPopupOwnsThisWindow ? (
        <ContextMenuController
          title={suggestionMenuTitleLabel}
          open
          closeOnSelect={false}
          position={{ x: interaction.suggestionMenu!.x, y: interaction.suggestionMenu!.y }}
          items={suggestionRows}
          onOpenChange={(open) => {
            if (!open) handleSuggestionClose();
          }}
        />
      ) : null}
    </div>
  );
}
//#endregion World3dHost
//#endregion 🔖️World3dHost
