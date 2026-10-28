/** 🎭 Launch the owned browser, provisioning once only for a missing pinned executable. */
export async function launchBrowser({launch,provision,signal}) {
  signal?.throwIfAborted();
  try {
    return await launch();
  } catch(error) {
    signal?.throwIfAborted();
    if(!/^browserType\.launch: Executable doesn't exist at /m.test(error?.message??""))throw error;
  }
  signal?.throwIfAborted();
  await provision();
  signal?.throwIfAborted();
  return launch();
}
