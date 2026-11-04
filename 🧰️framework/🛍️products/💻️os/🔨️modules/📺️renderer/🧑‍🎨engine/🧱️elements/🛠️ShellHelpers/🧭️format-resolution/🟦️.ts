/** 🧭️ TypeScript twin of the Rust `🚪️io/🧭️format-resolution` precedence, driven by the shared `format-resolution` fixture: explicit key, then longest extension suffix, then exact MIME, then unresolved. */

/** 🏷️ One registered format as the resolver sees it. */
export type FormatClaimsV1 = Readonly<{ kindId: string; shortId: string; aliases: readonly string[]; mimes: readonly string[]; extensions: readonly string[] }>;

/** 🔎️ The facts a caller knows about one picked source. */
export type FormatQueryV1 = Readonly<{ explicit?: string; fileName?: string; mime?: string }>;

/** 🪪️ The claim that decided a resolution. */
export type FormatResolutionBasisV1 = "explicit" | "extension" | "mime";

/** ✅️ A resolved format, with the format the MIME alone would have chosen when it disagrees. */
export type FormatResolutionV1 = Readonly<{ format: FormatClaimsV1; basis: FormatResolutionBasisV1; mimeDisagreesWith: string | null }>;

/** 🚫️ A resolution that must not guess. */
export class FormatResolutionErrorV1 extends Error {
  constructor(readonly code: "unknownExplicit" | "ambiguous", readonly claim: string, readonly kindIds: readonly string[]) {
    super(`format-resolution.${code}: ${claim}`);
  }
}

const asciiLower = (text: string): string => text.replace(/[A-Z]/gu, (letter) => letter.toLowerCase());

function extensionCandidates(fileName: string): readonly string[] {
  const base = asciiLower((fileName.split(/[/\\]/u).at(-1) ?? fileName).trim());
  const found: string[] = [];
  for (let index = base.indexOf("."); index >= 0; index = base.indexOf(".", index + 1)) if (base.length - index > 1) found.push(base.slice(index));
  return found;
}

function holders(formats: readonly FormatClaimsV1[], claim: string, holds: (format: FormatClaimsV1) => boolean): FormatClaimsV1 | null {
  const found = formats.filter(holds).filter((format, index, all) => all.findIndex((other) => other.kindId === format.kindId) === index);
  if (found.length > 1) throw new FormatResolutionErrorV1("ambiguous", claim, found.map((format) => format.kindId));
  return found[0] ?? null;
}

/** 🧮️ Resolves one query against an explicit format list; `null` is unresolved, never a guessed fallback. */
export function resolveFormatV1(formats: readonly FormatClaimsV1[], query: FormatQueryV1): FormatResolutionV1 | null {
  if (typeof query.explicit === "string") {
    const key = query.explicit.trim();
    const format = holders(formats, key, (candidate) => candidate.kindId === key || candidate.shortId === key || candidate.aliases.includes(key));
    if (format === null) throw new FormatResolutionErrorV1("unknownExplicit", key, []);
    return { format, basis: "explicit", mimeDisagreesWith: null };
  }
  const byMime = (): FormatClaimsV1 | null => {
    const mime = asciiLower((typeof query.mime === "string" ? query.mime : "").split(";")[0]!.trim());
    return mime === "" ? null : holders(formats, mime, (candidate) => candidate.mimes.includes(mime));
  };
  for (const extension of typeof query.fileName === "string" ? extensionCandidates(query.fileName) : []) {
    const format = holders(formats, extension, (candidate) => candidate.extensions.includes(extension));
    if (format === null) continue;
    let other: FormatClaimsV1 | null = null;
    try {
      other = byMime();
    } catch (error) {
      if (!(error instanceof FormatResolutionErrorV1)) throw error;
    }
    return { format, basis: "extension", mimeDisagreesWith: other !== null && other.kindId !== format.kindId ? other.kindId : null };
  }
  const format = byMime();
  return format === null ? null : { format, basis: "mime", mimeDisagreesWith: null };
}

/** 🔬️ One revision's declared content check among the revisions that share a claim set. */
export type RevisionSniffV1 = Readonly<{ kindId: string; window: number; contains: readonly string[] }>;

/** 🎯️ Picks the revision whose declared needle occurs in the first `window` bytes of `head`; no match falls back to the declared owner. Twin of Rust `pick_revision`. */
export function pickRevisionV1(ownerKindId: string, revisions: readonly RevisionSniffV1[], head: Uint8Array): string {
  const text = (bytes: Uint8Array): string => Array.from(bytes, (byte) => String.fromCharCode(byte)).join("");
  const found = revisions.find((revision) => {
    const bounded = text(head.subarray(0, Math.min(head.length, revision.window)));
    return revision.contains.some((needle) => needle !== "" && bounded.includes(needle));
  });
  return found === undefined ? ownerKindId : found.kindId;
}
