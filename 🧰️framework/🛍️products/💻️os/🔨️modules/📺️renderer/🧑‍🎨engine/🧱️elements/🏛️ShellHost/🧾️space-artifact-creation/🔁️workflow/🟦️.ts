/** 🔁️ Resolves a Workflow Catalogue drop through the owning Space's trusted creation catalog. */
import { dialectCoordinate, type ArtifactDialect } from "@semio-tech/framework";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, type BackboneWorkerRequest, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import type { DirectoryEvent } from "@semio-tech/framework/directory/schema";

type Catalog = Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }>;
type CreateRequest = Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }>;

export type WorkflowArtifactDropV1 = Readonly<{ pluginId: string; appId: string; nodeId: string; x: number; y: number }>;

export function workflowArtifactDropV1(args: Readonly<Record<string, unknown>> | undefined): WorkflowArtifactDropV1 | null {
  if (args === undefined || Object.keys(args).sort().join(",") !== "appId,nodeId,pluginId,x,y") return null;
  const { pluginId, appId, nodeId, x, y } = args;
  if (typeof pluginId !== "string" || !pluginId || typeof appId !== "string" || !appId || typeof nodeId !== "string" || !nodeId
    || typeof x !== "number" || !Number.isFinite(x) || typeof y !== "number" || !Number.isFinite(y)) return null;
  return { pluginId, appId, nodeId, x, y };
}

export function workflowArtifactCreationRequestV1(
  catalog: Pick<Catalog, "spaceId" | "catalogGenerationId" | "kinds">,
  app: Readonly<{ dialect?: ArtifactDialect; io?: Readonly<{ artifactSchema?: string }> }>,
  spaceId: string,
  requestId: string,
  name: string,
): CreateRequest | null {
  if (catalog.spaceId !== spaceId || !app.io?.artifactSchema || app.dialect === undefined || !name.trim()) return null;
  const dialect = dialectCoordinate(app.dialect);
  const kinds = catalog.kinds.filter((kind) => kind.schema === app.io?.artifactSchema && dialectCoordinate(kind.dialect) === dialect);
  return kinds.length === 1 ? { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId: catalog.catalogGenerationId, kindId: kinds[0]!.kindId, name } : null;
}

/** 📬️ Awaits the catalog addressed to this drop without changing the Space Index's selected catalog. */
export function readWorkflowArtifactCreationCatalogV1(worker: Worker, spaceId: string): Promise<Catalog | null> {
  const clientInstanceId = crypto.randomUUID();
  return new Promise((resolve) => {
    let catalog: Catalog | null = null;
    const finish = (result: Catalog | null): void => {
      clearTimeout(deadline);
      worker.removeEventListener("message", receive);
      resolve(result);
    };
    const receive = (event: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>): void => {
      const message = "wire" in event.data ? decodeBackboneWorkerResponse(event.data.wire) : event.data;
      if ((message.kind !== "space-artifact-creation-catalog" && message.kind !== "space-artifact-creation-catalog-status")
        || message.clientInstanceId !== clientInstanceId || message.spaceId !== spaceId) return;
      if (message.kind === "space-artifact-creation-catalog") catalog = message;
      else if (message.phase !== "loading") finish(message.phase === "ready" ? catalog : null);
    };
    const deadline = setTimeout(() => finish(null), 12_000);
    worker.addEventListener("message", receive);
    try {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-creation-catalog-open", spaceId, clientInstanceId }) });
    } catch {
      finish(null);
    }
  });
}

export type StudioWorkflowLookupV1 = "unknown-space" | "missing-workflow" | Readonly<{ artifactId: string }>;

export function studioWorkflowFromDirectoryV1(events: readonly DirectoryEvent[], spaceId: string, dialect: ArtifactDialect, schema: string): StudioWorkflowLookupV1 {
  const own = events.filter((event) => event.spaceId === spaceId);
  if (!own.some((event) => event.body.kind === "space.created") || own.some((event) => event.body.kind === "space.deleted")) return "unknown-space";
  const descriptors = new Set(own.flatMap((event) => event.body.kind === "document.announced"
    && event.body.descriptor.spaceId === spaceId && event.body.descriptor.artifactKind === dialect.artifactKind && event.body.descriptor.artifactSchema === schema
    ? [event.body.descriptor.documentId] : []));
  const coordinate = dialectCoordinate(dialect);
  const indexed = own.find((event) => event.body.kind === "document.indexed"
    && event.body.scope.spaceId === spaceId && descriptors.has(event.body.scope.documentId) && dialectCoordinate(event.body.entry.dialect) === coordinate);
  return indexed?.body.kind === "document.indexed" ? { artifactId: indexed.body.scope.documentId } : "missing-workflow";
}

