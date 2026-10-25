import { concurrentEditV1, retryOnConcurrentEditV1, type ChurnOptionsV1 } from "@semio-tech/framework-filesystem/observation/churn";

/** 🧾️ What one observation of the dependency direction scope must expose: the inventoried sources the resolver is expected to cover. */
export type DependencyDirectionObservationV1 = Readonly<{ expectedSources: readonly string[] }>;

const vanishedFileV1 = (error: unknown): Error | undefined => {
  const row = error as { code?: unknown; path?: unknown } | null;
  return row && typeof row === "object" && row.code === "ENOENT" ? concurrentEditV1("vanished", typeof row.path === "string" ? row.path : "inventory", undefined, "build") : undefined;
};

/** 🔁️ Runs one whole resolver pass on a fresh observation of the scope and takes a new observation when a peer moved a source under it: a source that vanished while the scope was walked, or a failed pass whose inventory differs from the inventory taken after the failure, is a concurrent edit (bounded by the `build` layer of the churn schema, announced in EN and DE); any other failure, including every forbidden edge, stands unchanged.
 * @see ../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json */
export async function observeDependencyDirectionV1<P extends DependencyDirectionObservationV1, T>(observe: () => P, run: (observation: P) => Promise<T>, options: ChurnOptionsV1 = {}): Promise<T> {
  const look = (): P => { try { return observe(); } catch (error) { throw vanishedFileV1(error) ?? error; } };
  return retryOnConcurrentEditV1("build", async () => {
    const observation = look();
    try { return await run(observation); } catch (error) {
      if (options.signal?.aborted) throw error;
      const now = look().expectedSources, before = new Set(observation.expectedSources), after = new Set(now);
      const gone = observation.expectedSources.find((source) => !after.has(source)), added = now.find((source) => !before.has(source));
      if (gone !== undefined) throw concurrentEditV1("vanished", gone, undefined, "build");
      if (added !== undefined) throw concurrentEditV1("appeared", added, undefined, "build");
      throw error;
    }
  }, options);
}
