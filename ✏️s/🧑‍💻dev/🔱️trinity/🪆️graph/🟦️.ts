/** 🔱️ Specific installation interprets Semio graph literals for intrinsic Jack queries. */
import type {ArtifactChild,JackGraphQuery,JackFlatPositionUv} from "@semio-tech/trinity-jack/source";
import type {SemioGraphSnapshot,SemioGraphEdge} from "@semio-tech/stdio-semio/source";
import {ValueError} from "../../../../🧰️framework/📦️packages/🟦️typescript/🌱️value/⚠️refusal/🟦️.ts";
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
/** 🪆️ Bind the selected retained Semio owner without copying its rich graph snapshot. */
export function bindSemioJackGraph(identity:ArtifactChild,graph:SemioGraphSnapshot):JackGraphQuery {
 const dialect=identity.target.dialect;
 if(dialect.artifactKind!=="s.stdio.semio"||dialect.standard!=="v1"||dialect.subset!=="graph")throw new ValueError("invalidValue","Jack composition requires exact Semio graph child dialect");
 const edges=new Map(graph.edges.map(edge=>[edge.id.value,edge]));
 return{identity,*nodeIds(){for(const node of graph.nodes)yield node.id.value;},*edges(){for(const edge of graph.edges)yield{id:edge.id.value,source:edge.source.value,target:edge.target.value};},displacement(id:string):JackFlatPositionUv{const edge=edges.get(id);if(!edge)throw new ValueError("invalidValue","Jack displacement requires retained edge");return{u:offset(edge,"u"),v:offset(edge,"v")};}};
}
