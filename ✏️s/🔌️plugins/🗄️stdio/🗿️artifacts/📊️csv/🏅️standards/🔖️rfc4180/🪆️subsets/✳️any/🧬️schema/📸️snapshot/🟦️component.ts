/** 🧬️ CsvSnapshot schema facet — mirrors 🦀️component.rs field-for-field. */

/** 🔤 One RFC 4180 field value plus whether the source quoted it. */
export interface CsvField {
  value: string;
  quoted: boolean;
}

/** 📄 One RFC 4180 record (row) — index-keyed within `CsvSnapshot.records`. */
export interface CsvRecord {
  fields: CsvField[];
}

/** 📸️ Persisted `stdio.csv` snapshot. `records[0]` is the header row when `hasHeader`. */
export interface CsvSnapshot {
  schema: string;
  hasHeader: boolean;
  records: CsvRecord[];
}
