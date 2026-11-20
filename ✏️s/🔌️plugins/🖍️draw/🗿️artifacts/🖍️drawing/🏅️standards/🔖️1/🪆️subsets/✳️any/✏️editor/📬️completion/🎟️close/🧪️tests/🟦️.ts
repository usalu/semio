/** 🧾️ Validates the neutral admission receipt shared with the native completion owner. */
import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import admission from "../📥️admit/📐️.json";
import schema from "../📥️admit/🔣️.json";

test("completion admission charges both the original source header and its new frame", () => {
  const validate = new Ajv2020().compile(schema);
  expect(validate(admission)).toBe(true);
  const { headerCopyIncludesSourceAndFrame, ...missingCopy } = admission;
  expect(validate(missingCopy)).toBe(false);
  expect(validate({ ...missingCopy, headerCopyIncludesSourceAndFrame: !headerCopyIncludesSourceAndFrame })).toBe(false);
});
