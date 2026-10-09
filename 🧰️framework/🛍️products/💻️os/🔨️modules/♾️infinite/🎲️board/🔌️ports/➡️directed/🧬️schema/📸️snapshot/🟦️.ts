export type BoardNodeShape="circle"|"rectangle";
export interface BoardHandleSnapshot {
  id: string;
  angle?: number | null;
  radius?: number | null;
  scale?: number | null;
  handleKind?: string | null;
  color?: string | null;
  iconKind?: string | null;
  text?: string | null;
  label?: string | null;
  selected?: boolean | null;
  style?: string | null;
  userData?: unknown;
  hidden?: boolean | null;
  visible?: boolean | null;
  locked?: boolean | null;
}
export interface BoardNodeSnapshot {
  id: string;
  x?: number | null;
  y?: number | null;
  shape?: BoardNodeShape | null;
  radius?: number | null;
  width?: number | null;
  height?: number | null;
  scale?: number | null;
  text?: string | null;
  label?: string | null;
  iconKind?: string | null;
  nodeKind?: string | null;
  draggable?: boolean | null;
  selected?: boolean | null;
  style?: string | null;
  userData?: unknown;
  root?: boolean | null;
  hidden?: boolean | null;
  visible?: boolean | null;
  locked?: boolean | null;
  handles?: Array<BoardHandleSnapshot> | null;
}
export interface BoardEdgeSnapshot {
  id: string;
  source?: string | null;
  target?: string | null;
  edgeKind?: string | null;
  sourceTip?: string | null;
  targetTip?: string | null;
  label?: string | null;
  text?: string | null;
  selected?: boolean | null;
  style?: string | null;
  userData?: unknown;
  hidden?: boolean | null;
  visible?: boolean | null;
  locked?: boolean | null;
}
export interface BoardRegionSnapshot {
  id: string;
  x?: number | null;
  y?: number | null;
  width?: number | null;
  height?: number | null;
  label?: string | null;
  hidden?: boolean | null;
  locked?: boolean | null;
  selected?: boolean | null;
}
export interface BoardSnapshot {schema:"board.ports.directed.v1"|"board.normal.undirected.v1"|"trinity.graph"|"reasoning.wires.identity.snapshot";camera?:{x:number;y:number;zoom:number} | null;nodes:BoardNodeSnapshot[];edges:BoardEdgeSnapshot[];targetRegions?:BoardRegionSnapshot[];meta?:unknown}
