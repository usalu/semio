/** 🧬️ Jack artifact schema — every field with its state class. */

export interface JackArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  name: string;
  /** @state artifact */
  manifestId?: string;
  /** @state artifact */
  manifest: Manifest;
  /** @state artifact */
  camera: Camera;
  /** @state artifact */
  nodes: Node[];
  /** @state artifact */
  edges: Edge[];
  /** @state artifact */
  rootNodeId?: string;
  /** @state config */
  jackQuery: string;
  /** @state config */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  viewportCamera: Camera;
  /** @state config */
  jackResultJson: string;
  /** @state config */
  editorSelection?: JackEditorSelection;
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
  x: number;
  y: number;
  width: number;
  height: number;
  ports: Port[];
}

export interface Port {
  id: string;
  kind: string;
  direction: string;
}

export interface Edge {
  id: string;
  kind: string;
  source: string;
  target: string;
}

export interface Manifest {
  nodeKinds: ManifestKind[];
  edgeKinds: ManifestKind[];
  portKinds: ManifestPortKind[];
}

export interface ManifestKind {
  name: string;
}

export interface ManifestPortKind {
  name: string;
  direction: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class trinityJackArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const trinityJackArtifactGuardReject = (at: string, why: string): never => {
  throw new trinityJackArtifactGuardRefusal(at, why);
};

type trinityJackArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type trinityJackArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type trinityJackArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const trinityJackArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : trinityJackArtifactGuardReject(at, "value is not an object");
export const trinityJackArtifactGuardArray = (value: unknown, at: string, bounds: trinityJackArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return trinityJackArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) trinityJackArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) trinityJackArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const trinityJackArtifactGuardString = (value: unknown, at: string, bounds: trinityJackArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return trinityJackArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) trinityJackArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) trinityJackArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) trinityJackArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const trinityJackArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : trinityJackArtifactGuardReject(at, "value is not a boolean"));
export const trinityJackArtifactGuardNumber = (value: unknown, at: string, bounds: trinityJackArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return trinityJackArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) trinityJackArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) trinityJackArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const trinityJackArtifactGuardInteger = (value: unknown, at: string, bounds: trinityJackArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? trinityJackArtifactGuardNumber(value, at, bounds) : trinityJackArtifactGuardReject(at, "value is not an integer");
export const trinityJackArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : trinityJackArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const trinityJackArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : trinityJackArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJackArtifact(value: unknown, at = "$"): JackArtifact {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    schema: trinityJackArtifactGuardString(row["schema"], `${at}.schema`),
    name: trinityJackArtifactGuardString(row["name"], `${at}.name`),
    manifestId: row["manifestId"] === undefined ? undefined : trinityJackArtifactGuardString(row["manifestId"], `${at}.manifestId`),
    manifest: parseManifest(row["manifest"], `${at}.manifest`),
    camera: parseCamera(row["camera"], `${at}.camera`),
    nodes: trinityJackArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseNode(item, `${at}.nodes[${index}]`)),
    edges: trinityJackArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parseEdge(item, `${at}.edges[${index}]`)),
    rootNodeId: row["rootNodeId"] === undefined ? undefined : trinityJackArtifactGuardString(row["rootNodeId"], `${at}.rootNodeId`),
    jackQuery: trinityJackArtifactGuardString(row["jackQuery"], `${at}.jackQuery`),
    lodModeByWindow: trinityJackArtifactGuardObject(row["lodModeByWindow"], `${at}.lodModeByWindow`),
    viewportCamera: parseCamera(row["viewportCamera"], `${at}.viewportCamera`),
    jackResultJson: trinityJackArtifactGuardString(row["jackResultJson"], `${at}.jackResultJson`),
    editorSelection: row["editorSelection"] === undefined ? undefined : parseJackEditorSelection(row["editorSelection"], `${at}.editorSelection`),
  };
}

export function parseCamera(value: unknown, at = "$"): Camera {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    x: trinityJackArtifactGuardNumber(row["x"], `${at}.x`),
    y: trinityJackArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: trinityJackArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseNode(value: unknown, at = "$"): Node {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    id: trinityJackArtifactGuardString(row["id"], `${at}.id`),
    kind: trinityJackArtifactGuardString(row["kind"], `${at}.kind`),
    name: trinityJackArtifactGuardString(row["name"], `${at}.name`),
    x: trinityJackArtifactGuardNumber(row["x"], `${at}.x`),
    y: trinityJackArtifactGuardNumber(row["y"], `${at}.y`),
    width: trinityJackArtifactGuardNumber(row["width"], `${at}.width`),
    height: trinityJackArtifactGuardNumber(row["height"], `${at}.height`),
    properties: trinityJackArtifactGuardObject(row["properties"], `${at}.properties`),
    ports: trinityJackArtifactGuardArray(row["ports"], `${at}.ports`).map((item, index) => parsePort(item, `${at}.ports[${index}]`)),
  };
}

export function parsePort(value: unknown, at = "$"): Port {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    id: trinityJackArtifactGuardString(row["id"], `${at}.id`),
    kind: trinityJackArtifactGuardString(row["kind"], `${at}.kind`),
    direction: trinityJackArtifactGuardString(row["direction"], `${at}.direction`),
    properties: trinityJackArtifactGuardObject(row["properties"], `${at}.properties`),
  };
}

export function parseEdge(value: unknown, at = "$"): Edge {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    id: trinityJackArtifactGuardString(row["id"], `${at}.id`),
    kind: trinityJackArtifactGuardString(row["kind"], `${at}.kind`),
    source: trinityJackArtifactGuardString(row["source"], `${at}.source`),
    target: trinityJackArtifactGuardString(row["target"], `${at}.target`),
    properties: trinityJackArtifactGuardObject(row["properties"], `${at}.properties`),
  };
}

export function parseManifest(value: unknown, at = "$"): Manifest {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    nodeKinds: trinityJackArtifactGuardArray(row["nodeKinds"], `${at}.nodeKinds`).map((item, index) => parseManifestKind(item, `${at}.nodeKinds[${index}]`)),
    edgeKinds: trinityJackArtifactGuardArray(row["edgeKinds"], `${at}.edgeKinds`).map((item, index) => parseManifestKind(item, `${at}.edgeKinds[${index}]`)),
    portKinds: trinityJackArtifactGuardArray(row["portKinds"], `${at}.portKinds`).map((item, index) => parseManifestPortKind(item, `${at}.portKinds[${index}]`)),
  };
}

export function parseManifestKind(value: unknown, at = "$"): ManifestKind {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    name: trinityJackArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseManifestPortKind(value: unknown, at = "$"): ManifestPortKind {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    name: trinityJackArtifactGuardString(row["name"], `${at}.name`),
    direction: trinityJackArtifactGuardString(row["direction"], `${at}.direction`),
  };
}

export function parseJackEditorSelection(value: unknown, at = "$"): JackEditorSelection {
  const row = trinityJackArtifactGuardObject(value, at);
  return {
    start: trinityJackArtifactGuardInteger(row["start"], `${at}.start`),
    end: trinityJackArtifactGuardInteger(row["end"], `${at}.end`),
  };
}
