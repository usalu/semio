/** 🧬️ Jack diff schema — sparse field delta. */

export interface JackDiff {
  /** @state artifact */
  artifact?: JackArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  name?: string;
  /** @state artifact */
  manifestId?: string | null;
  /** @state artifact */
  manifest?: Manifest;
  /** @state artifact */
  camera?: Camera;
  /** @state artifact */
  nodes?: JackNodesDelta;
  /** @state artifact */
  edges?: JackEdgesDelta;
  /** @state artifact */
  rootNodeId?: string | null;
  /** @state config */
  jackQuery?: string;
  /** @state config */
  lodModeByWindow?: Record<string, string | null>;
  /** @state config */
  viewportCamera?: Camera;
  /** @state config */
  jackResultJson?: string;
  /** @state config */
  editorSelection?: JackEditorSelection | null;
}

export interface JackNodesDelta {
  added: Node[];
  removed: string[];
  patched: JackNodePatchEntry[];
  reordered?: string[];
}

export interface JackNodePatchEntry {
  id: string;
  patch: JackNodePatch;
}

export interface JackNodePatch {
  name?: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

export interface JackEdgesDelta {
  added: Edge[];
  removed: string[];
  patched: JackEdgePatchEntry[];
  reordered?: string[];
}

export interface JackEdgePatchEntry {
  id: string;
  patch: JackEdgePatch;
}

export interface JackEdgePatch {
  key?: string;
  valueJson?: string | null;
}

export interface JackArtifact {
  schema: string;
  name: string;
  nodes: Node[];
  edges: Edge[];
}

export interface JackEditorSelection {
  start: number;
  end: number;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}

export interface Node {
  id: string;
  kind: string;
  name: string;
}

export interface Edge {
  id: string;
  kind: string;
  source: string;
  target: string;
}

export interface Manifest {
  nodeKinds: { name: string }[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackDiffGuardReject = (at: string, why: string): never => {
  throw new trinityJackDiffGuardRefusal(at, why);
};

type trinityJackDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackDiffGuardReject(at, "value is not an object");
export const trinityJackDiffGuardArray = (value: unknown, at: string, bounds: trinityJackDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackDiffGuardString = (value: unknown, at: string, bounds: trinityJackDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackDiffGuardReject(at, "value is not a boolean"));
export const trinityJackDiffGuardNumber = (value: unknown, at: string, bounds: trinityJackDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackDiffGuardInteger = (value: unknown, at: string, bounds: trinityJackDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackDiffGuardNumber(value, at, bounds) : trinityJackDiffGuardReject(at, "value is not an integer");
export const trinityJackDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackNodesDelta(value: unknown, at = "$"): JackNodesDelta {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    added: trinityJackDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseNode(item, `${at}.added[${index}]`)),
    removed: trinityJackDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => trinityJackDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: trinityJackDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseJackNodePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : trinityJackDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => trinityJackDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseJackNodePatchEntry(value: unknown, at = "$"): JackNodePatchEntry {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    id: trinityJackDiffGuardString(row["id"], `${at}.id`),
    patch: parseJackNodePatch(row["patch"], `${at}.patch`),
  };
}

export function parseJackNodePatch(value: unknown, at = "$"): JackNodePatch {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : trinityJackDiffGuardString(row["name"], `${at}.name`),
    x: row["x"] === undefined ? undefined : trinityJackDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : trinityJackDiffGuardNumber(row["y"], `${at}.y`),
    width: row["width"] === undefined ? undefined : trinityJackDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : trinityJackDiffGuardNumber(row["height"], `${at}.height`),
  };
}

export function parseJackEdgesDelta(value: unknown, at = "$"): JackEdgesDelta {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    added: trinityJackDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseEdge(item, `${at}.added[${index}]`)),
    removed: trinityJackDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => trinityJackDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: trinityJackDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseJackEdgePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : trinityJackDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => trinityJackDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseJackEdgePatchEntry(value: unknown, at = "$"): JackEdgePatchEntry {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    id: trinityJackDiffGuardString(row["id"], `${at}.id`),
    patch: parseJackEdgePatch(row["patch"], `${at}.patch`),
  };
}

export function parseNode(value: unknown, at = "$"): Node {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    id: trinityJackDiffGuardString(row["id"], `${at}.id`),
    kind: trinityJackDiffGuardString(row["kind"], `${at}.kind`),
    name: trinityJackDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseEdge(value: unknown, at = "$"): Edge {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    id: trinityJackDiffGuardString(row["id"], `${at}.id`),
    kind: trinityJackDiffGuardString(row["kind"], `${at}.kind`),
    source: trinityJackDiffGuardString(row["source"], `${at}.source`),
    target: trinityJackDiffGuardString(row["target"], `${at}.target`),
  };
}

export function parseManifest(value: unknown, at = "$"): Manifest {
  const row = trinityJackDiffGuardObject(value, at);
  return {
    nodeKinds: trinityJackDiffGuardArray(row["nodeKinds"], `${at}.nodeKinds`).map((item, index) => trinityJackDiffGuardObject(item, `${at}.nodeKinds[${index}]`)),
  };
}
