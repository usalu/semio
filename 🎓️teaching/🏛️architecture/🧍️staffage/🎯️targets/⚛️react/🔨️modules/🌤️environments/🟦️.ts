/** 🌤️ The named environment presets of the catalog, the air an asset's thermal view is solved in, and the quantities of that air a reader can set: their units, ranges, steps, address parameters and element attributes.
 *
 * The thermal view of an asset solves under the preset the reader chose, else under the asset's own `thermal.environment`; the reader's own values sit on top of either. A preset is applied whole: every field of the environment, the wind direction, the pressure and the sky and ground temperatures included.
 * A {@link Quantity} is shown in the unit a reader thinks in (per cent, hectopascal) and stored in the unit of the schema (fraction, pascal); the address and the attributes of `<staffage-figure>` carry the shown number.
 *
 * @see ../../../../🗂️catalog/🌤️environments/🔣️.json — the presets
 * @see ../../../../🧬️schema/🟦️.ts — `Environment`
 */
import { resolveThermalEnvironment } from "@teaching/staffage";
import type { Asset, Environment, EnvironmentDocument, EnvironmentPreset, Slug } from "@teaching/staffage";
import presetDocument from "../../../../🗂️catalog/🌤️environments/🔣️.json";

/** 🏞️ The presets in catalog order. */
export const ENVIRONMENT_PRESETS: readonly EnvironmentPreset[] = (presetDocument as unknown as EnvironmentDocument).presets;

/** 🗺️ The presets by id. */
export const PRESETS_BY_ID: ReadonlyMap<Slug, EnvironmentPreset> = new Map(ENVIRONMENT_PRESETS.map((preset) => [preset.id, preset]));

/** 🪪️ Whether `value` names a preset. */
export const isPreset = (value: string | null | undefined): value is Slug => PRESETS_BY_ID.has(value ?? "");

/** 🎯️ The case an asset's thermal view starts in: its preset id when it names one, and the environment itself. */
export type ReferenceCase = { readonly id: Slug | null; readonly environment: Environment };

/** 🎯️ The reference case of an asset as the exporters resolve it. */
export function referenceCase(asset: Asset): ReferenceCase {
  const choice = asset.thermal.referenceEnvironment;
  return { id: typeof choice === "string" ? choice : null, environment: resolveThermalEnvironment(asset, PRESETS_BY_ID) ?? asset.thermal.environment };
}

/** 🖐️ The environment values a reader set by hand, on top of a preset or of the asset's own environment. */
export type EnvironmentOverride = Partial<Environment>;

/** 🌡️ The environment a figure solves under: the chosen preset (all of it) or the asset's own environment, with the reader's values on top. `changed` says whether any value was set by hand, `own` whether it is the asset's own environment untouched. */
export function environmentOf(asset: Asset, preset: Slug | null, changes: EnvironmentOverride): { readonly environment: Environment; readonly base: Environment; readonly presetId: Slug | null; readonly changed: boolean; readonly own: boolean } {
  const chosen = preset === null ? undefined : PRESETS_BY_ID.get(preset);
  const base = chosen?.environment ?? asset.thermal.environment;
  const changed = Object.keys(changes).length > 0;
  return { environment: changed ? { ...base, ...changes } : base, base, presetId: chosen?.id ?? null, changed, own: chosen === undefined && !changed };
}

/** 🎚️ One number a reader sets: the field it fills, its address parameter and element attribute, the range, step and decimals of the shown number, its unit symbol, and the conversion between the shown and the stored number (`wrap` folds a bearing into 0 ≤ value < wrap). */
export type Quantity<K extends string = string> = {
  readonly key: K;
  readonly param: string;
  readonly attribute: string;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly digits: number;
  readonly unit: string;
  readonly wrap?: number;
  readonly shown: (stored: number) => number;
  readonly stored: (shown: number) => number;
};

const same = (value: number): number => value;
const quantity = <K extends string>(key: K, param: string, attribute: string, min: number, max: number, step: number, digits: number, unit: string, convert: Partial<Pick<Quantity, "shown" | "stored" | "wrap">> = {}): Quantity<K> => ({ key, param, attribute, min, max, step, digits, unit, shown: same, stored: same, ...convert });

