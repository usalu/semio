/** 🧬️ StepMutation union — discriminated on `mutation`, mirroring the Rust `StepMutation` enum. */

import type { StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepSnapshot, StepValue } from '../📸️snapshot/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

export type StepMutation =
  | { mutation: 'setSnapshot'; snapshot: StepSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: 'setFileDescription'; fileDescription: StepFileDescription }
  | { mutation: 'setFileName'; fileName: StepFileName }
  | { mutation: 'setFileSchema'; fileSchema: StepFileSchema }
  | { mutation: 'insertEntity'; index: number; entity: StepEntity }
  | { mutation: 'removeEntity'; id: StepEntity["id"] }
  | { mutation: 'setEntityName'; id: StepEntity["id"]; name: string }
  | { mutation: 'setEntityArg'; id: StepEntity["id"]; argIndex: number; value: StepValue }
  | { mutation: 'insertEntityArg'; id: StepEntity["id"]; argIndex: number; value: StepValue }
  | { mutation: 'removeEntityArg'; id: StepEntity["id"]; argIndex: number };
