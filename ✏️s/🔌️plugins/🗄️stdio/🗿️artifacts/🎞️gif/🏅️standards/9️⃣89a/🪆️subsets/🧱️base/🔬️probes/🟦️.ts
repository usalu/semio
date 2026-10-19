import { spawnSync } from "node:child_process";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const ENGINE = { family: "giflib", implementation: "giftext logical screen descriptor dump (giflib 6.1)", version: "6.1" } as const;
const PROBE_VERSION = "giflib@6.1";
//#endregion 🧬️Contract

//#region 🔬️Probe
/** 🖥️ The logical screen descriptor as giflib reports it.
 *
 *  `giftext` echoes the INPUT PATH in its output; that line is dropped, because a projection carrying
 *  the fixture's own filename would differ for reasons that have nothing to do with the bytes. */
function project(absPath: string): Record<string, unknown> {
  const result = spawnSync("giftext", [absPath], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`giftext refused ${absPath}: ${result.stderr}`);
  const lines = String(result.stdout)
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.includes(absPath));
  const screen = lines.find((line) => line.startsWith("Screen Size")) ?? null;
  const descriptor = lines.find((line) => line.includes("Aspect =")) ?? null;
  const field = (source: string | null, name: string): number | null => {
    if (!source) return null;
    const match = source.match(new RegExp(`${name}\\s*=\\s*(\\d+)`));
    return match ? Number(match[1]) : null;
  };
  return {
    screenWidth: field(screen, "Width"),
    screenHeight: field(screen, "Height"),
    colorResolution: field(descriptor, "ColorResolution"),
    bitsPerPixel: field(descriptor, "BitsPerPixel"),
    background: field(descriptor, "BackGround"),
    aspect: field(descriptor, "Aspect"),
    imageCount: lines.filter((line) => line.startsWith("Image #")).length,
  };
}

function emit(probe: string, status: string, measurements: Record<string, unknown>, message?: string): number {
  const report = {
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: PROBE_VERSION,
    engine: ENGINE,
    status,
    durationMs: 0,
    measurements,
    ...(message ? { diagnostics: [{ severity: "error", message }] } : {}),
  };
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  return status === "failed" ? 1 : 0;
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  //#endregion 🔬️Probe
  
  //#region 🚀️Entry
  const [probe = "", ...rest] = process.argv.slice(2);
  const inputs: string[] = [];
  for (let index = 0; index < rest.length; index += 1) if (rest[index] === "--input") inputs.push(rest[index + 1]!);
  try {
    if (probe === "gif-screen-project") process.exit(emit(probe, "ok", project(inputs[0]!)));
    else if (probe === "gif-screen-compare") {
      const expected = project(inputs[0]!);
      const actual = project(inputs[1]!);
      process.exit(emit(probe, "ok", { equal: JSON.stringify(expected) === JSON.stringify(actual), expected, actual }));
    } else {
      console.error("usage: bun 📜️script.ts <gif-screen-project|gif-screen-compare> --input <path> [--input <path>]");
      process.exit(2);
    }
  } catch (error) {
    process.exit(emit(probe || "(none)", "failed", {}, String((error as Error).message ?? error)));
  }
}
