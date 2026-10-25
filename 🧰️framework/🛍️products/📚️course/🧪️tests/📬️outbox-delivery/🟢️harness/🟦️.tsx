/** 🟢️ The shared declarations of the 📬️ answer outbox suite tests. */
import { decodeCommandEnvelope, encodeCommandOutcome, type CommandEnvelope, type CommandOutcome, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { type Command, type OpenTaskCommand, type RecordAnswerCommand } from "@semio-tech/course";
import { Outbox, ProctorUnavailable, localStore, memoryStorageOrigin, newId, courseInstance, type CommandVerdict, type StorageArea } from "@semio-tech/course-react";

export const TIMING = { minMs: 1, maxMs: 4 };

export const LEARNER = "a".repeat(32);

export const RUN = "b".repeat(32);

export const OTHER_RUN = "c".repeat(32);

export const encoder = new TextEncoder();

export function answer(task: string, order: readonly string[], run = RUN): RecordAnswerCommand {
  return { type: "record-answer", id: newId(), learner: LEARNER, run, task, answer: { kind: "sorting", order }, at: 1 };
}

export function opening(task: string, run = RUN): OpenTaskCommand {
  return { type: "open-task", id: newId(), learner: LEARNER, run, task, at: 1 };
}

export function gate<T>(): { readonly promise: Promise<T>; readonly open: (value: T) => void } {
  let open: (value: T) => void = () => undefined;
  const promise = new Promise<T>((resolve) => {
    open = resolve;
  });
  return { promise, open };
}

export function tick(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 10));
}

export function reply(status: number, body: unknown): HttpResponse {
  const text = JSON.stringify(body);
  return { status, text: async () => text, bytes: async () => encoder.encode(text) };
}

/** 🛂️ A proctor double that applies every command id at most once, like the framework's idempotency law. */
export function deduplicatingProctor(): { readonly transport: HttpTransport; readonly envelopes: CommandEnvelope[]; readonly applied: Map<string, CommandOutcome>; failNext(count: number): void; loseNextResponse(): void; rejectWith(detail: string): void } {
  const envelopes: CommandEnvelope[] = [];
  const applied = new Map<string, CommandOutcome>();
  let failures = 0;
  let lose = false;
  let rejection: string | undefined;
  const transport: HttpTransport = {
    async send(request) {
      if (request.method === "GET" && request.path === "/instance") return reply(200, courseInstance());
      const envelope = decodeCommandEnvelope(JSON.parse(String(request.body)));
      envelopes.push(envelope);
      if (failures > 0) {
        failures -= 1;
        return reply(503, { kind: "actorUnavailable", message: "busy" });
      }
      const command = JSON.parse(new TextDecoder().decode(envelope.payload)) as Command;
      const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: applied.size + 1, acceptedAt: envelope.clientHlc };
      const outcome: CommandOutcome =
        applied.get(envelope.idempotencyKey ?? envelope.commandId) ??
        (rejection === undefined
          ? {
              status: "accepted",
              receipt,
              events: [{ stream: envelope.target, seq: applied.size + 1, hlc: envelope.clientHlc, kind: `course.answer-recorded`, payload: encoder.encode(JSON.stringify({ ...command, type: "answer-recorded", at: 1 })) }],
              frontier: null,
            }
          : { status: "rejected", receipt, reason: { kind: "invalid", detail: rejection }, notices: [] });
      applied.set(envelope.idempotencyKey ?? envelope.commandId, outcome);
      if (lose) {
        lose = false;
        throw new ProctorUnavailable("connection reset after the proctor applied the command");
      }
      return reply(200, encodeCommandOutcome(outcome));
    },
  };
  return {
    transport,
    envelopes,
    applied,
    failNext: (count) => {
      failures = count;
    },
    loseNextResponse: () => {
      lose = true;
    },
    rejectWith: (detail) => {
      rejection = detail;
    },
  };
}

export function outboxOver(send: (command: Command, signal: AbortSignal) => Promise<CommandVerdict>, onSettled?: (command: Command, verdict: CommandVerdict) => void, area: StorageArea = memoryStorageOrigin().tab()): Outbox {
  return new Outbox({ send, store: localStore(area, "test"), timing: TIMING, onSettled });
}
