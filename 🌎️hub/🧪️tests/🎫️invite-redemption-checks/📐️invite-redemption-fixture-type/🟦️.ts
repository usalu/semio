/** 📐️ The shared invite redemption fixture type. */
export type InviteRedemptionFixture = {
  readonly schema: "semio.hub.invite-redemption-transaction/v1";
  readonly limits: { readonly credentialBytes: 4096; readonly identifierBytes: 4096; readonly maximumContenders: 3 };
  readonly receiptRelease: readonly { readonly name: string; readonly outcome: "rejected-before-commit" | "appended" | "uncertain"; readonly disposition: "pending" | "completed"; readonly digest: "exact" | "different"; readonly release: boolean }[];
  readonly backendOrders: readonly { readonly name: string; readonly first: "archive" | "member"; readonly between: "invite" | "archive"; readonly accepted: boolean; readonly appended: number }[];
  readonly authorityRaces: readonly { readonly name: string; readonly revocation: "session" | "archive" | "delete"; readonly order: readonly ("hint" | "revoke" | "fence")[]; readonly status: number; readonly appended: number }[];
  readonly spaceStates: readonly {
    readonly name: string;
    readonly spaceState: "writable" | "archived" | "deleted";
    readonly inviteRole: "author" | "spectator";
    readonly accepted: "none" | "same" | "other";
    readonly status: number;
    readonly appended: number;
    readonly membershipRole: "none" | "author" | "spectator";
  }[];
  readonly vectors: readonly {
    readonly name: string;
    readonly initial: "fresh" | "accepted-same" | "accepted-other" | "revoked" | "expired" | "missing" | "missing-user" | "missing-space" | "corrupt-marker" | "corrupt-event";
    readonly concurrent: boolean;
    readonly calls: readonly {
      readonly kind: "redeem" | "revoke" | "append-forged" | "restart" | "rebuild";
      readonly user: "same" | "other" | "missing";
      readonly actor: "exact" | "different";
      readonly credential: "exact" | "wrong-selector" | "wrong-secret";
      readonly failure: "none" | "after-marker" | "after-event" | "after-projection";
    }[];
    readonly expected: {
      readonly outcomes: readonly string[];
      readonly returnedEventIds: readonly (string | null)[];
      readonly marker: { readonly acceptedAt: number | null; readonly acceptedEventId: string | null };
      readonly events: number;
      readonly memberships: number;
      readonly publications: number;
      readonly replayEvents: number;
      readonly revoked: boolean;
    };
  }[];
  readonly hostiles: readonly { readonly name: string; readonly mutation: "raw-capability" | "client-space" | "client-space-state" | "client-role" | "client-event-id" | "unknown-field" | "oversized-identifier" }[];
};
