/** 🧭️ The wind control: one dial that sets the direction and the speed of the wind together, drawn as the ground around the asset in the viewpoint the reader looks from, with the same wind in words beside it.
 *
 * The dial is the horizontal plane (x ahead, z to the asset's left) projected like the figure above it: a circle in the plan, a flat ellipse in an elevation (the axis that points at the reader is foreshortened, away from the reader is up), the axonometric ellipse in the axonometric drawing and the 3D model. The arrow starts at the asset and points where the air moves to; its length is the speed on a square-root scale, so a breeze of 1 m/s is as easy to set as a gale.
 * Pointer: press or drag anywhere on the dial. Key presses and pointer moves that arrive before the next render build on the wind the dial last reported, so a held key never loses a step. Keyboard (the dial is one slider whose value is the bearing and whose value text names bearing, speed and direction in words): right and left raise and lower the bearing, up and down change the speed, Shift takes larger steps, Home points it ahead, End stills the air. The numeric fields of the section set the same two values exactly.
 *
 * @see ../../../../../🧬️schema/🟦️.ts — `Environment.windDirectionDeg`, the bearing convention
 * @see ../🌬️air/🟦️.tsx — the section the dial stands in
 */
import { useId, useRef, type KeyboardEvent, type PointerEvent, type ReactElement } from "react";
import { CAMERAS, axonometricCamera, formatNumber, project } from "@teaching/staffage";
import type { Language, PlanarViewpoint, Vec3 } from "@teaching/staffage";
import type { Translate } from "../../🌐️i18n/🟦️.ts";
import { windSector } from "../../🌊️flow/🟦️.ts";

/** 🌬️ The speed at the rim of the dial (m/s); faster wind is set in the numeric field. */
export const DIAL_MAX_MS = 20;

/** 👁️ The viewpoint a dial is drawn for: a planar drawing, or the axonometric view (also the perspective camera of the 3D model). */
export type DialView = PlanarViewpoint | "axonometric";

/** 📐️ Where the unit vectors ahead and to the left of the asset point on the dial (x right, y down), the longer of the two with length 1. */
export type GroundBasis = { readonly ahead: readonly [number, number]; readonly left: readonly [number, number] };

const DEPTH = 0.42;

/** 📐️ The ground basis of a viewpoint: its paper projection, with the axis the viewer looks along drawn foreshortened (away from the viewer is up). */
export function groundBasis(view: DialView): GroundBasis {
  const camera = view === "axonometric" ? axonometricCamera() : undefined;
  const looking: Vec3 = view === "axonometric" ? { x: 0, y: 0, z: 0 } : CAMERAS[view].looking;
  const axis = (unit: Vec3): [number, number] => {
    const p = project(view, unit, camera);
    const depth = unit.x * looking.x + unit.z * looking.z;
    return Math.hypot(p.x, p.y) > 1e-9 ? [p.x, -p.y] : [0, -DEPTH * depth];
  };
  const ahead = axis({ x: 1, y: 0, z: 0 });
  const left = axis({ x: 0, y: 0, z: 1 });
  const longest = Math.max(Math.hypot(...ahead), Math.hypot(...left));
  return { ahead: [ahead[0] / longest, ahead[1] / longest], left: [left[0] / longest, left[1] / longest] };
}

/** 📍️ The point of the dial (unit radius at {@link DIAL_MAX_MS}) of a wind. */
export function dialPoint(basis: GroundBasis, bearingDeg: number, speedMS: number): readonly [number, number] {
  const radius = Math.sqrt(Math.min(Math.max(speedMS, 0), DIAL_MAX_MS) / DIAL_MAX_MS);
  const phi = (bearingDeg * Math.PI) / 180;
  const [c, s] = [Math.cos(phi) * radius, Math.sin(phi) * radius];
  return [c * basis.ahead[0] + s * basis.left[0], c * basis.ahead[1] + s * basis.left[1]];
}

