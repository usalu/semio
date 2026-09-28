import { parsePptxSnapshot, pptxGuardObject } from '../../../📸️snapshot/🟦️.ts';
import type { PptxSnapshot } from '../../../📸️snapshot/🟦️.ts';

/** 🧩 Complete replacement payload for one PPTX snapshot. */
export interface PptxSetSnapshotMutation { snapshot: PptxSnapshot }

/** 🚪️ Parses the replacement through the shared snapshot authority. */
export function parsePptxSetSnapshotMutation(value: unknown, at = '$'): PptxSetSnapshotMutation {
  return { snapshot: parsePptxSnapshot(pptxGuardObject(value, at).snapshot, `${at}.snapshot`) };
}
