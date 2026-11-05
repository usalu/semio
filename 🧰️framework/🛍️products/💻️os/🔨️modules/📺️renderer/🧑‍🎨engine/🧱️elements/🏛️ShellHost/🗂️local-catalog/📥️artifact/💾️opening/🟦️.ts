import type { DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import type { LocalDocument, LocalDocumentSurface } from "@semio-tech/framework/config/mutations";
import type { LocalCatalogAdmissionV1 } from "../../🟦️.ts";
import { localArtifactAdmissionV1 } from "../🟦️.ts";

export type KeptArtifactReceiptV1 = Readonly<{ runtimeKey: string; clientInstanceId: string }>;

/** 💾️ Publishes an imported document only after its complete archive and catalog entry are durable. */
export async function keepArtifactOpeningV1(port: Readonly<{
  name: string; schema: string; surface: LocalDocumentSurface; dataDir: string | undefined; nowMs: number;
  signal: AbortSignal; current: () => boolean; progress: (completed: number, total: number) => void;
  identity: () => Promise<Readonly<{ parent_document_id: string | null }>>;
  archive: () => Promise<DocumentArchivePack>;
  write: (admission: LocalCatalogAdmissionV1) => Promise<boolean>;
  attach: (admission: LocalCatalogAdmissionV1) => Promise<KeptArtifactReceiptV1 | null>;
  record: (document: LocalDocument) => Promise<boolean>;
  publish: (document: LocalDocument) => boolean;
  close: (receipt: KeptArtifactReceiptV1) => Promise<void>;
  unlist: (document: LocalDocument) => Promise<void>;
  report: (error: unknown) => void;
}>): Promise<boolean> {
  let receipt: KeptArtifactReceiptV1 | null = null, document: LocalDocument | null = null, catalogAttempted = false;
  const checkpoint = (completed: number): void => {
    port.signal.throwIfAborted();
    if (!port.current()) throw new Error("natural-file.open-owner-retired");
    port.progress(completed, 5);
  };
  try {
    checkpoint(0);
    const identity = await port.identity();
    checkpoint(0);
    if (identity.parent_document_id === null) throw new Error("natural-file.document-identity-unavailable");
    const archive = await port.archive();
    checkpoint(1);
    const admission = await localArtifactAdmissionV1({ documentId: identity.parent_document_id, schema: port.schema, name: port.name, surface: port.surface, archive }, port.dataDir, port.nowMs);
    checkpoint(1);
    if ("refusal" in admission) throw new Error(`natural-file.${admission.refusal}`);
    document = admission.document;
    if (!(await port.write(admission))) throw new Error("natural-file.archive-not-persisted");
    checkpoint(2);
    receipt = await port.attach(admission);
    checkpoint(3);
    if (receipt === null) throw new Error("natural-file.attachment-refused");
    catalogAttempted = true;
    if (!(await port.record(document))) throw new Error("natural-file.catalog-not-persisted");
    checkpoint(4);
    if (!port.publish(document)) throw new Error("natural-file.panel-unavailable");
    port.progress(5, 5);
    return true;
  } catch (error) {
    if (receipt !== null) try { await port.close(receipt); } catch (cleanup) { port.report(cleanup); }
    if (catalogAttempted && document !== null) try { await port.unlist(document); } catch (cleanup) { port.report(cleanup); }
    throw error;
  }
}