/** 🎯️ The wind of a point of the dial: the inverse of {@link dialPoint}, the bearing in whole degrees and the speed in tenths. A point at the asset is still air and keeps the bearing it is given. */
export function dialWind(basis: GroundBasis, x: number, y: number, bearingDeg: number): { readonly bearingDeg: number; readonly speedMS: number } {
  const det = basis.ahead[0] * basis.left[1] - basis.ahead[1] * basis.left[0];
  const gx = (x * basis.left[1] - y * basis.left[0]) / det;
  const gz = (basis.ahead[0] * y - basis.ahead[1] * x) / det;
  const radius = Math.min(1, Math.hypot(gx, gz));
  const speedMS = Number((radius * radius * DIAL_MAX_MS).toFixed(1));
  return { bearingDeg: speedMS === 0 ? bearingDeg : (Math.round((Math.atan2(gz, gx) * 180) / Math.PI) + 360) % 360, speedMS };
}

/** 🗣️ The wind in one sentence of the reader's language. */
export function windWords(t: Translate, language: Language, bearingDeg: number, speedMS: number): string {
  return speedMS > 0 ? t("wind.words", { direction: t(`wind.towards.${windSector(bearingDeg)}`), bearing: formatNumber(bearingDeg, 0, language), speed: formatNumber(speedMS, 1, language) }) : t("wind.still");
}

const SIZE = 56;
const PAD = 24;
const MARKS = [["ahead", 0], ["left", 90], ["behind", 180], ["right", 270]] as const;
const RINGS = [1, 5, DIAL_MAX_MS] as const;
const path = (basis: GroundBasis, speedMS: number): string => `${Array.from({ length: 73 }, (_, i) => dialPoint(basis, i * 5, speedMS)).map(([x, y], i) => `${i === 0 ? "M" : "L"}${(x * SIZE).toFixed(2)} ${(y * SIZE).toFixed(2)}`).join("")}Z`;

