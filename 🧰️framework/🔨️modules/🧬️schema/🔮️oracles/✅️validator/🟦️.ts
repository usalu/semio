/** 🧬️ The one strict Ajv oracle for semio JSON schemas: Ajv in strict mode with the `x-semio-*` vendor annotation vocabulary
 * (`../../🧫️fixtures/🧬️vendor-annotation-vocabulary/🔣️.json`) registered as annotation keywords, each with its value's meta-schema. Every schema law builds its validator here
 * instead of registering vendor keywords one call site at a time, so a schema that gains an annotation (e.g. `x-semio-note` in
 * `📇️directory/🧬️schema/🔣️.json`) keeps compiling everywhere, and a keyword outside the vocabulary still fails
 * (ticket 26/09/23 S15). https://ajv.js.org/strict-mode.html#prohibit-ignored-keywords */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import Ajv, { type Options } from "ajv";
import addFormats, { type FormatName } from "ajv-formats";
import vocabulary from "../../🧫️fixtures/🧬️vendor-annotation-vocabulary/🔣️.json" with { type: "json" };
import manifestSchema from "../../../🛂️manifest/🧬️schema/🔣️.json" with { type: "json" };
import formatPolicy from "../../🧫️fixtures/✅️draft07-validation-vectors.json" with { type: "json" };

export const SEMIO_SCHEMA_VENDOR_VOCABULARY_V1 = vocabulary;

/** 🧬️ The schema documents a vocabulary meta-schema `$ref`s (`x-semio-ui` → the manifest's `InputUi` export), added before the keywords compile. */
export const SEMIO_SCHEMA_VENDOR_VOCABULARY_DOCUMENTS_V1: readonly { readonly $id: string }[] = [manifestSchema];

/** 🏷️ The owned validator's pinned `format` policy (`ASSERTED_STRING_FORMATS` in `🧬️schema/✅️validator/🦀️.rs`): the asserted
 * string formats, checked by `ajv-formats`, and the proto-derived spellings that stay annotations and never reject. */
export const SEMIO_SCHEMA_FORMAT_POLICY_V1: { readonly asserted: readonly FormatName[]; readonly annotation: readonly string[] } = { asserted: formatPolicy.assertedFormats as FormatName[], annotation: formatPolicy.annotationFormats };

/** 🧬️ A strict Ajv (`options` override the defaults) that knows every declared `x-semio-*` annotation and every pinned `format`. */
export function semioSchemaAjvV1(options: Options = {}): Ajv {
  const ajv = new Ajv({ strict: true, ...options });
  addFormats(ajv, [...SEMIO_SCHEMA_FORMAT_POLICY_V1.asserted]);
  for (const format of SEMIO_SCHEMA_FORMAT_POLICY_V1.annotation) ajv.addFormat(format, true);
  for (const document of SEMIO_SCHEMA_VENDOR_VOCABULARY_DOCUMENTS_V1) ajv.addSchema(document);
  for (const [keyword, metaSchema] of Object.entries(vocabulary.keywords)) ajv.addKeyword({ keyword, metaSchema });
  return ajv;
}

/** 🧺️ Adds every mutation leaf payload schema (`<mutations>/<leaf>/🧬️schema/🔣️.json`) of one aggregate collection to `ajv`, so the
 * aggregate document `<mutations>/🔣️.json` — a `oneOf` of leaf `$ref`s under the repo-wide aggregate rule — compiles. */
export function addSemioMutationLeafSchemasV1(ajv: Ajv, mutations: URL): Ajv {
  const directory = new URL(mutations);
  if (!directory.pathname.endsWith("/")) directory.pathname += "/";
  for (const leaf of readdirSync(mutations, { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const schema = new URL(`${encodeURIComponent(leaf.name)}/🧬️schema/🔣️.json`, directory);
    if (existsSync(schema)) ajv.addSchema(JSON.parse(readFileSync(schema, "utf8")));
  }
  return ajv;
}
