export type BoardValue = null | boolean | number | bigint | string | Uint8Array | BoardValue[] | BoardRecord;
export interface BoardRecord { [key:string]: BoardValue }
export interface CameraDescriptor {
  x: number;
  y: number;
  zoom: number;
}

export interface NodeDescriptor {
  id: string;
  x: number;
  y: number;
  draggable?: boolean | null;
  selected?: boolean | null;
  style?: string | null;
  text?: string | null;
  iconKind?: string | null;
  nodeKind?: string | null;
  userData?: BoardValue | null;
  visible?: boolean | null;
  locked?: boolean | null;
  root?: boolean | null;
  shape?: string | null;
  radius?: number | null;
  width?: number | null;
  height?: number | null;
  scale?: number | null;
}
