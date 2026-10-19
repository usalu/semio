#!/usr/bin/env bun
/** 📦️ Original JSON artifact owning package commands. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {resolve} from "node:path";
import {BundleScript,scriptInvocationBudget} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runBudgetedTestCommand} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
/** 🧾️ Genuine recursive original input and independent relational laws. */
class OriginalReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("original JSON receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");
  const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);
 }
}
/** 🦀️ Original mounted JSON receiving laws under the mandatory caller process. */
class OriginalReceivingNative extends OriginalReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-json",this.repoRoot,["--lib","json_original_receiving_","--no-fail-fast"],this.invocation);}
}
/** 🧬️ Current owning original schema proof without source copies or fresh authority. */
class NativeSchemaScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw Error("native JSON schema accepts no arguments");await runArtifactRustTests("semio-s-artifact-stdio-json",this.repoRoot,["--lib","sqlite_snapshot_json_owned_record_identity_matches_handwritten_definition","--no-fail-fast"],this.invocation);}
}
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-stdio-json",{ packages: ["semio-s-artifact-stdio-json", "semio-s-artifact-stdio-json-mutations", "semio-s-artifact-stdio-json-mutations-2", "semio-s-artifact-stdio-json-number", "semio-s-artifact-stdio-json-schema", "semio-s-artifact-stdio-json-schema-2", "semio-s-artifact-stdio-json-tests"],commands:{"native-schema-check":NativeSchemaScript,"test-original-receiving-source":OriginalReceivingSource,"test-original-receiving-native":OriginalReceivingNative},snapshotSqliteTests:["../../🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts", "../../🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧪️tests/🪶️sqlite/🟦️.ts","../../🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
