import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { mkdirSync, mkdtempSync, readdirSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { readProcessOwnerContextV1, processCacheDirectoryV1 } from "../../../../🏃️process/📋️context/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
import { stageArtifacts } from "../../../../🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { acquireResourceLease } from "../../../../🏃️process/🔒️leases/🟦️.ts";

/** 🔐️ Protects the Python build environment while dependency preparation or wheel compilation is using it. */
async function withPythonEnvironment(root: string, action: (signal: AbortSignal) => Promise<void>): Promise<void> {
  const controller = new AbortController(), cancel = () => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  try {
    const environment = resolve(root, process.env.UV_PROJECT_ENVIRONMENT ?? ".venv");
    const lease = await acquireResourceLease({ directory: readProcessOwnerContextV1(process.env,root).leaseDirectory, resource: `styling-python:${environment}`, mode: "exclusive", signal: controller.signal, onWait: ({ elapsedMs }) => console.log(`Waiting for styling Python environment: ${elapsedMs}ms`) });
    try { controller.signal.throwIfAborted(); await action(controller.signal); } finally { lease.release(); }
  } finally { process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
}

/** 📥️ Installs locked wheel-building tools without compiling an editable first-party package. */
export class StylingPythonDepsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Python styling preparation uses the locked build group");
    await withPythonEnvironment(this.root, signal => runOwnedCommand("uv", ["sync", "--locked", "--group", "build", "--no-install-project", "--project", this.root], this.root, "styling-python-deps", buildBudgetMs(),{signal}));
  }
}

/** 📦️ Publishes one reproducible wheel from the prepared backend without dependency resolution or downloads. */
export class StylingPythonBuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The Python styling package has one wheel build contract");
    await withPythonEnvironment(this.root, async signal => {
      const staging = join(process.env.SEMIO_STYLING_PYTHON_BUILD_ROOT ?? processCacheDirectoryV1(readProcessOwnerContextV1(process.env,this.root),"python","ui-styling"), "staging");
      mkdirSync(staging, { recursive: true });
      const temporary = mkdtempSync(join(staging, "wheel-"));
      try {
        await runOwnedCommand("uv", ["run", "--locked", "--no-sync", "--offline", "--no-python-downloads", "--group", "build", "--project", this.root, "python", "-I", "-m", "hatchling", "build", "--target", "wheel", "--directory", temporary], this.root, "styling-python-build", buildBudgetMs(),{signal});
        const wheels = readdirSync(temporary).filter(name => name.endsWith(".whl"));
        if (wheels.length !== 1) throw new Error(`Expected one styling wheel, got ${wheels.length}`);
        await stageArtifacts(join(this.root, "dist/build"), "@semio-tech/ui-styling-py:build", new Map([[wheels[0]!, join(temporary, wheels[0]!)]]), { signal, leaseDirectory:readProcessOwnerContextV1(process.env,this.root).leaseDirectory });
        console.log(`Published styling Python wheel: ${wheels[0]}`);
      } finally { rmSync(temporary, { recursive: true, force: true }); }
    });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(resolve(import.meta.dir, "../../📦️packages/🐍️python")).register("deps", StylingPythonDepsScript).register("build", StylingPythonBuildScript)).run(process.argv.slice(2), original));
