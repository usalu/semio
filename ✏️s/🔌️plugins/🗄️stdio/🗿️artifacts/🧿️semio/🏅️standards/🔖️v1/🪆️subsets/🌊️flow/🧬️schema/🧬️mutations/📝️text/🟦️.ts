/** 🧬️ SemioFlowMutation schema (TEXT representation facet mirror) — real, matches the facet
 * root's shape; the wire ENCODING is `keyword arg=value ...` (see 📝️text/📖️.grammar.semio). */
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
export interface SemioFlowSnapshot {
  schema: string;
  nodes: FlowNode[];
  edges: FlowEdge[];
}
export type SemioFlowMutation =
  | { mutation: "setSnapshot"; snapshot: SemioFlowSnapshot }
  | { mutation: "insertNode"; node: FlowNode }
  | { mutation: "removeNode"; id: string }
  | { mutation: "setNodeKind"; id: string; kind: string }
  | { mutation: "setNodeLabel"; id: string; label: string }
  | { mutation: "setNodePosition"; id: string; position: SemioPoint2 }
  | { mutation: "setNodeParam"; id: string; key: string; value: string }
  | { mutation: "removeNodeParam"; id: string; key: string }
  | { mutation: "insertEdge"; edge: FlowEdge }
  | { mutation: "removeEdge"; id: string }
  | { mutation: "setEdgeEndpoints"; id: string; from: PortRef; to: PortRef }
  | { mutation: "setEdgeKind"; id: string; kind: string };

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1FlowMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1FlowMutationsTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1FlowMutationsTextGuardRefusal(at, why);
};

type stdioSemioV1FlowMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1FlowMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1FlowMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1FlowMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1FlowMutationsTextGuardReject(at, "value is not an object");
export const stdioSemioV1FlowMutationsTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1FlowMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1FlowMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1FlowMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1FlowMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1FlowMutationsTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1FlowMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1FlowMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1FlowMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1FlowMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1FlowMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1FlowMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1FlowMutationsTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1FlowMutationsTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1FlowMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1FlowMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1FlowMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1FlowMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1FlowMutationsTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1FlowMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1FlowMutationsTextGuardNumber(value, at, bounds) : stdioSemioV1FlowMutationsTextGuardReject(at, "value is not an integer");
export const stdioSemioV1FlowMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1FlowMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1FlowMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1FlowMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SemioFlowMutationText {
  readonly mutation: "setSnapshot" | "insertNode" | "removeNode" | "setNodeKind" | "setNodeLabel" | "setNodePosition" | "setNodeParam" | "removeNodeParam" | "insertEdge" | "removeEdge" | "setEdgeEndpoints" | "setEdgeKind";
  readonly snapshot?: SemioFlowSnapshot;
  readonly node?: FlowNode;
  readonly edge?: FlowEdge;
  readonly id?: string;
  readonly kind?: string;
  readonly label?: string;
  readonly position?: SemioPoint2;
  readonly key?: string;
  readonly value?: string;
  readonly from?: PortRef;
  readonly to?: PortRef;
}
