import {admitRepositoryCompilerEnvironmentV1} from "./🌍️environment/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};
/** 🔐️ Enables the selected compiler only after rechecking an actually issued live enforcing reservation. */
export async function repositoryCompilerLaunchEnvironmentV1(storage:NativeStorageReservationV1,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Promise<Readonly<Record<string,string|undefined>>>{requireNativeStorageReservationV1(storage,control);if(validateJsonSchemaSubset(schema,policy).length||storage.allocation.kind!=="compiler-workspace")throw Error("Original compiler ingress policy and storage required");await storage.recheck();admitRepositoryCompilerEnvironmentV1(environment);return Object.freeze({...environment,RUSTC:environment.RUSTC??policy.compiler,RUSTC_WRAPPER:environment.RUSTC_WRAPPER??""});}
