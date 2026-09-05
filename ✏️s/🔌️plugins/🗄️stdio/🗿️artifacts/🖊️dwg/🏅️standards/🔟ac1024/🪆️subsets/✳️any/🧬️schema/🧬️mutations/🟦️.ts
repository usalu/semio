import type { DwgSnapshot } from '../📸️snapshot/🟦️.ts';
export type DwgMutation =
  | { mutation: 'setSnapshot'; snapshot: DwgSnapshot }
  | { mutation: 'setVersionInfo'; version: string; maintenanceVersion: number; codepage: number };
