import type {GenerationRowV1} from "../🟨️.mjs";
import type {EnvironmentPartsV1} from "../🧵️edge-reads/🟨️.mjs";
export type EnvironmentExtractorV1="project-json"|"package-json"|"go-mod"|"cargo-package";
export interface EnvironmentProjectV1{name:string;root:string;nativeRoot?:string;config?:string}
export interface EnvironmentReadersV1{read(path:string):Promise<string|undefined>;stat(path:string):Promise<{size:number;mtimeMs:number}|undefined>}
/** 🔏️ The digest of a JSON data file without the top level sections nothing of the graph reads. */
export function sectionalDigestV1(text:string,ignore:readonly string[]):string;
/** 🌍️ The value one manifest contributes to the environment, or null when the manifest does not exist. */
export function extractEnvironmentFieldV1(extractor:EnvironmentExtractorV1,text:string|undefined):unknown;
export function environmentRowHoldsV1(row:GenerationRowV1,readers:EnvironmentReadersV1):Promise<boolean>;
export function deriveEnvironmentV1(input:EnvironmentReadersV1&{projects:readonly EnvironmentProjectV1[];workspaceManifests:readonly string[];lock?:string;taxonomy?:string}):Promise<{digest:string;rows:GenerationRowV1[];parts:EnvironmentPartsV1}>;
/** 🌍️ The environment in its parts, each list in the canonical order of the digest. */
export function environmentPartsV1(input:{owners:readonly (readonly [string,string,string|null])[];packages:readonly (readonly [string,string])[];modules:readonly (readonly [string,string])[];workspaces:readonly (readonly [string,string])[];lock?:string|null;taxonomy?:string|null}):EnvironmentPartsV1;
/** 🔏️ The digest of an environment from its parts. */
export function environmentDigestV1(parts:EnvironmentPartsV1):string;
