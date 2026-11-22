import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, PerspectiveCamera, Points, PointsMaterial, Raycaster, Vector2, Vector3 } from "three";
import fixture from "./point-only.json";

test("an unlabelled point layer decodes to the expected point draw and stays an ordinary object pick target", () => {
    const bytes = Uint8Array.from(atob(fixture.layer.positionsB64), (character) => character.charCodeAt(0));
    const positions = Array.from(new Float32Array(bytes.buffer, bytes.byteOffset, bytes.byteLength / 4));
    expect(positions).toEqual(fixture.mesh.positions);
    expect([...positions, 1, 1, 1, 1]).toEqual(fixture.expected.vertices[0]!);
    const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(positions, 3));
    const material = new PointsMaterial({ size: fixture.layer.size, sizeAttenuation: fixture.layer.sizeAttenuation });
    const camera = new PerspectiveCamera(fixture.camera.fov, 1, 0.1, 10);
    camera.position.fromArray(fixture.camera.position);
    camera.up.fromArray(fixture.camera.up);
    camera.lookAt(new Vector3().fromArray(fixture.camera.target));
    camera.updateMatrixWorld();
    try {
        const points = new Points(geometry, material);
        points.name = fixture.instance;
        const [x, y] = fixture.pointer;
        const raycaster = new Raycaster();
        raycaster.setFromCamera(new Vector2((x! / 400) * 2 - 1, 1 - (y! / 400) * 2), camera);
        const hit = raycaster.intersectObject(points)[0];
        expect(hit?.object.name).toBe(fixture.instance);
        expect(hit?.point.toArray()).toEqual(fixture.mesh.positions);
    } finally {
        geometry.dispose();
        material.dispose();
    }
});
