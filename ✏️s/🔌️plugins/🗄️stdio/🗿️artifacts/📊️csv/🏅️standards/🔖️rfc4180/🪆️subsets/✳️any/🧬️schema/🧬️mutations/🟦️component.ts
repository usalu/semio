/** 🧬️ CsvMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
export type CsvMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').CsvSnapshot }
  | { mutation: 'setHasHeader'; hasHeader: boolean }
  | { mutation: 'insertRecord'; index: number; record: import('../📸️snapshot/🟦️component.ts').CsvRecord }
  | { mutation: 'removeRecord'; index: number }
  | { mutation: 'setField'; recordIndex: number; fieldIndex: number; value: string; quoted: boolean };
