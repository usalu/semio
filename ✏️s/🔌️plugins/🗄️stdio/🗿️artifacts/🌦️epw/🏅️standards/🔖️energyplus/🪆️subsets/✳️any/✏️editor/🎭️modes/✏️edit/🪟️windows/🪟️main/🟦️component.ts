/** 🌡️ EPW editor — main window: typed twin of `🦀️component.rs`'s `TableWindowKit` view-model.
 * Mirrors the Rust `render()` boundary's output shape — one row per hourly `EpwRecord`. */

/** 🔢️ The 35 EPW record columns, in `EpwRecord::field_at`'s canonical wire order — mirrors the Rust
 * `EPW_TABLE_COLUMNS` constant. */
export const EPW_TABLE_COLUMNS = [
  "year",
  "month",
  "day",
  "hour",
  "minute",
  "dataSourceUncertainty",
  "dryBulbTemp",
  "dewPointTemp",
  "relativeHumidity",
  "atmosphericPressure",
  "extraterrestrialHorizontalRadiation",
  "extraterrestrialDirectNormalRadiation",
  "horizontalInfraredRadiation",
  "globalHorizontalRadiation",
  "directNormalRadiation",
  "diffuseHorizontalRadiation",
  "globalHorizontalIlluminance",
  "directNormalIlluminance",
  "diffuseHorizontalIlluminance",
  "zenithLuminance",
  "windDirection",
  "windSpeed",
  "totalSkyCover",
  "opaqueSkyCover",
  "visibility",
  "ceilingHeight",
  "presentWeatherObservation",
  "presentWeatherCodes",
  "precipitableWater",
  "aerosolOpticalDepth",
  "snowDepth",
  "daysSinceLastSnowfall",
  "albedo",
  "liquidPrecipDepth",
  "liquidPrecipQuantity",
] as const;

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `EpwSnapshot`). */
export interface EpwEditMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: typeof EPW_TABLE_COLUMNS;
  rows: string[][];
}

/** ✏️ `set-cell` payload shape — mirrors `EpwEditorCommand::SetCell`. `row` indexes
 * `EpwSnapshot.records` directly; `column` is one of `EPW_TABLE_COLUMNS`. */
export interface EpwSetCell {
  row: number;
  column: (typeof EPW_TABLE_COLUMNS)[number];
  value: string;
}

export const EPW_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const EPW_EDIT_MAIN_BODY_KEY = "framework.window.table" as const;
