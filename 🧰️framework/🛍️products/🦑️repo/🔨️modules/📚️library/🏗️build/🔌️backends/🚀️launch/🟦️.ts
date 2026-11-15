import {buildBackendIdsV1,buildBackendRowV1,requireBuildBackendIdV1} from "../🟦️.ts";

/** 🚀️ What the door needs of a backend before any kernel runs: the arguments its own tool is launched with. */
export interface BuildBackendLaunchV1{bootstrapArguments(args:readonly string[]):readonly string[]|undefined}

/** 🚀️ The launch a backend's own tool needs for a command, asked of the backends the command concerns: the selected one, or the named one for `tools setup`; the namespace is the first argument the backend's nested launches carry. `undefined` when none launches anything. The launch modules are small and separate from the adapters so the door captures no adapter.
 * @see ../🚚️loading/🟦️.ts the adapters, loaded by kernels only */
export async function buildBackendBootstrapV1(args:readonly string[],tool:string):Promise<Readonly<{namespace:string;arguments:readonly string[]}>|undefined>{
 const setup=args[0]==="tools"&&args[1]==="setup";
 if(setup&&args[2]==="toolchains")return undefined;
 const target=setup?args[2]??tool:tool,ids=target==="all"?buildBackendIdsV1():[requireBuildBackendIdV1(target)];
 for(const id of ids){
  const row=buildBackendRowV1(id);
  if(!row.launch)continue;
  const launch=((await import(new URL(row.launch,new URL("../🟦️.ts",import.meta.url)).href)) as {launch:BuildBackendLaunchV1}).launch.bootstrapArguments(args),namespace=row.nativeCaller?.namespace;
  if(launch){if(!namespace)throw Error(`Build backend ${id} launches its tool and must declare nativeCaller`);return {namespace,arguments:launch};}
 }
 return undefined;
}
