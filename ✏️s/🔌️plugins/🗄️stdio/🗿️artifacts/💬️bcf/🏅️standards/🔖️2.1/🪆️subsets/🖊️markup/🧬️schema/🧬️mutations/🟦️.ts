/** 🧬️ BcfMutation union. The entity payloads (`BcfTopic`, `BcfComment`, `BcfViewpoint`, `BcfCamera`,
 * `BcfComponents`) are carried as `unknown` because `../📸️snapshot/🟦️.ts`'s `BcfSnapshot` is still the
 * pre-rewrite raw-`entries` stub with no entity types to mirror against; see `🦀️.rs` in this directory. */
export type BcfMutation =
  | { mutation: 'setVersion'; version: string }
  | { mutation: 'insertTopic'; topic: unknown; index?: number }
  | { mutation: 'removeTopic'; guid: string }
  | { mutation: 'setTopicMarkup'; guid: string; title?: string; description?: string; status?: string; priority?: string; labels?: string[]; creationDate?: string; creationAuthor?: string }
  | { mutation: 'insertComment'; topicGuid: string; comment: unknown; index?: number }
  | { mutation: 'removeComment'; topicGuid: string; guid: string }
  | { mutation: 'setComment'; topicGuid: string; guid: string; date?: string; author?: string; text?: string; viewpointRef?: string | null }
  | { mutation: 'insertViewpoint'; topicGuid: string; viewpoint: unknown; index?: number }
  | { mutation: 'removeViewpoint'; topicGuid: string; guid: string }
  | { mutation: 'setViewpointCamera'; topicGuid: string; guid: string; camera: unknown | null }
  | { mutation: 'setViewpointComponents'; topicGuid: string; guid: string; components: unknown | null }
  | { mutation: 'setViewpointSnapshot'; topicGuid: string; guid: string; snapshot: number[] | null };
