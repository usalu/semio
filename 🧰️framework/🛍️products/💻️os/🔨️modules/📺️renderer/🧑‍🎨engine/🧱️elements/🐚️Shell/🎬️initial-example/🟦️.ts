/** 🚦️ Defers a session's first view until its initial example and view publication settle. */
import { useEffect,useRef,useState } from "react";
export function useInitialExampleReadiness(instanceId: string | null, enabled: boolean, start: () => Promise<unknown> | undefined, publish: () => Promise<unknown>): boolean {
  const owner = useRef<{ instance: string } | null>(null);
  const mounted = useRef(false);
  const [ready,setReady] = useState<string | null>(null);
  useEffect(() => { mounted.current = true; return () => { mounted.current = false; }; },[]);
  useEffect(() => {
    if (!enabled || instanceId === null) { owner.current = null; return; }
    if (owner.current?.instance === instanceId) return;
    const work = start();
    if (!work) return;
    const current = { instance: instanceId };
    owner.current = current;
    setReady(null);
    const finish = () => { if (mounted.current && owner.current === current) setReady(instanceId); };
    void work.then(() => mounted.current && owner.current === current ? publish() : undefined).then(finish,finish);
  },[instanceId,enabled,start,publish]);
  return !enabled || (instanceId !== null && ready === instanceId && owner.current?.instance === instanceId);
}
