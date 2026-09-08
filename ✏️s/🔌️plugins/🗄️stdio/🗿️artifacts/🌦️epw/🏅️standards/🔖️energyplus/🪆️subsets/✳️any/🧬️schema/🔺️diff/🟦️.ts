/** 🔺️ EpwDiff schema facet — mirrors 🦀️.rs field-for-field. `location`/`dataPeriods`
 * are whole-substruct replace slots; `records` is an index-keyed removed/modified/added triple
 * with a genuinely sparse per-column patch on `modified`. */
import type { EpwLocation, EpwDataPeriods, EpwRecord } from '../📸️snapshot/🟦️.ts';

/** Sparse per-column patch over EpwRecord's 35 fields; every column independently optional. */
export interface EpwRecordDiff {
  year?: string; month?: string; day?: string; hour?: string; minute?: string;
  dataSourceUncertainty?: string; dryBulbTemp?: string; dewPointTemp?: string; relativeHumidity?: string;
  atmosphericPressure?: string; extraterrestrialHorizontalRadiation?: string;
  extraterrestrialDirectNormalRadiation?: string; horizontalInfraredRadiation?: string;
  globalHorizontalRadiation?: string; directNormalRadiation?: string; diffuseHorizontalRadiation?: string;
  globalHorizontalIlluminance?: string; directNormalIlluminance?: string; diffuseHorizontalIlluminance?: string;
  zenithLuminance?: string; windDirection?: string; windSpeed?: string; totalSkyCover?: string;
  opaqueSkyCover?: string; visibility?: string; ceilingHeight?: string; presentWeatherObservation?: string;
  presentWeatherCodes?: string; precipitableWater?: string; aerosolOpticalDepth?: string; snowDepth?: string;
  daysSinceLastSnowfall?: string; albedo?: string; liquidPrecipDepth?: string; liquidPrecipQuantity?: string;
}

export interface EpwRecordModified { index: number; diff: EpwRecordDiff; }
export interface EpwRecordAdded { index: number; record: EpwRecord; }
export interface EpwRecordsDiff { removed?: number[]; modified?: EpwRecordModified[]; added?: EpwRecordAdded[]; }

export interface EpwDiff {
  location?: EpwLocation;
  designConditions?: string;
  typicalExtremePeriods?: string;
  groundTemperatures?: string;
  holidaysDst?: string;
  comments1?: string;
  comments2?: string;
  dataPeriods?: EpwDataPeriods;
  records?: EpwRecordsDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioEpwEnergyplusAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioEpwEnergyplusAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioEpwEnergyplusAnyDiffGuardRefusal(at, why);
};

type stdioEpwEnergyplusAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioEpwEnergyplusAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioEpwEnergyplusAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioEpwEnergyplusAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not an object");
export const stdioEpwEnergyplusAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioEpwEnergyplusAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioEpwEnergyplusAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffGuardString = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioEpwEnergyplusAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioEpwEnergyplusAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioEpwEnergyplusAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioEpwEnergyplusAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioEpwEnergyplusAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioEpwEnergyplusAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioEpwEnergyplusAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioEpwEnergyplusAnyDiffGuardNumber(value, at, bounds) : stdioEpwEnergyplusAnyDiffGuardReject(at, "value is not an integer");
export const stdioEpwEnergyplusAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioEpwEnergyplusAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioEpwEnergyplusAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioEpwEnergyplusAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEpwRecordDiff(value: unknown, at = "$"): EpwRecordDiff {
  const row = stdioEpwEnergyplusAnyDiffGuardObject(value, at);
  return {
    year: row["year"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["year"], `${at}.year`),
    month: row["month"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["month"], `${at}.month`),
    day: row["day"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["day"], `${at}.day`),
    hour: row["hour"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["hour"], `${at}.hour`),
    minute: row["minute"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["minute"], `${at}.minute`),
    dataSourceUncertainty: row["dataSourceUncertainty"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["dataSourceUncertainty"], `${at}.dataSourceUncertainty`),
    dryBulbTemp: row["dryBulbTemp"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["dryBulbTemp"], `${at}.dryBulbTemp`),
    dewPointTemp: row["dewPointTemp"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["dewPointTemp"], `${at}.dewPointTemp`),
    relativeHumidity: row["relativeHumidity"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["relativeHumidity"], `${at}.relativeHumidity`),
    atmosphericPressure: row["atmosphericPressure"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["atmosphericPressure"], `${at}.atmosphericPressure`),
    extraterrestrialHorizontalRadiation: row["extraterrestrialHorizontalRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["extraterrestrialHorizontalRadiation"], `${at}.extraterrestrialHorizontalRadiation`),
    extraterrestrialDirectNormalRadiation: row["extraterrestrialDirectNormalRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["extraterrestrialDirectNormalRadiation"], `${at}.extraterrestrialDirectNormalRadiation`),
    horizontalInfraredRadiation: row["horizontalInfraredRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["horizontalInfraredRadiation"], `${at}.horizontalInfraredRadiation`),
    globalHorizontalRadiation: row["globalHorizontalRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["globalHorizontalRadiation"], `${at}.globalHorizontalRadiation`),
    directNormalRadiation: row["directNormalRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["directNormalRadiation"], `${at}.directNormalRadiation`),
    diffuseHorizontalRadiation: row["diffuseHorizontalRadiation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["diffuseHorizontalRadiation"], `${at}.diffuseHorizontalRadiation`),
    globalHorizontalIlluminance: row["globalHorizontalIlluminance"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["globalHorizontalIlluminance"], `${at}.globalHorizontalIlluminance`),
    directNormalIlluminance: row["directNormalIlluminance"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["directNormalIlluminance"], `${at}.directNormalIlluminance`),
    diffuseHorizontalIlluminance: row["diffuseHorizontalIlluminance"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["diffuseHorizontalIlluminance"], `${at}.diffuseHorizontalIlluminance`),
    zenithLuminance: row["zenithLuminance"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["zenithLuminance"], `${at}.zenithLuminance`),
    windDirection: row["windDirection"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["windDirection"], `${at}.windDirection`),
    windSpeed: row["windSpeed"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["windSpeed"], `${at}.windSpeed`),
    totalSkyCover: row["totalSkyCover"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["totalSkyCover"], `${at}.totalSkyCover`),
    opaqueSkyCover: row["opaqueSkyCover"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["opaqueSkyCover"], `${at}.opaqueSkyCover`),
    visibility: row["visibility"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["visibility"], `${at}.visibility`),
    ceilingHeight: row["ceilingHeight"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["ceilingHeight"], `${at}.ceilingHeight`),
    presentWeatherObservation: row["presentWeatherObservation"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["presentWeatherObservation"], `${at}.presentWeatherObservation`),
    presentWeatherCodes: row["presentWeatherCodes"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["presentWeatherCodes"], `${at}.presentWeatherCodes`),
    precipitableWater: row["precipitableWater"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["precipitableWater"], `${at}.precipitableWater`),
    aerosolOpticalDepth: row["aerosolOpticalDepth"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["aerosolOpticalDepth"], `${at}.aerosolOpticalDepth`),
    snowDepth: row["snowDepth"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["snowDepth"], `${at}.snowDepth`),
    daysSinceLastSnowfall: row["daysSinceLastSnowfall"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["daysSinceLastSnowfall"], `${at}.daysSinceLastSnowfall`),
    albedo: row["albedo"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["albedo"], `${at}.albedo`),
    liquidPrecipDepth: row["liquidPrecipDepth"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["liquidPrecipDepth"], `${at}.liquidPrecipDepth`),
    liquidPrecipQuantity: row["liquidPrecipQuantity"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardString(row["liquidPrecipQuantity"], `${at}.liquidPrecipQuantity`),
  };
}

export function parseEpwRecordModified(value: unknown, at = "$"): EpwRecordModified {
  const row = stdioEpwEnergyplusAnyDiffGuardObject(value, at);
  return {
    index: stdioEpwEnergyplusAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseEpwRecordDiff(row["diff"], `${at}.diff`),
  };
}

export function parseEpwRecordsDiff(value: unknown, at = "$"): EpwRecordsDiff {
  const row = stdioEpwEnergyplusAnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioEpwEnergyplusAnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseEpwRecordModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioEpwEnergyplusAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseEpwRecordAdded(item, `${at}.added[${index}]`)),
  };
}
