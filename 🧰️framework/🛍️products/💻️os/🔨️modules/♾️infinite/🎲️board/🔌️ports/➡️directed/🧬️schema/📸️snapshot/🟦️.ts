export type BoardNodeShape="circle"|"rectangle";
export interface BoardHandleSnapshot {
  id: string;
  angle?: number;
  radius?: number;
  scale?: number;
  handleKind?: string;
  color?: string;
  iconKind?: string;
  text?: string;
  label?: string;
  selected?: boolean;
  style?: string;
  userData?: unknown;
  hidden?: boolean;
  visible?: boolean;
  locked?: boolean;
}
export interface BoardNodeSnapshot {
  id: string;
  x?: number;
  y?: number;
  shape?: BoardNodeShape;
  radius?: number;
  width?: number;
  height?: number;
  scale?: number;
  text?: string;
  label?: string;
  iconKind?: string;
  nodeKind?: string;
  draggable?: boolean;
  selected?: boolean;
  style?: string;
  userData?: unknown;
  root?: boolean;
  hidden?: boolean;
  visible?: boolean;
  locked?: boolean;
  handles?: Array<BoardHandleSnapshot>;
}
export interface BoardEdgeSnapshot {
  id: string;
  source?: string;
  target?: string;
  edgeKind?: string;
  sourceTip?: string;
  targetTip?: string;
  label?: string;
  text?: string;
  selected?: boolean;
  style?: string;
  userData?: unknown;
  hidden?: boolean;
  visible?: boolean;
  locked?: boolean;
}
export interface BoardRegionSnapshot {
  id: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  label?: string;
  hidden?: boolean;
  locked?: boolean;
  selected?: boolean;
}
export interface BoardSnapshot {schema:"board.ports.directed.v1"|"board.normal.undirected.v1"|"trinity.graph"|"reasoning.wires.identity.snapshot";camera?:{x:number;y:number;zoom:number};nodes:BoardNodeSnapshot[];edges:BoardEdgeSnapshot[];targetRegions?:BoardRegionSnapshot[];meta?:unknown}
