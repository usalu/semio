import { bindCatalogV1 } from "../../../🗣️language/📖️catalog/🟦️.ts";
import { sayV1 } from "../../../🗣️language/📣️progress/🟦️.ts";
import { runOwnedCommand } from "../../../../🎛️owned-execution/🟦️.ts";
import { installOutputContractV1, machineOutputV1, writeResultLineV1 } from "../../🟦️.ts";

const [mode = "noise", ...args] = process.argv.slice(2);
installOutputContractV1(args);

if (mode === "noise") {
  console.log("[noise] console.log");
  console.info("[noise] console.info");
  console.debug("[noise] console.debug");
  process.stdout.write("[noise] process.stdout.write\n");
  writeResultLineV1(JSON.stringify({ ok: true }));
  await new Promise<void>((accept, reject) => machineOutputV1.write(JSON.stringify({ ok: "second" }) + "\n", (error) => (error ? reject(error) : accept())));
}

if (mode === "progress") {
  const steps = bindCatalogV1("steps", { version: 1, id: "steps", languages: ["en", "de"], messages: { done: { en: "[steps] done.", de: "[steps] fertig." } } });
  sayV1(steps, "done");
  writeResultLineV1(JSON.stringify({ ok: true }));
}

if (mode === "forward") {
  await runOwnedCommand(process.execPath, ["-e", "process.stdout.write(JSON.stringify({ child: true }) + String.fromCharCode(10))"], process.cwd(), "forward", 20000, { outputStyle: "stream-without-prefixes" });
}
