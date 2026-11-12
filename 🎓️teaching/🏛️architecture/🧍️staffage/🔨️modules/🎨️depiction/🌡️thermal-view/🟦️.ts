/** 🌡️ The thermal view: every shape is painted by the solved physical quantity of its thermal region (or the area-weighted value of the regions of a merged small-scale form) on a
 * diverging ramp, with a legend that states quantity, unit, numerical range, sign convention, environmental conditions and assumptions.
 *
 * The values come from the {@link ThermalField} and exist before any colour (`ItemValue` of every item). Seven quantities: surface temperature, skin temperature (beneath clothing,
 * fur or feathers), emitted radiant flux εσT⁴, net radiative flux (with the sky/ground/surroundings split in the inspector label), sensible heat flux, latent heat flux and moisture
 * flux. Temperatures are centred on the air temperature, fluxes on 0 W/m² (positive = leaving the surface); `shared` ranges are the documented comparison ranges of
 * 📏️quantities (identical for every asset, scale and time), `auto` normalises to the values present and the legend says so, `span` (surface temperature only; every other quantity falls
 * back to `shared`) is a symmetric ±spanK range around the air temperature, so moderate sources stay readable in a mild room, and the legend states it. Open strokes that carry a region (ruling 2) are painted
 * as a halo of the region colour under the ink line. Air transport lives in 💨️transport-view.
 *
 * View independence: the field is solved once in world space; this function only maps region values to the shapes of the chosen viewpoint (elevation, front or plan). The quantity, unit, legend, centre and
 * range (also `auto`, which spans every region of the field, not the shapes of the view) are therefore identical in all three views, and a region has the same value wherever it is visible. A region without any shape in the
 * viewpoint is occluded by construction (a sole, a back, an underside) and is named in the interpretation of the legend rather than silently dropped.
 *
 * @see ../../🌡️thermal/🟦️.ts — `solveThermal`, `QUANTITY_SPECS`, `radiationSplit`
 * @see ../../🔗️contracts/🟦️.ts — {@link ThermalField}
 */
import type { Asset, Slug, Text } from "../../../🧬️schema/🟦️.ts";
import { QUANTITY_SPECS, areaWeighted, radiationSplit } from "../../🌡️thermal/🟦️.ts";
import type { DisplayRange, Fill, Legend, ThermalField, ThermalQuantity } from "../../🔗️contracts/🟦️.ts";
import { type Ctx } from "../🧱️canvas/🟦️.ts";
import { type Scale, type Theme, divergingColour, legendOf, scaleOf, toneColour, tx } from "../🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Style, outlineRuns, regionsOf } from "../🧩️shapes/🟦️.ts";
import { commandsOf } from "../📐️geometry/🟦️.ts";

const PATTERN_FRACTION = 0.35;
const HALO_FACTOR = 2.6;
const MIN_HALO_MM = 0.3;

const compass = (deg: number): string => `${Math.round(((deg % 360) + 360) % 360)}°`;
const n = (x: number, digits = 0): string => x.toFixed(digits);

function conditionsOf(field: ThermalField, timeS: number): Text {
  const e = field.environment;
  const sky = e.skyTempC == null ? "" : `, sky ${n(e.skyTempC, 1)} °C`;
  const ground = e.groundTempC == null ? "" : `, ground ${n(e.groundTempC, 1)} °C`;
  const skyDe = e.skyTempC == null ? "" : `, Himmel ${n(e.skyTempC, 1)} °C`;
  const groundDe = e.groundTempC == null ? "" : `, Boden ${n(e.groundTempC, 1)} °C`;
  return tx(
    `Air ${n(e.airTempC, 1)} °C, mean radiant ${n(e.meanRadiantTempC, 1)} °C${sky}${ground}, RH ${n(e.relativeHumidity * 100)} %, wind ${n(e.windMS, 1)} m/s towards bearing ${compass(e.windDirectionDeg)} (0° ahead, 90° left), sun ${n(e.solarWM2)} W/m² (diffuse ${n(e.diffuseFraction * 100)} %), ${n(e.pressurePa / 1000, 1)} kPa, t = ${n(timeS, 1)} s.`,
    `Luft ${n(e.airTempC, 1)} °C, mittlere Strahlungstemperatur ${n(e.meanRadiantTempC, 1)} °C${skyDe}${groundDe}, rel. Feuchte ${n(e.relativeHumidity * 100)} %, Wind ${n(e.windMS, 1)} m/s Richtung ${compass(e.windDirectionDeg)} (0° voraus, 90° links), Sonne ${n(e.solarWM2)} W/m² (diffus ${n(e.diffuseFraction * 100)} %), ${n(e.pressurePa / 1000, 1)} kPa, t = ${n(timeS, 1)} s.`,
  );
}

