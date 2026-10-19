import { contentId } from "@semio-tech/schema-ts/artifact-reference";

/** 🌱️ Admitted document and actor-clock authority for replay-stable entity creation. */
export interface OperationEntityAuthority { readonly parentDocumentId: string; readonly authoringSeed: string }

/** 🪪️ Uses the same unambiguous UTF-8 framing as AppOperationContext::entity_id. */
export function operationEntityId(authority: OperationEntityAuthority, prefix: string, localKey: string): string {
  if (!authority.authoringSeed) throw Error("entity creation requires an admitted authoring seed");
  const encode = new TextEncoder();
  const field = (value: string) => encode.encode(value).length + ":" + value;
  return contentId(prefix, encode.encode(field(authority.parentDocumentId) + field(authority.authoringSeed) + field(localKey)));
}
