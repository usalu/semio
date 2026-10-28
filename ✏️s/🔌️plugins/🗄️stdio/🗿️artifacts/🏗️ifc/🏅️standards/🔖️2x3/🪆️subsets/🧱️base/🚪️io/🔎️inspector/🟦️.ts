import {parseIfcEntityIdentity,ifcEntityIdentity} from "../../../../../../🪪️identity/🟦️.ts";
import type {Ifc2x3Snapshot} from "../../🧬️schema/📸️snapshot/🟦️.ts";
export {parseIfcEntityIdentity as parseIfc2x3SelectionIdentity,ifcEntityIdentity as ifc2x3SelectionIdentity};
/** 🎯️ Resolve a stable IFC2x3 inspector identity under the initiating own snapshot. */
export function ifc2x3SelectionPath(snapshot:Ifc2x3Snapshot,identity:string,suffix:string,tick:()=>void=()=>{}):string{const id=parseIfcEntityIdentity(identity);if(id===undefined||suffix.length>8192||suffix!==""&&!suffix.startsWith("/"))throw new Error("ifc2x3-selection-path");for(const [row,instance]of snapshot.document.instances.entries()){tick();if(instance.id===id)return "/document/instances/"+row+suffix;}throw new Error("ifc2x3-selection-missing");}
