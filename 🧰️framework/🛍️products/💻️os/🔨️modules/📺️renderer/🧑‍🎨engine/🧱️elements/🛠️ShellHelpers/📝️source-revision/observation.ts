import type { LocalSourceRevisionV1, SourceRevisionIdentityV1, SourceRevisionObservationV1 } from "./🟦️.ts";

/** 📥️ Admits only the current original capture before publishing its interaction state. */
export async function readSourceRevisionV1<State>(request: Readonly<{
  source: LocalSourceRevisionV1;
  observation: SourceRevisionObservationV1;
  current(): boolean;
  capture(signal: AbortSignal): Promise<Readonly<{ identity: SourceRevisionIdentityV1; state: State }>>;
  accept(state: State): void;
}>): Promise<State | null> {
  const { source, observation } = request;
  if (observation.signal.aborted) return null;
  try {
    const capture = await request.capture(observation.signal);
    if (!request.current() || !source.publish(observation, capture.identity) || !source.isCurrent(observation) || !request.current()) return null;
    request.accept(capture.state);
    return capture.state;
  } catch (error) {
    if (observation.signal.aborted) return null;
    throw error;
  } finally {
    source.fail(observation);
  }
}
