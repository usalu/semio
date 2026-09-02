/** 🧬️ LasMutation union. */
import type { LasSnapshot, LasVlr, LasPoint } from '../📸️snapshot/🟦️.ts';

export type LasMutation =
  | { mutation: 'setSnapshot'; snapshot: LasSnapshot }
  | { mutation: 'setVersion'; major: number; minor: number }
  | { mutation: 'setSystemIdentifier'; systemIdentifier: string }
  | { mutation: 'setSoftwareInfo'; generatingSoftware: string }
  | { mutation: 'setCreationDate'; dayOfYear: number; year: number }
  | { mutation: 'setScaleAndOffset'; scale: [number, number, number]; offset: [number, number, number] }
  | { mutation: 'setBounds'; max: [number, number, number]; min: [number, number, number] }
  | { mutation: 'setPointsByReturn'; counts: [number, number, number, number, number] }
  | { mutation: 'insertVlr'; index: number; vlr: LasVlr }
  | { mutation: 'removeVlr'; index: number }
  | { mutation: 'setVlrData'; index: number; data: number[] }
  | { mutation: 'insertPoint'; index: number; point: LasPoint }
  | { mutation: 'removePoint'; index: number }
  | { mutation: 'setPoint'; index: number; point: LasPoint };
