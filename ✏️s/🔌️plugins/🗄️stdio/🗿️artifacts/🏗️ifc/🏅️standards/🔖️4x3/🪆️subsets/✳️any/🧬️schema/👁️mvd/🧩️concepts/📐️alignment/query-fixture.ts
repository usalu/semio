/** 🧬️ Internal complex projections retain canonical declared owners without asserting population legality. */
import {alignmentFixture} from './🧪️fixture.ts';
import {ifc4x3Catalog} from '../../../📚️catalog/🟦️.ts';
import {ifcReal} from '../../../../🚪️io/step-values.ts';
import type {IfcValue} from '../../../📸️snapshot/🟦️.ts';
export function alignmentQueryFixture(row:{kind:string;variant:string}){
 const source=alignmentFixture({kind:row.kind,lengths:[10,0],variant:'complex'}),unknown={name:'IFCUNKNOWN',args:[] as IfcValue[]},relation=source.entities[1]!,parameter=source.entities.at(-1)!;
 const part=(name:string,fields:Record<string,IfcValue>)=>({name,args:ifc4x3Catalog.entities[name]!.attributes.map(attribute=>fields[attribute.name]??{kind:'unset' as const})});
 if(row.variant.startsWith('foreign-relationship-owner'))relation.complex!.push(part('IFCRELAGGREGATES',{RelatingObject:{kind:'reference',value:99n},RelatedObjects:{kind:'aggregate',value:[]}}));
 if(row.variant.startsWith('foreign-parameter-owner'))parameter.complex!.push(part('IFCCURVESEGMENT',{SegmentLength:{kind:'typedValue',value:{name:'IFCLENGTHMEASURE',items:[ifcReal(0)]}}}));
 if(row.variant==='duplicate-relationship-owner')relation.complex!.push({name:relation.name,args:structuredClone(relation.args)});
 const id=row.variant==='target-late-unknown'?1n:row.variant==='segment-late-unknown'?5n:row.variant==='parameter-late-unknown'||row.variant==='foreign-parameter-owner-late-unknown'?6n:row.variant==='relation-late-unknown'||row.variant==='foreign-relationship-owner-late-unknown'?2n:undefined;
 if(id!==undefined)source.entities.find(entity=>entity.id===id)!.complex!.push(unknown);
 if(row.variant==='discovery-unknown-only')source.entities.push({id:99n,name:'IFCUNKNOWN',args:[],complex:[]});
 return source;
}
