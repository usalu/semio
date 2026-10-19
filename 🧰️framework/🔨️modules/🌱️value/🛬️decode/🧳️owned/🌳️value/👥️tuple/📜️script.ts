#!/usr/bin/env bun
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../🏃️process/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
import {runOwnedCommand} from '../../../../../🏃️process/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1} from '../../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {runAdmittedExactCargoLawsV1} from '../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts';
/** 🧳️ Verifies original heterogeneous values, fixed native frontiers and physical partial-child cancellation. */
class OriginalTupleReceivingScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-tuple-receiving requires source or native');
  const source=resolve(this.root,'🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-tuple:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalTupleOracle();if(args[0]==='source')return;
  const packageName='semio-framework-value',manifest=resolve(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml');
  const receipts=await runAdmittedExactCargoLawsV1({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_value'},laws:['value::native_decoding::owned::tree::tuple::tests::original_heterogeneous_tuple_receiving_preserves_every_paid_prefix_and_wire_kind']}]});
  console.log('[DEBUG] Original tuple receiving native '+receipts[0]!.assertions+' exact custody law');
 }
}
const router=new ScriptRouter(import.meta.dir).register('verify',OriginalTupleReceivingScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
