import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, PerspectiveCamera, Points, PointsMaterial, Raycaster, Vector2, Vector3 } from "three";
import fixture from "./point-only.json";

test("visible unlabelled point lanes remain ordinary object pick targets", () => {
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(fixture.mesh.positions, 3));
    const material = new PointsMaterial();
    const camera = new PerspectiveCamera(fixture.camera.fov, 1, 0.1, 10);
    camera.position.fromArray(fixture.camera.position);
    camera.up.fromArray(fixture.camera.up);
    camera.lookAt(new Vector3().fromArray(fixture.camera.target));
    camera.updateMatrixWorld();
    try {
        const raycaster = new Raycaster();
        for (const variant of fixture.variants) {
            const points = new Points(geometry, material);
            points.name = fixture.instance;
            points.userData.vertexIds = variant.vertexIds;
            const [x,y] = fixture.pointer;
            raycaster.setFromCamera(new Vector2((x! / 400) * 2 - 1, 1 - (y! / 400) * 2), camera);
            const hit = raycaster.intersectObject(points)[0];
            expect(hit?.object.name).toBe(fixture.instance);
            expect(hit?.point.toArray()).toEqual(fixture.mesh.positions);
            expect(points.userData.vertexIds[hit!.index!] !== undefined).toBe(variant.semantic);
        }
    } finally {
        geometry.dispose();
        material.dispose();
    }
});
