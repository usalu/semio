import {AsyncLocalStorage} from "node:async_hooks";
import {parseActivationReceipt,type ActivationReceipt} from "@semio-tech/framework-os-dev/activation";
import {resolveTestBrowserHostRootsV1} from "../🌐️browser-host/🟦️.ts";
import { COMPONENT_MODULE_DIRECTORIES, EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../🤖️generated/🧩️plugins/🟦️.ts";
import { DEFAULT_PLAYGROUND_VARIANT, PLAYGROUND_BUILD_TARGETS } from "../🤖️generated/🎮️playgrounds/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "@semio-tech/plugin-registry/playground/assets/composition";
const activationPortV1=new AsyncLocalStorage<ActivationReceipt>();
/** 💉️ Runs `body` with the activation receipt a served variant reads supplied by the caller, parsed through the activation schema, instead of the machine's activation directory. A test supplies its own receipt through it and never reads machine state.
 * @see ../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts */
export function withSpecificOsDevActivationV1<T>(receipt:unknown,body:()=>Promise<T>):Promise<T>{return activationPortV1.run(parseActivationReceipt(receipt),body);}
/** 🏗️ Specific builders explicitly contribute their installed catalog and asset delivery providers. */
export function specificOsDevBuildInventoryV1() { return { plugins: PLUGIN_BUILD_TARGETS, extensions: EXTENSION_TARGETS, playgrounds: PLAYGROUND_BUILD_TARGETS, moduleDirectories: COMPONENT_MODULE_DIRECTORIES, assetProviders: PLAYGROUND_ASSET_PROVIDERS_V1, activation:()=>activationPortV1.getStore(),browserHost:()=>{const roots=resolveTestBrowserHostRootsV1(process.env);return roots?{moduleRoot:roots.moduleRoot,activationRoot:roots.activationRoot,extensionsRoot:roots.browserHostRoot+"/extensions"}:undefined;}, defaultVariant: DEFAULT_PLAYGROUND_VARIANT }; }
