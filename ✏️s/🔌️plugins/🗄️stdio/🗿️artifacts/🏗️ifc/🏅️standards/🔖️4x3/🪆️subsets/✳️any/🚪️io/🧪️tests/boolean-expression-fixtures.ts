/** 🌳️ Every source node is an explicit real IFC declaration; keys only link actual operands. */
import {ifcBlockFixture} from './csg-block-fixtures.ts';
import {ifcReal} from '../step.ts';
import type {IfcEntity,IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
export type IfcBooleanExpressionNode={key:string;kind:'block';origin:number[];size:number[]}|{key:string;kind:'boolean';operator:string;first:string;second:string};
export interface IfcBooleanExpressionCase{id:string;root:string;rootForm:string;nodes:IfcBooleanExpressionNode[];expected:string;volume:number|null;physical:string;}
export function ifcBooleanExpressionFixture(row:IfcBooleanExpressionCase){
 const initial=row.nodes.find(node=>node.kind==='block');if(!initial||initial.kind!=='block')throw Error('Authored expression requires a real initial block');
 const fixture=ifcBlockFixture({id:row.id,dimensions:initial.size,position:{origin:initial.origin,axis:null,refDirection:null},unitPrefix:null,mapping:[],expected:row.expected,bounds:null,volume:row.volume,damage:'none'}),nodes=new Map<string,IfcEntity>([[initial.key,fixture.block]]),unset:IfcValue={kind:'unset'},ref=(entity:IfcEntity|undefined):IfcValue=>({kind:'reference',value:entity?.id??999999999n}),put=(name:string,args:IfcValue[])=>{fixture.add(name,args);return fixture.snapshot.entities.at(-1)!;};
 for(const node of row.nodes){if(nodes.has(node.key))continue;if(node.kind==='block'){const point=put('IFCCARTESIANPOINT',[{kind:'aggregate',value:node.origin.map(ifcReal)}]),position=put('IFCAXIS2PLACEMENT3D',[ref(point),unset,unset]);nodes.set(node.key,put('IFCBLOCK',[ref(position),...node.size.map(ifcReal)]));}else nodes.set(node.key,put('IFCBOOLEANRESULT',[{kind:'enum',value:node.operator},unset,unset]));}
 for(const node of row.nodes)if(node.kind==='boolean'){const entity=nodes.get(node.key)!;entity.args[1]=ref(nodes.get(node.first));entity.args[2]=ref(nodes.get(node.second));}
 const expression=nodes.get(row.root);if(!expression)throw Error('Authored expression root must exist');fixture.solid.args[0]=ref(expression);
 if(row.rootForm==='direct'){fixture.representation.args[3]={kind:'aggregate',value:[ref(expression)]};fixture.snapshot.entities=fixture.snapshot.entities.filter(entity=>entity.id!==fixture.solid.id);}
 return{...fixture,nodes,expression,root:row.rootForm==='direct'?expression:fixture.solid};
}
