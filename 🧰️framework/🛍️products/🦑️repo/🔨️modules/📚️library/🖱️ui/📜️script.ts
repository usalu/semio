
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runDevScript, runBuildScript, runCheckUiPrimitivesScript, runCheckChromeI18nScript } from "./🟦️.ts";

/** 📚️ This bundle has no local `.storybook` config — its stories live in the root Storybook's `ui` scope, so `dev`/`build` delegate there instead of running a broken standalone instance. */
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDevScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🚫️ Fails when raw DOM primitives (`<button>`, `<svg>`, …) or `component:` escape hatches leak outside `ui/`/`framework/`,
 * or when the allowlist carries a stale entry for a file that no longer has any hits. */
class CheckUiPrimitivesScript extends BundleScript {
  async run(): Promise<void> {
    await runCheckUiPrimitivesScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

/** 🚫️ Fails when a scanned chrome file has an untranslated-English hit outside the allowlist, or when the
 * allowlist carries a stale entry for a file that no longer has any hits. */
class CheckChromeI18nScript extends BundleScript {
  async run(): Promise<void> {
    await runCheckChromeI18nScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
//#endregion 🌐️chrome-i18n-lint


const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("check-ui-primitives", CheckUiPrimitivesScript)
  .register("check-chrome-i18n", CheckChromeI18nScript);

await runRepoScriptMain(router, import.meta.url);
