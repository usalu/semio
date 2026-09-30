import Ajv from "ajv";
import fc from "fast-check";

type TestSource = { readonly directory: string; readonly url: string };

/** ✏️ Runs the language-agnostic supersede fold steps (`🔗️causal/🧫️fixtures/🧫️supersede-fold`) through the TypeScript
 * fold twin — the same expectations the Rust `fold_history` meets — after validating them against their schema (Ajv),
 * and checks with fast-check that the resolved supersessions do not depend on the order the events arrive in. */
export async function registerSupersedeFoldTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "foldSupersessions">, source: TestSource): Promise<void> {
  const { foldSupersessions } = dependencies;
  const { describe, expect, it } = vitest;

  type Json = Readonly<Record<string, any>>;
  type Event = Parameters<typeof foldSupersessions>[2][number];

  const hex = (value: string): number[] => Array.from({ length: value.length / 2 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
  const replacement = (value: Json) => (value.kind === "input" ? { kind: "input" as const, schema: value.schema as string, payload: hex(value.payloadHex) } : { kind: "withdrawn" as const });
  const clock = (logicalMs: number) => ({ actor: 0, physical_ms: logicalMs, logical: 0 });
  const event = (step: Json): Event => {
    const transition = step.transition as Json;
    const shape =
      transition.kind === "supersede"
        ? { kind: "supersede" as const, scope: transition.scope as string | null, inputs: (transition.inputs as Json[]).map((input) => ({ target: input.target as string, replacement: replacement(input.replacement) })) }
        : transition.kind === "branch"
          ? { kind: "branch" as const, alternativeId: transition.alternativeId as string }
          : transition.kind === "checkout"
            ? { kind: "checkout" as const, alternativeId: transition.alternativeId as string | null }
            : { kind: transition.kind as "revert" | "reinstate" | "commit" | "repin" };
    return { id: step.id, actor: step.actor, timestamp: clock(step.logicalMs), transition: shape };
  };
  const project = (fold: ReturnType<typeof foldSupersessions>) => ({
    alternative: fold.alternative,
    supersessions: [...fold.supersessions.entries()]
      .sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))
      .map(([target, supersession]) => ({ target, transitionId: supersession.transitionId, actor: supersession.actor, logicalMs: supersession.timestamp.physical_ms, scope: supersession.scope, replacement: supersession.replacement })),
  });
  const expected = (expect: Json) => ({ alternative: expect.alternative, supersessions: (expect.supersessions as Json[]).map((row) => ({ ...row, replacement: replacement(row.replacement) })) });

  async function load(): Promise<Readonly<{ fixture: Json; schema: object }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([readFile(join(root, "🔗️causal/🧫️fixtures/🧫️supersede-fold/🔣️.json"), "utf8"), readFile(join(root, "🔗️causal/🧬️schema/🔣️supersede-fold/🔣️.json"), "utf8")]);
    return { fixture: JSON.parse(fixture) as Json, schema: JSON.parse(schema) as object };
  }

  describe("supersede fold twin", () => {
    it("meets every language-agnostic fold step", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      const operations = new Set<string>((fixture.edits as Json[]).flatMap((edit) => edit.mutationIds as string[]));
      const document = fixture.documentId as string;
      expect(foldSupersessions(document, operations, []).trunk).toBe(fixture.trunkAlternativeId);
      const events: Event[] = [];
      for (const step of fixture.steps as Json[]) {
        if (step.kind === "transition") events.push(event(step));
        else if (step.kind === "expect") expect(project(foldSupersessions(document, operations, events)), step.label).toEqual(expected(step.expect));
        else if (step.kind === "reload") expect(project(foldSupersessions(document, operations, events)), step.label).toEqual(project(foldSupersessions(document, operations, events)));
        else if (step.kind === "refuse") expect(() => foldSupersessions(document, operations, [...events, event(step.transition)]), step.label).toThrow(step.detail);
        else throw new Error(`unknown step kind ${step.kind}`);
      }
    });

    it("resolves the same supersessions for every arrival order", async () => {
      const { fixture } = await load();
      const operations = new Set<string>((fixture.edits as Json[]).flatMap((edit) => edit.mutationIds as string[]));
      const document = fixture.documentId as string;
      const events = (fixture.steps as Json[]).filter((step) => step.kind === "transition").map(event);
      const reference = project(foldSupersessions(document, operations, events));
      expect(reference.supersessions.length).toBeGreaterThan(0);
      fc.assert(fc.property(fc.shuffledSubarray(events, { minLength: events.length, maxLength: events.length }), (shuffled) => {
        expect(project(foldSupersessions(document, operations, shuffled))).toEqual(reference);
      }));
    });
  });
}
