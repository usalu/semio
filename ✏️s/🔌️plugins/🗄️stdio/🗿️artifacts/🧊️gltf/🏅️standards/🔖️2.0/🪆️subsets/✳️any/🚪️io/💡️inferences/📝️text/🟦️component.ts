/** 📝 Canonical UTF-8 text I/O envelope for the complete glTF geometric inference. */
import type { GltfInference } from '../../../🧬️schema/💡️inferences/🟦️component.ts';
export const GLTF_INFERENCE_TEXT_SCHEMA = 's.stdio.gltf.inference' as const;
export const GLTF_INFERENCE_TEXT_VERSION = 2 as const;
export interface GltfInferenceTextDocument { schema: typeof GLTF_INFERENCE_TEXT_SCHEMA; version: typeof GLTF_INFERENCE_TEXT_VERSION; payloadLength: number; checksum: string; value: GltfInference }
/** 🔤 Four LF-terminated headers followed by RFC 8785 canonical JSON for `value`, without a trailing LF. */
export type GltfInferenceText = string;
