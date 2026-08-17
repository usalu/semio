/** 📊️ Remodel editor — Report window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelSnapshot, config: &RemodelConfig)` boundary — a `TableScene` over
 * whichever reconstruction dataset `config.report_table` selects (`report_table_json`'s own six
 * dataset shapes, camera/tracks/gcps/qcStages/matches/frames). The read-only viewer has no
 * mutation-capable twin for this window (the model window is the only one the viewer ports today). */

/** 📊️ Which dataset `report_table_json` renders — mirrors the six branches of that Rust `match`. */
export type RemodelReportTableName = "frames" | "cameras" | "tracks" | "gcps" | "qcStages" | "matches";

export interface RemodelReportColumn {
  id: string;
  label: string;
}

/** 📊️ One row — the shape varies per `RemodelReportTableName`, so cells stay loosely typed here,
 * exactly as `report_table_json`'s own `serde_json::json!` row builders do per branch. */
export type RemodelReportRow = Record<string, string | number | boolean | null>;

/** 📊️ The Report window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelReportViewModel {
  windowKindId: "remodel-report";
  bodyKey: "remodel.play.report";
  surfaceId: "remodel.play.report";
  table: RemodelReportTableName;
  columns: RemodelReportColumn[];
  rows: RemodelReportRow[];
}

export const REMODEL_PLAY_WINDOW_REPORT = "remodel-report" as const;
export const REMODEL_PLAY_BODY_REPORT = "remodel.play.report" as const;
