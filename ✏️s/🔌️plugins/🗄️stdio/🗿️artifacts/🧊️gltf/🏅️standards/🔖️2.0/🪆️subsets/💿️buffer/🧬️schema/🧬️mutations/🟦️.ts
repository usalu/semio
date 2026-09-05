/** 🧬 Transparent TypeScript aggregate for the buffer slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateBufferPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💿️buffer/🌱️create/🟦️.ts';
import type { GltfCreateBufferViewPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🌱️create/🟦️.ts';
import type { GltfDeleteBufferPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💿️buffer/🗑️delete/🟦️.ts';
import type { GltfDeleteBufferViewPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🗑️delete/🟦️.ts';
import type { GltfMoveBufferPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💿️buffer/🚚️move/🟦️.ts';
import type { GltfMoveBufferViewPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🚚️move/🟦️.ts';
import type { GltfReorderBufferViewsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🪟️buffer-view/🔀️reorder/🟦️.ts';
import type { GltfReorderBuffersPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💿️buffer/🔀️reorder/🟦️.ts';

export type GltfBufferMutation =
  | { readonly mutation: 'createBufferView'; readonly payload: GltfCreateBufferViewPayload }
  | { readonly mutation: 'createBuffer'; readonly payload: GltfCreateBufferPayload }
  | { readonly mutation: 'reorderBufferViews'; readonly payload: GltfReorderBufferViewsPayload }
  | { readonly mutation: 'reorderBuffers'; readonly payload: GltfReorderBuffersPayload }
  | { readonly mutation: 'deleteBufferView'; readonly payload: GltfDeleteBufferViewPayload }
  | { readonly mutation: 'deleteBuffer'; readonly payload: GltfDeleteBufferPayload }
  | { readonly mutation: 'moveBufferView'; readonly payload: GltfMoveBufferViewPayload }
  | { readonly mutation: 'moveBuffer'; readonly payload: GltfMoveBufferPayload };
