/** 📰️ Ignore nonalphabet octets while retaining sparse original-source offsets. */
import {ifcCheckpoint,IfcStepError,type IfcIoControl} from "./control.ts";
export interface IfcStepSource {readonly text:string;readonly originalLength:number;offset(index:number):number}

/** 🔎️ ISO 10303-21 clauses 5.2 and 5.6 ignore C0/DEL characters even inside tokens. */
export async function prepareIfcStepSource(source:string,control:IfcIoControl):Promise<IfcStepSource>{
 let text="",chunk="",removed=0,pending=false;const gaps:{at:number;removed:number}[]=[];
 for(let at=0;at<source.length;at++){const code=source.charCodeAt(at);if(pending){if(code<0xdc00||code>0xdfff)throw new IfcStepError("step.string","Unpaired raw Unicode surrogate",at-1);pending=false;}else if(code>=0xd800&&code<=0xdbff)pending=true;else if(code>=0xdc00&&code<=0xdfff)throw new IfcStepError("step.string","Unpaired raw Unicode surrogate",at);if(code<32||code===127){removed++;const normalized=at+1-removed;if(gaps.at(-1)?.at===normalized)gaps[gaps.length-1]!.removed=removed;else gaps.push({at:normalized,removed});}else{chunk+=source[at]!;if(chunk.length===4096){text+=chunk;chunk="";}}if((at+1)%4096===0)await ifcCheckpoint(control,"parse-source",at+1,source.length);}
 if(pending)throw new IfcStepError("step.string","Unpaired raw Unicode surrogate",source.length-1);
 text+=chunk;return{text:removed?text:source,originalLength:source.length,offset(index:number):number{let low=0,high=gaps.length;while(low<high){const middle=Math.floor((low+high)/2);if(gaps[middle]!.at<=index)low=middle+1;else high=middle;}return index+(low?gaps[low-1]!.removed:0);}};
}
