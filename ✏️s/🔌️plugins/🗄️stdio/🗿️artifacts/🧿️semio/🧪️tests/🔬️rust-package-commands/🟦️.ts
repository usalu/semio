import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { BundleScript } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { runBudgetedTestCommand } from "@semio-tech/framework/process/test-execution";
import { resolveTypescriptCompilerV1 } from "@semio-tech/framework/process/execution";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");


/** 🔤️ Qualifies complete Text field witnesses under the original caller invocation. */
export class TextReceivingSource extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("original Text receiving accepts no arguments");const source=resolve(this.root,"../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🟦️.ts");const execute=(args:string[])=>runBudgetedTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});await execute([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source]);await execute(["test",source]);}
}
/** 🦀️ Executes genuine original complete Text receiving in its actual owning package. */
export class TextReceivingNative extends TextReceivingSource{
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-stdio-semio-text",this.repoRoot,["--lib","semio_text_original_receiving_","--no-fail-fast"],this.invocation);}
}

/** 🧪️ Executes the contracts owned by this component. */
export class OwnedVerifyScript extends BundleScript {
async run(segments: string[]): Promise<void> {
if(segments[0]==="drawing-original"&&segments[1]==="source"){
const path=resolve(this.root,"../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🧩️component/🌳️nodes/🧾️original/🧪️tests/🟦️.ts");
runCmd(process.execPath,["test",path],{cwd:this.repoRoot});
runCmd(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck",path],{cwd:this.repoRoot});return;
}if(segments[0]==="value-path"&&segments[1]==="source"){
const path=resolve(this.root,"../../🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/💾️binary/🧬️mutations/🧭️path");
runCmd(process.execPath,["test",join(path,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});
runCmd(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck",join(path,"🟦️.ts"),join(path,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});return;
}if(["flow-original","native-prefix"].includes(segments[0]!)&&segments[1]==="source"){
const path=resolve(this.root,"../../🏅️standards/🔖️v1/🪆️subsets/"+(segments[0]==="native-prefix"?"✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🫙️prefix/🟦️.ts":"🌊️flow/🚪️io/💾️binary/📸️snapshot/🧪️tests/🧾️original/🟦️.ts"));runCmd(process.execPath,["test",path],{cwd:this.repoRoot});
runCmd(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck",path],{cwd:this.repoRoot});return;
}if (segments[0] === "stdio-document-contract") {
const { testSemioObjectDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
testSemioObjectDocumentContract();
const { testSemioKitDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
testSemioKitDocumentContract();
const { testSemioGeometryContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🧪️tests/🔬️unit/🟦️.ts");
testSemioGeometryContract();
const stdioSchemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${stdioSchemaRoot}/✉️base/🧬️schema/🧮️geometry/🟦️.ts`, ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts"].map((file) => `${stdioSchemaRoot}/📦️object/🧬️schema/${file}`), ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${stdioSchemaRoot}/🧰️kit/🧬️schema/${file}`)], { cwd: this.repoRoot });
if (segments[1] === "native") {
const { runCargo } = await import("@semio-tech/framework/process/execution");
for (const [crate, filter] of [["semio-s-artifact-stdio-semio-base", "stdio_document_contract"], ["semio-s-artifact-stdio-semio-object", ""], ["semio-s-artifact-stdio-semio-kit", ""]] as const) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", crate, "--lib", ...(filter ? [filter] : []), "--", "--nocapture"], this.repoRoot);
}
return;
}
throw new Error('Unknown owned verification '+segments.join(' '));
}
}

/** 📦️ Every Cargo package the semio artifact family is cut into: the members of its own workspace. */
const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);
