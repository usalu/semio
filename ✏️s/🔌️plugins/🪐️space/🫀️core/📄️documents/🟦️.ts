/** 🧭️ Authored source encoding admitted by an explicitly selected codec. */
export type SpaceDocumentFormat = "json" | "dsl";

/** 📄️ Opaque document identity and its authored source input. */
export interface SpaceDocumentSource {
  slug: string;
  format: SpaceDocumentFormat;
  codec: string;
  text: string;
}

/** 🔐️ Owned boundary around source decoding into a snapshot document. */
export interface SpaceDocumentCodec {
  id: string;
  decode(format: SpaceDocumentFormat, text: string): string;
}

/** 📄️ Validated snapshot document ready for caller-owned admission. */
export interface SpaceDocumentDocument {
  slug: string;
  document: string;
}

/** 🧪️ Prepares every source without changing runtime state. */
export function prepareSpaceDocumentSources(sources: readonly SpaceDocumentSource[], codecs: readonly SpaceDocumentCodec[]): SpaceDocumentDocument[] {
  if (!sources.length) throw new Error("document sources are empty");
  const bindings = new Map<string, SpaceDocumentCodec>();
  for (const codec of codecs) {
    if (!codec.id.trim() || bindings.has(codec.id)) throw new Error("document codec identity is empty or repeated");
    bindings.set(codec.id, codec);
  }
  const slugs = new Set<string>();
  return sources.map((source) => {
    if (!source.slug.trim() || slugs.has(source.slug)) throw new Error("document slug is empty or repeated");
    slugs.add(source.slug);
    if (!source.text.trim() || !["json", "dsl"].includes(source.format)) throw new Error("document source format or text is invalid");
    const codec = bindings.get(source.codec);
    if (!codec) throw new Error("document codec is missing");
    const document = codec.decode(source.format, source.text), value = JSON.parse(document);
    if (!value || typeof value !== "object" || Array.isArray(value) || !Object.keys(value).length) throw new Error("document document is empty or is not an object");
    return { slug: source.slug, document };
  });
}
