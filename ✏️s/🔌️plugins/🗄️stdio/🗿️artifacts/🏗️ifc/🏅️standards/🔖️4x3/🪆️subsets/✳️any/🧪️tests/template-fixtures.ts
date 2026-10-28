import type {IfcSnapshot,IfcValue,IfcEntity} from '../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcTemplateApplication} from '../🧬️schema/🏷️templates/✍️apply/types.ts';
import {ifcAttributeDeclarations} from '../🧬️schema/📚️catalog/🟦️.ts';
import {ifc4x3Template} from '../🧬️schema/🏷️templates/🟦️.ts';
import {ifcTemplateIdentityKeysAsync} from '../🧬️schema/🏷️templates/✍️apply/🟦️.ts';
export const s=(value:string):IfcValue=>({kind:'string',value});
export const a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value});
export const r=(value:bigint):IfcValue=>({kind:'reference',value});
export const real=(value:number):IfcValue=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,false);return{kind:'real',value:{bits:view.getBigUint64(0,false)}};};
export const empty:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[a([s('ViewDefinition []')]),s('2;1')],fileName:[s('templates.ifc'),s('2026-10-21T12:00:00'),a([s('semio')]),a([s('semio')]),s('semio'),s('semio'),s('')],fileSchema:[a([s('IFC4X3_ADD2')])]},entities:[]};
export function entity(name:string,id=1n,fields:Record<string,IfcValue>={}):IfcEntity{return{id,name:name.toUpperCase(),complex:[],args:ifcAttributeDeclarations(name)!.map(a=>fields[a.name]??(a.name==='GlobalId'?s('0000000000000000000001'):{kind:'unset'}))};}
export const application=():IfcTemplateApplication=>({template:'Pset_ActionRequest',target:1n,entries:[{name:'RequestSourceLabel',kind:'P_SINGLEVALUE',value:s('Architect')}],identities:[{key:'set',id:2n,guid:'0000000000000000000002'},{key:'entry:0',id:3n},{key:'relation',id:4n,guid:'0000000000000000000004'}]});
export const base=():IfcSnapshot=>({...empty,entities:[entity('IfcActionRequest')]});
export function fixture(target:string):IfcSnapshot{
 const fields:Record<string,IfcValue>={Name:s('selected target'),PredefinedType:{kind:'enum',value:'NOTDEFINED'}};
 const entities:IfcEntity[]=[];
 if(target==='IfcActor'){fields.TheActor=r(11n);entities.push(entity('IfcPerson',11n,{FamilyName:s('Architect')}));}
 if(target==='IfcMaterial')fields.Name=s('Steel');
 if(target==='IfcArbitraryClosedProfileDef'){fields.ProfileType={kind:'enum',value:'AREA'};fields.OuterCurve=r(14n);entities.push(entity('IfcCartesianPoint',11n,{Coordinates:a([real(0),real(0)])}),entity('IfcCartesianPoint',12n,{Coordinates:a([real(1),real(0)])}),entity('IfcCartesianPoint',13n,{Coordinates:a([real(0),real(1)])}),entity('IfcPolyline',14n,{Points:a([r(11n),r(12n),r(13n),r(11n)])}));}
 return{...empty,entities:[entity(target,1n,fields),...entities]};
}
export async function retained(snapshot:IfcSnapshot,app:Omit<IfcTemplateApplication,'identities'>):Promise<IfcTemplateApplication>{const command:IfcTemplateApplication={...app,identities:[]},keys=await ifcTemplateIdentityKeysAsync(snapshot,command);command.identities=keys.map((key,index)=>({key,id:BigInt(100+index),...(['set','relation','performance','control'].includes(key)&&!['PSET_MATERIALDRIVEN','PSET_PROFILEDRIVEN'].includes(ifc4x3Template(app.template)!.kind)?{guid:`0000000000000000000${String(index+100).padStart(3,'0')}`}:{})}));return command;}
