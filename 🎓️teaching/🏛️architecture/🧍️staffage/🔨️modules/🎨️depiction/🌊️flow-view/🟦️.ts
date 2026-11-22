/** 🌊️ The thermodynamic flow view: the flow field of the transport model drawn over any view as stream ribbons, advected tracers, vapour clouds and fog, with legends in the units of what each mark shows.
 *
 * - `streams`: every kept stream is a line tangent to the velocity of the instant, coloured along its length by the temperature excess of the air over the ambient air (diverging, warm above, neutral for entrained ambient air), as wide as the local speed asks ({@link FLOW_STYLE}) and marked with arrow heads in the direction of flow.
 * - `tracers`: massless particles advected along the streams; a streak is the path of the last `exposureS` seconds, so its length is the speed and its bright head leads; it fades in near the start and out near the end of its stream.
 * - `moisture`: air that carries more vapour than the ambient air, as dotted blue-green lines along the streams and translucent clouds of the vapour puffs; the dotted line and the hollow tracer heads are the colour-independent reading.
 * - `fog`: puffs whose mixture is below its dew point, the only visible water: white discs whose common outline is a cyan rim (all rims are drawn first, the fills cover the inner ones), under the streams.
 * Ranges: shared comparison spans of ±{@link SHARED_FLOW_EXCESS_K} K, 0…{@link SHARED_FLOW_VAPOUR_GKG} g/kg vapour and 0…{@link SHARED_FLOW_FOG_GKG} g/kg condensate; `auto` spans the values present (the temperature excess between ±0.5 K and the shared span, so a hot jet never greys out the plume beside it) and says so; `fixed` and `span` give the temperature span.
 * The paint ({@link flowPaint}) and the generalisation ({@link flowDetailOf}) are shared with the screen renderer of the web target, so paper, screen and 3D show the same marks in the same colours.
 *
 * @see ../../💨️transport/🌊️flow/🟦️.ts — field, streams, tracers
 * @see ../../🔗️contracts/🟦️.ts — `FlowField`, `FlowMarks`, `FlowLayer`
 */
import type { DisplayRange, FlowField, FlowLayer, FlowMarks, Legend, Vec3 } from "../../🔗️contracts/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { type Vec, circleCommands, commandsOf, headOutline } from "../📐️geometry/🟦️.ts";
import { type Theme, THEMES, divergingColour, mix, overlayColours, tx } from "../🖍️palette/🟦️.ts";
import { projector } from "../💨️transport-view/🟦️.ts";
import { flowDetailOf, flowMarks } from "../../💨️transport/🌊️flow/🟦️.ts";
import { formatNumber } from "../../📐️units/🟦️.ts";

/** 🎚️ Shared comparison span of the temperature excess of the flow (K, ±), of its vapour excess and of its condensate (g/kg). */
export const SHARED_FLOW_EXCESS_K = 5;
export const SHARED_FLOW_VAPOUR_GKG = 20;
export const SHARED_FLOW_FOG_GKG = 2;

/** 🩶️ The share of the temperature span around zero in which the colour blends from the neutral grey of ambient air to the first warm or cool colour. */
export const NEUTRAL_BAND = 0.12;

/** 📏️ The paper sizes of the flow marks in millimetres: stream width from `streamMinMm` (still air) to `streamMaxMm` (the fastest air of the field), arrow head length and width, tracer width and head radius, the dotted vapour line, the clamp of puff radii and the shells of a vapour cloud (share of the radius, share of the opacity; three stacked discs fade towards the edge as a Gaussian puff does). */
export const FLOW_STYLE = { streamMinMm: 0.09, streamMaxMm: 0.3, headLengthMm: 1.1, headWidthMm: 0.75, tracerMm: 0.26, tracerHeadMm: 0.24, vapourLineMm: 0.3, vapourDashMm: [0.01, 0.75], puffMinMm: 0.3, puffMaxMm: 14, vapourThresholdGKg: 0.3, bins: 9, puffShells: [[1, 0.3], [0.72, 0.35], [0.42, 0.35]] } as const;

/** 🎨️ The colours and sizes of the marks of one flow field: a pure function of theme, field and range, used by every renderer. */
export type FlowPaint = {
  readonly excess: { readonly min: number; readonly max: number };
  readonly vapourMaxGKg: number;
  readonly fogMaxGKg: number;
  readonly speedMaxMS: number;
  readonly mode: DisplayRange["mode"];
  readonly excessColour: (excessK: number) => string;
  readonly vapourColour: (gKg: number) => string;
  readonly vapourOpacity: (gKg: number) => number;
  readonly fogColour: string;
  readonly fogRim: string;
  readonly fogOpacity: (gKg: number) => number;
  readonly streamWidthMm: (speedMS: number) => number;
  readonly streamOpacity: (speedMS: number) => number;
};

