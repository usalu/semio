/** 🧬️ SemioObjectArtifact schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}
export interface SemioObjectArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ transform: SemioTransform;
  /** @state artifact @child kind=s.stdio.semio.brep */ brep?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.mesh */ mesh?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectArtifactGuardRefusal(at, why);
};

type stdioSemioV1ObjectArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectArtifactGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectArtifactGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectArtifactGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectArtifactGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectArtifactGuardNumber(value, at, bounds) : stdioSemioV1ObjectArtifactGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectArtifact(value: unknown, at = "$"): SemioObjectArtifact {
  const row = stdioSemioV1ObjectArtifactGuardObject(value, at);
  return {
    schema: stdioSemioV1ObjectArtifactGuardString(row["schema"], `${at}.schema`),
    transform: stdioSemioV1ObjectArtifactGuardObject(row["transform"], `${at}.transform`),
    brep: row["brep"] === undefined ? undefined : stdioSemioV1ObjectArtifactGuardObject(row["brep"], `${at}.brep`),
    mesh: row["mesh"] === undefined ? undefined : stdioSemioV1ObjectArtifactGuardObject(row["mesh"], `${at}.mesh`),
    properties: row["properties"] === undefined ? undefined : stdioSemioV1ObjectArtifactGuardObject(row["properties"], `${at}.properties`),
  };
}
