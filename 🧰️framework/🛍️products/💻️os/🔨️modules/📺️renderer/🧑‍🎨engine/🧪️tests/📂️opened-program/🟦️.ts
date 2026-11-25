/** 📂️ Where an opened document's program is shown: the language-neutral cases of `📂️opened-program/🔣️.json`, validated by Ajv (third party) against their own schema and replayed against {@link openedProgramPublicationV1}. */
import { expect, it } from "vitest";
import Ajv from "ajv";
import { openedProgramPublicationV1 } from "../../🧱️elements/🏛️ShellHost/📂️opened-program/🟦️.ts";
import fixture from "../../🧱️elements/🏛️ShellHost/📂️opened-program/🔣️.json";

it("validates the opened-program cases with Ajv", () => {
  const { schema, ...document } = fixture;
  const ajv = new Ajv();
  expect(ajv.validate(schema, document), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, { ...document, cases: document.cases.map(entry => ({ ...entry, publication: "keep-both" })) })).toBe(false);
});
for (const entry of fixture.cases) it("decides " + entry.id, () => {
  expect(openedProgramPublicationV1(entry.hostMode, entry.primary, entry.opened)).toBe(entry.publication);
});
