/** 🧊️ Whole-request Boolean limits are captured once before caller observation and source traversal. */
import{expect,test}from'bun:test';
import Ajv from'ajv/dist/2020';
import cases from'./boolean-control-cases.json';
import schema from'./boolean-control-cases.schema.json';
import rectangle from'./rounded-rectangle-cases.json';
import{ifcRoundedRectangleFixture,type IfcRoundedRectangleCase}from'./rounded-rectangle-fixtures.ts';
import{captureIfcGeometryControl,type IfcGeometryControl}from'../geometry/control.ts';
import{extractIfc4x3Triangles}from'../geometry.ts';
import{ifc4x3ToGltf}from'../gltf.ts';
type Policy=typeof cases.defaults;
const capture=(value:unknown)=>captureIfcGeometryControl({booleanEvaluation:value}as unknown as IfcGeometryControl)as unknown as{booleanEvaluation:Policy};
test('IFC original Boolean policy has a closed neutral schema',()=>{const validate=new Ajv({strict:true,allowUnionTypes:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,allBoolean:true})).toBe(false);});
for(const source of[undefined,{}])test('IFC Boolean original defaults '+JSON.stringify(source),()=>expect(capture(source).booleanEvaluation).toEqual(cases.defaults));
for(const row of cases.invalid)test('IFC Boolean policy admission '+row.id,()=>expect(()=>capture(row.policy)).toThrow('geometry.control'));
for(const name of Object.keys(cases.defaults)as(keyof Policy)[])for(const value of[NaN,Infinity,Number.MAX_SAFE_INTEGER+1])test('IFC Boolean nonfinite or unsafe '+name+' '+String(value),()=>expect(()=>capture({[name]:value})).toThrow('geometry.control'));
test('IFC Boolean policy is copied once before progress getters can replace original values',()=>{
 const source={...cases.defaults},reads=new Map<string,number>(),policy=Object.fromEntries(Object.keys(source).map(name=>[name,0]));for(const name of Object.keys(source)as(keyof Policy)[])Object.defineProperty(policy,name,{enumerable:true,get(){reads.set(name,(reads.get(name)??0)+1);return source[name];}});let policyReads=0;const control={get booleanEvaluation(){policyReads++;return policy;},get progress(){for(const name of Object.keys(source)as(keyof Policy)[])source[name]=1;return()=>{};}},result=captureIfcGeometryControl(control as IfcGeometryControl)as unknown as{booleanEvaluation:Policy};expect(result.booleanEvaluation).toEqual(cases.defaults);expect(policyReads).toBe(1);expect([...reads.values()]).toEqual([1,1,1,1]);expect(result.booleanEvaluation).not.toBe(policy);
});
for(const endpoint of[extractIfc4x3Triangles,ifc4x3ToGltf])for(const row of cases.invalid)test(endpoint.name+' refuses original Boolean policy before source traversal '+row.id,async()=>{
 const source=ifcRoundedRectangleFixture(rectangle.cases[0]!as IfcRoundedRectangleCase).snapshot,before=structuredClone(source);let observed=false;await expect(endpoint(source,{booleanEvaluation:row.policy,progress(){observed=true;}}as unknown as IfcGeometryControl)).rejects.toHaveProperty('code','geometry.control');expect(observed).toBe(false);expect(source).toEqual(before);
});
