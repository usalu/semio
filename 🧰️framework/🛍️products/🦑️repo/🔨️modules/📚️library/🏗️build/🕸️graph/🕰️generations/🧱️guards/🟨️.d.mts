import type { GenerationPolicyV1 } from "../🟨️.mjs";
/** 📏️ The declared names and limits of the graph generation store; the schema file is their single source. */
export const GENERATION_POLICY_V1: GenerationPolicyV1;
export const P: GenerationPolicyV1;
export const HEX: RegExp;
export function sha256(bytes: string | Uint8Array): string;
export function isObject(value: unknown): value is Record<string, unknown>;
export function unpack(bytes: Uint8Array, limit: number): unknown;
export function aborted(signal?: AbortSignal): void;
export function fail(message: string, code?: string): Error & { code?: string };
export function requireKeys(value: unknown, required: readonly string[], optional: readonly string[], what: string): void;
export function requireHex(value: unknown, what: string): void;
export function requireCount(value: unknown, what: string, maximum?: number): void;
export function requireText(value: unknown, what: string, maximum?: number): void;
/** ⏱️ A yielder for one long loop: `await yielder()` is free until `yieldMilliseconds` of the loop have passed. */
export function createYielderV1(): () => Promise<void>;
