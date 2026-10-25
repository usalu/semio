import{findToolV1,toolLanguageV1}from"../../../🧰️tools/📖️registry/🟦️.ts";
import{readToolSourceV1,systemToolPinPortV1,type ToolPinPortV1}from"../../../🧰️tools/📌️pin/🟦️.ts";
import{toolRootV1}from"../../../🧰️tools/🔎️resolve/🟦️.ts";
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
