import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

/** 📋️ Validates the language-agnostic replay reports (`⚔️conflict/🧫️fixtures/🧫️replay-report`) against their schema
 * (Ajv) and checks the TypeScript finalize gate against the verdict the Rust `ReplayReport::blocks_finalize` meets. */
export async function registerReplayReportTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "replayReportBlocksFinalize" | "REPLAY_SEVERITIES">, source: TestSource): Promise<void> {
  const { replayReportBlocksFinalize, REPLAY_SEVERITIES } = dependencies;
  const { describe, expect, it } = vitest;

  type Json = Readonly<Record<string, any>>;

  async function load(): Promise<Readonly<{ fixture: Json; schema: Json }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([readFile(join(root, "⚔️conflict/🧫️fixtures/🧫️replay-report/🔣️.json"), "utf8"), readFile(join(root, "⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json"), "utf8")]);
    return { fixture: JSON.parse(fixture) as Json, schema: JSON.parse(schema) as Json };
  }

  describe("replay report", () => {
    it("gates finalize exactly like the language-agnostic verdicts", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(schema.definitions.Severity.enum).toEqual([...REPLAY_SEVERITIES]);
      for (const row of fixture.cases as Json[]) {
        expect(replayReportBlocksFinalize(row.report), row.id).toBe(row.expect.blocksFinalize);
        const worst = (row.report.outcomes as Json[]).reduce<string | null>((level, outcome) => (outcome.worst !== null && (level === null || REPLAY_SEVERITIES.indexOf(outcome.worst) > REPLAY_SEVERITIES.indexOf(level as never)) ? outcome.worst : level), null);
        expect(row.report.worst, row.id).toBe(worst);
      }
    });
  });
}
