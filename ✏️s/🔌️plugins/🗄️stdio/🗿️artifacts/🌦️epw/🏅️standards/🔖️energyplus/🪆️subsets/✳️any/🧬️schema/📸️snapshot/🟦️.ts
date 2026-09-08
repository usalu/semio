/** 🧬️ EpwSnapshot schema facet — mirrors 🦀️.rs field-for-field. All numeric-looking
 * columns are `string` (retention, see the Rust file's module doc comment for why). */

export interface EpwLocation {
  city: string; stateProvince: string; country: string; source: string; wmo: string;
  latitude: string; longitude: string; timeZone: string; elevation: string;
}

export interface EpwDataPeriod {
  name: string; startDayOfWeek: string; startDate: string; endDate: string;
}

export interface EpwDataPeriods {
  recordsPerHour: number;
  periods: EpwDataPeriod[];
}

/** 🌡️ One hourly EPW data record — all 35 spec columns, in spec order. */
export interface EpwRecord {
  year: string; month: string; day: string; hour: string; minute: string;
  dataSourceUncertainty: string; dryBulbTemp: string; dewPointTemp: string; relativeHumidity: string;
  atmosphericPressure: string; extraterrestrialHorizontalRadiation: string;
  extraterrestrialDirectNormalRadiation: string; horizontalInfraredRadiation: string;
  globalHorizontalRadiation: string; directNormalRadiation: string; diffuseHorizontalRadiation: string;
  globalHorizontalIlluminance: string; directNormalIlluminance: string; diffuseHorizontalIlluminance: string;
  zenithLuminance: string; windDirection: string; windSpeed: string; totalSkyCover: string;
  opaqueSkyCover: string; visibility: string; ceilingHeight: string; presentWeatherObservation: string;
  presentWeatherCodes: string; precipitableWater: string; aerosolOpticalDepth: string; snowDepth: string;
  daysSinceLastSnowfall: string; albedo: string; liquidPrecipDepth: string; liquidPrecipQuantity: string;
}

