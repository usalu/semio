import {join} from "node:path";
import {issueRepositoryScriptProvidersV1} from "../../../../🏃️process/🔌️providers/🟦️.ts";
import {SCRIPT_PROVIDERS_ENV} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🚪️ What the runner hands the entry: the isolated workspace, the framework ports whose implementation the class replaces, the time bound of the original process and the build mode. */
export interface HermeticEntryOptionsV1{readonly workspace:string;readonly providers:Readonly<Record<string,string>>;readonly maximumElapsedMilliseconds:number;readonly buildMode:string}

/** 🎬️ Issues, inside a hermetic action, the original process authority a repository script receives from its caller: the repository provider designation with the class replacements, and a fresh original envelope bounded in time whose only capability is the isolated workspace; scripts that need native build authority refuse loudly. The runner overlays the returned values on its strict environment. Owner of the wire formats: [providers](../../../../🏃️process/🔌️providers/🟦️.ts) and the routing envelope. */
export function hermeticEnvironmentV1(options:HermeticEntryOptionsV1):Record<string,string>{
 const environment:Record<string,string|undefined>={};
 issueRepositoryScriptProvidersV1(environment as NodeJS.ProcessEnv);
 const designation=JSON.parse(environment[SCRIPT_PROVIDERS_ENV]!) as {version:1;providers:Record<string,string>};
 for(const[name,path]of Object.entries(options.providers))designation.providers[name]=join(options.workspace,path);
 environment[SCRIPT_PROVIDERS_ENV]=JSON.stringify(designation);
 const envelope=createScriptProcessEnvelope({version:1,owner:"bazel:hermetic",maximumElapsedMilliseconds:options.maximumElapsedMilliseconds} as never,{tool:"bazel",workspaceRoot:options.workspace,arguments:["repo"],selection:["repo"],buildMode:options.buildMode,launches:[["repo"]]},Date.now());
 return Object.fromEntries(Object.entries(scriptProcessEnvironment(envelope,environment)).filter((entry):entry is [string,string]=>entry[1]!==undefined));
}
