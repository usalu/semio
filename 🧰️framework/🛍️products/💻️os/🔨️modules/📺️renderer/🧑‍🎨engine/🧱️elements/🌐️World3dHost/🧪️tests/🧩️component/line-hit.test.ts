import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, LineBasicMaterial, LineSegments, Raycaster, Vector3 } from "three";
import { world3dComponentInteractionTarget, world3dLinePick } from "../../🔗️instance-selection/🟦️.ts";
import type { WorldInstanceRecord, WorldMeshRecord } from "../../🔭️scene-schema/🟦️.ts";
import fixture from "./line-hit.json";

test("analytic wires hover their exact edge in component mode", () => {
    const geometry = new BufferGeometry();
    const material = new LineBasicMaterial();
    try {
        geometry.setAttribute("position", new Float32BufferAttribute(fixture.positions, 3));
        const segments = new LineSegments(geometry, material);
        const source = { handle: "a".repeat(64), revision: "b".repeat(64) };
        const instances = [{ id: "node@wire#0", meshId: "wire", componentSource: source }] as WorldInstanceRecord[];
        const meshes = [{ id: "wire", data: { componentReferences: {edge: fixture.labels} } }] as WorldMeshRecord[];
        for (const row of fixture.cases) {
            const ray = new Raycaster(new Vector3(1, row.y, 5), new Vector3(0, 0, -1));
            ray.params.Line = { threshold: 0.1 };
            const intersection = ray.intersectObject(segments)[0];
            expect(intersection?.index).toBe(row.y);
            const pick = world3dLinePick(fixture.edgeIds, intersection?.index, row.edges, row.whole);
            expect(pick ?? null).toEqual(row.hit);
            const target = pick?.mode === "edge" ? world3dComponentInteractionTarget(instances, meshes, instances[0]!.id, "edge", pick.id) : undefined;
            expect(target?.id.split("~")[2] ?? null).toBe(row.label);
        }
        expect(world3dLinePick(fixture.edgeIds, undefined, true, false)).toBeUndefined();
        expect(world3dLinePick([], 0, true, false)).toBeUndefined();
    } finally {
        geometry.dispose();
        material.dispose();
    }
});
