#!/usr/bin/env repo
import {resolve} from 'node:path';
import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import {receiveScriptProcessInvocation} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import {runOwnedCommand} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts';
import {configuredExactCargoLawPolicyV1} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
import {runAdmittedExactCargoLawsV1} from '../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts';
import {scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1} from '@semio-tech/process/compiler-capacity';
import { resolveTypescriptCompilerV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { isMainModuleV1 } from "../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
import { runtimeTypeEnvironmentV1 } from "../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class OriginalHistoryReadScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-history-read requires source or native');
  const source=resolve(this.root,'🧪️tests/🟦️.ts');
  await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types',runtimeTypeEnvironmentV1(),source],this.repoRoot,'history:original-read:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalHistoryRead();oracle.verifyOriginalProjectedSource();oracle.verifyCapturedSnapshotSourceOracle();
  if(args[0]==='source')return;
  const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml'),env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-os-kernel',target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_vcs::history_read::tests::original_history_reads_keep_forward_backing_and_refuse_each_denied_currency','os_vcs::history_read::tests::original_history_reads_return_every_cancelled_birth_and_reject_foreign_generations','os_vcs::history_read::tests::original_history_reads_refuse_unpublished_groups_and_foreign_original_issuers','os_vcs::history_read::tests::original_history_store_retirement_returns_read_sources_and_waits_for_live_leases','os_vcs::history_read::tests::original_projected_source_requires_retirement_only_from_its_actual_authority','os_store::component::captured_snapshot_source::tests::captured_snapshot_source_uses_original_registry_and_installed_opaque_issuer']}]});
 }
}
const router=new ScriptRouter(import.meta.dirname).register('verify',OriginalHistoryReadScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
