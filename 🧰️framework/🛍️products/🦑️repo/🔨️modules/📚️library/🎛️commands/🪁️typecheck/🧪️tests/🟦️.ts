import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {compileJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readTypecheckUnitsSelectionV1,resolveTypecheckUnitsSelectionV1} from "../🟦️.ts";

const read = (name: string) => JSON.parse(readFileSync(join(import.meta.dir, "..", name), "utf8"));
describe("typecheck units selection", () => {
  const schema = read("🧬️schema/🎯️units.json"), cases = read("🧫️fixtures/🎯️units.json"), own = compileJsonSchemaSubset(schema), oracle = new Ajv2020({strict: false}).compile(schema);
  test("accepted words equal their selection, in the owned validator and in Ajv", () => {
    for (const row of cases.accepted) {expect(oracle(row.selection)).toBe(true); expect(own.validate(row.selection)).toEqual([]); expect(readTypecheckUnitsSelectionV1(row.words)).toEqual(row.selection);}
  });
  test("rejected words are refused by the parser, the owned validator and Ajv", () => {
    for (const words of cases.rejected) {
      expect(() => readTypecheckUnitsSelectionV1(words)).toThrow();
      const loose = {project: words[0], only: [], jobs: 1, replan: false};
      if (!(words[0] ?? "").endsWith(".json") || /^(?:[\\/~]|[A-Za-z]:|\.\.)/u.test(words[0] ?? "")) {expect(oracle(loose)).toBe(false); expect(own.validate(loose).length).toBeGreaterThan(0);}
    }
  });
  test("a configuration outside the workspace is refused", async () => {
    await expect(resolveTypecheckUnitsSelectionV1(import.meta.dir, ["../../../../../../../../../../../../../../tmp/none.json"])).rejects.toThrow();
  });
});
