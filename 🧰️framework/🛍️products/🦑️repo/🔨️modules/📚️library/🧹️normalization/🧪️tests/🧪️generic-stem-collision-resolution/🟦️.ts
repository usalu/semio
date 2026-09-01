//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import fastGlob from "fast-glob";
import { describe, expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { classifyPackageGlueContent } from "../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const vectors = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as {
  siblingCases: readonly { id: string; dir: string; implName: string; roleName: string }[];
  gluePurityCases: readonly { id: string; path: string }[];
  packageBoundaryHoistCases: readonly { id: string; packageDir: string; ownerTestPath: string }[];
};

/** 🧬️ Independent structural oracle for the fixture shape itself (a third-party JSON Schema
 * validator, matching the sibling `🧪️package-boundary-classification` suite's own convention). */
const schema = {
  $id: "https://semio.local/generic-stem-collision-resolution-vectors",
  type: "object",
  required: ["siblingCases", "gluePurityCases", "packageBoundaryHoistCases"],
  additionalProperties: false,
  properties: {
    packageBoundaryHoistCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "packageDir", "ownerTestPath"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          packageDir: { type: "string", minLength: 1 },
          ownerTestPath: { type: "string", minLength: 1 },
        },
      },
    },
    siblingCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "dir", "implName", "roleName"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          dir: { type: "string", minLength: 1 },
          implName: { type: "string", minLength: 1 },
          roleName: { type: "string", minLength: 1 },
        },
      },
    },
    gluePurityCases: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        required: ["id", "path"],
        additionalProperties: false,
        properties: {
          id: { type: "string", minLength: 1 },
          path: { type: "string", minLength: 1 },
        },
      },
    },
  },
};
const validator = new Ajv({ strict: true, allErrors: true });
const validateVectors = validator.compile(schema);
//#endregion 🧬️Contract

//#region 🧪️Collision
/** 🐙️ 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION §goal-collide: `canonicalFile`'s generic-stem
 * short-circuit (`🧹️normalization/🟦️.ts:3122-3124`) discards the leading role/target emoji on a
 * file whose trailing stem is generic (`component`, `index`, …) — so a `🦀️component.rs` and its
 * `🧪️component.rs` test sibling, or an `⌨️component.rs`/`🧊️component.rs` per-target pair, both
 * project to the same bare kind-only leaf and collide. The taxonomy fix is a DIRECTORY
 * disambiguator that already exists (`tests`, `wgpu-target`, `tui-target`) — every one of the 44
 * cases census'd in `📓️goal-collide-census.md` was hand-resolved by moving the role/target-tagged
 * sibling into that directory. This suite is the regression guard: on the pre-fix tree every
 * `siblingCases` row fails (both names still coexist in `dir`); post-fix, none do. */
describe("generic-stem collision resolution (26/08/17/END-TO-END-TAXONOMY-NORMALIZATION)", () => {
  test("fixture vectors satisfy the independent schema implementation", () => {
    expect(validateVectors(vectors), JSON.stringify(validateVectors.errors)).toBe(true);
    const ids = [...vectors.siblingCases, ...vectors.gluePurityCases, ...vectors.packageBoundaryHoistCases].map((row) => row.id);
    expect(new Set(ids).size).toBe(ids.length);
  });

  for (const row of vectors.siblingCases) test(row.id, async () => {
    const dir = resolve(repoRoot, row.dir);
    // 🥇️ primary oracle: a direct node:fs listing of the directory.
    const direct = new Set(readdirSync(dir));
    // 🥈️ independent oracle: a third-party glob library re-deriving the same listing from scratch.
    const globbed = new Set((await fastGlob("*", { cwd: dir, onlyFiles: false, dot: true })));
    expect(globbed).toEqual(direct);
    const bothPresent = direct.has(row.implName) && direct.has(row.roleName);
    expect(bothPresent, `${row.dir} still holds both ${row.implName} and ${row.roleName} directly — the collision is back`).toBe(false);
  });

  for (const row of vectors.gluePurityCases) test(row.id, () => {
    const content = readFileSync(resolve(repoRoot, row.path), "utf8");
    // 🐙️ the real production classifier, not a reimplementation — must not be "implementation".
    const role = classifyPackageGlueContent("rust", content, 32);
    expect(role, `${row.path} classifies as "${role}" — a struct/enum/trait/union/impl crept back into package glue, which packageImplementationDestination will hoist onto the owner's canonical slot`).toBe("declaration");
  });

  /** 🐙️ A stricter variant of the same defect: `📦️packages/🟦️typescript/📦️index.ts` (the package's
   * own implementation) and any OTHER generic/empty-stem TypeScript file inside that SAME package
   * boundary both classify `role: "implementation"` and both get unconditionally hoisted to
   * `${owner}/${kindOnly}` by `packageImplementationDestination` — with no check that the slot is
   * already claimed. Nesting the test into its own `🧪️tests/` subdirectory does not help here (unlike
   * the plain sibling cases above) because the hoist ignores the file's already-correct parent
   * placement once its own stem is empty/generic. The only fix available without a taxonomy.json
   * change is topological: keep any second implementation-role file OUT of the package boundary
   * entirely, e.g. at the package owner's own `🧪️tests/`. */
  for (const row of vectors.packageBoundaryHoistCases) test(row.id, async () => {
    const packageDir = resolve(repoRoot, row.packageDir);
    const direct = new Set(readdirSync(packageDir));
    const globbed = new Set(await fastGlob("*", { cwd: packageDir, onlyFiles: false, dot: true }));
    expect(globbed).toEqual(direct);
    const testEntry = [...direct].find((name) => name === "🧪️tests" || /^🧪️.*\.test\./u.test(name) || /^🧪️.*\.test$/u.test(name));
    expect(testEntry, `${row.packageDir} still holds a test entry (${testEntry}) inside the package boundary — it will hoist-collide with the package's own implementation`).toBeUndefined();
    expect(readdirSync(resolve(repoRoot, row.ownerTestPath).replace(/\/[^/]+$/u, "")), `${row.ownerTestPath}'s directory is missing`).toContain(row.ownerTestPath.split("/").pop()!);
  });
});
//#endregion 🧪️Collision
