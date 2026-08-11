/** 🧬️ PlyMutation union — discriminated on `mutation`, mirroring the Rust `PlyMutation` enum. */

import type { PlyElement, PlyFormat, PlyRow, PlySnapshot, PlyValue } from '../📸️snapshot/🟦️component.ts';

export type PlyMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: PlySnapshot }
  | { mutation: 'setFormat'; format: PlyFormat }
  | { mutation: 'insertComment'; index: number; comment: string }
  | { mutation: 'removeComment'; index: number }
  | { mutation: 'addElement'; index: number; element: PlyElement }
  | { mutation: 'removeElement'; name: string }
  | { mutation: 'insertRow'; elementName: string; index: number; row: PlyRow }
  | { mutation: 'removeRow'; elementName: string; index: number }
  | { mutation: 'setRowProperty'; elementName: string; rowIndex: number; propertyName: string; value: PlyValue };
