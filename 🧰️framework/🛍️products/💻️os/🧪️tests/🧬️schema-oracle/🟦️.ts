/** 🧬️ The one strict Ajv oracle for semio JSON schemas: Ajv in strict mode with the `x-semio-*` vendor annotation vocabulary
 * (`../../🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json`) registered as annotation keywords, each with its value's meta-schema. Every schema law builds its validator here
 * instead of registering vendor keywords one call site at a time, so a schema that gains an annotation (e.g. `x-semio-note` in
 * `📇️directory/🧬️schema/🔣️.json`) keeps compiling everywhere, and a keyword outside the vocabulary still fails
 * (ticket 26/09/23 S15). https://ajv.js.org/strict-mode.html#prohibit-ignored-keywords */
import Ajv, { type Options } from "ajv";
import vocabulary from "../../🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json" with { type: "json" };

export const SEMIO_SCHEMA_VENDOR_VOCABULARY_V1 = vocabulary;

/** 🧬️ A strict Ajv (`options` override the defaults) that knows every declared `x-semio-*` annotation. */
export function semioSchemaAjvV1(options: Options = {}): Ajv {
  const ajv = new Ajv({ strict: true, ...options });
  for (const [keyword, metaSchema] of Object.entries(vocabulary.keywords)) ajv.addKeyword({ keyword, metaSchema });
  return ajv;
}
