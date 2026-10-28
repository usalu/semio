/** 🛢️ Declared circular CSG roots remain editable through canonical diffs and actual STEP/GLB exchange. */
import{expect,test}from'bun:test';
import{readFile}from'node:fs/promises';
import{createHash}from'node:crypto';
import Ajv from'ajv/dist/2020';
import{Vector3,Matrix4}from'three';
import cases from'./csg-round-cases.json';
import schema from'./csg-round.schema.json';
import supplemental from'./csg-round-supplemental-cases.json';
import supplementalSchema from'./csg-round-supplemental.schema.json';
import orientationCases from'./circular-solid-orientation-cases.json';
import orientationSchema from'./circular-solid-orientation.schema.json';
import circleCases from'./circle-extrusion-cases.json';
import mappedCircleCases from'./mapped-circle-cases.json';
import mappingCases from'./mapping-cases.json';
import{ifcCircleFixture,ifcMappedCircleFixture}from'./circle-extrusion-fixtures.ts';
import{ifcMappingFixture}from'./mapping-fixtures.ts';
import{ifcRoundFixture}from'./csg-round-fixtures.ts';
import{extractIfc4x3Triangles,IfcGeometryGraph,ifcIdentity,type IfcTriangleMesh}from'../geometry.ts';
import{ifcCsgSolidSteps}from'../geometry/csg.ts';
import{ifc4x3ToGltf,writeIfcGltfGlb,readIfcGltfGlb,gltfToIfc4x3}from'../gltf.ts';
import{writeIfc4x3Step,parseIfc4x3Step,ifcReal}from'../step.ts';
import{IfcStepError,runIfcIoSteps,type IfcIoControl}from'../control.ts';
import type{IfcGeometryControl}from'../geometry/control.ts';
import{ifcMutationDiff,inverseIfcMutation,applyIfcMutation}from'../../🧬️schema/🧬️mutations/🟦️.ts';
import{applyIfcDiff,inverseIfcDiff}from'../../🧬️schema/🔺️diff/🟦️.ts';
import type{IfcEntity}from'../../🧬️schema/📸️snapshot/🟦️.ts';
type Mesh=Pick<IfcTriangleMesh,'positions'|'indices'>;
interface MeshObserver{volume(mesh:Mesh):number}
const observer:MeshObserver={volume(mesh){let value=0;const origin=new Vector3(...mesh.positions[0]!);for(let at=0;at<mesh.indices.length;at+=3){const a=new Vector3(...mesh.positions[mesh.indices[at]!]!).sub(origin),b=new Vector3(...mesh.positions[mesh.indices[at+1]!]!).sub(origin),c=new Vector3(...mesh.positions[mesh.indices[at+2]!]!).sub(origin);value+=a.dot(b.cross(c))/6;}return value;}};
const bounds=(mesh:Mesh)=>[Math.min,Math.max].map(operation=>[0,1,2].map(axis=>operation(...mesh.positions.map(point=>point[axis]!))));
const closed=(mesh:Mesh)=>{const edges=new Map<string,number[]>();for(let at=0;at<mesh.indices.length;at+=3)for(let k=0;k<3;k++){const a=mesh.indices[at+k]!,b=mesh.indices[at+(k+1)%3]!,key=Math.min(a,b)+':'+Math.max(a,b),entry=edges.get(key)??[];entry.push(a<b?1:-1);edges.set(key,entry);}for(const entry of edges.values())expect(entry.toSorted()).toEqual([-1,1]);};
const frame=(origin:number[],axis:number[],reference:number[])=>{const z=new Vector3(...axis as[number,number,number]).normalize(),y=new Vector3().crossVectors(z,new Vector3(...reference as[number,number,number])).normalize(),x=new Vector3().crossVectors(y,z).normalize();return new Matrix4().makeBasis(x,y,z).setPosition(...origin as[number,number,number]);};
const glb=async(source:Parameters<typeof ifc4x3ToGltf>[0])=>{const result=await ifc4x3ToGltf(source,{batchSize:4096});expect(result.diagnostics.map(d=>[d.code,d.severity])).toEqual([['exchange.loss','warning']]);return writeIfcGltfGlb(result.artifact!,{batchSize:4096});};

