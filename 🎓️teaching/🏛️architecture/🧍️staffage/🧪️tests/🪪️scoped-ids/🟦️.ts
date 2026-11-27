/** 🪪️ Scene-scoped ids of a model document stay valid `Slug`s of at most 64 characters, deterministic and unique.
 *
 * THE REFERENCES are Ajv with the `Slug` of the staffage schema and with the kernel schema `semio.model/v1`.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import type { ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { ID_MAX, scopedId } from "../../🔨️modules/🧊️volume/🪪️ids/🟦️.ts";
import { writeModel } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";

const read = (url: string) => JSON.parse(readFileSync(new URL(url, import.meta.url), "utf8"));
const staffage = read("../../🧬️schema/🔣️.json");
const isSlug = new Ajv({ strict: false }).compile({ $ref: "#/$defs/Slug", $defs: staffage.$defs });
const isModel = new Ajv({ strict: false, allErrors: true }).compile(read("../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🔣️.json"));

const words = ["reclaimed", "sandstone", "window", "surround", "mullion", "transom", "lintel", "keystone", "sill", "jamb"];
const name = (seed: number, length: number): string => {
  let out = "";
  for (let i = 0; out.length < length; i++) out += `${i === 0 ? "" : "-"}${words[(seed + i * 7) % words.length]}${(seed * 31 + i) % 10}`;
  return out.slice(0, length).replace(/-+$/u, "");
};

describe("🪪️ scoped ids", () => {
  it("is the plain prefix for a short id", () => expect(scopedId("s50-body", "torso")).toBe("s50-body-torso"));

  it("bounds long ids, keeps them readable, deterministic and distinct", () => {
    const scenes = ["s50-body", "s100-thermal", "s500-structure"];
    const made = new Map<string, string>();
    const ids = new Set<string>([name(1, 120), name(2, 120)]);
    for (let i = 0; i < 400; i++) ids.add(`${name(i % 7, 60)}-${i}`);
    for (let length = 1; length <= 120; length++) ids.add(name(length, length));
    for (const scene of scenes) {
      for (const id of ids) {
        const scoped = scopedId(scene, id);
        expect(scoped.length).toBeLessThanOrEqual(ID_MAX);
        expect(isSlug(scoped), scoped).toBe(true);
        expect(scopedId(scene, id)).toBe(scoped);
        expect(`${scene}-${id}`.startsWith(scoped.slice(0, 40))).toBe(true);
        const key = `${scene}\u0000${id}`;
        const clash = made.get(scoped);
        expect(clash === undefined || clash === key, `${scoped}: ${clash} and ${key}`).toBe(true);
        made.set(scoped, key);
      }
    }
    expect(`s50-body-${name(1, 120)}`.length).toBeGreaterThan(ID_MAX);
  });

  it("makes unique valid ids in the procedural and resolved model of a long-named asset", async () => {
    const asset = JSON.parse(readFileSync(`${STAFFAGE_ROOT}/🗂️catalog/🛋️furniture/operating-table/🔣️.json`, "utf8")).asset as Asset;
    const texts = [await writeModel(asset, { pose: asset.poses.find((p) => p.key !== true)!.id }), await writeModel(asset, { clip: asset.clips[0]!.id }), await writeModel(asset, { pose: asset.poses.find((p) => p.key !== true)!.id, resolved: true })];
    for (const text of texts) {
      const document = JSON.parse(text) as ModelDocument;
      expect(isModel(document), JSON.stringify(isModel.errors?.slice(0, 3))).toBe(true);
      for (const list of [document.nodes, document.solids, document.materials, document.meshes, document.skins]) {
        const ids = list.map((x) => x.id);
        expect(new Set(ids).size).toBe(ids.length);
        for (const id of ids) expect(id.length, id).toBeLessThanOrEqual(ID_MAX);
      }
    }
  });
});
