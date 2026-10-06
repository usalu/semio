/** 🔣️ Remodeling physical tagged JSON mutation admission. */
import {REMODELING_MUTATION_SPECS, type RemodelingMutationTag, type RemodelingMutation} from "../../../../🧬️schema/🧬️mutations/🟦️.ts";
import {decodeRecord} from "../../📸️snapshot/🔣️json/🟦️.ts";


/** 🦠️ Decodes a parsed RFC 8259 value into a validated tagged mutation. */
export function decodeRemodelingMutation(json: unknown): RemodelingMutation {
  if (typeof json !== "object" || json === null || Array.isArray(json)) throw new Error(`mutation: expected an object, got ${JSON.stringify(json)}`);
  const source = json as Record<string, unknown>;
  const tag = source.mutation;
  if (typeof tag !== "string" || !(tag in REMODELING_MUTATION_SPECS)) throw new Error(`mutation: unknown tag ${JSON.stringify(tag)}`);
  const { mutation: _tag, ...rest } = source;
  const body = decodeRecord(rest, REMODELING_MUTATION_SPECS[tag as RemodelingMutationTag], `${tag}`);
  return { mutation: tag, ...body } as unknown as RemodelingMutation;
}
