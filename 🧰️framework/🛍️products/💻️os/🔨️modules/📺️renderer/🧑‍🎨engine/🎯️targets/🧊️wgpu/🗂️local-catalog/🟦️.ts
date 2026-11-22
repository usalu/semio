import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { localCatalogActorConfig } from "@semio-tech/framework/backbone/config-lane";
import { admitLocalDocument, LOCAL_CATALOG_CONFIG_SCHEMA, retireLocalDocument, type LocalCatalog, type LocalCatalogConfigMutation, type LocalDocument } from "@semio-tech/framework/config/mutations";
import { LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, decodeLocalCatalogArchiveV1, foldLocalCatalogEnvelopesV1, localCatalogMutationEnvelopeV1 } from "../../../🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts";
import { awaitLocalCatalogMutationV1 } from "../../../🧱️elements/🏛️ShellHost/🗂️local-catalog/💾️acknowledgment/🟦️.ts";

const GRANT = { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 } as const;

/** 🗂️ The page's client of the device's local document catalog: the SAME event-sourced `os.config.local-catalog` facet React's shell opens on the backbone store worker (folder lane under `<dataDir>/os`), so a document kept from either renderer is listed by both. Mutations carry React's own envelope and wait for React's verified read-back. */
export type WgpuLocalCatalog = Readonly<{
  admit: (document: LocalDocument) => Promise<boolean>;
  retire: (documentId: string) => Promise<boolean>;
  catalog: () => LocalCatalog;
  dispose: () => void;
}>;

export type WgpuLocalCatalogOptions = Readonly<{
  dataDir: string | undefined;
  actor: string;
  workerFactory?: () => Worker;
  onChange: (catalog: LocalCatalog) => void;
}>;

/** 🗂️ Opens the facet on its own store worker; the catalog follows the store's archive and other tabs' mutations. */
export function createWgpuLocalCatalog(options: WgpuLocalCatalogOptions): WgpuLocalCatalog {
  const worker = options.workerFactory?.() ?? new Worker(new URL("../../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url), { type: "module" });
  const clientInstanceId = crypto.randomUUID();
  const abort = new AbortController();
  let current: LocalCatalog = { documents: [] };
  let owner: { readonly status: (persisted: boolean) => void; readonly archive: (bytes: Uint8Array) => void } | undefined;
  let queue: Promise<unknown> = Promise.resolve();
  const publish = (next: LocalCatalog): void => {
    current = next;
    options.onChange(next);
  };
  worker.onmessage = (event: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>) => {
    const message = "wire" in event.data ? decodeBackboneWorkerResponse(event.data.wire) : event.data;
    if (message.kind !== "event" || message.documentId !== LOCAL_CATALOG_CONFIG_SCHEMA || message.clientInstanceId !== clientInstanceId) return;
    const change = message.event;
    if (change.kind === "documentArchiveReplaced") {
      const decoded = decodeLocalCatalogArchiveV1(change.archive);
      if (decoded === undefined) return;
      publish(decoded);
      owner?.archive(change.archive);
    } else if (change.kind === "remoteMutations") publish(foldLocalCatalogEnvelopesV1(current, change.envelopes));
    else if (change.kind === "status") owner?.status(change.persisted && change.pendingMutations === 0 && change.writeFailure === undefined);
  };
  worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", clientInstanceId, ...localCatalogActorConfig(options.actor, GRANT, options.dataDir) }) });
  const record = (mutation: LocalCatalogConfigMutation): Promise<boolean> => {
    const commit = queue.catch(() => null).then(async () => {
      const envelope = localCatalogMutationEnvelopeV1(options.actor, mutation, current);
      const next = foldLocalCatalogEnvelopesV1(current, [envelope]);
      const acknowledged = await awaitLocalCatalogMutationV1({
        mutation, signal: abort.signal, timeoutMs: LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, current: () => !abort.signal.aborted,
        subscribe: (listener) => {
          if (owner !== undefined) throw new Error("local-catalog.commit-owner-busy");
          owner = { status: listener.status, archive: (bytes) => { const catalog = decodeLocalCatalogArchiveV1(bytes); if (catalog !== undefined) listener.archive(catalog); } };
          return () => { owner = undefined; };
        },
        submit: () => {
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [envelope] } }) });
          publish(next);
        },
        probe: () => worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId, message: { kind: "externalChanged" } }) }),
      });
      return acknowledged !== null;
    });
    queue = commit;
    return commit;
  };
  return {
    admit: (document) => record(admitLocalDocument(document)),
    retire: (documentId) => record(retireLocalDocument(documentId)),
    catalog: () => current,
    dispose: () => {
      abort.abort();
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId }) });
      worker.terminate();
    },
  };
}
