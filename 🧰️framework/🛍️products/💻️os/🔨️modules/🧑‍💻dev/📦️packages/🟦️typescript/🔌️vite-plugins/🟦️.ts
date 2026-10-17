/** 🔌️ Public entry `@semio-tech/framework-os-dev/vite-plugins`: the explicit surface that importers outside the module use. */
export { descriptorRouteDecision, semioDescriptorRouteGuardVitePlugin, semioLocalHubSessionVitePlugin, semioServiceWorkerScopeVitePlugin } from "../../../🔌️vite-plugins/🟦️.ts";
export { REPO_ROOT } from "../../../🔌️vite-plugins/🧭️repo-root/🟦️.ts";
export { CANONICAL_BOOTSTRAP_FOLDER_MIRROR_MAX_BYTES, CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH, publishCanonicalBootstrapFolderMirror, reserveCanonicalBootstrapFolderMirror, retireCanonicalBootstrapFolderMirror, stageCanonicalBootstrapFolderMirror } from "../../../🔌️vite-plugins/📇️bootstrap-folder-mirror/🟦️.ts";
export { backboneDbHandleFor, readBackbonePayload, writeBackbonePayload } from "../../../🔌️vite-plugins/🗄️backbone-storage/🟦️.ts";
export { devStreamMuxServer } from "../../../🔌️vite-plugins/🔀️stream-mux-server/🟦️.ts";
export { semioProductionTestBoundaryVitePlugin } from "../../../🔌️vite-plugins/🚧️production-test-boundary/🟦️.ts";
export { semioBackboneVitePlugin } from "../../../🔌️vite-plugins/🧱️backbone-vite-plugin/🟦️.ts";
export { reportActivationFreshness, semioActivationVitePlugin } from "../../../🔌️vite-plugins/🚀️activation-vite-plugin/🟦️.ts";
export { semioBlobVitePlugin } from "../../../🔌️vite-plugins/🫧️blob-vite-plugin/🟦️.ts";
export { UNWATCHED_REPOSITORY_SEGMENTS, createSourceFreshnessRegistry, repositorySourceWatchRoots, requestedTransformFile, semioPlaygroundReactRefreshCoherenceVitePlugin, semioSourceFreshnessVitePlugins, semioSourceWatchVitePlugin, semioTransformFreshnessVitePlugin, unwatchedRepositoryPathMatcher } from "../../../🔌️vite-plugins/🔖️source-freshness/🟦️.ts";
export { AGENT_BRIDGE_RENDEZVOUS_DIR_ENV, liveAgentBridgeOfferFor, semioAgentBridgeRendezvousVitePlugin } from "../../../🔌️vite-plugins/🛰️agent-bridge-rendezvous/🟦️.ts";
export { AGENT_CREDENTIALS_DIR_ENV, semioAgentCredentialInstallVitePlugin } from "../../../🔌️vite-plugins/🔐️agent-credential-install/🟦️.ts";
export type { DescriptorRouteDecision, DescriptorRouteGuardSpec } from "../../../🔌️vite-plugins/🟦️.ts";
export type { CanonicalBootstrapFolderMirrorFrontierV1, CanonicalBootstrapFolderMirrorOwnerV1, CanonicalBootstrapFolderMirrorReserveV1 } from "../../../🔌️vite-plugins/📇️bootstrap-folder-mirror/🟦️.ts";
export type { ActivationComponentSpec } from "../../../🔌️vite-plugins/🚀️activation-vite-plugin/🟦️.ts";
export type { SourceFreshnessRegistry, SourceStamp } from "../../../🔌️vite-plugins/🔖️source-freshness/🟦️.ts";
