import { TutorialBar, createTutorialClock, useTutorialClock, useLabel, type TutorialChapterMarker } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useEffect, useState } from "react";

const meta = {
  title: "🖱️ui⚛️react/TutorialBar",
  component: TutorialBar,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof TutorialBar>;
export default meta;
type Story = StoryObj<typeof Demo>;

function Demo({ recordingInitially = false, durationMs = 60000 }: { recordingInitially?: boolean; durationMs?: number }) {
  const title = useLabel("ui.common.example");
  const chapterTitle = useLabel("tutorial.chapter");
  const [clock] = useState(() => createTutorialClock(durationMs));
  const time = useTutorialClock(clock);
  const [playing, setPlaying] = useState(false);
  const [rate, setRate] = useState(1);
  const [muted, setMuted] = useState(true);
  const [captionsOn, setCaptionsOn] = useState(true);
  const [recording, setRecording] = useState(recordingInitially);
  const [chapters, setChapters] = useState<TutorialChapterMarker[]>([{ id: "start", atMs: 0, title: chapterTitle }]);
  useEffect(() => () => clock.dispose(), [clock]);
  return <div className="min-h-64 pt-double">
    <TutorialBar title={title} durationMs={durationMs} clock={clock} playing={playing && time < durationMs} rate={rate} muted={muted} captionsOn={captionsOn} recording={recording} recordAvailable chapters={durationMs > 0 ? chapters : []}
      onPlayPause={() => { if (clock.isPlaying()) { clock.pause(); setPlaying(false); } else { clock.play(); setPlaying(durationMs > 0); } }}
      onStop={() => { clock.pause(); clock.seek(0); setPlaying(false); }}
      onSeek={clock.seek} onRateChange={(value) => { clock.setRate(value); setRate(value); }} onMutedChange={setMuted} onCaptionsChange={setCaptionsOn}
      onRecordToggle={() => setRecording((value) => !value)}
      onAddChapter={() => setChapters((value) => [...value, { id: `chapter-${value.length}`, atMs: time, title: `${chapterTitle} ${value.length + 1}` }])} />
    <output className="p-double" aria-live="polite">{chapterTitle}: {chapters.length}</output>
  </div>;
}

export const Default: Story = { render: () => <Demo /> };
export const Recording: Story = { render: () => <Demo recordingInitially /> };
export const EmptyTimeline: Story = { render: () => <Demo durationMs={0} /> };
