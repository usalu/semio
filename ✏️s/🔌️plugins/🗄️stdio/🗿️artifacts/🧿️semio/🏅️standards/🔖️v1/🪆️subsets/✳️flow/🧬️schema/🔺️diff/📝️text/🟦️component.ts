/** 🔺️ SemioFlowDiff schema (TEXT representation facet mirror) — real, matches the facet root's
 * shape; the wire ENCODING is `nodes=[...];[...];[...] edges=...` (see 📝️text/📖️component.grammar.semio). */
export interface NamedModified<K, D> {
  key: K;
  diff: D;
}
export interface NamedTripleDiff<K, D, T> {
  removed: K[];
  modified: NamedModified<K, D>[];
  added: T[];
}
export interface SemioPoint2 {
  x: number;
  y: number;
}
export interface PortRef {
  node: string;
  port: string;
}
export interface FlowParam {
  key: string;
  value: string;
}
export interface FlowNode {
  id: string;
  kind: string;
  label: string;
  params: FlowParam[];
  position: SemioPoint2;
}
export interface FlowEdge {
  id: string;
  from: PortRef;
  to: PortRef;
  kind: string;
}
export interface FlowParamDiff {
  value?: string;
}
export type FlowParamsDiff = NamedTripleDiff<string, FlowParamDiff, FlowParam>;
export interface FlowNodeDiff {
  kind?: string;
  label?: string;
  params?: FlowParamsDiff;
  position?: SemioPoint2;
}
export type FlowNodesDiff = NamedTripleDiff<string, FlowNodeDiff, FlowNode>;
export interface FlowEdgeDiff {
  from?: PortRef;
  to?: PortRef;
  kind?: string;
}
export type FlowEdgesDiff = NamedTripleDiff<string, FlowEdgeDiff, FlowEdge>;
export interface SemioFlowDiff {
  /** @state persistent */ nodes?: FlowNodesDiff;
  /** @state persistent */ edges?: FlowEdgesDiff;
}
