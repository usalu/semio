/** 🔺️ EpwDiff schema facet — mirrors 🦀️component.rs field-for-field. `location`/`dataPeriods`
 * are whole-substruct replace slots; `records` is an index-keyed removed/modified/added triple
 * with a genuinely sparse per-column patch on `modified`. */
import type { EpwLocation, EpwDataPeriods, EpwRecord } from '../📸️snapshot/🟦️component.ts';

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
