/** 👓️ One view of a command walk: the prefix set it leaves out and its contract in English and German. */
export interface WalkViewV1 { readonly prefixes: "scheduling" | "scheduling-native"; readonly contract: { readonly en: string; readonly de: string } }
/** 📚️ The closed view policy. */
export interface WalkViewPolicyV1 { readonly version: 1; readonly views: { readonly scheduling: WalkViewV1; readonly native: WalkViewV1 }; readonly recipes: Readonly<Record<string, "scheduling" | "native">>; readonly closures: Readonly<Record<string, "scheduling" | "native" | "whole">> }
export const WALK_VIEW_POLICY_V1: WalkViewPolicyV1;
export function walkViewOfRecipeV1(recipe: { readonly fn: string; readonly role?: string; readonly view?: string }): "scheduling" | "native" | undefined;
export function closureRoleViewV1(role: string): "scheduling" | "native" | undefined;
export function walkViewPrefixesV1(view: string): readonly string[];
export function walkViewAnswerV1(value: readonly unknown[], compiled: readonly string[], generated: readonly unknown[], view?: string, ports?: readonly string[]): string;
