import{decodeBoundedArchiveV1,type BoundedArchiveEntryV1,type BoundedArchiveV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts";
import{type ResourceLeaseControl}from"../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import policy from"./📋️policy.json"with{type:"json"};
import schema from"./🧬️schema.json"with{type:"json"};
if(validateJsonSchemaSubset(schema.definitions.Policy,policy).length)throw Error("Closed SDK package archive policy required");
/** 📄️ Projects a validated package entry behind the repository-owned archive interface. */
export type NxPackageArchiveEntryV1=BoundedArchiveEntryV1;
/** 📦️ Carries only bounded package material and the consumed compressed-byte identity. */
export type NxPackageArchiveV1=BoundedArchiveV1;
/** 🗃️ Applies the finite SDK package-prefix policy to the shared Core tar framing owner. */
export function decodeNxPackageArchiveV1(input:Uint8Array,control:ResourceLeaseControl):Promise<NxPackageArchiveV1>{return decodeBoundedArchiveV1(input,{version:1,format:"tar.gz",prefix:"package",policy},control);}
