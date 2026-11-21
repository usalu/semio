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
});


test("independent JSON export of authored polygons retains native binary32 widths and asset bytes",()=>{
  const law=JSON.parse(readFileSync(join(import.meta.dir, "../../🚪️io/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).authoredJson;
  const source=structuredClone(law.source);
  const geometry=new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(source.vertices.flat(),3));
  source.textures.tex.bytes=Array.from({length:law.textureRepeat},()=>source.textures.tex.bytes).flat();
  const decoded=JSON.parse(JSON.stringify(source));
  expect(decoded.vertices[0][0]).toBe(law.coordinate);
  expect(Math.fround(decoded.vertices[0][0])).toBe(geometry.getAttribute("position").getX(0));
  for(const key of ["faces","attributes","materials","surface"])expect(decoded[key]).toEqual(law.source[key]);
  expect(decoded.textures.tex.bytes).toHaveLength(1536);
  expect(decoded.textures.tex.bytes.slice(-3)).toEqual(law.source.textures.tex.bytes);
});
