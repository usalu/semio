import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import ts from "typescript";
import { BackSide, DoubleSide, Mesh, MeshBasicMaterial, PlaneGeometry, Raycaster, Vector3 } from "three";
import fixture from "../🧫️fixtures/🎬️scene-grid/🔣️.json";

const source = ts.createSourceFile("Scene.tsx", readFileSync(new URL("../../🧱️elements/🎬️Scene/🟦️.tsx", import.meta.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
let side: typeof BackSide | typeof DoubleSide = BackSide;
let explicit = false;
function visit(node: ts.Node): void {
  if ((ts.isJsxSelfClosingElement(node) || ts.isJsxOpeningElement(node)) && node.tagName.getText(source) === "Grid") {
    const attribute = node.attributes.properties.find(item => ts.isJsxAttribute(item) && item.name.getText(source) === "side");
    explicit = attribute?.getText(source) === "side={THREE.DoubleSide}";
    if (explicit) side = DoubleSide;
  }
  ts.forEachChild(node, visit);
}
visit(source);

describe("scene floor grid", () => {
  test("declares the language-neutral double-sided surface contract", () => {
    expect(fixture.surface).toBe("double-sided");
    expect(explicit).toBe(true);
  });
  for (const camera of fixture.cameras) test(`remains visible from ${camera.name} using Three's geometry oracle`, () => {
    const geometry = new PlaneGeometry(100, 100).rotateX(Math.PI / 2);
    const material = new MeshBasicMaterial({ side });
    try {
      const mesh = new Mesh(geometry, material);
      mesh.updateMatrixWorld();
      const origin = new Vector3(...camera.position as [number, number, number]);
      const ray = new Raycaster(origin, origin.clone().negate().normalize());
      expect(ray.intersectObject(mesh).length > 0).toBe(camera.visible);
    } finally {
      geometry.dispose();
      material.dispose();
    }
  });
});
