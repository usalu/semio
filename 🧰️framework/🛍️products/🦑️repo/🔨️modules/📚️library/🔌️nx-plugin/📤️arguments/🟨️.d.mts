import type { NativeOwnerCapabilities } from "../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 📤️ Encodes the closed language-neutral literal argument envelope. */
export function encodeOwnerArgumentsV1(arguments_: readonly string[]): string;
/** 📥️ Returns genuine original arguments and a private-key-free mutable environment. */
export function consumeOwnerArgumentsV1(environment: Readonly<Record<string, string | undefined>>): { arguments: string[]; environment: Record<string, string | undefined> };
/** 📜️ Admits the literal discovered native command behind repository-owned authority. */
export function declaredNativeOwnerCommandV1(manifest: string, workingDirectory: string, command: string): NativeOwnerCapabilities["command"];
/** 🔐️ Issues the native child role with its original caller envelope. */
export function issueNativeOwnerInvocationV1(options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>, environment: Readonly<Record<string, string | undefined>>, now: number): Promise<Readonly<Record<string, unknown>>>;
/** 🎯️ Delegates the original task using the closed private argument transport. */
export function executeOwnerArgumentsV1(options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>, delegate: (options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>) => Promise<unknown>): Promise<unknown>;
/** 🐚️ Keeps literal command arguments within the installed executor's owned lifecycle. */
export function executeLiteralBuildArgumentsV1<T>(options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>, delegate: (options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>) => Promise<T>): Promise<T>;
/** 🚀️ Runs the repository owner through the installed Nx process lifecycle. */
export default function executor(options: Readonly<Record<string, unknown>>, context: Readonly<Record<string, unknown>>): Promise<unknown>;
