import { readFileSync } from "node:fs";
import { join } from "node:path";

export type OrderedAppendBroadcastFixture = {
  readonly schema: "semio.hub.directory.ordered-append-broadcast/v1";
  readonly maximumEventsPerDecision: 2;
  readonly cases: readonly {
    readonly id: string;
    readonly persistedSequences: readonly number[];
    readonly appendSucceeds: boolean;
    readonly expectedBroadcastSequences: readonly number[];
  }[];
};

/** 📣️ Validates the neutral append/broadcast law and the exact single-writer production seam. */
export function orderedDirectoryPublicationOracle(repoRoot: string): number {
  const base = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8")) as OrderedAppendBroadcastFixture;
  const caseIds = ["concurrent-single-events", "paired-user-member-events", "append-failure", "empty-idempotent-decision"] as const;
  if (fixture.schema !== "semio.hub.directory.ordered-append-broadcast/v1" || fixture.maximumEventsPerDecision !== 2) throw new Error("ordered directory publication fixture envelope drift");
  if (fixture.cases.length !== caseIds.length || caseIds.some((id, index) => fixture.cases[index]?.id !== id)) throw new Error("ordered directory publication fixture case inventory drift");
  if (new Set(fixture.cases.map((row) => row.id)).size !== fixture.cases.length) throw new Error("ordered directory publication fixture has duplicate cases");
  for (const row of fixture.cases) {
    const sequences = [...row.persistedSequences, ...row.expectedBroadcastSequences];
    if (typeof row.appendSucceeds !== "boolean" || sequences.some((value) => !Number.isSafeInteger(value) || value < 1)) throw new Error(`ordered directory publication fixture row is not a bounded decision: ${row.id}`);
    const expected = row.appendSucceeds ? row.persistedSequences : [];
    if (JSON.stringify(expected) !== JSON.stringify(row.expectedBroadcastSequences)) throw new Error(`ordered directory publication oracle differs for ${row.id}`);
    if (row.persistedSequences.length > fixture.maximumEventsPerDecision) throw new Error(`ordered directory publication fixture exceeds its decision bound for ${row.id}`);
  }
  const source = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🦀️.rs"), "utf8");
  const body = (text: string, name: string): string => {
    const signature = text.indexOf(`fn ${name}(`);
    if (signature < 0) return "";
    const start = text.indexOf("{", signature);
    let depth = 0;
    for (let index = start; index < text.length; index += 1) {
      if (text[index] === "{") depth += 1;
      else if (text[index] === "}" && --depth === 0) return text.slice(start + 1, index);
    }
    return "";
  };
  const exact = (text: string): boolean => {
    const append = body(text, "append_and_publish_locked");
    const publish = body(text, "publish_persisted_locked");
    const common = ["execute", "execute_create_space_with_id", "execute_artifact_authority"].map((name) => body(text, name));
    const checkpoint = body(text, "publish_reserved_artifact_checkpoint");
    const invite = body(text, "redeem_invite");
    const idempotent = body(text, "execute_idempotent");
    const ordered = (method: string, steps: readonly string[]): boolean => {
      let previous = -1;
      return (
        !method.includes("drop(clock)") &&
        steps.every((step) => {
          const index = method.indexOf(step);
          if (index <= previous) return false;
          previous = index;
          return true;
        })
      );
    };
    return (
      append.indexOf("self.dir.append_events(events).await?") >= 0 &&
      append.indexOf("self.dir.append_events(events).await?") < append.indexOf("self.publish_persisted_locked(clock, persisted)") &&
      publish.includes("for event in &persisted") &&
      publish.includes("self.tx.send(DirectoryStreamMessage::Event { event: Box::new(event.clone()) })") &&
      common.every((method) => method.includes("self.append_and_publish_locked(&clock,") && !method.includes("drop(clock)")) &&
      ordered(invite, ["self.write.lock().await", "self.dir.redeem_invite_atomic(", "InviteRedemptionCommit::NewlyCommitted", "self.publish_persisted_locked(&clock, persisted)"]) &&
      ordered(idempotent, ["self.write.lock().await", "claim_or_read_directory_command_receipt(", "append_decided_events(", "complete_directory_command_receipt(", "self.publish_persisted_locked(&clock, persisted)"]) &&
      checkpoint.includes("self.publish_persisted_locked(&clock, persisted)") &&
      !checkpoint.includes("drop(clock)")
    );
  };
  if (!exact(source)) throw new Error("directory append and broadcast do not share one writer-guard lifetime");
  const hostiles = [
    source.replace("self.append_and_publish_locked(&clock, &decision.events).await?", "drop(clock); self.dir.append_events(&decision.events).await?"),
    source.replace("self.publish_persisted_locked(clock, persisted)", "drop(clock); persisted"),
    source.replace("self.publish_persisted_locked(&clock, persisted)", "drop(clock); persisted"),
    source.replace("let committed = self.dir.redeem_invite_atomic(", "drop(clock); let committed = self.dir.redeem_invite_atomic("),
    source.replace("let persisted = match self.dir.append_decided_events(", "drop(clock); let persisted = match self.dir.append_decided_events("),
  ];
  for (const hostile of hostiles) if (exact(hostile)) throw new Error("directory ordered-publication oracle accepted an unlocked append or fanout");
  return fixture.cases.length + hostiles.length;
}
