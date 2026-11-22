/** 🖌️ `FlowOverlay`: draws the marks of a flow field on a 2D canvas laid over a figure — the same streams, tracers, vapour clouds and fog, in the same colours and paper sizes, as the paper drawing of the library, but redrawn every frame so the tracers move with the clock.
 *
 * The owner supplies the projection of a world point (ahead x, up y, left z) to canvas pixels and how many pixels a paper millimetre is: a drawing passes its own paper mapping, the 3D stage its camera. Vapour clouds are radial gradients (soft Gaussian puffs), fog is a white cloud whose common outline is a cyan rim. The canvas never takes pointer events and is hidden from assistive technology: the legend and the status of the figure say what it shows.
 *
 * @see ../../../../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts — the paper twin (`flowView`) and the shared paint
 */
import { FLOW_STYLE, flowDetailOf, flowMarks, flowPaint } from "@teaching/staffage";
import type { DisplayRange, FlowField, FlowLayer, FlowPaint, Vec3 } from "@teaching/staffage";

/** 🧭️ How world points land on the canvas: `project` writes CSS pixels (x right, y down) into `out` and returns false for a point that cannot be shown (behind a perspective camera); `pxPerMm` is the size of a paper millimetre in CSS pixels. */
export type FlowProjection = { readonly project: (at: Vec3, out: [number, number]) => boolean; readonly pxPerMm: number };

/** 🎛️ What one frame of the overlay shows. */
export type FlowFrame = { readonly flow: FlowField | null; readonly timeS: number; readonly layers: readonly FlowLayer[]; readonly theme: "light" | "dark"; readonly range: DisplayRange; readonly denominator: number; readonly projection: FlowProjection };

/** 📐️ The longest side of the backing store in device pixels: a figure zoomed far in is drawn coarser instead of allocating a huge canvas. */
export const MAX_BACKING_PX = 4096;

const rgba = (hex: string, alpha: number): string => `rgba(${Number.parseInt(hex.slice(1, 3), 16)},${Number.parseInt(hex.slice(3, 5), 16)},${Number.parseInt(hex.slice(5, 7), 16)},${alpha})`;

/** 🖌️ The flow renderer of one canvas. */
export class FlowOverlay {
  readonly #context: CanvasRenderingContext2D | null;
  readonly #paints = new WeakMap<FlowField, Map<string, FlowPaint>>();
  #ratio = 1;

  constructor(readonly canvas: HTMLCanvasElement) {
    canvas.setAttribute("aria-hidden", "true");
    canvas.style.pointerEvents = "none";
    this.#context = canvas.getContext("2d");
  }

  /** 📐️ Fits the backing store to the CSS box of the canvas (device pixels, at most twice the CSS size and {@link MAX_BACKING_PX} on its longest side). */
  resize(): void {
    const ratio = Math.min(2, Math.max(1, globalThis.devicePixelRatio || 1), MAX_BACKING_PX / Math.max(1, this.canvas.clientWidth, this.canvas.clientHeight));
    const width = Math.max(1, Math.round(this.canvas.clientWidth * ratio));
    const height = Math.max(1, Math.round(this.canvas.clientHeight * ratio));
    if (this.canvas.width !== width || this.canvas.height !== height) {
      this.canvas.width = width;
      this.canvas.height = height;
    }
    this.#ratio = ratio;
  }

