import Ajv from "ajv";
import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

type Edit = Readonly<{ id: string; actor: string; logicalMs: number; mutationIds: readonly string[] }>;
type Expect = Readonly<{ applied: readonly string[]; redo: readonly string[] }>;
type TransitionStep = Readonly<{ kind: "revert" | "reinstate"; actor: string; mutationIds: readonly string[]; logicalMs: number; id: string }>;
type Step =
  | (TransitionStep & { id?: string })
  | Readonly<{ kind: "expect"; expect: Expect; label?: string }>
  | Readonly<{ kind: "reload" | "hub-restart"; label?: string }>;
type Fixture = Readonly<{
  schema: string;
  version: number;
  documentId: string;
  edits: readonly Edit[];
  steps: readonly Step[];
  observations: readonly string[];
}>;

/** ♻️ Pure fold twin of Rust `fold_history` for revert/reinstate durability scenarios. */
function foldHistory(edits: readonly Edit[], transitions: readonly TransitionStep[]): { applied: string[]; redo: string[] } {
  const owners = new Map<string, string>();
  const authors = new Map<string, string>();
  for (const edit of edits) {
    authors.set(edit.id, edit.actor);
    for (const mutationId of edit.mutationIds) owners.set(mutationId, edit.id);
  }
  type Event =
    | { key: readonly [number, string]; kind: "edit"; edit: Edit }
    | { key: readonly [number, string]; kind: "transition"; transition: TransitionStep };
  const events: Event[] = [
    ...edits.map((edit) => ({ key: [edit.logicalMs, edit.id] as const, kind: "edit" as const, edit })),
    ...transitions.map((transition) => ({ key: [transition.logicalMs, transition.id] as const, kind: "transition" as const, transition })),
  ];
  events.sort((left, right) => left.key[0] - right.key[0] || left.key[1].localeCompare(right.key[1]));
  const active = new Set<string>();
  const redo: string[] = [];
  const owned = (mutationIds: readonly string[]): string[] => {
    const ids: string[] = [];
    for (const mutationId of mutationIds) {
      const editId = owners.get(mutationId);
      if (!editId) throw new Error(`unknown operation ${mutationId}`);
      if (!ids.includes(editId)) ids.push(editId);
    }
    return ids;
  };
  for (const event of events) {
    if (event.kind === "edit") {
      active.add(event.edit.id);
      for (let index = redo.length - 1; index >= 0; index -= 1) {
        if (authors.get(redo[index]!) === event.edit.actor) redo.splice(index, 1);
      }
    } else if (event.transition.kind === "revert") {
      for (const editId of owned(event.transition.mutationIds)) {
        if (active.delete(editId)) redo.push(editId);
      }
    } else {
      for (const editId of owned(event.transition.mutationIds)) {
        const position = redo.indexOf(editId);
        if (position >= 0) {
          redo.splice(position, 1);
          active.add(editId);
        }
      }
    }
  }
  const applied = edits
    .filter((edit) => active.has(edit.id))
    .sort((left, right) => left.logicalMs - right.logicalMs || left.id.localeCompare(right.id))
    .map((edit) => edit.id);
  return { applied, redo: [...redo] };
}

/** 🗄️ Validates the durable collaborative redo corpus (Ajv) and executes its fold scenario. */
export async function registerTests(vitest: NonNullable<ImportMeta["vitest"]>, source: TestSource): Promise<void> {
  const { describe, expect, it } = vitest;
  const root = dirname(fileURLToPath(source.url));
  const [fixtureRaw, schemaRaw] = await Promise.all([
    readFile(join(root, "🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1/🔣️.json"), "utf8").catch(() =>
      readFile(join(root, "🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1/🔣️.json"), "utf8"),
    ),
    readFile(join(root, "🔗️causal/🧬️schema/🗄️durable-collaborative-redo-v1/🔣️.json"), "utf8"),
  ]);
  const fixture = JSON.parse(fixtureRaw) as Fixture;
  const schema = JSON.parse(schemaRaw);
  const ajv = new Ajv({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);

  describe("durable collaborative redo fixture", () => {
    it("matches its JSON Schema", () => {
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.schema).toBe("semio.history.durable-collaborative-redo.v1");
      expect(fixture.observations).toContain("durable-collaborative-redo");
      expect(fixture.observations).toContain("survives-hub-restart");
    });

    it("folds selective undo/redo across reload and hub-restart", () => {
      const transitions: TransitionStep[] = [];
      for (const step of fixture.steps) {
        if (step.kind === "revert" || step.kind === "reinstate") {
          transitions.push({
            kind: step.kind,
            actor: step.actor,
            mutationIds: step.mutationIds,
            logicalMs: step.logicalMs,
            id: ("id" in step && step.id) || `${step.kind}-${step.logicalMs}`,
          });
        } else if (step.kind === "expect") {
          const fold = foldHistory(fixture.edits, transitions);
          expect(fold.applied, step.label).toEqual([...step.expect.applied]);
          expect(fold.redo, step.label).toEqual([...step.expect.redo]);
        } else {
          const first = foldHistory(fixture.edits, transitions);
          const second = foldHistory(fixture.edits, transitions);
          const label = step.kind === "reload" || step.kind === "hub-restart" ? step.label : undefined;
          expect(second, label).toEqual(first);
        }
      }
    });
  });
}
