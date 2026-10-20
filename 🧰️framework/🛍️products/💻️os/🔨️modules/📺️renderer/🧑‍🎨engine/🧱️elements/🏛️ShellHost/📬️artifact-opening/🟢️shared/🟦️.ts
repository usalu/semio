/** 🟢️ The shared declarations of useArtifactOpening. */
/** 👁️✏️ `PluginRuntime`'s `PluginWasmHandle` wraps the raw `exchange` ABI behind typed methods —
 * `transactionPrepare`/`transactionCommit`/`transactionUndo`/`transactionRedo` and (as of ticket
 * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` lane K2)
 * `setMergePolicy`/`resolveConflict`/`readConflicts` are wrapped this way (`adaptPluginHandle`,
 * `PluginRuntime/🟦️.tsx`), each internally riding its own `AppChannelClient` — see
 * {@link pluginHandleFor} below. `openArtifact`/`setDefaultApp`/`clearDefaultApp` (contract freeze
 * §3) are NOT wrapped yet — the raw `exchange` method itself isn't re-exposed for them, so this
 * lease cannot construct its own `AppChannelClient` from the handle it's given. Feature-detected
 * here (structurally optional, not a type-lying cast) so this activates the moment `PluginRuntime`
 * adds the same three methods the transaction/merge families already have, with zero changes on
 * this side. See `📓️w1-c-report.md` "NOT done" — until then these three are local-only (the
 * opening-prefs fold below and the session switch in `openArtifactWithAppRef` both still work;
 * only the wire notify is inert). */
export type PendingAppChannelMethods = {
  readonly openArtifact?: (artifactRef: string, role: number, pluginId?: string, appId?: string) => Promise<unknown>;
  readonly setDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string) => Promise<unknown>;
  readonly clearDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number) => Promise<unknown>;
};
