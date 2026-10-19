import {resolveTestBrowserHostRootsV1} from "../🌐️browser-host/🟦️.ts";
import { COMPONENT_MODULE_DIRECTORIES, EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../🤖️generated/🧩️plugins/🟦️.ts";
import { DEFAULT_PLAYGROUND_VARIANT, PLAYGROUND_BUILD_TARGETS } from "../🤖️generated/🎮️playgrounds/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "@semio-tech/plugin-registry/playground/assets/composition";
/** 🏗️ Specific builders explicitly contribute their installed catalog and asset delivery providers. */
export function specificOsDevBuildInventoryV1() { return { plugins: PLUGIN_BUILD_TARGETS, extensions: EXTENSION_TARGETS, playgrounds: PLAYGROUND_BUILD_TARGETS, moduleDirectories: COMPONENT_MODULE_DIRECTORIES, assetProviders: PLAYGROUND_ASSET_PROVIDERS_V1, browserHost:()=>{const roots=resolveTestBrowserHostRootsV1(process.env);return roots?{moduleRoot:roots.moduleRoot,activationRoot:roots.activationRoot,extensionsRoot:roots.browserHostRoot+"/extensions"}:undefined;}, defaultVariant: DEFAULT_PLAYGROUND_VARIANT }; }
