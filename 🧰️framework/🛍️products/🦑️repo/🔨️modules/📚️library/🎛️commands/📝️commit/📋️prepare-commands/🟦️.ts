function shSingleQuote(s: string): string {
  return `'${s.replace(/'/g, `'\"'\"'`)}'`;
}

function formatCommitPrepareCommandBlock(command: string): string {
  return `\`\`\`\n${command}\n\`\`\``;
}

/** 🏷️GPG-signed annotated tag; tag object name and `-m` message are the same (`…🚩️`). */
export function formatGitSignedTagCommand(tagName: string, head = "HEAD"): string {
  const q = shSingleQuote(tagName);
  return `git tag -s -m ${q} ${q} ${head}`;
}

/** 📋️Four copy-paste ``` blocks: signed tag, squash, push, all-in-one. */
export function formatCommitPrepareCommands(opts: { tagName: string; wipSha: string; messageFile?: string }): string {
  const msg = opts.messageFile ?? ".git/compose-commit-message";
  const tag = formatGitSignedTagCommand(opts.tagName);
  const squash = `git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)}`;
  const push = `git push --follow-tags`;
  const all = `${tag} && git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)} && git push --follow-tags`;
  return `${[tag, squash, push, all].map(formatCommitPrepareCommandBlock).join("\n\n")}\n`;
}

/** 📋️Prepare-only agent reply: four `git` blocks, then tag name, then full commit message. */
export function formatCommitPrepareAgentReply(opts: { tagName: string; wipSha: string; messageFile?: string; commitMessage: string }): string {
  const commands = formatCommitPrepareCommands({
    tagName: opts.tagName,
    wipSha: opts.wipSha,
    messageFile: opts.messageFile,
  });
  const tagNameBlock = formatCommitPrepareCommandBlock(opts.tagName.trim());
  const messageBlock = formatCommitPrepareCommandBlock(opts.commitMessage.trimEnd());
  return `${commands}${tagNameBlock}\n\n${messageBlock}\n`;
}
