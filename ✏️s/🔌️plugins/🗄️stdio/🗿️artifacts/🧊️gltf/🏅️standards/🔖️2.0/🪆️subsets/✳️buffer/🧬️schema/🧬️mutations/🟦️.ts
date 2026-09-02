/** 🧬 Transparent TypeScript aggregate for the buffer slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateBufferPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️💾️create-buffer/🟦️.ts';
import type { GltfCreateBufferViewPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️👁️create-buffer-view/🟦️.ts';
import type { GltfDeleteBufferPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️💾️delete-buffer/🟦️.ts';
import type { GltfDeleteBufferViewPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️👁️delete-buffer-view/🟦️.ts';
import type { GltfMoveBufferPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️💾️move-buffer/🟦️.ts';
import type { GltfMoveBufferViewPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️👁️move-buffer-view/🟦️.ts';
import type { GltfReorderBufferViewsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️👁️reorder-buffer-views/🟦️.ts';
import type { GltfReorderBuffersPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️💾️reorder-buffers/🟦️.ts';

export type GltfBufferMutation =
  | { readonly mutation: 'createBufferView'; readonly payload: GltfCreateBufferViewPayload }
  | { readonly mutation: 'createBuffer'; readonly payload: GltfCreateBufferPayload }
  | { readonly mutation: 'reorderBufferViews'; readonly payload: GltfReorderBufferViewsPayload }
  | { readonly mutation: 'reorderBuffers'; readonly payload: GltfReorderBuffersPayload }
  | { readonly mutation: 'deleteBufferView'; readonly payload: GltfDeleteBufferViewPayload }
  | { readonly mutation: 'deleteBuffer'; readonly payload: GltfDeleteBufferPayload }
  | { readonly mutation: 'moveBufferView'; readonly payload: GltfMoveBufferViewPayload }
  | { readonly mutation: 'moveBuffer'; readonly payload: GltfMoveBufferPayload };
