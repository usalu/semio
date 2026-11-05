/** 🔏️ Reads an opaque archive only from its exact live plugin and document instance. */
import type { AppDocumentIdentity } from "@semio-tech/framework/wire-contracts";

export type ArchiveOwnerEntryV1 = Readonly<{ documentId: string; session: Readonly<{ pluginId: string; instanceId: number }> }>;
export type ArchiveOwnerPluginV1 = Readonly<{
  readAppDocumentArchive?: (instanceId: number) => Promise<DocumentArchivePack>;
  readAppDocumentIdentity: (instanceId: number) => Promise<Readonly<{ parent_document_id: string | null }>>;
}>;

/** 🔏️ Reads only the captured owner's archive: its plugin-scoped instance and native document identity must remain current before and after both asynchronous reads. */
export async function readOwnedDocumentArchiveV1(plugin: ArchiveOwnerPluginV1, entry: ArchiveOwnerEntryV1, runtimeKey: string, sessions: ReadonlyMap<string, ArchiveOwnerEntryV1>): Promise<DocumentArchivePack | null> {
  if (!plugin.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
  const { pluginId, instanceId } = entry.session;
  const held = (): boolean => {
    if (sessions.get(runtimeKey) !== entry) return false;
    for (const [key, other] of sessions) if (key !== runtimeKey && other.session.pluginId === pluginId && other.session.instanceId === instanceId && other.documentId !== entry.documentId) return false;
    return true;
  };
  const owned = async (): Promise<boolean> => {
    if (!held()) return false;
    const identity = await plugin.readAppDocumentIdentity(instanceId);
    return held() && identity.parent_document_id === entry.documentId;
  };
  if (!(await owned())) return null;
  const archive = await plugin.readAppDocumentArchive(instanceId);
  return (await owned()) ? archive : null;
}