/** 💯️ A fraction shown in per cent. */
export const PER_CENT = { shown: (stored: number): number => stored * 100, stored: (shown: number): number => shown / 100 } as const;

/** 🌤️ Every field of the environment as a quantity, in the order of the address: temperatures in °C, humidity and the diffuse share of the sun in per cent, wind in m/s and as the bearing the air moves towards (0° ahead, 90° to the asset's left), sun in W/m², pressure in hPa. */
export const ENVIRONMENT_QUANTITIES = [
  quantity("airTempC", "air", "air-temperature", -60, 70, 0.5, 1, "°C"),
  quantity("meanRadiantTempC", "mrt", "radiant-temperature", -60, 90, 0.5, 1, "°C"),
  quantity("skyTempC", "sky", "sky-temperature", -90, 70, 0.5, 1, "°C"),
  quantity("groundTempC", "gnd", "ground-temperature", -60, 90, 0.5, 1, "°C"),
  quantity("relativeHumidity", "rh", "humidity", 0, 100, 1, 0, "%", PER_CENT),
  quantity("windMS", "wind", "wind-speed", 0, 40, 0.1, 1, "m/s"),
  quantity("windDirectionDeg", "wdir", "wind-direction", 0, 360, 5, 0, "°", { wrap: 360 }),
  quantity("solarWM2", "sun", "sun", 0, 1400, 10, 0, "W/m²"),
  quantity("diffuseFraction", "dif", "diffuse", 0, 100, 1, 0, "%", PER_CENT),
  quantity("pressurePa", "hpa", "pressure", 300, 1100, 1, 2, "hPa", { shown: (stored) => stored / 100, stored: (shown) => Math.round(shown * 100) }),
] as const satisfies readonly Quantity<keyof Environment>[];

/** 🔑️ The environment quantities by field. */
export const ENVIRONMENT_QUANTITY: Readonly<Record<keyof Environment, Quantity<keyof Environment>>> = Object.fromEntries(ENVIRONMENT_QUANTITIES.map((entry) => [entry.key, entry])) as unknown as Readonly<Record<keyof Environment, Quantity<keyof Environment>>>;

const rounded = (value: number, digits: number): number => Number(value.toFixed(digits));

/** 👁️ The number a reader sees for a stored value: converted and rounded to the decimals of the quantity. */
export const shownValue = (entry: Quantity, stored: number): number => rounded(entry.shown(stored), entry.digits);

/** 💾️ The stored value of a shown number: folded when the quantity wraps, held inside its range, rounded to its decimals. */
export function storedValue(entry: Quantity, shown: number): number {
  const folded = entry.wrap === undefined ? shown : ((shown % entry.wrap) + entry.wrap) % entry.wrap;
  return entry.stored(rounded(Math.min(Math.max(folded, entry.min), entry.wrap === undefined ? entry.max : entry.wrap), entry.digits) % (entry.wrap ?? Infinity));
}

/** 📥️ Reads the quantities that a source (an address query, the attributes of an element) names with a finite number; `name` picks which name of the quantity the source uses. */
export function readQuantities<K extends string>(list: readonly Quantity<K>[], get: (name: string) => string | null, name: "param" | "attribute"): Partial<Record<K, number>> {
  const values: Partial<Record<K, number>> = {};
  for (const entry of list) {
    const text = get(entry[name]);
    const parsed = text === null || text.trim() === "" ? Number.NaN : Number(text);
    if (Number.isFinite(parsed)) values[entry.key] = storedValue(entry, parsed);
  }
  return values;
}

/** 📤️ Writes the given quantities to a sink in the order of the list. */
export function writeQuantities<K extends string>(list: readonly Quantity<K>[], values: Partial<Record<K, number>>, set: (name: string, value: string) => void, name: "param" | "attribute" = "param"): void {
  for (const entry of list) {
    const value = values[entry.key];
    if (value !== undefined) set(entry[name], String(shownValue(entry, value)));
  }
}
