//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { ListPromptsRequestSchema, ListResourcesRequestSchema, ListToolsRequestSchema } from "@modelcontextprotocol/sdk/types.js";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

type Surface = {
  tools: { name: string; descriptionKey: string }[];
  resources: { uri: string; name: string; descriptionKey: string }[];
  prompts: { name: string; descriptionKey: string }[];
  profiles: { slug: string; extraTicketArgument: string | null }[];
  ticketTools: string[];
};

/** 📋️ Loads the authored surface table and the authored description table. */
function authored(ctx: AdapterContext): { surface: Surface; describe: (profile: string, key: string) => string } {
  const surface = JSON.parse(readFileSync(ctx.fixture("shared://📋️surface.json"), "utf8")) as Surface;
  const table = JSON.parse(readFileSync(ctx.fixture("asset://🧬️schema/🔣️descriptions.json"), "utf8")) as { descriptions: Record<string, Record<string, string>> };
  const describe = (profile: string, key: string): string => table.descriptions[key]?.[profile] || table.descriptions[key]?.generic || "";
  return { surface, describe };
}

/** 📋️ Starts a reference SDK server advertising the authored surface for one profile. */
async function referenceListing(surface: Surface, describe: (profile: string, key: string) => string, profile: string): Promise<Record<string, unknown[]>> {
  const extra = surface.profiles.find((entry) => entry.slug === profile)?.extraTicketArgument ?? null;
  const server = new Server({ name: "repo", version: "1.0.0" }, { capabilities: { tools: {}, resources: {}, prompts: {} } });
  server.setRequestHandler(ListToolsRequestSchema, () => ({
    tools: surface.tools
      .map((tool) => ({
        name: tool.name,
        description: describe(profile, tool.descriptionKey),
        inputSchema: { type: "object" as const, properties: extra !== null && surface.ticketTools.includes(tool.name) ? { [extra]: { type: "string" } } : {} },
      }))
      .sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)),
  }));
  server.setRequestHandler(ListResourcesRequestSchema, () => ({
    resources: surface.resources.map((resource) => ({ uri: resource.uri, name: resource.name, description: describe(profile, resource.descriptionKey), mimeType: "text/plain" })).sort((left, right) => (left.uri < right.uri ? -1 : left.uri > right.uri ? 1 : 0)),
  }));
  server.setRequestHandler(ListPromptsRequestSchema, () => ({
    prompts: surface.prompts.map((prompt) => ({ name: prompt.name, description: describe(profile, prompt.descriptionKey), arguments: [{ name: "prompt", required: true }] })).sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0)),
  }));
  const [clientSide, serverSide] = InMemoryTransport.createLinkedPair();
  await server.connect(serverSide);
  await clientSide.start();
  const send = (request: unknown): Promise<Record<string, unknown>> =>
    new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("reference server did not answer")), 5000);
      clientSide.onmessage = (message: unknown) => {
        clearTimeout(timer);
        resolve(message as Record<string, unknown>);
      };
      void clientSide.send(request as never).catch(reject);
    });
  await send({ jsonrpc: "2.0", id: 0, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "listing", version: "1" } } });
  await clientSide.send({ jsonrpc: "2.0", method: "notifications/initialized" } as never);
  const listing: Record<string, unknown[]> = {};
  for (const [method, field] of [
    ["tools/list", "tools"],
    ["resources/list", "resources"],
    ["prompts/list", "prompts"],
  ] as const) {
    const envelope = await send({ jsonrpc: "2.0", id: method, method, params: {} });
    listing[field] = (((envelope.result ?? {}) as Record<string, unknown>)[field] ?? []) as unknown[];
  }
  return listing;
}

function entries(items: unknown[], key: string): { id: string; description: string }[] {
  return items.map((item) => ({ id: String((item as Record<string, unknown>)[key] ?? ""), description: String((item as Record<string, unknown>).description ?? "") }));
}

//#endregion 🧭️Oracle

//#region 🧭️Adapter

/** 🟦️ TypeScript oracle for the advertised surface, backed by `@modelcontextprotocol/sdk`. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "generic-profile-surface": {
      oracle: async (ctx) => {
        const { surface, describe } = authored(ctx);
        const listing = await referenceListing(surface, describe, "generic");
        return { projection: { tools: entries(listing.tools, "name"), resources: entries(listing.resources, "uri"), prompts: entries(listing.prompts, "name") } };
      },
    },
    "ide-profile-surface": {
      oracle: async (ctx) => {
        const { surface, describe } = authored(ctx);
        const profiles = [];
        for (const profile of surface.profiles) {
          const listing = await referenceListing(surface, describe, profile.slug);
          const ticketArguments = listing.tools
            .filter((tool) => surface.ticketTools.includes(String((tool as Record<string, unknown>).name)))
            .map((tool) => {
              const properties = (((tool as Record<string, unknown>).inputSchema ?? {}) as Record<string, unknown>).properties ?? {};
              const names = Object.keys(properties as Record<string, unknown>);
              return { tool: String((tool as Record<string, unknown>).name), extraArgument: names.includes("plan_id") ? "plan_id" : names.includes("spec_id") ? "spec_id" : "" };
            });
          profiles.push({ slug: profile.slug, ticketArguments });
        }
        return { projection: { profiles } };
      },
    },
  },
});

//#endregion 🧭️Adapter
