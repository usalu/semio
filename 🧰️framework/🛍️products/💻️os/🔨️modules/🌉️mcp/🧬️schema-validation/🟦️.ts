/** ✅️ JSON Schema draft 2020-12 validation for the conformance suite, kept inside the package that
 * declares `ajv` rather than in the module-root surface — a `🔨️modules/*` component file must not
 * reach for an external package directly (repo policy `not-to-unlisted`, and CLAUDE.md's "use all
 * external libraries behind an interface"). Only a plain boolean-ish result crosses this boundary,
 * so no Ajv type ever escapes into the module's public API. */
import Ajv2020 from "ajv/dist/2020.js";

/** ✅️ True iff `schema` is a structurally valid JSON Schema draft 2020-12 document — compiling it
 * with a real `Ajv2020` instance is itself the proof: Ajv rejects anything that does not parse as a
 * schema under that draft. A fresh instance per call avoids `$id` collisions across unrelated tool
 * schemas that share no registry. */
export function isValidJsonSchema2020_12(schema: unknown): { readonly valid: true } | { readonly valid: false; readonly error: string } {
  try {
    new Ajv2020({ strict: false }).compile(schema as object);
    return { valid: true };
  } catch (error) {
    return { valid: false, error: String(error) };
  }
}