test('circular CSG neutral declarations and supplemental sources are closed and pinned',async()=>{
 for(const[data,definition,count]of[[cases,schema,20],[supplemental,supplementalSchema,18]]as const){const validate=new Ajv({strict:true}).compile(definition);expect(validate(data)).toBe(true);expect(validate({...data,allCsg:true})).toBe(false);expect(data.cases).toHaveLength(count);}
 for(const[name,hash]of[['csg-round-cases.json','d56ae3ca50e01a8189c80316c09bd32926e809fdfa9ef5e1292dd7d50fbf3f3b'],['csg-round.schema.json','0f536379e488052fb77078db557addd07743b5ff60d140dd63285d9f7e9f2729'],['csg-round-supplemental-cases.json','b8d05f32a2d1c23b2975040e46d1f47b1107054e2ee067236218f447e2c8903f'],['csg-round-supplemental.schema.json','5abe16a7f384c6e3d6ffa71448c789f6aab9b4792973cdfbe9c73bbbb76d6869']])expect(createHash('sha256').update(await readFile(new URL(name!,import.meta.url))).digest('hex')).toBe(hash!);
});
for(const row of cases.cases)test('actual circular CSG '+row.id,async()=>{
 const fixture=ifcRoundFixture(row),before=structuredClone(fixture.snapshot),control={batchSize:4096,curveSampling:row.source.policy},result=await extractIfc4x3Triangles(fixture.snapshot,control);expect(fixture.snapshot).toEqual(before);
 if(row.expected.status==='refuse'){expect(result.artifact??[]).toEqual([]);expect(result.diagnostics.some(d=>d.severity==='error')).toBe(true);return;}
 expect(result.diagnostics).toEqual([]);expect(result.artifact).toHaveLength(1);const mesh=result.artifact![0]!,n=row.expected.segments!,cone=row.source.primitive==='cone';expect(mesh.entityId).toBe(fixture.solid.id);expect(mesh.productId).toBe(fixture.product.id);expect(mesh.positions).toHaveLength(cone?n+1:2*n);expect(mesh.indices).toHaveLength(cone?6*n-6:12*n-12);expect(observer.volume(mesh)).toBeCloseTo(row.expected.polygonVolume!,9);closed(mesh);
 for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)expect(bounds(mesh)[side]![axis]).toBeCloseTo(row.expected.bounds![side]![axis]!,10);
 const physical=await parseIfc4x3Step(await writeIfc4x3Step(fixture.snapshot,{batchSize:4096}),{batchSize:4096}),again=await extractIfc4x3Triangles(physical,control);expect(again.diagnostics).toEqual([]);expect(again.artifact).toEqual(result.artifact);
 const out=await ifc4x3ToGltf(fixture.snapshot,control);expect(out.diagnostics.map(d=>[d.code,d.severity])).toEqual([['exchange.loss','warning']]);const bytes=await writeIfcGltfGlb(out.artifact!,{batchSize:4096}),imported=await gltfToIfc4x3(await readIfcGltfGlb(bytes,{batchSize:4096}),{batchSize:4096}),round=await extractIfc4x3Triangles(imported.artifact!,{batchSize:4096});expect(round.diagnostics).toEqual([]);expect(observer.volume(round.artifact![0]!)).toBeCloseTo(row.expected.polygonVolume!,5);expect(fixture.snapshot).toEqual(before);
});
for(const primitive of['cylinder','cone'])for(const row of supplemental.cases)test('circular CSG exact source '+primitive+' '+row.id,async()=>{
 const fixture=ifcRoundFixture({id:row.id,source:{...cases.cases[0]!.source,...row.base,primitive}}),damage=row.damage,target:IfcEntity=damage.target==='primitive'||damage.target==='radius'?fixture.primitive:damage.target==='root'?fixture.solid:damage.target==='position'?fixture.position:damage.target==='point'?fixture.positionPoint:fixture.positionDirection!;
 if(damage.operation==='appendUnset')target.args.push({kind:'unset'});
 else if(damage.operation==='removeLast')target.args.pop();
 else if(damage.operation==='complexConstituent')target.complex=[{name:'IFCCSGPRIMITIVE3D',args:[fixture.primitive.args[0]!]}];
 else if(damage.operation==='unsetPosition')target.args[0]={kind:'unset'};
 else if(damage.operation==='pointPosition')target.args[0]={kind:'reference',value:fixture.positionPoint.id};
 else if(damage.operation==='realBits')target.args[2]={kind:'real',value:{bits:BigInt(damage.realWord!)}};
 else if(damage.operation==='translateAll1e20')fixture.positionPoint.args[0]={kind:'aggregate',value:[1e20,1e20,1e20].map(ifcReal)};
 else throw Error('Unimplemented explicit supplemental damage');
 const before=structuredClone(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(result.artifact??[]).toEqual([]);expect(result.diagnostics.some(d=>d.severity==='error')).toBe(true);expect(fixture.snapshot).toEqual(before);
});
for(const primitive of['cylinder','cone'])for(const[measure,index,value,ratio]of[['radius',2,2,4],['height',1,6,2]]as const)test('circular CSG canonical '+measure+' diff and both inverse forms '+primitive,async()=>{
 const fixture=ifcRoundFixture({id:primitive,source:{...cases.cases[0]!.source,primitive}}),source=fixture.snapshot,physical=await writeIfc4x3Step(source,{batchSize:4096}),original=await glb(source),mutation={mutation:'setEntityValue' as const,id:fixture.primitive.id,index,path:[],value:ifcReal(value)},diff=ifcMutationDiff(source,mutation),inverse=inverseIfcMutation(source,mutation),changed=applyIfcDiff(source,diff),mesh=(await extractIfc4x3Triangles(changed,{batchSize:4096})).artifact![0]!;
 expect(observer.volume(mesh)).toBeCloseTo(cases.cases[primitive==='cone'?1:0]!.expected.polygonVolume!*ratio,9);expect(await glb(changed)).not.toEqual(original);
 const restored=applyIfcDiff(changed,inverseIfcDiff(source,diff));expect(applyIfcMutation(changed,inverse)).toEqual(source);expect(restored).toEqual(source);expect(await writeIfc4x3Step(restored,{batchSize:4096})).toBe(physical);expect(await glb(restored)).toEqual(original);
});
for(const primitive of['cylinder','cone'])test('circular CSG original item grant adjacent boundary '+primitive,async()=>{
 const fixture=ifcRoundFixture({id:primitive,source:{...cases.cases[0]!.source,primitive}}),graph=(await IfcGeometryGraph.capture(fixture.snapshot,{batchSize:4096})).artifact!,solid=graph.lookup(fixture.solid.id)!,grant=primitive==='cone'?645:1284;
 await expect(runIfcIoSteps(ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,cases.cases[0]!.source.policy,grant-1),{batchSize:4096},'csg')).rejects.toHaveProperty('code','geometry.limit');
 const mesh=await runIfcIoSteps(ifcCsgSolidSteps(graph,solid,ifcIdentity(),1,cases.cases[0]!.source.policy,grant),{batchSize:4096},'csg');expect(mesh.positions.length*3+mesh.indices.length).toBe(grant);
});
for(const primitive of['cylinder','cone'])test('public circular CSG shared-product original grant '+primitive,async()=>{
 const fixture=ifcRoundFixture({id:primitive,source:{...cases.cases[0]!.source,primitive}}),grant=primitive==='cone'?645:1284;
 for(const guid of['0000000000000000000003','0000000000000000000004']){const args=structuredClone(fixture.product.args);args[0]={kind:'string',value:guid};fixture.add('IFCBUILDINGELEMENTPROXY',args);}
 const before=structuredClone(fixture.snapshot),short=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,maxValues:3*grant-1}),exact=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,maxValues:3*grant});expect(short.artifact).toHaveLength(2);expect(short.diagnostics.some(d=>d.message.includes('original value grant'))).toBe(true);expect(exact.diagnostics).toEqual([]);expect(exact.artifact).toHaveLength(3);expect(fixture.snapshot).toEqual(before);
});
for(const primitive of['cylinder','cone'])test('circular CSG original curve and output grants are captured '+primitive,async()=>{
 const fixture=ifcRoundFixture({id:primitive,source:{...cases.cases[0]!.source,primitive}}),sampling={...cases.cases[0]!.source.policy},control:IfcGeometryControl={batchSize:4096,maxValues:primitive==='cone'?645:1284,curveSampling:sampling,progress(){sampling.relativeChordError=1;sampling.maxSegments=4;control.maxValues=1;control.curveSampling={absoluteChordErrorMetres:1,relativeChordError:1,maxSegments:4};}};
 const result=await extractIfc4x3Triangles(fixture.snapshot,control);expect(result.diagnostics).toEqual([]);expect(result.artifact![0]!.positions).toHaveLength(primitive==='cone'?73:144);
});
for(const primitive of['cylinder','cone'])for(const damage of['subnormal-height','overflow-si-height','overflow-si-radius','overflow-edge-difference'])test('circular CSG finite numeric admission '+primitive+' '+damage,async()=>{
 const source={...cases.cases[0]!.source,primitive,height:damage==='subnormal-height'?2**-1023:damage==='overflow-si-height'?1e308:3,radius:damage==='overflow-si-radius'||damage==='overflow-edge-difference'?1e308:1,unitPrefix:damage.startsWith('overflow-si')?'KILO':null},fixture=ifcRoundFixture({id:damage,source}),before=structuredClone(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(result.artifact??[]).toEqual([]);expect(result.diagnostics.some(d=>d.severity==='error')).toBe(true);expect(fixture.snapshot).toEqual(before);
});
for(const primitive of['cylinder','cone'])test('circular CSG finite binary64 mesh refuses collapsed Float32 GLB '+primitive,async()=>{
 const source=structuredClone(cases.cases[0]!.source);source.primitive=primitive;source.position.location=[1e8,1e8,1e8];const fixture=ifcRoundFixture({id:primitive,source}),before=structuredClone(fixture.snapshot),mesh=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(mesh.diagnostics).toEqual([]);expect(mesh.artifact).toHaveLength(1);closed(mesh.artifact![0]!);await expect(ifc4x3ToGltf(fixture.snapshot,{batchSize:4096})).rejects.toHaveProperty('code','gltf.precision');expect(fixture.snapshot).toEqual(before);
});
for(const row of circleCases.cases.filter(row=>row.expected==='supported'))test('shared circular solid ordered declared sweep '+row.id,async()=>{
 const fixture=ifcCircleFixture(row),before=structuredClone(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,curveSampling:row.sampling}),mesh=result.artifact![0]!,n=row.segments!,profile=frame([...(row.profileOrigin??[0,0]),0],[0,0,1],[...(row.profileX??[1,0]),0]),world=frame(row.solidOrigin??[0,0,0],row.solidZ??[0,0,1],row.solidX??[1,0,0]),displacement=new Vector3(...row.direction as[number,number,number]).normalize().multiplyScalar(row.depth),scale=row.millimetres?.001:1;
 expect(result.diagnostics).toEqual([]);expect(mesh.positions).toHaveLength(2*n);closed(mesh);
 for(let layer=0;layer<2;layer++)for(let at=0;at<n;at++){const angle=2*Math.PI*at/n,expected=new Vector3(row.radius*Math.cos(angle),row.radius*Math.sin(angle),0).applyMatrix4(profile).addScaledVector(displacement,layer).applyMatrix4(world).multiplyScalar(scale);for(let axis=0;axis<3;axis++)expect(mesh.positions[layer*n+at]![axis]).toBeCloseTo(expected.getComponent(axis),12);}expect(fixture.snapshot).toEqual(before);
});
for(const row of mappedCircleCases.cases.filter(row=>row.expected==='accept'))test('shared circular solid ordered mapped sweep '+row.id,async()=>{
 const fixture=ifcMappedCircleFixture(row,mappingCases.baseBounds),before=structuredClone(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096,curveSampling:row.sampling,maxValues:row.maxValues}),mesh=result.artifact![0]!,n=row.segments!,c=Math.cos(row.tilt),s=Math.sin(row.tilt),world=new Matrix4().makeBasis(new Vector3(c,0,-s).multiplyScalar(row.scales[0]!),new Vector3(0,row.reflected?-1:1,0).multiplyScalar(row.scales[1]!),new Vector3(s,0,c).multiplyScalar(row.scales[2]!)).setPosition(10,20,30),depth=mappingCases.baseBounds[1]![2]!-mappingCases.baseBounds[0]![2]!;
 expect(result.diagnostics).toEqual([]);expect(mesh.positions).toHaveLength(2*n);closed(mesh);
 for(let layer=0;layer<2;layer++)for(let at=0;at<n;at++){const angle=2*Math.PI*at/n+row.profileRotation,expected=new Vector3(Math.cos(angle),Math.sin(angle),layer*depth).applyAxisAngle(new Vector3(0,1,0),row.solidTilt).applyMatrix4(world);for(let axis=0;axis<3;axis++)expect(mesh.positions[layer*n+at]![axis]).toBeCloseTo(expected.getComponent(axis),10);}expect(fixture.snapshot).toEqual(before);
});
test('circular sweep combined orientation contract is closed',()=>{const validate=new Ajv({strict:true}).compile(orientationSchema);expect(validate(orientationCases)).toBe(true);expect(validate({...orientationCases,allSweeps:true})).toBe(false);});
for(const row of orientationCases.cases)test('circular sweep independent profile and displacement frames '+row.id,async()=>{
 const fixture=ifcMappingFixture({layers:[{origin:[0,0,0],originRotation:0,target:[0,0,0],axes:[[Math.sign(row.representedScale[0]!),0,0],[0,1,0],[0,0,1]],scale:Math.abs(row.representedScale[0]!),secondary:row.representedScale.slice(1)}]},[[0,0,0],[1,1,row.depth]],'circle'),a=(values:number[])=>({kind:'aggregate' as const,value:values.map(ifcReal)}),profileRef=fixture.solid.args[0];
 if(profileRef?.kind!=='reference')throw Error('Actual profile reference required');const profile=fixture.snapshot.entities.find(entity=>entity.id===profileRef.value)!,point=fixture.add('IFCCARTESIANPOINT',[a(row.profileOrigin)]),direction=fixture.add('IFCDIRECTION',[a([Math.cos(row.profileRotation),Math.sin(row.profileRotation)])]);profile.args[2]=fixture.add('IFCAXIS2PLACEMENT2D',[point,direction]);fixture.solid.args[2]=fixture.add('IFCDIRECTION',[a(row.direction)]);
 const before=structuredClone(fixture.snapshot),result=await extractIfc4x3Triangles(fixture.snapshot,{batchSize:4096});expect(result.diagnostics).toEqual([]);const mesh=result.artifact![0]!,n=row.segments,world=new Matrix4().makeScale(...row.representedScale as[number,number,number]),displacement=new Vector3(...row.direction as[number,number,number]).normalize().multiplyScalar(row.depth);expect(mesh.positions).toHaveLength(2*n);expect(observer.volume(mesh)).toBeCloseTo(row.polygonVolume,9);closed(mesh);
 for(let layer=0;layer<2;layer++)for(let at=0;at<n;at++){const angle=2*Math.PI*at/n,expected=new Vector3(row.radius*Math.cos(angle),row.radius*Math.sin(angle),0).applyAxisAngle(new Vector3(0,0,1),row.profileRotation).add(new Vector3(...row.profileOrigin as[number,number],0)).addScaledVector(displacement,layer).applyMatrix4(world);for(let axis=0;axis<3;axis++)expect(mesh.positions[layer*n+at]![axis]).toBeCloseTo(expected.getComponent(axis),12);}expect(fixture.snapshot).toEqual(before);
});
test('circular CSG accuracy refusal retains the original segment grant diagnostic',async()=>{const row=cases.cases.find(row=>row.id==='segment-cap')!,result=await extractIfc4x3Triangles(ifcRoundFixture(row).snapshot,{batchSize:4096,curveSampling:row.source.policy});expect(result.artifact??[]).toEqual([]);expect(result.diagnostics.some(d=>d.message.includes('geometry.accuracy'))).toBe(true);});
for(const endpoint of['triangles','glb'])for(const phase of['csg','mesh-orientation'])for(const frontier of['initial','interior','completion'])for(const fault of['error','abort','undefined'])test('circular CSG original observer '+endpoint+' '+phase+' '+frontier+' '+fault,async()=>{
 const source=ifcRoundFixture(cases.cases.find(row=>row.id==='cylinder-mapped-reflected')!).snapshot,before=structuredClone(source),events:number[]=[],run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const out=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(out.artifact!,control);};
 await run({batchSize:128,progress:event=>{if(event.phase===phase)events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,abort=new AbortController(),replacement=new AbortController(),original=fault==='error'?new Error('original circular CSG observer'):undefined;let calls=0,redirected=0,caught=false;
 const control:IfcIoControl={batchSize:128,signal:abort.signal,progress(event){if(event.phase!==phase||calls++!==target)return;control.signal=replacement.signal;control.progress=()=>redirected++;if(fault==='abort')abort.abort();else throw original;}};
 try{await run(control);}catch(error){caught=true;if(fault==='abort'){expect(error).toBeInstanceOf(IfcStepError);expect(error).toHaveProperty('code','io.cancelled');}else expect(error).toBe(original);}expect(caught).toBe(true);expect(calls).toBe(target+1);expect(redirected).toBe(0);expect(source).toEqual(before);
});
