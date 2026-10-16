/** 🪪️ The identity config facet: the folder lane binding, sign-in and sign-out mutations and the last-envelope-wins fold. */
import type { BackboneWorkerTestDependencies } from "../../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type ArtifactEvent = import("../../../🟦️.ts").ArtifactEvent;
type Identity = import("../../🧬️schema/🧬️mutations/🪪️sign-in/🟦️.ts").Identity;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerIdentityConfigFacetTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { identityActorConfig, IDENTITY_CONFIG_SCHEMA, foldIdentityEvent } = dependencies;
  //#region 🔖️IdentityTests
  describe("identity config facet", () => {
    beforeEach(resetWorkerLaw);
    function sampleIdentity(overrides: Partial<Identity> = {}): Identity {
      return { userId: "u-1", email: "ada@semio.dev", displayName: "Ada", hubBaseUrl: "http://hub.test", issuedAtMs: 1_000, ...overrides };
    }

    it("identityActorConfig binds the folder lane under `${dataDir}/os` when given a dataDir, else local-only", () => {
      const actorIdentityGrant = { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 };
      expect(identityActorConfig("actor-1", actorIdentityGrant, "/tmp/s-user1")).toEqual({
        documentId: IDENTITY_CONFIG_SCHEMA,
        schema: IDENTITY_CONFIG_SCHEMA,
        bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/s-user1/os" }],
        actor: "actor-1",
        actorIdentityGrant,
      });
      expect(identityActorConfig("actor-1", actorIdentityGrant)).toEqual({ documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings: [], actor: "actor-1", actorIdentityGrant });
    });

    it("sign-in -> sign-out -> sign-in round-trips through applyIdentityConfigMutation, and each inverts the last", async () => {
      const { applyIdentityConfigMutation, inverseIdentityConfigMutation, signIn, signOut } = await import("../../🧬️schema/🧬️mutations/🟦️");

      const first = sampleIdentity();
      const afterFirstSignIn = applyIdentityConfigMutation(null, signIn(first));
      expect(afterFirstSignIn).toEqual(first);

      const afterSignOut = applyIdentityConfigMutation(afterFirstSignIn, signOut());
      expect(afterSignOut).toBeNull();

      const second = sampleIdentity({ userId: "u-2", email: "devon@semio.dev", displayName: "Devon", issuedAtMs: 2_000 });
      const afterSecondSignIn = applyIdentityConfigMutation(afterSignOut, signIn(second));
      expect(afterSecondSignIn).toEqual(second);

      // ↩️ sign-out's inverse, from the base it cleared, restores exactly the prior session.
      expect(inverseIdentityConfigMutation(signOut(), afterFirstSignIn)).toEqual([signIn(first)]);
      // ↩️ sign-out's inverse with no prior session is a no-op.
      expect(inverseIdentityConfigMutation(signOut(), null)).toEqual([]);
      // ↩️ sign-in's inverse, from no prior session, is a sign-out.
      expect(inverseIdentityConfigMutation(signIn(first), null)).toEqual([signOut()]);
      // ↩️ sign-in's inverse, from a prior session (switching accounts), restores the prior one.
      expect(inverseIdentityConfigMutation(signIn(second), afterFirstSignIn)).toEqual([signIn(first)]);
    });

    it("foldIdentityEvent folds sign-in -> sign-out -> sign-in as last-envelope-wins, ignoring non-remoteMutations events", () => {
      const first = sampleIdentity();
      const second = sampleIdentity({ userId: "u-2" });
      const decodePayload = (payload: unknown): Identity | null | undefined => {
        if (payload === null) return null;
        if (typeof payload === "object" && payload !== null && "userId" in payload) return payload as Identity;
        return undefined;
      };
      const envelope = (payload: unknown): ArtifactEvent => ({
        kind: "remoteMutations",
        envelopes: [
          {
            id: "e",
            actor: "a",
            document: IDENTITY_CONFIG_SCHEMA,
            schemaVersion: IDENTITY_CONFIG_SCHEMA,
            payloadHash: "",
            diff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload },
            inverse: { targetOperation: "e", inverseDiff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: null }, baseVersion: 0, undoPolicy: "exactBaseOnly" },
          },
        ],
      });

      let state: Identity | null = null;
      state = foldIdentityEvent(state, envelope(first), decodePayload);
      expect(state).toEqual(first);
      state = foldIdentityEvent(state, envelope(null), decodePayload);
      expect(state).toBeNull();
      state = foldIdentityEvent(state, envelope(second), decodePayload);
      expect(state).toEqual(second);
      // 🚧️ A non-`remoteMutations` event (e.g. `status`) passes state through unchanged.
      state = foldIdentityEvent(state, { kind: "status", persisted: true, pendingMutations: 0, remote: { kind: "detached" } }, decodePayload);
      expect(state).toEqual(second);
    });
  });
  //#endregion 🔖️IdentityTests
}
