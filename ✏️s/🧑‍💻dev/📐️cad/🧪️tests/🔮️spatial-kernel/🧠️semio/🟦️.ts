/// <reference types="vitest/importMeta" />
import { SemioBrepKernel } from "@semio-tech/s-spatial-kernel-ts/semio";
// #region 🧪️Tests
/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts`. */
export type SemioTestDependencies = {
  readonly SemioBrepKernel: typeof SemioBrepKernel;
};

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts");
  await registerTests1(import.meta.vitest, { SemioBrepKernel }, { url: import.meta.url });
}
// #endregion 🧪️Tests