/** 🧭️ The wind dial. */
export function WindDial(props: { readonly view: DialView; readonly viewName: string; readonly bearingDeg: number; readonly speedMS: number; readonly onChange: (wind: { readonly bearingDeg: number; readonly speedMS: number }) => void; readonly t: Translate; readonly language: Language; readonly disabled?: boolean }): ReactElement {
  const { view, t, language } = props;
  const latest = useRef({ bearingDeg: props.bearingDeg, speedMS: props.speedMS });
  latest.current = { bearingDeg: props.bearingDeg, speedMS: props.speedMS };
  const { bearingDeg, speedMS } = latest.current;
  const onChange = (wind: { readonly bearingDeg: number; readonly speedMS: number }): void => {
    latest.current = wind;
    props.onChange(wind);
  };
  const basis = groundBasis(view);
  const helpId = useId();
  const dragging = useRef(false);
  const box = useRef<SVGSVGElement | null>(null);
  const tip = dialPoint(basis, bearingDeg, Math.max(speedMS, 0));
  const length = Math.hypot(tip[0], tip[1]) * SIZE;
  const angle = (Math.atan2(tip[1], tip[0]) * 180) / Math.PI;
  const text = speedMS > 0 ? t("wind.value", { bearing: formatNumber(bearingDeg, 0, language), speed: formatNumber(speedMS, 1, language), direction: t(`wind.towards.${windSector(bearingDeg)}`) }) : t("wind.still");
  const at = (event: PointerEvent): void => {
    const rect = box.current?.getBoundingClientRect();
    if (rect === undefined || rect.width === 0 || rect.height === 0) return;
    const scale = Math.min(rect.width, rect.height) / 2 / (SIZE + PAD);
    onChange(dialWind(basis, (event.clientX - rect.left - rect.width / 2) / scale / SIZE, (event.clientY - rect.top - rect.height / 2) / scale / SIZE, latest.current.bearingDeg));
  };
  const onKey = (event: KeyboardEvent): void => {
    const { bearingDeg, speedMS } = latest.current;
    const turn = event.shiftKey ? 45 : 5;
    const pace = event.shiftKey ? 1 : 0.1;
    const next = event.key === "ArrowRight" ? { bearingDeg: (bearingDeg + turn) % 360, speedMS } : event.key === "ArrowLeft" ? { bearingDeg: (bearingDeg - turn + 360) % 360, speedMS } : event.key === "ArrowUp" ? { bearingDeg, speedMS: Number(Math.min(40, speedMS + pace).toFixed(1)) } : event.key === "ArrowDown" ? { bearingDeg, speedMS: Number(Math.max(0, speedMS - pace).toFixed(1)) } : event.key === "Home" ? { bearingDeg: 0, speedMS } : event.key === "End" ? { bearingDeg, speedMS: 0 } : null;
    if (next === null) return;
    event.preventDefault();
    onChange(next);
  };
  return (
    <div data-shared-border-cell="" className="flex min-w-0 flex-col items-stretch gap-single px-double py-single">
      <div
        role="slider"
        tabIndex={props.disabled ? -1 : 0}
        aria-label={t("wind.dial")}
        aria-valuemin={0}
        aria-valuemax={359}
        aria-valuenow={bearingDeg}
        aria-valuetext={text}
        aria-describedby={helpId}
        aria-disabled={props.disabled ? true : undefined}
        data-wind-dial={view}
        data-bearing={bearingDeg}
        data-speed={speedMS}
        onKeyDown={onKey}
        onPointerDown={(event) => {
          if (props.disabled) return;
          dragging.current = true;
          event.currentTarget.setPointerCapture?.(event.pointerId);
          at(event);
        }}
        onPointerMove={(event) => dragging.current && at(event)}
        onPointerUp={() => void (dragging.current = false)}
        onPointerCancel={() => void (dragging.current = false)}
        className="mx-auto aspect-square w-full max-w-56 cursor-crosshair touch-none select-none text-foreground focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
      >
        <svg ref={box} viewBox={`${-SIZE - PAD} ${-SIZE - PAD} ${2 * (SIZE + PAD)} ${2 * (SIZE + PAD)}`} width="100%" height="100%" aria-hidden="true" focusable="false">
          {RINGS.map((ring) => <path key={ring} d={path(basis, ring)} fill={ring === DIAL_MAX_MS ? "color-mix(in srgb, currentColor 6%, transparent)" : "none"} stroke="currentColor" strokeOpacity={ring === DIAL_MAX_MS ? 0.7 : 0.3} strokeWidth={ring === DIAL_MAX_MS ? 1 : 0.6} strokeDasharray={ring === DIAL_MAX_MS ? undefined : "2 2"} />)}
          {MARKS.map(([name, bearing]) => {
            const [x, y] = dialPoint(basis, bearing, DIAL_MAX_MS);
            const far = Math.hypot(x, y) || 1;
            return (
              <g key={name}>
                <line x1={0} y1={0} x2={x * SIZE} y2={y * SIZE} stroke="currentColor" strokeOpacity={0.25} strokeWidth={0.6} />
                <text x={x * SIZE + (x / far) * 8} y={y * SIZE + (y / far) * 8} fontSize={7} textAnchor="middle" dominantBaseline="central" fill="currentColor" fillOpacity={0.75}>{t(`wind.mark.${name}`)}</text>
              </g>
            );
          })}
          <circle r={2.4} fill="currentColor" />
          {speedMS > 0 ? (
            <g data-wind-arrow="" transform={`rotate(${angle.toFixed(2)})`} stroke="var(--accent, currentColor)" fill="var(--accent, currentColor)">
              <line x1={0} y1={0} x2={Math.max(0, length - 6)} y2={0} strokeWidth={2.2} strokeLinecap="round" />
              <path d={`M${length} 0L${length - 8} -4L${length - 8} 4Z`} stroke="none" />
            </g>
          ) : (
            <circle data-wind-arrow="still" r={5} fill="none" stroke="var(--accent, currentColor)" strokeWidth={1.4} />
          )}
        </svg>
      </div>
      <output data-wind-words="" aria-live="polite" className="text-xs leading-normal text-foreground">{windWords(t, language, bearingDeg, speedMS)}</output>
      <p id={helpId} className="m-0 text-[0.65rem] leading-normal text-muted-foreground">{`${t("wind.dialView", { view: props.viewName })} ${t("wind.dialHelp")}`}</p>
    </div>
  );
}
