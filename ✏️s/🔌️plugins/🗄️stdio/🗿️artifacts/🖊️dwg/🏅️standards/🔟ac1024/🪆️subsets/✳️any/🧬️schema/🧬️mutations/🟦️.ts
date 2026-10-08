import type { DwgSnapshot } from '../📸️snapshot/🟦️.ts';
export type DwgMutation =
  | { mutation: 'setVersionInfo'; version: string; maintenanceVersion: number; codepage: number };
