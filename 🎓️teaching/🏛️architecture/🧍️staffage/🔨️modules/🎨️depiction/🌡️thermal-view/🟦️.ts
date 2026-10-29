/** 🌡️ The thermal view: each shape filled by its region's solved quantity on a diverging ramp, plume streamlines as tapered strips and tracer parcels.
 *
 * Surface temperature is centred on the air temperature, fluxes on 0 W/m² (positive = leaving the surface); warm above, cool below. Airflow
 * parcels are dots coloured by excess temperature, water-vapour parcels are open blue-green circles whose radius grows with the vapour mass fraction.
 *
 * @see ../../🌡️thermal/🟦️.ts — `solveThermal`
 * @see ../../🔗️contracts/🟦️.ts — {@link ThermalField}
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";
import { plumeLegend, specificHumidity, thermalColour, vapourPressurePa } from "../../🌡️thermal/🟦️.ts";
import type { Fill, Legend, Point, RegionField, ThermalField, ThermalQuantity } from "../../🔗️contracts/🟦️.ts";
import { type Canvas, type Ctx } from "../🧱️canvas/🟦️.ts";
import { overlayColours, type Scale, divergingColour, legendOf, scaleOf, toneColour, tx } from "../🖍️palette/🟦️.ts";
import { type AnalyticView, type Prepared, type Style } from "../🧩️shapes/🟦️.ts";
import { type Vec, arrowOutline, circleCommands, commandsOf, strip } from "../📐️geometry/🟦️.ts";

const QUANTITIES: Readonly<Record<ThermalQuantity, { readonly title: Text; readonly unit: string; readonly pick: (r: RegionField) => number }>> = {
  "surface-temperature": { title: tx("Surface temperature", "Oberflächentemperatur"), unit: "°C", pick: (r) => r.surfaceTempC },
  "radiant-flux": { title: tx("Radiant heat flux", "Strahlungswärmestrom"), unit: "W/m²", pick: (r) => r.radiantFluxWM2 },
  "convective-flux": { title: tx("Convective heat flux", "Konvektiver Wärmestrom"), unit: "W/m²", pick: (r) => r.convectiveFluxWM2 },
  "latent-flux": { title: tx("Latent heat flux", "Latenter Wärmestrom"), unit: "W/m²", pick: (r) => r.latentFluxWM2 },
  "net-flux": { title: tx("Net heat flux", "Netto-Wärmestrom"), unit: "W/m²", pick: (r) => r.netFluxWM2 },
};

const PATTERN_FRACTION = 0.35;
const MAX_STRIP_SEGMENTS = 16;
const NEGLIGIBLE_SPEED_MS = 0.05;
const NEGLIGIBLE_EXCESS_K = 0.05;

const FIELD_MARGIN_HEIGHTS = 1.5;
const FIELD_MARGIN_MAX_M = 3;

function reach(streamline: ThermalField["streamlines"][number], inside: (p: Point) => boolean): number {
  let last = 0;
  const firstOutside = streamline.points.findIndex((p) => !inside(p));
  streamline.points.forEach((_, i) => {
    if (firstOutside >= 0 && i >= firstOutside) return;
    if ((streamline.speedMS[i] ?? 0) > NEGLIGIBLE_SPEED_MS || (streamline.carries === "airflow" && Math.abs(streamline.excessK[i] ?? 0) > NEGLIGIBLE_EXCESS_K)) last = i;
  });
  return last + 1;
}

/** 🌡️ Builds the thermal view: shape styles, plume overlay and legends (surface quantity, airflow excess temperature, water vapour). */
export function thermalView(ctx: Ctx, shapes: readonly Prepared[], field: ThermalField): AnalyticView {
  const { options, state, colours } = ctx;
  const meta = QUANTITIES[options.thermalQuantity];
  const centre = options.thermalQuantity === "surface-temperature" ? field.environment.airTempC : 0;
  const b = state.bounds;
  const margin = Math.min(FIELD_MARGIN_MAX_M, FIELD_MARGIN_HEIGHTS * Math.max(1e-6, b.maxY - b.minY));
  const inside = (p: Point): boolean => p.x >= b.minX - margin && p.x <= b.maxX + margin && p.y >= b.minY - margin && p.y <= b.maxY + margin;
  const used = [...new Set(shapes.flatMap((p) => (p.shape.region && field.regions[p.shape.region] ? [p.shape.region] : [])))];
  const scale: Scale = scaleOf(used.map((r) => meta.pick(field.regions[r]!)), centre, options.range, meta.unit === "°C" ? 1 : 10);
  const regionTitle = (id: Slug): Text => state.asset.thermal.regions.find((r) => r.id === id)?.title ?? tx(id, id);
  const isTemperature = options.thermalQuantity === "surface-temperature";
  const surface: Legend = legendOf({
    title: meta.title,
    quantity: options.thermalQuantity,
    unit: meta.unit,
    scale,
    colourAt: (v) => divergingColour(v, scale, true, ctx.theme),
    convention: isTemperature
      ? tx(`Diverging scale centred on the air temperature ${centre} °C (neutral grey): warm = hotter than the air, cool = colder.`, `Divergierende Skala mit der Lufttemperatur ${centre} °C als Mitte (neutrales Grau): warm = wärmer als die Luft, kühl = kälter.`)
      : tx("Diverging scale centred on 0 W/m² (neutral grey): warm = heat leaves the surface (positive), cool = the surface gains heat (negative).", "Divergierende Skala mit 0 W/m² als Mitte (neutrales Grau): warm = Wärme verlässt die Oberfläche (positiv), kühl = Oberfläche nimmt Wärme auf (negativ)."),
    interpretation: tx(`Steady-state estimate per thermal region at air ${field.environment.airTempC} °C, radiant ${field.environment.meanRadiantTempC} °C, wind ${field.environment.windMS} m/s; shapes without a region are grey.`, `Stationäre Schätzung je thermischer Region bei Luft ${field.environment.airTempC} °C, Strahlung ${field.environment.meanRadiantTempC} °C, Wind ${field.environment.windMS} m/s; Formen ohne Region sind grau.`),
  });

  const styleOf = (p: Prepared): Style => {
    const region = p.shape.region ? field.regions[p.shape.region] : undefined;
    const glaze = p.shape.role === "glazing";
    const outlineMm = p.weightMm * 0.5;
    if (!region) return { fill: { color: toneColour(ctx.theme, p.shape.tone), ...(glaze ? { opacity: 0.4 } : {}) }, outlineMm };
    const v = meta.pick(region);
    const d = v - centre;
    const span = d > 0 ? scale.max - centre : centre - scale.min;
    const strong = span > 0 && Math.abs(d) / span > PATTERN_FRACTION;
    const pattern: Fill["pattern"] | undefined = options.patterns && strong ? (d < 0 ? "cold" : options.thermalQuantity === "latent-flux" ? "vapour" : "hot") : undefined;
    return {
      fill: { color: divergingColour(v, scale, true, ctx.theme), ...(glaze ? { opacity: 0.6 } : {}), ...(pattern ? { pattern } : {}) },
      value: { quantity: options.thermalQuantity, value: v, unit: meta.unit, label: regionTitle(p.shape.region!) },
      outlineMm,
    };
  };

  const humidity = specificHumidity(vapourPressurePa(field.environment), field.environment.pressurePa);
  const vapourExcessG = (kgPerKg: number): number => (kgPerKg - humidity) * 1000;
  const airLegend = plumeLegend("airflow", field, options.range);
  const vapourLegend = plumeLegend("vapour", field, options.range);
  const hasVapour = field.streamlines.some((s) => s.carries === "vapour" && s.active > 0.01) || field.parcels.some((q) => q.carries === "vapour");
  const hasAir = field.streamlines.some((s) => s.carries === "airflow" && s.active > 0.01) || field.parcels.some((q) => q.carries === "airflow");
  const airColour = (excessK: number): string => (excessK >= 0 ? thermalColour(airLegend, excessK) : divergingColour(excessK, { min: -airLegend.max, centre: 0, max: airLegend.max }, true, ctx.theme));
  const vapourColour = (kgPerKg: number): string => thermalColour(vapourLegend, Math.max(0, vapourExcessG(kgPerKg)));

  const emitterTitle = (id: Slug): Text => state.asset.thermal.emitters.find((e) => e.id === id)?.title ?? tx(id, id);
  const emitterDiameter = (id: Slug): number => state.asset.thermal.emitters.find((e) => e.id === id)?.diameterM ?? 0.05;

  const overlay = (canvas: Canvas): void => {
    for (const s of field.streamlines) {
      const n = reach(s, inside);
      if (s.active <= 0.01 || n < 2) continue;
      const stride = Math.max(1, Math.ceil((n - 1) / MAX_STRIP_SEGMENTS));
      const idx = [...Array.from({ length: Math.floor((n - 1) / stride) + 1 }, (_, i) => i * stride)];
      if (idx[idx.length - 1] !== n - 1) idx.push(n - 1);
      const pts: Vec[] = idx.map((i) => ctx.paper(s.points[i]!));
      const w0 = Math.min(1.6, Math.max(0.5, emitterDiameter(s.emitter) * ctx.k)) * (0.4 + 0.6 * Math.min(1, s.active));
      const [left, right] = strip(pts, idx.map((_, i) => w0 * (1 - 0.9 * (i / (idx.length - 1)))));
      for (let i = 0; i + 1 < idx.length; i++) {
        const excess = s.excessK[idx[i]!] ?? 0;
        const color = s.carries === "airflow" ? airColour(excess) : overlayColours(ctx.theme).vapour;
        canvas.path("field", `stream:${s.emitter}:${i}`, commandsOf([left[i]!, left[i + 1]!, right[i + 1]!, right[i]!], true), {
          fill: { color, opacity: (s.carries === "airflow" ? 0.35 : 0.22) + 0.5 * Math.min(1, s.active) },
          value: s.carries === "airflow" ? { quantity: "excess-temperature", value: excess, unit: "K", label: emitterTitle(s.emitter) } : { quantity: "speed", value: s.speedMS[idx[i]!] ?? 0, unit: "m/s", label: emitterTitle(s.emitter) },
        });
      }
    }
    field.parcels.forEach((q, i) => {
      if (!inside(q.at)) return;
      const at = ctx.paper(q.at);
      const speed = Math.hypot(q.velocity.x, q.velocity.y);
      if (q.carries === "vapour") {
        if (speed <= NEGLIGIBLE_SPEED_MS) return;
        canvas.path("field", `parcel:${q.emitter}:${i}`, circleCommands(at, 0.3 + 0.5 * Math.min(1, Math.max(0, vapourExcessG(q.vapourKgPerKg)) / vapourLegend.max)), {
          stroke: { color: vapourColour(q.vapourKgPerKg), widthMm: 0.22, opacity: 0.95 },
          value: { quantity: "vapour-excess", value: vapourExcessG(q.vapourKgPerKg), unit: "g/kg", label: emitterTitle(q.emitter) },
        });
        return;
      }
      if (speed <= NEGLIGIBLE_SPEED_MS && Math.abs(q.excessK) <= NEGLIGIBLE_EXCESS_K) return;
      const color = airColour(q.excessK);
      const dir = { x: q.velocity.x / (speed || 1), y: -q.velocity.y / (speed || 1) };
      const tail = { x: at.x - dir.x * Math.min(2, 0.5 + speed * 0.6), y: at.y - dir.y * Math.min(2, 0.5 + speed * 0.6) };
      canvas.path("field", `parcel:${q.emitter}:${i}`, commandsOf(arrowOutline(tail, { x: at.x + dir.x * 0.3, y: at.y + dir.y * 0.3 }, 0.12, 0.7, 0.6), true), {
        fill: { color, opacity: 0.9 },
        value: { quantity: "excess-temperature", value: q.excessK, unit: "K", label: emitterTitle(q.emitter) },
      });
    });
  };

  const legends: Legend[] = [surface, ...(hasAir ? [airLegend] : []), ...(hasVapour ? [vapourLegend] : [])];
  return { styleOf, overlay, legends };
}
