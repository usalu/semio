#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🌱️ Tests the actual neutral value package under its explicitly supplied native policy. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "portable") {
      if (args.length !== 1) throw Error("Expected test portable");
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")], this.repoRoot, "value:portable:construction", 15_000);
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")], this.repoRoot, "value:portable:decoding", 30_000);
      return;
    }
    const { rest } = resolveTestLevel(args);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}
/** 🛬️ Exercises canonical borrowed construction, allocation admission and owner retirement. */
/** 🧮️ Executes closed Int64 scalar transport and actual test-only GraphQL mapping. */
class GraphqlInt64SourceTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-graphql-int64-source");
  const tests=resolve(this.root,"../../🧬️schema/🔗️graphql/🔢️int64/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,["test","--timeout","30000",tests],this.repoRoot,"value:graphql:int64:source",30000);
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",tests],this.repoRoot,"value:graphql:int64:types",30000);
 }
}
class ControlledValueTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    const {rest}=resolveTestLevel(args);
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_",...rest]},readCargoTestPolicyV1(process.env));
  }
}
/** 🛫️ Exercises explicit borrowed output construction and cumulative encoding admission. */
class ControlledEncodingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{const{rest}=resolveTestLevel(args);await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_encoding_",...rest]},readCargoTestPolicyV1(process.env));}
}
/** 🏷️ Validates the canonical type corpus and direct lower ownership. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-type-ownership");
  const absence=resolve(this.root,"../../🏷️type/🧪️tests/🚮️absence/🟦️.ts");
  const source=resolve(this.root,"../../🏷️type/🟦️.ts"),tests=resolve(this.root,"../../🏷️type/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,tests,absence],this.repoRoot,"value:type:types",15000);
  await runOwnedCommand(process.execPath,["test",tests],this.repoRoot,"value:type:ownership",15000);
  await runOwnedCommand(process.execPath,["test","--timeout","30000",absence],this.repoRoot,"value:type:absence",30000);
 }
}
/** ⚠️ Checks typed refusal schema, independent paths and actual portable decoder errors. */
class RefusalPortableTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-refusal-portable");
  const test = resolve(this.root, "../../⚠️refusal/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], this.repoRoot, "value:refusal:types", 30000);
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", test], this.repoRoot, "value:refusal:portable", 30000);
 }
}
/** ⚠️ Proves controlled refusal transport against the closed schema and independent wire owners. */
class RefusalCodecTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-refusal-codec");
  const test=resolve(this.root,"../../⚠️refusal/🔁️codec/🧪️tests/🟦️.ts");
  const source=resolve(this.root,"../../⚠️refusal/🔁️codec/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,test],this.repoRoot,"value:refusal:codec:types",30000);
  await runOwnedCommand(process.execPath,["test","--timeout","30000",test],this.repoRoot,"value:refusal:codec:portable",30000);
 }
}
/** 🛬️ Checks neutral decode discovery and the original schema and SQLite reference corpus. */
class DecodeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-decode-ownership");
  const tests = [resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🚮️absence/🟦️.ts")];
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], this.repoRoot, "value:decode:types", 30000);
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", resolve(this.root, "../../🛬️decode/🧪️tests/🧩️ownership/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts"), resolve(this.root, "../../🛬️decode/🧪️tests/🚮️absence/🟦️.ts")], this.repoRoot, "value:decode:ownership", 30000);
 }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-decode-ownership", DecodeOwnershipTestScript).register("test-graphql-int64-source",GraphqlInt64SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript).register("test-refusal-portable", RefusalPortableTestScript).register("test-refusal-codec",RefusalCodecTestScript), { defaultCommand: "test" });
