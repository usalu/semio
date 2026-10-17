/** 🧾️ SDK entry `@semio-tech/framework/app-channel`: explicit public surface of the framework module that plugins build on. */
export { AppChannelClient } from "../../../🔨️modules/🧾️app-channel/🟦️.ts";
export { decodeAppCommand, encodeAppCommand } from "../../../🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts";
export { decodeAppFrame, encodeAppFrame } from "../../../🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts";
export { decodeConflictsFromWire, decodeDispatchReportFromWire, decodeFaultFromWire, decodeMergeReportFromWire, faultDisplayMessage, faultMessages } from "../../../🔨️modules/🧾️app-channel/📋️report-decoders/🟦️.ts";
export { APP_CHANNEL_VERSION, AppChannelRequestSequence, CHANNEL_MISMATCH_CODE, admitGuestChannelVersion } from "../../../🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts";
export { DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, DocumentArchiveFaultError, DocumentArchiveLoadHost } from "../../../🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";
export type { AppChannelOperationProgressV1 } from "../../../🔨️modules/🧾️app-channel/📣️unsolicited-feed/🟦️.ts";
export type { AppChannelHandle } from "../../../🔨️modules/🧾️app-channel/🔢️request-sequence/🟦️.ts";
export type { DocumentArchiveLoadAnswer, DocumentArchiveLoadOutcome, DocumentArchiveLoadStep, DocumentArchiveMergeV1 } from "../../../🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";
