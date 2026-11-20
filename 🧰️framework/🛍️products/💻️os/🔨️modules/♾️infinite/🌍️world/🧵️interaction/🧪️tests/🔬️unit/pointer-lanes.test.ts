import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, Points, PointsMaterial, Raycaster, Vector3 } from "three";
import fixture from "./pointer-lanes.json";

test("vertex picking reaches points in a scene without vortex geometry", () => {
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute([0,0,0], 3));
    const material = new PointsMaterial();
    try {
        const vertex = new Points(geometry, material);
        const vortex = new Points(geometry, material);
        vortex.position.z = 2;
        vortex.updateMatrixWorld();
        const ray = new Raycaster(new Vector3(0,0,5), new Vector3(0,0,-1));
        for (const [mode, granularity, vortices, expectedVortex] of fixture) {
            const choosePoint = mode === "select" && granularity === "vertex";
            if (!choosePoint) {
                expect(mode === "brush").toBe(expectedVortex);
                continue;
            }
            const hits = ray.intersectObjects(vortices ? [vertex, vortex] : [vertex]);
            expect(hits[0]?.object === vortex).toBe(expectedVortex);
        }
    } finally {
        geometry.dispose();
        material.dispose();
    }
});
