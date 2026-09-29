/** 🏅️ Badge rules evaluated after every submission over all submitted results of a learner.
 *
 * @see ../../README.md — the badge rules
 * @see ./🦀️.rs — the Rust twin
 */
import type { Badge, BadgeRule, Quiz, RunResult } from "../../🧬️schema/🟦️.ts";

/** 💎️ Whether some result of `quiz` scored 1. */
function perfectQuiz(quiz: string, results: readonly RunResult[]): boolean {
  return results.some((result) => result.quiz === quiz && result.score === 1);
}

/** 🎯️ Whether every catalog task matching the selector scored 1 in some result; a selector matching no task never awards. */
function perfectTasks(rule: Extract<BadgeRule, { kind: "perfect-tasks" }>, quizzes: readonly Quiz[], results: readonly RunResult[]): boolean {
  const selected = quizzes.flatMap((quiz) => (rule.quiz === undefined || quiz.id === rule.quiz ? quiz.tasks.filter((task) => rule.taskKind === undefined || task.kind === rule.taskKind).map((task) => [quiz.id, task.id] as const) : []));
  return selected.length > 0 && selected.every(([quiz, task]) => results.some((result) => result.quiz === quiz && result.tasks.some((taskResult) => taskResult.task === task && taskResult.score === 1)));
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
          return perfectQuiz(badge.rule.quiz, results);
        case "perfect-tasks":
          return perfectTasks(badge.rule, quizzes, results);
        case "completed-quizzes":
          return completedQuizzes(quizzes, results);
      }
    })
    .map((badge) => badge.id);
}