export interface EpwSnapshot {
  schema: string;
  location: EpwLocation;
  designConditions: string;
  typicalExtremePeriods: string;
  groundTemperatures: string;
  holidaysDst: string;
  comments1: string;
  comments2: string;
  dataPeriods: EpwDataPeriods;
  records: EpwRecord[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioEpwEnergyplusAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioEpwEnergyplusAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioEpwEnergyplusAnySnapshotGuardRefusal(at, why);
};

type stdioEpwEnergyplusAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioEpwEnergyplusAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioEpwEnergyplusAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioEpwEnergyplusAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not an object");
export const stdioEpwEnergyplusAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioEpwEnergyplusAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioEpwEnergyplusAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioEpwEnergyplusAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioEpwEnergyplusAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioEpwEnergyplusAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioEpwEnergyplusAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioEpwEnergyplusAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioEpwEnergyplusAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioEpwEnergyplusAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioEpwEnergyplusAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioEpwEnergyplusAnySnapshotGuardNumber(value, at, bounds) : stdioEpwEnergyplusAnySnapshotGuardReject(at, "value is not an integer");
export const stdioEpwEnergyplusAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioEpwEnergyplusAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioEpwEnergyplusAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioEpwEnergyplusAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEpwSnapshot(value: unknown, at = "$"): EpwSnapshot {
  const row = stdioEpwEnergyplusAnySnapshotGuardObject(value, at);
  return {
    schema: stdioEpwEnergyplusAnySnapshotGuardString(row["schema"], `${at}.schema`),
    location: parseEpwLocation(row["location"], `${at}.location`),
    designConditions: stdioEpwEnergyplusAnySnapshotGuardString(row["designConditions"], `${at}.designConditions`),
    typicalExtremePeriods: stdioEpwEnergyplusAnySnapshotGuardString(row["typicalExtremePeriods"], `${at}.typicalExtremePeriods`),
    groundTemperatures: stdioEpwEnergyplusAnySnapshotGuardString(row["groundTemperatures"], `${at}.groundTemperatures`),
    holidaysDst: stdioEpwEnergyplusAnySnapshotGuardString(row["holidaysDst"], `${at}.holidaysDst`),
    comments1: stdioEpwEnergyplusAnySnapshotGuardString(row["comments1"], `${at}.comments1`),
    comments2: stdioEpwEnergyplusAnySnapshotGuardString(row["comments2"], `${at}.comments2`),
    dataPeriods: parseEpwDataPeriods(row["dataPeriods"], `${at}.dataPeriods`),
    records: stdioEpwEnergyplusAnySnapshotGuardArray(row["records"], `${at}.records`).map((item, index) => parseEpwRecord(item, `${at}.records[${index}]`)),
  };
}

export function parseEpwLocation(value: unknown, at = "$"): EpwLocation {
  const row = stdioEpwEnergyplusAnySnapshotGuardObject(value, at);
  return {
    city: stdioEpwEnergyplusAnySnapshotGuardString(row["city"], `${at}.city`),
    stateProvince: stdioEpwEnergyplusAnySnapshotGuardString(row["stateProvince"], `${at}.stateProvince`),
    country: stdioEpwEnergyplusAnySnapshotGuardString(row["country"], `${at}.country`),
    source: stdioEpwEnergyplusAnySnapshotGuardString(row["source"], `${at}.source`),
    wmo: stdioEpwEnergyplusAnySnapshotGuardString(row["wmo"], `${at}.wmo`),
    latitude: stdioEpwEnergyplusAnySnapshotGuardString(row["latitude"], `${at}.latitude`),
    longitude: stdioEpwEnergyplusAnySnapshotGuardString(row["longitude"], `${at}.longitude`),
    timeZone: stdioEpwEnergyplusAnySnapshotGuardString(row["timeZone"], `${at}.timeZone`),
    elevation: stdioEpwEnergyplusAnySnapshotGuardString(row["elevation"], `${at}.elevation`),
  };
}

export function parseEpwDataPeriod(value: unknown, at = "$"): EpwDataPeriod {
  const row = stdioEpwEnergyplusAnySnapshotGuardObject(value, at);
  return {
    name: stdioEpwEnergyplusAnySnapshotGuardString(row["name"], `${at}.name`),
    startDayOfWeek: stdioEpwEnergyplusAnySnapshotGuardString(row["startDayOfWeek"], `${at}.startDayOfWeek`),
    startDate: stdioEpwEnergyplusAnySnapshotGuardString(row["startDate"], `${at}.startDate`),
    endDate: stdioEpwEnergyplusAnySnapshotGuardString(row["endDate"], `${at}.endDate`),
  };
}

export function parseEpwDataPeriods(value: unknown, at = "$"): EpwDataPeriods {
  const row = stdioEpwEnergyplusAnySnapshotGuardObject(value, at);
  return {
    recordsPerHour: stdioEpwEnergyplusAnySnapshotGuardInteger(row["recordsPerHour"], `${at}.recordsPerHour`, {"minimum": 0}),
    periods: stdioEpwEnergyplusAnySnapshotGuardArray(row["periods"], `${at}.periods`).map((item, index) => parseEpwDataPeriod(item, `${at}.periods[${index}]`)),
  };
}

export function parseEpwRecord(value: unknown, at = "$"): EpwRecord {
  const row = stdioEpwEnergyplusAnySnapshotGuardObject(value, at);
  return {
    year: stdioEpwEnergyplusAnySnapshotGuardString(row["year"], `${at}.year`),
    month: stdioEpwEnergyplusAnySnapshotGuardString(row["month"], `${at}.month`),
    day: stdioEpwEnergyplusAnySnapshotGuardString(row["day"], `${at}.day`),
    hour: stdioEpwEnergyplusAnySnapshotGuardString(row["hour"], `${at}.hour`),
    minute: stdioEpwEnergyplusAnySnapshotGuardString(row["minute"], `${at}.minute`),
    dataSourceUncertainty: stdioEpwEnergyplusAnySnapshotGuardString(row["dataSourceUncertainty"], `${at}.dataSourceUncertainty`),
    dryBulbTemp: stdioEpwEnergyplusAnySnapshotGuardString(row["dryBulbTemp"], `${at}.dryBulbTemp`),
    dewPointTemp: stdioEpwEnergyplusAnySnapshotGuardString(row["dewPointTemp"], `${at}.dewPointTemp`),
    relativeHumidity: stdioEpwEnergyplusAnySnapshotGuardString(row["relativeHumidity"], `${at}.relativeHumidity`),
    atmosphericPressure: stdioEpwEnergyplusAnySnapshotGuardString(row["atmosphericPressure"], `${at}.atmosphericPressure`),
    extraterrestrialHorizontalRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["extraterrestrialHorizontalRadiation"], `${at}.extraterrestrialHorizontalRadiation`),
    extraterrestrialDirectNormalRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["extraterrestrialDirectNormalRadiation"], `${at}.extraterrestrialDirectNormalRadiation`),
    horizontalInfraredRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["horizontalInfraredRadiation"], `${at}.horizontalInfraredRadiation`),
    globalHorizontalRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["globalHorizontalRadiation"], `${at}.globalHorizontalRadiation`),
    directNormalRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["directNormalRadiation"], `${at}.directNormalRadiation`),
    diffuseHorizontalRadiation: stdioEpwEnergyplusAnySnapshotGuardString(row["diffuseHorizontalRadiation"], `${at}.diffuseHorizontalRadiation`),
    globalHorizontalIlluminance: stdioEpwEnergyplusAnySnapshotGuardString(row["globalHorizontalIlluminance"], `${at}.globalHorizontalIlluminance`),
    directNormalIlluminance: stdioEpwEnergyplusAnySnapshotGuardString(row["directNormalIlluminance"], `${at}.directNormalIlluminance`),
    diffuseHorizontalIlluminance: stdioEpwEnergyplusAnySnapshotGuardString(row["diffuseHorizontalIlluminance"], `${at}.diffuseHorizontalIlluminance`),
    zenithLuminance: stdioEpwEnergyplusAnySnapshotGuardString(row["zenithLuminance"], `${at}.zenithLuminance`),
    windDirection: stdioEpwEnergyplusAnySnapshotGuardString(row["windDirection"], `${at}.windDirection`),
    windSpeed: stdioEpwEnergyplusAnySnapshotGuardString(row["windSpeed"], `${at}.windSpeed`),
    totalSkyCover: stdioEpwEnergyplusAnySnapshotGuardString(row["totalSkyCover"], `${at}.totalSkyCover`),
    opaqueSkyCover: stdioEpwEnergyplusAnySnapshotGuardString(row["opaqueSkyCover"], `${at}.opaqueSkyCover`),
    visibility: stdioEpwEnergyplusAnySnapshotGuardString(row["visibility"], `${at}.visibility`),
    ceilingHeight: stdioEpwEnergyplusAnySnapshotGuardString(row["ceilingHeight"], `${at}.ceilingHeight`),
    presentWeatherObservation: stdioEpwEnergyplusAnySnapshotGuardString(row["presentWeatherObservation"], `${at}.presentWeatherObservation`),
    presentWeatherCodes: stdioEpwEnergyplusAnySnapshotGuardString(row["presentWeatherCodes"], `${at}.presentWeatherCodes`),
    precipitableWater: stdioEpwEnergyplusAnySnapshotGuardString(row["precipitableWater"], `${at}.precipitableWater`),
    aerosolOpticalDepth: stdioEpwEnergyplusAnySnapshotGuardString(row["aerosolOpticalDepth"], `${at}.aerosolOpticalDepth`),
    snowDepth: stdioEpwEnergyplusAnySnapshotGuardString(row["snowDepth"], `${at}.snowDepth`),
    daysSinceLastSnowfall: stdioEpwEnergyplusAnySnapshotGuardString(row["daysSinceLastSnowfall"], `${at}.daysSinceLastSnowfall`),
    albedo: stdioEpwEnergyplusAnySnapshotGuardString(row["albedo"], `${at}.albedo`),
    liquidPrecipDepth: stdioEpwEnergyplusAnySnapshotGuardString(row["liquidPrecipDepth"], `${at}.liquidPrecipDepth`),
    liquidPrecipQuantity: stdioEpwEnergyplusAnySnapshotGuardString(row["liquidPrecipQuantity"], `${at}.liquidPrecipQuantity`),
  };
}
