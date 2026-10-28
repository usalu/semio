/** 🧫️ Neutral graph fixtures preserve declaration-owned direct and inherited positions. */
import {ifc4x3Catalog,type IfcEntityDeclaration} from '../../../📚️catalog/🟦️.ts';
import type {IfcEntity,IfcSnapshot,IfcValue} from '../../../📸️snapshot/🟦️.ts';
import {ifcReal} from '../../../../🚪️io/step-values.ts';

export interface AlignmentCase {kind:string;lengths:readonly number[];variant?:string}
const names:Record<string,readonly [string,string,string]>={horizontal:['IFCALIGNMENTHORIZONTAL','IFCALIGNMENTHORIZONTALSEGMENT','SegmentLength'],vertical:['IFCALIGNMENTVERTICAL','IFCALIGNMENTVERTICALSEGMENT','HorizontalLength'],cant:['IFCALIGNMENTCANT','IFCALIGNMENTCANTSEGMENT','HorizontalLength']};
const ref=(value:bigint):IfcValue=>({kind:'reference',value});
const aggregate=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value});
function entity(id:bigint,name:string,fields:Record<string,IfcValue>={}):IfcEntity{return {id,name,args:ifc4x3Catalog.entities[name]!.allAttributes.map(item=>fields[item.name]??{kind:'unset'}),complex:[]};}
function field(row:IfcEntity,name:string,value:IfcValue):void{const index=ifc4x3Catalog.entities[row.name]!.allAttributes.findIndex(item=>item.name===name);if(index<0)throw Error('Missing fixture declaration '+name);row.args[index]=value;}
function complex(row:IfcEntity):void{
 const all=ifc4x3Catalog.entities[row.name]!.allAttributes,args=row.args,parts:{name:string;args:IfcValue[]}[]=[];let name:string|null=row.name;
 while(name){const declaration:IfcEntityDeclaration=ifc4x3Catalog.entities[name]!;parts.push({name,args:declaration.attributes.map(item=>args[all.findIndex(attribute=>attribute.name===item.name)]!)});name=declaration.supertype;}
 row.name=parts[0]!.name;row.args=parts[0]!.args;row.complex=parts.slice(1);
}
export function alignmentFixture(row:AlignmentCase):IfcSnapshot{
 const [layoutName,parameterName,lengthName]=names[row.kind]!,layout=entity(1n,layoutName),members:IfcValue[]=[],entities=[layout];
 for(let i=0;i<row.lengths.length;i++){const id=3n+BigInt(i)*2n,segment=entity(id,'IFCALIGNMENTSEGMENT',{DesignParameters:ref(id+1n)}),parameter=entity(id+1n,parameterName,{[lengthName]:ifcReal(row.lengths[i]!)});members.push(ref(id));entities.push(segment,parameter);}
 const nesting=entity(2n,'IFCRELNESTS',{RelatingObject:ref(1n),RelatedObjects:aggregate(members)});entities.splice(1,0,nesting);
 const last=entities.at(-1)!,terminal=entities.at(-2),variant=row.variant;
 if(variant==='signedzero')field(last,lengthName,ifcReal(-0));
 if(variant==='wrong-design')Object.assign(last,entity(last.id,row.kind==='vertical'?'IFCALIGNMENTHORIZONTALSEGMENT':'IFCALIGNMENTVERTICALSEGMENT'));
 if(variant==='complex')for(const item of entities)complex(item);
 if(variant==='duplicate-target')entities.push(structuredClone(layout));
 if(variant==='missing-target')entities.shift();
 if(variant==='wrong-target')Object.assign(layout,entity(layout.id,row.kind==='vertical'?'IFCALIGNMENTHORIZONTAL':'IFCALIGNMENTVERTICAL'));
 if(variant==='unknown-target')layout.name='IFCUNKNOWNLAYOUT';
 if(variant==='mixed-layout')layout.complex=[{name:'IFCALIGNMENTVERTICAL',args:[]}];
 if(variant==='competing-lists')entities.push({...structuredClone(nesting),id:99n});
 if(variant==='unrelated-relationship')nesting.name='IFCRELAGGREGATES';
 if(variant==='other-owner'){field(nesting,'RelatingObject',ref(99n));field(nesting,'OwnerHistory',ref(1n));}
 if(variant==='duplicate-attribute'){complex(nesting);nesting.complex!.push({name:'IFCRELNESTS',args:structuredClone(nesting.args)});}
 if(variant==='nonreference-member')members[members.length-1]={kind:'string',value:'0'};
 if(variant==='dangling-terminal')members[members.length-1]=ref(99n);
 if(variant==='ambiguous-terminal')entities.push(structuredClone(terminal!));
 if(variant==='wrong-member')terminal!.name='IFCWALL';
 if(variant==='dangling-design')field(terminal!,'DesignParameters',ref(99n));
 if(variant==='typed-measure')field(last,lengthName,{kind:'typedValue',value:{name:'IFCNONNEGATIVELENGTHMEASURE',items:[{kind:'typedValue',value:{name:'IFCLENGTHMEASURE',items:[ifcReal(0)]}}]}});
 if(variant==='wrong-measure-tag')field(last,lengthName,{kind:'typedValue',value:{name:'IFCPARAMETERVALUE',items:[ifcReal(0)]}});
 if(variant==='typed-multiple-items')field(last,lengthName,{kind:'typedValue',value:{name:'IFCNONNEGATIVELENGTHMEASURE',items:[ifcReal(0),ifcReal(0)]}});
 if(variant==='integer-zero'||variant==='integer-large')field(last,lengthName,{kind:'integer',value:variant==='integer-zero'?0n:9223372036854775807n});
 if(variant==='tiny')field(last,lengthName,ifcReal(Number.MIN_VALUE));
 if(variant==='negative')field(last,lengthName,ifcReal(-1));
 if(variant==='string-zero')field(last,lengthName,{kind:'string',value:'0'});
 if(variant==='unknown-complex')layout.complex=[{name:'IFCUNKNOWN',args:[]}];
 if(variant==='earlier-unresolved'){members[0]=ref(99n);field(last,lengthName,ifcReal(10));}
 return {schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[aggregate([{kind:'string',value:'IFC4X3_ADD2'}])]},entities};
}
