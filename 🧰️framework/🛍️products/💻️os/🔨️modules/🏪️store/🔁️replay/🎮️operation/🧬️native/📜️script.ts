#!/usr/bin/env bun
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
import {runOwnedCommand} from '../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from '../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {readCargoTestPolicyV1} from '../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts';
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class NativeSemanticPreparationScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('native-semantic-preparation requires source or native');
  const source=resolve(this.root,'🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'store:native-semantic:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyNativeSemanticPreparation();
  if(args[0]==='source')return;
  const packageName="semio-framework-os-kernel",manifest=resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml"),p=readCargoTestPolicyV1(process.env);const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,cargoTargetDir:p.targetDirectory,manifestPaths:{[packageName]:manifest},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-os-kernel',target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_store::component::snapshot_read_registry_handle::birth::tests::original_registry_birth_keeps_four_kib_quantum_fifo_and_exact_cumulative_custody','os_store::component::replay_preparation::native::tests::original_semantic_factory_matches_neutral_outputs_and_exact_physical_turns','os_store::component::derived_operation::tests::original_derived_semantic_task_admits_copy_and_capacity_as_independent_currencies']}]});
  console.log('[DEBUG] Original semantic native custody '+receipts[0]!.assertions+' exact laws');
 }
}
const router=new ScriptRouter(import.meta.dir).register('verify',NativeSemanticPreparationScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
