import { interactivityIsRuntimeSource } from "../../../../../../../../📜️script.ts";

/** 🧪️Proves runtime discovery admits authored product roots and rejects repo-local scratch/archive roots. */
export function interactivityRuntimeSourceSelfTests(): void {
  if (!interactivityIsRuntimeSource("🧰️framework/🔨️modules/runtime/🦀️.rs")) throw new Error("[verify interactivity] authored framework runtime source was falsely excluded.");
  if (!interactivityIsRuntimeSource("✏️s/🔌️plugins/example/🦀️.rs")) throw new Error("[verify interactivity] authored plugin runtime source was falsely excluded.");
  for (const relPath of ["temp/compose/hostile/🦀️.rs", "compose/hostile/🦀️.rs", "♻️mit-bestand/hostile/🦀️.rs", ".🧬semio/hostile/🦀️.rs"])
    if (interactivityIsRuntimeSource(relPath)) throw new Error(`[verify interactivity] non-production source ${relPath} was falsely admitted.`);
}
