import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-rs/index";
import { artifactsStorageSchema as limits } from "@semio-tech/repo-lib/artifacts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {createHash} from "node:crypto";
export type PlayPagesModeV1="prototype"|"release";
export type PlayPageManifestV1=Readonly<{version:1;mode:PlayPagesModeV1;family:string;files:readonly Readonly<{path:string;bytes:number;sha256:string}>[]}>;
export type PlayPageCohortV1=Readonly<{version:1;mode:PlayPagesModeV1;pages:readonly Readonly<{family:string;manifestSha256:string}>[]}>;
const policy=limits.definitions.Protocol.const;
function finite(value:unknown,definition:"Page"|"Cohort"):void{if(validateJsonSchemaSubset(schema.definitions[definition],value).length||Buffer.byteLength(JSON.stringify(value))>policy.maximumManifestBytes)throw Error("Closed bounded Play "+definition+" required");}
/** 🧾️ Copies one closed cohort descriptor without conferring publication or reader authority. */
export function readPlayPageCohortV1(value:unknown,mode:PlayPagesModeV1):PlayPageCohortV1{
 finite(value,"Cohort");const row=value as PlayPageCohortV1;if(row.mode!==mode||new Set(row.pages.map(page=>page.family)).size!==row.pages.length||!row.pages.some(page=>page.family==="play"))throw Error("Distinct exact-mode Play cohort required");return Object.freeze({...row,pages:Object.freeze(row.pages.map(page=>Object.freeze({...page})))});
}
/** 📄️ Copies the bounded content roster of one exact declared family. */
export function readPlayPageManifestV1(value:unknown,mode:PlayPagesModeV1,family:string):PlayPageManifestV1{
 finite(value,"Page");const row=value as PlayPageManifestV1;if(row.mode!==mode||row.family!==family||row.files.length>=policy.maximumFiles||new Set(row.files.map(file=>file.path)).size!==row.files.length||row.files.some(file=>file.path===schema.definitions.Protocol.const.manifest||/[\\:\u0000-\u001f\u007f]/u.test(file.path)||file.path.split("/").some(part=>!part||part==="."||part===".."))||row.files.reduce((sum,file)=>sum+file.bytes,0)>policy.maximumOutputBytes)throw Error("Exact finite Play family roster required");return Object.freeze({...row,files:Object.freeze(row.files.map(file=>Object.freeze({...file})))});
}
/** 🔗️ Checks DATA consistency with exact bytes; live callers must retain their real artifact readers. */
export function requirePlayPageCohortMemberV1(value:unknown,family:string,bytes:Uint8Array,mode:PlayPagesModeV1):PlayPageManifestV1{
 const cohort=readPlayPageCohortV1(value,mode),member=cohort.pages.find(row=>row.family===family);if(bytes.byteLength>policy.maximumManifestBytes||!member||createHash("sha256").update(bytes).digest("hex")!==member.manifestSha256)throw Error("Mixed or undeclared Play page cohort refused");return readPlayPageManifestV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes)),mode,family);
}
