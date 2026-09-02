/** 🧬 Transparent TypeScript aggregate for the buffer slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateBufferPayload } from './🌱️💾️create-buffer/🟦️.ts';
import type { GltfCreateBufferViewPayload } from './🌱️👁️create-buffer-view/🟦️.ts';
import type { GltfDeleteBufferPayload } from './🗑️💾️delete-buffer/🟦️.ts';
import type { GltfDeleteBufferViewPayload } from './🗑️👁️delete-buffer-view/🟦️.ts';
import type { GltfMoveBufferPayload } from './🚚️💾️move-buffer/🟦️.ts';
import type { GltfMoveBufferViewPayload } from './🚚️👁️move-buffer-view/🟦️.ts';
import type { GltfReorderBufferViewsPayload } from './🔀️👁️reorder-buffer-views/🟦️.ts';
import type { GltfReorderBuffersPayload } from './🔀️💾️reorder-buffers/🟦️.ts';

export type GltfBufferMutation =
  | { readonly mutation: 'createBufferView'; readonly payload: GltfCreateBufferViewPayload }
  | { readonly mutation: 'createBuffer'; readonly payload: GltfCreateBufferPayload }
  | { readonly mutation: 'reorderBufferViews'; readonly payload: GltfReorderBufferViewsPayload }
  | { readonly mutation: 'reorderBuffers'; readonly payload: GltfReorderBuffersPayload }
  | { readonly mutation: 'deleteBufferView'; readonly payload: GltfDeleteBufferViewPayload }
  | { readonly mutation: 'deleteBuffer'; readonly payload: GltfDeleteBufferPayload }
  | { readonly mutation: 'moveBufferView'; readonly payload: GltfMoveBufferViewPayload }
  | { readonly mutation: 'moveBuffer'; readonly payload: GltfMoveBufferPayload };