/** 🎨️ The paint of a flow field under a display range (see the module text for the spans). */
export function flowPaint(theme: Theme, flow: FlowField, range: DisplayRange): FlowPaint {
  const colours = THEMES[theme];
  const excesses = flow.streams.flatMap((s) => s.excessK);
  const span = range.mode === "auto" ? Math.min(SHARED_FLOW_EXCESS_K, Math.max(0.5, ...excesses.map(Math.abs))) : range.mode === "span" ? range.spanK : SHARED_FLOW_EXCESS_K;
  const excess = range.mode === "fixed" ? { min: range.min - flow.ambient.airTempC, max: range.max - flow.ambient.airTempC } : { min: -span, max: span };
  const auto = range.mode === "auto";
  const vapourMaxGKg = auto ? Math.max(1, ...flow.puffs.map((p) => p.vapourExcessKgPerKg * 1000)) : SHARED_FLOW_VAPOUR_GKG;
  const fogMaxGKg = auto ? Math.max(0.1, ...flow.puffs.map((p) => p.condensateKgPerKg * 1000)) : SHARED_FLOW_FOG_GKG;
  const speedMaxMS = Math.max(0.05, ...flow.streams.flatMap((s) => s.speedMS));
  const vapour = overlayColours(theme).vapour;
  const neutral = mix(colours.softInk, colours.background, 0.2);
  const unit = (v: number, max: number): number => Math.min(1, Math.max(0, v / Math.max(1e-9, max)));
  return {
    excess,
    vapourMaxGKg,
    fogMaxGKg,
    speedMaxMS,
    mode: range.mode === "span" ? "shared" : range.mode,
    excessColour: (v) => {
      const scale = { min: Math.min(excess.min, -1e-9), centre: 0, max: Math.max(excess.max, 1e-9) };
      const f = Math.min(1, Math.abs(v) / (v > 0 ? scale.max : -scale.min));
      return f < NEUTRAL_BAND ? mix(neutral, divergingColour(Math.sign(v) * 1e-9, scale, true, theme), f / NEUTRAL_BAND) : divergingColour(v, scale, true, theme);
    },
    vapourColour: (g) => mix(mix(colours.background, vapour, 0.45), vapour, unit(g, vapourMaxGKg)),
    vapourOpacity: (g) => 0.08 + 0.3 * Math.sqrt(unit(g, vapourMaxGKg)),
    fogColour: theme === "dark" ? "#e9f1ef" : "#ffffff",
    fogRim: "#34d1bf",
    fogOpacity: (g) => 0.55 + 0.4 * Math.sqrt(unit(g, fogMaxGKg)),
    streamWidthMm: (speed) => FLOW_STYLE.streamMinMm + (FLOW_STYLE.streamMaxMm - FLOW_STYLE.streamMinMm) * Math.sqrt(unit(speed, speedMaxMS)),
    streamOpacity: (speed) => 0.4 + 0.5 * Math.sqrt(unit(speed, speedMaxMS)),
  };
}

const EXPLANATORY = tx("Explanatory visualization: moving air, its warmth and water vapour are invisible; lines, streaks and clouds show the modelled field. Only the fog marks depict water one could see.", "Erklärende Visualisierung: bewegte Luft, ihre Wärme und Wasserdampf sind unsichtbar; Linien, Streifen und Wolken zeigen das modellierte Feld. Nur die Nebelzeichen stellen sichtbares Wasser dar.");

const stopsOf = (min: number, max: number, colourAt: (v: number) => string): Legend["stops"] => Array.from({ length: 9 }, (_, i) => ({ at: i / 8, color: colourAt(min + ((max - min) * i) / 8) }));

