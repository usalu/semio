/** 🧪️ Dash input matches the language-neutral cases and independent Ajv grammar. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parseStrokeDash } from "../../🟦️.ts";
import cases from "../../🧫️fixtures/🔣️.json";
import schema from "../../../🧬️mutations/🧫️fixtures/🎛️field-patch/🧬️schema/🔣️.json";
test("stroke dashes accept editable patterns and reject invalid input", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  for (const entry of cases) {
    expect(validate({field:"strokeDash",value:entry.value})).toBe(!entry.error);
    if (entry.error) expect(() => parseStrokeDash(entry.value)).toThrow();
    else expect(parseStrokeDash(entry.value)).toEqual(entry.dash);
  }
});
