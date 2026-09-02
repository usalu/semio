// #region 🧲️Header
// 💻️ framework/ui/elements/⭕️Ring/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { type ElementProps } from "../../🔨️modules/🆔️element-identity/🟦️.ts";
import { Label } from "../🏷️Label/🟦️.tsx";
import { usePanelGhost } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import { createDOMEventBinding } from "../🖱️ContextMenu/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🧫️Ring
// SVG ring container with draggable Orbs. Fires onOrbChange(orbId, oldT, newT) on drag.

interface RingOrbData {
  id: string;
  t: number;
  disabled?: boolean;
  selected?: boolean;
  hovered?: boolean;
}

interface RingProps extends ElementProps {
  orbs: RingOrbData[];
  radius?: number;
  size?: number;
  onOrbChange?: (orbId: string, oldT: number, newT: number) => void;
  onOrbSelect?: (orbId: string) => void;
  onOrbHoverChange?: (orbId: string, hovered: boolean) => void;
  showLabel?: boolean;
  className?: string;
}

// #region 🔮️RingMarker
/** @emoji 🔮️ Private circular position marker for a Ring. */
interface RingMarkerProps {
  id: string;
  t: number;
  disabled?: boolean;
  selected?: boolean;
  hovered?: boolean;
  dragging?: boolean;
  radius?: number;
  onPointerDown?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerMove?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerUp?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerEnter?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerLeave?: (e: React.PointerEvent<SVGCircleElement>) => void;
}

function RingMarker({ id, t, disabled = false, selected = false, hovered = false, radius = 40, dragging = false, onPointerDown, onPointerMove, onPointerUp, onPointerEnter, onPointerLeave }: RingMarkerProps) {
  const angle = t * 2 * Math.PI - Math.PI / 2;
  const cx = Math.cos(angle) * radius;
  const cy = Math.sin(angle) * radius;
  const orbRadius = selected ? 7 : 5;
  return (
    <circle
      data-slot="orb"
      data-orb-id={id}
      cx={cx}
      cy={cy}
      r={orbRadius}
      className={cn(
        dragging ? "" : "transition-all duration-150",
        disabled ? "fill-muted-foreground/40 cursor-not-allowed" : "fill-foreground cursor-grab active:cursor-grabbing",
        selected && !disabled && "fill-accent stroke-accent-foreground stroke-1",
        hovered && !disabled && !selected && "fill-accent-foreground",
      )}
      style={{ pointerEvents: disabled ? "none" : "auto" }}
      onPointerDown={disabled ? undefined : onPointerDown}
      onPointerMove={disabled ? undefined : onPointerMove}
      onPointerUp={disabled ? undefined : onPointerUp}
      onPointerEnter={disabled ? undefined : onPointerEnter}
      onPointerLeave={disabled ? undefined : onPointerLeave}
    />
  );
}
// #endregion 🔮️RingMarker

function Ring({ id, orbs, radius = 40, size = 100, onOrbChange, onOrbSelect, onOrbHoverChange, showLabel, className }: RingProps) {
  const panelGhost = usePanelGhost();
  const svgRef = reactHostPort.useRef<SVGSVGElement>(null);
  const [draggingOrbId, setDraggingOrbId] = reactHostPort.useState<string | null>(null);
  const [localT, setLocalT] = reactHostPort.useState<number | null>(null);
  const dragStartT = reactHostPort.useRef<number>(0);
  const rafId = reactHostPort.useRef<number>(0);
  const pendingT = reactHostPort.useRef<number | null>(null);
  const center = size / 2;
  const angleFromEvent = reactHostPort.useCallback(
    (e: React.PointerEvent | PointerEvent): number => {
      if (!svgRef.current) return 0;
      const rect = svgRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left - center;
      const y = e.clientY - rect.top - center;
      let angle = Math.atan2(y, x) + Math.PI / 2;
      if (angle < 0) angle += 2 * Math.PI;
      return (angle / (2 * Math.PI)) % 1;
    },
    [center],
  );
  const handleOrbPointerDown = reactHostPort.useCallback(
    (orbId: string, t: number) => (e: React.PointerEvent<SVGCircleElement>) => {
      e.preventDefault();
      e.stopPropagation();
      panelGhost?.begin(svgRef.current ?? e.currentTarget);
      setDraggingOrbId(orbId);
      setLocalT(t);
      dragStartT.current = t;
      pendingT.current = null;
      onOrbSelect?.(orbId);
    },
    [onOrbSelect, panelGhost],
  );
  const flushPendingChange = reactHostPort.useCallback(
    (orbId: string) => {
      if (pendingT.current !== null) {
        onOrbChange?.(orbId, dragStartT.current, pendingT.current);
        pendingT.current = null;
      }
    },
    [onOrbChange],
  );
  reactHostPort.useEffect(() => {
    if (!draggingOrbId) return;
    const onMove = (e: PointerEvent) => {
      const newT = angleFromEvent(e);
      setLocalT(newT);
      pendingT.current = newT;
      if (!rafId.current) {
        const orbId = draggingOrbId;
        rafId.current = requestAnimationFrame(() => {
          rafId.current = 0;
          flushPendingChange(orbId);
        });
      }
    };
    const onUp = (e: PointerEvent) => {
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      const newT = angleFromEvent(e);
      setLocalT(null);
      onOrbChange?.(draggingOrbId, dragStartT.current, newT);
      setDraggingOrbId(null);
      panelGhost?.end();
    };
    const onCancel = () => {
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      setLocalT(null);
      setDraggingOrbId(null);
      panelGhost?.end();
    };
    const bindings = createDOMEventBinding();
    bindings.listen(window, "pointermove", onMove);
    bindings.listen(window, "pointerup", onUp);
    bindings.listen(window, "pointercancel", onCancel);
    return () => bindings.dispose();
  }, [angleFromEvent, draggingOrbId, flushPendingChange, onOrbChange, panelGhost]);
  reactHostPort.useEffect(() => {
    return () => {
      if (rafId.current) cancelAnimationFrame(rafId.current);
    };
  }, []);
  const ringElement = (
    <svg
      ref={svgRef}
      data-dim
      data-slot="ring"
      data-detail-panel-control="fit"
      id={id}
      width={size}
      height={size}
      viewBox={`${-center} ${-center} ${size} ${size}`}
      className={cn("w-fit shrink-0 touch-none select-none overflow-visible", className)}
      style={{ overflow: "visible" }}
    >
      <circle data-slot="ring-track" cx={0} cy={0} r={radius} className="fill-none stroke-muted-foreground/30 stroke-[length:var(--stroke-default)]" />
      {orbs.map((orb) => (
        <RingMarker
          key={orb.id}
          id={orb.id}
          t={draggingOrbId === orb.id && localT !== null ? localT : orb.t}
          disabled={orb.disabled}
          selected={orb.selected}
          hovered={orb.hovered}
          dragging={draggingOrbId === orb.id}
          radius={radius}
          onPointerDown={handleOrbPointerDown(orb.id, orb.t)}
          onPointerEnter={onOrbHoverChange ? () => onOrbHoverChange(orb.id, true) : undefined}
          onPointerLeave={onOrbHoverChange ? () => onOrbHoverChange(orb.id, false) : undefined}
        />
      ))}
    </svg>
  );
  if (showLabel) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={className}>
        {ringElement}
      </Label>
    );
  }
  return ringElement;
}

export { Ring };
export type { RingOrbData, RingProps };

// #endregion 🧫️Ring
