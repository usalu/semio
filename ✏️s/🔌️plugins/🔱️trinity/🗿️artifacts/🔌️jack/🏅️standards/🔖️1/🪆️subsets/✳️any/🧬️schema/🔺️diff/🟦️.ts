/** 🔺️ Jack sparse durable delta with whole child-handle replacement. */
import { parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";
import { parseCamera, type Camera } from "../🟦️.ts";

export interface JackDiff {
  /** 🔄️ @state artifact */ schema: string | null;
  /** ✏️ @state artifact */ name: string | null;
  /** 📜️ @state artifact */ manifestId: string | null;
  /** 📷️ @state artifact */ camera: Camera | null;
  /** 📄️ @state artifact @child capability=s.trinity.jack.graph */ content: ArtifactChild | null;
  /** 🌳️ @state artifact — a missing key leaves the id unchanged, `null` clears it */ rootNodeId?: string | null;
  /** 🔍️ @state artifact */ query: string | null;
}

const nullableString = (value: unknown, at: string): string | null => {
  if (value === null || typeof value === "string") return value;
  throw new Error(`${at}: value must be a string or null`);
};

/** 🪪️ Parses the exact native JackDiff carrier and refuses legacy nodes/edges deltas. */
export function parseJackDiff(value: unknown, at = "$"): JackDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: diff must be an object`);
  const row = value as Record<string, unknown>;
  const required = ["schema", "name", "manifestId", "camera", "content", "query"];
  const optional = ["rootNodeId"];
  if (Object.keys(row).some((key) => !required.includes(key) && !optional.includes(key)) || required.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match JackDiff`);
  return {
    schema: nullableString(row.schema, `${at}.schema`),
    name: nullableString(row.name, `${at}.name`),
    manifestId: nullableString(row.manifestId, `${at}.manifestId`),
    camera: row.camera === null ? null : parseCamera(row.camera, `${at}.camera`),
    content: row.content === null ? null : parseArtifactChild(row.content),
    rootNodeId: row.rootNodeId === undefined ? undefined : nullableString(row.rootNodeId, `${at}.rootNodeId`),
    query: nullableString(row.query, `${at}.query`),
  };
}
