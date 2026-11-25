import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🔣️.json" with { type: "json" };

const policy = schema.definitions.MissingModel.const;

/** 🕳️ The plugin whose retained component model the native child reported missing. */
export type MissingMcpModelV1 = Readonly<{ pluginId: string; cratePath: string }>;

/** 🔎️ Reads the typed missing-model refusal from a relayed response: a tool error (`structuredContent`) or a JSON-RPC error (`data`), never prose. */
export function readMissingMcpModelV1(frame: Readonly<{ result?: unknown; error?: unknown }>): MissingMcpModelV1 | undefined {
  const result = frame.result as { isError?: unknown; structuredContent?: { code?: unknown; details?: unknown } } | null | undefined;
  const error = frame.error as { data?: { gatewayCode?: unknown; details?: unknown } } | null | undefined;
  const source = result?.isError === true ? { code: result.structuredContent?.code, details: result.structuredContent?.details } : { code: error?.data?.gatewayCode, details: error?.data?.details };
  if (source.code !== policy.errorCode || source.details === undefined || validateJsonSchemaSubset(schema.definitions.MissingModelDetails, source.details).length) return undefined;
  const { pluginId, cratePath } = (source.details as Record<string, MissingMcpModelV1>)[policy.detailsKey]!;
  return { pluginId, cratePath };
}
