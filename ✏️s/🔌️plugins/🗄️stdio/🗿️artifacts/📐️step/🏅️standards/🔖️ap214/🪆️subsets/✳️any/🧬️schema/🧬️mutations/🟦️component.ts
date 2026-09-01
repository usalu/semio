/** 🧬️ StepMutation union — discriminated on `mutation`, mirroring the Rust `StepMutation` enum. */

import type { StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepSnapshot, StepValue } from '../📸️snapshot/🟦️component.ts';

export type StepMutation =
  | { mutation: 'setSnapshot'; snapshot: StepSnapshot }
  | { mutation: 'setFileDescription'; fileDescription: StepFileDescription }
  | { mutation: 'setFileName'; fileName: StepFileName }
  | { mutation: 'setFileSchema'; fileSchema: StepFileSchema }
  | { mutation: 'insertEntity'; index: number; entity: StepEntity }
  | { mutation: 'removeEntity'; id: number }
  | { mutation: 'setEntityName'; id: number; name: string }
  | { mutation: 'setEntityArg'; id: number; argIndex: number; value: StepValue }
  | { mutation: 'insertEntityArg'; id: number; argIndex: number; value: StepValue }
  | { mutation: 'removeEntityArg'; id: number; argIndex: number };
