/** 🧮️ The hub stdio package identities, the component manifest, wasm and descriptor names and the shipped artifact, io, deadline and function bounds. */
import { SHIPPED_COMPONENT_MAX_BYTES } from "@semio-tech/framework-plugin/describe/component-receipt";

export const PACKAGE_NAME = "semio-hub-stdio";
export const PLUGIN_ID = "stdio";
export const ARTIFACT_OWNER = "✏️s/🔌️plugins/🗄️stdio";
export const COMPONENT_PACKAGE = "semio-hub-stdio-component";
export const COMPONENT_MANIFEST = "../../🧩️component/📦️packages/🦀️rust/Cargo.toml";
export const WASM_OUT = "semio_hub_stdio_component.wasm";
export const DESCRIPTOR_PACK = "🛂️.descriptor.semio";
export const DESCRIPTOR_JSON = "🔣️.json";
export const ARTIFACT_MAX_BYTES = SHIPPED_COMPONENT_MAX_BYTES;
export const IO_CHUNK_BYTES = 64 * 1024;
export const CATALOG_DEADLINE_MS = 1_200_000;
export const COMPONENT_FUNCTION_MAX = 1_000_000;
export const COMPONENT_PROFILE = "release";
