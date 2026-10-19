/** 🪪️ Canonical, bounded interaction addresses over ordered UTF-8 identity segments. */
export const SCOPED_IDENTITY_MAX_BYTES = 65_536;
export const SCOPED_IDENTITY_MAX_SEGMENTS = 32;
const prefix = "scope:1:", encoder = new TextEncoder(), decoder = new TextDecoder("utf-8", { fatal: true });

/** 🧩️ Encodes exact ordered nonempty Unicode scalar strings. */
export function encodeScopedIdentity(segments: readonly string[]): string {
  if (!Array.isArray(segments) || !segments.length || segments.length > SCOPED_IDENTITY_MAX_SEGMENTS) throw Error("scoped identity segments");
  const parts: string[] = [prefix];
  let extent = prefix.length;
  for (const segment of segments) {
    if (typeof segment !== "string" || !segment.length) throw Error("scoped identity segment");
    if (segment.length > SCOPED_IDENTITY_MAX_BYTES - extent) throw Error("scoped identity capacity");
    const bytes = encoder.encode(segment);
    if (decoder.decode(bytes) !== segment) throw Error("scoped identity Unicode");
    const length = `${bytes.length}:`;
    extent += length.length + bytes.length;
    if (extent > SCOPED_IDENTITY_MAX_BYTES) throw Error("scoped identity capacity");
    parts.push(length, segment);
  }
  return parts.join("");
}


/** 👁️ Admits the whole canonical identity before visiting its ordered segments. */
export function visitScopedIdentitySegments(identity: string, visitor: (segment: string) => void): number {
  const segments = decodeScopedIdentity(identity);
  for (const segment of segments) visitor(segment);
  return segments.length;
}

/** 🔎️ Decodes only exact canonical byte lengths and UTF-8 segment boundaries. */
export function decodeScopedIdentity(identity: string): readonly string[] {
  if (typeof identity !== "string" || !identity.startsWith(prefix)) throw Error("scoped identity prefix");
  if (identity.length > SCOPED_IDENTITY_MAX_BYTES) throw Error("scoped identity capacity");
  const bytes = encoder.encode(identity);
  if (bytes.length > SCOPED_IDENTITY_MAX_BYTES || decoder.decode(bytes) !== identity) throw Error("scoped identity capacity or Unicode");
  const segments: string[] = [];
  let cursor = prefix.length;
  while (cursor < bytes.length) {
    if (segments.length === SCOPED_IDENTITY_MAX_SEGMENTS) throw Error("scoped identity segments");
    const begin = cursor;
    while (cursor < bytes.length && bytes[cursor] !== 58) cursor++;
    const lengthText = decoder.decode(bytes.subarray(begin, cursor));
    if (!/^[1-9][0-9]*$/.test(lengthText) || cursor === bytes.length) throw Error("scoped identity length");
    const length = Number(lengthText), end = cursor + 1 + length;
    if (!Number.isSafeInteger(length) || end > bytes.length) throw Error("scoped identity length");
    segments.push(decoder.decode(bytes.subarray(cursor + 1, end)));
    cursor = end;
  }
  if (!segments.length) throw Error("scoped identity segments");
  return segments;
}
