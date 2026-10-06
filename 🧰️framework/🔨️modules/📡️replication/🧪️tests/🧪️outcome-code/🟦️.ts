import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

/** 📖️ Validates the language-agnostic outcome-code vocabulary (`🎮️mutation/🧫️fixtures/🧫️outcome-code`) against its schema
 * (Ajv) and checks the TypeScript twin `outcomeCodeLevel` admits exactly its codes at exactly their levels. */
export async function registerOutcomeCodeTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "OUTCOME_CODES" | "APPLY_OUTCOME_CODE_PREFIX" | "outcomeCodeLevel">, source: TestSource): Promise<void> {
  const { OUTCOME_CODES, APPLY_OUTCOME_CODE_PREFIX, outcomeCodeLevel } = dependencies;
  const { describe, expect, it } = vitest;

  type Json = Readonly<Record<string, any>>;

  async function load(): Promise<Readonly<{ fixture: Json; schema: Json }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([readFile(join(root, "🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json"), "utf8"), readFile(join(root, "🎮️mutation/🧬️schema/🔣️outcome-code/🔣️.json"), "utf8")]);
    return { fixture: JSON.parse(fixture) as Json, schema: JSON.parse(schema) as Json };
  }

  describe("outcome code vocabulary", () => {
    it("validates each actual outcome entry and reproduces the vocabulary", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      for (const row of fixture.codes as Json[]) expect(validate(row), JSON.stringify(validate.errors)).toBe(true);
      expect(OUTCOME_CODES.map(([code, level]) => ({ code, level }))).toEqual((fixture.codes as Json[]).map((row) => ({ code: row.code, level: row.level })));
      for (const row of fixture.codes as Json[]) expect(outcomeCodeLevel(row.code), row.code).toBe(row.level);
    });

    it("admits the apply family at fatal and refuses every rejected code", async () => {
      const { fixture } = await load();
      expect(fixture.apply.prefix).toBe(APPLY_OUTCOME_CODE_PREFIX);
      const pattern = new RegExp(fixture.apply.pattern);
      for (const code of fixture.apply.accepted as string[]) {
        expect(pattern.test(code), code).toBe(true);
        expect(outcomeCodeLevel(code), code).toBe(fixture.apply.level);
      }
      for (const code of fixture.rejected as string[]) {
        expect(outcomeCodeLevel(code), code).toBeNull();
        expect((fixture.codes as Json[]).some((row) => row.code === code) || pattern.test(code), code).toBe(false);
      }
    });
  });
}
