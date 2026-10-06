/** 🔣️ Remodeling physical JSON diff codec. */
import {REMODELING_DIFF_SPEC, type RemodelingDiff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import {decodeRecord,writeRecordJson} from "../../📸️snapshot/🔣️json/🟦️.ts";


/** 🔺️ Decodes a parsed RFC 8259 value into a validated `RemodelingDiff`. */
export const decodeRemodelingDiff = (json: unknown): RemodelingDiff => decodeRecord(json, REMODELING_DIFF_SPEC, "") as unknown as RemodelingDiff;


/** 📄️ Encodes a diff as `serde_json::to_string_pretty` would render it. */
export const remodelingDiffToJsonText = (diff: RemodelingDiff): string => writeRecordJson(diff as unknown as Record<string, unknown>, REMODELING_DIFF_SPEC, 0);


/** 📄️ Encodes a diff into a plain JSON value. */
export const encodeRemodelingDiff = (diff: RemodelingDiff): unknown => JSON.parse(remodelingDiffToJsonText(diff));
