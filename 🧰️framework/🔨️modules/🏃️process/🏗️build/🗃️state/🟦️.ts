import { withCompilerCapacityScopeV1, type CompilerAdmissionControl } from "../🎟️capacity/🟦️.ts";
import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1}from"../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{readNativeStorageReservationPolicyV1,type NativeStorageReservationV1}from"../../📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{requireNativeStoragePathV1}from"../../📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import { join, resolve, relative, isAbsolute } from "node:path";
import { readBuildMode, type BuildMode } from "../🎚️mode/🟦️.ts";
import { acquireResourceLease, checkResourceLeaseControl, resourceLeasePathV1, requireMaterializedPathV1 } from "../../🔒️leases/🟦️.ts";

/** 🗃️ Owns one compiler's mutable intermediates independently of its staged deliverable. */
export type CompilerStateOwner = Readonly<{ root: string; owner: string; compiler: string; configuration: string; mode: BuildMode; platform?: string; architecture?: string }>;

/** 🔑️ Partitions state by compiler, configuration, platform, architecture, owner and intent. */
export function compilerStateDirectory(options: CompilerStateOwner): string {
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => !["control", "storage", "storageControl", "environment", "capacity", "requestedParallelism", "files"].includes(key) && value !== undefined)));
  if (failures.length) throw Error("Invalid compiler state owner: " + failures.join("; "));
  const mode = readBuildMode(options.mode), platform = options.platform ?? process.platform, architecture = options.architecture ?? process.arch;
  const identity = createHash("sha256").update(JSON.stringify([options.owner, options.compiler, options.configuration, platform, architecture])).digest("hex").slice(0, 24);
  const directory = join(resolve(options.root), "compiler", `${platform}-${architecture}`, mode, identity);
  return requireMaterializedPathV1(directory);
}

/** 🧮️ Bounds mutable compiler lock metadata independently of input and toolchain history. */
export function compilerStateResourceV1(directory:string):string{return "compiler-state:v1:"+(parseInt(createHash("sha256").update(requireMaterializedPathV1(resolve(directory))).digest("hex").slice(0,4),16)%schema.definitions.Protocol.const.writerShards);}

/** 🔒️ Serializes access to owned mutable compiler state while retaining warm intermediates. */
export async function withCompilerState<T>(options: CompilerStateOwner & { readonly control: CompilerAdmissionControl; readonly storage:NativeStorageReservationV1; readonly storageControl:CompilerAdmissionControl; readonly files: readonly string[]; readonly environment?: Readonly<Record<string, string | undefined>>; readonly capacity?: boolean; readonly requestedParallelism?: 1 }, operation: (directory: string, environment: Readonly<Record<string, string | undefined>>, control:CompilerAdmissionControl) => Promise<T>): Promise<T> {
  const initialControl=options.control;requireIssuedResourceControlV1(initialControl);
  const storageControl=options.storageControl;requireIssuedResourceControlV1(storageControl);requireIssuedResourceControlRefinementV1(storageControl,initialControl);
  const storage=options.storage,policy=readNativeStorageReservationPolicyV1(storage,storageControl);
  if(typeof operation!=="function")throw Error("Actual compiler state operation required");
  const files=[...options.files];if(validateJsonSchemaSubset(schema.definitions.Materialization,{files}).length)throw Error("Complete compiler materialized paths required");
  const directory=compilerStateDirectory(options),environment=Object.freeze({...options.environment??process.env}),leaseDirectory=policy.leaseDirectory,requestedParallelism=options.requestedParallelism,capacity=options.capacity!==false;
  requireNativeStoragePathV1(policy.root,resolve(options.root));requireNativeStoragePathV1(policy.root,directory);
  for (const file of files) {
    const path = requireMaterializedPathV1(join(directory, file)), child = relative(directory, path);
    if (isAbsolute(file) || child === ".." || child.startsWith("../") || child.startsWith("..\\") || file.includes("\0")) throw Error("Compiler materialized path must remain within its state owner");
    requireNativeStoragePathV1(policy.root,path);
  }
  const resource=compilerStateResourceV1(directory);resourceLeasePathV1(leaseDirectory,resource);
  let lease:Awaited<ReturnType<typeof acquireResourceLease>>|undefined;
  const run=async(environment:Readonly<Record<string,string|undefined>>,control:CompilerAdmissionControl):Promise<T>=>{
    lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",signal:control.signal,control});
    checkResourceLeaseControl(control);await storage.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(policy.root,directory);await mkdir(directory,{recursive:true});checkResourceLeaseControl(control);requireNativeStoragePathV1(policy.root,directory);
    return operation(directory,environment,control);
  };
  const accepted=capacity?withCompilerCapacityScopeV1({directory:leaseDirectory,control:initialControl,environment,...(requestedParallelism===undefined?{}:{requestedParallelism})},(facts,control)=>run(Object.freeze({...environment,SEMIO_COMPILER_PARALLELISM:String(facts.parallelism),GOMAXPROCS:String(facts.parallelism)}),control)):withRequestedIssuedResourceControlV1(initialControl,0,control=>run(environment,control));
  return withIssuedResourceOperationV1(initialControl,async()=>{let value:T|undefined,failed=false,failure:unknown;try{value=await accepted;await storage.recheck();checkResourceLeaseControl(initialControl);}catch(error){failed=true;failure=error;}try{lease?.release();}catch(error){if(failed&&!Object.is(failure,error))throw new AggregateError([failure,error],"Compiler state work and writer release failed");throw error;}if(failed)throw failure;return value as T;});
}
