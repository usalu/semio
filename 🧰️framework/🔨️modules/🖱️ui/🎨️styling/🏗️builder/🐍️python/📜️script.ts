import{BundleScript,ScriptRouter,receiveScriptProcessInvocation}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{buildBudgetMs}from"../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{runStylingPythonV1}from"./🟦️.ts";
import{resolve}from"node:path";
/** 📥️ Mutates only the genuinely held locked Python dependency environment. */
export class StylingPythonDepsScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("Python deps accepts no options");await runStylingPythonV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"styling:python-deps",buildBudgetMs()),{version:1,operation:"deps",mode:readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode});}}
/** 🎡️ Publishes one wheel after actual compiler CPU and accepted children settle. */
export class StylingPythonBuildScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("Python build accepts no options");await runStylingPythonV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"styling:python-build",buildBudgetMs()),{version:1,operation:"build",mode:readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode});}}
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>new ScriptRouter(resolve(import.meta.dir,"../../📦️packages/🐍️python")).register("deps",StylingPythonDepsScript).register("build",StylingPythonBuildScript).run(process.argv.slice(2),original));
