/** 🔌️ The owned interface to external programs: the export commands reach `ffmpeg` (video), `rsvg-convert` (rasterising) and `staffage-dwg` only through {@link ToolPort}, so a test or a host swaps them and the absence of one is a named, bilingual {@link ToolMissingError}. No program is a runtime dependency of the library.
 *
 * @see ../🎬️cli/🟦️.ts — the export command
 * @see ../../🎨️depiction/🖨️preview/🟦️.ts — the preview command
 * @see https://ffmpeg.org/ffmpeg.html · https://gitlab.gnome.org/GNOME/librsvg */
import { spawn } from "node:child_process";
import type { Text } from "../../../🧬️schema/🟦️.ts";

/** 🧰️ The tool-independent interface to external programs: runs `tool` with `args`, resolves with its output, rejects with the signal's reason when `signal` aborts (the child is killed) and with a {@link ToolMissingError} when the program is not there. */
export type ToolPort = { readonly run: (tool: string, args: readonly string[], signal?: AbortSignal) => Promise<string> };

const REPO_TOOLS = new Set(["ffmpeg"]);

const HINTS: Readonly<Record<string, Text>> = {
  ffmpeg: { en: "Install it with the repo tool registry (`repo tools exec ffmpeg`), SEMIO_TOOL_FFMPEG or a system package (brew, apt, winget).", de: "Installieren Sie es über die Werkzeugverwaltung des Repos (`repo tools exec ffmpeg`), SEMIO_TOOL_FFMPEG oder ein Systempaket (brew, apt, winget)." },
  "rsvg-convert": { en: "Install librsvg (brew install librsvg, apt install librsvg2-bin) so that `rsvg-convert` is on the PATH.", de: "Installieren Sie librsvg (brew install librsvg, apt install librsvg2-bin), sodass `rsvg-convert` im PATH liegt." },
  "staffage-dwg": { en: "Build the staffage-dwg crate (🖊️dwg/📦️packages/🦀️rust) and put the binary on the PATH.", de: "Bauen Sie das Crate staffage-dwg (🖊️dwg/📦️packages/🦀️rust) und legen Sie die Binärdatei in den PATH." },
};

/** 🚫️ Thrown when an external program is not there: names the tool, says what to do in English and German and keeps the cause. The export it stopped has written nothing of that file. */
export class ToolMissingError extends Error {
  readonly tool: string;

  constructor(tool: string, readonly detail: string = "") {
    super(`required tool ${tool} not found${detail === "" ? "" : `: ${detail}`}`);
    this.name = "ToolMissingError";
    this.tool = tool;
  }

  /** 🌍️ The refusal in both languages. */
  get reason(): Text {
    const hint = HINTS[this.tool] ?? { en: "Make it available on the PATH.", de: "Machen Sie es im PATH verfügbar." };
    return { en: `The program "${this.tool}" is not available. ${hint.en}`, de: `Das Programm "${this.tool}" ist nicht verfügbar. ${hint.de}` };
  }
}

const ABSENT = /not found|no such file|nicht gefunden|could not be found|cannot find|is not recognized/iu;

/** 🛠️ Runs registered tools through `repo tools exec <tool> -- args` and system tools (`rsvg-convert`, `staffage-dwg`) from the PATH; an absent program rejects with {@link ToolMissingError}, a failing one with its exit code and output. */
export const repoToolPort: ToolPort = {
  run(tool, args, signal) {
    const [command, argv] = REPO_TOOLS.has(tool) ? ["repo", ["tools", "exec", tool, "--", ...args]] : [tool, [...args]];
    return new Promise((done, fail) => {
      const child = spawn(command!, argv as string[], { stdio: ["ignore", "pipe", "pipe"], ...(signal ? { signal } : {}) });
      let output = "";
      child.stdout.on("data", (chunk: Buffer) => (output += chunk.toString("utf8")));
      child.stderr.on("data", (chunk: Buffer) => (output += chunk.toString("utf8")));
      child.on("error", (error) => fail((error as NodeJS.ErrnoException).code === "ENOENT" ? new ToolMissingError(tool, command === tool ? `${command} is not on the PATH` : `${command} is not on the PATH`) : error));
      child.on("close", (code) => (code === 0 ? done(output) : fail(REPO_TOOLS.has(tool) && ABSENT.test(output) ? new ToolMissingError(tool, output.trim().split("\n")[0] ?? "") : new Error(`${tool} exited ${code}\n${output}`))));
    });
  },
};
