#!/usr/bin/env bun
import {runRepositoryExactCargoLaws as runExactCargoLaws,configuredRepositoryCargoPreparationPolicyV1} from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { advanceScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { admitJsonReadSourceOwner } from "../../📥️decode/🧪️testing/📥️invocation/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {configuredExactCargoLawPolicyV1} from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { scriptInvocationBudget } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧩️ Runs strict schema, canonical ownership and mandatory JSON source-policy laws. */
class ReadPolicyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-read-policy-source accepts no arguments");
    const owner=admitJsonReadSourceOwner(this.invocation, resolve(this.root, "../../..")),original=owner.invocation,budget=()=>owner.childBudgetMilliseconds();
    const tests = [resolve(this.root, "../../📥️decode/🛂️policy/🧪️tests/🟦️.ts"), resolve(this.root, "../../📥️decode/🧾️receipt/🧪️tests/🟦️.ts"), resolve(this.root, "../../📥️decode/🧵️operation/🧪️tests/🟦️.ts"), resolve(this.root, "../../📥️decode/🛂️policy/📏️depth/🧪️tests/🟦️.ts"), resolve(this.root, "../../📥️decode/🧪️testing/📥️invocation/🧪️tests/🟦️.ts")];
    await advanceScriptInvocation(original, "test-read-policy-source.typecheck", "running");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: this.repoRoot, budgetMs: budget(), signal:original.control.signal, throwOnFailure: true});
    await advanceScriptInvocation(original, "test-read-policy-source.typecheck", "complete");
    await advanceScriptInvocation(original, "test-read-policy-source.tests", "running");
    await runBudgetedTestCommand(process.execPath, ["test", ...tests], {cwd: this.repoRoot, budgetMs: budget(), signal:original.control.signal, throwOnFailure: true});
    await advanceScriptInvocation(original, "test-read-policy-source.tests", "complete");
  }
}

/** 📦️ Validates the neutral JSON ownership and source contract. */
class ReferenceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-reference accepts no arguments");
    const test = resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🚮️ Runs the complete reference corpus and the physical specialization absence law. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    const tests = [resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")];
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", "--timeout", "30000", ...tests], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: {...process.env, SEMIO_TEST_ARTIFACT_DIR: readCargoTestPolicyV1(process.env).artifactDirectory}, throwOnFailure: true});
  }
}

/** 🧾️ Executes every original JSON primitive law and additive explicit-member law. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments[0]==="tree-wire"){
      await runExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),manifestPaths:{"semio-framework-pack-json":resolve(this.root,"Cargo.toml")},env:{...process.env,CARGO_BUILD_BUILD_DIR:readCargoTestPolicyV1(process.env).buildDirectory},invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,buildBudgetMs()),lawMilliseconds:600000},cwd:this.repoRoot,cargoArgs:segments.slice(1),groups:[{package:"semio-framework-pack-json",target:{kind:"lib"},laws:["original_tree_wire_source_same_pointer_two_pass_system_denial_and_full_close","original_tree_wire_cancel_retains_every_partial_typed_source_and_destination"]}]});return;
    }
    if(segments[0]==="tree-header"){
      await runExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),manifestPaths:{"semio-framework-pack-json":resolve(this.root,"Cargo.toml")},env:{...process.env,CARGO_BUILD_BUILD_DIR:readCargoTestPolicyV1(process.env).buildDirectory},invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,buildBudgetMs()),lawMilliseconds:600000},cwd:this.repoRoot,cargoArgs:segments.slice(1),groups:[{package:"semio-framework-pack-json",target:{kind:"lib"},laws:["original_canonical_tree_constructor_moves_header_without_payload_copy","original_canonical_tree_nested_headers_preserve_body_minima_and_complete_custody"]}]});return;
    }
    const {rest} = resolveTestLevel(segments);
    await runCargoTestsV1({manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-pack-json"], cwd: this.root, extraArgs: rest, signal: this.invocation.control.signal, remainingMilliseconds: () => this.invocation.control.remainingMilliseconds()}, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation);
  }
}

/** 📨️ Differential neutral laws for the original typed Tree wire source. */
class TreeWireSourceScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("test-tree-wire-source accepts no arguments");const file=resolve(this.root,"../../🛫️encode/🧭️tree/📨️wire/🧪️tests/🟦️.ts");for(const arguments_ of [[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",file],["test",file]])await runBudgetedTestCommand(process.execPath,arguments_,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true})}}
const router = new ScriptRouter(import.meta.dir).register("test-tree-wire-source",TreeWireSourceScript).register("test-read-policy-source", ReadPolicyScript).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-ownership" }));
