/** 🧬️ DwgMutation union. AC1018 re-exports AC1024's mutations wholesale
 *  (`🦀️.rs`: `pub use ...v_ac1024::subsets::any::schema::mutations::*;`), so this mirrors
 *  AC1024's variant set exactly. */
export type DwgMutation =
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️.ts').DwgSnapshot }
  | { mutation: 'setVersionInfo'; version: string; maintenanceVersion: number; codepage: number };
