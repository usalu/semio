import type { ScriptInvocation } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolve } from "node:path";
import { scriptInvocationBudget } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTypescriptCompilerV1 } from "../../../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🏃️ Runs the `OriginalTupleReceivingScript` command under the caller's roots and invocation. */
export async function runOriginalTupleReceivingScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
if(args.length!==1||!['source','native'].includes(args[0]!))throw new Error('original-tuple-receiving requires source or native');
const source=resolve(context.root,'🧪️tests/🟦️.ts');
await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(context.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],context.repoRoot,'value:original-tuple:types',scriptInvocationBudget(context.invocation,120000),{signal:context.invocation.control.signal});
const oracle=await import("../🟦️.ts");oracle.verifyOriginalTupleOracle();if(args[0]==='source')return;
const packageName='semio-framework-value',manifest=resolve(context.repoRoot,'🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml');
const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:context.repoRoot,manifestPath:manifest,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_value'},laws:['value::native_decoding::owned::tree::tuple::tests::original_heterogeneous_tuple_receiving_preserves_every_paid_prefix_and_wire_kind']}]});
 }
