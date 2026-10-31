/** 🌡️ The thermal view: every shape is painted by the solved physical quantity of its thermal region (or the area-weighted value of the regions of a merged small-scale form) on a
 * diverging ramp, with a legend that states quantity, unit, numerical range, sign convention, environmental conditions and assumptions.
 *
 * The values come from the {@link ThermalField} and exist before any colour (`ItemValue` of every item). Seven quantities: surface temperature, skin temperature (beneath clothing,
 * fur or feathers), emitted radiant flux εσT⁴, net radiative flux (with the sky/ground/surroundings split in the inspector label), sensible heat flux, latent heat flux and moisture
 * flux. Temperatures are centred on the air temperature, fluxes on 0 W/m² (positive = leaving the surface); `shared` ranges are the documented comparison ranges of
 * 📏️quantities (identical for every asset, scale and time), `auto` normalises to the values present and the legend says so. Open strokes that carry a region (ruling 2) are painted
 * as a halo of the region colour under the ink line. Air transport lives in 💨️transport-view.
 *
 * @see ../../🌡️thermal/🟦️.ts — `solveThermal`, `QUANTITY_SPECS`, `radiationSplit`
 * @see ../../🔗️contracts/🟦️.ts — {@link ThermalField}
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";
import { QUANTITY_SPECS, radiationSplit } from "../../🌡️thermal/🟦️.ts";
import type { Fill, Legend, ThermalField } from "../../🔗️contracts/🟦️.ts";
import { type Ctx } from "../🧱️canvas/🟦️.ts";
import { type Scale, divergingColour, legendOf, scaleOf, toneColour, tx } from "../🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Style, areaWeighted, outlineRuns, regionsOf } from "../🧩️shapes/🟦️.ts";
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
    `Air ${n(e.airTempC, 1)} °C, mean radiant ${n(e.meanRadiantTempC, 1)} °C${sky}${ground}, RH ${n(e.relativeHumidity * 100)} %, wind ${n(e.windMS, 1)} m/s towards ${compass(e.windDirectionDeg)}, sun ${n(e.solarWM2)} W/m² (diffuse ${n(e.diffuseFraction * 100)} %), ${n(e.pressurePa / 1000, 1)} kPa, t = ${n(timeS, 1)} s.`,
    `Luft ${n(e.airTempC, 1)} °C, mittlere Strahlungstemperatur ${n(e.meanRadiantTempC, 1)} °C${skyDe}${groundDe}, rel. Feuchte ${n(e.relativeHumidity * 100)} %, Wind ${n(e.windMS, 1)} m/s Richtung ${compass(e.windDirectionDeg)}, Sonne ${n(e.solarWM2)} W/m² (diffus ${n(e.diffuseFraction * 100)} %), ${n(e.pressurePa / 1000, 1)} kPa, t = ${n(timeS, 1)} s.`,
  );
}

/** 🌡️ Builds the thermal view: shape styles, open-stroke halos and the legend of the requested quantity. */
export function thermalView(ctx: Ctx, shapes: readonly Prepared[], field: ThermalField): AnalyticView {
  const { options, state, theme } = ctx;
  const asset = state.asset;
  const spec = QUANTITY_SPECS[options.thermalQuantity];
  const centre = spec.centre(field.environment);
  const regionValue = (id: Slug): number | undefined => {
    const f = field.regions[id];
    if (!f) return undefined;
    return spec.pick(f) ?? (options.thermalQuantity === "skin-temperature" ? f.surfaceTempC : undefined);
  };
  const valueOfShape = (p: Prepared): number | undefined => areaWeighted(asset, p.shape, regionValue);
  const painted = shapes.filter((p) => regionsOf(p.shape).length > 0 && (p.closed ? p.shape.filled : true));
  const values = new Map<string, number>();
  for (const p of painted) {
    const v = valueOfShape(p);
    if (v !== undefined) values.set(p.shape.id, v);
  }
  const scale: Scale = scaleOf([...values.values()], centre, options.range, spec.quantity.endsWith("temperature") ? 1 : 10, spec.shared);
  const mode = options.range.mode;
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
  const colourOf = (v: number): string => divergingColour(v, scale, true, theme);
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

  const convention = isTemperature
    ? tx(
        `${spec.title.en} in ${spec.unit}, diverging scale centred on the ${spec.centreMeaning.en} ${n(centre, 1)} ${spec.unit} (neutral grey): warm = above the centre, cool = below. ${mode === "auto" ? "AUTO range: normalised to the values present in this figure, not comparable between figures." : mode === "shared" ? `Shared comparison range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, identical for every asset, scale and time.` : `Fixed range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}`,
        `${spec.title.de} in ${spec.unit}, divergierende Skala mit ${spec.centreMeaning.de} ${n(centre, 1)} ${spec.unit} als Mitte (neutrales Grau): warm = über der Mitte, kühl = darunter. ${mode === "auto" ? "AUTO-Bereich: auf die in dieser Abbildung vorhandenen Werte normiert, nicht zwischen Abbildungen vergleichbar." : mode === "shared" ? `Gemeinsamer Vergleichsbereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, gleich für jedes Asset, jeden Maßstab und jede Zeit.` : `Fester Bereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}`,
      )
    : tx(
        `${spec.title.en} in ${spec.unit}, diverging scale centred on ${n(centre)} ${spec.unit} (${spec.centreMeaning.en}): warm = positive = leaving the surface, cool = negative = the surface gains. ${mode === "auto" ? "AUTO range: normalised to the values present in this figure, not comparable between figures." : mode === "shared" ? `Shared comparison range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, identical for every asset, scale and time.` : `Fixed range ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}`,
        `${spec.title.de} in ${spec.unit}, divergierende Skala mit ${n(centre)} ${spec.unit} als Mitte (${spec.centreMeaning.de}): warm = positiv = verlässt die Fläche, kühl = negativ = die Fläche nimmt auf. ${mode === "auto" ? "AUTO-Bereich: auf die in dieser Abbildung vorhandenen Werte normiert, nicht zwischen Abbildungen vergleichbar." : mode === "shared" ? `Gemeinsamer Vergleichsbereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}, gleich für jedes Asset, jeden Maßstab und jede Zeit.` : `Fester Bereich ${n(scale.min)} … ${n(scale.max)} ${spec.unit}.`}`,
      );
  const conditions = conditionsOf(field, state.timeS);
  const halo = tx("Open strokes of a region are drawn as a halo of the region colour under the ink line: the ink stays the drawing, the colour is the analysis.", "Offene Linien einer Region sind als Halo in der Regionsfarbe unter der Tuschelinie gezeichnet: die Tusche bleibt die Zeichnung, die Farbe die Analyse.");
  const assumptions = [field.notes[0]!, ...field.assumptions.slice(-2), halo];
  const interpretation = tx(`${spec.meaning.en} Values are area-weighted for merged regions; shapes without a region are grey.`, `${spec.meaning.de} Bei zusammengefassten Regionen sind die Werte flächengewichtet; Formen ohne Region sind grau.`);
  const legend: Legend = legendOf({ title: spec.title, quantity: options.thermalQuantity, unit: spec.unit, scale, colourAt: colourOf, convention, interpretation, range: mode, conditions, assumptions });
  return { styleOf, overlay, legends: [legend] };
}
