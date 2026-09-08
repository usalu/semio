// #region Header
/** 🌱️ Closed browser twin of the schema-owned host artifact-creation contract. */
// #endregion Header

export const SPACE_ARTIFACT_CREATION_MAX_BYTES = 4096;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES = 65_536;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS = 64;
export const SPACE_ARTIFACT_CREATE_SCHEMA_V1 = "semio.hub.space-artifact-create/v1";
export const SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1 = "semio.hub.space-artifact-creation-catalog/v1";
export const SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1 = "semio.hub.space-artifact-creation-status/v1";

export type SpaceArtifactCreateV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATE_SCHEMA_V1;
  requestId: string;
  kindId: string;
  name: string;
}>;

export type SpaceArtifactCreationPhaseV1 = "accepted" | "preparing" | "ready" | "indeterminate" | "failed" | "cancelled";

export type SpaceArtifactCreationDialectV1 = Readonly<{
  artifactKind: string;
  standard: string;
  subset: string;
}>;

export type SpaceArtifactCreationReadyV1 = Readonly<{
  documentId: string;
  kindId: string;
  artifactSchema: string;
  parentDialect: SpaceArtifactCreationDialectV1;
}>;

export type SpaceArtifactCreationStatusV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1;
  requestId: string;
  spaceId: string;
  phase: SpaceArtifactCreationPhaseV1;
  ready?: SpaceArtifactCreationReadyV1;
}>;

export type SpaceArtifactCreationKindV1 = Readonly<{
  kindId: string;
  schema: string;
  dialect: SpaceArtifactCreationDialectV1;
  label: Readonly<{ en: string; de: string }>;
}>;

export type SpaceArtifactCreationCatalogV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;

function record(value: unknown): Readonly<Record<string, unknown>> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Readonly<Record<string, unknown>>) : null;
}

function exactFields(value: Readonly<Record<string, unknown>>, fields: readonly string[]): boolean {
  return Object.keys(value).sort().join(",") === [...fields].sort().join(",");
}

function identity(value: unknown): string | null {
  return typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value) ? value : null;
}

function requestId(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{32}$)[0-9a-f]{32}$/u.test(value) ? value : null;
}

function digest(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value) ? value : null;
}

function name(value: unknown): string | null {
  return typeof value === "string" && value.length > 0 && [...value].length <= 128 && !value.startsWith(" ") && !value.endsWith(" ") && !/[\u0000-\u001f\u007f-\u009f]/u.test(value) && !/[\ud800-\udfff]/u.test(value) ? value : null;
}

function ready(value: unknown): SpaceArtifactCreationReadyV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["documentId", "kindId", "artifactSchema", "parentDialect"])) return null;
  const dialect = record(row.parentDialect);
  if (dialect === null || !exactFields(dialect, ["artifactKind", "standard", "subset"])) return null;
  const documentId = typeof row.documentId === "string" && /^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(row.documentId) ? row.documentId : null,
    kindId = identity(row.kindId),
    artifactSchema = identity(row.artifactSchema),
    artifactKind = identity(dialect.artifactKind),
    standard = identity(dialect.standard),
    subset = identity(dialect.subset);
  return documentId !== null && kindId !== null && artifactSchema !== null && artifactKind === kindId && standard !== null && subset !== null
    ? { documentId, kindId, artifactSchema, parentDialect: { artifactKind, standard, subset } }
    : null;
}

function creationKind(value: unknown): SpaceArtifactCreationKindV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["kindId", "schema", "dialect", "label"])) return null;
  const dialect = record(row.dialect),
    labels = record(row.label);
  if (dialect === null || !exactFields(dialect, ["artifactKind", "standard", "subset"]) || labels === null || !exactFields(labels, ["en", "de"])) return null;
  const kindId = identity(row.kindId),
    schema = identity(row.schema),
    artifactKind = identity(dialect.artifactKind),
    standard = identity(dialect.standard),
    subset = identity(dialect.subset),
    en = name(labels.en),
    de = name(labels.de);
  return kindId !== null && schema !== null && artifactKind === kindId && standard !== null && subset !== null && en !== null && de !== null
    ? { kindId, schema, dialect: { artifactKind, standard, subset }, label: { en, de } }
    : null;
}

