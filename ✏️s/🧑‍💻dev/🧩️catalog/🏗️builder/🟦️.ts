import { COMPONENT_MODULE_DIRECTORIES, EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { DEFAULT_PLAYGROUND_VARIANT, PLAYGROUND_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition/🟦️.ts";
/** 🏗️ Specific builders explicitly contribute their installed catalog and asset delivery providers. */
export function specificOsDevBuildInventoryV1() { return { plugins: PLUGIN_BUILD_TARGETS, extensions: EXTENSION_TARGETS, playgrounds: PLAYGROUND_BUILD_TARGETS, moduleDirectories: COMPONENT_MODULE_DIRECTORIES, assetProviders: PLAYGROUND_ASSET_PROVIDERS_V1, defaultVariant: DEFAULT_PLAYGROUND_VARIANT }; }
