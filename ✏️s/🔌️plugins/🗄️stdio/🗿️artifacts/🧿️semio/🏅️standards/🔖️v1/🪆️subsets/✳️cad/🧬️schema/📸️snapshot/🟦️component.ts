/** 🧬️ SemioCadSnapshot schema — real facet mirror of `🦀️component.rs` (source of truth). */
export interface SemioPoint2 {
  x: number;
  y: number;
}

export type CadEntity =
  | { kind: "line"; a: SemioPoint2; b: SemioPoint2 }
  | { kind: "arc"; center: SemioPoint2; radius: number; startAngle: number; endAngle: number }
  | { kind: "circle"; center: SemioPoint2; radius: number }
  | { kind: "ellipse"; center: SemioPoint2; majorAxisEnd: SemioPoint2; ratio: number; startParam: number; endParam: number }
  | { kind: "polyline"; vertices: SemioPoint2[]; closed: boolean }
  | { kind: "text"; position: SemioPoint2; height: number; rotation: number; content: string }
  | { kind: "insert"; blockName: string; insertionPoint: SemioPoint2; scale: SemioPoint2; rotation: number }
  | { kind: "solid"; p1: SemioPoint2; p2: SemioPoint2; p3: SemioPoint2; p4: SemioPoint2 }
  | { kind: "dimension"; defPoint: SemioPoint2; textPosition: SemioPoint2; measurement: number; text: string };

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
  /** @state persistent */ schema: string;
  /** @state persistent */ layers: CadLayer[];
  /** @state persistent */ blocks: CadBlock[];
  /** @state persistent */ entities: CadEntityRecord[];
}
