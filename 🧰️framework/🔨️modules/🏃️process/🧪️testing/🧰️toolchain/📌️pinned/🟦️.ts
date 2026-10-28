import{findToolV1,toolLanguageV1}from"../../../🧰️tools/📖️registry/🟦️.ts";
import{readToolSourceV1,systemToolPinPortV1,type ToolPinPortV1}from"../../../🧰️tools/📌️pin/🟦️.ts";
import{resolveToolV1,systemToolHostV1,toolRootV1,type ToolHostV1}from"../../../🧰️tools/🔎️resolve/🟦️.ts";
import protocol from"./🔣️.json"with{type:"json"};

/** 📌️ The Rust toolchain the repository pins and the environment that runs every `cargo` and `rustc` of a test on it. */
export type PinnedRustToolchainV1=Readonly<{channel:string;environment:Readonly<{RUSTUP_TOOLCHAIN:string}>}>;

const message=(key:keyof typeof protocol.messages,values:Readonly<Record<string,string>>,environment:Readonly<Record<string,string|undefined>>):string=>{
 const language=toolLanguageV1(environment);
 return protocol.messages[key][language].replace(/\{(\w+)\}/gu,(whole,name:string)=>values[name]??whole);
};

/** 🧷️ Reads the channel of the registered `cargo` tool pin, the one source the product reads, and answers the toolchain selection a test spawns with: rustup chooses by the directory file only inside the checkout, so a scratch workspace silently gets the machine default (the stable channel refuses every `-Z` flag). The compiler program is never named or replaced, and an incoming selection that disagrees with the pin is refused like the product refuses it.
 * @see ../../../🧰️tools/🔣️.json */
export function pinnedRustToolchainV1(root:string=toolRootV1(import.meta.dirname),environment:Readonly<Record<string,string|undefined>>=process.env,port:ToolPinPortV1=systemToolPinPortV1):PinnedRustToolchainV1{
 const entry=findToolV1(protocol.tool);if(entry===undefined)throw Error(message("unregistered",{tool:protocol.tool},environment));
 const channel=readToolSourceV1(entry.pin.version,root,port);if(channel===null||channel==="")throw Error(message("unpinned",{tool:protocol.tool,root},environment));
 const incoming=environment.RUSTUP_TOOLCHAIN;if(incoming!==undefined&&incoming!==""&&incoming!==channel&&!incoming.startsWith(channel+"-"))throw Error(message("disagrees",{incoming,channel},environment));
 return Object.freeze({channel,environment:Object.freeze({RUSTUP_TOOLCHAIN:channel})});
}

/** 🧲️ The environment variable that hands the pinned binary of a registered tool to the tests of a route: `SEMIO_TEST_<TOOL>_BINARY`. */
export const pinnedTestToolVariableV1=(id:string):string=>"SEMIO_TEST_"+id.toUpperCase().replace(/-/gu,"_")+"_BINARY";

/** 🔩️ Resolves every named tool from the tool registry exactly as the product does (environment override, then the provisioned pin) and answers the environment that gives each program to the laws that run it; a tool the registry does not know or that is not provisioned on this host is refused in the chosen language, so a law that needs a real tool fails instead of skipping.
 * @see ../../../🧰️tools/🔣️.json */
export function pinnedTestToolsV1(ids:readonly string[],root:string=toolRootV1(import.meta.dirname),environment:Readonly<Record<string,string|undefined>>=process.env,host:ToolHostV1=systemToolHostV1(root,environment)):Readonly<Record<string,string>>{
 const bound:Record<string,string>={};
 for(const id of ids){
  const entry=findToolV1(id);if(entry===undefined)throw Error(message("unregisteredTool",{tool:id},environment));
  const found=resolveToolV1(entry,host);
  if(found.command===null||found.prefix.length>0)throw Error(message("missingTool",{tool:id,tried:found.tried.join(", ")},environment));
  bound[pinnedTestToolVariableV1(id)]=found.command;
 }
 return Object.freeze(bound);
}
