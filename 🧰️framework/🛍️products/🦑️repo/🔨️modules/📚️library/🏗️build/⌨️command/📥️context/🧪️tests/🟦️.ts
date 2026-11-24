/** 🐍️ The command environment sets `SEMIO_PYTHON` from the provisioned environment, lets an explicit value win, leaves the key unset without one and restores it afterwards. */
import {describe, expect, test} from "bun:test";
import {mkdirSync, mkdtempSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import {pythonEnvironmentInterpreterV1, pythonEnvironmentOfProjectV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts";
import {repositoryPythonPatchV1, withProcessEnvironmentPatchV1} from "../🟦️.ts";

function withWorkspace(synchronized: boolean, body: (workspace: string, interpreter: string) => void | Promise<void>): Promise<void> {
  const workspace = mkdtempSync(join(tmpdir(), "semio-command-python-"));
  writeFileSync(join(workspace, "pyproject.toml"), "[project]\nname = \"a\"\n");
  writeFileSync(join(workspace, "uv.lock"), "version = 1\n");
  const interpreter = pythonEnvironmentInterpreterV1(pythonEnvironmentOfProjectV1(workspace, "workspace"));
  if (synchronized) {
    mkdirSync(dirname(interpreter), {recursive: true});
    writeFileSync(interpreter, "");
  }
  return Promise.resolve(body(workspace, interpreter)).finally(() => rmSync(workspace, {recursive: true, force: true}));
}

describe("the SEMIO_PYTHON entry of the repository command environment", () => {
  test("is the provisioned interpreter when the environment exists", () => withWorkspace(true, (workspace, interpreter) => {
    expect(repositoryPythonPatchV1({}, workspace)).toEqual({SEMIO_PYTHON: interpreter});
  }));

  test("keeps an explicit value over the provisioned interpreter", () => withWorkspace(true, workspace => {
    expect(repositoryPythonPatchV1({SEMIO_PYTHON: "explicit"}, workspace)).toEqual({SEMIO_PYTHON: "explicit"});
  }));

  test("leaves the key unset without a provisioned environment", () => withWorkspace(false, async workspace => {
    const saved = process.env.SEMIO_PYTHON;
    delete process.env.SEMIO_PYTHON;
    try {
      const patch = repositoryPythonPatchV1({}, workspace);
      expect(patch).toEqual({SEMIO_PYTHON: undefined});
      await withProcessEnvironmentPatchV1(patch, async () => {
        expect("SEMIO_PYTHON" in process.env).toBe(false);
      });
    } finally {
      if (saved !== undefined) process.env.SEMIO_PYTHON = saved;
    }
  }));

  test("is restored after the body, also when it throws", async () => {
    const saved = process.env.SEMIO_PYTHON;
    try {
      process.env.SEMIO_PYTHON = "before";
      await withProcessEnvironmentPatchV1({SEMIO_PYTHON: "patched"}, async previous => {
        expect(process.env.SEMIO_PYTHON).toBe("patched");
        expect(previous.SEMIO_PYTHON).toBe("before");
      });
      expect(process.env.SEMIO_PYTHON).toBe("before");
      await expect(withProcessEnvironmentPatchV1({SEMIO_PYTHON: "patched"}, async () => {
        throw new Error("boom");
      })).rejects.toThrow("boom");
      expect(process.env.SEMIO_PYTHON).toBe("before");
      delete process.env.SEMIO_PYTHON;
      await withProcessEnvironmentPatchV1({SEMIO_PYTHON: "patched"}, async () => {});
      expect("SEMIO_PYTHON" in process.env).toBe(false);
    } finally {
      if (saved === undefined) delete process.env.SEMIO_PYTHON; else process.env.SEMIO_PYTHON = saved;
    }
  });
});
