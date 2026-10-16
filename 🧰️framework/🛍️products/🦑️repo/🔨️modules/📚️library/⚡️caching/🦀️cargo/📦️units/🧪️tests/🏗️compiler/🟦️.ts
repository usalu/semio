import{withCompilerWorkspaceWriteV1}from"../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import type{RepositoryCompilerStoreV1}from"../../../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import type{ScriptInvocation}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type{NativeStoragePolicyV1,NativeStoragePortV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type{RustUnitPlanV1}from"../../🟦️.ts";
import{compileRustUnitV1}from"../../🏗️compiler/🟦️.ts";
import{rustLibraryCompilerPortV1,type RustUnitDependencyArtifactV1}from"../../🏗️compiler/🦀️rustc/🟦️.ts";
import{withRustCompilerStateV1}from"../../🏗️compiler/💾️state/🟦️.ts";
/** 🧪️ Keeps real prototype state loans inside the same actual compilation assertions; release retains its genuine temporary backing. */
export function withRustTestCompilerV1<T>(options:Readonly<{root:string;workspace:string;invocation:ScriptInvocation;store:RepositoryCompilerStoreV1;control:ResourceLeaseControl;plan:RustUnitPlanV1;dependencies:ReadonlyMap<string,RustUnitDependencyArtifactV1>;environment:Readonly<Record<string,string|undefined>>;policy:NativeStoragePolicyV1;storage:NativeStoragePortV1}>,body:(options:Parameters<typeof compileRustUnitV1>[0])=>Promise<T>):Promise<T>{const{root,workspace,invocation,store,control,plan,dependencies,environment}=options;return withCompilerWorkspaceWriteV1(store.storage,options.policy.leaseDirectory,control,()=>plan.unit.mode==="prototype"?withRustCompilerStateV1({workspace,invocation,store,control,plan},(loan,state,actual)=>body({root,plan,control:actual,policy:state.policy,compiler:rustLibraryCompilerPortV1(root,invocation,dependencies,environment,loan),fundedWorkspace:{storage:state.storage,directory:state.workspace.directory,temporary:state.workspace.temporary}})):body({root,plan,control,policy:options.policy,storage:options.storage,compiler:rustLibraryCompilerPortV1(root,invocation,dependencies,environment,null)}));}
