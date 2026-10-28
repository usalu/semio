import {expect, test} from "bun:test";
import {spawn} from "node:child_process";
import {EventEmitter} from "node:events";
import {fileURLToPath} from "node:url";
import {onTerminationV1, terminationSignalsV1} from "../../🏁️main/🟦️.ts";

/** 🛑️ Contract: a Windows process listens to the console signals as well (`SIGBREAK`, `SIGHUP`), because `SIGTERM` never reaches it; a POSIX process keeps `SIGINT` and `SIGTERM`. The handler runs once per arrival and the disposer removes every listener. */
const module = fileURLToPath(new URL("../../🏁️main/🟦️.ts", import.meta.url));

test("the signal set depends on the platform", () => {
  expect(terminationSignalsV1("win32")).toEqual(["SIGINT", "SIGTERM", "SIGBREAK", "SIGHUP"]);
  for (const platform of ["linux", "darwin"]) expect(terminationSignalsV1(platform)).toEqual(["SIGINT", "SIGTERM"]);
});

test("every Windows console signal reaches the handler once and the disposer removes the listeners", () => {
  const target = new EventEmitter(), seen: string[] = [];
  const dispose = onTerminationV1((signal) => seen.push(signal), "win32", target as never);
  for (const signal of terminationSignalsV1("win32")) { target.emit(signal); target.emit(signal); }
  expect(seen).toEqual(["SIGINT", "SIGTERM", "SIGBREAK", "SIGHUP"]);
  const again = onTerminationV1(() => seen.push("x"), "win32", target as never);
  again();
  for (const signal of terminationSignalsV1("win32")) expect(target.listenerCount(signal)).toBe(0);
  dispose();
});

test.skipIf(process.platform==="win32")("a real process stops on SIGTERM through the handler (POSIX hosts)", async () => {
  const child = spawn(process.execPath, ["-e", `import(${JSON.stringify(module)}).then(({onTerminationV1})=>{onTerminationV1(signal=>{console.log("stopped:"+signal);process.exit(0);});console.log("ready");setInterval(()=>{},1000);});`], {stdio: ["ignore", "pipe", "inherit"]});
  let output = "";
  const done = new Promise<number | null>((resolve) => child.on("close", resolve));
  await new Promise<void>((resolve) => child.stdout.on("data", (chunk) => { output += chunk; if (output.includes("ready")) resolve(); }));
  child.kill("SIGTERM");
  expect(await done).toBe(0);
  expect(output).toContain("stopped:SIGTERM");
});
