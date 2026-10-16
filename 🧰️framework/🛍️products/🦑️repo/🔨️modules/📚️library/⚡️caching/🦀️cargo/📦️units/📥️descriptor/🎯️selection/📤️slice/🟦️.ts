import{createHash}from"node:crypto";
import type{RustUnitV1}from"../../../🟦️.ts";
/** 🧩️ Names only one selected unit and its exact transitive compiler predecessors. */
export function selectedRustDescriptorSliceV1(units:readonly RustUnitV1[],selected:string):{units:RustUnitV1[];encoded:string;digest:string}{
 const byId=new Map(units.map(unit=>[unit.id,unit])),closure=new Map<string,RustUnitV1>(),active=new Set<string>();if(byId.size!==units.length)throw Error("Duplicate selected descriptor unit");const visit=(id:string):void=>{if(closure.has(id))return;if(active.has(id))throw Error("Cyclic selected descriptor predecessors");const unit=byId.get(id);if(!unit)throw Error("Selected descriptor predecessor missing");active.add(id);for(const edge of unit.dependencies)visit(edge.unit);active.delete(id);closure.set(id,unit);};visit(selected);const selectedUnits=[...closure.values()],encoded=JSON.stringify({version:1,units:selectedUnits});if(Buffer.byteLength(encoded)>4194304)throw Error("Selected Rust descriptor byte capacity exceeded before publication");return{units:selectedUnits,encoded,digest:createHash("sha256").update(encoded).digest("hex")};
}
