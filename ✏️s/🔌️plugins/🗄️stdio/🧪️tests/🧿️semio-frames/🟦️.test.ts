/** 🧿️ The six `stdio/semio` kit and object frame schemas agree with their TypeScript parsers, the GraphQL mirror and the grammar mirror, judged by Ajv on language-agnostic fixture instances.
 * @see ../🧫️fixtures/🧿️semio-frames/🔣️.json
 */
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { parseSemioKitSnapshotPackHeader } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/💾️binary/📸️snapshot/🟦️.ts";
import { parseSemioKitDiffFrame } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/💾️binary/🔺️diff/🟦️.ts";
import { parseSemioKitSnapshotDsl } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/📝️text/📸️snapshot/🟦️.ts";
import { parseSemioKitDiffDsl } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/📝️text/🔺️diff/🟦️.ts";
import { parseSemioObjectDiffFrame } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/💾️binary/🔺️diff/🟦️.ts";
import { parseSemioObjectDiffDsl } from "../../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/📝️text/🔺️diff/🟦️.ts";

type Frame = {
  readonly id: string;
  readonly folder: string;
  readonly title: string;
  readonly parser: string;
  readonly facet: "binary" | "text";
  readonly cases: readonly { readonly name: string; readonly instance: unknown; readonly valid: boolean }[];
};
type Fixture = { readonly schema: string; readonly subsetsRoot: string; readonly frames: readonly Frame[] };

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧿️semio-frames/🔣️.json", import.meta.url), "utf8")) as Fixture;
const parsers: Readonly<Record<string, (value: unknown) => unknown>> = {
  parseSemioKitSnapshotPackHeader,
  parseSemioKitDiffFrame,
  parseSemioKitSnapshotDsl,
  parseSemioKitDiffDsl,
  parseSemioObjectDiffFrame,
  parseSemioObjectDiffDsl,
};

/** 🔍️ Reads a sibling file of a frame schema, relative to the facet folder. */
function sibling(frame: Frame, name: string): string {
  return readFileSync(new URL(`../../${fixture.subsetsRoot}/${frame.folder}/${name}`, import.meta.url), "utf8");
}

test("the fixture declares its schema", () => {
  expect(fixture.schema).toBe("semio.stdio.semio-frames/v1");
  expect(fixture.frames.length).toBe(6);
});

for (const frame of fixture.frames) {
  const schema = JSON.parse(sibling(frame, "🔣️.json")) as { title: string; properties: Record<string, unknown>; required?: string[] };

  test(`${frame.id}: every property is mandatory and the title names the parser export`, () => {
    expect(schema.title).toBe(frame.title);
    expect([...(schema.required ?? [])].sort()).toEqual(Object.keys(schema.properties).sort());
    expect(frame.parser).toBe(`parse${frame.title}`);
  });

  test(`${frame.id}: the GraphQL mirror declares exactly the mandatory fields non-null`, () => {
    const graphql = sibling(frame, "🔗️.graphql");
    const fields = [...graphql.matchAll(/(\w+):\s*(\w+)(!?)/gu)].map((match) => [match[1], match[3]]);
    expect(fields.map(([name]) => name).sort()).toEqual(Object.keys(schema.properties).sort());
    expect(fields.every(([, bang]) => bang === "!")).toBe(true);
  });

  if (frame.facet === "binary") {
    test(`${frame.id}: the Kaitai mirror lists every mandatory field as an unconditional sequence member`, () => {
      const ksy = sibling(frame, "🥋️.ksy");
      const sequence = ksy.slice(ksy.indexOf("seq:"));
      const ids = [...sequence.matchAll(/- id: (\w+)/gu)].map((match) => match[1]!.replace(/_(\w)/gu, (_, letter: string) => letter.toUpperCase()));
      for (const name of schema.required ?? []) expect(ids).toContain(name);
      expect(sequence.includes("if:")).toBe(false);
    });
  }

  for (const row of frame.cases) {
    test(`${frame.id}: ${row.name}`, () => {
      const ajv = new Ajv({ strict: false });
      const validate = ajv.compile(schema);
      expect(validate(row.instance)).toBe(row.valid);
      const parse = parsers[frame.parser]!;
      if (row.valid) expect(parse(row.instance)).toEqual(row.instance);
      else expect(() => parse(row.instance)).toThrow();
    });
  }
}
