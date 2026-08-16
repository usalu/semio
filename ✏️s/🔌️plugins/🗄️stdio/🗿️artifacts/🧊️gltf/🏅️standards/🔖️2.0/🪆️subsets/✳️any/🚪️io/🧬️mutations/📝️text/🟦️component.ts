/** 📝 Canonical one-line semantic glTF mutation representation. */
import type { GltfMutation, GltfMutationRejection } from '../../../🧬️schema/🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type GltfMutationTextApplication = { text: string; value: GltfMutation; rejection?: never } | { text: string; value?: never; rejection: GltfMutationRejection };
export type GltfMutationsText = string;
