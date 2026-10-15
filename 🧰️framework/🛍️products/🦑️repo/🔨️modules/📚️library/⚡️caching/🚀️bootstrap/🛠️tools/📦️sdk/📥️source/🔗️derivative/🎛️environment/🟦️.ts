import schema from "../🧬️schema/🔣️.json";

/** 🎛️ Closes runtime options for the bounded SDK compiler and preflight child roles. */
export function nxSdkCompilerEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string|undefined>>{
 for(const name of schema.definitions.RefusedRuntimeEnvironment.const)if(environment[name])throw Error("Closed SDK compiler runtime options required");
 return Object.freeze({...environment,...schema.definitions.RuntimeEnvironment.const});
}
