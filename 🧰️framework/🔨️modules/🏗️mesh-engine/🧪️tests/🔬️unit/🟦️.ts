import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv/dist/2020";
import { BufferGeometry, Float32BufferAttribute } from "three";

test("original attributed mesh fixture agrees with independent metadata publication", () => {
  const base = join(import.meta.dir, "../🧫️fixtures/🧹️metadata");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  expect(new Ajv().validate(schema, fixture)).toBe(true);
  const mesh = fixture.mesh;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(mesh.positions, 3));
  geometry.setAttribute("normal", new Float32BufferAttribute(mesh.normals, 3));
  geometry.setIndex(mesh.indices);
  geometry.userData = { attributes: mesh.attributes, materials: mesh.materials, textures: mesh.textures };
  const wire = JSON.parse(JSON.stringify(geometry.toJSON()));
  for (const field of ["attributes", "materials", "textures"]) expect(wire.userData[field]).toEqual(mesh[field]);
  expect(geometry.getAttribute("position").count).toBe(1);
  geometry.dispose();
  console.log("[DEBUG] independent metadata publication attributes=1 materials=1 textures=1 vertices=1");
});
