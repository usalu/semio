/** 🧬️ SemioCadSnapshot schema — real facet mirror of `🦀️.rs` (source of truth). */
import type {SemioPoint2} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type {SemioPoint2} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";

export type CadEntity =
  | { kind: "line"; a: SemioPoint2; b: SemioPoint2 }
  | { kind: "arc"; center: SemioPoint2; radius: Binary64; startAngle: Binary64; endAngle: Binary64 }
  | { kind: "circle"; center: SemioPoint2; radius: Binary64 }
  | { kind: "ellipse"; center: SemioPoint2; majorAxisEnd: SemioPoint2; ratio: Binary64; startParam: Binary64; endParam: Binary64 }
  | { kind: "polyline"; vertices: SemioPoint2[]; closed: boolean }
  | { kind: "text"; position: SemioPoint2; height: Binary64; rotation: Binary64; content: string }
  | { kind: "insert"; blockName: string; insertionPoint: SemioPoint2; scale: SemioPoint2; rotation: Binary64 }
  | { kind: "solid"; p1: SemioPoint2; p2: SemioPoint2; p3: SemioPoint2; p4: SemioPoint2 }
  | { kind: "dimension"; defPoint: SemioPoint2; textPosition: SemioPoint2; measurement: Binary64; text: string };

export interface CadLayer {
  name: string;
  colorIndex: number;
  lineType: string;
  visible: boolean;
}

/** Referential invariant (checked by `SemioCadValidator`, not the type system): `layer` must name a real `CadLayer`. */
export interface CadEntityRecord {
  handle: string;
  layer: string;
  entity: CadEntity;
}

export interface CadBlock {
  name: string;
  basePoint: SemioPoint2;
  entities: CadEntityRecord[];
}

export interface SemioCadSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ layers: CadLayer[];
  /** @state artifact */ blocks: CadBlock[];
  /** @state artifact */ entities: CadEntityRecord[];
}