/** 🪐️ Reads the sealed Space Index history before requesting its singleton Workflow through the ordinary creation saga. */
export function openStudioWorkflowV1(options: Readonly<{
  worker: Worker;
  baseUrl: string;
  spaceId: string;
  requestId: string;
  app: Readonly<{ dialect: ArtifactDialect; io: Readonly<{ artifactSchema: string }> }>;
  name: string;
  current: () => boolean;
  create: (request: CreateRequest) => void;
}>): Promise<StudioWorkflowLookupV1 | null> {
  const { worker, baseUrl, spaceId, requestId, app, name, current, create } = options;
  const clientInstanceId = crypto.randomUUID();
  return new Promise((resolve) => {
    let events: DirectoryEvent[] = [];
    const coordinate = dialectCoordinate(app.dialect);
    let request: CreateRequest | null = null;
    let settled = false;
    let phase: "lookup" | "catalog" | "creating" | "recheck" = "lookup";
    const send = (request: BackboneWorkerRequest): void => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
    const readIndex = (): void => send({ kind: "directory-space-open", baseUrl, spaceId, clientInstanceId });
    const finish = (result: StudioWorkflowLookupV1 | null): void => {
      if (settled) return;
      settled = true;
      clearInterval(liveness);
      clearTimeout(deadline);
      worker.removeEventListener("message", receive);
      try {
        send({ kind: "directory-space-close", spaceId, clientInstanceId });
        if (phase === "creating" && request !== null) send({ kind: "space-artifact-create-cancel", requestId, spaceId });
      } finally {
        resolve(result);
      }
    };
    const receive = (event: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>): void => {
      const message = "wire" in event.data ? decodeBackboneWorkerResponse(event.data.wire) : event.data;
      if (!current()) {
        finish(null);
      } else if ((message.kind === "directory-space-events" || message.kind === "directory-space-history-ready")
        && message.spaceId === spaceId && message.clientInstanceId === clientInstanceId) {
        const historyReady = message.kind === "directory-space-history-ready";
        if (!historyReady) {
          events.push(...message.events.filter(({ body }) => body.kind === "space.created" || body.kind === "space.deleted"
            || (body.kind === "document.announced" && body.descriptor.artifactSchema === app.io.artifactSchema)
            || (body.kind === "document.indexed" && dialectCoordinate(body.entry.dialect) === coordinate)));
          if (phase === "lookup" || phase === "recheck") return;
        }
        const lookup = studioWorkflowFromDirectoryV1(events, spaceId, app.dialect, app.io.artifactSchema);
        if (typeof lookup === "object" || (historyReady && (lookup === "unknown-space" || phase === "recheck"))) {
          finish(lookup);
        } else if (historyReady && phase === "lookup") {
          phase = "catalog";
          void readWorkflowArtifactCreationCatalogV1(worker, spaceId).then((catalog) => {
            if (settled || !current()) return;
            request = catalog === null ? null : workflowArtifactCreationRequestV1(catalog, app, spaceId, requestId, name);
            if (request === null) {
              finish(null);
              return;
            }
            phase = "creating";
            create(request);
          }).catch(() => finish(null));
        }
      } else if (message.kind === "space-artifact-creation-status" && phase === "creating" && request !== null
        && message.requestId === requestId && message.spaceId === spaceId && message.catalogGenerationId === request.expectedCatalogGenerationId) {
        if (message.phase === "ready" && message.ready?.kindId === request.kindId && message.ready.artifactSchema === app.io.artifactSchema
          && dialectCoordinate(message.ready.parentDialect) === dialectCoordinate(app.dialect)) {
          phase = "recheck";
          finish({ artifactId: message.ready.artifactId });
        } else if (message.phase === "failed" || message.phase === "indeterminate" || message.phase === "cancelled") {
          phase = "recheck";
          events = [];
          readIndex();
        }
      }
    };
    const liveness = setInterval(() => { if (!current()) finish(null); }, 250);
    const deadline = setTimeout(() => finish(null), 180_000);
    worker.addEventListener("message", receive);
    readIndex();
  });
}
