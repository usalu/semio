import { describe } from "bun:test";
import { registerSafegitenvReplacesUnusableConfiguredTests } from "./🔶️safegitenv-replaces-unusable-configured/🟦️.ts";
import { registerBuildmicrocommitmetricsMergesUlocGitTests } from "./🔹️buildmicrocommitmetrics-merges-uloc-git/🟦️.ts";
import { registerInstallmicrocommitgithooksWritesPortableHooksTests } from "./🟡️installmicrocommitgithooks-writes-portab/🟦️.ts";

describe("micro-commit", () => {
  registerSafegitenvReplacesUnusableConfiguredTests();
  registerBuildmicrocommitmetricsMergesUlocGitTests();
  registerInstallmicrocommitgithooksWritesPortableHooksTests();
});
