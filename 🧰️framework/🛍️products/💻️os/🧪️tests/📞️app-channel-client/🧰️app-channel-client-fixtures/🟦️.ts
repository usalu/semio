/** 🧰️ The shared app channel client test fixtures: the command sequence reader, the fake and scripted channel handles and the settle helper. */
import { decodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { type AppChannelHandle, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../../🛠️backbone-wire-helpers/🟦️.ts";

export const commandSeq = (command: AppCommandValue): number => {
  const sequence = Object.values(command)[0]!.seq;
  if (typeof sequence === "bigint" && (sequence < 0n || sequence > BigInt(Number.MAX_SAFE_INTEGER))) throw new RangeError("Fixture receipt exceeds the exact Done sequence range");
  return Number(sequence);
};

export type Conflict = import("@semio-tech/framework-kernel/merge-outcome").Conflict;

export type DispatchReport = import("@semio-tech/framework-kernel/merge-outcome").DispatchReport;

export type LocalInteractionIdentity = import("@semio-tech/framework-replication/wire/local-interaction").LocalInteractionIdentity;

export type LocalInteractionQueryReply = import("@semio-tech/framework-replication/wire/local-interaction/transport").LocalInteractionQueryReply;

export type LocalInteractionQueryToken = import("@semio-tech/framework-replication/wire/local-interaction").LocalInteractionQueryToken;

export type MergeReport = import("@semio-tech/framework-kernel/merge-outcome").MergeReport;

/** 🧪️ A fake handle that decodes whatever {@link AppChannelClient} `enqueue`d and pushes
 * caller-supplied frames back as this SAME instance's next outcome — enough to assert the client
 * frames/unframes correctly (and correlates replies through the real `outcomes` broadcast) without
 * a real plugin instance. */
export function fakeHandle(reply: (instanceId: number, commands: AppCommandValue[]) => AppFrameValue[]): AppChannelHandle {
  const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
  return {
    enqueue: (instanceId, events) => {
      const commands = events.map(decodeAppCommand);
      const frames = reply(instanceId, commands).map(encodeAppFrame);
      broadcast.push({ instanceId, frames });
    },
    outcomes: broadcast.stream,
  };
}
