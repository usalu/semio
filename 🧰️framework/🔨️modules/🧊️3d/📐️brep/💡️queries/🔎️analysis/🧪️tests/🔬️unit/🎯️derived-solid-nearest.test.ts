import { expect, test } from "bun:test";
import { Box3, BoxGeometry, BufferGeometry, CircleGeometry, CylinderGeometry, Matrix4, SphereGeometry, Triangle, Vector3 } from "three";
import fixture from "../🧫️fixtures/derived-solid-nearest.json";

const point = (axes: number[]) => new Vector3().fromArray(axes);
const sphere = (radius: number, origin: number[]) => new SphereGeometry(radius,128,96)
  .translate(origin[0],origin[1],origin[2]);
function bounds(geometries: BufferGeometry[], expected: number[][]) {
  const union = new Box3();
  for (const geometry of geometries) {
    geometry.computeBoundingBox();
    union.union(geometry.boundingBox!);
  }
  expect(union.min.distanceTo(point(expected[0]))).toBeLessThan(0.002);
  expect(union.max.distanceTo(point(expected[1]))).toBeLessThan(0.002);
}

function nearest(geometries: BufferGeometry[], target: Vector3) {
  let distance = Infinity;
  const triangle = new Triangle();
  const corner = new Vector3();
  for (const shape of geometries) {
    const vertices = shape.getAttribute("position"), index = shape.getIndex()!;
    for (let i=0; i<index.count; i+=3) {
      triangle.a.fromBufferAttribute(vertices,index.getX(i));
      triangle.b.fromBufferAttribute(vertices,index.getX(i+1));
      triangle.c.fromBufferAttribute(vertices,index.getX(i+2));
      distance = Math.min(distance, triangle.closestPointToPoint(target,corner).distanceTo(target));
    }
  }
  return distance;
}

test("Three.js reproduces both exterior and cavity boundary distances", () => {
  const {sphereShell,boxShell}=fixture;
  const box = new BoxGeometry(boxShell.size[0],boxShell.size[1],boxShell.size[2])
    .translate(boxShell.origin[0]+boxShell.size[0]/2,boxShell.origin[1]+boxShell.size[1]/2,boxShell.origin[2]+boxShell.size[2]/2);
  for (const [shape, outside] of [[sphereShell,sphere(sphereShell.outerRadius,sphereShell.outer)],[boxShell,box]] as const) {
    const inside = sphere(shape.innerRadius,shape.inner);
    bounds([outside,inside],shape.bounds);
    for (const row of shape.queries) {
      const target=point(row.target);
      expect(Math.abs(nearest([outside,inside],target)-target.distanceTo(point(row.point)))).toBeLessThan(0.001);
    }
    outside.dispose();
    inside.dispose();
  }
});

test("Three.js reproduces nearest distances to a spherical band and frustum", () => {
  const {band,frustum}=fixture;
  const [low,high]=band.span;
  const polar0=Math.acos(high/band.radius),polar1=Math.acos(low/band.radius);
  const skin=new SphereGeometry(band.radius,128,64,0,Math.PI*2,polar0,polar1-polar0).rotateX(Math.PI/2);
  const [lower,upper]=[low,high].map(z=>new CircleGeometry(Math.sqrt(band.radius**2-z*z),128).translate(0,0,z));
  const cone = new CylinderGeometry(frustum.radii[1],frustum.radii[0],frustum.height,128)
    .rotateX(Math.PI/2).translate(0,0,frustum.height/2);
  for (const [shape, geometries] of [[band,[skin,lower,upper]],[frustum,[cone]]] as const) {
    bounds(geometries,shape.bounds);
    for (const row of shape.queries) {
      const target=point(row.target);
      expect(Math.abs(nearest(geometries,target)-target.distanceTo(point(row.point)))).toBeLessThan(0.003);
    }
    if (shape === frustum) {
      const {axes,origin,bounds: expected}=frustum.placed;
      const pose=new Matrix4().makeBasis(point(axes[0]),point(axes[1]),point(axes[2])).setPosition(point(origin));
      const placed=cone.clone().applyMatrix4(pose);
      bounds([placed],expected);
      placed.dispose();
    }
    geometries.forEach(geometry=>geometry.dispose());
  }
});
