/** 🌫️ Moist-air thermodynamics of the transport model: saturation, humidity ratio, enthalpy with liquid water, saturation adjustment and density.
 *
 * Mixing is conserved per kilogram of dry air in total enthalpy `h` and total water `w_t` (vapour + liquid), so a mixture of warm saturated air and cold
 * air condenses along the mixing line and releases its latent heat. Saturation is over liquid water (Buck 1996 vapour pressure, within 0.05 % of IAPWS-95
 * for 0…40 °C), also below 0 °C (supercooled liquid; no ice phase). The saturation humidity ratio multiplies that pressure by the enhancement factor
 * `1.00062 + 3.14×10⁻⁸·P + 5.6×10⁻⁷·t²` (P in Pa, t in °C), which is what humid-air oracles such as CoolProp report.
 *
 * @see https://doi.org/10.1175/1520-0450(1996)035<0601:NEFCVP>2.0.CO;2 — Buck 1981/1996 saturation vapour pressure
 * @see https://coolprop.org/fluid_properties/HumidAir.html — CoolProp humid air (the third-party oracle)
 */

/** 🌍️ Standard gravity (m/s², CODATA conventional). */
export const GRAVITY_MS2 = 9.80665;
/** 🌫️ Specific heat of dry air (J/(kg·K)). */
export const CP_DRY_AIR = 1006;
/** 🌫️ Specific heat of water vapour (J/(kg·K)). */
export const CP_VAPOUR = 1860;
/** 💧️ Specific heat of liquid water (J/(kg·K)). */
export const CP_LIQUID = 4186;
/** 🔥️ Latent heat of vaporisation at 0 °C (J/kg). */
export const LATENT_HEAT_0C = 2.501e6;
/** 🌫️ Specific gas constant of dry air (J/(kg·K)). */
export const R_DRY_AIR = 287.05;
/** ⚖️ Ratio of the molar masses of water and dry air. */
export const EPSILON = 0.622;

/** 💧️ Saturation vapour pressure over liquid water (Pa) at a temperature in °C. */
export const saturationPressurePa = (tC: number): number => 611.21 * Math.exp(((18.678 - tC / 234.5) * tC) / (257.14 + tC));

/** 💧️ Humidity ratio (kg/kg dry air) of a vapour pressure at a total pressure. */
export const humidityRatio = (vapourPa: number, pressurePa: number): number => (EPSILON * vapourPa) / (pressurePa - vapourPa);

/** 💧️ Saturation humidity ratio of moist air at a temperature; unbounded (never saturated) from the boiling point of the total pressure upwards, where the enhanced vapour pressure reaches it. */
export const saturationRatio = (tC: number, pressurePa: number): number => {
  const e = saturationPressurePa(tC) * (1.00062 + 3.14e-8 * pressurePa + 5.6e-7 * tC * tC);
  return e >= pressurePa ? Number.POSITIVE_INFINITY : humidityRatio(e, pressurePa);
};

/** 💧️ Specific humidity (kg vapour per kg moist air) of a humidity ratio. */
export const specificHumidity = (ratio: number): number => ratio / (1 + ratio);

/** 💧️ Humidity ratio of a specific humidity. */
export const ratioOfSpecific = (q: number): number => q / (1 - q);

/** 🔥️ Total enthalpy (J per kg dry air) of air at `tC` with vapour ratio `wv` and liquid ratio `wl`; zero at 0 °C dry air and 0 °C liquid water. */
export const enthalpy = (tC: number, wv: number, wl: number): number => CP_DRY_AIR * tC + wv * (LATENT_HEAT_0C + CP_VAPOUR * tC) + wl * CP_LIQUID * tC;

/** 🌫️ The thermodynamic state of a parcel: temperature, vapour ratio and liquid ratio (all per kg dry air). */
export type Moist = { readonly tC: number; readonly wv: number; readonly wl: number };

/** 🌫️ Saturation adjustment: the state of air with total enthalpy `h` and total water `wt`; supersaturation condenses until the vapour is saturated at the temperature that closes the enthalpy balance (Newton, converged below 1e-10 K). */
export function moistFrom(h: number, wt: number, pressurePa: number): Moist {
  const tu = (h - wt * LATENT_HEAT_0C) / (CP_DRY_AIR + wt * CP_VAPOUR);
  const ws = saturationRatio(tu, pressurePa);
  if (wt <= ws) return { tC: tu, wv: wt, wl: 0 };
  let t = tu;
  for (let i = 0; i < 40; i++) {
    const a = saturationRatio(t, pressurePa);
    const b = saturationRatio(t + 1e-3, pressurePa);
    const f = enthalpy(t, a, wt - a) - h;
    const df = (enthalpy(t + 1e-3, b, wt - b) - enthalpy(t, a, wt - a)) / 1e-3;
    const next = t - f / df;
    if (Math.abs(next - t) < 1e-11) {
      t = next;
      break;
    }
    t = next;
  }
  const wv = saturationRatio(t, pressurePa);
  return { tC: t, wv, wl: Math.max(0, wt - wv) };
}

/** 📦️ Volume (m³) of moist air per kg dry air from the ideal-gas law; liquid water takes no volume. */
export const volumePerDryKg = (tC: number, wv: number, pressurePa: number): number => (R_DRY_AIR * (tC + 273.15) * (1 + wv / EPSILON)) / pressurePa;

/** 🌫️ Density of moist air (kg/m³). */
export const moistDensity = (tC: number, wv: number, pressurePa: number): number => (1 + wv) / volumePerDryKg(tC, wv, pressurePa);

/** 🔥️ Latent heat of vaporisation (J/kg) at a temperature in °C. */
export const latentHeat = (tC: number): number => LATENT_HEAT_0C - 2361 * tC;
