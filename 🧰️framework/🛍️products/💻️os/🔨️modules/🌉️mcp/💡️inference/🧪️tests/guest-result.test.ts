import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import cases from "./🧫️fixtures/guest-result.json";

test("an inference reply must match its request and complete before success", () => {
  const database = new Database(":memory:");
  try {
    const project = database.query<{ outcome: string; payload: string | null; payloadBytes: number }, [string, string, number, string | null]>(`
      SELECT
        CASE WHEN ?1 <> ?2 THEN 'WRONG_SCHEMA' WHEN ?3 THEN 'SUCCEEDED' ELSE 'FAILED' END AS outcome,
        CASE WHEN ?1 = ?2 AND json_valid(?4) THEN json(?4) ELSE NULL END AS payload,
        CASE WHEN ?1 = ?2 THEN coalesce(length(CAST(?4 AS BLOB)), 0) ELSE 0 END AS payloadBytes
    `);
    for (const row of cases) {
      const projected = project.get(row.requested, row.returned, Number(row.complete), row.wire)!;
      expect(projected.outcome).toBe(row.outcome);
      expect(projected.payload === null ? null : JSON.parse(projected.payload)).toEqual(row.payload);
      expect(projected.payloadBytes).toBe(row.payloadBytes);
    }
  } finally {
    database.close();
  }
});
