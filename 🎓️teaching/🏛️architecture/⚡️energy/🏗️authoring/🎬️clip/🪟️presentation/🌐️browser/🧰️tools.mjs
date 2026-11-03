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

/** ⏩ Yield between fake-clock timers through message tasks instead of real timers, whose 4 ms nesting clamp and 15.6 ms Windows resolution made each frame wait seconds; a missing hook leaves Playwright's own pacing untouched. */
export async function accelerateClock(page,hops=6){
  await page.evaluate(hops=>{
    const controller=globalThis.__pwClock?.controller,embedder=controller?._embedder;
    if(typeof embedder?.setTimeout!=="function")return;
    const channel=new MessageChannel(),queue=[];
    channel.port1.onmessage=()=>queue.shift()();
    const post=step=>{queue.push(step);channel.port2.postMessage(0)};
    embedder.setTimeout=task=>{let left=hops;const step=()=>{if(--left<=0)task();else post(step)};post(step);return()=>{}};
  },hops);
}

/** 🎞️ Wait until every finite CSS transition has run to its end, because the browser advances them in real time outside the fake clock. */
export async function finishTransitions(page,limit=5000){
  let timer;
  try{
    await Promise.race([
      page.evaluate(()=>Promise.all(document.getAnimations().filter(animation=>Number.isFinite(animation.effect?.getComputedTiming().endTime)).map(animation=>animation.finished.catch(()=>{})))),
      new Promise(resolve=>{timer=setTimeout(resolve,limit)})
    ]);
  }finally{clearTimeout(timer)}
}
