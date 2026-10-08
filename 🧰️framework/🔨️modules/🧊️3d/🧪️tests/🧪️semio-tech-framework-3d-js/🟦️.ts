import type * as brep from "../../🟦️.ts";
import { readFileSync } from "node:fs";
import Ajv from "ajv/dist/2020.js";
import union from "lodash/union.js";
import difference from "lodash/difference.js";
import xor from "lodash/xor.js";
import uniq from "lodash/uniq.js";
import { BufferGeometry, Float32BufferAttribute, OrthographicCamera, PerspectiveCamera, Ray, Triangle, Vector3 } from "three";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "isRenderableMeshTransfer" | "meshTransferToGeometryData" | "meshTransferFromPreviewPayload" | "mergeMeshTransfers">, source: TestSource): Promise<void> {
  const { isRenderableMeshTransfer, meshTransferToGeometryData, meshTransferFromPreviewPayload, mergeMeshTransfers } = dependencies;
  type MeshTransfer = brep.MeshTransfer;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/framework-3d-js", () => {
    it("preserves pure analytic wire points and edges with independent Three geometry", () => {
      const fixtureUrl = new URL("../🖱️ui/🎬️scene/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json",source.url);
      const fixture = JSON.parse(readFileSync(fixtureUrl,"utf8"));
      const transfer = meshTransferFromPreviewPayload(fixture.transfer)!;
      expect(isRenderableMeshTransfer(transfer)).toBe(true);
      const mesh = meshTransferToGeometryData(transfer);
      expect([...mesh.position]).toEqual(fixture.mesh.positions);
      expect([...mesh.edges]).toEqual(fixture.mesh.edgePositions);
      expect(mesh.vertexIds).toEqual(fixture.mesh.vertexIds);
      expect(mesh.edgeIds).toEqual(fixture.mesh.edgeIds);
      expect(mesh.index.length).toBe(fixture.expected.indices);
      const geometry = new BufferGeometry().setAttribute("position",new Float32BufferAttribute(mesh.position,3));
      expect(geometry.getAttribute("position").count).toBe(fixture.expected.vertices);
      let perimeter=0;
      for(let index=0;index<mesh.edges.length;index+=6)perimeter+=new Vector3().fromArray(mesh.edges,index).distanceTo(new Vector3().fromArray(mesh.edges,index+3));
      expect(perimeter).toBe(fixture.expected.perimeter);
      const edgeOnly = meshTransferToGeometryData(meshTransferFromPreviewPayload({...fixture.transfer,points:[],vertex_groups:[]})!);
      expect([...edgeOnly.edges]).toEqual(fixture.edgeOnly.mesh.edgePositions);
      expect(edgeOnly.vertexIds?.length ?? 0).toBe(fixture.edgeOnly.expected.topologyVertices);
      const edgeGeometry = new BufferGeometry().setAttribute("position",new Float32BufferAttribute(edgeOnly.edges,3));
      edgeGeometry.computeBoundingBox();
      expect(edgeGeometry.getAttribute("position").count).toBe(fixture.edgeOnly.expected.renderVertices);
      expect(edgeGeometry.boundingBox!.min.toArray()).toEqual(fixture.edgeOnly.expected.boundsMin);
      expect(edgeGeometry.boundingBox!.max.toArray()).toEqual(fixture.edgeOnly.expected.boundsMax);
      edgeGeometry.dispose();
      const camera = new PerspectiveCamera(fixture.camera.fov,1,0.1,10);
      camera.position.fromArray(fixture.camera.position);camera.up.fromArray(fixture.camera.up);camera.lookAt(new Vector3().fromArray(fixture.camera.target));camera.updateMatrixWorld();
      for(const [index,point] of [[0,new Vector3(0,0,0)],[1,new Vector3(0.25,0,0)]] as const){
        const screen = point.project(camera);
        expect((screen.x+1)*200).toBeCloseTo(fixture.picks[index].pointer[0],8);
        expect((1-screen.y)*200).toBeCloseTo(fixture.picks[index].pointer[1],8);
      }
      const pointer = fixture.picks[1].pointer;
      const direction = new Vector3(pointer[0]/200-1,1-pointer[1]/200,0.5).unproject(camera).sub(camera.position).normalize();
      expect(new Ray(camera.position,direction).distanceSqToSegment(new Vector3(...fixture.mesh.edgePositions.slice(0,3)),new Vector3(...fixture.mesh.edgePositions.slice(3,6)))).toBeCloseTo(0,12);
      const pointFixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json",source.url),"utf8"));
      const point = new Vector3(...pointFixture.cases.find((row:{kind:string})=>row.kind === "point").points[0]);
      expect(new Ray(point.clone().add(new Vector3(0,0,4)),new Vector3(0,0,-1)).distanceSqToPoint(point)).toBe(0);
      geometry.dispose();
      console.log("[DEBUG] originalAnalyticWire independentThree=true topologyVertices=4 edges=4 triangles=0");
    });
    it("preserves exact multi-object component selection with independent Lodash merges", () => {
      const fixture = JSON.parse(readFileSync(new URL("../🖱️ui/🎬️scene/🧫️fixtures/🎯️component-selection-merges/🔣️.json",source.url),"utf8"));
      const camera = new PerspectiveCamera(50,1,0.1,10);camera.position.set(0,0,4);camera.up.set(0,1,0);camera.lookAt(0,0,0);camera.updateMatrixWorld();
      for(const row of fixture.gumball.raySegmentCases){
        const ray = new Ray(new Vector3(...row.origin),new Vector3(...row.direction).normalize());
        expect(ray.distanceSqToSegment(new Vector3(...row.a),new Vector3(...row.b))).toBeCloseTo(row.distance ** 2,12);
      }
      let originalAnchor: Vector3 | undefined;
      for(const row of fixture.gumball.pivotCases){
        const pivot = new Vector3(...row.pivot);
        const ray = new Vector3((row.pointer[0]/200)-1,1-(row.pointer[1]/200),0.5).unproject(camera).sub(camera.position);
        const start = camera.position.clone().add(ray.clone().multiplyScalar((pivot.z-camera.position.z)/ray.z));
        const anchor = start.clone().sub(pivot);
        if(originalAnchor)expect(anchor.distanceTo(originalAnchor)).toBeLessThan(1e-12);else originalAnchor=anchor;
        const moved = start.clone().add(new Vector3(...fixture.gumball.delta)).project(camera);
        expect((moved.x+1)*200).toBeCloseTo(row.movedPointer[0],8);
        expect((1-moved.y)*200).toBeCloseTo(row.movedPointer[1],8);
      }
      let selected: string[] = [];
      for(const step of fixture.steps){
        const incoming: string[] = step.incoming.map((index:number)=>fixture.targets[index]);
        selected = step.merge === "replace" ? incoming : step.merge === "additive" ? union(selected,incoming) : step.merge === "subtractive" ? difference(selected,incoming) : xor(selected,incoming);
        expect(selected).toEqual(step.selected.map((index:number)=>fixture.targets[index]));
        const active = step.active === null ? null : fixture.objects[step.active];
        const groups = uniq(selected.filter(target=>active!==null && target.startsWith(`${active}.face.`)).map(target=>target.slice(target.indexOf(".face.")+6).split("~")[0]));
        expect(groups).toEqual(step.groups);
      }
      for(const step of fixture.overlayLifecycle.steps){
        const exact: string[] = step.targets.map((index:number)=>fixture.overlayLifecycle.targets[index]);
        expect(uniq(exact.map(target=>target.slice(0,target.indexOf(".face."))))).toEqual(fixture.objects);
        expect(exact.map(target=>Number(target.slice(target.indexOf(".face.")+6).split("~")[0]))).toEqual(step.groups);
        expect(exact.filter(target=>target.startsWith(`${fixture.objects[0]}.face.`))).toEqual([fixture.targets[0]]);
      }
      const wide = Array.from({length:fixture.gumball.wideSelection.count},(_,index)=>`${fixture.objects[0]}.face.${index}~${fixture.source.handle}~${BigInt(fixture.gumball.wideSelection.labelStart)+BigInt(index)}~${fixture.source.revision}`);
      expect(union(wide.slice(0,64),wide.slice(64))).toEqual(wide);
      expect(uniq(wide).length).toBe(fixture.gumball.wideSelection.count);
      expect(wide.reduce((bytes,id)=>bytes+new TextEncoder().encode(id).length,0)).toBeLessThan(16384);
      console.log("[DEBUG] componentSelectionMerge modes=4 independentLodash=true exactSourceLabels=true nonactiveFaceChanges=2 wideTargets=80");
    });
    it("merges complete original face edge and vertex provenance", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🔗️mesh-transfer-merge/🔣️.json", source.url),"utf8"));
      const transfers = fixture.transfers.map((wire:unknown) => meshTransferFromPreviewPayload(wire)!);
      const merged = mergeMeshTransfers(transfers);
      const output = meshTransferToGeometryData(merged);
      expect([...merged.index]).toEqual(fixture.merged.index);
      expect([...merged.edges]).toEqual(fixture.merged.edges);
      expect([...merged.points!]).toEqual(fixture.merged.points);
      expect(merged.faceGroups.map(group=>group.start)).toEqual([0,3]);
      expect(merged.edgeGroups.map(group=>group.start)).toEqual([0,1]);
      expect(merged.vertexGroups!.map(group=>group.start)).toEqual([0,1]);
      expect(output.faceIds).toEqual(fixture.expected.faceIds);
      expect(output.edgeIds).toEqual(fixture.expected.edgeIds);
      expect(output.vertexIds).toEqual(fixture.expected.vertexIds);
      expect(output.componentReferences).toEqual(fixture.expected.componentReferences);
      let area=0,length=0;
      const geometry = new BufferGeometry().setAttribute("position",new Float32BufferAttribute(output.position,3)).setIndex([...merged.index]);
      for(let offset=0;offset<merged.index.length;offset+=3){
        const points=[0,1,2].map(index=>new Vector3().fromBufferAttribute(geometry.getAttribute("position"),merged.index[offset+index]!));
        area+=new Vector3().subVectors(points[1]!,points[0]!).cross(new Vector3().subVectors(points[2]!,points[0]!)).length()/2;
      }
      for(let offset=0;offset<merged.edges.length;offset+=6)length+=new Vector3().fromArray(merged.edges,offset).distanceTo(new Vector3().fromArray(merged.edges,offset+3));
      expect(area).toBe(fixture.expected.area);expect(length).toBe(fixture.expected.edgeLength);
      geometry.dispose();
      console.log(`[DEBUG] originalTransferMerge domains=3 independentThreeArea=${area} edgeLength=${length}`);
    });
    it("validates original vertex selection with independent Three geometry", () => {
      const fixture = JSON.parse(readFileSync(new URL("../🖱️ui/🎬️scene/🧫️fixtures/🎯️vertex-selection/🔣️.json", source.url), "utf8"));
      const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(fixture.positions.flat(),3));
      const camera = new OrthographicCamera(-1,1,1,-1,0.1,10);
      camera.position.set(0,0,4);camera.lookAt(0,0,0);camera.updateMatrixWorld();
      const original = new Set<string>();
      for(let index=0;index<fixture.vertexIds.length;index+=1){
        const point = new Vector3().fromBufferAttribute(geometry.getAttribute("position"),index).project(camera);
        if(fixture.vertexIds[index]!==fixture.nonselectable && Math.abs(point.x)<=1 && Math.abs(point.y)<=1)original.add(String(fixture.vertexIds[index]));
      }
      expect([...original].sort()).toEqual(fixture.expected);
      geometry.dispose();
      console.log("[DEBUG] originalVertexSelection independentThree=true surfaceSamplesExcluded=true");
    });
    it("rejects malformed original vertex ranges before rendering", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json",source.url),"utf8"));
      const points = fixture.cases[0].points;
      const raw = {position:[0,0,0,1,0,0,0,1,0],normal:[0,0,1,0,0,1,0,0,1],index:[0,1,2],edges:[],points:points.flat(),face_groups:[],edge_groups:[],vertex_groups:points.map((_:unknown,index:number)=>({start:index,count:1,entity_id:String(index+1)}))};
      const validate = new Ajv({strict:true}).compile({type:"array",minItems:points.length,maxItems:points.length,prefixItems:points.map((_:unknown,index:number)=>({type:"object",required:["start","count","entity_id"],additionalProperties:false,properties:{start:{const:index},count:{const:1},entity_id:{type:"string",minLength:1,maxLength:128}}}))});
      expect(validate(raw.vertex_groups)).toBe(true);
      for(const refusal of fixture.refusals){
        const source=structuredClone(raw);source.vertex_groups[refusal.row]![refusal.field as "start"]=refusal.value;
        expect(validate(source.vertex_groups),JSON.stringify(validate.errors)).toBe(false);
        const mesh=meshTransferFromPreviewPayload(source)!;
        expect(isRenderableMeshTransfer(mesh),JSON.stringify(refusal)).toBe(false);
        expect(meshTransferToGeometryData(mesh).position.length).toBe(0);
      }
    });
    it("preserves only original topology vertex provenance", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🎯️vertex-provenance/🔣️.json", source.url), "utf8"));
      for (const row of fixture.cases) {
        const labels = row.points.map((_: unknown, index: number) => (18446744073709551615n - BigInt(index)).toString());
        const raw = { position: row.kind === "point" ? [] : [0,0,0,1,0,0,0,1,0], normal: row.kind === "point" ? [] : [0,0,1,0,0,1,0,0,1], index: row.kind === "point" ? [] : [0,1,2], edges: [], points: row.points.flat(), face_groups: [], edge_groups: [], vertex_groups: labels.map((label: string, index: number) => ({start:index,count:1,entity_id:label})) };
        const mesh = meshTransferFromPreviewPayload(raw)!;
        const output = meshTransferToGeometryData(mesh) as unknown as { readonly position: Float32Array; readonly vertexIds: readonly number[]; readonly componentReferences: Readonly<Record<string, readonly string[]>> };
        const surfaceCount = raw.position.length / 3;
        expect(output.vertexIds).toEqual([...Array(surfaceCount).fill(fixture.unselectable),...labels.map((_: string,index: number)=>index)]);
        expect(output.componentReferences.vertex).toEqual(labels);
        const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(output.position,3)).setIndex(raw.index);
        expect(geometry.getAttribute("position").count).toBe(surfaceCount + row.points.length);
        for (const [index,point] of row.points.entries()) expect(new Vector3().fromBufferAttribute(geometry.getAttribute("position"),surfaceCount+index).distanceTo(new Vector3(...point))).toBe(0);
        geometry.dispose();
        console.log(`[DEBUG] originalVertexTransfer kind=${row.kind} points=${row.points.length} independentThree=true`);
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
    it("measures shape-value fixture tessellations with independent Three volume, area and manifold checks", () => {
      const fixture = JSON.parse(readFileSync(new URL("./📐️brep/⚙️engine/🧫️fixtures/🪬️shape-values/🔣️.json",source.url),"utf8"));
      expect(fixture.tessellations.length).toBeGreaterThanOrEqual(5);
      for (const entry of fixture.tessellations) {
        const expected = fixture.cases.find((row: { name: string }) => row.name === entry.case);
        const geometry = new BufferGeometry().setAttribute("position", new Float32BufferAttribute(entry.positions, 3)).setIndex(entry.indices);
        const position = geometry.getAttribute("position");
        const corner = [new Vector3(), new Vector3(), new Vector3()];
        const edgeUses = new Map<string, number>();
        const weld = (point: Vector3) => [point.x, point.y, point.z].map((value) => Math.round(value * 1e5)).join(",");
        let volume = 0, area = 0;
        for (let triangle = 0; triangle < entry.indices.length; triangle += 3) {
          corner.forEach((point, index) => point.fromBufferAttribute(position, entry.indices[triangle + index]));
          volume += corner[0].dot(corner[1].clone().cross(corner[2])) / 6;
          area += new Triangle(corner[0], corner[1], corner[2]).getArea();
          for (let index = 0; index < 3; index++) {
            const key = [weld(corner[index]), weld(corner[(index + 1) % 3])].sort().join("|");
            edgeUses.set(key, (edgeUses.get(key) ?? 0) + 1);
          }
        }
        const relative = entry.relativeTolerance ?? 1e-6;
        expect(Math.abs(volume - expected.measures.volume) / expected.measures.volume, `${entry.case} volume ${volume}`).toBeLessThanOrEqual(relative);
        if (expected.measures.area !== undefined) expect(Math.abs(area - expected.measures.area) / expected.measures.area, `${entry.case} area ${area}`).toBeLessThanOrEqual(relative);
        expect([...edgeUses.values()].every((uses) => uses === 2), `${entry.case} watertight`).toBe(true);
        if (expected.components?.face !== undefined) expect(entry.faceEntityIds.length).toBe(expected.components.face);
        geometry.dispose();
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
