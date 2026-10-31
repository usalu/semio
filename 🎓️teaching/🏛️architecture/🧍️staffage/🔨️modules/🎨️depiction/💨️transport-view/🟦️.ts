/** 💨️ The transport overlay: parcels, pathlines, instantaneous streamlines, modelled visible condensation and emitter markers drawn over any view, with legends that name what each layer is.
 *
 * - `parcels`: discs of the parcel radius coloured by temperature excess (K, diverging, warm above ambient) for airflow parcels or by vapour excess (g/kg, sequential) for vapour carriers.
 * - `pathlines`: thin solid lines, the path each parcel actually travelled since its emission ("particle pathlines").
 * - `streamlines`: dashed lines with arrow heads tangent to the velocity field at this instant ("instantaneous streamlines"); in unsteady flow they differ from pathlines.
 * - `condensation`: hollow cyan discs with a white veil, the separately modelled visible condensation (supersaturated mixture), not the analytical parcels.
 * Emitter markers (arrow along the outflow direction, filled while flowing, outlined otherwise) and their flow label are drawn whenever any layer is shown.
 * Shared comparison ranges (`range.mode = "shared"`): temperature excess ±20 K, vapour excess 0…20 g/kg; `auto` spans the values present and the legend says it is normalised.
 *
 * @see ../../🔗️contracts/🟦️.ts — `TransportField`, `TransportLayer`, `Legend`
 * @see ../../💨️transport/🟦️.ts — `solveTransport`
 */
import type { Language } from "../../../🧬️schema/🟦️.ts";
import type { DisplayRange, Legend, Parcel, TransportField, TransportLayer } from "../../🔗️contracts/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { type Vec, arrowOutline, circleCommands, commandsOf, headOutline } from "../📐️geometry/🟦️.ts";
import { divergingColour, mix, overlayColours, tx } from "../🖍️palette/🟦️.ts";

/** 🎚️ Shared comparison span of the temperature excess (K, ±) and of the vapour excess (g/kg). */
export const SHARED_EXCESS_K = 20;
export const SHARED_VAPOUR_GKG = 20;

const MIN_DISC_MM = 0.15;
const MAX_DISC_MM = 12;
const MARKER_MM = 5;
const LABEL_MM = 1.6;

type Result = { readonly overlay: (canvas: Canvas) => void; readonly legends: readonly Legend[] };

const stopsOf = (min: number, max: number, colourAt: (v: number) => string): Legend["stops"] => Array.from({ length: 9 }, (_, i) => ({ at: i / 8, color: colourAt(min + ((max - min) * i) / 8) }));

function spanOf(values: readonly number[], shared: number, range: DisplayRange): { readonly min: number; readonly max: number } {
  if (range.mode === "fixed") return { min: range.min, max: range.max };
  return range.mode === "shared" ? { min: -shared, max: shared } : { min: -Math.max(1, ...values.map(Math.abs)), max: Math.max(1, ...values.map(Math.abs)) };
}

