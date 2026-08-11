/** 🧬️ ObjMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
type S = import('../📸️snapshot/🟦️component.ts').ObjSnapshot;
type Vertex = import('../📸️snapshot/🟦️component.ts').ObjVertex;
type TexCoord = import('../📸️snapshot/🟦️component.ts').ObjTexCoord;
type Normal = import('../📸️snapshot/🟦️component.ts').ObjNormal;
type Face = import('../📸️snapshot/🟦️component.ts').ObjFace;
type UsemtlRange = import('../📸️snapshot/🟦️component.ts').ObjUsemtlRange;
type SmoothingRange = import('../📸️snapshot/🟦️component.ts').ObjSmoothingRange;
type UnknownStatement = import('../📸️snapshot/🟦️component.ts').ObjUnknownStatement;

export type ObjMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: S }
  | { mutation: 'insertVertex'; index: number; vertex: Vertex }
  | { mutation: 'removeVertex'; index: number }
  | { mutation: 'setVertex'; index: number; vertex: Vertex }
  | { mutation: 'insertTexCoord'; index: number; texcoord: TexCoord }
  | { mutation: 'removeTexCoord'; index: number }
  | { mutation: 'setTexCoord'; index: number; texcoord: TexCoord }
  | { mutation: 'insertNormal'; index: number; normal: Normal }
  | { mutation: 'removeNormal'; index: number }
  | { mutation: 'setNormal'; index: number; normal: Normal }
  | { mutation: 'insertFace'; index: number; face: Face }
  | { mutation: 'removeFace'; index: number }
  | { mutation: 'setFace'; index: number; face: Face }
  | { mutation: 'setGroup'; name: string; faces: number[] }
  | { mutation: 'removeGroup'; name: string }
  | { mutation: 'setObject'; name: string; faces: number[] }
  | { mutation: 'removeObject'; name: string }
  | { mutation: 'setMtllib'; mtllib?: string }
  | { mutation: 'setUsemtl'; usemtl: UsemtlRange[] }
  | { mutation: 'setSmoothingGroups'; smoothingGroups: SmoothingRange[] }
  | { mutation: 'setUnknownStatements'; unknownStatements: UnknownStatement[] };
