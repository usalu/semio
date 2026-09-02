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
