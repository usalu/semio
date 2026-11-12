"""🎞️ Video oracle (PyAV and the ffmpeg binary of imageio-ffmpeg, both wheel-bundled): the ffprobe facts of a clip, and the path of the pinned ffmpeg executable. Usage: video.py probe <file> | video.py ffmpeg."""
import json
import sys
from fractions import Fraction

import av
import imageio_ffmpeg


def rate(value):
    value = Fraction(value)
    return "%d/%d" % (value.numerator, value.denominator)


def probe(path):
    with av.open(path) as container:
        stream = container.streams.video[0]
        context = stream.codec_context
        facts = {
            "codec": context.name,
            "pixelFormat": context.pix_fmt,
            "width": context.width,
            "height": context.height,
            "rFrameRate": rate(stream.base_rate or stream.guessed_rate or stream.average_rate),
            "durationS": container.duration / av.time_base,
            "tags": dict(stream.metadata),
        }
        times = [float(packet.pts * stream.time_base) for packet in container.demux(stream) if packet.pts is not None and packet.size]
    with av.open(path) as container:
        frames = sum(1 for _ in container.decode(video=0))
    return {**facts, "framesRead": frames, "packetTimesS": sorted(times)}


command = sys.argv[1]
print(json.dumps(probe(sys.argv[2]) if command == "probe" else {"ffmpeg": imageio_ffmpeg.get_ffmpeg_exe()}))
