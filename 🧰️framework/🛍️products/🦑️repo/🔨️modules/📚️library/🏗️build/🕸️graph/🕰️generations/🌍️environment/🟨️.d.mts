import type {GenerationRowV1} from "../🟨️.mjs";
export type EnvironmentExtractorV1="project-json"|"package-json"|"go-mod"|"cargo-package";
export interface EnvironmentProjectV1{name:string;root:string;nativeRoot?:string;config?:string}
export interface EnvironmentReadersV1{read(path:string):Promise<string|undefined>;stat(path:string):Promise<{size:number;mtimeMs:number}|undefined>}
/** 🌍️ The value one manifest contributes to the environment, or null when the manifest does not exist. */
export function extractEnvironmentFieldV1(extractor:EnvironmentExtractorV1,text:string|undefined):unknown;
export function environmentRowHoldsV1(row:GenerationRowV1,readers:EnvironmentReadersV1):Promise<boolean>;
export function deriveEnvironmentV1(input:EnvironmentReadersV1&{projects:readonly EnvironmentProjectV1[];workspaceManifests:readonly string[];lock?:string;taxonomy?:string}):Promise<{digest:string;rows:GenerationRowV1[]}>;
