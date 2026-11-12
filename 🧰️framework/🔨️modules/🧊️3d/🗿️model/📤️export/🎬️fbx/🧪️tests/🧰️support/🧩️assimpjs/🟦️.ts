/** 🧩️ A typed handle on the `assimpjs` WebAssembly build of assimp, the second independent FBX reader of the tests (a test-only dependency; the package ships no types). */
import { createRequire } from "node:module";

/** 🧾️ The result of one conversion. */
export type AssimpResult = { IsSuccess(): boolean; GetErrorCode(): string; GetFile(index: number): { GetContent(): Uint8Array } };
/** 🔧️ The module surface the tests use. */
export type Assimp = { FileList: new () => { AddFile(name: string, content: Uint8Array): void }; ConvertFileList(list: InstanceType<Assimp["FileList"]>, format: "assjson"): AssimpResult };

/** 📥️ Instantiates assimp. */
export async function loadAssimp(): Promise<Assimp> {
  const factory = createRequire(import.meta.url)("assimpjs") as () => Promise<Assimp>;
  return factory();
}
