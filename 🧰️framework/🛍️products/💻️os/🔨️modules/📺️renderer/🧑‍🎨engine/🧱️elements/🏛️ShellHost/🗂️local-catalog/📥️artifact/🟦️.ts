import { encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import type { LocalDocumentSurface } from "@semio-tech/framework/config/mutations";
import type { LocalCatalogAdmissionV1, LocalCatalogRefusalV1 } from "../🟦️.ts";

const utf8 = new TextEncoder();
const STORAGE_SUFFIX_CAPACITY = 64;
const bounded = (value: string, empty = false): boolean => (empty || value.length > 0) && value.trim() === value && utf8.encode(value).length <= 256 && !/[\u0000-\u001f\u007f]/u.test(value);

/** 📥️ Keeps the imported document's real identity, selected surface and complete recursive archive. */
export async function localArtifactAdmissionV1(input: Readonly<{ documentId: string; schema: string; name: string; surface: LocalDocumentSurface; archive: DocumentArchivePack }>, dataDir: string | undefined, nowMs: number): Promise<LocalCatalogAdmissionV1 | Readonly<{ refusal: LocalCatalogRefusalV1 }>> {
  if (!dataDir) return { refusal: "no-data-folder" };
  if (!bounded(input.documentId) || !bounded(input.schema) || !bounded(input.name, true) || !bounded(input.surface.pluginId) || !bounded(input.surface.appId) || input.archive.parent_pack.length === 0 || !Number.isSafeInteger(nowMs) || nowMs < 0) return { refusal: "invalid-request" };
  const digest = await crypto.subtle.digest("SHA-256", utf8.encode(input.documentId));
  const basename = Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
  const target = `${dataDir.replace(/[\\/]+$/u, "")}/os/local-documents/${basename}`;
  if (target.length + STORAGE_SUFFIX_CAPACITY > 256) return { refusal: "invalid-request" };
  try {
    const archive = encodeDocumentArchiveBytes(input.archive);
    return {
      document: { documentId: input.documentId, schema: input.schema, name: input.name, surface: input.surface, storage: "folder", target, admittedAtMs: nowMs },
      archive,
      binding: { kind: "folder", dataClass: "persistedLocalOnly", path: target },
    };
  } catch { return { refusal: "invalid-request" }; }
}