  #paintOf(flow: FlowField, theme: "light" | "dark", range: DisplayRange): FlowPaint {
    let cache = this.#paints.get(flow);
    if (cache === undefined) this.#paints.set(flow, (cache = new Map()));
    const key = `${theme}:${JSON.stringify(range)}`;
    let paint = cache.get(key);
    if (paint === undefined) cache.set(key, (paint = flowPaint(theme, flow, range)));
    return paint;
  }

  /** 🎞️ Draws one frame; a frame without a field or without layers clears the canvas. */
  draw(frame: FlowFrame): void {
    const c = this.#context;
    if (c === null) return;
    this.resize();
    c.setTransform(this.#ratio, 0, 0, this.#ratio, 0, 0);
    c.clearRect(0, 0, this.canvas.clientWidth, this.canvas.clientHeight);
    const { flow, layers, projection } = frame;
    if (flow === null || layers.length === 0) return;
    const want = new Set(layers);
    const paint = this.#paintOf(flow, frame.theme, frame.range);
    const marks = flowMarks(flow, frame.timeS, flowDetailOf(frame.denominator));
    const mm = projection.pxPerMm;
    const a: [number, number] = [0, 0];
    const b: [number, number] = [0, 0];
    const metre = (at: Vec3): number => {
      if (!projection.project(at, a) || !projection.project({ x: at.x, y: at.y + 1, z: at.z }, b)) return 0;
      const up = Math.hypot(b[0] - a[0], b[1] - a[1]);
      projection.project({ x: at.x + 0.7071, y: at.y, z: at.z + 0.7071 }, b);
      return Math.max(up, Math.hypot(b[0] - a[0], b[1] - a[1]));
    };
    const radiusOf = (at: Vec3, radiusM: number): number => Math.min(FLOW_STYLE.puffMaxMm * mm, Math.max(FLOW_STYLE.puffMinMm * mm, radiusM * metre(at)));
    c.lineCap = "round";
    c.lineJoin = "round";
    if (want.has("moisture"))
      for (const puff of marks.puffs) {
        const g = puff.vapourExcessKgPerKg * 1000;
        if (g <= FLOW_STYLE.vapourThresholdGKg) continue;
        const r = radiusOf(puff.at, puff.radiusM);
        if (r <= 0 || !projection.project(puff.at, a)) continue;
        const colour = paint.vapourColour(g);
        const gradient = c.createRadialGradient(a[0], a[1], 0, a[0], a[1], r);
        gradient.addColorStop(0, rgba(colour, paint.vapourOpacity(g)));
        gradient.addColorStop(0.55, rgba(colour, paint.vapourOpacity(g) * 0.55));
        gradient.addColorStop(1, rgba(colour, 0));
        c.fillStyle = gradient;
        c.beginPath();
        c.arc(a[0], a[1], r, 0, 2 * Math.PI);
        c.fill();
      }
    if (want.has("fog")) {
      const fog = marks.puffs.filter((puff) => puff.condensateKgPerKg > 0);
      c.strokeStyle = rgba(paint.fogRim, 0.9);
      c.lineWidth = Math.max(1, 0.3 * mm);
      for (const puff of fog) {
        if (!projection.project(puff.at, a)) continue;
        c.beginPath();
        c.arc(a[0], a[1], radiusOf(puff.at, puff.radiusM), 0, 2 * Math.PI);
        c.stroke();
      }
      for (const puff of fog) {
        if (!projection.project(puff.at, a)) continue;
        c.fillStyle = rgba(paint.fogColour, paint.fogOpacity(puff.condensateKgPerKg * 1000));
        c.beginPath();
        c.arc(a[0], a[1], radiusOf(puff.at, puff.radiusM), 0, 2 * Math.PI);
        c.fill();
      }
    }
    if (want.has("streams"))
      for (const ribbon of marks.ribbons) {
        let open = false;
        for (let i = 1; i < ribbon.points.length; i++) {
          if (!projection.project(ribbon.points[i - 1]!, a) || !projection.project(ribbon.points[i]!, b)) continue;
          const speed = (ribbon.speedMS[i - 1]! + ribbon.speedMS[i]!) / 2;
          c.strokeStyle = rgba(paint.excessColour((ribbon.excessK[i - 1]! + ribbon.excessK[i]!) / 2), paint.streamOpacity(speed));
          c.lineWidth = Math.max(1.1, paint.streamWidthMm(speed) * mm);
          c.beginPath();
          c.moveTo(a[0], a[1]);
          c.lineTo(b[0], b[1]);
          c.stroke();
          open = true;
        }
        if (!open) continue;
        for (const arrow of ribbon.arrows) {
          if (!projection.project(arrow.at, a) || !projection.project({ x: arrow.at.x + arrow.direction.x * 0.05, y: arrow.at.y + arrow.direction.y * 0.05, z: arrow.at.z + arrow.direction.z * 0.05 }, b)) continue;
          const dx = b[0] - a[0];
          const dy = b[1] - a[1];
          const l = Math.hypot(dx, dy);
          if (l < 0.2) continue;
          const ux = dx / l;
          const uy = dy / l;
          const length = Math.max(4, FLOW_STYLE.headLengthMm * mm);
          const half = Math.max(1.5, (FLOW_STYLE.headWidthMm * mm) / 2);
          c.fillStyle = rgba(paint.excessColour(ribbon.excessK[Math.max(0, ribbon.points.indexOf(arrow.at))]!), 0.95);
          c.beginPath();
          c.moveTo(a[0], a[1]);
          c.lineTo(a[0] - ux * length - uy * half, a[1] - uy * length + ux * half);
          c.lineTo(a[0] - ux * length + uy * half, a[1] - uy * length - ux * half);
          c.closePath();
          c.fill();
        }
      }
    if (want.has("moisture")) {
      c.setLineDash([0.01 * mm, Math.max(3, FLOW_STYLE.vapourDashMm[1] * mm)]);
      c.lineWidth = Math.max(1.5, FLOW_STYLE.vapourLineMm * mm);
      for (const ribbon of marks.ribbons)
        for (let i = 1; i < ribbon.points.length; i++) {
          const g = Math.min(ribbon.vapourExcessKgPerKg[i - 1]!, ribbon.vapourExcessKgPerKg[i]!) * 1000;
          if (g <= FLOW_STYLE.vapourThresholdGKg || !projection.project(ribbon.points[i - 1]!, a) || !projection.project(ribbon.points[i]!, b)) continue;
          c.strokeStyle = rgba(paint.vapourColour(g), 0.9);
          c.beginPath();
          c.moveTo(a[0], a[1]);
          c.lineTo(b[0], b[1]);
          c.stroke();
        }
      c.setLineDash([]);
    }
    if (want.has("tracers"))
      for (const streak of marks.streaks) {
        if (streak.opacity <= 0.02) continue;
        const colour = paint.excessColour(streak.excessK);
        const width = Math.max(2, FLOW_STYLE.tracerMm * mm);
        const n = streak.points.length;
        let drawn = false;
        for (let i = 1; i < n; i++) {
          if (!projection.project(streak.points[i - 1]!, a) || !projection.project(streak.points[i]!, b)) continue;
          c.strokeStyle = rgba(colour, 0.9 * streak.opacity * (0.25 + (0.75 * i) / (n - 1)));
          c.lineWidth = width * (0.5 + (0.5 * i) / (n - 1));
          c.beginPath();
          c.moveTo(a[0], a[1]);
          c.lineTo(b[0], b[1]);
          c.stroke();
          drawn = true;
        }
        if (!drawn) continue;
        const g = streak.vapourExcessKgPerKg * 1000;
        const head = Math.max(1.8, FLOW_STYLE.tracerHeadMm * mm);
        c.beginPath();
        if (streak.condensateKgPerKg > 0) {
          c.arc(b[0], b[1], head * 1.3, 0, 2 * Math.PI);
          c.fillStyle = rgba(paint.fogColour, streak.opacity);
          c.fill();
          c.strokeStyle = rgba(paint.fogRim, streak.opacity);
          c.lineWidth = 1;
          c.stroke();
        } else if (want.has("moisture") && g > FLOW_STYLE.vapourThresholdGKg) {
          c.arc(b[0], b[1], head * 1.3, 0, 2 * Math.PI);
          c.strokeStyle = rgba(paint.vapourColour(g), streak.opacity);
          c.lineWidth = Math.max(1, 0.14 * mm);
          c.stroke();
        } else {
          c.arc(b[0], b[1], head, 0, 2 * Math.PI);
          c.fillStyle = rgba(colour, streak.opacity);
          c.fill();
        }
      }
  }
}
