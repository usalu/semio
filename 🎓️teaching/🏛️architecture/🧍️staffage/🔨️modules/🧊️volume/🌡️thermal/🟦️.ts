/** 🌡️ The thermal look of the model: one material per thermal-region set, painted from the solved field with the very ramp, range, centre and environment rules of the 2D thermal view (`thermalPaint`: surface temperature centred on the air temperature, `shared` −20…60 °C or the `spanK` range of the scene, warm above the centre, regions merged by area × exposure),
 * and the emitters as translucent cone glyphs coloured by their temperature excess on the transport ramp (±20 K).
 *
 * A solid without a thermal region, or whose regions carry no value for the quantity, is grey like in the 2D view (glazing 40 % opaque); a valued glazing is 60 % opaque.
 *
 * @see ../../🎨️depiction/🌡️thermal-view/🟦️.ts — `thermalPaint`
 * @see ../../💨️transport/🟦️.ts — `solveTransport`
 */
import type { Asset, Slug, Text } from "../../../🧬️schema/🟦️.ts";
import type { DisplayRange, EmitterState, ThermalField, ThermalQuantity } from "../../🔗️contracts/🟦️.ts";
import { type Theme, divergingColour, toneColour, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { thermalPaint } from "../../🎨️depiction/🌡️thermal-view/🟦️.ts";
import { SHARED_EXCESS_K } from "../../🎨️depiction/💨️transport-view/🟦️.ts";
import { type LookSpec, type Paint, type Surface } from "../🎨️looks/🟦️.ts";

/** ⚙️ What the thermal look reads besides the field. */
export type ThermalSettings = { readonly quantity: ThermalQuantity; readonly range: DisplayRange; readonly theme: Theme };

const glazing = (surface: Surface): boolean => surface.role === "glazing";

/** 🎨️ The thermal look of a set of surfaces; the paint of a key at an instant is read from the solved field. */
export function thermalLook(asset: Asset, settings: ThermalSettings, surfaces: readonly Surface[]): LookSpec<ThermalField> {
  const keyOf = (s: Surface): string => `${s.regions.length === 0 ? "none" : `region-${s.regions.join("-")}`}-${Math.round(s.tone * 1000)}${glazing(s) ? "-glazing" : ""}`;
  const representative = new Map<string, Surface>();
  for (const s of surfaces) if (!representative.has(keyOf(s))) representative.set(keyOf(s), s);
  const regionTitle = (id: Slug): Text => asset.thermal.regions.find((r) => r.id === id)?.title ?? tx(id, id);
  return {
    view: "thermal",
    unlit: false,
    keyOf,
    titleOf: (_key, s) => (s.regions.length === 0 ? tx("No thermal region", "Keine thermische Region") : tx(s.regions.map((r) => regionTitle(r).en).join(" + "), s.regions.map((r) => regionTitle(r).de).join(" + "))),
    paint: (field) => {
      const paint = thermalPaint(asset, field, settings.quantity, settings.range, settings.theme);
      const out = new Map<string, Paint>();
      for (const [key, s] of representative) {
        const value = s.regions.length === 0 ? undefined : paint.valueOf(s.regions);
        const grey = { hex: toneColour(settings.theme, s.tone), alpha: glazing(s) ? 0.4 : 1 };
        out.set(key, value === undefined ? grey : { hex: paint.colourOf(value), alpha: glazing(s) ? 0.6 : 1 });
      }
      return out;
    },
  };
}

/** 🔥️ The paint of an emitter glyph: the temperature excess over the ambient air on the diverging ramp of the transport view (warm above), 55 % opaque. */
export function emitterPaint(state: EmitterState, airTempC: number, theme: Theme): Paint {
  const excess = state.temperatureC - airTempC;
  return { hex: divergingColour(excess, { min: -SHARED_EXCESS_K, centre: 0, max: SHARED_EXCESS_K }, true, theme), alpha: 0.55 };
}
