/** 🔏️ The PDF measurement tool of the gallery cases: it expands a document's compressed streams,
 * strips the metadata a second run of the same source rewrites (dates, ids, producer, XMP), and
 * digests what is left, so two renders can be compared for sameness and two kinds for distinctness.
 * It lives here because a digest is evidence a test gathers — the gallery module itself neither
 * hashes nor inflates anything.
 */
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { inflateRawSync, inflateSync } from "node:zlib";


function inflatePdfStreamBody(body: Buffer): Buffer {
  try {
    return inflateSync(body);
  } catch {
    return inflateRawSync(body);
  }
}

/** 🔏️ Hashes PDF bytes after removing volatile document metadata and expanding streams. */
export function pdfStableHash(pdfPath: string): string {
  const raw = readFileSync(pdfPath).toString("binary");
  const inflated = raw.replace(/stream\r?\n([\s\S]*?)\r?\nendstream/g, (_all, body: string) => {
    try {
      return `stream\n${inflatePdfStreamBody(Buffer.from(body, "binary")).toString("binary")}\nendstream`;
    } catch {
      return `stream\n${body}\nendstream`;
    }
  });
  const text = inflated
    .replace(/\/CreationDate\s*\([^)]*\)/g, "")
    .replace(/\/ModDate\s*\([^)]*\)/g, "")
    .replace(/\/ID\s*\[[^\]]*\]/g, "")
    .replace(/\(D:[0-9+\-'Z]+\)/g, "")
    .replace(/\/Producer\s*\([^)]*\)/g, "")
    .replace(/\/Creator\s*\([^)]*\)/g, "")
    .replace(/<x:xmpmeta[\s\S]*?<\/x:xmpmeta>/g, "");
  return createHash("sha256").update(text, "binary").digest("hex");
}
