import Ajv from "ajv";
import jsonSchema from "../../../🧬️schema/🔣️.json";
const validate=new Ajv({strict:false}).compile(jsonSchema);
/** 🧪️ Declared finite JSON inputs and actual Text examples retain complete canonical fields. */
import { describe, expect, test } from "bun:test";
import { block5dToJsonText } from "../../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block5dCanonicalJsonText,block5dFromJsonText } from "../../📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block5dFromDslText } from "../../📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️";

const FIXTURES = [
  { asset: "hexagonal-cut-concrete-forest-left", fixture: "⬅️hexagonal-cut-concrete-forest-left.json", example: "🌲️hexagonal-cut-concrete-forest-left", content: "🌲️hexagonal-cut-concrete-forest" },
  { asset: "nakagin-capsule", fixture: "🏢️nakagin-capsule.json", example: "🏢️nakagin-capsule", content: "🏢️nakagin-capsule" },
] as const;

async function read(path: string): Promise<string> {
  return Bun.file(new URL(path, import.meta.url)).text();
}

describe("block5d io", () => {
  for (const { asset, example, content, fixture } of FIXTURES) {
    test(`${asset}: declared JSON input retains complete canonical fields and stable emitted words`, async () => {
      const expected = await read(`../../🧫️fixtures/${fixture}`);
      expect(validate(JSON.parse(expected)),JSON.stringify(validate.errors)).toBe(true);
      const canonical=block5dCanonicalJsonText(expected);
      expect(validate(JSON.parse(canonical)),JSON.stringify(validate.errors)).toBe(true);
      expect(block5dCanonicalJsonText(canonical)).toBe(canonical);
      expect(block5dFromJsonText(canonical)).toEqual(block5dFromJsonText(expected));
    });

    test(`${asset}: actual Text example agrees with the declared JSON semantic input`, async () => {
      const expected = await read(`../../🧫️fixtures/${fixture}`);
      const dsl = await read(`../../../📚️examples/${example}/🖼️assets/${content}/🗣️.dsl.semio`);
      expect(block5dToJsonText(block5dFromDslText(dsl))).toBe(block5dCanonicalJsonText(expected));
    });
  }
});
