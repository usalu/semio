import { Surface } from "@semio-tech/ui-react";
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TiledMapHost/component.tsx
/** 🗺️ `TiledMapHost` — tiled-map `ComponentSceneHost`: drives the map wasm session
 * (raster/vector tile fetch+cache, marquee/click feature selection, camera pan/zoom, hover popup)
 * through a demand-scheduled render loop, reusing `World3dHost`'s window-instance context and
 * `🟦️Interpreter`'s surface context-menu plumbing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Button, cn, floatingMenuSurfaceClass, Icon, ContextMenuController, SelectionMarquee, useLabel } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { useMapState } from "./🧰️map-state/🟦️.ts";
import { useMapRenderEffects } from "./🎞️map-render-effects/🟦️.ts";
import { useMapPointer, type MapCore3 } from "./👆️map-pointer/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️TiledMapHost
//#region TiledMapHost
export function TiledMapHost(props: ComponentSceneHostProps) {
  const core = {} as MapCore3;
  Object.assign(core, useMapState(props));
  Object.assign(core, useMapRenderEffects(core));
  Object.assign(core, useMapPointer(core));
  const { canvasRef, containerRef, contextMenu, contextMenuTitleLabel, emptySceneLabel, marqueeOverlay, node, onMapKeyDown, overlayIndex, popupFeature, popupRef, renderAttempt, rendererUnavailable, retryRenderer, scene, setContextMenu, sourceAvailableLabel } = core;
  const mapLabel = useLabel("ui.surfaceContextMenu.map");
  const canvasUnavailableLabel = useLabel("ui.host.canvasUnavailable");
  const retryLabel = useLabel("ui.common.retry");

  if (!scene) return <div className="semio-tiled-map-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div ref={containerRef} className="semio-tiled-map-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none" data-surface-id={node.surfaceId} style={{ touchAction: "none" }}>
      <canvas key={renderAttempt} ref={canvasRef} role="application" aria-label={mapLabel} aria-hidden={rendererUnavailable || undefined} tabIndex={rendererUnavailable ? -1 : 0} onKeyDown={onMapKeyDown} style={rendererUnavailable ? { visibility: "hidden" } : undefined} className="absolute inset-0 block size-full touch-none focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring" />
      {rendererUnavailable ? (
        <div className="absolute inset-0 z-20 flex flex-col items-center justify-center gap-2 p-2 text-center">
          <p role="alert" className="text-sm text-muted-foreground">{canvasUnavailableLabel}</p>
          <Button icon="rotate-ccw" text={retryLabel} onClick={retryRenderer} />
        </div>
      ) : null}
      {marqueeOverlay?.shape === "rect" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} /> : null}
      {marqueeOverlay?.shape === "polygon" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} /> : null}
      <ContextMenuController title={contextMenuTitleLabel} open={contextMenu.open} position={contextMenu.position} items={contextMenu.items} onOpenChange={(open) => setContextMenu((prev) => ({ ...prev, open }))} />
      {popupFeature && !rendererUnavailable ? (
        <Surface overlay level="menu" fill="glass" ref={popupRef} className={cn("pointer-events-none absolute z-10 max-w-56 -translate-x-1/2 -translate-y-full px-2 py-1.5", floatingMenuSurfaceClass)} data-border-probe-art="" style={{ left: 0, top: 0 }}>
          {(() => {
            const meta = overlayIndex.meta.get(popupFeature.kind)?.get(popupFeature.id);
            const title = meta?.label ?? popupFeature.id;
            return (
              <div className="flex items-start gap-1.5">
                {meta?.icon ? <Icon icon={meta.icon} size="small" className="mt-0.5 shrink-0" /> : null}
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium">{title}</div>
                  {meta?.link ? <span className="text-xs text-secondary underline-offset-2">{sourceAvailableLabel}</span> : null}
                </div>
              </div>
            );
          })()}
        </Surface>
      ) : null}
    </div>
  );
}
//#endregion TiledMapHost
//#endregion 🔖️TiledMapHost
