/** 🎛️ Flattened positions use actual endpoint components and typed numeric offsets. */
import type {JackArtifact} from "../../🟦️.ts";
import type {SemioGraphSnapshot,SemioGraphEdge} from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts";
import {ValueError} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts";
import {inferJackTopology} from "../🧭topology/🟦️.ts";
export interface JackFlatPositionUv {u:number;v:number}
export interface JackFlatPosition {positions:Record<string,JackFlatPositionUv>}
function offset(edge:SemioGraphEdge,key:string):number {
 const values=edge.properties.filter(value=>value.key===key);if(values.length>1)throw new ValueError("invalidValue","Jack flattened offset requires unique property key");const value=values[0]?.value;if(value===undefined)return 0;
 if(value.kind!=="float"&&value.kind!=="int")throw new ValueError("invalidValue","Jack flattened offset requires numeric intrinsic family");
 const text=value.lexeme.trim();
 if(value.kind==="float"&&/^nan64_[0-9a-fA-F]{16}$/.test(text)){const word=BigInt("0x"+text.slice(6));if((word&0x7ff0000000000000n)!==0x7ff0000000000000n||(word&0xfffffffffffffn)===0n)throw new ValueError("invalidValue","Jack flattened NaN word has wrong class");const bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,word);return bytes.getFloat64(0);}
 if(value.kind==="int"){if(!/^-?(0|[1-9][0-9]*)$/.test(text))throw new ValueError("invalidValue","Jack flattened integer lexeme invalid");return Number(BigInt(text));}
 if(/^[+-]?(inf|infinity)$/i.test(text))return text.startsWith("-")?-Infinity:Infinity;
 if(/^[+-]?nan$/i.test(text))return NaN;
 if(!/^[+-]?(([0-9]+(\.[0-9]*)?)|(\.[0-9]+))([eE][+-]?[0-9]+)?$/.test(text))throw new ValueError("invalidValue","Jack flattened binary64 lexeme invalid");
 return Number(text);
}
/** 🪆️ Missing child and invalid endpoint ownership propagate before the derived position walk. */
export function inferJackFlatPosition(parent:JackArtifact,children:ReadonlyMap<string,SemioGraphSnapshot>):JackFlatPosition {
 inferJackTopology(parent,children);
 const child=children.get(parent.content.childId)!,ids=child.nodes.map(node=>node.id.value).sort(),positions:Record<string,JackFlatPositionUv>=Object.create(null),edges=[...child.edges].sort((a,b)=>a.id.value<b.id.value?-1:a.id.value>b.id.value?1:0);
 const offsets=new Map(edges.map(edge=>[edge.id.value,{u:offset(edge,"u"),v:offset(edge,"v")}]));
 const visit=(seed:string)=>{if(Object.hasOwn(positions,seed))return;positions[seed]={u:0,v:0};const stack=[seed];while(stack.length){const source=stack.pop()!,held=positions[source]!;for(const edge of edges){if(edge.source.value!==source||Object.hasOwn(positions,edge.target.value))continue;const delta=offsets.get(edge.id.value)!;positions[edge.target.value]={u:held.u+delta.u,v:held.v+delta.v};stack.push(edge.target.value);}}};
 if(ids.length){const seed=parent.rootNodeId??ids[0]!;if(!ids.includes(seed))throw new ValueError("invalidValue","Jack flattened root requires retained node");visit(seed);}
 while(Object.keys(positions).length<ids.length){const remaining=ids.filter(id=>!Object.hasOwn(positions,id)),seed=remaining.find(id=>!edges.some(edge=>edge.target.value===id&&remaining.includes(edge.source.value)))??remaining[0]!;visit(seed);}
 return{positions};
}
