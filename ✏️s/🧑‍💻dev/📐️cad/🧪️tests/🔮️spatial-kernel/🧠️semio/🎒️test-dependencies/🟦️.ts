/** 🎒️ The values the module hands its extracted suites, owned by the module directory so that the suites never import the module back. */

import type { SemioBrepKernel } from "@semio-tech/s-spatial-kernel-ts/semio";

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts`. */
export type SemioTestDependencies = {
  readonly SemioBrepKernel: typeof SemioBrepKernel;
};
