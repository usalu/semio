/** 🔒 Validation mechanics private to structure and geometry mutation leaves. */
import { reject, type GltfMutationRejection } from '../📚️top-level/🟦️component.ts';
export const itemIndex = (value: number, length: number, path: string): GltfMutationRejection | undefined => Number.isInteger(value) && value >= 0 && value < length ? undefined : reject('gltf.mutation.index-out-of-range', path, `index ${value} is outside 0..${Math.max(0, length - 1)}`);
export const positionIn = (value: number, length: number, path: string): GltfMutationRejection | undefined => Number.isInteger(value) && value >= 0 && value <= length ? undefined : reject('gltf.mutation.position-out-of-range', path, `position ${value} is outside 0..${length}`);
