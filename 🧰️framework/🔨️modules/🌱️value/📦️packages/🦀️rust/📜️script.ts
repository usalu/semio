#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🌱️ Tests the actual neutral value package under its explicitly supplied native policy. */
/** 🧱️ Checks declared refusal authority without replacing the original process deadline. */
class NativeLiteralRefusalSourceScript extends BundleScript{
 private childBudget():number{const remaining=this.invocation.control.remainingMilliseconds();if(remaining!==null&&remaining<1)throw Error("Original native refusal source deadline exhausted");return remaining===null?0:Math.floor(remaining);}
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-native-literal-refusal-source");const test=resolve(this.root,"../../🛬️decode/⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",test],this.repoRoot,"value:original-refusal:types",this.childBudget(),{signal:this.invocation.control.signal});
  await runOwnedCommand(process.execPath,["test",test],this.repoRoot,"value:original-refusal:source",this.childBudget(),{signal:this.invocation.control.signal});
 }
}
class NativeRecipientSourceTestScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-native-recipient-source");const test=resolve(this.root,"../../🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",test], this.repoRoot, "value:recipient:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });await runOwnedCommand(process.execPath, ["test",test], this.repoRoot, "value:recipient:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });}
}
/** 🔐️ Executes original detached recipient identity and real retained physical custody laws. */
class NativeRecipientTestScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-native-recipient");await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","native_local_original","--","--nocapture"]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env));}
}
/** 🧮️ Exercises the existing complete neutral value package route. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "portable") {
      if (args.length !== 1) throw Error("Expected test portable");
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")], this.repoRoot, "value:portable:construction", Math.min(15_000, this.invocation.control.remainingMilliseconds()), { signal: this.invocation.control.signal });
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")], this.repoRoot, "value:portable:decoding", Math.min(30_000, this.invocation.control.remainingMilliseconds()), { signal: this.invocation.control.signal });
      return;
    }
    const { rest } = resolveTestLevel(args);
    await runCargoTestsV1({ ...({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value"], cwd: this.root, extraArgs: rest }), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env));
  }
}
/** 🛬️ Exercises canonical borrowed construction, allocation admission and owner retirement. */
/** 🧮️ Executes closed Int64 scalar transport and actual test-only GraphQL mapping. */
class GraphqlInt64SourceTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-graphql-int64-source");
  const tests=resolve(this.root,"../../🧬️schema/🔗️graphql/🔢️int64/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",tests], this.repoRoot, "value:graphql:int64:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",tests], this.repoRoot, "value:graphql:int64:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** 🗂️ Checks full retirement categories and the independent ordered payload oracle. */
class OrderedRetirementSourceTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-ordered-retirement-source");
  const tests=resolve(this.root,"../../🗂️ordered/♻️retirement/🧪️tests/🟦️.ts");
  const paged=resolve(this.root,"../../📋️list/🧪️tests/📋️list/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",tests,paged], this.repoRoot, "value:ordered:retirement:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test",tests,paged], this.repoRoot, "value:ordered:retirement:source", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
class ControlledValueTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    const {rest}=resolveTestLevel(args);
    await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_",...rest]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env));
  }
}
/** 🛫️ Exercises explicit borrowed output construction and cumulative encoding admission. */
class ControlledEncodingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{const{rest}=resolveTestLevel(args);await runCargoTestsV1({ ...({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_encoding_",...rest]}), signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds() }, readCargoTestPolicyV1(process.env));}
}
/** 🏷️ Validates the canonical type corpus and direct lower ownership. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-type-ownership");
  const absence=resolve(this.root,"../../🏷️type/🧪️tests/🚮️absence/🟦️.ts");
  const source=resolve(this.root,"../../🏷️type/🟦️.ts"),tests=resolve(this.root,"../../🏷️type/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,tests,absence], this.repoRoot, "value:type:types", Math.min(15000, this.invocation.control.remainingMilliseconds()), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test",tests], this.repoRoot, "value:type:ownership", Math.min(15000, this.invocation.control.remainingMilliseconds()), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",absence], this.repoRoot, "value:type:absence", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** ⚠️ Checks typed refusal schema, independent paths and actual portable decoder errors. */
class RefusalPortableTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-refusal-portable");
  const test = resolve(this.root, "../../⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], this.repoRoot, "value:refusal:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", test], this.repoRoot, "value:refusal:portable", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** ⚠️ Proves controlled refusal transport against the closed schema and independent wire owners. */
class RefusalCodecTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-refusal-codec");
  const test=resolve(this.root,"../../⚠️refusal/🔁️codec/🧪️tests/🟦️.ts");
  const source=resolve(this.root,"../../⚠️refusal/🔁️codec/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,test], this.repoRoot, "value:refusal:codec:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test","--timeout","30000",test], this.repoRoot, "value:refusal:codec:portable", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}
/** 🛬️ Checks neutral decode discovery and the original schema and SQLite reference corpus. */
class DecodeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-decode-ownership");
  const tests = [resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🚮️absence/🟦️.ts")];
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], this.repoRoot, "value:decode:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🚮️absence/🟦️.ts")], this.repoRoot, "value:decode:ownership", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
 }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-native-literal-refusal-source",NativeLiteralRefusalSourceScript).register("test-native-recipient-source",NativeRecipientSourceTestScript).register("test-native-recipient",NativeRecipientTestScript).register("test-decode-ownership", DecodeOwnershipTestScript).register("test-graphql-int64-source",GraphqlInt64SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript).register("test-refusal-portable", RefusalPortableTestScript).register("test-refusal-codec",RefusalCodecTestScript).register("test-ordered-retirement-source",OrderedRetirementSourceTestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
