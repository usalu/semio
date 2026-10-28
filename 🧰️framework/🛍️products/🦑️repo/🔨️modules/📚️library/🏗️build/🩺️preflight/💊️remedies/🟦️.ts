import texts from "./📖️texts/🔣️.json" with {type:"json"};
import {existsSync} from "node:fs";
import type {BuildPreflightFactsV1} from "../🟦️.ts";

/** 🏠️ What the remedies also depend on besides the facts: whether the workspace runs inside a container. */
export type BuildPreflightContextV1=Readonly<{container:boolean}>;

/** 🐳️ Whether this process runs inside a container, from the marker files and variables the container tools set; read-only and without a process start. See https://containers.dev/implementors/spec/ */
export function buildPreflightContainerV1(environment:Readonly<Record<string,string|undefined>>=process.env,exists:(path:string)=>boolean=existsSync):boolean{
 return environment.REMOTE_CONTAINERS==="true"||environment.DEVCONTAINER==="true"||environment.CODESPACES==="true"||exists("/.dockerenv")||exists("/run/.containerenv");
}

/** 💊️ Names, in English and German, what to do about each refused requirement: a swap-less filesystem (with the container advice inside one), a case-folding filesystem per platform, a missing tool per platform. A refusal without a declared remedy gets none; the problem lines stay the authority. */
export function buildPreflightRemediesV1(facts:Pick<BuildPreflightFactsV1,"platform"|"tools"|"exchange"|"caseSensitive">,requiredTools:readonly string[],requiredExchange:boolean,context:BuildPreflightContextV1):string[]{
 const rows:{en:string;de:string}[]=[],platform=facts.platform as "linux"|"darwin"|"win32";
 if(requiredExchange&&facts.exchange!=="supported"){rows.push(texts.exchange);if(context.container)rows.push(texts.exchangeContainer);}
 if(requiredExchange&&facts.caseSensitive==="no"){const row=(texts.caseFolding as Record<string,{en:string;de:string}>)[platform];if(row)rows.push(row);}
 for(const tool of requiredTools)if(!facts.tools.includes(tool)){const row=(texts.tools as Record<string,Record<string,{en:string;de:string}>>)[tool]?.[platform];if(row)rows.push(row);}
 return rows.flatMap(row=>[`${texts.labels.en}: ${row.en}`,`${texts.labels.de}: ${row.de}`]);
}
