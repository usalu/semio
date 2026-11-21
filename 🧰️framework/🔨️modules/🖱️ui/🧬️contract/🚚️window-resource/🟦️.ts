import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export const WINDOW_RESOURCE = schema.const;
const OFFSET = 0xcbf29ce484222325n;
const encoder = new TextEncoder();

function hashFrom(hash: bigint, bytes: Uint8Array): bigint {
  for (const byte of bytes) hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * 0x100000001b3n);
  return hash;
}

export type WindowResource = Readonly<{ key: string; owner: bigint; digest: bigint; bytes: number; pages: number }>;
export type WindowResourcePage = Readonly<{ resource: WindowResource; index: number }>;

export function parseWindowResourcePage(key: string): WindowResourcePage | undefined {
  if (!key.startsWith(WINDOW_RESOURCE.prefix)) return;
  const fields = /^([0-9a-f]{16})\.([0-9a-f]{16})\.([1-9][0-9]*)\.([1-9][0-9]*)$/.exec(key.slice(WINDOW_RESOURCE.prefix.length));
  if (!fields) return;
  const bytes = Number(fields[3]);
  const index = Number(fields[4]);
  const pages = Math.ceil(bytes / WINDOW_RESOURCE.pageBytes);
  if (!Number.isSafeInteger(bytes) || !Number.isSafeInteger(index) || bytes > WINDOW_RESOURCE.maximumBytes || index > pages) return;
  return { resource: { key: key.slice(0, key.lastIndexOf(".")), owner: BigInt("0x" + fields[1]), digest: BigInt("0x" + fields[2]), bytes, pages }, index };
}

/** 🪪️ Verifies the descriptor against the mounted window and semantic node key. */
export function windowResourceMatchesOwner(resource: WindowResource, window: string, key: string): boolean {
  return resource.owner === hashFrom(hashFrom(hashFrom(OFFSET, encoder.encode(window)), Uint8Array.of(0)), encoder.encode(key));
}

/** 📥️ Retains only the contiguous pages of one exact resource until completion. */
export class WindowResourceReader {
  readonly resource: WindowResource;
  private nextPage = 1;
  private hash = OFFSET;
  private chunks: string[] | undefined = [];
  private bytes = 0;

  constructor(resource: WindowResource) { this.resource = resource; }
  get next(): number | undefined { return this.nextPage <= this.resource.pages ? this.nextPage : undefined; }
  get completedBytes(): number { return this.bytes; }

  accept(page: WindowResourcePage, text: string): boolean {
    if (page.resource.key !== this.resource.key || page.index !== this.next || !text || !this.chunks) return false;
    const bytes = encoder.encode(text);
    if (new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes) !== text) return false;
    const end = this.bytes + bytes.length;
    const expected = Math.min(page.index * WINDOW_RESOURCE.pageBytes, this.resource.bytes);
    const terminal = page.index === this.resource.pages;
    if (end > expected || end < expected - (terminal ? 0 : 3)) return false;
    const hash = hashFrom(this.hash, bytes);
    if (terminal && hash !== this.resource.digest) return false;
    this.bytes = end;
    this.hash = hash;
    this.chunks.push(text);
    this.nextPage++;
    return true;
  }

  complete(): string | undefined { return this.next === undefined ? this.chunks?.join("") : undefined; }

  take(): string | undefined {
    const source = this.complete();
    if (source !== undefined) this.chunks = undefined;
    return source;
  }
}
