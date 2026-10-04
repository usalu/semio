/** 🧬️ PlaybookMutation union — one variant per 🧬️mutations/<slug> triad leaf; steps and blocks are edited on the `flow`
 * child's own lane (stdio flow leaves), never by a parent-lane playbook mutation. */
export type PlaybookMutation = { mutation: 'changeTitle'; newTitle?: string };
