import{expect,test}from"bun:test";
import{readyNormaliseLineV1,readyProgressKeyV1,readyStallTrackerV1,readyStalledV1,READY_TREE_POLICY_V1 as P}from"../../🟦️.ts";

/** 🪟️ A Windows log is judged like a Unix log: the same line spelled with CRLF, backslashes, doubled JSON backslashes or another path case is one line, a heartbeat that only counts seconds is no progress, and a work counter that advances is progress. */
const crlf=(lines:readonly string[]):string[]=>lines.map(line=>line+"\r");
const heartbeat=(seconds:number,drive:string,slash:string):string[]=>[
 `[repository:command] still running after ${seconds} s\r`,
 `[repo] still working after ${seconds+1} s; last phase started: source.served in ${drive}:${slash}Users${slash}Semio${slash}.cache${slash}f\r`,
];

test("a heartbeat that only counts seconds is no progress on Windows however its path is spelled",()=>{
 const tracker=readyStallTrackerV1();
 expect(tracker.observe(heartbeat(6134,"C","\\"))).toBe(true);
 expect(tracker.observe(heartbeat(6144,"C","\\"))).toBe(false);
 expect(tracker.observe(heartbeat(6154,"c","/"))).toBe(false);
 expect(tracker.observe(heartbeat(6164,"C","\\\\"))).toBe(false);
 expect(readyStalledV1(P.stallStopMilliseconds,P)).toBe(true);
});

test("the normalised key of a line ignores carriage returns, path separators and drive-letter case",()=>{
 const unix=readyNormaliseLineV1("opened c:/users/semio/.cache/f/ready for 90 s");
 for(const spelled of["opened C:\\Users\\Semio\\.cache\\f\\ready for 91 s\r","opened C:\\\\Users\\\\Semio\\\\.cache\\\\f\\\\ready for 92 s","opened c:/USERS/semio/.cache/f/ready for 93 s"])expect(readyNormaliseLineV1(spelled),spelled).toBe(unix);
 expect(readyNormaliseLineV1("opened D:\\Other\\place for 90 s")).not.toBe(unix);
});

test("an advancing work counter is progress and the same counter again is not, with CRLF lines",()=>{
 const tracker=readyStallTrackerV1(),line=(step:number)=>`[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-artifact-input-stage:${step}","mode":"exclusive","elapsedMs":0}}`;
 expect(tracker.observe(crlf([line(1832)]))).toBe(true);
 expect(tracker.observe(crlf([line(1832)]))).toBe(false);
 expect(tracker.observe(crlf([line(1833)]))).toBe(true);
 expect(readyProgressKeyV1(crlf([line(1840),"[repository:command] still running after 6264 s"]))).toBe("native-artifact-input-stage:1840");
 expect(readyProgressKeyV1(crlf(["[repository:command] still running after 6264 s"]))).toBeNull();
});
