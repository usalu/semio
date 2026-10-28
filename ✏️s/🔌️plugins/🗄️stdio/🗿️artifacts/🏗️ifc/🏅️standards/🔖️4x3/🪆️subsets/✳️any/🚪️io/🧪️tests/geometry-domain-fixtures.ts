/** 🧪️ Explicit carrier cases alter actual declared primitive and coordinate source slots only. */
import type{IfcValue}from'../../🧬️schema/📸️snapshot/🟦️.ts';
import circular from'./csg-round-cases.json';
import{ifcRoundFixture}from'./csg-round-fixtures.ts';
import{ifcReal}from'../step.ts';
interface IfcGeometryDomainCase{id:string;primitive:string;target:string;carrier:string;value:number}
/** 🧬️ Original graph identities and borrowed coordinates remain visible to independent raw-STEP observers. */
export function ifcGeometryDomainFixture(row:IfcGeometryDomainCase){
 const source=structuredClone(circular.cases[0]!.source);source.primitive=row.primitive;source.position.axis=[0,0,1];source.position.refDirection=[1,0,0];const fixture=ifcRoundFixture({id:row.id,source}),value:IfcValue=row.carrier==='integer'?{kind:'integer',value:BigInt(row.value)}:ifcReal(row.value);
 if(row.target==='height'||row.target==='radius')fixture.primitive.args[row.target==='height'?1:2]=value;
 else if(['point','axis','reference'].includes(row.target)){const reference=row.target==='reference'?fixture.position.args[2]:undefined,target=row.target==='point'?fixture.positionPoint:row.target==='axis'?fixture.positionDirection!:fixture.snapshot.entities.find(entity=>entity.id===(reference?.kind==='reference'?reference.value:-1n))!,items=target.args[0];if(items?.kind!=='aggregate')throw Error('Authored source coordinates');items.value[0]=value;}
 else throw Error('A declared carrier target is required');return fixture;
}
