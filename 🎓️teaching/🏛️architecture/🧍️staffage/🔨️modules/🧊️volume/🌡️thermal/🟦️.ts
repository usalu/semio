/** 🌡️ The thermal look of the model: one material per thermal-region set, painted from the solved field with the very ramp, range, centre and environment rules of the 2D thermal view (`thermalPaint`: surface temperature centred on the air temperature, `shared` −20…60 °C or the `spanK` range of the scene, warm above the centre, regions merged by area × exposure).
 * The air around the body is not part of this look: the flow parts of ../🌊️flow carry it.
 *
 * A solid without a thermal region, or whose regions carry no value for the quantity, is grey like in the 2D view (glazing 40 % opaque); a valued glazing is 60 % opaque.
 * A region of several layers is modelled as nested shells (one solid set per layer, innermost first): its outermost shell shows the surface value, every inner shell the skin value beneath the layers (the skin temperature) for the temperature quantities and the region value for the fluxes.
 *
 * @see ../../🎨️depiction/🌡️thermal-view/🟦️.ts — `thermalPaint`
 * @see ../🌊️flow/🟦️.ts — the air flow of the thermal scenes
 */
import type { Asset, Slug, Text } from "../../../🧬️schema/🟦️.ts";
import type { DisplayRange, ThermalField, ThermalQuantity } from "../../🔗️contracts/🟦️.ts";
import { type Theme, toneColour, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { thermalPaint } from "../../🎨️depiction/🌡️thermal-view/🟦️.ts";
import { type LookSpec, type Paint, type Surface } from "../🎨️looks/🟦️.ts";

/** ⚙️ What the thermal look reads besides the field. */
export type ThermalSettings = { readonly quantity: ThermalQuantity; readonly range: DisplayRange; readonly theme: Theme };

const glazing = (surface: Surface): boolean => surface.role === "glazing";

/** 🎨️ The thermal look of a set of surfaces; the paint of a key at an instant is read from the solved field. */
export function thermalLook(asset: Asset, settings: ThermalSettings, surfaces: readonly Surface[]): LookSpec<ThermalField> {
  const keyOf = (s: Surface): string => `${s.regions.length === 0 ? "none" : `region-${s.regions.join("-")}${s.layer === undefined ? "" : `-${s.layer}`}`}-${Math.round(s.tone * 1000)}${glazing(s) ? "-glazing" : ""}`;
  const layersOf = (region: Slug): readonly Slug[] => asset.thermal.regions.find((r) => r.id === region)?.layers.map((l) => l.id) ?? [];
  const inner = (s: Surface): boolean => {
    const layers = s.regions.length === 1 && s.layer !== undefined ? layersOf(s.regions[0]!) : [];
    return layers.length > 1 && layers.indexOf(s.layer!) < layers.length - 1;
  };
  const representative = new Map<string, Surface>();
  for (const s of surfaces) if (!representative.has(keyOf(s))) representative.set(keyOf(s), s);
  const regionTitle = (id: Slug): Text => asset.thermal.regions.find((r) => r.id === id)?.title ?? tx(id, id);
  return {
    view: "thermal",
    unlit: false,
    keyOf,
    titleOf: (_key, s) => (s.regions.length === 0 ? tx("No thermal region", "Keine thermische Region") : tx(`${s.regions.map((r) => regionTitle(r).en).join(" + ")}${s.layer === undefined ? "" : ` (${s.layer})`}`, `${s.regions.map((r) => regionTitle(r).de).join(" + ")}${s.layer === undefined ? "" : ` (${s.layer})`}`)),
    paint: (field) => {
      const paint = thermalPaint(asset, field, settings.quantity, settings.range, settings.theme);
      const out = new Map<string, Paint>();
      for (const [key, s] of representative) {
        const skin = inner(s) && (settings.quantity === "surface-temperature" || settings.quantity === "skin-temperature") ? field.regions[s.regions[0]!]?.skinTempC : undefined;
        const value = s.regions.length === 0 ? undefined : (skin ?? paint.valueOf(s.regions));
        const grey = { hex: toneColour(settings.theme, s.tone), alpha: glazing(s) ? 0.4 : 1 };
        out.set(key, value === undefined ? grey : { hex: paint.colourOf(value), alpha: glazing(s) ? 0.6 : 1 });
      }
      return out;
    },
  };
}
