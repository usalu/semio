/** 🗜️ Bounded content check of a zip-based (OPC) file before any document is created: the central directory names `[Content_Types].xml`, its `Override` for a declared main part content type names the main part, and the root namespace of that part selects the dialect. Reads only the zip tail, the central directory and two small entries, inflates through the platform `DecompressionStream`, and never materializes the whole file. */

/** 📜️ What a format claims about its package: the main part content types that make the file a valid package of this format, and the dialect (`artifactRef`) each root namespace of the main part routes to. */
export type PackageClaimV1 = Readonly<{ mainPartContentTypes: readonly string[]; dialects: readonly Readonly<{ artifactRef: string; namespace: string }>[] }>;

/** 📖️ Random access to the picked bytes; `end` is exclusive. */
export type PackageSourceV1 = Readonly<{ size: number; read: (start: number, end: number) => Promise<Uint8Array> }>;

/** ⚖️ `mismatch`: not a package of the claimed format. `match`: it is, and `artifactRef` is the dialect its root namespace selects (`null`: none of the declared namespaces, so the format owner decides). */
export type PackageVerdictV1 = Readonly<{ kind: "mismatch" }> | Readonly<{ kind: "match"; artifactRef: string | null }>;

/** 🔬️ Read bounds of the check. */
export const PACKAGE_SNIFF_LIMITS_V1 = Object.freeze({ tailBytes: 22 + 65535, centralDirectoryBytes: 8 * 1024 * 1024, contentTypesBytes: 2 * 1024 * 1024, rootBytes: 8192, compressedBytes: 8 * 1024 * 1024 });

type Entry = Readonly<{ method: number; compressed: number; offset: number }>;

const END_OF_CENTRAL_DIRECTORY = 0x06054b50;
const CENTRAL_DIRECTORY_ENTRY = 0x02014b50;
const LOCAL_FILE_HEADER = 0x04034b50;
const CONTENT_TYPES_PART = "[content_types].xml";

const view = (bytes: Uint8Array): DataView => new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);

async function readCentralDirectory(source: PackageSourceV1, signal: AbortSignal): Promise<ReadonlyMap<string, Entry> | null> {
  if (source.size < 22) return null;
  signal.throwIfAborted();
  const tailStart = Math.max(0, source.size - PACKAGE_SNIFF_LIMITS_V1.tailBytes);
  const tail = await source.read(tailStart, source.size);
  const tailView = view(tail);
  let end = -1;
  for (let at = tail.length - 22; at >= 0; at -= 1) {
    if (tailView.getUint32(at, true) === END_OF_CENTRAL_DIRECTORY && at + 22 + tailView.getUint16(at + 20, true) === tail.length) {
      end = at;
      break;
    }
  }
  if (end < 0) return null;
  const count = tailView.getUint16(end + 10, true);
  const size = tailView.getUint32(end + 12, true);
  const offset = tailView.getUint32(end + 16, true);
  if (count === 0xffff || size === 0xffffffff || offset === 0xffffffff || size > PACKAGE_SNIFF_LIMITS_V1.centralDirectoryBytes || offset + size > source.size) return null;
  signal.throwIfAborted();
  const directory = await source.read(offset, offset + size);
  const directoryView = view(directory);
  const decoder = new TextDecoder("utf-8");
  const entries = new Map<string, Entry>();
  let at = 0;
  for (let index = 0; index < count; index += 1) {
    if (at + 46 > directory.length || directoryView.getUint32(at, true) !== CENTRAL_DIRECTORY_ENTRY) return null;
    const nameLength = directoryView.getUint16(at + 28, true);
    const extraLength = directoryView.getUint16(at + 30, true);
    const commentLength = directoryView.getUint16(at + 32, true);
    if (at + 46 + nameLength > directory.length) return null;
    entries.set(decoder.decode(directory.subarray(at + 46, at + 46 + nameLength)).toLowerCase(), { method: directoryView.getUint16(at + 10, true), compressed: directoryView.getUint32(at + 20, true), offset: directoryView.getUint32(at + 42, true) });
    at += 46 + nameLength + extraLength + commentLength;
  }
  return entries;
}