/** 🧾️ The legends of the flow view for the layers shown: temperature excess (K), speed (m/s), vapour excess (g/kg) and condensate (g/kg), each with the ambient state it is read against. */
export function flowLegends(flow: FlowField, marks: FlowMarks, paint: FlowPaint, layers: readonly FlowLayer[]): Legend[] {
  const want = new Set(layers);
  const a = flow.ambient;
  const n = (value: number, digits: number, language: "en" | "de"): string => formatNumber(value, digits, language);
  const conditions = tx(
    `Ambient air ${n(a.airTempC, 1, "en")} °C, ${n(a.relativeHumidity * 100, 0, "en")} % relative humidity, dew point ${n(a.dewPointC, 1, "en")} °C, air speed ${n(a.windMS, 2, "en")} m/s; field at ${n(flow.timeS, 2, "en")} s. Integral jet and plume model with entrainment, not CFD.`,
    `Umgebungsluft ${n(a.airTempC, 1, "de")} °C, ${n(a.relativeHumidity * 100, 0, "de")} % relative Feuchte, Taupunkt ${n(a.dewPointC, 1, "de")} °C, Luftgeschwindigkeit ${n(a.windMS, 2, "de")} m/s; Feld bei ${n(flow.timeS, 2, "de")} s. Integrales Strahl- und Fahnenmodell mit Einmischung, keine CFD.`,
  );
  const normalised = paint.mode === "auto" ? tx(" Range normalised to the values present.", " Bereich auf die vorhandenen Werte normiert.") : tx("", "");
  const moving = marks.ribbons.length > 0;
  const legends: Legend[] = [];
  if ((want.has("streams") || want.has("tracers")) && moving) {
    legends.push({
      title: tx("Air temperature excess of the flow", "Übertemperatur der strömenden Luft"),
      quantity: "flow-excess-temperature",
      unit: "K",
      convention: tx("Diverging scale centred on the ambient air (neutral grey): warm colours are air warmer than the surroundings, which rises; cool colours are colder air, which sinks; grey lines are ambient air drawn into a jet or plume or moved by the wind.", "Divergierende Skala mit der Umgebungsluft als Mitte (neutrales Grau): warme Farben sind Luft wärmer als die Umgebung, sie steigt; kühle Farben sind kältere Luft, sie sinkt; graue Linien sind Umgebungsluft, die in einen Strahl oder eine Fahne eingemischt oder vom Wind bewegt wird."),
      min: paint.excess.min,
      max: paint.excess.max,
      stops: stopsOf(paint.excess.min, paint.excess.max, paint.excessColour),
      interpretation: tx(`Colour of stream lines and tracers; values beyond the range take the end colour.${normalised.en}`, `Farbe der Stromlinien und Teilchen; Werte außerhalb des Bereichs erhalten die Endfarbe.${normalised.de}`),
      range: paint.mode,
      conditions,
      assumptions: [EXPLANATORY],
    });
    const ink = paint.excessColour(0);
    legends.push({
      title: tx("Air speed and direction", "Luftgeschwindigkeit und Richtung"),
      quantity: "flow-speed",
      unit: "m/s",
      convention: tx(`Stream lines follow the velocity of this instant and are ${FLOW_STYLE.streamMinMm} to ${FLOW_STYLE.streamMaxMm} mm wide from still to the fastest air; arrow heads and the bright heads of the tracers point where the air goes.`, `Stromlinien folgen der Geschwindigkeit dieses Augenblicks und sind von ruhender bis zur schnellsten Luft ${formatNumber(FLOW_STYLE.streamMinMm, 2, "de")} bis ${formatNumber(FLOW_STYLE.streamMaxMm, 1, "de")} mm breit; Pfeilspitzen und die hellen Köpfe der Teilchen zeigen, wohin die Luft strömt.`),
      min: 0,
      max: paint.speedMaxMS,
      stops: [{ at: 0, color: ink }, { at: 1, color: ink }],
      interpretation: tx(`A tracer streak is the path of the last ${n(flow.model.exposureS, 1, "en")} s: its length is speed times that time (never shorter than 0.9 mm of paper); one tracer is released every ${n(flow.model.periodS, 1, "en")} s on each line.`, `Ein Teilchenstreifen ist der Weg der letzten ${n(flow.model.exposureS, 1, "de")} s: seine Länge ist Geschwindigkeit mal dieser Zeit (nie kürzer als 0,9 mm auf dem Papier); auf jeder Linie startet alle ${n(flow.model.periodS, 1, "de")} s ein Teilchen.`),
      range: "auto",
      conditions,
    });
  }
  if (want.has("moisture") && marks.puffs.some((p) => p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg))
    legends.push({
      title: tx("Water vapour excess", "Wasserdampfüberschuss"),
      quantity: "flow-vapour-excess",
      unit: "g/kg",
      convention: tx("Blue-green clouds and dotted lines: air that carries more water vapour than the ambient air (evaporation, transpiration, breath, steam); denser colour is more vapour.", "Blaugrüne Wolken und gepunktete Linien: Luft, die mehr Wasserdampf trägt als die Umgebungsluft (Verdunstung, Transpiration, Atem, Dampf); dichtere Farbe bedeutet mehr Dampf."),
      min: 0,
      max: paint.vapourMaxGKg,
      stops: stopsOf(0, paint.vapourMaxGKg, paint.vapourColour),
      interpretation: tx(`Specific-humidity excess over the ambient air; vapour itself is invisible.${normalised.en}`, `Überschuss der spezifischen Feuchte über der Umgebungsluft; Dampf selbst ist unsichtbar.${normalised.de}`),
      range: paint.mode,
      conditions,
      assumptions: [EXPLANATORY],
    });
  if (want.has("fog") && marks.puffs.some((p) => p.condensateKgPerKg > 0))
    legends.push({
      title: tx("Fog: condensed water", "Nebel: kondensiertes Wasser"),
      quantity: "flow-condensate",
      unit: "g/kg",
      convention: tx("White clouds with a cyan rim: the mixture of moist and ambient air is colder than its dew point, so droplets form; they vanish again where more dry air is mixed in.", "Weiße Wolken mit türkisem Rand: die Mischung aus feuchter und Umgebungsluft ist kälter als ihr Taupunkt, es bilden sich Tröpfchen; sie verschwinden wieder, wo mehr trockene Luft eingemischt wird."),
      min: 0,
      max: paint.fogMaxGKg,
      stops: [{ at: 0, color: mix(paint.fogColour, paint.fogRim, 0.15) }, { at: 1, color: paint.fogColour }],
      interpretation: tx(`Liquid water per kilogram of moist air from the saturation adjustment of the mixing line.${normalised.en}`, `Flüssiges Wasser je Kilogramm feuchter Luft aus der Sättigungsanpassung der Mischgeraden.${normalised.de}`),
      range: paint.mode,
      conditions,
    });
  return legends;
}