/** 📥️ Seals the only client-supplied creation fields. */
export function sealSpaceArtifactCreateV1(value: Readonly<{ requestId: string; kindId: string; name: string }>): SpaceArtifactCreateV1 {
  const parsedRequestId = requestId(value.requestId),
    parsedKindId = identity(value.kindId),
    parsedName = name(value.name);
  if (parsedRequestId === null || parsedKindId === null || parsedName === null) throw new Error("space artifact creation: invalid intent");
  return { schema: SPACE_ARTIFACT_CREATE_SCHEMA_V1, requestId: parsedRequestId, kindId: parsedKindId, name: parsedName };
}

/** 🧾️ Parses a canonical client request without admitting extra authority fields. */
export function parseSpaceArtifactCreateJsonV1(source: string): SpaceArtifactCreateV1 {
  if (new TextEncoder().encode(source).byteLength > SPACE_ARTIFACT_CREATION_MAX_BYTES) throw new Error("space artifact creation: capacity");
  const row = record(JSON.parse(source));
  if (row === null || !exactFields(row, ["schema", "requestId", "kindId", "name"]) || row.schema !== SPACE_ARTIFACT_CREATE_SCHEMA_V1) throw new Error("space artifact creation: invalid fields");
  const result = sealSpaceArtifactCreateV1({ requestId: String(row.requestId), kindId: String(row.kindId), name: String(row.name) });
  if (JSON.stringify(result) !== source) throw new Error("space artifact creation: noncanonical");
  return result;
}

/** 📤️ Decodes one canonical, bounded server status and withholds incomplete ready tuples. */
export function parseSpaceArtifactCreationStatusJsonV1(source: string): SpaceArtifactCreationStatusV1 {
  if (new TextEncoder().encode(source).byteLength > SPACE_ARTIFACT_CREATION_MAX_BYTES) throw new Error("space artifact creation status: capacity");
  const row = record(JSON.parse(source));
  if (row === null) throw new Error("space artifact creation status: invalid record");
  const phases: readonly SpaceArtifactCreationPhaseV1[] = ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"];
  const phase = phases.includes(row.phase as SpaceArtifactCreationPhaseV1) ? (row.phase as SpaceArtifactCreationPhaseV1) : null,
    parsedRequestId = requestId(row.requestId),
    spaceId = identity(row.spaceId),
    parsedReady = row.ready === undefined ? undefined : ready(row.ready);
  if (row.schema !== SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1 || parsedRequestId === null || spaceId === null || phase === null) throw new Error("space artifact creation status: invalid owner");
  if (!exactFields(row, phase === "ready" ? ["schema", "requestId", "spaceId", "phase", "ready"] : ["schema", "requestId", "spaceId", "phase"])) throw new Error("space artifact creation status: invalid fields");
  if ((phase === "ready") !== (parsedReady !== undefined && parsedReady !== null)) throw new Error("space artifact creation status: invalid ready");
  const result: SpaceArtifactCreationStatusV1 = phase === "ready" ? { schema: SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1, requestId: parsedRequestId, spaceId, phase, ready: parsedReady! } : { schema: SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1, requestId: parsedRequestId, spaceId, phase };
  if (JSON.stringify(result) !== source) throw new Error("space artifact creation status: noncanonical");
  return result;
}

/** 🗂️ Decodes only the exact selected-current, bounded and sorted creation presentation. */
export function parseSpaceArtifactCreationCatalogJsonV1(source: string): SpaceArtifactCreationCatalogV1 {
  if (new TextEncoder().encode(source).byteLength > SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES) throw new Error("space artifact creation catalog: capacity");
  const row = record(JSON.parse(source));
  if (row === null || !exactFields(row, ["schema", "spaceId", "catalogGenerationId", "kinds"])) throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = identity(row.spaceId),
    catalogGenerationId = digest(row.catalogGenerationId);
  if (row.schema !== SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1 || spaceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS)
    throw new Error("space artifact creation catalog: invalid owner");
  const kinds = row.kinds.map(creationKind);
  if (kinds.some((kind) => kind === null)) throw new Error("space artifact creation catalog: invalid kind");
  const sealed = kinds as SpaceArtifactCreationKindV1[];
  if (sealed.some((kind, index) => index > 0 && sealed[index - 1]!.kindId >= kind.kindId)) throw new Error("space artifact creation catalog: invalid order");
  const result: SpaceArtifactCreationCatalogV1 = { schema: SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1, spaceId, catalogGenerationId, kinds: sealed };
  if (JSON.stringify(result) !== source) throw new Error("space artifact creation catalog: noncanonical");
  return result;
}
