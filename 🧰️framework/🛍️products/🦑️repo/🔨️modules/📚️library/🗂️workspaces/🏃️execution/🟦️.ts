import { BundleScript } from "../../🏃️process/🧭️routing/🟦️.ts";
import { publishWorkspaceMembership } from "../📣️publication/🟦️.ts";

/** 🎛️ Admits exactly one workspace publication mode. */
export function parseWorkspacePublicationArguments(segments: readonly string[]): "check" | "write" {
  if (segments.length === 1 && segments[0] === "--check") return "check";
  if (segments.length === 1 && segments[0] === "--write") return "write";
  throw new Error("usage: bun ./📜️script.ts workspaces <--write|--check>");
}

/** 🚪️ Routes workspace publication without owning discovery or document behavior. */
export class WorkspacePublicationScript extends BundleScript {
  run(segments: string[]): void {
    publishWorkspaceMembership(this.repoRoot, parseWorkspacePublicationArguments(segments), {
      discovery: {
        onProgress: ({ directoriesScanned }) => {
          if (directoriesScanned % 256 === 0) console.log(`[workspaces] scanned ${directoriesScanned} directories`);
        },
      },
    });
  }
}
