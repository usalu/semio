/** 🧊️ Artifact facade for the complete persisted Puzzle3d model. */
export * from "./📸️snapshot/🟦️.ts";
export type {Puzzle3dSnapshot as Puzzle3dArtifact} from "./📸️snapshot/🟦️.ts";
export {parsePuzzle3dSnapshot as parsePuzzle3dArtifact} from "./📸️snapshot/🟦️.ts";

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dArtifactGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dArtifactGuardRefusal(at, why);
};

type puzzlePuzzle3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle3dArtifactGuardReject(at, "value is not an object");
export const puzzlePuzzle3dArtifactGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) puzzlePuzzle3dArtifactGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const puzzlePuzzle3dArtifactGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dArtifactGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dArtifactGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle3dArtifactGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle3dArtifactGuardNumber(value, at, bounds) : puzzlePuzzle3dArtifactGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface Puzzle3dAttribute {
  readonly id?: string;
  readonly key?: string;
  readonly value?: string;
  readonly definition?: string | null;
}

export function parsePuzzle3dAttribute(value: unknown, at = "$"): Puzzle3dAttribute {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "key", "value", "definition"]);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    key: row["key"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["key"], `${at}.key`),
    value: row["value"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["value"], `${at}.value`),
    definition: row["definition"] === undefined ? undefined : (row["definition"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["definition"], `${at}.definition`)),
  };
}

export interface Puzzle3dAuthor {
  readonly id?: string;
  readonly name?: string;
  readonly email?: string;
  readonly role?: string | null;
  readonly rank?: number | null;
}

export function parsePuzzle3dAuthor(value: unknown, at = "$"): Puzzle3dAuthor {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "name", "email", "role", "rank"]);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    email: row["email"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["email"], `${at}.email`),
    role: row["role"] === undefined ? undefined : (row["role"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["role"], `${at}.role`)),
    rank: row["rank"] === undefined ? undefined : (row["rank"] === null ? null : puzzlePuzzle3dArtifactGuardInteger(row["rank"], `${at}.rank`, {"minimum": -2147483648, "maximum": 2147483647})),
  };
}

export interface Puzzle3dCatalogAttractionKind {
  readonly id: string;
  readonly label?: string;
  readonly name?: string;
}

export function parsePuzzle3dCatalogAttractionKind(value: unknown, at = "$"): Puzzle3dCatalogAttractionKind {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "label", "name"]);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
  };
}

export interface Puzzle3dCatalogCableKind {
  readonly id: string;
  readonly label?: string;
  readonly name?: string;
  readonly defaultAttractionKind?: string;
}

export function parsePuzzle3dCatalogCableKind(value: unknown, at = "$"): Puzzle3dCatalogCableKind {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "label", "name", "defaultAttractionKind"]);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    defaultAttractionKind: row["defaultAttractionKind"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["defaultAttractionKind"], `${at}.defaultAttractionKind`),
  };
}

export interface Puzzle3dCatalogVortexKind {
  readonly id: string;
  readonly code?: string | null;
  readonly label?: string | null;
  readonly order?: number | null;
  readonly compatibleWith?: readonly string[];
  readonly description?: string;
  readonly icon?: string;
  readonly color?: string;
  readonly defaultCableKind?: string;
}

export function parsePuzzle3dCatalogVortexKind(value: unknown, at = "$"): Puzzle3dCatalogVortexKind {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "code", "label", "order", "compatibleWith", "description", "icon", "color", "defaultCableKind"]);
  return {
    id: puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    code: row["code"] === undefined ? undefined : (row["code"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["code"], `${at}.code`)),
    label: row["label"] === undefined ? undefined : (row["label"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["label"], `${at}.label`)),
    order: row["order"] === undefined ? undefined : (row["order"] === null ? null : puzzlePuzzle3dArtifactGuardInteger(row["order"], `${at}.order`, {"minimum": -2147483648, "maximum": 2147483647})),
    compatibleWith: row["compatibleWith"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["compatibleWith"], `${at}.compatibleWith`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.compatibleWith[${index}]`)),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["icon"], `${at}.icon`),
    color: row["color"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["color"], `${at}.color`),
    defaultCableKind: row["defaultCableKind"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["defaultCableKind"], `${at}.defaultCableKind`),
  };
}

export interface Puzzle3dKindCompatibility {
  readonly source: string;
  readonly target: string;
  readonly bidirectional?: boolean;
  readonly important?: boolean;
  readonly specificity?: "general" | "object" | "attraction" | "vortex" | "cable";
}

export function parsePuzzle3dKindCompatibility(value: unknown, at = "$"): Puzzle3dKindCompatibility {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["source", "target", "bidirectional", "important", "specificity"]);
  return {
    source: puzzlePuzzle3dArtifactGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle3dArtifactGuardString(row["target"], `${at}.target`),
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardMember(row["specificity"], `${at}.specificity`, ["general", "object", "attraction", "vortex", "cable"] as const),
  };
}

export interface Puzzle3dReferenceSource {
  readonly url?: string;
  readonly mediaKind?: string | null;
}

export function parsePuzzle3dReferenceSource(value: unknown, at = "$"): Puzzle3dReferenceSource {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["url", "mediaKind"]);
  return {
    url: row["url"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["url"], `${at}.url`),
    mediaKind: row["mediaKind"] === undefined ? undefined : (row["mediaKind"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["mediaKind"], `${at}.mediaKind`)),
  };
}

export interface Puzzle3dRepresentation {
  readonly id?: string;
  readonly name?: string;
  readonly url?: string;
  readonly mime?: string;
  readonly tags?: readonly string[];
  readonly lod?: string | null;
  readonly description?: string;
}

export function parsePuzzle3dRepresentation(value: unknown, at = "$"): Puzzle3dRepresentation {
  const row = puzzlePuzzle3dArtifactGuardClosed(puzzlePuzzle3dArtifactGuardObject(value, at), at, ["id", "name", "url", "mime", "tags", "lod", "description"]);
  return {
    id: row["id"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["name"], `${at}.name`),
    url: row["url"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["url"], `${at}.url`),
    mime: row["mime"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["mime"], `${at}.mime`),
    tags: row["tags"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => puzzlePuzzle3dArtifactGuardString(item, `${at}.tags[${index}]`)),
    lod: row["lod"] === undefined ? undefined : (row["lod"] === null ? null : puzzlePuzzle3dArtifactGuardString(row["lod"], `${at}.lod`)),
    description: row["description"] === undefined ? undefined : puzzlePuzzle3dArtifactGuardString(row["description"], `${at}.description`),
  };
}
