import schema from "./🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

export type ChannelVersionConsumerV1 = Readonly<{
  path: string;
  occurrences: number;
  hostileValues?: readonly number[];
  hostileOccurrences?: number;
  arbitrary?: true;
  guest?: true;
  derived?: string;
}>;
export type ChannelVersionContributionOwnerV1 = Readonly<{
  ownerRoot: string;
  document: Readonly<{ schema: "semio.os.channel-version-consumers/v1"; consumers: readonly ChannelVersionConsumerV1[] }>;
}>;

/** 📍️Admits a bounded owner-relative contribution or consumer path. */
export function admitChannelVersionContributionPathV1(input: unknown): string {
  if (validateJsonSchemaSubset(schema.$defs.PathV1, input, schema).length || typeof input !== "string" || input.normalize("NFC") !== input) throw Error("Invalid channel contribution path");
  return input as string;
}

/** 📣️Admits bounded consumer declarations belonging to distinct present owners. */
export function admitChannelVersionContributionsV1(input: unknown): readonly ChannelVersionConsumerV1[] {
  const errors = validateJsonSchemaSubset(schema.$defs.OwnersV1, input, schema);
  if (errors.length) throw Error(`Invalid channel consumer contribution: ${errors.slice(0, 8).join("; ")}`);
  const owners = input as readonly ChannelVersionContributionOwnerV1[];
  if (new Set(owners.map(owner => admitChannelVersionContributionPathV1(owner.ownerRoot).toLowerCase())).size !== owners.length) throw Error("Duplicate channel contribution owner");
  const consumers = owners.flatMap(owner => owner.document.consumers.map(consumer => ({ ...consumer, path: `${owner.ownerRoot}/${admitChannelVersionContributionPathV1(consumer.path)}` })));
  if (new Set(consumers.map(consumer => consumer.path.toLowerCase())).size !== consumers.length) throw Error("Duplicate channel version consumer");
  return consumers.sort((left, right) => left.path.localeCompare(right.path));
}
