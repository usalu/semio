/** 🔺️ Jack sparse durable delta with whole child-handle replacement. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseCamera, parseManifest, type Camera, type Manifest } from "../🟦️.ts";

export interface JackDiff {
  /** @state artifact */ schema: string | null;
  /** @state artifact */ name: string | null;
  /** @state artifact */ manifestId: string | null;
  /** @state artifact */ manifest: Manifest | null;
  /** @state artifact */ camera: Camera | null;
  /** @state artifact @child kind=s.stdio.semio.graph */ content: ArtifactChild | null;
  /** @state artifact */ rootNodeId: string | null;
}

const nullableString = (value: unknown, at: string): string | null => {
  if (value === null || typeof value === "string") return value;
  throw new Error(`${at}: value must be a string or null`);
};

/** 🪪️ Parses the exact native JackDiff carrier and refuses legacy nodes/edges deltas. */
export function parseJackDiff(value: unknown, at = "$"): JackDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: diff must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match JackDiff`);
  return {
    schema: nullableString(row.schema, `${at}.schema`),
    name: nullableString(row.name, `${at}.name`),
    manifestId: nullableString(row.manifestId, `${at}.manifestId`),
    manifest: row.manifest === null ? null : parseManifest(row.manifest, `${at}.manifest`),
    camera: row.camera === null ? null : parseCamera(row.camera, `${at}.camera`),
    content: row.content === null ? null : parseArtifactChild(row.content),
    rootNodeId: nullableString(row.rootNodeId, `${at}.rootNodeId`),
  };
}
