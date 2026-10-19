/** 📖️ Source-text law reader of the frame Worker: the entry file and every module it was split into, as one text (the former single `frame-worker/🟦️.ts`). */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const WORKER_FILES = ["🟦️.ts", "🧬️bindings-types/🟦️.ts", "🧵️state/🟦️.ts", "⏱️steps-and-reporting/🟦️.ts", "📨️receive/🟦️.ts", "🎞️frame-and-introspection/🟦️.ts", "🧯️close-and-fault/🟦️.ts", "🔌️boot-and-plugins/🟦️.ts", "🖼️page-image-decode/🟦️.ts", "📦️assets-and-host-io/🟦️.ts"] as const;

/** 🎞️ Reads the frame Worker modules of one directory in a fixed order. */
export function readFrameWorkerDirectory(directory: string): string {
  return WORKER_FILES.map((file) => readFileSync(join(directory, file), "utf8")).join("\n");
}

/** 🎞️ Reads the frame Worker modules; `root` is the directory of the calling test, three levels below the engine folder. */
export function readFrameWorkerSource(root: string): string {
  return readFrameWorkerDirectory(join(root, "../../../🎯️targets/🧊️wgpu/🎞️frame-worker"));
}
