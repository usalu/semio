import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { expect, test } from "vitest";

const artifacts = fileURLToPath(new URL("../../🗿️artifacts/", import.meta.url));
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));

for (const dimension of ["◻️2d", "🧊️3d", "🖐️5d"]) {
  test(`${dimension} publishes every declared mutation schema`, () => {
    const subset = join(artifacts, dimension, "🏅️standards/🔖️1/🪆️subsets/✳️any");
    const schema = join(subset, "🧬️schema");
    const mutations = join(schema, "🧬️mutations");
    const union = read(join(mutations, "🔣️.json"));
    const payloads = readdirSync(mutations, { recursive: true, encoding: "utf8" })
      .filter(path => path.replaceAll("\\", "/").endsWith("/🧬️schema/🔣️.json"))
      .map(path => read(join(mutations, path)))
      .filter(payload => typeof payload.properties?.mutation?.const === "string");
    expect(union.oneOf.map((entry: { $ref: string }) => entry.$ref).sort()).toEqual(payloads.map(payload => payload.$id).sort());
    if (dimension === "🖐️5d") {
      const ajv = new Ajv({ strict: false });
      ajv.addSchema(read(join(schema, "🔣️.json")));
      for (const payload of payloads) ajv.addSchema(payload);
      const validate = ajv.compile(union);
      const fixtures = join(subset, "🧪️tests/🧫️fixtures");
      const scene = read(join(fixtures, "🔀️scene/🔣️.json"));
      const edits = scene.steps.map(({ forward }: { forward: Record<string, unknown> }) => {
        const { mutation, ...edit } = forward;
        expect(mutation).toBe("spliceSceneRows");
        return edit;
      });
      expect(validate({ mutation: "spliceSceneRowBatch", edits }), JSON.stringify(validate.errors)).toBe(true);
      expect(validate(read(join(fixtures, "🧬️mutations/🔀️splice-scene-row-batch/🚫️empty/🦠️mutation/🔣️.json")))).toBe(false);
    }
  });
}

type SceneTable = "parts" | "fasteners" | "kindCompatibility" | "targetVolumes";
type Splice = { table: SceneTable; start: number; deleteCount: number } & Partial<Record<SceneTable, readonly unknown[]>>;

function rowOperations(edit: Splice): Operation[] {
  const path = `/${edit.table}/${edit.start}`;
  const removes = Array.from({ length: edit.deleteCount }, (): Operation => ({ op: "remove", path }));
  const adds = (edit[edit.table] ?? []).map((value, offset): Operation => ({ op: "add", path: `/${edit.table}/${edit.start + offset}`, value }));
  return [...removes, ...adds];
}

test("5D authored scene edits reproduce the declared snapshots through JSON Patch", () => {
  const fixtures = join(artifacts, "🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures");
  const scene = read(join(fixtures, "🔀️scene/🔣️.json"));
  const steps: { forward: Splice; inverse: Splice }[] = scene.steps;
  const forward = steps.flatMap(step => rowOperations(step.forward));
  const inverse = steps.slice().reverse().flatMap(step => rowOperations(step.inverse));
  expect(applyPatch(scene.before, forward, true, false).newDocument).toEqual(scene.after);
  expect(applyPatch(scene.after, inverse, true, false).newDocument).toEqual(scene.before);

  const middle = join(fixtures, "🧬️mutations/🔀️splice-scene-rows/🔀️middle");
  const before = read(join(middle, "📸️snapshot/⬅️before/🔣️.json"));
  const after = read(join(middle, "📸️snapshot/➡️after/🔣️.json"));
  expect(applyPatch(before, rowOperations(read(join(middle, "🦠️mutation/🔣️.json"))), true, false).newDocument).toEqual(after);
});
