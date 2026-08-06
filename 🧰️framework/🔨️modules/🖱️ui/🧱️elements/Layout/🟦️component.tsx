// #region 🧲️Header
// 💻️ framework/ui/elements/Layout/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { type UiStatus } from "@semio-tech/ui-styling";
// 🚧️W3-interim: these still live in the ui-react barrel (not yet extracted to their own
// 🧱️elements/<Element>/ or 🧱️elements/🫀️core/ dirs) — W3 rewires this import per-symbol as each
// dependency's own element/core file lands. Do not import the barrel from any OTHER new leaf file
// without the same marker; grep for `🚧️W3-interim` must be empty before W6 closes.
import { cn, type Anchor, ANCHORS, type PanelProps, Panel, type MobilePanelProps, MobilePanel, UiMobileProvider, GhostProvider, surfaceClass, SurfaceScope, chromeStatusBorderClass, CanvasSkeleton } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
// #endregion 🔌️Adapters

// #region 🪨️Layout
// Top-level layout orchestrating navbar, panels, canvas, and footer.
// Consumers MUST provide a canvas element.

/**
 * Props interface for the top-level Layout component.
 **/
export interface LayoutProps {
  navbar?: React.ReactNode;
  /** @emoji 🎥️ Optional chrome row directly under `navbar`, above the canvas/panels row (e.g. {@link TutorialBar}) — `flex-shrink-0` like `navbar`/`footer`, never affecting the middle column's z-index invariant below. */
  subnavbar?: React.ReactNode;
  footer?: React.ReactNode;
  /** @emoji 🧭️ Per-anchor panel config — panels float over the navbar/footer/canvas, keyed by which anchor they grow from. */
  panels?: Partial<Record<Anchor, Omit<PanelProps, "anchor">>>;
  mobilePanel?: MobilePanelProps;
  canvas: React.ReactNode;
  /** @emoji 🌀️ When set, paints a loading/waiting ring on the canvas viewport wrapper. */
  canvasStatus?: UiStatus;
  /** @emoji 🦴 Optional skeleton shown inside the canvas ring while `canvasStatus` is busy. */
  canvasSkeleton?: React.ReactNode;
  mobile?: boolean;
  className?: string;
}

const Layout: React.FC<LayoutProps> = ({ navbar, subnavbar, footer, panels, mobilePanel, canvas, canvasStatus, canvasSkeleton, mobile = false, className = "" }) => (
  <UiMobileProvider mobile={mobile}>
    <GhostProvider>
      {/* 🎨️ One continuous base floor for navbar + canvas + footer — chrome rows stay transparent over this paint. */}
      <div data-slot="layout" data-level="base" className={cn("relative flex flex-col overflow-hidden", surfaceClass, mobile ? "h-full w-full" : "h-screen w-screen", className)}>
        <SurfaceScope level="base" fill="surface">
          {navbar && <div className="flex-shrink-0">{navbar}</div>}
          {subnavbar && <div className="flex-shrink-0">{subnavbar}</div>}
          {mobile ? (
            <div className="flex flex-col flex-1 min-h-0">
              {mobilePanel && mobilePanel.visible && <MobilePanel {...mobilePanel} />}
              {/* 📱️ The canvas stays mounted (never unmounted) while the mobile panel covers it, so the WASM/3D
                  world keeps its context instead of replugging on every toggle — it just stops being visible. */}
              <div className={cn("flex-1 min-w-0 min-h-0 relative", mobilePanel?.visible && "hidden", chromeStatusBorderClass(canvasStatus))}>
                {canvasStatus === "loading" || canvasStatus === "waiting" ? (canvasSkeleton ?? <CanvasSkeleton />) : canvas}
              </div>
            </div>
          ) : (
            // Positioned within this region (relative, between navbar and footer), not the whole display — panels
            // open below the navbar / above the footer instead of floating over them, while still overlaying canvas
            // the same way a window's options rail overlays its own canvas.
            <div className="flex flex-1 min-h-0 relative">
              {/* 🎓️ No z-index here (was z-0): trapping this column in its own stacking context would make
                  windows unreachable by [data-introduction-elevated] — a window can only rise above the
                  fullscreen introduction veil if it participates in the root stacking context. */}
              <div className="flex flex-col flex-1 min-w-0 relative">
                <div className="flex flex-1 min-h-0 relative">
                  <div className={cn("flex-1 min-w-0 min-h-0 relative", chromeStatusBorderClass(canvasStatus))}>
                    {canvasStatus === "loading" || canvasStatus === "waiting" ? (canvasSkeleton ?? <CanvasSkeleton />) : canvas}
                  </div>
                </div>
              </div>
              {ANCHORS.map((anchor) => {
                const panelProps = panels?.[anchor];
                return panelProps ? <Panel key={anchor} {...panelProps} anchor={anchor} /> : null;
              })}
            </div>
          )}
          {footer && <div className="flex-shrink-0">{footer}</div>}
        </SurfaceScope>
      </div>
    </GhostProvider>
  </UiMobileProvider>
);

export { Layout };

// #endregion 🪨️Layout
