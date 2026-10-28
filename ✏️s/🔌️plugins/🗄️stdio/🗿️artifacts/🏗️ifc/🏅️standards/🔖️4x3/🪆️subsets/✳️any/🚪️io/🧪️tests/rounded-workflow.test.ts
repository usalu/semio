/** 🔲️ Actual rounded sweeps retain canonical editing, exact grants and original observer failures. */
import{expect,test}from'bun:test';
import Ajv from'ajv/dist/2020';
import{Vector3}from'three';
import{GLTFLoader}from'three/examples/jsm/loaders/GLTFLoader.js';
import cases from'./rounded-rectangle-cases.json';
import workflow from'./rounded-workflow-cases.json';
import schema from'./rounded-workflow-cases.schema.json';
import{ifcRoundedRectangleFixture,type IfcRoundedRectangleCase}from'./rounded-rectangle-fixtures.ts';
import{extractIfc4x3Triangles,IfcGeometryGraph,ifcIdentity,type IfcTriangleMesh}from'../geometry.ts';
import{ifcRoundedRectangleExtrusionSteps}from'../geometry/rounded.ts';
import{ifc4x3ToGltf,writeIfcGltfGlb,readIfcGltfGlb,gltfToIfc4x3}from'../gltf.ts';
import{writeIfc4x3Step,parseIfc4x3Step,ifcReal}from'../step.ts';
import{IfcStepError,runIfcIoSteps}from'../control.ts';
import type{IfcGeometryControl}from'../geometry/control.ts';
import{ifcMutationDiff,inverseIfcMutation,applyIfcMutation}from'../../🧬️schema/🧬️mutations/🟦️.ts';
import{applyIfcDiff,inverseIfcDiff}from'../../🧬️schema/🔺️diff/🟦️.ts';
type Mesh=Pick<IfcTriangleMesh,'positions'|'indices'>;
interface SolidObserver{volume(mesh:Mesh):number}
const observer:SolidObserver={volume(mesh){const origin=new Vector3(...mesh.positions[0]!);let result=0;for(let at=0;at<mesh.indices.length;at+=3){const[a,b,c]=[0,1,2].map(k=>new Vector3(...mesh.positions[mesh.indices[at+k]!]!).sub(origin));result+=a!.dot(b!.cross(c!))/6;}return result;}};
const base=cases.cases[0]!as IfcRoundedRectangleCase;
const glb=async(source:Parameters<typeof ifc4x3ToGltf>[0])=>{const result=await ifc4x3ToGltf(source,{batchSize:4096});expect(result.diagnostics.map(item=>[item.code,item.severity])).toEqual([['exchange.loss','warning']]);return writeIfcGltfGlb(result.artifact!,{batchSize:4096});};
test('rounded profile workflow has a closed language-neutral contract',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(workflow)).toBe(true);expect(validate({...workflow,allIfc:true})).toBe(false);});
for(const row of workflow.edits)test('rounded profile canonical '+row.id+' diff and both inverse forms',async()=>{
 const fixture=ifcRoundedRectangleFixture(base),source=fixture.snapshot,before=structuredClone(source),physical=await writeIfc4x3Step(source),original=await glb(source),mutation={mutation:'setEntityValue'as const,id:row.target==='profile'?fixture.profileEntity.id:fixture.solidEntity.id,index:row.index,path:[],value:ifcReal(row.value)},diff=ifcMutationDiff(source,mutation),changed=applyIfcDiff(source,diff),parsed=await parseIfc4x3Step(await writeIfc4x3Step(changed)),mesh=(await extractIfc4x3Triangles(parsed,{batchSize:4096})).artifact![0]!,n=workflow.defaultChords,area=row.dimensions[0]!*row.dimensions[1]!-(4-n*Math.sin(2*Math.PI/n)/2)*row.radius**2;
 expect(observer.volume(mesh)).toBeCloseTo(area*row.depth,12);expect(await glb(changed)).not.toEqual(original);expect(changed).not.toEqual(source);expect(source).toEqual(before);
 const restored=applyIfcDiff(changed,inverseIfcDiff(source,diff));expect(restored).toEqual(source);expect(applyIfcMutation(changed,inverseIfcMutation(source,mutation))).toEqual(source);expect(await writeIfc4x3Step(restored)).toBe(physical);expect(await glb(restored)).toEqual(original);
});
test('rounded INTEGER radius is repaired only by an explicit canonical REAL mutation',async()=>{
 const row=cases.cases.find(row=>row.id==='integer-radius')!as IfcRoundedRectangleCase,fixture=ifcRoundedRectangleFixture(row),source=fixture.snapshot,physical=await writeIfc4x3Step(source),mutation={mutation:'setEntityValue'as const,id:fixture.profileEntity.id,index:5,path:[],value:ifcReal(workflow.canonicalIntegerRepair.radius)},diff=ifcMutationDiff(source,mutation),changed=applyIfcDiff(source,diff);
 expect((await extractIfc4x3Triangles(source,{batchSize:4096})).artifact).toEqual([]);expect(fixture.profileEntity.args[5]!.kind).toBe('integer');const mesh=(await extractIfc4x3Triangles(changed,{batchSize:4096})).artifact![0]!;expect(mesh.positions).toHaveLength(2*workflow.canonicalIntegerRepair.boundary);expect(observer.volume(mesh)).toBeCloseTo(workflow.canonicalIntegerRepair.polygonVolume,12);expect(await writeIfc4x3Step(changed)).not.toBe(physical);expect(applyIfcMutation(changed,inverseIfcMutation(source,mutation))).toEqual(source);expect(await writeIfc4x3Step(applyIfcDiff(changed,inverseIfcDiff(source,diff)))).toBe(physical);
});
for(const row of workflow.segmentCaps)test('rounded total segment cap '+row.id,async()=>{
 const fixture=ifcRoundedRectangleFixture({...base,dimensions:row.dimensions,radius:row.radius}),before=structuredClone(fixture.snapshot),sampling={absoluteChordErrorMetres:0,relativeChordError:1,maxSegments:row.minimum},exact=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,curveSampling:sampling});expect(exact.diagnostics).toEqual([]);expect(exact.artifact![0]!.positions).toHaveLength(2*row.boundary);if(row.minimum>4){const short=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,curveSampling:{...sampling,maxSegments:row.minimum-1}});expect(short.artifact).toEqual([]);expect(short.diagnostics.some(item=>item.severity==='error')).toBe(true);}expect(fixture.snapshot).toEqual(before);
});
test('rounded sweep preflights the adjacent original output grant',async()=>{
 const fixture=ifcRoundedRectangleFixture(base),graph=(await IfcGeometryGraph.capture(fixture.snapshot,{batchSize:4096})).artifact!,solid=graph.lookup(fixture.solidEntity.id)!;
 await expect(runIfcIoSteps(ifcRoundedRectangleExtrusionSteps(graph,solid,ifcIdentity(),1,base.sampling,workflow.defaultValues-1),{batchSize:4096},'extrusions')).rejects.toHaveProperty('code','geometry.limit');
 const mesh=await runIfcIoSteps(ifcRoundedRectangleExtrusionSteps(graph,solid,ifcIdentity(),1,base.sampling,workflow.defaultValues),{batchSize:4096},'extrusions');expect(mesh.positions.length*3+mesh.indices.length).toBe(workflow.defaultValues);
});
test('rounded shared products consume one original output grant',async()=>{
 const fixture=ifcRoundedRectangleFixture(base);for(const guid of['0000000000000000000003','0000000000000000000004']){const args=structuredClone(fixture.productEntity.args);args[0]={kind:'string',value:guid};fixture.add('IFCBUILDINGELEMENTPROXY',args);}const source=fixture.snapshot,before=structuredClone(source),short=await extractIfc4x3Triangles(source,{batchSize:4096,maxValues:3*workflow.defaultValues-1}),exact=await extractIfc4x3Triangles(source,{batchSize:4096,maxValues:3*workflow.defaultValues});expect(short.artifact).toHaveLength(2);expect(short.diagnostics.some(item=>item.severity==='error')).toBe(true);expect(exact.artifact).toHaveLength(3);expect(exact.diagnostics).toEqual([]);expect(source).toEqual(before);
});
for(const endpoint of[extractIfc4x3Triangles,ifc4x3ToGltf])test(endpoint.name+' captures rounded curve and value grants',async()=>{
 const fixture=ifcRoundedRectangleFixture(base),curveSampling={...base.sampling},control:IfcGeometryControl={batchSize:4096,maxValues:endpoint===extractIfc4x3Triangles?workflow.defaultValues:10000,curveSampling,progress(){curveSampling.relativeChordError=1;curveSampling.maxSegments=4;control.maxValues=1;control.curveSampling={absoluteChordErrorMetres:1,relativeChordError:1,maxSegments:4};}},outcome=await endpoint(fixture.snapshot,control);expect(outcome.artifact).toBeDefined();if(Array.isArray(outcome.artifact))expect(outcome.artifact[0]!.positions).toHaveLength(2*workflow.defaultBoundary);else expect(outcome.artifact!.document.accessors[0]!.count).toBe(BigInt(2*workflow.defaultBoundary));
});
for(const endpoint of[extractIfc4x3Triangles,ifc4x3ToGltf])for(const frontier of['initial','interior','completion'])for(const kind of['domain','undefined','cancel'])test(endpoint.name+' rounded '+frontier+' '+kind,async()=>{
 const controller=new AbortController(),failure:unknown=kind==='domain'?new IfcStepError('geometry.invalid','original rounded observer'):undefined,source=ifcRoundedRectangleFixture(base).snapshot,before=structuredClone(source);let reached=false,caught:unknown=Symbol('not caught');
 try{await endpoint(source,{signal:controller.signal,batchSize:16,progress(event){if(event.phase==='extrusions'&&(frontier==='initial'?event.completed===0:frontier==='interior'?event.completed>0&&event.total===0:event.completed>0&&event.completed===event.total)){reached=true;if(kind==='cancel')controller.abort();else throw failure;}}});}catch(error){caught=error;}
 expect(reached).toBe(true);if(kind==='cancel')expect(caught).toHaveProperty('code','io.cancelled');else expect(caught).toBe(failure);expect(source).toEqual(before);
});
for(const row of cases.cases.filter(row=>row.geometry==='supported'))test('actual rounded GLB and independent Three mesh '+row.id,async()=>{
 const fixture=ifcRoundedRectangleFixture(row as IfcRoundedRectangleCase),before=structuredClone(fixture.snapshot),control={batchSize:4096,curveSampling:row.sampling},source=await extractIfc4x3Triangles(fixture.snapshot,control),out=await ifc4x3ToGltf(fixture.snapshot,control),bytes=await writeIfcGltfGlb(out.artifact!,{batchSize:4096}),independent=await new GLTFLoader().parseAsync(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength)as ArrayBuffer,'');independent.scene.updateMatrixWorld(true);let points=0,triangles=0;independent.scene.traverse(object=>{if(!('geometry'in object))return;const geometry=object.geometry as{getAttribute(name:string):{count:number};index:{count:number}|null};points+=geometry.getAttribute('position').count;triangles+=(geometry.index?.count??0)/3;});expect(points).toBe(2*row.expectedBoundary!);expect(triangles).toBe(4*row.expectedBoundary!-4);expect(observer.volume(source.artifact![0]!)).toBeCloseTo(row.polygonVolume!,9);const back=await gltfToIfc4x3(await readIfcGltfGlb(bytes,{batchSize:4096}),{batchSize:4096}),mesh=(await extractIfc4x3Triangles(back.artifact!,{batchSize:4096})).artifact![0]!;expect(observer.volume(mesh)).toBeCloseTo(row.polygonVolume!,5);expect(fixture.snapshot).toEqual(before);
});
