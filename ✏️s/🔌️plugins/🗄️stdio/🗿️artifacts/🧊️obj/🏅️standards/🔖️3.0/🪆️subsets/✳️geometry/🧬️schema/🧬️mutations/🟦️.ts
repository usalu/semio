/** 🧬️ ObjMutation union — mirrors 🦀️.rs's `#[serde(tag = "mutation")]` enum. */
type S = import('../📸️snapshot/🟦️.ts').ObjSnapshot;
type Vertex = import('../📸️snapshot/🟦️.ts').ObjVertex;
type TexCoord = import('../📸️snapshot/🟦️.ts').ObjTexCoord;
type Normal = import('../📸️snapshot/🟦️.ts').ObjNormal;
type Face = import('../📸️snapshot/🟦️.ts').ObjFace;
type UsemtlRange = import('../📸️snapshot/🟦️.ts').ObjUsemtlRange;
type SmoothingRange = import('../📸️snapshot/🟦️.ts').ObjSmoothingRange;
type UnknownStatement = import('../📸️snapshot/🟦️.ts').ObjUnknownStatement;

export type ObjMutation =
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