/** 💨️ Builds the transport overlay and its legends for the requested layers. */
export function transportView(ctx: Ctx, field: TransportField, layers: readonly TransportLayer[], range: DisplayRange): Result {
  const { colours, theme, k } = ctx;
  const language: Language = ctx.language;
  const want = new Set(layers);
  const overlays = overlayColours(theme);
  const airParcels = field.parcels.filter((p) => p.carries === "airflow");
  const vapourParcels = field.parcels.filter((p) => p.carries === "vapour");
  const temperature = spanOf(airParcels.map((p) => p.excessK), SHARED_EXCESS_K, range);
  const vapour = spanOf(vapourParcels.map((p) => p.vapourExcessKgPerKg * 1000), SHARED_VAPOUR_GKG, range);
  const temperatureColour = (v: number): string => divergingColour(v, { min: temperature.min, centre: 0, max: temperature.max }, true, theme);
  const vapourColour = (v: number): string => mix(colours.background, overlays.vapour, 0.2 + (0.8 * Math.min(1, Math.max(0, v))) / Math.max(1e-9, vapour.max));
  const title = (id: string) => ctx.state.asset.thermal.emitters.find((e) => e.id === id)?.title ?? tx(id, id);
  const mode = range.mode;

  const disc = (canvas: Canvas, p: Parcel, i: number): void => {
    const at = ctx.paper(p.at);
    const radius = Math.min(MAX_DISC_MM, Math.max(MIN_DISC_MM, p.radiusM * k));
    const isVapour = p.carries === "vapour";
    const value = isVapour ? p.vapourExcessKgPerKg * 1000 : p.excessK;
    const color = isVapour ? vapourColour(value) : temperatureColour(value);
    canvas.path("field", `parcel:${p.id}:${i}`, circleCommands(at, radius), {
      fill: { color, opacity: 0.35, ...(isVapour ? { pattern: "vapour" as const } : {}) },
      stroke: { color, widthMm: 0.08, opacity: 0.8 },
      value: { quantity: isVapour ? "vapour-excess" : "excess-temperature", value, unit: isVapour ? "g/kg" : "K", label: title(p.emitter) },
    });
  };

  const overlay = (canvas: Canvas): void => {
    if (want.has("pathlines"))
      field.pathlines.forEach((line, i) => {
        if (line.points.length < 2) return;
        canvas.path("field", `pathline:${line.parcel}`, commandsOf(line.points.map((q) => ctx.paper(q)), false), { stroke: { color: colours.softInk, widthMm: 0.08, opacity: 0.55, cap: "round", join: "round" }, value: { quantity: "pathline-age", value: line.ages[line.ages.length - 1] ?? 0, unit: "s", label: tx(`Particle pathline ${i + 1} of ${title(line.emitter).en}`, `Teilchenbahn ${i + 1} von ${title(line.emitter).de}`) } });
      });
    if (want.has("parcels")) field.parcels.forEach((p, i) => disc(canvas, p, i));
    if (want.has("condensation"))
      field.condensation.forEach((c, i) => {
        const at = ctx.paper(c.at);
        const radius = Math.min(MAX_DISC_MM, Math.max(MIN_DISC_MM, c.radiusM * k));
        canvas.path("field", `condensation:${i}`, circleCommands(at, radius), { fill: { color: colours.ink, opacity: 0.12 }, stroke: { color: "#34d1bf", widthMm: 0.18, opacity: 0.95 }, value: { quantity: "condensate", value: c.condensateKgPerKg * 1000, unit: "g/kg", label: tx("Modelled visible condensation", "Modellierte sichtbare Kondensation") } });
      });
    if (want.has("streamlines"))
      field.streamlines.forEach((line, i) => {
        const pts: Vec[] = line.points.map((q) => ctx.paper(q));
        canvas.path("field", `streamline:${i}`, commandsOf(pts, false), { stroke: { color: colours.ink, widthMm: 0.1, dashMm: [1.2, 0.8], opacity: 0.7, cap: "butt" }, value: { quantity: "speed", value: Math.max(...line.speedMS), unit: "m/s", label: tx("Instantaneous streamline", "Momentane Stromlinie") } });
        const a = pts[pts.length - 2];
        const b = pts[pts.length - 1];
        if (a !== undefined && b !== undefined) canvas.path("field", `streamline-head:${i}`, commandsOf(headOutline(b, { x: b.x - a.x, y: b.y - a.y }, 0.9, 0.6), true), { fill: { color: colours.ink, opacity: 0.7 } });
      });
    field.emitters.forEach((e) => {
      const at = ctx.paper(e.at);
      const d: Vec = { x: Math.cos(e.directionRad), y: -Math.sin(e.directionRad) };
      const tip = { x: at.x + d.x * MARKER_MM, y: at.y + d.y * MARKER_MM };
      const value = { quantity: "volume-flow", value: e.volumeFlowM3S * 1000, unit: "L/s", label: title(e.emitter) };
      canvas.path("annotations", `emitter:${e.emitter}`, commandsOf(arrowOutline(at, tip, 0.2, 1.1, 0.8), true), e.active ? { fill: { color: colours.ink, opacity: 0.9 }, value } : { stroke: { color: colours.softInk, widthMm: 0.12 }, value });
      if (e.active) canvas.text("annotations", `emitter-label:${e.emitter}`, { x: tip.x + 0.8, y: tip.y }, `${title(e.emitter)[language]} ${(e.volumeFlowM3S * 1000).toFixed(e.volumeFlowM3S < 0.01 ? 2 : 1)} L/s · ${e.exitVelocityMS.toFixed(1)} m/s`, LABEL_MM, colours.ink);
    });
    const tag = (id: string, text: string, p: Vec | undefined): void => {
      if (p !== undefined) canvas.text("annotations", id, { x: p.x + 1, y: p.y }, text, LABEL_MM, colours.softInk);
    };
    if (want.has("streamlines")) {
      const longest = [...field.streamlines].sort((a, b) => b.points.length - a.points.length)[0];
      tag("label:streamlines", tx("instantaneous streamlines", "momentane Stromlinien")[language], longest === undefined ? undefined : ctx.paper(longest.points[longest.points.length - 1]!));
    }
    if (want.has("pathlines")) {
      const longest = [...field.pathlines].sort((a, b) => b.points.length - a.points.length)[0];
      tag("label:pathlines", tx("particle pathlines", "Teilchenbahnen")[language], longest === undefined ? undefined : ctx.paper(longest.points[longest.points.length - 1]!));
    }
    if (want.has("condensation")) {
      const first = field.condensation[0];
      tag("label:condensation", tx("modelled visible condensation", "modellierte sichtbare Kondensation")[language], first === undefined ? undefined : ctx.paper(first.at));
    }
  };

  const normalised = mode === "auto" ? tx(" Range normalised to the values present.", " Bereich auf die vorhandenen Werte normiert.") : tx("", "");
  const model = field.notes[0] ?? tx("Integral air-transport model, not CFD.", "Integrales Luftbewegungsmodell, keine CFD.");
  const legends: Legend[] = [];
  if (want.has("parcels") && airParcels.length > 0)
    legends.push({
      title: tx("Parcel temperature excess", "Teilchen-Übertemperatur"),
      quantity: "excess-temperature",
      unit: "K",
      convention: tx("Diverging scale centred on the ambient air temperature (neutral grey): warm = warmer than the ambient air, cool = colder.", "Divergierende Skala mit der Umgebungslufttemperatur als Mitte (neutrales Grau): warm = wärmer als die Umgebungsluft, kühl = kälter."),
      min: temperature.min,
      max: temperature.max,
      stops: stopsOf(temperature.min, temperature.max, temperatureColour),
      interpretation: tx(`Each disc is one air parcel released by an emitter, drawn with its entrained radius at time ${field.timeS.toFixed(2)} s.${normalised.en}`, `Jede Scheibe ist ein von einem Emitter freigesetztes Luftteilchen, gezeichnet mit seinem Mischradius zur Zeit ${field.timeS.toFixed(2)} s.${normalised.de}`),
      range: mode,
      conditions: model,
    });
  if (want.has("parcels") && vapourParcels.length > 0)
    legends.push({
      title: tx("Parcel vapour excess", "Teilchen-Feuchteüberschuss"),
      quantity: "vapour-excess",
      unit: "g/kg",
      convention: tx("Sequential scale from the paper colour (no excess) to blue-green (strong vapour excess over the ambient air); hatched discs carry vapour, not heat.", "Sequenzielle Skala von der Papierfarbe (kein Überschuss) zu Blaugrün (starker Feuchteüberschuss gegenüber der Umgebungsluft); schraffierte Scheiben tragen Wasserdampf, keine Wärme."),
      min: 0,
      max: vapour.max,
      stops: stopsOf(0, vapour.max, vapourColour),
      interpretation: tx(`Specific-humidity excess of transpired vapour parcels over the ambient air.${normalised.en}`, `Überschuss der spezifischen Feuchte transpirierter Dampfteilchen über der Umgebungsluft.${normalised.de}`),
      range: mode,
      conditions: model,
    });
  const line = (title: ReturnType<typeof tx>, quantity: string, unit: string, max: number, convention: ReturnType<typeof tx>, interpretation: ReturnType<typeof tx>, color: string): Legend => ({ title, quantity, unit, convention, min: 0, max, stops: [{ at: 0, color }, { at: 1, color }], interpretation, range: mode, conditions: model });
  if (want.has("streamlines") && field.streamlines.length > 0)
    legends.push(line(tx("Instantaneous streamlines", "Momentane Stromlinien"), "speed", "m/s", Math.max(0.01, ...field.streamlines.flatMap((s) => s.speedMS)), tx("Dashed lines with arrows, tangent to the velocity field at this instant (ambient wind, jets, plumes, inhalation sinks).", "Gestrichelte Linien mit Pfeilen, tangential zum Geschwindigkeitsfeld in diesem Augenblick (Umgebungswind, Strahlen, Fahnen, Einatemsenken)."), tx("Not particle paths: in unsteady flow streamlines and pathlines differ.", "Keine Teilchenbahnen: in instationärer Strömung unterscheiden sich Stromlinien und Bahnen."), colours.ink));
  if (want.has("pathlines") && field.pathlines.length > 0)
    legends.push(line(tx("Particle pathlines", "Teilchenbahnen"), "age", "s", Math.max(0.01, ...field.pathlines.map((p) => p.ages[p.ages.length - 1] ?? 0)), tx("Thin solid lines: the path each parcel travelled from its emission to now, fixed in world space.", "Dünne durchgezogene Linien: der Weg, den jedes Teilchen von der Emission bis jetzt zurückgelegt hat, im Weltraum fixiert."), tx("Earlier parcels do not follow the emitter when it moves or turns.", "Frühere Teilchen folgen dem Emitter nicht, wenn er sich bewegt oder dreht."), colours.softInk));
  if (want.has("condensation") && field.condensation.length > 0)
    legends.push(line(tx("Modelled visible condensation", "Modellierte sichtbare Kondensation"), "condensate", "g/kg", Math.max(0.01, ...field.condensation.map((c) => c.condensateKgPerKg * 1000)), tx("Cyan-outlined discs: parcels whose mixture of exhaled and ambient air is supersaturated.", "Türkis umrandete Scheiben: Teilchen, deren Mischung aus Ausatem- und Umgebungsluft übersättigt ist."), tx("Liquid water per kg of moist air from saturation adjustment of the mixing line; modelled, separate from the analytical parcels.", "Flüssiges Wasser je kg feuchter Luft aus der Sättigungsanpassung der Mischgeraden; modelliert, getrennt von den analytischen Teilchen."), "#34d1bf"));
  return { overlay, legends };
}
