#!/usr/bin/env bun
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
import {runOwnedCommand} from '../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from '../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {readCargoTestPolicyV1} from '../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts';
/** 🏭️ Validates one quoted concrete semantic source and both persistent native ports. */
export class SharedSemanticSourceScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('shared-semantic-source requires source or native');
  const source=resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'store:semantic-source:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,['test',source],this.repoRoot,'store:semantic-source:oracle',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  if(args[0]==='source')return;
  const packageName='semio-framework-os-kernel',manifest=resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml'),policy=readCargoTestPolicyV1(process.env);
  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:this.invocation.control.remainingMilliseconds()},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,manifestPaths:{[packageName]:manifest},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_store::component::tests::shared_semantic_source_tests::original_shared_semantic_source_constructs_one_paired_issuer_and_closes_all_actual_frontiers']}]});
  console.log('[DEBUG] Shared semantic source exact native assertions='+receipts[0]!.assertions);
 }
}
const router=new ScriptRouter(import.meta.dir).register('verify',SharedSemanticSourceScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
