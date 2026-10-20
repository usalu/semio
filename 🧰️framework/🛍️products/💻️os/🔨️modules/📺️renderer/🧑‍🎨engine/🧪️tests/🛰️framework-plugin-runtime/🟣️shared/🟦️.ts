/** 🟣️ The shared declarations of the framework plugin runtime suite tests. */
import { createTurnOutcomeBroadcast, type TurnOutcome } from "@semio-tech/framework";

export function exchangeStyleChannel(respond: (instanceId: number, frames: Uint8Array[]) => Uint8Array[] | Promise<Uint8Array[]>): {
  readonly enqueue: (instanceId: number, events: readonly Uint8Array[]) => void;
  readonly outcomes: AsyncIterable<TurnOutcome>;
} {
  const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
  return {
    enqueue: (instanceId, events) => {
      void (async () => {
        try {
          const frames = await respond(instanceId, [...events]);
          broadcast.push({ instanceId, frames });
        } catch (error) {
          broadcast.push({ instanceId, error });
        }
      })();
    },
    outcomes: broadcast.stream,
  };
}
