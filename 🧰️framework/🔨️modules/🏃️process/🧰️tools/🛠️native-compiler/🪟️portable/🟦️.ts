import { win32 } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = schema.definitions.Protocol.const;

/** 🗂️ Names provider paths beneath a supplied store root without asserting ownership or readiness. */
export function portableMsvcPathsV1(root: string) {
  const store = win32.join(root, ...policy.partition), compiler = win32.join(store, "VC", "Tools", "MSVC", policy.compilerVersion), sdk = win32.join(store, "sdk"), bin = win32.join(compiler, "bin", "Hostx64", "x64"), sdkInclude = win32.join(sdk, "Include", policy.sdkVersion), sdkLib = win32.join(sdk, "Lib", policy.sdkVersion);
  return { store, compiler, sdk, bin, sdkInclude, sdkLib, cl: win32.join(bin, "cl.exe"), link: win32.join(bin, "link.exe"), lib: win32.join(bin, "lib.exe") };
}

/** 🎯️ Classifies explicit native compiler choices without asserting provider readiness. */
export function portableMsvcEligibilityV1(environment:Readonly<Record<string,string|undefined>>,context:Readonly<{platform:string;architecture:string;target:string|null}>):"eligible"|"platform"|"architecture"|"target"|"override"{
 if(context.platform!=="win32")return "platform";
 if(context.architecture!=="x64")return "architecture";
 if(context.target!==null&&context.target!==policy.target)return "target";
 const names=new Set(policy.explicitEnvironment.map(name=>name.toLowerCase())),patterns=policy.explicitPatterns.map(pattern=>new RegExp("^"+pattern.toLowerCase().replaceAll("*",".*")+"$","u"));
 return Object.entries(environment).some(([name,value])=>value!==undefined&&(names.has(name.toLowerCase())||patterns.some(pattern=>pattern.test(name.toLowerCase()))))?"override":"eligible";
}

