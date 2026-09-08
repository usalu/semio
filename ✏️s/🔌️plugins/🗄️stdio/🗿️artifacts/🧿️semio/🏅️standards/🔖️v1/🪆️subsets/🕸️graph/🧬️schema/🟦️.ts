/** 🧬️ SemioGraphArtifact schema — full artifact state, mirrors `SemioGraphSnapshot` field for
 * field. */
export interface SemioGraphArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: import("./📸️snapshot/🟦️.ts").SemioGraphNode[];
  /** @state artifact */ edges: import("./📸️snapshot/🟦️.ts").SemioGraphEdge[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphArtifactGuardRefusal(at, why);
};

type stdioSemioV1GraphArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphArtifactGuardReject(at, "value is not an object");
export const stdioSemioV1GraphArtifactGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphArtifactGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphArtifactGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphArtifactGuardNumber(value, at, bounds) : stdioSemioV1GraphArtifactGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphArtifact(value: unknown, at = "$"): SemioGraphArtifact {
  const row = stdioSemioV1GraphArtifactGuardObject(value, at);
  return {
    schema: stdioSemioV1GraphArtifactGuardString(row["schema"], `${at}.schema`),
    nodes: stdioSemioV1GraphArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => stdioSemioV1GraphArtifactGuardObject(item, `${at}.nodes[${index}]`)),
    edges: stdioSemioV1GraphArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => stdioSemioV1GraphArtifactGuardObject(item, `${at}.edges[${index}]`)),
  };
}
