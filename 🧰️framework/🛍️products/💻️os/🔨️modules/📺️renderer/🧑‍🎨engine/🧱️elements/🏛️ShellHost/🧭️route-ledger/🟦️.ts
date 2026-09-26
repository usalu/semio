/** 🧭️ Which shell route is in effect, and the one rule the route effect follows: a URI whose application took effect is
 * never applied again by itself. A hub document opened from a space keeps the space's URI (`/spaces/<id>`), and
 * re-applying that URI — whenever a hub program load or a session change re-fired the route effect — switched the human
 * back to the space's index: measured as a note closing ~1.4 s after it mounted (🎫️ 26/09/23 C10). Only a replaced
 * session or an explicit navigation {@link ShellRouteLedgerV1.invalidate}s the record; an application that was running when
 * that happened records nothing, so the next route effect applies the URI to the replacing session. */
export type ShellRouteLedgerV1 = {
  readonly shouldApply: (uri: string) => boolean;
  readonly begin: () => number;
  readonly settle: (generation: number, uri: string, tookEffect: boolean) => void;
  readonly invalidate: () => void;
  readonly applied: () => string | null;
};

export function createShellRouteLedgerV1(): ShellRouteLedgerV1 {
  let applied: string | null = null;
  let generation = 0;
  return {
    shouldApply: (uri) => uri !== applied,
    begin: () => generation,
    settle: (started, uri, tookEffect) => {
      if (tookEffect && started === generation) applied = uri;
    },
    invalidate: () => {
      generation += 1;
      applied = null;
    },
    applied: () => applied,
  };
}
