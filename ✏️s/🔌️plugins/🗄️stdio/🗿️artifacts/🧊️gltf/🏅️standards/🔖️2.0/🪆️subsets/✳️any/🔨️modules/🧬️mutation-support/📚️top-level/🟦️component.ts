/** 🔒 Pure mechanics private to executable document-level glTF leaves. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
export interface GltfMutationRejection { code: string; path: string; detail: string }
export type GltfLeafResult = { accepted: true; snapshot: GltfSnapshot } | { accepted: false; rejection: GltfMutationRejection };
export const reject = (code: string, path: string, detail: string): GltfMutationRejection => ({ code, path, detail });
export const clone = (snapshot: GltfSnapshot): GltfSnapshot => structuredClone(snapshot);
export const same = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
export const run = <P>(base: GltfSnapshot, payload: P, validate: (payload: P, base: GltfSnapshot) => GltfMutationRejection | undefined, mutate: (snapshot: GltfSnapshot, payload: P) => void): GltfLeafResult => { const rejection = validate(payload, base); if (rejection) return { accepted: false, rejection }; const snapshot = clone(base); mutate(snapshot, payload); return { accepted: true, snapshot }; };
