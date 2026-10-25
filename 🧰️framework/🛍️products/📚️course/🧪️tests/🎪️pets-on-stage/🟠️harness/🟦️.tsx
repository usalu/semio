/** 🟠️ The shared declarations of the 🐾️ pet companions suite tests. */
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { type Menagerie } from "@semio-tech/pets";
import type { Query } from "@semio-tech/course";
import { courseInstance, type PresenceConnect } from "@semio-tech/course-react";
import { schemaConformanceFixture as vectors } from "@semio-tech/pets/testing/fixtures/schema-conformance";
import { COURSE } from "../../🪛️pet-companions-kit/🟦️.tsx";

export const SAMPLE = vectors.menagerie as unknown as Menagerie;

export const TIMING = { minMs: 1, maxMs: 4 };

export const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });

export const encoder = new TextEncoder();

export const decoder = new TextDecoder();

export function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that knows the course and nothing else. */
export function courseProctor(): HttpTransport {
  return {
    send: async (request) => {
      if (request.method === "GET" && request.path === "/instance") return reply(200, courseInstance());
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
      const query = JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query;
      if (query.type !== "course") return reply(404, { kind: "notFound", message: query.type });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(COURSE)), frontier: null }));
    },
  };
}