type Result = { readonly overlay: (canvas: Canvas) => void; readonly legends: readonly Legend[] };

/** 🖌️ Builds the flow overlay and its legends for the requested layers at the scale and viewpoint of the context. */
export function flowView(ctx: Ctx, flow: FlowField, layers: readonly FlowLayer[], range: DisplayRange): Result {
  const { theme, k } = ctx;
  const paper = projector(ctx);
  const want = new Set(layers);
  const marks = flowMarks(flow, flow.timeS, flowDetailOf(ctx.options.scale.denominator));
  const paint = flowPaint(theme, flow, range);
  const bin = (excessK: number): number => Math.round(((FLOW_STYLE.bins - 1) * (Math.min(paint.excess.max, Math.max(paint.excess.min, excessK)) - paint.excess.min)) / Math.max(1e-9, paint.excess.max - paint.excess.min));
  const binColour = (index: number): string => paint.excessColour(paint.excess.min + ((paint.excess.max - paint.excess.min) * index) / (FLOW_STYLE.bins - 1));
  const radius = (radiusM: number): number => Math.min(FLOW_STYLE.puffMaxMm, Math.max(FLOW_STYLE.puffMinMm, radiusM * k));
  const overlay = (canvas: Canvas): void => {
    if (want.has("moisture"))
      marks.puffs.forEach((p, i) => {
        const g = p.vapourExcessKgPerKg * 1000;
        if (g <= FLOW_STYLE.vapourThresholdGKg) return;
        const at = paper(p.at);
        FLOW_STYLE.puffShells.forEach(([share, weight], shell) => canvas.path("field", `flow-vapour:${i}:${shell}`, circleCommands(at, radius(p.radiusM) * share), { fill: { color: paint.vapourColour(g), opacity: paint.vapourOpacity(g) * weight }, ...(shell === 0 ? { value: { quantity: "flow-vapour-excess", value: g, unit: "g/kg", label: tx("Water vapour excess", "Wasserdampfüberschuss") } } : {}) }));
      });
    if (want.has("fog")) {
      const fog = marks.puffs.filter((p) => p.condensateKgPerKg > 0);
      fog.forEach((p, i) => canvas.path("field", `flow-fog-rim:${i}`, circleCommands(paper(p.at), radius(p.radiusM)), { stroke: { color: paint.fogRim, widthMm: 0.3, opacity: 0.9 } }));
      fog.forEach((p, i) => {
        const g = p.condensateKgPerKg * 1000;
        canvas.path("field", `flow-fog:${i}`, circleCommands(paper(p.at), radius(p.radiusM)), { fill: { color: paint.fogColour, opacity: paint.fogOpacity(g) }, value: { quantity: "flow-condensate", value: g, unit: "g/kg", label: tx("Fog: condensed water", "Nebel: kondensiertes Wasser") } });
      });
    }
    if (want.has("streams"))
      marks.ribbons.forEach((ribbon) => {
        const pts: Vec[] = ribbon.points.map((q: Vec3) => paper(q));
        let start = 0;
        for (let i = 1; i <= pts.length; i++) {
          if (i < pts.length && bin(ribbon.excessK[i]!) === bin(ribbon.excessK[start]!)) continue;
          const end = Math.min(pts.length - 1, i);
          if (end > start) {
            const speed = ribbon.speedMS.slice(start, end + 1).reduce((sum, v) => sum + v, 0) / (end + 1 - start);
            const peak = ribbon.excessK.slice(start, end + 1).reduce((best, v) => (Math.abs(v) > Math.abs(best) ? v : best), 0);
            canvas.path("field", `flow-stream:${ribbon.stream}:${start}`, commandsOf(pts.slice(start, end + 1), false), { stroke: { color: binColour(bin(ribbon.excessK[start]!)), widthMm: paint.streamWidthMm(speed), opacity: paint.streamOpacity(speed), cap: "round", join: "round" }, value: { quantity: "flow-excess-temperature", value: peak, unit: "K", label: tx(`Stream line ${ribbon.stream + 1}, ${formatNumber(speed, 2, "en")} m/s`, `Stromlinie ${ribbon.stream + 1}, ${formatNumber(speed, 2, "de")} m/s`) } });
          }
          start = end;
          if (i >= pts.length - 1) break;
        }
        ribbon.arrows.forEach((arrow, j) => {
          const tip = paper(arrow.at);
          const ahead = paper({ x: arrow.at.x + arrow.direction.x, y: arrow.at.y + arrow.direction.y, z: arrow.at.z + arrow.direction.z });
          const d = { x: ahead.x - tip.x, y: ahead.y - tip.y };
          if (Math.abs(d.x) + Math.abs(d.y) < 0.2 * k) return;
          const index = ribbon.points.indexOf(arrow.at);
          canvas.path("field", `flow-arrow:${ribbon.stream}:${j}`, commandsOf(headOutline(tip, d, FLOW_STYLE.headLengthMm, FLOW_STYLE.headWidthMm), true), { fill: { color: binColour(bin(ribbon.excessK[Math.max(0, index)]!)), opacity: 0.9 } });
        });
      });
    if (want.has("moisture"))
      marks.ribbons.forEach((ribbon) => {
        let run: Vec[] = [];
        let peak = 0;
        const flush = (): void => {
          if (run.length > 1) canvas.path("field", `flow-vapour-line:${ribbon.stream}`, commandsOf(run, false), { stroke: { color: paint.vapourColour(peak), widthMm: FLOW_STYLE.vapourLineMm, dashMm: FLOW_STYLE.vapourDashMm, opacity: 0.9, cap: "round" } });
          run = [];
          peak = 0;
        };
        ribbon.points.forEach((q, i) => {
          const g = ribbon.vapourExcessKgPerKg[i]! * 1000;
          if (g > FLOW_STYLE.vapourThresholdGKg) {
            run.push(paper(q));
            peak = Math.max(peak, g);
          } else flush();
        });
        flush();
      });
    if (want.has("tracers"))
      marks.streaks.forEach((streak, i) => {
        if (streak.opacity <= 0.02) return;
        const pts = streak.points.map((q) => paper(q));
        const head = pts[pts.length - 1]!;
        const g = streak.vapourExcessKgPerKg * 1000;
        const color = paint.excessColour(streak.excessK);
        canvas.path("field", `flow-tracer:${i}`, commandsOf(pts, false), { stroke: { color, widthMm: FLOW_STYLE.tracerMm, opacity: 0.85 * streak.opacity, cap: "round", join: "round" } });
        if (streak.condensateKgPerKg > 0) canvas.path("field", `flow-tracer-head:${i}`, circleCommands(head, FLOW_STYLE.tracerHeadMm * 1.3), { fill: { color: paint.fogColour, opacity: streak.opacity }, stroke: { color: paint.fogRim, widthMm: 0.1, opacity: streak.opacity } });
        else if (want.has("moisture") && g > FLOW_STYLE.vapourThresholdGKg) canvas.path("field", `flow-tracer-head:${i}`, circleCommands(head, FLOW_STYLE.tracerHeadMm * 1.3), { stroke: { color: paint.vapourColour(g), widthMm: 0.14, opacity: streak.opacity } });
        else canvas.path("field", `flow-tracer-head:${i}`, circleCommands(head, FLOW_STYLE.tracerHeadMm), { fill: { color, opacity: streak.opacity } });
      });
  };
  return { overlay, legends: flowLegends(flow, marks, paint, layers) };
}
