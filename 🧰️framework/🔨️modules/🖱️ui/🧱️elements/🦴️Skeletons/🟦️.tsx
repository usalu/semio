// #region 🧲️Header
// 💻️ framework/ui/elements/🦴️Skeletons/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { shellFloorFillClass } from "../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { WindowChrome, windowChromeTitleChipClass } from "../🗂️WindowChrome/🟦️.tsx";
import { LevelProvider, SurfaceScope, useSurface } from "../🌈️Surface/🟦️.tsx";
import { SceneSkeleton } from "../🎬️Scene/🟦️.tsx";
import { SharedBorderRow, SharedBorderRowCell } from "../🪢️SharedBorderRow/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🦴️Skeletons

/** 🦴 Shared pulse fill for declarative and chrome skeleton placeholders. */
export const skeletonPulseClass = "animate-pulse rounded bg-muted-foreground/20 motion-reduce:animate-none";

/** 🧱️ One rectangular skeleton block. */
export const SkeletonBlock: React.FC<{ className?: string }> = ({ className = "" }) => <div className={cn(skeletonPulseClass, className)} aria-hidden />;

const SkeletonLoadingRow: React.FC<{ readonly name: string }> = ({ name }) => (
  <SharedBorderRowCell data-shared-border-state="loading" className="opacity-50 pointer-events-none">
    <span className="flex-1 truncate">{name}</span>
  </SharedBorderRowCell>
);

export type ElementSkeletonKind =
  | "text"
  | "button"
  | "separator"
  | "image"
  | "input"
  | "select"
  | "toggle"
  | "keyValue"
  | "slider"
  | "numberStepper"
  | "ring"
  | "iconSelect"
  | "field"
  | "group"
  | "section"
  | "stack"
  | "tree"
  | "componentScene"
  | "externalSlot";

/** ⚒️ Picks a skeleton placeholder for a declarative {@link UiNode} kind. */
export function elementSkeleton(kind: ElementSkeletonKind): React.ReactElement {
  switch (kind) {
    case "text":
      return <SkeletonBlock className="h-4 w-3/5 max-w-full" />;
    case "button":
      return <SkeletonBlock className="h-medium w-28" />;
    case "separator":
      return <SkeletonBlock className="h-px w-full" />;
    case "image":
      return <SkeletonBlock className="h-32 w-full max-w-sm" />;
    case "input":
    case "select":
      return <SkeletonBlock className="h-medium w-full" />;
    case "toggle":
      return <SkeletonBlock className="h-medium w-24" />;
    case "keyValue":
      return (
        <div className="flex flex-col gap-single w-full">
          <SkeletonBlock className="h-4 w-full" />
          <SkeletonBlock className="h-4 w-4/5" />
        </div>
      );
    case "slider":
      return <SkeletonBlock className="h-4 w-full" />;
    case "numberStepper":
      return <SkeletonBlock className="h-medium w-20" />;
    case "ring":
      return <SkeletonBlock className="size-large rounded-full" />;
    case "iconSelect":
      return <SkeletonBlock className="h-medium w-full" />;
    case "field":
      return (
        <SharedBorderRow orientation="vertical" className="w-full">
          <SharedBorderRowCell><SkeletonBlock className="h-3 w-24" /></SharedBorderRowCell>
          <SharedBorderRowCell className="flex-col items-stretch"><SkeletonBlock className="h-medium w-full" /></SharedBorderRowCell>
        </SharedBorderRow>
      );
    case "group":
    case "section":
      return (
        <SharedBorderRow orientation="vertical" className="w-full">
          <SharedBorderRowCell><SkeletonBlock className="h-4 w-32" /></SharedBorderRowCell>
          <SharedBorderRowCell className="flex-col items-stretch"><SkeletonBlock className="h-medium w-full" /></SharedBorderRowCell>
          <SharedBorderRowCell className="flex-col items-stretch"><SkeletonBlock className="h-medium w-full" /></SharedBorderRowCell>
        </SharedBorderRow>
      );
    case "stack":
      return (
        <div className="flex flex-col gap-single w-full h-full min-h-0 p-single">
          <SkeletonBlock className="h-4 w-40" />
          <SkeletonBlock className="flex-1 min-h-24 w-full" />
        </div>
      );
    case "tree":
      return <PanelTreeSkeleton />;
    case "componentScene":
      return <SceneSkeleton />;
    case "externalSlot":
      return <SkeletonBlock className="h-full min-h-32 w-full" />;
  }
}

/** 🪟️ Mimics a mode-dock window body while plugin UI is still loading. */
export const WindowBodySkeleton: React.FC<{ className?: string }> = ({ className = "" }) => (
  <div className={cn("flex h-full min-h-0 w-full flex-col gap-double p-double", className)} role="status" aria-busy="true">
    <SkeletonBlock className="h-4 w-48" />
    <SkeletonBlock className="min-h-0 flex-1 w-full" />
  </div>
);

/** 🌳️ Panel tree placeholder while a tab body is refreshing. */
export const PanelTreeSkeleton: React.FC<{ className?: string }> = ({ className = "" }) => (
  <SharedBorderRow orientation="vertical" className={cn("w-full", className)} role="status" aria-busy="true">
    <SkeletonLoadingRow name="…" />
    <SkeletonLoadingRow name="…" />
    <SkeletonLoadingRow name="…" />
  </SharedBorderRow>
);

const CanvasSkeletonWindow: React.FC<{ active?: boolean }> = ({ active = false }) => (
  <SurfaceScope level="window">
    <WindowChrome
      level="window"
      active={active}
      borderKind="loading"
      stackSlot="canvas-skeleton-stack"
      frameBorderSlot="canvas-skeleton-frame-border"
      stackClassName="relative z-window h-full min-h-0 w-full min-w-0 overflow-hidden"
      bodyClassName="flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-single"
      bodySurfaceFill="surface"
      titleChips={
        <span className={cn(windowChromeTitleChipClass, "flex items-center gap-single px-single")} aria-hidden>
          <SkeletonBlock className="size-small shrink-0 rounded-sm" />
          <SkeletonBlock className="h-3 w-14 max-w-full" />
        </span>
      }
      body={
        <div className="flex h-full min-h-0 w-full flex-col p-double" aria-hidden>
          <SkeletonBlock className="min-h-0 flex-1 w-full" />
        </div>
      }
    />
  </SurfaceScope>
);

/** 🚀️ Full canvas placeholder while the primary plugin session boots — two even mode-dock windows, each one rectangle with its title chip glass-overlaid on the top row above the placeholder. */
export const CanvasSkeleton: React.FC<{ className?: string; label?: string }> = ({ className = "", label }) => {
  const parent = useSurface();
  const floorClass = shellFloorFillClass(parent);
  return (
    <LevelProvider level="base">
      <div className={cn("box-border flex h-full min-h-0 w-full flex-col overflow-hidden", floorClass, className)} role="status" aria-busy="true" aria-label={label}>
        <div className="flex h-full min-h-0 w-full flex-row">
          <div className="flex min-h-0 min-w-0 flex-1 flex-col">
            <CanvasSkeletonWindow active />
          </div>
          <div className="flex min-h-0 min-w-0 flex-1 flex-col">
            <CanvasSkeletonWindow />
          </div>
        </div>
      </div>
    </LevelProvider>
  );
};

// #endregion 🦴️Skeletons
