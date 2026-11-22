import {findToolV1,resolveToolV1,systemToolHostV1,toolRootV1} from "../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts";

/** 🧭️ The Bazel program of this checkout, found by the resolver `repo` itself uses (`SEMIO_TOOL_BAZEL`, then the provisioned pin of the tool registry); `undefined` only when Bazel is truly not provisioned, so tests and probes need no binary path of their own.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🔎️resolve/🟦️.ts */
export function resolveBazelBinaryV1(root:string=toolRootV1(),environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const entry=findToolV1("bazel");
 if(entry===undefined)return undefined;
 const found=resolveToolV1(entry,systemToolHostV1(root,environment));
 return found.command===null||found.prefix.length>0?undefined:found.command;
}
