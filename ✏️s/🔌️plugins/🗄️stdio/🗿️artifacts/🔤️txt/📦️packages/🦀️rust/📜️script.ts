#!/usr/bin/env bun
/** 📦️ txt Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runArtifactRustTests} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runRepositoryTestCommand} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";

/** 🛫️ Runs original TXT output custody laws under the unchanged supplied invocation. */
class OriginalTxtEncodingScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!['source','native'].includes(args[0]))throw Error('test-original-encoding requires source or native');
  if(args[0]==='source')await runRepositoryTestCommand(process.execPath,['test',resolve(this.root,'../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts')],{cwd:this.repoRoot,budgetMs:this.invocation.control.remainingMilliseconds()??undefined,signal:this.invocation.control.signal});
  else await runArtifactRustTests('semio-s-artifact-stdio-txt',this.repoRoot,['--lib','txt_original_encoding_','--no-fail-fast','--success-output','immediate'],this.invocation);
 }
}
/** 🫴️ Runs TXT semantic receiving under the original finite invocation. */
class OriginalTxtReceivingScript extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["source","native"].includes(args[0]))throw Error("test-original-receiving requires source or native");
  if(args[0]==="source")await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:this.invocation.control.remainingMilliseconds()??undefined,signal:this.invocation.control.signal});
  else await runArtifactRustTests("semio-s-artifact-stdio-txt",this.repoRoot,["--lib","txt_original_receiving_","--no-fail-fast","--success-output","immediate"],this.invocation);
 }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-txt", { packages: ["semio-s-artifact-stdio-txt", "semio-s-artifact-stdio-txt-examples", "semio-s-artifact-stdio-txt-inferences", "semio-s-artifact-stdio-txt-mutations", "semio-s-artifact-stdio-txt-schema", "semio-s-artifact-stdio-txt-tests"], commands:{"test-original-encoding":OriginalTxtEncodingScript,"test-original-receiving":OriginalTxtReceivingScript}, snapshotSqliteTests: ["../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts", "../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🗂️typed-file/🟦️.ts"] });
