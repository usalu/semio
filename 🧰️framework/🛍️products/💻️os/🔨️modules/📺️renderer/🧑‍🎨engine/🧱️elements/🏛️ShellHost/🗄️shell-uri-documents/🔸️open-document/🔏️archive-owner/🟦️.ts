/** 🔏️ A folder document's archive is read from the app instance that currently shows it, and only then. */
import type { DocumentArchivePack } from "@semio-tech/framework/wire-contracts";

export type ArchiveOwnerEntryV1 = Readonly<{ documentId: string; session: Readonly<{ instanceId: number }> }>;
export type ArchiveOwnerPluginV1 = Readonly<{
  readAppDocumentArchive?: (instanceId: number) => Promise<DocumentArchivePack>;
  readAppDocumentIdentity?: (instanceId: number) => Promise<Readonly<{ parentDocumentId?: string | null }>>;
}>;

/** 🔏️ Reads the archive of `entry` and fails closed (`null`, nothing to write) unless the instance still belongs to it: no other open document shares the instance, and the instance's own document identity, read before and after the archive, never names another document. Throws only when the archive reader is unavailable. */
export async function readOwnedDocumentArchiveV1(plugin: ArchiveOwnerPluginV1, entry: ArchiveOwnerEntryV1, runtimeKey: string, sessions: ReadonlyMap<string, ArchiveOwnerEntryV1>): Promise<DocumentArchivePack | null> {
  if (!plugin.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
  const instanceId = entry.session.instanceId;
  const owned = async (): Promise<boolean> => {
    if (sessions.get(runtimeKey) !== entry) return false;
    for (const [key, other] of sessions) if (key !== runtimeKey && other.session.instanceId === instanceId && other.documentId !== entry.documentId) return false;
    const identity = plugin.readAppDocumentIdentity === undefined ? null : await plugin.readAppDocumentIdentity(instanceId);
    return identity?.parentDocumentId == null || identity.parentDocumentId === entry.documentId;
  };
  if (!(await owned())) return null;
  const archive = await plugin.readAppDocumentArchive(instanceId);
  return (await owned()) ? archive : null;
}
