/** 🔺️ SemioWorkflowDiff schema (binary facet mirror) — real, matches the text facet's shape; the
 * wire ENCODING is the text line's UTF-8 bytes verbatim (see 🔺️diff/💾️binary/🥋️component.ksy). */
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
export interface WorkflowParam {
  key: string;
  value: string;
}
export interface WorkflowNode {
  id: string;
  kind: string;
  label: string;
  params: WorkflowParam[];
  position: SemioPoint2;
}
export interface WorkflowEdge {
  id: string;
  from: PortRef;
  to: PortRef;
  kind: string;
}
export interface WorkflowParamDiff {
  value?: string;
}
export type WorkflowParamsDiff = NamedTripleDiff<string, WorkflowParamDiff, WorkflowParam>;
export interface WorkflowNodeDiff {
  kind?: string;
  label?: string;
  params?: WorkflowParamsDiff;
  position?: SemioPoint2;
}
export type WorkflowNodesDiff = NamedTripleDiff<string, WorkflowNodeDiff, WorkflowNode>;
export interface WorkflowEdgeDiff {
  from?: PortRef;
  to?: PortRef;
  kind?: string;
}
export type WorkflowEdgesDiff = NamedTripleDiff<string, WorkflowEdgeDiff, WorkflowEdge>;
export interface SemioWorkflowDiff {
  /** @state persistent */ nodes?: WorkflowNodesDiff;
  /** @state persistent */ edges?: WorkflowEdgesDiff;
}
