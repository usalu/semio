export interface ArtifactGenesis<Snapshot>{readonly snapshot:Snapshot;readonly digest:readonly number[]}
export function admitArtifactGenesis<Snapshot>(value:unknown,admitSnapshot:(value:unknown)=>Snapshot):ArtifactGenesis<Snapshot>{
 if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Genesis requires typed facts");
 const fields=value as Record<string,unknown>;
 if(Object.keys(fields).length!==2||!Object.hasOwn(fields,"snapshot")||!Object.hasOwn(fields,"digest"))throw Error("Genesis requires snapshot and digest only");
 if(!Array.isArray(fields.digest)||fields.digest.length!==32||fields.digest.some(word=>!Number.isInteger(word)||word<0||word>255))throw Error("Genesis digest requires32 unsigned words");
 return{snapshot:admitSnapshot(fields.snapshot),digest:fields.digest.slice()};
}
