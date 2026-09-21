/** 🏷️ The TypeScript half of the shared `LocalizedLabel` contract.
 *
 * Reads the SAME `🌐️locale/🧫️fixtures/🏷️localized-label/🔣️.json` and `🌐️locale/🧬️schema/🔣️.json` the Rust
 * decoder reads in `🌐️locale/🧪️tests/🏷️localized-label-fixture`, so the two implementations cannot
 * drift: AJV is the schema oracle, `historyEntryLabelText` is the resolver, and every expectation is
 * keyed `<terminology>.<locale>` out of the generated axes rather than spelled per language here.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { SHELL_LOCALES, SHELL_TERMINOLOGIES } from "../../../🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts";
import { historyEntryLabelText, type LocalizedLabel } from "../../🟦️.ts";

/** 📄️ `fileURLToPath`, never `new URL(...).pathname`: the latter percent-encodes every emoji segment. */
const read = (relative: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8")) as unknown;

const fixture = read("../../../../🛍️products/💻️os/🔨️modules/🌐️locale/🧫️fixtures/🏷️localized-label/🔣️.json") as {
  readonly axes: { readonly terminologies: readonly string[]; readonly locales: readonly string[] };
  readonly rows: readonly { readonly id: string; readonly wire: unknown; readonly resolve: Readonly<Record<string, string>> }[];
  readonly refusals: readonly { readonly id: string; readonly wire: unknown; readonly schemaError: string; readonly resolve: Readonly<Record<string, string>> }[];
};
const schema = read("../../../../🛍️products/💻️os/🔨️modules/🌐️locale/🧬️schema/🔣️.json") as object;

const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

const assertResolves = (id: string, wire: unknown, expectations: Readonly<Record<string, string>>): void => {
  let seen = 0;
  for (const terminology of SHELL_TERMINOLOGIES) {
    for (const locale of SHELL_LOCALES) {
      const key = `${terminology}.${locale}`;
      const expected = expectations[key];
      expect(expected, `row '${id}' has no expectation for '${key}' — a new axis needs a translated cell`).toBeDefined();
      expect(historyEntryLabelText(wire as LocalizedLabel, terminology, locale), `row '${id}' cell '${key}'`).toBe(expected);
      seen += 1;
    }
  }
  expect(Object.keys(expectations)).toHaveLength(seen);
};

describe("LocalizedLabel shared fixture", () => {
  it("declares exactly the generated axes", () => {
    expect(fixture.axes.terminologies).toEqual([...SHELL_TERMINOLOGIES]);
    expect(fixture.axes.locales).toEqual([...SHELL_LOCALES]);
  });

  it("carries a schema twin that requires every generated axis and admits no other", () => {
    const document = schema as { required: string[]; additionalProperties: boolean; properties: Record<string, unknown>; definitions: { localeRow: { required: string[]; additionalProperties: boolean; properties: Record<string, unknown> } } };
    expect(document.required).toEqual([...SHELL_TERMINOLOGIES]);
    expect(document.additionalProperties).toBe(false);
    expect(Object.keys(document.properties)).toEqual([...SHELL_TERMINOLOGIES]);
    expect(document.definitions.localeRow.required).toEqual([...SHELL_LOCALES]);
    expect(document.definitions.localeRow.additionalProperties).toBe(false);
    expect(Object.keys(document.definitions.localeRow.properties)).toEqual([...SHELL_LOCALES]);
  });

  it("accepts every complete row and resolves the cells the fixture declares", () => {
    expect(fixture.rows.length).toBeGreaterThan(0);
    for (const row of fixture.rows) {
      expect(validate(row.wire), `row '${row.id}': ${JSON.stringify(validate.errors)}`).toBe(true);
      assertResolves(row.id, row.wire, row.resolve);
    }
  });

  it("refuses an incomplete or unknown-axis carrier and resolves its absent cells empty, never to another locale", () => {
    expect(fixture.refusals.length).toBeGreaterThan(0);
    for (const row of fixture.refusals) {
      expect(validate(row.wire), `refusal '${row.id}' must be refused by the schema`).toBe(false);
      expect((validate.errors ?? []).map((error) => error.message)).toContain(row.schemaError);
      assertResolves(row.id, row.wire, row.resolve);
    }
  });
});
