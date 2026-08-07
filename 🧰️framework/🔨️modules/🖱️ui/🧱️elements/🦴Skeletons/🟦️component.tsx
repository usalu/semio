// #region 🧲️Header
// 💻️ framework/ui/elements/🦴Skeletons/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../🏷️ClassNames/🟦️component.tsx";
import { SceneSkeleton } from "../🎬Scene/🟦️component.tsx";
import { LoadingRow } from "../🎛️Chrome/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 🦴Skeletons

/** @emoji 🦴 Shared pulse fill for declarative and chrome skeleton placeholders. */
export const skeletonPulseClass = "animate-pulse rounded bg-muted-foreground/20 motion-reduce:animate-none";

/** @emoji 🦴 One rectangular skeleton block. */
export const SkeletonBlock: React.FC<{ className?: string }> = ({ className = "" }) => <div className={cn(skeletonPulseClass, className)} aria-hidden />;

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

/** @emoji 🦴 Picks a skeleton placeholder for a declarative {@link UiNode} kind. */
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
        <div className="flex flex-col gap-single w-full">
          <SkeletonBlock className="h-3 w-24" />
          <SkeletonBlock className="h-medium w-full" />
        </div>
      );
    case "group":
    case "section":
      return (
        <div className="flex flex-col gap-double w-full p-single">
          <SkeletonBlock className="h-4 w-32" />
          <SkeletonBlock className="h-medium w-full" />
          <SkeletonBlock className="h-medium w-full" />
        </div>
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

/** @emoji 🦴 Mimics a mode-dock window body while plugin UI is still loading. */
export const WindowBodySkeleton: React.FC<{ className?: string }> = ({ className = "" }) => (
  <div className={cn("flex h-full min-h-0 w-full flex-col gap-double p-double", className)} role="status" aria-busy="true">
    <SkeletonBlock className="h-4 w-48" />
    <SkeletonBlock className="min-h-0 flex-1 w-full" />
  </div>
);

/** @emoji 🦴 Panel tree placeholder while a tab body is refreshing. */
export const PanelTreeSkeleton: React.FC<{ className?: string }> = ({ className = "" }) => (
  <div className={cn("flex flex-col gap-single p-single w-full", className)} role="status" aria-busy="true">
    <LoadingRow name="…" />
    <LoadingRow name="…" />
    <LoadingRow name="…" />
  </div>
);

/** @emoji 🦴 Full canvas placeholder while the primary plugin session boots. */
export const CanvasSkeleton: React.FC<{ className?: string; label?: string }> = ({ className = "", label }) => (
  <div className={cn("relative flex h-full min-h-0 w-full flex-col overflow-hidden", className)} role="status" aria-busy="true" aria-label={label}>
    <div className="flex h-medium w-full shrink-0 gap-single p-single">
      <SkeletonBlock className="h-full w-28" />
      <SkeletonBlock className="h-full flex-1" />
      <SkeletonBlock className="h-full w-16" />
    </div>
    <div className="min-h-0 flex-1 p-double">
      <SkeletonBlock className="h-full w-full" />
    </div>
  </div>
);

// #endregion 🦴Skeletons
