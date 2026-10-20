import { expect, test } from "bun:test";
import parser from "yargs-parser";
import catalogue from "../../🔣️catalog.json" with { type: "json" };
import { normalizeBuildOptionsV1 } from "../🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };

for (const row of fixture.cases) {
  test(`option normalization: ${row.name}`, () => {
    const once = normalizeBuildOptionsV1(row.args);
    expect(once).toEqual(row.expected);
    expect(normalizeBuildOptionsV1(once), "idempotent").toEqual(once);
  });
}

test("the catalogue declares every global enumeration with a required value", () => {
  for (const name of ["--tool", "--workers", "--lang"]) expect(catalogue.flags.find((flag) => flag.name === name)?.value, name).toBe("required");
});

test("an independent argument parser reads the typed and the normalized spelling alike", () => {
  const options = { string: fixture.oracle.strings, alias: fixture.oracle.aliases, configuration: { "parse-numbers": false, "camel-case-expansion": false } };
  for (const args of fixture.oracle.args) {
    const typed = parser([...args], options), normalized = parser(normalizeBuildOptionsV1(args), options);
    for (const key of fixture.oracle.strings) expect(normalized[key], `${key} in ${args.join(" ")}`).toEqual(typed[key]);
    expect(normalized._.map(String), args.join(" ")).toEqual(typed._.map(String));
  }
});
