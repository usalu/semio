import assert from "node:assert/strict";
import Ajv from "ajv";
import { cross, det, dot, inv, multiply, norm, transpose } from "mathjs";
import fixture from "../../🧫️fixtures/🌀️geometric-cells/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🌀️geometric-cells/🔣️.json" with { type: "json" };

type GeometricCase = typeof fixture.cases[number];
type Matrix = number[][];
const zeros = (side: number): Matrix => Array.from({ length: side }, () => Array<number>(side).fill(0));

export function geometricMatrixOracle(row: GeometricCase): number[] {
  const p = row.properties;
  const u = row.displacement;
  if (row.kind === "tet4") {
    const affine = row.positions.map(position => [1, ...position]);
    const inverse = inv(affine) as Matrix;
    const gradients = row.positions.map((_, node) => inverse.slice(1).map(axis => axis[node]!));
    const strain = zeros(6).map(values => values.concat(Array<number>(6).fill(0)));
    for (let node = 0; node < 4; node++) {
      const [x, y, z] = gradients[node]!;
      strain[0]![3 * node] = x!; strain[1]![3 * node + 1] = y!; strain[2]![3 * node + 2] = z!;
      strain[3]![3 * node] = y!; strain[3]![3 * node + 1] = x!;
      strain[4]![3 * node + 1] = z!; strain[4]![3 * node + 2] = y!;
      strain[5]![3 * node] = z!; strain[5]![3 * node + 2] = x!;
    }
    const constitutive = zeros(6);
    const scale = p.e / ((1 + p.nu!) * (1 - 2 * p.nu!));
    for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) constitutive[i]![j] = scale * (i === j ? 1 - p.nu! : p.nu!);
    for (let i = 3; i < 6; i++) constitutive[i]![i] = p.e / (2 * (1 + p.nu!));
    const [xx, yy, zz, xy, yz, xz] = multiply(constitutive, multiply(strain, u)) as number[];
    const stress = [[xx!, xy!, xz!], [xy!, yy!, yz!], [xz!, yz!, zz!]];
    const blocks = multiply(multiply(gradients, stress), transpose(gradients)) as unknown as Matrix;
    const volume = Math.abs(det(affine)) / 6;
    const matrix = zeros(12);
    for (let i = 0; i < 4; i++) for (let j = 0; j < 4; j++) for (let axis = 0; axis < 3; axis++) matrix[3 * i + axis]![3 * j + axis] = volume * blocks[i]![j]!;
    return matrix.flat();
  }
  const delta = row.positions[1]!.map((value, axis) => value - row.positions[0]![axis]!);
  const length = norm(delta) as number;
  const direction = delta.map(value => value / length);
  const stride = row.kind === "frame3" ? 6 : 3;
  const axial = p.e * p.a! / length * (dot(direction, direction.map((_, axis) => u[stride + axis]! - u[axis]!)) as number);
  if (row.kind === "bar3") {
    const projector = zeros(3).map((values, i) => values.map((_, j) => (i === j ? 1 : 0) - direction[i]! * direction[j]!));
    const matrix = zeros(6);
    for (let i = 0; i < 6; i++) for (let j = 0; j < 6; j++) matrix[i]![j] = axial / length * (Math.floor(i / 3) === Math.floor(j / 3) ? 1 : -1) * projector[i % 3]![j % 3]!;
    return matrix.flat();
  }
  const reference = Math.abs(direction[2]!) > .99 ? [1, 0, 0] : [0, 0, 1];
  const lateral = cross(reference, direction) as number[];
  const y = lateral.map(value => value / (norm(lateral) as number));
  const z = cross(direction, y) as number[];
  const c = Math.cos(p.roll!), s = Math.sin(p.roll!);
  const axes = [direction, y.map((value, axis) => value * c + z[axis]! * s), z.map((value, axis) => value * c - y[axis]! * s)];
  const transform = zeros(12);
  for (let block = 0; block < 4; block++) for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) transform[3 * block + i]![3 * block + j] = axes[i]![j]!;
  const bending = zeros(4);
  for (const [point, weight] of [[-Math.sqrt(3 / 5), 5 / 9], [0, 8 / 9], [Math.sqrt(3 / 5), 5 / 9]]) {
    const t = (point! + 1) / 2;
    const derivatives = [(-6 * t + 6 * t * t) / length, 1 - 4 * t + 3 * t * t, (6 * t - 6 * t * t) / length, -2 * t + 3 * t * t];
    for (let i = 0; i < 4; i++) for (let j = 0; j < 4; j++) bending[i]![j] += axial * length * weight! / 2 * derivatives[i]! * derivatives[j]!;
  }
  const local = zeros(12);
  for (const [indices, signs] of [[[1, 5, 7, 11], [1, 1, 1, 1]], [[2, 4, 8, 10], [1, -1, 1, -1]]]) for (let i = 0; i < 4; i++) for (let j = 0; j < 4; j++) local[indices![i]!]![indices![j]!] = signs![i]! * signs![j]! * bending[i]![j]!;
  return (multiply(multiply(transpose(transform), local), transform) as unknown as Matrix).flat();
}

export function verifyGeometricCellOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const expected = geometricMatrixOracle(row);
    assert.equal(expected.length, row.matrix.length, row.id);
    for (let cell = 0; cell < expected.length; cell++) assert(Math.abs(expected[cell]! - row.matrix[cell]!) <= fixture.tolerance * Math.max(1, Math.abs(row.matrix[cell]!)), `${row.id} cell ${cell}`);
    const side = Math.sqrt(expected.length);
    for (let i = 0; i < side; i++) for (let j = 0; j < side; j++) assert(Math.abs(expected[i * side + j]! - expected[j * side + i]!) <= fixture.tolerance * Math.max(1, Math.abs(expected[i * side + j]!)), `${row.id} symmetry`);
  }
  console.log(`[DEBUG] ${fixture.cases.length} geometric states and ${fixture.cases.reduce((count, row) => count + row.matrix.length, 0)} cells match independent mathjs matrix contraction and Hermite quadrature`);
}
