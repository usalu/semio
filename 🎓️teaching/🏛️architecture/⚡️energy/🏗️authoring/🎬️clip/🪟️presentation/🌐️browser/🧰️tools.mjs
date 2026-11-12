/** 🖥️ Launch options of the owned headless browser: its GPU runs inside the browser process, because the separate GPU process of the pinned headless shell delivers no animation frames on current macOS hosts and every paint barrier would wait forever. */
export const browserLaunchOptions = {headless:true,args:["--in-process-gpu"]};

/** 🎛️ Disable incidental transitions only in deterministic presentation capture. */
export const presentationTransitionStyle = "*,*::before,*::after{transition:none!important}";

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

/** 🖼️ Keep native paint scheduling independent of the controlled animation clock. */
export async function captureNativePaint(page){
  await page.evaluate(()=>{globalThis.semioNativePaint=globalThis.requestAnimationFrame.bind(globalThis)});
}

/** 🎨 Give ResizeObserver and the compositor an actual browser rendering opportunity. */
export async function paintBarrier(page){
  await page.evaluate(()=>new Promise(resolve=>globalThis.semioNativePaint(()=>resolve())));
}

/** ⏸️ Complete incidental CSS transitions while leaving authored keyframe clocks frozen. */
export async function finishIncidentalTransitions(page){
  await page.evaluate(()=>{for(const animation of document.getAnimations())if(animation instanceof CSSTransition)animation.finish()});
  await page.clock.runFor(0);
}

/** 🎞️ Wait until every finite CSS transition has run to its end, because the browser advances them in real time outside the fake clock. */
export async function finishTransitions(page,limit=5000){
  let timer;
  try{
    await Promise.race([
      page.evaluate(()=>Promise.all(document.getAnimations().filter(animation=>animation instanceof CSSTransition && animation.playState==="running" && Number.isFinite(animation.effect?.getComputedTiming().endTime)).map(animation=>animation.finished.catch(()=>{})))),
      new Promise(resolve=>{timer=setTimeout(resolve,limit)})
    ]);
  }finally{clearTimeout(timer)}
}
