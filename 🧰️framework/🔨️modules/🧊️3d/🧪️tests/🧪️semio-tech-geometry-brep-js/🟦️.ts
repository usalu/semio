import type * as brep from "../../🟦️.ts";
import { readFileSync } from "node:fs";
import { BufferGeometry, Float32BufferAttribute, Vector3 } from "three";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "isRenderableMeshTransfer" | "meshTransferToGeometryData" | "meshTransferFromPreviewPayload">, source: TestSource): Promise<void> {
  const { isRenderableMeshTransfer, meshTransferToGeometryData, meshTransferFromPreviewPayload } = dependencies;
  type MeshTransfer = brep.MeshTransfer;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/geometry-brep-js", () => {
    it("preserves only original topology vertex provenance", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json", source.url), "utf8"));
      for (const row of fixture.cases) {
        const labels = row.points.map((_: unknown, index: number) => (18446744073709551615n - BigInt(index)).toString());
        const raw = { position: [0,0,0,1,0,0,0,1,0], normal: [0,0,1,0,0,1,0,0,1], index: [0,1,2], edges: [], points: row.points.flat(), face_groups: [], edge_groups: [], vertex_groups: labels.map((label: string, index: number) => ({start:index,count:1,entity_id:label})) };
        const mesh = meshTransferFromPreviewPayload(raw)!;
        const output = meshTransferToGeometryData(mesh) as unknown as { readonly position: Float32Array; readonly vertexIds: readonly number[]; readonly componentReferences: Readonly<Record<string, readonly string[]>> };
        expect(output.vertexIds).toEqual([fixture.unselectable,fixture.unselectable,fixture.unselectable,...labels.map((_: string,index: number)=>index)]);
        expect(output.componentReferences.vertex).toEqual(labels);
        const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(output.position,3)).setIndex(raw.index);
        expect(geometry.getAttribute("position").count).toBe(3 + row.points.length);
        for (const [index,point] of row.points.entries()) expect(new Vector3().fromBufferAttribute(geometry.getAttribute("position"),3+index).distanceTo(new Vector3(...point))).toBe(0);
      }
    });
    it("preserves analytic face and edge picking references without narrowing labels", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json", source.url), "utf8"));
      const mesh = meshTransferFromPreviewPayload(fixture.transfer);
      expect(mesh).not.toBeNull();
      const output = meshTransferToGeometryData(mesh!) as unknown as { readonly faceIds: readonly number[]; readonly edgeIds: readonly number[]; readonly componentReferences: unknown };
      expect(output.faceIds).toEqual(fixture.expected.faceIds);
      expect(output.edgeIds).toEqual(fixture.expected.edgeIds);
      expect(output.componentReferences).toEqual(fixture.expected.componentReferences);
      const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(fixture.transfer.position, 3)).setIndex(fixture.transfer.index);
      for (const [id, group] of fixture.transfer.face_groups.entries()) geometry.addGroup(group.start, group.count, id);
      const faceIds: number[] = [];
      for (const group of geometry.groups) for (let index=group.start; index<group.start+group.count; index+=3) faceIds.push(group.materialIndex!);
      expect(faceIds).toEqual(fixture.expected.faceIds);
      let area=0;
      const position=geometry.getAttribute("position"), indices=geometry.getIndex()!;
      for (let index=0;index<indices.count;index+=3) {
        const [a,b,c]=[0,1,2].map(offset=>new Vector3().fromBufferAttribute(position,indices.getX(index+offset)));
        area+=new Vector3().subVectors(b!,a!).cross(new Vector3().subVectors(c!,a!)).length()/2;
      }
      expect(area).toBe(fixture.expected.area);
      geometry.dispose();
      console.log(`[DEBUG] BRep analytic preview references domains=2 independentThreeArea=${area}`);
    });
    it("rejects incomplete, overlapping and malformed analytic picking ranges", () => {
      const fixture=JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json",source.url),"utf8"));
      for(const row of fixture.refusals){
        const transfer=structuredClone(fixture.transfer);
        transfer[`${row.domain}_groups`][row.row][row.field]=row.value;
        const mesh=meshTransferFromPreviewPayload(transfer)!;
        expect(isRenderableMeshTransfer(mesh)).toBe(false);
        expect(meshTransferToGeometryData(mesh).position.length).toBe(0);
      }
    });
    it("isRenderableMeshTransfer accepts triangle meshes", () => {
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [{ start: 0, count: 3, entityId: "face-1" as brep.kernelGeometry.FaceRef }],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      expect(isRenderableMeshTransfer(mesh)).toBe(true);
    });
  });

}
