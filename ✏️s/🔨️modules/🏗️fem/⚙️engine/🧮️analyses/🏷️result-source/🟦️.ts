import { encodeScopedIdentity, decodeScopedIdentity } from "../../../../../../🧰️framework/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts";

/** 🏷️ A physical result is addressed by its source kind and semantic identifier. */
export interface ResultSource { readonly kind: "loadCase" | "combination"; readonly id: string; }

/** 🪪️ Serializes the declared source namespace through the shared canonical codec. */
export function encodeResultSourceIdentity(source: { readonly kind: string; readonly id: string }): string {
  if (source.kind !== "loadCase" && source.kind !== "combination") throw Error("result source kind");
  return encodeScopedIdentity([source.kind, source.id]);
}

/** 🔎️ Admits only exactly two canonical segments from the result source vocabulary. */
export function decodeResultSourceIdentity(identity: string): ResultSource {
  const segments = decodeScopedIdentity(identity);
  if (segments.length !== 2) throw Error("result source arity");
  const [kind, id] = segments;
  if (kind !== "loadCase" && kind !== "combination") throw Error("result source kind");
  return { kind, id: id! };
}
