/** 📏️ The displayable thermal quantities with their units, picks, documented fixed comparison ranges and diverging centres.
 *
 * Shared ranges are identical for every asset, scale and time so figures compare: temperatures −20…60 °C (outer surfaces, centred on the air temperature) and 10…40 °C (skin, centred on the neutral skin temperature 33 °C); emitted radiant flux 230…700 W/m² (the emission σT⁴ of a black surface between −20 and 60 °C), centred on σT_air⁴; net radiative and sensible
 * flux ±200 W/m²; latent flux ±300 W/m²; moisture flux ±450 g/(m² h) (300 W/m² at 20 °C), all centred on 0. Values beyond a range saturate the colour and are never clipped
 * in the inspected value.
 *
 * @see ../../🔗️contracts/🟦️.ts — THERMAL_QUANTITIES, RegionField
 */
import type { Environment, Text } from "../../../🧬️schema/🟦️.ts";
import { type RegionField, THERMAL_QUANTITIES, type ThermalQuantity } from "../../🔗️contracts/🟦️.ts";
import { SIGMA, kelvin } from "../🔬️physics/🟦️.ts";

/** 📏️ How one quantity is read, labelled and ranged. */
export type QuantitySpec = {
  readonly quantity: ThermalQuantity;
  readonly title: Text;
  readonly unit: string;
  readonly digits: number;
  readonly pick: (r: RegionField) => number | null;
  readonly centre: (env: Environment) => number;
  readonly shared: { readonly min: number; readonly max: number };
  readonly centreMeaning: Text;
  readonly meaning: Text;
};

const tx = (en: string, de: string): Text => ({ en, de });

/** 📏️ Specification of every {@link ThermalQuantity}. */
export const QUANTITY_SPECS: Readonly<Record<ThermalQuantity, QuantitySpec>> = {
  "surface-temperature": {
    quantity: "surface-temperature",
    title: tx("Surface temperature", "Oberflächentemperatur"),
    unit: "°C",
    digits: 1,
    pick: (r) => r.surfaceTempC,
    centre: (e) => e.airTempC,
    shared: { min: -20, max: 60 },
    centreMeaning: tx("air temperature", "Lufttemperatur"),
    meaning: tx("Temperature of the outermost surface (clothing, fur, plumage, bark, leaf, paint) in contact with the air.", "Temperatur der äußersten Oberfläche (Kleidung, Fell, Gefieder, Rinde, Blatt, Anstrich) in Kontakt mit der Luft."),
  },
  "skin-temperature": {
    quantity: "skin-temperature",
    title: tx("Skin temperature (beneath clothing, fur or feathers)", "Hauttemperatur (unter Kleidung, Fell oder Federn)"),
    unit: "°C",
    digits: 1,
    pick: (r) => r.skinTempC,
    centre: () => 33,
    shared: { min: 10, max: 40 },
    centreMeaning: tx("neutral skin temperature", "neutrale Hauttemperatur"),
    meaning: tx("Temperature of the skin itself; covered areas show the skin value beneath the layer, not the outer surface. Regions without skin show their surface temperature.", "Temperatur der Haut selbst; bedeckte Bereiche zeigen den Hautwert unter der Schicht, nicht die Außenfläche. Regionen ohne Haut zeigen ihre Oberflächentemperatur."),
  },
  "emitted-radiant-flux": {
    quantity: "emitted-radiant-flux",
    title: tx("Emitted radiant flux εσT⁴", "Abgestrahlte Strahlungsleistung εσT⁴"),
    unit: "W/m²",
    digits: 0,
    pick: (r) => r.emittedRadiantWM2,
    centre: (e) => SIGMA * kelvin(e.airTempC) ** 4,
    shared: { min: 230, max: 700 },
    centreMeaning: tx("σT⁴ of a black surface at air temperature", "σT⁴ einer schwarzen Fläche bei Lufttemperatur"),
    meaning: tx("Longwave power the surface itself emits per area (what a thermal camera sees), independent of what it receives.", "Langwellige Leistung, die die Fläche selbst je Fläche abstrahlt (was eine Wärmebildkamera sieht), unabhängig vom Empfang."),
  },
  "net-radiative-flux": {
    quantity: "net-radiative-flux",
    title: tx("Net radiative flux", "Netto-Strahlungsstrom"),
    unit: "W/m²",
    digits: 0,
    pick: (r) => r.netRadiativeWM2,
    centre: () => 0,
    shared: { min: -200, max: 200 },
    centreMeaning: tx("0 W/m²", "0 W/m²"),
    meaning: tx("Longwave exchange with sky, ground and surroundings; positive = the surface loses heat by radiation. Absorbed sunlight is not included.", "Langwelliger Austausch mit Himmel, Boden und Umgebung; positiv = die Fläche verliert Wärme durch Strahlung. Absorbiertes Sonnenlicht ist nicht enthalten."),
  },
  "sensible-heat-flux": {
    quantity: "sensible-heat-flux",
    title: tx("Sensible heat flux (convection)", "Fühlbarer Wärmestrom (Konvektion)"),
    unit: "W/m²",
    digits: 0,
    pick: (r) => r.sensibleWM2,
    centre: () => 0,
    shared: { min: -200, max: 200 },
    centreMeaning: tx("0 W/m²", "0 W/m²"),
    meaning: tx("Heat convected from the surface to the air; positive = the air is heated.", "Von der Fläche an die Luft konvektiv abgegebene Wärme; positiv = die Luft wird erwärmt."),
  },
  "latent-heat-flux": {
    quantity: "latent-heat-flux",
    title: tx("Latent heat flux (evaporation)", "Latenter Wärmestrom (Verdunstung)"),
    unit: "W/m²",
    digits: 0,
    pick: (r) => r.latentWM2,
    centre: () => 0,
    shared: { min: -300, max: 300 },
    centreMeaning: tx("0 W/m²", "0 W/m²"),
    meaning: tx("Heat carried away by evaporating water (sweat, skin diffusion, transpiration, wet surfaces); positive = heat leaves the surface.", "Durch verdunstendes Wasser (Schweiß, Hautdiffusion, Transpiration, nasse Flächen) abgeführte Wärme; positiv = Wärme verlässt die Fläche."),
  },
  "moisture-flux": {
    quantity: "moisture-flux",
    title: tx("Moisture flux", "Feuchtestrom"),
    unit: "g/(m²·h)",
    digits: 0,
    pick: (r) => r.moistureGM2H,
    centre: () => 0,
    shared: { min: -450, max: 450 },
    centreMeaning: tx("0 g/(m²·h)", "0 g/(m²·h)"),
    meaning: tx("Water vapour released by the surface per area and hour; positive = vapour leaves the surface.", "Von der Fläche je Fläche und Stunde abgegebener Wasserdampf; positiv = Dampf verlässt die Fläche."),
  },
};

/** 📏️ The quantities in display order. */
export const QUANTITIES = THERMAL_QUANTITIES;

/** 📏️ The value of a quantity of a region, `null` when the region does not have it (skin temperature of non-bodies). */
export const valueOf = (quantity: ThermalQuantity, r: RegionField): number | null => QUANTITY_SPECS[quantity].pick(r);
