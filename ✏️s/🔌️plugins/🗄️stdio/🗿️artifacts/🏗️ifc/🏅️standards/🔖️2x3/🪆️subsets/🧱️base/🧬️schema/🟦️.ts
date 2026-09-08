/** 🧬️ Ifc2x3Artifact schema. */
export interface Part21Decimal { negative: boolean; coefficient: string; scale: number; exponent?: number; }
export type Part21Value =
  | { kind: 'ref'; value: number }
  | { kind: 'str'; value: string }
  | { kind: 'enum'; value: string }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: Part21Decimal }
  | { kind: 'list'; values: Part21Value[] }
  | { kind: 'typed'; typeName: string; values: Part21Value[] }
  | { kind: 'unset' }
  | { kind: 'derived' };
export interface Part21Entity { typeName: string; arguments: Part21Value[]; }
export interface Part21Instance { id: number; entities: Part21Entity[]; }
export interface Part21Header { fileDescription: Part21Value[]; fileName: Part21Value[]; fileSchema: Part21Value[]; }
export interface Part21Document { header: Part21Header; instances: Part21Instance[]; }
export interface Ifc2x3EdmPreamble {
  producer: string; module: string; creationDate: string; host: string; database: string; databaseVersion: string;
  databaseCreationDate: string; schema: string; model: string; modelCreationDate: string; headerModel: string;
  headerModelCreationDate: string; user: string; group: string; license: string; options: string;
}
export interface Ifc2x3Artifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: Part21Document;
  /** @state artifact */ edmPreamble?: Ifc2x3EdmPreamble;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc2x3BaseArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc2x3BaseArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioIfc2x3BaseArtifactGuardRefusal(at, why);
};

type stdioIfc2x3BaseArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc2x3BaseArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc2x3BaseArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc2x3BaseArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc2x3BaseArtifactGuardReject(at, "value is not an object");
export const stdioIfc2x3BaseArtifactGuardArray = (value: unknown, at: string, bounds: stdioIfc2x3BaseArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc2x3BaseArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc2x3BaseArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc2x3BaseArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc2x3BaseArtifactGuardString = (value: unknown, at: string, bounds: stdioIfc2x3BaseArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc2x3BaseArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc2x3BaseArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc2x3BaseArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc2x3BaseArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc2x3BaseArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc2x3BaseArtifactGuardReject(at, "value is not a boolean"));
export const stdioIfc2x3BaseArtifactGuardNumber = (value: unknown, at: string, bounds: stdioIfc2x3BaseArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc2x3BaseArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc2x3BaseArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc2x3BaseArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc2x3BaseArtifactGuardInteger = (value: unknown, at: string, bounds: stdioIfc2x3BaseArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc2x3BaseArtifactGuardNumber(value, at, bounds) : stdioIfc2x3BaseArtifactGuardReject(at, "value is not an integer");
export const stdioIfc2x3BaseArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc2x3BaseArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc2x3BaseArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc2x3BaseArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfc2x3Artifact(value: unknown, at = "$"): Ifc2x3Artifact {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : parsePart21Document(row["schema"], `${at}.schema`),
    document: row["document"] === undefined ? undefined : stdioIfc2x3BaseArtifactGuardString(row["document"], `${at}.document`),
    edmPreamble: row["edmPreamble"] === undefined ? undefined : parseIfc2x3EdmPreamble(row["edmPreamble"], `${at}.edmPreamble`),
  };
}

export function parsePart21Decimal(value: unknown, at = "$"): Part21Decimal {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    negative: stdioIfc2x3BaseArtifactGuardBoolean(row["negative"], `${at}.negative`),
    coefficient: stdioIfc2x3BaseArtifactGuardString(row["coefficient"], `${at}.coefficient`),
    scale: stdioIfc2x3BaseArtifactGuardInteger(row["scale"], `${at}.scale`),
    exponent: row["exponent"] === undefined ? undefined : stdioIfc2x3BaseArtifactGuardInteger(row["exponent"], `${at}.exponent`),
  };
}

export function parsePart21Entity(value: unknown, at = "$"): Part21Entity {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    typeName: stdioIfc2x3BaseArtifactGuardString(row["typeName"], `${at}.typeName`),
    arguments: stdioIfc2x3BaseArtifactGuardArray(row["arguments"], `${at}.arguments`).map((item, index) => parsePart21Value(item, `${at}.arguments[${index}]`)),
  };
}

