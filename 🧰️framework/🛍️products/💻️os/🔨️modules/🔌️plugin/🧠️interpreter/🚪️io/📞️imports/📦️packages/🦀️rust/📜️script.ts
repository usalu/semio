import{BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runCargo}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{isMainModuleV1}from"../../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
/** 🧭️ Checks the host import implementation through the owned Cargo interface. */
class Check extends BundleScript{async run(args:string[]):Promise<void>{await runCargo(["check","--manifest-path","Cargo.toml",...args],this.root)}}
/** 🧪️ Runs the real Source and import service laws. */
class Test extends BundleScript{async run(args:string[]):Promise<void>{await runCargo(["test","--manifest-path","Cargo.toml","--lib",...args],this.root)}}
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(new ScriptRouter(import.meta.dirname).register("check",Check).register("test",Test),{invocation:original}));
