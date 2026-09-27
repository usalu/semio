/** 🧬️ Canonical mutation fixtures must agree with the public tagged wire schema. */
import { expect, it } from "bun:test";
import Ajv from "ajv";
import { fileURLToPath } from "node:url";
import aggregate from "../../🔣️.json";
import documentSchema from "../../../🔣️.json";
import fixture from "../../../../../🎨️style/🧬️schema/🧬️mutations/📝️update-text/🧫️fixtures/🔣️.json";

it("validates every canonical tagged mutation through the aggregate schema", async () => {
  const ajv = new Ajv({strict: false, validateFormats: false}).addSchema(documentSchema);
  const cwd = fileURLToPath(new URL("../../../../../", import.meta.url));
  for await (const path of new Bun.Glob("*/🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json").scan({cwd, absolute: true})) ajv.addSchema(await Bun.file(path).json());
  const validate = ajv.compile(aggregate);
  for (const edit of fixture.edits) expect(validate({mutation: "updateText", layerId: "text", ...edit})).toBe(true);
  for (const size of fixture.invalidSizes) expect(validate({mutation: "updateText", layerId: "text", content: "changed", size})).toBe(false);
  let count = 0;
  for await (const path of new Bun.Glob("*/🧫️fixtures/🧬️mutations/*/*/🦠️mutation/🔣️.json").scan({cwd, absolute: true})) {
    const mutation = await Bun.file(path).json();
    expect(validate(mutation), `${mutation.mutation}: ${ajv.errorsText(validate.errors)}`).toBe(true);
    expect(validate({...mutation, mutation: "unknown"})).toBe(false);
    expect(validate({...mutation, unexpected: true})).toBe(false);
    count += 1;
  }
  expect(count).toBe(aggregate.oneOf.length);
});
