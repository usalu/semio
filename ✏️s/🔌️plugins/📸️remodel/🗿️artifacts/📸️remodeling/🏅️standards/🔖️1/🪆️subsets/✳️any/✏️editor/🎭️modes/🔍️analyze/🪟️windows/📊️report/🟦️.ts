/** 📊️ Remodeling editor — Report window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelingSnapshot, config: &RemodelingConfig)` boundary — a `TableScene` over
 * whichever reconstruction dataset `config.report_table` selects (`report_table_json`'s own six
 * dataset shapes, camera/tracks/gcps/qcStages/matches/frames). The read-only viewer has no
 * mutation-capable twin for this window (the model window is the only one the viewer ports today). */

/** 📊️ Which dataset `report_table_json` renders — mirrors the six branches of that Rust `match`. */
export type RemodelingReportTableName = "frames" | "cameras" | "tracks" | "gcps" | "qcStages" | "matches";

export interface RemodelingReportColumn {
  id: string;
  label: string;
}

/** 📊️ One row — the shape varies per `RemodelingReportTableName`, so cells stay loosely typed here,
 * exactly as `report_table_json`'s own `serde_json::json!` row builders do per branch. */
export type RemodelingReportRow = Record<string, string | number | boolean | null>;

/** 📊️ The Report window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelingReportViewModel {
  windowKindId: "remodeling-report";
  bodyKey: "remodeling.play.report";
  surfaceId: "remodeling.play.report";
  table: RemodelingReportTableName;
  columns: RemodelingReportColumn[];
  rows: RemodelingReportRow[];
}

export const REMODELING_PLAY_WINDOW_REPORT = "remodeling-report" as const;
export const REMODELING_PLAY_BODY_REPORT = "remodeling.play.report" as const;
