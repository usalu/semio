/** 👁️ The presence view of the quiz client: the peers seen on a page as cursors, their places in words and the anchors they point at. */

import { learnerTag, presenceRoster, type Anchor, type CatalogView, type Cursor, type CursorState, type Identity, type Place, type PresenceRoster, type PresenceState, type ThinkingState } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../../🌐️i18n/🟦️.ts";
import { learnerName } from "../../🪪️identity/🟦️.tsx";
import type { RoomMember } from "../🔌️socket/🟦️.ts";
import { type PresenceSelf, type QuizPresenceSnapshot, THINKING_SUFFIX, type WatchedState } from "../🧭️rooms/🟦️.ts";

//#region 👁️View
/** 🖱️ Another learner in a room: its label, its palette slot (the one of its roster session, so a learner has one
 * colour everywhere; the room session's until the roster knows it), its pointer, keyboard focus and dragged item. */
export interface PeerCursor {
  readonly session: string;
  readonly tag: string;
  readonly label: string;
  readonly colour: number;
  readonly cursor: Cursor | undefined;
  readonly focus: Anchor | undefined;
  readonly drag: string | undefined;
}

/** 👁️ What the screens show of presence: the roster (undefined until the roster room is joined), the own tag, the
 * palette slot of every learner online, the other learners in this place's room, the other learners of every room the
 * client joined or watches by scope, and the drafts of the others in every thinking room by scope. */
export interface PresenceView {
  readonly me: string | undefined;
  readonly roster: PresenceRoster | undefined;
  readonly colours: ReadonlyMap<string, number>;
  readonly peers: readonly PeerCursor[];
  readonly rooms: ReadonlyMap<string, readonly PeerCursor[]>;
  readonly thinking: ReadonlyMap<string, readonly ThinkingState[]>;
}

/** 👁️ Presence while nothing is shared. */
export const EMPTY_PRESENCE_VIEW: PresenceView = { me: undefined, roster: undefined, colours: new Map(), peers: [], rooms: new Map(), thinking: new Map() };

function isThinking(state: WatchedState): state is ThinkingState {
  return Object.hasOwn(state, "answers");
}

/** 👁️ The view of a snapshot, labelling learners in `text`'s language (anonymous learners by their tag); the own
 * sessions (any tab of the own learner) are left out of every room. */
export function presenceView(snapshot: QuizPresenceSnapshot, text: QuizText): PresenceView {
  const rosterRoom = snapshot.roster;
  const joined = rosterRoom !== undefined && rosterRoom.status === "open";
  const display = (state: PresenceState): string => learnerName(state.identity, state.tag, text);
  const roster = joined ? presenceRoster(rosterRoom.members, display) : undefined;
  const slot = new Map(rosterRoom?.members.map((member) => [member.session, member.colour]) ?? []);
  const colours = new Map((roster?.learners ?? []).map((learner) => [learner.tag, slot.get(learner.session) ?? 0]));
  const names = new Map((roster?.learners ?? []).map((learner) => [learner.tag, learner.display]));
  const others = <S extends { readonly tag: string }>(members: readonly RoomMember<S>[], self: string | undefined): readonly RoomMember<S>[] => members.filter((member) => member.session !== self && member.state.tag !== snapshot.tag);
  const cursors = (members: readonly RoomMember<CursorState>[], self: string | undefined): readonly PeerCursor[] =>
    others(members, self).map((member) => ({
      session: member.session,
      tag: member.state.tag,
      label: names.get(member.state.tag) ?? learnerName(undefined, member.state.tag, text),
      colour: colours.get(member.state.tag) ?? member.colour,
      cursor: member.state.cursor,
      focus: member.state.focus,
      drag: member.state.drag?.item,
    }));
  const room = snapshot.room;
  const open = room !== undefined && room.status === "open";
  const peers = open ? cursors(room.members, room.self) : [];
  const rooms = new Map<string, readonly PeerCursor[]>();
  const thinking = new Map<string, readonly ThinkingState[]>();
  if (open && snapshot.scopes.room !== undefined) rooms.set(snapshot.scopes.room, peers);
  for (const [scope, members] of open ? room.watched : []) {
    if (scope.endsWith(THINKING_SUFFIX))
      thinking.set(
        scope,
        others(members, undefined).flatMap((member) => (isThinking(member.state) ? [member.state] : [])),
      );
    else
      rooms.set(
        scope,
        cursors(
          members.flatMap((member) => (isThinking(member.state) ? [] : [{ ...member, state: member.state }])),
          undefined,
        ),
      );
  }
  const drafts = snapshot.thinking;
  if (drafts !== undefined && drafts.status === "open" && snapshot.scopes.thinking !== undefined)
    thinking.set(
      snapshot.scopes.thinking,
      others(drafts.members, drafts.self).map((member) => member.state),
    );
  return { me: snapshot.tag, roster, colours, peers, rooms, thinking };
}

/** 🗺️ Where a learner is, in words. */
export function placeText(place: Place, catalog: CatalogView | undefined, text: QuizText, locale: QuizLocale): string {
  const quiz = catalog?.quizzes.find((candidate) => candidate.id === place.quiz);
  const title = quiz === undefined ? (place.quiz ?? "") : localized(quiz.title, locale);
  switch (place.screen) {
    case "introduction":
      return text("quiz.presence.atIntroduction");
    case "identity":
      return text("quiz.presence.atIdentity");
    case "home":
      return text("quiz.presence.atHome");
    case "leaderboard":
      return text("quiz.presence.atLeaderboard");
    case "badges":
      return text("quiz.presence.atBadges");
    case "learner":
      return text("quiz.presence.atLearner");
    case "preferences":
      return text("quiz.presence.atPreferences");
    case "quiz":
      return text("quiz.presence.atQuiz", { quiz: title });
    case "run":
      return text("quiz.presence.atRun", { quiz: title });
    case "results":
      return text("quiz.presence.atResults", { quiz: title });
  }
}

/** 🪪️ The presence self of an identified learner, once its identity is known. */
export function presenceSelf(learner: { readonly id: string; readonly identity?: Identity } | undefined): PresenceSelf | undefined {
  return learner?.identity === undefined ? undefined : { tag: learnerTag(learner.id), identity: learner.identity };
}

/** ⚓️ The anchor keys of the landmarks every learner at a place renders. */
export const PRESENCE_ANCHORS = {
  introduction: "introduction",
  leaderboard: "leaderboard",
  badges: "badges",
  run: "run",
  results: "results",
  home: (card: string): Anchor => `home:${card}`,
  quiz: (quiz: string): Anchor => `quiz:${quiz}`,
  quizTasks: (quiz: string): Anchor => `quiz:${quiz}:tasks`,
  task: (task: string): Anchor => `task:${task}`,
  result: (task: string): Anchor => `result:${task}`,
  item: (item: string): Anchor => `item:${item}`,
  category: (category: string): Anchor => `category:${category}`,
} as const;
//#endregion 👁️View
