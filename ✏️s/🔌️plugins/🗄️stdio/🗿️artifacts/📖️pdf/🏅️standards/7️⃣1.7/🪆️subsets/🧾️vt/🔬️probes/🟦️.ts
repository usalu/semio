import { runToolSyncV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";


//#endregion 🔌️Adapters

//#region 🧬️Contract
const ENGINE = { family: "pypdf", implementation: "pypdf 6.14 standard security handler reader", version: "6.14.2" } as const;
const PROBE_VERSION = "pypdf@6.14.2";

const READER = String.raw`
import json, sys
from pypdf import PdfReader

def project(path):
    reader = PdfReader(path)
    encrypted = reader.is_encrypted
    out = {"encrypted": bool(encrypted), "pageCount": None, "encryption": None}
    if encrypted:
        try:
            reader.decrypt("")
        except Exception:
            pass
        enc = getattr(reader, "encryption", None)
        if enc is not None:
            out["encryption"] = {"algorithm": str(getattr(enc, "algorithm", None))}
    try:
        out["pageCount"] = len(reader.pages)
    except Exception:
        out["pageCount"] = None
    return out

paths = sys.argv[1:]
if len(paths) == 1:
    print(json.dumps(project(paths[0]), sort_keys=True))
else:
    a, b = project(paths[0]), project(paths[1])
    print(json.dumps({"equal": a == b, "expected": a, "actual": b}, sort_keys=True))
`;
//#endregion 🧬️Contract

//#region 🔬️Probe
function run(paths: readonly string[]): Record<string, unknown> {
  const result = runToolSyncV1("python", ["-c", READER, ...paths], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`pypdf reader failed: ${result.stderr}`);
  return JSON.parse(result.stdout) as Record<string, unknown>;
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
    if (probe === "pdf-encryption-project") process.exit(emit(probe, "ok", run([inputs[0]!])));
    else if (probe === "pdf-encryption-compare") process.exit(emit(probe, "ok", run([inputs[0]!, inputs[1]!])));
    else {
      console.error("usage: bun 📜️script.ts <pdf-encryption-project|pdf-encryption-compare> --input <path> [--input <path>]");
      process.exit(2);
    }
  } catch (error) {
    process.exit(emit(probe || "(none)", "failed", {}, String((error as Error).message ?? error)));
  }
}
