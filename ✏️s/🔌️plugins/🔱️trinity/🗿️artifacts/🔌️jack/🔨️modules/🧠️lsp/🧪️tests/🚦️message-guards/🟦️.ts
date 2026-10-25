import { describe, expect, test } from "vitest";
import vector from "./🔣️.json" with { type: "json" };
import { isJsonRpcNotification, isJsonRpcRequest, isJsonRpcResponse } from "../../🔖️protocol/🟦️.ts";

describe("jack lsp message guards", () => {
  for (const row of vector.cases) {
    test(row.name, () => {
      expect(isJsonRpcRequest(row.message)).toBe(row.request);
      expect(isJsonRpcNotification(row.message)).toBe(row.notification);
      expect(isJsonRpcResponse(row.message)).toBe(row.response);
    });
  }
});
