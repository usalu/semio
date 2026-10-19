#!/usr/bin/env bun
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../🏃️process/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
import {runOwnedCommand} from '../../../🏃️process/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1} from '../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {runAdmittedExactCargoLawsV1} from '../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts';
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class OriginalSlotScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-slot requires source or native');
  const source=resolve(this.root,'🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-slot:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalSlot();
  if(args[0]==='source')return;
  const manifest=resolve(this.repoRoot,"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml");const receipts=await runAdmittedExactCargoLawsV1({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-value',target:{kind:'lib',name:'semio_framework_value'},laws:['retained_clone::original_slot::tests::original_slot_preserves_native_payload_and_waits_for_every_projected_reader','retained_clone::original_slot::tests::original_slot_generation_exhaustion_refuses_before_native_exchange','retained_clone::original_slot::tests::original_slot_exchange_matches_all_independent_portable_outputs','retained_clone::original_slot::tests::original_slot_retirement_returns_all_readers_then_uses_same_captured_payload_issuer']}]});
  console.log('[DEBUG] Original slot native custody '+receipts[0]!.assertions+' exact laws');
 }
}
const router=new ScriptRouter(import.meta.dir).register('verify',OriginalSlotScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