/** 📝️ The sentence of a legend that states how its range was chosen, in both languages. */
function rangeNote(display: DisplayRange, scale: Scale, unit: string): Text {
  const bounds = `${n(scale.min)} … ${n(scale.max)} ${unit}`;
  switch (display.mode) {
    case "auto":
      return tx("Comparative illustration. AUTO range: normalised to the values of every thermal region of the solved field (the same in elevation, front and plan), not comparable between figures; read absolute values from the inspector, not from the colour.", "Vergleichende Illustration. AUTO-Bereich: auf die Werte aller thermischen Regionen des gelösten Feldes normiert (in Seitenansicht, Vorderansicht und Grundriss gleich), nicht zwischen Abbildungen vergleichbar; absolute Werte stehen im Inspektor, nicht in der Farbe.");
    case "shared":
      return tx(`Absolute values. Shared comparison range ${bounds}, identical for every asset, scale and time.`, `Absolute Werte. Gemeinsamer Vergleichsbereich ${bounds}, gleich für jedes Asset, jeden Maßstab und jede Zeit.`);
    case "span":
      return tx(`Comparative illustration. Span ±${n(display.spanK)} K around the air temperature (${bounds}), identical for every cast of one scene; values beyond it saturate, so the colour is not an absolute temperature reading.`, `Vergleichende Illustration. Spanne ±${n(display.spanK)} K um die Lufttemperatur (${bounds}), gleich für alle Figuren einer Szene; Werte darüber hinaus sättigen, die Farbe ist daher keine absolute Temperaturablesung.`);
    default:
      return tx(`Absolute values. Fixed range ${bounds}.`, `Absolute Werte. Fester Bereich ${bounds}.`);
  }
}

/** 🎨️ The value-to-colour mapping of a thermal view, shared by every depiction of the same field (the 2D views and the 3D model): the quantity spec, the centre, the display range actually applied (`span` exists for surface temperature only, every other quantity falls back to `shared`), the scale, the value of a region and of a merged set of regions (area-weighted) and the diverging colour of a value (warm above the centre). */
export type ThermalPaint = { readonly spec: (typeof QUANTITY_SPECS)[ThermalQuantity]; readonly centre: number; readonly display: DisplayRange; readonly scale: Scale; readonly regionValue: (id: Slug) => number | undefined; readonly valueOf: (ids: readonly Slug[]) => number | undefined; readonly colourOf: (value: number) => string };

/** 🖍️ The {@link ThermalPaint} of a solved field: the single source of the thermal ramp, range and centre of the 2D thermal view and the 3D model. */
export function thermalPaint(asset: Asset, field: ThermalField, quantity: ThermalQuantity, range: DisplayRange, theme: Theme): ThermalPaint {
  const spec = QUANTITY_SPECS[quantity];
  const centre = spec.centre(field.environment);
  const regionValue = (id: Slug): number | undefined => {
    const f = field.regions[id];
    if (!f) return undefined;
    return spec.pick(f) ?? (quantity === "skin-temperature" ? f.surfaceTempC : undefined);
  };
  const display: DisplayRange = range.mode === "span" && quantity !== "surface-temperature" ? { mode: "shared" } : range;
  const fieldValues = asset.thermal.regions.flatMap((region) => regionValue(region.id) ?? []);
  const scale: Scale = scaleOf(fieldValues, centre, display, spec.quantity.endsWith("temperature") ? 1 : 10, spec.shared);
  return { spec, centre, display, scale, regionValue, valueOf: (ids) => areaWeighted(asset.thermal.regions, field, ids, quantity) ?? undefined, colourOf: (value) => divergingColour(value, scale, true, theme) };
}