export function parsePart21Instance(value: unknown, at = "$"): Part21Instance {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    id: stdioIfc2x3BaseArtifactGuardInteger(row["id"], `${at}.id`),
    entities: stdioIfc2x3BaseArtifactGuardArray(row["entities"], `${at}.entities`).map((item, index) => parsePart21Entity(item, `${at}.entities[${index}]`)),
  };
}

export function parsePart21Header(value: unknown, at = "$"): Part21Header {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    fileDescription: stdioIfc2x3BaseArtifactGuardArray(row["fileDescription"], `${at}.fileDescription`).map((item, index) => parsePart21Value(item, `${at}.fileDescription[${index}]`)),
    fileName: stdioIfc2x3BaseArtifactGuardArray(row["fileName"], `${at}.fileName`).map((item, index) => parsePart21Value(item, `${at}.fileName[${index}]`)),
    fileSchema: stdioIfc2x3BaseArtifactGuardArray(row["fileSchema"], `${at}.fileSchema`).map((item, index) => parsePart21Value(item, `${at}.fileSchema[${index}]`)),
  };
}

export function parsePart21Document(value: unknown, at = "$"): Part21Document {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    header: parsePart21Header(row["header"], `${at}.header`),
    instances: stdioIfc2x3BaseArtifactGuardArray(row["instances"], `${at}.instances`).map((item, index) => parsePart21Instance(item, `${at}.instances[${index}]`)),
  };
}

export function parseIfc2x3EdmPreamble(value: unknown, at = "$"): Ifc2x3EdmPreamble {
  const row = stdioIfc2x3BaseArtifactGuardObject(value, at);
  return {
    producer: stdioIfc2x3BaseArtifactGuardString(row["producer"], `${at}.producer`),
    module: stdioIfc2x3BaseArtifactGuardString(row["module"], `${at}.module`),
    creationDate: stdioIfc2x3BaseArtifactGuardString(row["creationDate"], `${at}.creationDate`),
    host: stdioIfc2x3BaseArtifactGuardString(row["host"], `${at}.host`),
    database: stdioIfc2x3BaseArtifactGuardString(row["database"], `${at}.database`),
    databaseVersion: stdioIfc2x3BaseArtifactGuardString(row["databaseVersion"], `${at}.databaseVersion`),
    databaseCreationDate: stdioIfc2x3BaseArtifactGuardString(row["databaseCreationDate"], `${at}.databaseCreationDate`),
    schema: stdioIfc2x3BaseArtifactGuardString(row["schema"], `${at}.schema`),
    model: stdioIfc2x3BaseArtifactGuardString(row["model"], `${at}.model`),
    modelCreationDate: stdioIfc2x3BaseArtifactGuardString(row["modelCreationDate"], `${at}.modelCreationDate`),
    headerModel: stdioIfc2x3BaseArtifactGuardString(row["headerModel"], `${at}.headerModel`),
    headerModelCreationDate: stdioIfc2x3BaseArtifactGuardString(row["headerModelCreationDate"], `${at}.headerModelCreationDate`),
    user: stdioIfc2x3BaseArtifactGuardString(row["user"], `${at}.user`),
    group: stdioIfc2x3BaseArtifactGuardString(row["group"], `${at}.group`),
    license: stdioIfc2x3BaseArtifactGuardString(row["license"], `${at}.license`),
    options: stdioIfc2x3BaseArtifactGuardString(row["options"], `${at}.options`),
  };
}
