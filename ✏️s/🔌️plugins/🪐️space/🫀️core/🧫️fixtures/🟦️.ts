/** 🧭️ Authored source encoding admitted by an explicitly selected codec. */
export type SpaceFixtureFormat = "json" | "dsl";

/** 🧫️ Opaque fixture identity and its authored source input. */
export interface SpaceFixtureSource {
  slug: string;
  format: SpaceFixtureFormat;
  codec: string;
  text: string;
}

/** 🔐️ Owned boundary around source decoding into a snapshot document. */
export interface SpaceFixtureCodec {
  id: string;
  decode(format: SpaceFixtureFormat, text: string): string;
}

/** 📄️ Validated snapshot document ready for one atomic admission. */
export interface SpaceFixtureDocument {
  slug: string;
  document: string;
}

/** 🧪️ Prepares every source before any registry mutation can occur. */
export function prepareSpaceFixtureSources(sources: readonly SpaceFixtureSource[], codecs: readonly SpaceFixtureCodec[]): SpaceFixtureDocument[] {
  if (!sources.length) throw new Error("fixture sources are empty");
  const bindings = new Map<string, SpaceFixtureCodec>();
  for (const codec of codecs) {
    if (!codec.id.trim() || bindings.has(codec.id)) throw new Error("fixture codec identity is empty or repeated");
    bindings.set(codec.id, codec);
  }
  const slugs = new Set<string>();
  return sources.map((source) => {
    if (!source.slug.trim() || slugs.has(source.slug)) throw new Error("fixture slug is empty or repeated");
    slugs.add(source.slug);
    if (!source.text.trim() || !["json", "dsl"].includes(source.format)) throw new Error("fixture source format or text is invalid");
    const codec = bindings.get(source.codec);
    if (!codec) throw new Error("fixture codec is missing");
    const document = codec.decode(source.format, source.text), value = JSON.parse(document);
    if (!value || typeof value !== "object" || Array.isArray(value) || !Object.keys(value).length) throw new Error("fixture document is empty or is not an object");
    return { slug: source.slug, document };
  });
}
