/** 🧪️ Tests for inputs read-oracle windows. */
import {expect, test} from "bun:test";
import {captureV1, resolveCommandV1} from "../../🏃️run/🟦️.ts";
import fixture from "./🔣️.json" with {type: "json"};

/** 🪟️ Contract: on Windows a plan command resolves through `PATH` and `PATHEXT` like the shell, an executable starts directly and a batch shim is refused in both languages instead of failing as "not found"; every other host keeps the name. The lookup rule is the documented `cmd.exe` order, so the vectors are authored from it. */
test("every Windows lookup vector resolves as the shell would", () => {
  for (const row of fixture.cases) {
    const files = new Set(row.files.map((file) => file.toLowerCase())), resolved = resolveCommandV1(row.command, row.env, "win32", (path) => files.has(path.toLowerCase()));
    if ("path" in row.expect) expect(resolved, row.name).toEqual({path: row.expect.path as string});
    else {
      expect("refusal" in resolved, row.name).toBe(true);
      const lines = (resolved as {refusal: string}).refusal.split("\n");
      expect(lines.length).toBe(2);
      expect(lines[0]).toContain((row.expect as {refusal: string}).refusal);
      expect(lines[0]).toContain("batch file");
      expect(lines[1]).toContain("Batch-Datei");
    }
  }
});

test("other hosts keep the name unchanged, whatever the files say", () => {
  for (const platform of ["linux", "darwin"]) expect(resolveCommandV1("npm", {PATH: "/bin"}, platform, () => true)).toEqual({path: "npm"});
});

test("a refused command ends the capture with the not-found status and the reason, without starting anything", async () => {
  const done = await captureV1(["npm", "run", "x"], {cwd: process.cwd(), env: {}, timeoutMs: 1000, resolution: () => ({refusal: "REASON"})});
  expect(done).toEqual({exit: 127, output: "REASON", seconds: 0, timedOut: false});
  const real = await captureV1([process.execPath, "-e", "console.log('ok')"], {cwd: process.cwd(), env: {PATH: process.env.PATH}, timeoutMs: 20000});
  expect(real.exit).toBe(0);
  expect(real.output.trim()).toBe("ok");
});
