import { expect, test } from "bun:test";
import { Vector4 } from "three";
import { calcNURBSDerivatives } from "three/addons/curves/NURBSUtils.js";
import fixtures from "../🧫️fixtures/rational-jets.json";

test("Three.js and the portable spline fixtures agree on the nonuniform jets", () => {
  for (const row of fixtures) {
    const controls = row.controls.map(([x,y,z],index) => new Vector4(x,y,z,row.weights[index]));
    for (const [index,t] of row.at.entries()) {
      const actual = calcNURBSDerivatives(row.degree,row.knots,controls,t,2);
      for (const [order,expected] of row.jets[index].entries()) {
        const coordinates = actual[order].toArray();
        for (const [axis,value] of expected.entries()) expect(coordinates[axis]).toBeCloseTo(value,10);
      }
    }
  }
});
