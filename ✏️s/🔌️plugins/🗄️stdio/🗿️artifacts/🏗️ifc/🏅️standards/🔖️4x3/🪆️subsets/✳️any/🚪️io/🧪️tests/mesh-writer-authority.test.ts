import Ajv from'ajv';
import{expect,test}from'bun:test';
import cases from'./mesh-writer-authority-cases.json';
import schema from'./mesh-writer-authority.schema.json';
import{meshWriterFixture}from'./mesh-writer-fixtures.ts';
import{IfcGeometryGraph,type IfcTriangleMesh}from'../geometry.ts';
import{ifcNumber}from'../step.ts';
import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'../geometry/mesh-writer.ts';
import{ifcCsgPrimitiveWriteSteps}from'../geometry/csg.ts';
import{ifcRectangleExtrusionWriteSteps}from'../geometry/rectangle.ts';
import{ifcCircleExtrusionWriteSteps}from'../geometry/circle.ts';
import{ifcRoundedRectangleExtrusionWriteSteps}from'../geometry/rounded.ts';
import type{IfcIoSteps}from'../control.ts';
type Fixture=Awaited<ReturnType<typeof meshWriterFixture>>;
function source(kind:string,fixture:Fixture,make:IfcMeshWriterFactory<Pick<IfcTriangleMesh,'positions'|'indices'>>){const{graph,entity,world,scale,policy}=fixture;return kind==='rectangle'?ifcRectangleExtrusionWriteSteps(graph,entity,world,scale,make):kind==='circle'?ifcCircleExtrusionWriteSteps(graph,entity,world,scale,policy,make):kind==='rounded'?ifcRoundedRectangleExtrusionWriteSteps(graph,entity,world,scale,policy,make):ifcCsgPrimitiveWriteSteps(graph,entity,world,scale,policy,make);}
function drain<T>(steps:IfcIoSteps<T>):T{try{for(;;){const step=steps.next();if(step.done)return step.value;}}finally{steps.return(undefined as T);}}
test('source writer authority damage cases are closed exact catalog obligations',()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,coerceIntegerToReal:true})).toBe(false);});
for(const row of cases.cases)test('direct writer refuses missing original source/domain authority '+row.kind+' '+row.damage,async()=>{let fixture=await meshWriterFixture(row.kind),births=0;if(row.damage==='foreign')fixture={...fixture,entity:structuredClone(fixture.entity)};else{const target=fixture.source.entities.find(entity=>entity.id===fixture.entity.id)!,profile=target.args[0];if(profile?.kind!=='reference')throw Error('Original profile reference');const owner=row.damage==='integer-depth'?target:fixture.source.entities.find(entity=>entity.id===profile.value)!,slot=row.damage==='integer-y'?4:3;owner.args[slot]={kind:'integer',value:BigInt(ifcNumber(owner.args[slot]))};const captured=await IfcGeometryGraph.capture(fixture.source,{batchSize:4096});fixture={...fixture,graph:captured.artifact!,entity:captured.artifact!.lookup(target.id)!};}const make:IfcMeshWriterFactory<Pick<IfcTriangleMesh,'positions'|'indices'>>=(v,t)=>{births++;return ifcArrayMeshWriterFactory(1_000_000)(v,t);};expect(()=>drain(source(row.kind,fixture,make))).toThrow(row.error);expect(births).toBe(0);});
