/** 🏅️ Badge rules evaluated after every submission over all submitted results of a learner; a rule that names a least challenge counts only the results at a challenge that meets it.
 *
 * @see ../../README.md — the badge rules
 * @see ./🦀️.rs — the Rust twin
 */
import type { Badge, BadgeRule, Challenge, Quiz, RunResult } from "../../🧬️schema/🟦️.ts";
import { challengeMeets } from "../⛰️challenge/🟦️.ts";

/** 🧗️ Whether a result counts for a rule: every result without a least challenge, else those at a challenge that meets it. */
function counts(result: RunResult, least: Challenge | undefined): boolean {
  return least === undefined || challengeMeets(result.challenge, least);
}

/** 💎️ Whether some result of the rule's quiz that counts scored 1. */
function perfectQuiz(rule: Extract<BadgeRule, { kind: "perfect-quiz" }>, results: readonly RunResult[]): boolean {
  return results.some((result) => result.quiz === rule.quiz && counts(result, rule.challenge) && result.score === 1);
}

/** 🎯️ Whether every catalog task matching the selector scored 1 in some result that counts; a selector matching no task never awards. */
function perfectTasks(rule: Extract<BadgeRule, { kind: "perfect-tasks" }>, quizzes: readonly Quiz[], results: readonly RunResult[]): boolean {
  const selected = quizzes.flatMap((quiz) => (rule.quiz === undefined || quiz.id === rule.quiz ? quiz.tasks.filter((task) => rule.taskKind === undefined || task.kind === rule.taskKind).map((task) => [quiz.id, task.id] as const) : []));
  const counted = results.filter((result) => counts(result, rule.challenge));
  return selected.length > 0 && selected.every(([quiz, task]) => counted.some((result) => result.quiz === quiz && result.tasks.some((taskResult) => taskResult.task === task && taskResult.score === 1)));
}

/** ✅️ Whether every catalog quiz has at least one submitted result; an empty catalog never awards. */
function completedQuizzes(quizzes: readonly Quiz[], results: readonly RunResult[]): boolean {
  return quizzes.length > 0 && quizzes.every((quiz) => results.some((result) => result.quiz === quiz.id));
}

/** 🎖️ The ids of the badges newly earned, in catalog badge order, given all submitted results including the new one and the ids already held. */
export function earnedBadges(badges: readonly Badge[], quizzes: readonly Quiz[], results: readonly RunResult[], held: readonly string[]): string[] {
  return badges
    .filter((badge) => {
      if (held.includes(badge.id)) return false;
      switch (badge.rule.kind) {
        case "perfect-quiz":
          return perfectQuiz(badge.rule, results);
        case "perfect-tasks":
          return perfectTasks(badge.rule, quizzes, results);
        case "completed-quizzes":
          return completedQuizzes(quizzes, results);
      }
    })
    .map((badge) => badge.id);
}
