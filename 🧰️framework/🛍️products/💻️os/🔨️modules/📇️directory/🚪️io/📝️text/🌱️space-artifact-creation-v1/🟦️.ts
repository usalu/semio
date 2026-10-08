/** 🚪️ Canonical Directory JSON admission and physical receipt projection. */
import { type SpaceArtifactCreateV1, type SpaceArtifactCreationCatalogV1, type SpaceArtifactCreationKindV1, type SpaceArtifactCreationPhaseV1, type SpaceArtifactCreationStatusV1, SPACE_ARTIFACT_CREATE_SCHEMA_V1, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS, SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1, SPACE_ARTIFACT_CREATION_MAX_BYTES, SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1, creationKind, creationProgress, digest, exactFields, identity, name, ready, record, requestId, sealSpaceArtifactCreateV1 } from "../../../🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

/** 🧾️ Parses a canonical client request without admitting extra authority fields. */
export function parseSpaceArtifactCreateJsonV1(source: string): SpaceArtifactCreateV1 {
  if (new TextEncoder().encode(source).byteLength > SPACE_ARTIFACT_CREATION_MAX_BYTES) throw new Error("space artifact creation: capacity");
  const row = record(JSON.parse(source));
  if (row === null || !exactFields(row, ["schema", "requestId", "expectedCatalogGenerationId", "kindId", "name"]) || row.schema !== SPACE_ARTIFACT_CREATE_SCHEMA_V1) throw new Error("space artifact creation: invalid fields");
  const result = sealSpaceArtifactCreateV1({ requestId: String(row.requestId), expectedCatalogGenerationId: String(row.expectedCatalogGenerationId), kindId: String(row.kindId), name: String(row.name) });
  if (JSON.stringify(result) !== source) throw new Error("space artifact creation: noncanonical");
  return result;
}
/** 📤️ Decodes one canonical, bounded server status and withholds incomplete ready tuples; only a running
 * creation carries progress. */
export function parseSpaceArtifactCreationStatusJsonV1(source: string): SpaceArtifactCreationStatusV1 {
  if (new TextEncoder().encode(source).byteLength > SPACE_ARTIFACT_CREATION_MAX_BYTES) throw new Error("space artifact creation status: capacity");
  const row = record(JSON.parse(source));
  if (row === null) throw new Error("space artifact creation status: invalid record");
  const phases: readonly SpaceArtifactCreationPhaseV1[] = ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"];
  const phase = phases.includes(row.phase as SpaceArtifactCreationPhaseV1) ? (row.phase as SpaceArtifactCreationPhaseV1) : null,
    parsedRequestId = requestId(row.requestId),
    spaceId = identity(row.spaceId),
    catalogGenerationId = digest(row.catalogGenerationId),
    parsedReady = row.ready === undefined ? undefined : ready(row.ready),
    parsedProgress = row.progress === undefined ? undefined : creationProgress(row.progress),
    running = phase === "accepted" || phase === "preparing";
  if (row.schema !== SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1 || parsedRequestId === null || spaceId === null || catalogGenerationId === null || phase === null) throw new Error("space artifact creation status: invalid owner");
  const fields = ["schema", "requestId", "spaceId", "catalogGenerationId", "phase", ...(phase === "ready" ? ["ready"] : []), ...(running && row.progress !== undefined ? ["progress"] : [])];
  if (!exactFields(row, fields)) throw new Error("space artifact creation status: invalid fields");
  if ((phase === "ready") !== (parsedReady !== undefined && parsedReady !== null)) throw new Error("space artifact creation status: invalid ready");
  if (parsedProgress === null) throw new Error("space artifact creation status: invalid progress");
  const base = { schema: SPACE_ARTIFACT_CREATION_STATUS_SCHEMA_V1, requestId: parsedRequestId, spaceId, catalogGenerationId, phase } as const;
  const result: SpaceArtifactCreationStatusV1 = phase === "ready" ? { ...base, ready: parsedReady! } : parsedProgress !== undefined ? { ...base, progress: parsedProgress } : base;
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
