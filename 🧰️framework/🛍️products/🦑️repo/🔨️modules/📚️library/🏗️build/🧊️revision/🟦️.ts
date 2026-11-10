/** 🧊️ MVCC revisions of staged inputs: pin a revision of exactly a build's input closure, read frozen content, key results by content. */
export {REVISION_MESSAGES_V1,REVISION_POLICY_V1,isRevisionDigestV1,isSafeRevisionPathV1,revisionErrorV1,revisionTextV1} from "./🧬️schema/🟦️.ts";
export type {CasRefV1,DigestV1,EntryModeV1,InputManifestV1,ManifestEntryV1,PinV1,RevisionBodyV1,RevisionErrorCodeV1,RevisionErrorV1,RevisionPolicyV1,RevisionV1,StageReplyV1,StageRequestV1,TreeEntryV1,TreeRefV1} from "./🧬️schema/🟦️.ts";
export {actionKeyV1,buildTreeV1,canonicalBytesV1,canonicalJsonV1,compareCodePointsV1,flattenTreeV1,manifestOfV1,parseRevisionV1,revisionDigestV1,sealRevisionV1} from "./🌳️tree/🟦️.ts";
export {loadManifestV1,loadRevisionV1,pinRevisionV1,releasePinV1,sealEntriesV1,renewPinV1,stageRevisionV1,withPinnedRevisionV1} from "./📌️pin/🟦️.ts";
export type {PinnedRevisionV1,PinRevisionOptionsV1,RevisionIoV1,RevisionProgressV1} from "./📌️pin/🟦️.ts";
export {materializeRevisionV1,openPinnedWorldSyncV1,openPinnedWorldV1} from "./🔓️view/🟦️.ts";
export type {PinnedWorldSyncV1,PinnedWorldV1} from "./🔓️view/🟦️.ts";
export {createRevisionCaptureHookV1,createRevisionPinHookV1} from "./🔌️plan/🟦️.ts";
export type {RevisionPinAnswerV1,RevisionPinRequestV1} from "./🔌️plan/🟦️.ts";
export {CAPTURE_POLICY_V1,createCapturingFsV1,createPinnedFsV1,pinModuleV1,withCapturedPlanV1} from "./🗂️fs/🟦️.ts";
export type {CapturedPlanOptionsV1,ReadFsV1} from "./🗂️fs/🟦️.ts";
