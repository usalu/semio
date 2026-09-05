/** 🧬️ SemioCadMutation schema — real facet mirror of `🦀️.rs` (source of truth).
 * Named-variant enum, imperative verbs, discriminated on the `mutation` tag. */
import type { CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot, SemioPoint2 } from "../📸️snapshot/🟦️";

export type SemioCadMutation =
  | { mutation: "setSnapshot"; snapshot: SemioCadSnapshot }
  | { mutation: "addLayer"; layer: CadLayer }
  | { mutation: "removeLayer"; name: string }
  | { mutation: "setLayer"; name: string; colorIndex?: number; lineType?: string; visible?: boolean }
  | { mutation: "addBlock"; block: CadBlock }
  | { mutation: "removeBlock"; name: string }
  | { mutation: "setBlockBasePoint"; name: string; basePoint: SemioPoint2 }
  | { mutation: "addEntity"; entity: CadEntityRecord }
  | { mutation: "removeEntity"; handle: string }
  | { mutation: "setEntityLayer"; handle: string; layer: string }
  | { mutation: "setEntityGeometry"; handle: string; entity: CadEntity }
  | { mutation: "addBlockEntity"; blockName: string; entity: CadEntityRecord }
  | { mutation: "removeBlockEntity"; blockName: string; handle: string }
  | { mutation: "setBlockEntityLayer"; blockName: string; handle: string; layer: string }
  | { mutation: "setBlockEntityGeometry"; blockName: string; handle: string; entity: CadEntity };
