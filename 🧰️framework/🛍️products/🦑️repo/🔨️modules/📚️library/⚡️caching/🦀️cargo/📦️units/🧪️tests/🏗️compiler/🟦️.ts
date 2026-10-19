import{withCompilerWorkspaceWriteV1}from"../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import type{RepositoryCompilerStoreV1}from"../../../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import type{ScriptInvocation}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import type{NativeStoragePortV1}from"@semio-tech/process/artifacts/native-build/storage";
import type{NativeStoragePolicyV1}from"@semio-tech/process/artifacts/native-build/storage";
import { type RustUnitPlanV1 } from "../../📋️plan/🟦️.ts";
import{compileRustUnitV1}from"../../🏗️compiler/🟦️.ts";
import{rustLibraryCompilerPortV1,type RustUnitDependencyArtifactV1}from"../../🏗️compiler/🦀️rustc/🟦️.ts";
import{withRustCompilerStateV1}from"../../🏗️compiler/💾️state/🟦️.ts";
/** 🧪️ Keeps real prototype state loans inside the same actual compilation assertions; release retains its genuine temporary backing. */
export function withRustTestCompilerV1<T>(options:Readonly<{root:string;workspace:string;invocation:ScriptInvocation;store:RepositoryCompilerStoreV1;control:ResourceLeaseControl;plan:RustUnitPlanV1;dependencies:ReadonlyMap<string,RustUnitDependencyArtifactV1>;environment:Readonly<Record<string,string|undefined>>;policy:NativeStoragePolicyV1;storage:NativeStoragePortV1}>,body:(options:Parameters<typeof compileRustUnitV1>[0])=>Promise<T>):Promise<T>{const{root,workspace,invocation,store,control,plan,dependencies,environment}=options;if(plan.unit.mode==="prototype")return withRustCompilerStateV1({workspace,invocation,store,control,plan},(loan,state,actual)=>body({root,plan,control:actual,policy:state.policy,compiler:rustLibraryCompilerPortV1(root,invocation,dependencies,environment,loan),fundedWorkspace:{storage:state.storage,directory:state.workspace.directory,temporary:state.workspace.temporary,funding:{kind:"compiler-state",loan}}}));return withCompilerWorkspaceWriteV1(store.storage,options.policy.leaseDirectory,control,actual=>body({root,plan,control:actual,policy:options.policy,storage:options.storage,compiler:rustLibraryCompilerPortV1(root,invocation,dependencies,environment,null)}));}
