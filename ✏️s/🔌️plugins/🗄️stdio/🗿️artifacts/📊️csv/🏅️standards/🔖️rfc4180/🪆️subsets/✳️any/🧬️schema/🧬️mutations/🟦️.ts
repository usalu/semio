/** 🧬️ CsvMutation union — mirrors 🦀️.rs's `#[serde(tag = "mutation")]` enum. */
export type CsvMutation =
  | { mutation: 'setHasHeader'; hasHeader: boolean }
  | { mutation: 'insertRecord'; index: number; record: import('../📸️snapshot/🟦️.ts').CsvRecord }
  | { mutation: 'removeRecord'; index: number }
  | { mutation: 'setField'; recordIndex: number; fieldIndex: number; value: string; quoted: boolean }
