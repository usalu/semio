import {nativeSystemFromProviderV1} from "../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts";
import {nodeNativeLinkerEnvironmentV1} from "../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts";
import {toolForCommandV1,resolveRegisteredToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🧲️ Prepares the received Node caller before its first Native path or storage demand, without changing ambient environment. */
export function prepareNxDaemonNativeV1(original:ScriptInvocation,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string|undefined>>{
 requireIssuedResourceControlInvocationV1(control,original);checkResourceLeaseControl(control);
 if("bun" in process.versions)return environment;
 const root=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot,compiler=process.platform==="linux"?toolForCommandV1("cc"):undefined,registered=compiler?resolveRegisteredToolV1(compiler,root,{env:environment}):undefined;
 const prepared=registered?nodeNativeLinkerEnvironmentV1(environment,{...registered,suffix:registered.suffix??[]}):environment;
 nativeSystemFromProviderV1(prepared);checkResourceLeaseControl(control);return prepared;
}
