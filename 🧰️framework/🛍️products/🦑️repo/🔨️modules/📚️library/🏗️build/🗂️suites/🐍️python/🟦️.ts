import{join,posix,win32}from"node:path";
import protocol from"./🔣️.json"with{type:"json"};

/** 🐍️ Reads one exact interpreter path emitted by the declared managed interpreter query. */
export function readWorkspaceTestPythonOutputV1(output:string,directory:string,platform:string=process.platform):string{
 const path=output.endsWith("\r\n")?output.slice(0,-2):output.endsWith("\n")?output.slice(0,-1):output,host=platform==="win32"?win32:posix;
 if(!path||path.length>protocol.maximumPathUnits||/[\r\n\0]/u.test(path)||!host.isAbsolute(path)||platform==="win32"&&host.parse(path).root.length<2)throw Error("Managed Python query requires one absolute interpreter path");
 const relative=host.relative(directory,path);
 if(!relative||relative===".."||relative.startsWith(".."+host.sep)||host.isAbsolute(relative))throw Error("Managed Python interpreter escaped its declared managed interpreter root");
 return path;
}

/** 🧭️ Builds a read-only query for the managed workspace interpreter series. */
export function workspaceTestPythonArgumentsV1(series:string):string[]{return[...protocol.query,series,...protocol.queryOptions];}

/** 🌿️ Selects the verified manager and suppresses conflicting uv preference. */
export function workspaceTestPythonVariablesV1(environment:NodeJS.ProcessEnv,planned:NodeJS.ProcessEnv,locator:string,manager:string,platform:NodeJS.Platform=process.platform):NodeJS.ProcessEnv{
 const ambient={...environment};
 if(platform==="win32"){const protectedNames=new Set([...Object.keys(planned),"UV_PYTHON_PREFERENCE",locator].map(name=>name.toUpperCase()));for(const name of Object.keys(ambient))if(protectedNames.has(name.toUpperCase()))delete ambient[name];}
 return{...ambient,...planned,UV_PYTHON_PREFERENCE:undefined,[locator]:manager};
}

/** 🛑️ Rejects cancelled or failed discovery with bounded sanitized diagnostics. */
export function requireWorkspaceTestPythonSuccessV1(result:{status:number|null;signal:string|null;stderr:string},signal:AbortSignal):void{
 signal.throwIfAborted();
 if(result.status!==0||result.signal!==null)throw Error(`Managed Python discovery failed (${result.signal??result.status}): ${result.stderr.slice(0,protocol.maximumOutputBytes).replace(/[\u0000-\u001f\u007f]/gu,"")}`);
}

/** 🎟️ Resolves the installed workspace interpreter through registered uv without creating an environment or downloading. */
export async function workspaceTestPythonEnvironmentV1(checkout:string,environment:NodeJS.ProcessEnv,signal:AbortSignal):Promise<NodeJS.ProcessEnv>{
 const{pythonEnvironmentOfProjectV1,nativeDistributionV1,toolchainHostV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts"),{repositoryCachePartitionRootV1}=await import("../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),{runToolV1}=await import("../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts"),{findToolV1}=await import("../../../../../../../🔨️modules/🏃️process/🧰️tools/📖️registry/🟦️.ts");
 signal.throwIfAborted();
 const{preparedNativeToolV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts"),toolsRoot=repositoryCachePartitionRootV1(checkout,"tools"),host=toolchainHostV1(),managerLocator=findToolV1(protocol.manager)?.locate.find(row=>row.kind==="environment");
 if(!managerLocator||managerLocator.kind!=="environment")throw Error("Registered uv environment locator required");
 const managerBinary=nativeDistributionV1(protocol.manager as "uv",host).binaries.find(name=>/^uv(?:\.exe)?$/u.test(name));
 if(!managerBinary)throw Error("Pinned uv distribution requires its uv binary");
 const manager=join(preparedNativeToolV1(protocol.manager as "uv",host,toolsRoot,environment),managerBinary);
 const plan=pythonEnvironmentOfProjectV1(checkout,protocol.project,{dependencies:repositoryCachePartitionRootV1(checkout,"dependencies"),tools:toolsRoot}),result=await runToolV1(protocol.manager,workspaceTestPythonArgumentsV1(plan.series),{root:checkout,cwd:checkout,env:workspaceTestPythonVariablesV1(environment,plan.variables,managerLocator.name,manager),signal,capture:true,check:false,maxBuffer:protocol.maximumOutputBytes,heavy:false,caller:import.meta.url});
 requireWorkspaceTestPythonSuccessV1(result,signal);
 const executable=readWorkspaceTestPythonOutputV1(result.stdout,plan.interpreters),locator=findToolV1(protocol.tool)?.locate.find(row=>row.kind==="environment");
 if(!locator||locator.kind!=="environment")throw Error("Registered Python environment locator required");
 return{...environment,[locator.name]:executable};
}
