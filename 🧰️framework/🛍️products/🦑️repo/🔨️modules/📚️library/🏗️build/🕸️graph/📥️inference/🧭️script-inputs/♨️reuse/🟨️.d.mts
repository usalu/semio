/** ♨️ The reuse store of TypeScript programs a long-lived deriver keeps between questions. */
export type TypeScriptProgramReuseV1 = Readonly<{
  begin(arm: boolean): void;
  end(): Readonly<{ sources: ReadonlySet<string>; reused: readonly number[] }>;
  sourceFile(config: string, original: (...args: any[]) => any, readFile: (path: string) => string | undefined, settings?: string): (...args: any[]) => any;
  oldProgram(config: string): unknown;
  keep(config: string, program: { getSourceFiles(): readonly { fileName: string }[]; structureIsReused?: number }): void;
}>;
/** 🏭️ A fresh reuse store. */
export function createTypeScriptProgramReuseV1(): TypeScriptProgramReuseV1;
/** 🤝️ The shared parsed source files of the programs of every owner. */
export type SharedSourceFilesV1 = Readonly<{ shared: true; sourceFile: TypeScriptProgramReuseV1["sourceFile"]; oldProgram(config: string): undefined; keep(): void; clear(): void; size(): number; characters(): number; textBytes(): number }>;
export function createSharedSourceFilesV1(options?: { maximumFiles?: number; maximumCharacters?: number }): SharedSourceFilesV1;
export const sharedSourceFilesV1: SharedSourceFilesV1;
/** 🔌️ Installs the store the program questions of this process use. */
export function useTypeScriptProgramReuseV1(store: TypeScriptProgramReuseV1 | undefined): void;
/** 🔎️ The installed store, if any. */
export function activeTypeScriptProgramReuseV1(): TypeScriptProgramReuseV1 | undefined;