/** 🟥️ Builds the thermal view: shape styles, open-stroke halos and the legend of the requested quantity. */
export function thermalView(ctx: Ctx, shapes: readonly Prepared[], field: ThermalField): AnalyticView {
  const { options, state, theme } = ctx;
  const asset = state.asset;
  const { spec, centre, display, scale, regionValue, valueOf, colourOf } = thermalPaint(asset, field, options.thermalQuantity, options.range, theme);
  const valueOfShape = (p: Prepared): number | undefined => valueOf(regionsOf(p.shape));
  const painted = shapes.filter((p) => regionsOf(p.shape).length > 0 && (p.closed ? p.shape.filled : true));
  const values = new Map<string, number>();
  for (const p of painted) {
    const v = valueOfShape(p);
    if (v !== undefined) values.set(p.shape.id, v);
  }
  const mode = display.mode;
  const note = rangeNote(display, scale, spec.unit);
  const regionTitle = (id: Slug): Text => asset.thermal.regions.find((r) => r.id === id)?.title ?? tx(id, id);
  const isTemperature = spec.unit === "°C";

  const labelOf = (p: Prepared): Text => {
    const ids = regionsOf(p.shape);
    if (ids.length > 1) return tx(`${ids.length} merged regions, area-weighted`, `${ids.length} zusammengefasste Regionen, flächengewichtet`);
    const title = regionTitle(ids[0]!);
    if (options.thermalQuantity === "net-radiative-flux") {
      const s = radiationSplit(field, ids[0]!);
      return tx(`${title.en}: sky ${n(s.sky)}, ground ${n(s.ground)}, surroundings ${n(s.surroundings)} W/m²`, `${title.de}: Himmel ${n(s.sky)}, Boden ${n(s.ground)}, Umgebung ${n(s.surroundings)} W/m²`);
    }
    if (options.thermalQuantity === "skin-temperature" && field.regions[ids[0]!]?.skinTempC === null) return tx(`${title.en} (no skin: surface temperature)`, `${title.de} (keine Haut: Oberflächentemperatur)`);
    return title;
  };
  const patternOf = (v: number): Fill["pattern"] | undefined => {
    const d = v - scale.centre;
    const span = d > 0 ? scale.max - scale.centre : scale.centre - scale.min;
    if (!options.patterns || !(span > 0) || Math.abs(d) / span <= PATTERN_FRACTION) return undefined;
    return d < 0 ? "cold" : options.thermalQuantity === "latent-heat-flux" || options.thermalQuantity === "moisture-flux" ? "vapour" : "hot";
  };

  const styleOf = (p: Prepared): Style => {
    const v = values.get(p.shape.id);
    const glaze = p.shape.role === "glazing";
    const outlineMm = p.weightMm * 0.5;
    if (v === undefined) return { fill: { color: toneColour(theme, p.shape.tone), ...(glaze ? { opacity: 0.4 } : {}) }, outlineMm };
    const pattern = patternOf(v);
    return { fill: { color: colourOf(v), ...(glaze ? { opacity: 0.6 } : {}), ...(pattern ? { pattern } : {}) }, value: { quantity: options.thermalQuantity, value: v, unit: spec.unit, label: labelOf(p) }, outlineMm };
  };

  const overlay: AnalyticView["overlay"] = (canvas) => {
    for (const { p, run, n: index } of outlineRuns(shapes, false)) {
      const v = values.get(p.shape.id);
      if (p.closed || v === undefined) continue;
      canvas.path("field", `halo:${p.shape.id}:${index}`, commandsOf(run.points, run.closed), {
        stroke: { color: colourOf(v), widthMm: Math.max(MIN_HALO_MM, p.weightMm * HALO_FACTOR), cap: "round", join: "round" },
        value: { quantity: options.thermalQuantity, value: v, unit: spec.unit, label: labelOf(p) },
        bone: p.shape.bone,
        ...(p.shape.region ? { region: p.shape.region } : {}),
      });
    }
  };

  const warmHatch = options.thermalQuantity === "latent-heat-flux" || options.thermalQuantity === "moisture-flux" ? tx("vapour hatch", "Dampfschraffur") : tx("hot hatch", "Heißschraffur");
  const hatchNote = options.patterns
    ? tx(` Hatching is a second, colour-independent reading: unhatched = within ±${Math.round(PATTERN_FRACTION * 100)} % of the range on either side of the centre; ${warmHatch.en} = beyond ${Math.round(PATTERN_FRACTION * 100)} % above the centre; cold hatch = beyond ${Math.round(PATTERN_FRACTION * 100)} % below it.`, ` Die Schraffur ist eine zweite, farbunabhängige Lesart: unschraffiert = innerhalb von ±${Math.round(PATTERN_FRACTION * 100)} % des Bereichs beiderseits der Mitte; ${warmHatch.de} = über ${Math.round(PATTERN_FRACTION * 100)} % oberhalb der Mitte; Kälteschraffur = über ${Math.round(PATTERN_FRACTION * 100)} % unterhalb.`)
    : tx(" Hatching is off: the colour is the only reading.", " Schraffur ist aus: die Farbe ist die einzige Lesart.");
  const clamped = Math.abs(scale.centre - centre) > 1e-9;
  const clampNote = clamped ? tx(` The ${spec.centreMeaning.en} ${n(centre, 1)} ${spec.unit} lies outside the fixed range, so neutral grey sits at its nearest end, ${n(scale.centre, 1)} ${spec.unit}.`, ` Die ${spec.centreMeaning.de} ${n(centre, 1)} ${spec.unit} liegt außerhalb des festen Bereichs, daher liegt das neutrale Grau an dessen nächstem Ende, ${n(scale.centre, 1)} ${spec.unit}.`) : tx("", "");
  const convention = isTemperature
    ? tx(
        `${spec.title.en} in ${spec.unit}, diverging scale centred on the ${spec.centreMeaning.en} ${n(scale.centre, 1)} ${spec.unit} (neutral grey): warm = above the centre, cool = below. ${note.en}${clampNote.en}${hatchNote.en}`,
        `${spec.title.de} in ${spec.unit}, divergierende Skala mit ${spec.centreMeaning.de} ${n(scale.centre, 1)} ${spec.unit} als Mitte (neutrales Grau): warm = über der Mitte, kühl = darunter. ${note.de}${clampNote.de}${hatchNote.de}`,
      )
    : tx(
        `${spec.title.en} in ${spec.unit}, diverging scale centred on ${n(scale.centre)} ${spec.unit} (${spec.centreMeaning.en}): warm = positive = leaving the surface, cool = negative = the surface gains. ${note.en}${clampNote.en}${hatchNote.en}`,
        `${spec.title.de} in ${spec.unit}, divergierende Skala mit ${n(scale.centre)} ${spec.unit} als Mitte (${spec.centreMeaning.de}): warm = positiv = verlässt die Fläche, kühl = negativ = die Fläche nimmt auf. ${note.de}${clampNote.de}${hatchNote.de}`,
      );
  const conditions = conditionsOf(field, state.timeS);
  const halo = tx("Open strokes of a region are drawn as a halo of the region colour under the ink line: the ink stays the drawing, the colour is the analysis.", "Offene Linien einer Region sind als Halo in der Regionsfarbe unter der Tuschelinie gezeichnet: die Tusche bleibt die Zeichnung, die Farbe die Analyse.");
  const assumptions = [field.notes[0]!, ...field.assumptions.slice(-2), halo];
  const drawn = new Set([...values.keys()].flatMap((id) => regionsOf(shapes.find((p) => p.shape.id === id)!.shape)));
  const unseen = asset.thermal.regions.filter((region) => regionValue(region.id) !== undefined && !drawn.has(region.id));
  const unseenNote = unseen.length === 0 ? tx("", "") : tx(` Not visible from this viewpoint: ${unseen.map((u) => u.title.en).join(", ")}.`, ` Aus dieser Ansicht nicht sichtbar: ${unseen.map((u) => u.title.de).join(", ")}.`);
  const interpretation = tx(`${spec.meaning.en} Values of merged regions are weighted by area × exposed fraction; shapes without a region are grey.${unseenNote.en}`, `${spec.meaning.de} Die Werte zusammengefasster Regionen sind nach Fläche × exponiertem Anteil gewichtet; Formen ohne Region sind grau.${unseenNote.de}`);
  const legend: Legend = legendOf({ title: spec.title, quantity: options.thermalQuantity, unit: spec.unit, scale, colourAt: colourOf, convention, interpretation, range: mode, conditions, assumptions });
  return { styleOf, overlay, legends: [legend] };
}