async function inflateHead(compressed: Uint8Array, limit: number): Promise<Readonly<{ bytes: Uint8Array; truncated: boolean }>> {
  const input = new ReadableStream<BufferSource>({ start: (controller) => { controller.enqueue(new Uint8Array(compressed)); controller.close(); } });
  const reader = input.pipeThrough(new DecompressionStream("deflate-raw")).getReader();
  const chunks: Uint8Array[] = [];
  let total = 0;
  let truncated = false;
  try {
    while (total <= limit) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
      total += value.length;
    }
    truncated = total > limit;
  } finally {
    await reader.cancel().catch(() => undefined);
  }
  const joined = new Uint8Array(Math.min(total, limit));
  let at = 0;
  for (const chunk of chunks) {
    if (at >= joined.length) break;
    joined.set(chunk.subarray(0, joined.length - at), at);
    at += chunk.length;
  }
  return { bytes: joined, truncated };
}

async function readEntry(source: PackageSourceV1, entry: Entry, limit: number, signal: AbortSignal): Promise<Readonly<{ bytes: Uint8Array; truncated: boolean }> | null> {
  if (entry.compressed > PACKAGE_SNIFF_LIMITS_V1.compressedBytes || entry.offset + 30 > source.size) return null;
  signal.throwIfAborted();
  const header = await source.read(entry.offset, entry.offset + 30);
  if (header.length < 30 || view(header).getUint32(0, true) !== LOCAL_FILE_HEADER) return null;
  const start = entry.offset + 30 + view(header).getUint16(26, true) + view(header).getUint16(28, true);
  if (start + entry.compressed > source.size) return null;
  signal.throwIfAborted();
  if (entry.method === 0) {
    const head = await source.read(start, start + Math.min(entry.compressed, limit));
    return { bytes: head, truncated: entry.compressed > limit };
  }
  if (entry.method !== 8) return null;
  try {
    return await inflateHead(await source.read(start, start + entry.compressed), limit);
  } catch {
    signal.throwIfAborted();
    return null;
  }
}

const attribute = (tag: string, name: string): string | null => {
  const found = new RegExp(`\\b${name}\\s*=\\s*(?:"([^"]*)"|'([^']*)')`, "iu").exec(tag);
  return found === null ? null : (found[1] ?? found[2] ?? null);
};

/** 🧮️ Checks one picked package against the claim of its format; unreadable, truncated, oversized or foreign content is a `mismatch`, never a guess. */
export async function inspectPackageV1(source: PackageSourceV1, claim: PackageClaimV1, signal: AbortSignal): Promise<PackageVerdictV1> {
  const mismatch: PackageVerdictV1 = { kind: "mismatch" };
  let entries: ReadonlyMap<string, Entry> | null;
  try {
    entries = await readCentralDirectory(source, signal);
  } catch {
    signal.throwIfAborted();
    return mismatch;
  }
  const typesEntry = entries?.get(CONTENT_TYPES_PART);
  if (entries === null || typesEntry === undefined) return mismatch;
  const types = await readEntry(source, typesEntry, PACKAGE_SNIFF_LIMITS_V1.contentTypesBytes, signal);
  if (types === null || types.truncated) return mismatch;
  const wanted = new Set(claim.mainPartContentTypes.map((contentType) => contentType.toLowerCase()));
  const text = new TextDecoder("utf-8").decode(types.bytes);
  let mainPart: string | null = null;
  for (const [tag] of text.matchAll(/<Override\b[^>]*>/giu)) {
    const contentType = attribute(tag, "ContentType")?.split(";")[0]?.trim().toLowerCase();
    const partName = attribute(tag, "PartName");
    if (contentType !== undefined && partName !== null && wanted.has(contentType)) {
      mainPart = partName.replace(/^\/+/u, "").toLowerCase();
      break;
    }
  }
  const mainEntry = mainPart === null ? undefined : entries.get(mainPart);
  if (mainEntry === undefined) return mismatch;
  const root = await readEntry(source, mainEntry, PACKAGE_SNIFF_LIMITS_V1.rootBytes, signal);
  if (root === null) return mismatch;
  const head = new TextDecoder("utf-8").decode(root.bytes);
  return { kind: "match", artifactRef: claim.dialects.find((dialect) => head.includes(dialect.namespace))?.artifactRef ?? null };
}
