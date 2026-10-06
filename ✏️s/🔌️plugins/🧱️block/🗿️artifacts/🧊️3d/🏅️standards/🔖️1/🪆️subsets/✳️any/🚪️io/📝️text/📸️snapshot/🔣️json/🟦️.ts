/** 🔣️ Physical Block 3D snapshot JSON codec. */
import {parseBlock3dSnapshot, type Block3dSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import * as p from "../../../../../../../../../../🧬️schema/🧱️shared/📐️scalar/🟦️.ts";
import * as sharedJson from "../../../../../../../../../../🧬️schema/🧱️shared/🚪️io/🔣️json/🟦️.ts";

function jsonGeometry(value:any,decode:boolean):void{
 const scalar=(word:any)=>decode?sharedJson.word(word):sharedJson.out(p.word(word));
 const fields=(record:any,arrays:readonly string[],single:string)=>{for(const key of arrays)if(record[key]!==undefined){if(!Array.isArray(record[key]))throw Error("Block3d JSON vector differs");record[key]=record[key].map(scalar)}if(record[single]!==undefined)record[single]=scalar(record[single])};
 if(value.vortices!==undefined){if(!Array.isArray(value.vortices))throw Error("Block3d JSON vortices differ");for(const vortex of value.vortices)fields(vortex,["position","direction"],"radius")}
 if(value.camera3d!==undefined)fields(value.camera3d,["position","target"],"zoom");
}

/** 🔣️ Emit the actual owner with exact Binary64Transport at authored geometry roles. */
export function block3dSnapshotToJsonText(value:Block3dSnapshot):string{const owned=parseBlock3dSnapshot(value);jsonGeometry(owned,false);return JSON.stringify(owned)}

/** 📥️ Read only declared floating roles before the typed Snapshot parser. */
export function block3dSnapshotFromJsonText(text:string):Block3dSnapshot{const value=JSON.parse(text);jsonGeometry(value,true);return parseBlock3dSnapshot(value)}
