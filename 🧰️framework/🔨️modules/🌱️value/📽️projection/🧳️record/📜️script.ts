#!/usr/bin/env bun
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../🏃️process/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
import {runOwnedCommand} from '../../../🏃️process/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1} from '../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {runAdmittedExactCargoLawsV1} from '../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts';
/** 🧳️ Verifies original wide record output, fixed native frontiers and physical partial-field cancellation. */
class OriginalRecordProjectionScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-record-projection requires source or native');
  const source=resolve(this.root,'🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-record:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalRecordProjectionOracle();if(args[0]==='source')return;
  const packageName='semio-framework-value',manifest=resolve(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml');
  const receipts=await runAdmittedExactCargoLawsV1({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_value'},laws:['value::snapshot_projection::tests::record::original_record_projection_stages_one_paid_field_and_preserves_all_cancellation_prefixes']}]});
  console.log('[DEBUG] Original record projection native '+receipts[0]!.assertions+' exact custody law');
 }
}
const router=new ScriptRouter(import.meta.dir).register('verify',OriginalRecordProjectionScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
