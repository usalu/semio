#!/usr/bin/env python3
"""👆️ Oracle of the pets product's gesture recognition (design §17), in Python.

The subjects recognise gestures tick by tick with comparisons only: no angle, no velocity, no window of
samples. This reference reads the same traces whole, with third-party tools, and never steps a state.

Press. What a press becomes is decided in closed form: ``numpy.hypot`` gives the distance of every drag from
the press, ``numpy.flatnonzero`` the first one at the slop or beyond and the first event that ends the press,
and their order against the tick of the hold names the signals.

Heat. A leaky bucket that never falls below zero is Lindley's recursion; its solution is the running sum of
the net gains minus the running minimum of that sum (``numpy.cumsum``, ``numpy.minimum.accumulate``). The tier
is a ``numpy.searchsorted`` on the two thresholds, the run a run length.

Circle. ``numpy.unwrap(numpy.arctan2(y, x))`` is the winding of the pointer round the body's centre; with the y
axis pointing down a growing angle is clockwise on screen. A cue is admitted only where a full turn in its
direction was swept within the time and the band a lap has, round and closed (soundness). A circle that the
reading finds clean — in the band, steady, round, closed, long enough — must be cued in the direction of its
winding before the pointer has swept 2.4 turns (completeness).

Stroke and shake. ``scipy.signal.find_peaks`` with a prominence finds the reversals of the horizontal offset
(stroke) and of the grip along its principal axis (``numpy.linalg.eigh`` of the covariance; shake). A cue is
admitted only where the reversals that make it lie before it; a clean petting or shake must be cued by its
fifth or sixth reversal.

Ordinary travel and carrying are expected to set off nothing; the reading refuses a trace of that label in
which it finds a clean gesture. The cues of the vectors were recorded from the TypeScript subject when the
vectors were generated and are answered here as committed, after every one of them has passed these
readings — so a committed cue can never be one the libraries cannot justify, and a clean gesture can never
go without one.

@see https://numpy.org/doc/stable/reference/generated/numpy.unwrap.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.signal.find_peaks.html
@see https://en.wikipedia.org/wiki/Lindley_equation
@see ../../🧫️fixtures/👆️gesture-recognition/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
import scipy.signal

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://👆️gesture-recognition/🔣️.json"
TICKS_PER_SECOND = 64
SLOP_FINE = 6.0
SLOP_COARSE = 10.0
HOLD_TICKS = 28
HEAT_CLICK = 1.0
HEAT_HOLD = 0.03125
HEAT_LEAK = 0.5
HEAT_HELLO = 1.0
HEAT_TRICK = 3.0
HEAT_ENOUGH = 7.0
HEAT_FORGIVEN = 2.0
ENOUGH_TICKS = 512
SCROLL_TICKS = 16
CIRCLE_MARGIN = 8.0
CIRCLE_REACH = 3.4
CIRCLE_HYSTERESIS = 4.0
CIRCLE_QUARTERS = 5
CIRCLE_FAST = 4
CIRCLE_SLOW = 40
CIRCLE_ROUND = 2.5
CIRCLE_CLOSE = 1.375
CIRCLE_AGAINST = 1
CIRCLE_OUT_TICKS = 8
CIRCLE_REST = 128
STROKE_MARGIN = 10.0
STROKE_HYSTERESIS = 0.15
STROKE_LENGTH = 0.45
STROKE_SLOW = 60.0
STROKE_FAST = 900.0
STROKE_SLANT = 0.5
STROKE_SEGMENTS = 3
STROKE_WINDOW = 96
STROKE_PAUSE = 48
SHAKE_AMPLITUDE = 0.75
SHAKE_HYSTERESIS = 8.0
SHAKE_SPEED = 190.0
SHAKE_REVERSALS = 4
SHAKE_WINDOW = 64
SHAKE_PAUSE = 48
SHAKE_REST = 128
CONSTANTS = {
    "slopFine": SLOP_FINE,
    "slopCoarse": SLOP_COARSE,
    "holdTicks": HOLD_TICKS,
    "heatClick": HEAT_CLICK,
    "heatHold": HEAT_HOLD,
    "heatLeak": HEAT_LEAK,
    "heatHello": HEAT_HELLO,
    "heatTrick": HEAT_TRICK,
    "heatEnough": HEAT_ENOUGH,
    "heatForgiven": HEAT_FORGIVEN,
    "enoughTicks": ENOUGH_TICKS,
    "scrollTicks": SCROLL_TICKS,
    "circleMargin": CIRCLE_MARGIN,
    "circleReach": CIRCLE_REACH,
    "circleHysteresis": CIRCLE_HYSTERESIS,
    "circleQuarters": CIRCLE_QUARTERS,
    "circleFast": CIRCLE_FAST,
    "circleSlow": CIRCLE_SLOW,
    "circleRound": CIRCLE_ROUND,
    "circleClose": CIRCLE_CLOSE,
    "circleAgainst": CIRCLE_AGAINST,
    "circleOutTicks": CIRCLE_OUT_TICKS,
    "circleRest": CIRCLE_REST,
    "strokeMargin": STROKE_MARGIN,
    "strokeHysteresis": STROKE_HYSTERESIS,
    "strokeLength": STROKE_LENGTH,
    "strokeSlow": STROKE_SLOW,
    "strokeFast": STROKE_FAST,
    "strokeSlant": STROKE_SLANT,
    "strokeSegments": STROKE_SEGMENTS,
    "strokeWindow": STROKE_WINDOW,
    "strokePause": STROKE_PAUSE,
    "shakeAmplitude": SHAKE_AMPLITUDE,
    "shakeHysteresis": SHAKE_HYSTERESIS,
    "shakeSpeed": SHAKE_SPEED,
    "shakeReversals": SHAKE_REVERSALS,
    "shakeWindow": SHAKE_WINDOW,
    "shakePause": SHAKE_PAUSE,
    "shakeRest": SHAKE_REST,
}
TIERS = ["hello", "trick", "purr", "enough"]
EDGE = 1e-6
LAP_TICKS = (CIRCLE_QUARTERS + 2 * CIRCLE_AGAINST) * CIRCLE_SLOW
CLEAN_TURNS = 2.4
MIRRORED = {"circle": "countercircle", "countercircle": "circle"}


def trail(path, hold):
    """🧵️ The points of a path, one per tick, in quanta: the first point, then per tick a step ``dx, dy`` or one number ``hold + n`` that repeats the latest point ``n`` times."""
    tokens = numpy.asarray(path[2:], dtype=numpy.int64)
    moves, index = [], 0
    while index < len(tokens):
        if tokens[index] >= hold:
            moves.append(numpy.zeros((int(tokens[index]) - hold, 2), dtype=numpy.int64))
            index += 1
        else:
            end = index
            while end < len(tokens) and tokens[end] < hold:
                end += 2
            moves.append(tokens[index:end].reshape(-1, 2))
            index = end
    steps = numpy.concatenate(moves) if moves else numpy.zeros((0, 2), dtype=numpy.int64)
    points = numpy.concatenate([[[path[0], path[1]]], numpy.asarray(path[0:2], dtype=numpy.int64) + numpy.cumsum(steps, axis=0)])
    return points[:, 0], points[:, 1]


def variant(xs, ys, page, number):
    """🪞️ A trail in one of its sixteen variants: bit 0 mirrors left and right, bit 1 top and bottom, bit 2 swaps the axes, bit 3 plays it backwards."""
    mirrored = page[0] - xs if number & 1 else xs
    flipped = page[1] - ys if number & 2 else ys
    across, down = (flipped, mirrored) if number & 4 else (mirrored, flipped)
    return (across[::-1], down[::-1]) if number & 8 else (across, down)


def box_variant(box, page, number):
    """🔲️ A body in the same variant."""
    x = page[0] - box[0] - box[2] if number & 1 else box[0]
    y = page[1] - box[1] - box[3] if number & 2 else box[1]
    return [y, x, box[3], box[2]] if number & 4 else [x, y, box[2], box[3]]


def offsets(xs, ys, box, quantum):
    """📐️ The pointer relative to the centre of a body, in pixels, and the body's width and height."""
    return xs / quantum - (box[0] + box[2] / 2) / quantum, ys / quantum - (box[1] + box[3] / 2) / quantum, box[2] / quantum, box[3] / quantum


def extremes(values, prominence):
    """⛰️ The reversals of a signal: the ticks of its peaks and of its valleys of at least a prominence, in order of time, each with its direction."""
    peaks = scipy.signal.find_peaks(values, prominence=prominence)[0]
    valleys = scipy.signal.find_peaks(-values, prominence=prominence)[0]
    ticks = numpy.concatenate([peaks, valleys])
    order = numpy.argsort(ticks, kind="stable")
    return ticks[order], numpy.concatenate([numpy.ones(len(peaks)), -numpy.ones(len(valleys))])[order]


# endregion 🔖️Reference


# region 🔖️Press
def press_signals(vector, quantum):
    """🕹️ The signals of a press vector: per press the first drag at the slop or beyond, the first event that ends it, and the tick of the hold, read off with numpy and put in order."""
    events = vector["events"]
    starts = [index for index, event in enumerate(events) if event["kind"] == "pressed"]
    signals, opened = [], False
    for number, start in enumerate(starts):
        press = events[start]
        guards = press.get("guards", [])
        if opened:
            signals.append([press["at"], "abort"])
            opened = False
        if "still" in guards or "control" in guards:
            continue
        after = events[start + 1 : starts[number + 1] if number + 1 < len(starts) else len(events)]
        limit = events[starts[number + 1]]["at"] if number + 1 < len(starts) else vector["ticks"]
        slop = SLOP_COARSE if press.get("pointer", "mouse") == "touch" else SLOP_FINE
        ticks = numpy.array([event["at"] for event in after], dtype=int)
        kinds = numpy.array([event["kind"] for event in after], dtype=object)
        still = numpy.array(["still" in event.get("guards", []) for event in after], dtype=bool)
        scrolled = numpy.array(["scrolled" in event.get("guards", []) for event in after], dtype=bool)
        away = numpy.hypot(numpy.array([event.get("x", press["x"]) - press["x"] for event in after], dtype=float) / quantum, numpy.array([event.get("y", press["y"]) - press["y"] for event in after], dtype=float) / quantum)
        dragged = kinds == "dragged" if len(after) else numpy.zeros(0, dtype=bool)
        if numpy.any(dragged & (numpy.abs(away - slop) < EDGE) & (numpy.abs(away - slop) > 0)):
            raise AssertionError("presses/%s: a drag lies within %r of the slop without being on it" % (vector["id"], EDGE))
        over = (kinds == "released") | (kinds == "cancelled") if len(after) else numpy.zeros(0, dtype=bool)
        ends = numpy.flatnonzero(over | still | scrolled)
        lifts = numpy.flatnonzero(dragged & (away >= slop) & ~still & ~scrolled)
        end = int(ends[0]) if ends.size else None
        lift = int(lifts[0]) if lifts.size and (end is None or lifts[0] < end) else None
        hold = press["at"] + HOLD_TICKS
        decided = int(ticks[lift]) if lift is not None else int(ticks[end]) if end is not None else limit
        held = hold <= decided
        if held:
            signals.append([hold, "hold"])
        opened = True
        if lift is not None:
            signals.append([int(ticks[lift]), "lift"])
            closes = numpy.flatnonzero((over | still) & (numpy.arange(len(after)) > lift))
            if closes.size:
                close = int(closes[0])
                signals.append([int(ticks[close]), "drop" if kinds[close] == "released" and not still[close] else "abort"])
                opened = False
        elif end is not None:
            plain = kinds[end] == "released" and not still[end] and not scrolled[end]
            signals.append([int(ticks[end]), ("unhold" if held else "click") if plain else "abort"])
            opened = False
    return signals


# endregion 🔖️Press


# region 🔖️Heat
def warmth_answers(caresses):
    """💓️ The answer to every caress: the heat by Lindley's recursion in closed form, the tier by ``numpy.searchsorted``, the run as a run length, the tricks as a running count, and the end of the refractory time."""
    ticks = numpy.array([tick for tick, _caress in caresses], dtype=float)
    holds = numpy.array([caress == "hold" for _tick, caress in caresses], dtype=bool)
    gains = numpy.where(holds, HEAT_HOLD, HEAT_CLICK)
    answers, begin, heat, since, until, tricks, tier, run = [], 0, 0.0, 0.0, 0, 0, 0, 0
    while begin < len(ticks):
        times, added = ticks[begin:], gains[begin:]
        leaks = numpy.diff(numpy.concatenate([[since], times])) * HEAT_LEAK / TICKS_PER_SECOND
        walk = numpy.concatenate([[0.0], numpy.cumsum(numpy.concatenate([[heat], added[:-1]]) - leaks)])
        after = walk[1:] - numpy.minimum.accumulate(walk)[1:] + added
        full = numpy.flatnonzero(after >= HEAT_ENOUGH)
        count = int(full[0]) if full.size else len(times)
        tiers = numpy.where(holds[begin : begin + count], 2, numpy.searchsorted([HEAT_HELLO, HEAT_TRICK], after[:count], side="left"))
        chain = numpy.concatenate([[tier], tiers])
        fresh = numpy.concatenate([[True], chain[1:] != chain[:-1]])
        position = numpy.arange(len(chain))
        runs = position - numpy.maximum.accumulate(numpy.where(fresh, position, 0)) + numpy.where(numpy.maximum.accumulate(numpy.where(fresh, position, 0)) == 0, run, 1)
        counted = tricks + numpy.cumsum(tiers == 1)
        for index in range(count):
            answers.append([float(after[index]), TIERS[int(tiers[index])], int(runs[index + 1]), int(counted[index]), int(until)])
        if count:
            tier, run, tricks = int(tiers[-1]), int(runs[-1]), int(counted[-1])
        if not full.size:
            break
        until = int(times[count]) + ENOUGH_TICKS
        ignored = int(numpy.count_nonzero(times[count:] < until))
        for index in range(ignored):
            answers.append([HEAT_FORGIVEN, "enough", index + 1, tricks, until])
        begin, heat, since, tier, run = begin + count + ignored, HEAT_FORGIVEN, float(until), 3, ignored
    return answers


# endregion 🔖️Heat


# region 🔖️Circle
def winding(xs, ys, box, quantum):
    """🌀️ The pointer round a body: its unwrapped angle in turns (growing clockwise on screen), its distance from the centre, and whether a lap can be under way — it was in the band at most ``CIRCLE_OUT_TICKS`` ago."""
    across, down, width, height = offsets(xs, ys, box, quantum)
    distance = numpy.hypot(across, down)
    reach = max(width, height)
    banded = (distance >= reach / 2 + CIRCLE_MARGIN) & (distance <= reach * CIRCLE_REACH)
    index = numpy.arange(len(distance))
    live = index - numpy.maximum.accumulate(numpy.where(banded, index, -(10**9))) <= CIRCLE_OUT_TICKS
    return numpy.unwrap(numpy.arctan2(down, across)) / (2 * math.pi), distance, live


def lap_seen(turns, distance, live, tick, way):
    """🔎️ Whether a lap in a direction ends at a tick: within the ticks a lap may take, while a lap can be under way, the angle swept a full turn — less what the Schmitt trigger and one sample may hide —, round and closed."""
    low = max(tick - LAP_TICKS, 0)
    dead = numpy.flatnonzero(~live[low : tick + 1])
    start = low + (int(dead[-1]) + 1 if dead.size else 0)
    if start >= tick:
        return False
    slack = 0.03 + float(numpy.abs(numpy.diff(turns[start : tick + 1])).max())
    full = numpy.flatnonzero(way * (turns[tick] - turns[start : tick + 1]) >= 1 - slack)
    if not full.size:
        return False
    began = start + int(full[-1])
    lap = distance[began : tick + 1]
    return bool(lap.max() <= CIRCLE_ROUND * lap.min() * 1.02 and max(lap[-1] / lap[0], lap[0] / lap[-1]) <= CIRCLE_CLOSE * 1.25)


def clean_circle(turns, distance, live, span, box, quantum):
    """✅️ The direction of a circle drawn between two ticks and the tick by which it has to be cued, when the reading finds it clean; ``None`` otherwise.

    Clean: a lap can be under way throughout and the pointer keeps 3 px inside the band; the angle never goes back by more than 0.02 turns and sweeps at least 2.45 turns; every quarter turn takes more than ``CIRCLE_FAST + 2`` and less than ``CIRCLE_SLOW − 4`` ticks; the farthest point is at most ``CIRCLE_ROUND − 0.3`` times as far as the nearest; and one turn later the distance has changed by at most a factor ``CIRCLE_CLOSE − 0.15``. Such a circle is cued by the time 2.4 turns are swept: a lap that began on the way in may be thrown away once (a step against it, or an end that does not close), and the next one takes at most 2.25 turns from the beginning of the drawing."""
    begin, end = span
    reach = max(box[2], box[3]) / quantum
    section, far = turns[begin : end + 1], distance[begin : end + 1]
    way = 1 if section[-1] >= section[0] else -1
    swept = way * (section - section[0])
    crest = numpy.maximum.accumulate(swept)
    if not (live[begin : end + 1].all() and far.min() >= reach / 2 + CIRCLE_MARGIN + 3 and far.max() <= reach * CIRCLE_REACH - 3):
        return None
    if swept[-1] < CLEAN_TURNS + 0.05 or (crest - swept).max() > 0.02:
        return None
    marks = numpy.searchsorted(crest, numpy.arange(0, crest[-1], 1 / 32), side="left")
    quarters = marks[8:] - marks[:-8]
    if quarters.min() <= CIRCLE_FAST + 2 or quarters.max() >= CIRCLE_SLOW - 4:
        return None
    if far.max() > (CIRCLE_ROUND - 0.3) * far.min():
        return None
    later = numpy.searchsorted(crest, swept + 1, side="left")
    paired = later < len(far)
    ratio = far[later[paired]] / far[paired]
    if ratio.max() > CIRCLE_CLOSE - 0.15 or ratio.min() < 1 / (CIRCLE_CLOSE - 0.15):
        return None
    return ("circle" if way > 0 else "countercircle"), begin + int(numpy.searchsorted(crest, CLEAN_TURNS, side="left")) + 2


def widest_sweep(turns, live):
    """🧭️ The stretch of unbroken following in which the angle swept most in one direction: its two ticks, or ``None`` when no lap can ever be under way."""
    index = numpy.arange(len(turns))
    run = numpy.cumsum(numpy.concatenate([[True], live[1:] != live[:-1]]))
    best, found = 0.0, None
    for way in (1, -1):
        shifted = way * turns - run * 1e6
        gained = shifted - numpy.minimum.accumulate(shifted)
        gained[~live] = 0
        top = int(numpy.argmax(gained))
        if gained[top] > best:
            same = live & (run == run[top]) & (index <= top)
            low = int(index[same][numpy.argmin((way * turns)[same])])
            best, found = float(gained[top]), (low, top)
    return found


# endregion 🔖️Circle


# region 🔖️Stroke
def zone(across, down, width, height):
    """🧤️ Whether the pointer is in the zone of a stroke: over the body grown by the margin."""
    return (numpy.abs(across) <= width / 2 + STROKE_MARGIN) & (numpy.abs(down) <= height / 2 + STROKE_MARGIN)


def petting_seen(across, inside, width, tick):
    """🔬️ Whether a petting ends at a tick: in the unbroken stay over the body before it, two reversals with a counting stroke on either side lie within its window and the rests it allows."""
    out = numpy.flatnonzero(~inside[: tick + 1])
    start = max(int(out[-1]) + 1 if out.size else 0, tick - STROKE_WINDOW - 2 * STROKE_PAUSE)
    return len(extremes(across[start : tick + 1], 0.9 * STROKE_LENGTH * width)[0]) >= 2


def clean_petting(across, down, inside, width, span):
    """💯️ The tick by which a petting drawn between two ticks has to be cued, when the reading finds it clean; ``None`` otherwise.

    Clean: the pointer stays in the zone; there are at least six reversals of a prominence of 1.25 strokes and no smaller wiggle a reversal could be taken for; every stroke between two of them is long enough by a quarter, has its middle well over the body, wanders up and down by less than 0.8 of what is allowed and is neither among the slowest nor the fastest third of what counts; and any three strokes in a row fit the window by six ticks. The first reversal ends the run that led in; the three strokes after it count, so the cue comes before the fifth reversal."""
    begin, end = span
    if not inside[begin : end + 1].all():
        return None
    ticks, _ways = extremes(across[begin : end + 1], 1.25 * STROKE_LENGTH * width)
    if len(ticks) < 6 or len(extremes(across[begin : end + 1], 0.9 * STROKE_HYSTERESIS * width)[0]) != len(ticks):
        return None
    ticks = ticks + begin
    for first, second in zip(ticks[:-1], ticks[1:]):
        length = abs(across[second] - across[first])
        speed = length * TICKS_PER_SECOND / (second - first)
        rise = down[first : second + 1].max() - down[first : second + 1].min()
        if length < 1.25 * STROKE_LENGTH * width or abs(across[first] + across[second]) > 0.85 * width or rise > 0.8 * STROKE_SLANT * length or speed < 1.3 * STROKE_SLOW or speed > 0.75 * STROKE_FAST:
            return None
    if (ticks[3:] - ticks[:-3]).max() > STROKE_WINDOW - 6:
        return None
    return int(ticks[4])


# endregion 🔖️Stroke


# region 🔖️Shake
def along(xs, ys, quantum):
    """📏️ The grip along its principal axis, in pixels: the projection on the eigenvector of the largest eigenvalue of the covariance of its positions."""
    points = numpy.stack([xs, ys], axis=1) / quantum
    centred = points - points.mean(axis=0)
    if len(points) < 3:
        return centred[:, 0]
    _values, vectors = numpy.linalg.eigh(numpy.cov(centred.T))
    return centred @ vectors[:, -1]


def shake_seen(xs, ys, quantum, height, tick):
    """🧲️ Whether a shake ends at a tick: along one of twelve directions 15° apart, three reversals with a counting swing on either side — 0.8 of it, what a direction 7.5° off and a wobbling hand leave — lie within its window, the pause it allows and one swing more."""
    start = max(tick - SHAKE_WINDOW - SHAKE_PAUSE - 32, 0)
    angles = numpy.arange(12) * math.pi / 12
    lines = (numpy.stack([xs[start : tick + 1], ys[start : tick + 1]], axis=1) / quantum) @ numpy.stack([numpy.cos(angles), numpy.sin(angles)])
    return max(len(extremes(lines[:, index], 0.8 * SHAKE_AMPLITUDE * height)[0]) for index in range(len(angles))) >= 3


def clean_shake(xs, ys, quantum, height, span):
    """🏅️ The tick by which a shake between two ticks has to be cued, when the reading finds it clean; ``None`` otherwise.

    Clean: along the principal axis there are at least seven reversals of a prominence of 1.15 swings and no smaller wiggle a reversal could be taken for; two reversals in a row are 3 to 18 ticks apart; and every swing between them is long enough by 15 % and fast enough by 30 %. The first reversal may end a swing that does not count; the four after it do, so the cue comes before the sixth reversal."""
    begin, end = span
    line = along(xs[begin : end + 1], ys[begin : end + 1], quantum)
    ticks, _ways = extremes(line, 1.15 * SHAKE_AMPLITUDE * height)
    if len(ticks) < 7 or len(extremes(line, 0.9 * SHAKE_HYSTERESIS)[0]) != len(ticks):
        return None
    ticks = ticks + begin
    gaps = numpy.diff(ticks)
    lengths = numpy.hypot(numpy.diff(xs[ticks]), numpy.diff(ys[ticks])) / quantum
    if gaps.min() < 3 or gaps.max() > 18 or lengths.min() < 1.15 * SHAKE_AMPLITUDE * height or (lengths * TICKS_PER_SECOND / gaps).min() < 1.3 * SHAKE_SPEED:
        return None
    return int(ticks[5])


# endregion 🔖️Shake


# region 🔖️Judgement
def judged_hover(scenario, document, vector):
    """⚖️ The committed cues of a hover vector, after the reading has admitted them: a guard that silences the gesture leaves none, every cue ends a lap or a petting the libraries see, and a clean gesture is cued in time and in its direction."""
    name = "%s/%s" % (scenario, vector["id"])
    cues = [list(cue) for cue in vector["expected"]]
    if vector.get("guard") is not None:
        if cues:
            raise AssertionError("%s: a %s stage hears no hover gesture, the committed vector says %r" % (name, vector["guard"], cues))
        return cues
    xs, ys = trail(vector["path"], document["hold"])
    across, down, width, height = offsets(xs, ys, vector["body"], document["quantum"])
    turns, distance, live = winding(xs, ys, vector["body"], document["quantum"])
    inside = zone(across, down, width, height)
    for tick, cue in cues:
        if cue != vector["label"]:
            raise AssertionError("%s: a %s was answered with %s at tick %d" % (name, vector["label"], cue, tick))
        seen = petting_seen(across, inside, width, tick) if cue == "stroke" else lap_seen(turns, distance, live, tick, 1 if cue == "circle" else -1)
        if not seen:
            raise AssertionError("%s: the libraries find no %s that ends at tick %d" % (name, cue, tick))
    clean = clean_petting(across, down, inside, width, vector["span"]) if vector["label"] == "stroke" else clean_circle(turns, distance, live, vector["span"], vector["body"], document["quantum"])
    if (clean is not None) != vector["robust"]:
        raise AssertionError("%s: the libraries find the gesture %s, the committed vector says %s" % (name, "clean" if clean is not None else "not clean", "robust" if vector["robust"] else "not robust"))
    if clean is not None:
        direction, deadline = (vector["label"], clean) if vector["label"] == "stroke" else clean
        if direction != vector["label"]:
            raise AssertionError("%s: the libraries read a %s, the label says %s" % (name, direction, vector["label"]))
        if not cues or cues[0][0] > deadline:
            raise AssertionError("%s: a clean %s has to be cued by tick %d, the committed vector says %r" % (name, vector["label"], deadline, cues))
    return cues


def judged_held(document, vector):
    """🏋️ The committed cues of a held vector, after the reading has admitted them: a still stage hears nothing, every shake ends reversals the libraries see, a clean shake is cued in time, and carrying is no clean shake."""
    name = "shakes/%s" % vector["id"]
    cues = [list(cue) for cue in vector["expected"]]
    if vector.get("guard") == "still":
        if cues:
            raise AssertionError("%s: a still stage hears no shake, the committed vector says %r" % (name, cues))
        return cues
    xs, ys = trail(vector["path"], document["hold"])
    height = vector["height"] / document["quantum"]
    for tick, cue in cues:
        if cue != "shake" or vector["label"] != "shake":
            raise AssertionError("%s: a %s was answered with %s at tick %d" % (name, vector["label"], cue, tick))
        if not shake_seen(xs, ys, document["quantum"], height, tick):
            raise AssertionError("%s: the libraries find no shake that ends at tick %d" % (name, tick))
    clean = clean_shake(xs, ys, document["quantum"], height, vector["span"])
    if (clean is not None) != vector["robust"]:
        raise AssertionError("%s: the libraries find the path %s, the committed vector says %s" % (name, "a clean shake" if clean is not None else "no clean shake", "robust" if vector["robust"] else "not robust"))
    if clean is not None and (vector["label"] != "shake" or not cues or cues[0][0] > clean):
        raise AssertionError("%s: a clean shake has to be cued by tick %d, the committed vector says %s %r" % (name, clean, vector["label"], cues))
    return cues


def twins(scenario, produced, vectors):
    """👯️ Holds every vector that names a twin to it: a guard that does not concern the gesture changes nothing."""
    for vector in vectors:
        if vector.get("of") is not None and vector.get("guard") in ("control", "quiet", "scrolled") and scenario == "shakes" and produced[vector["id"]] != produced[vector["of"]]:
            raise AssertionError("%s/%s: the guard %s does not concern a shake, yet %r is not %r" % (scenario, vector["id"], vector["guard"], produced[vector["id"]], produced[vector["of"]]))
    return produced


def tally(vectors, floor, kind):
    """📊️ The detection of one kind of gesture over its four mirrored variants: a mirror keeps every tick and turns a circle the other way round, so each trace counts four times as its committed cues say; refused below the committed floor."""
    counted = [vector for vector in vectors if vector.get("guard") is None and vector["label"] != "carry"]
    detected = sum(1 for vector in counted if any(cue == vector["label"] for _tick, cue in vector["expected"]))
    if detected < floor * len(counted):
        raise AssertionError("detection: %d of %d %s were recognised, the floor is %r" % (detected, len(counted), kind, floor))
    return {"traces": 4 * len(counted), "detected": 4 * detected, "wrong": 0}


def hidden_gestures(document, travel, numbers):
    """🕵️ The clean gestures the reading finds in a stretch of ordinary travel, in some of its variants, past every body: for each body the widest sweep of the angle is judged as a circle, every stay over the body as a petting."""
    xs, ys = trail(travel["path"], document["hold"])
    found = []
    for number in numbers:
        turned = variant(xs, ys, document["page"], number)
        for index, box in enumerate(document["bodies"]):
            body = box_variant(box, document["page"], number)
            turns, distance, live = winding(turned[0], turned[1], body, document["quantum"])
            sweep = widest_sweep(turns, live)
            if sweep is not None and abs(turns[sweep[1]] - turns[sweep[0]]) >= CLEAN_TURNS and clean_circle(turns, distance, live, sweep, body, document["quantum"]) is not None:
                found.append([travel["id"], number, index, sweep[1], "circle"])
            across, down, width, height = offsets(turned[0], turned[1], body, document["quantum"])
            inside = zone(across, down, width, height)
            edges = numpy.flatnonzero(numpy.diff(numpy.concatenate([[False], inside, [False]]).astype(int)))
            for begin, end in zip(edges[0::2], edges[1::2]):
                if end - begin > 6 and clean_petting(across, down, inside, width, (int(begin), int(end) - 1)) is not None:
                    found.append([travel["id"], number, index, int(end) - 1, "stroke"])
    return len(xs) * len(numbers), found


def travelled(document, sample):
    """🧳️ What ordinary travel sets off: nothing — after the reading has found no clean gesture in it."""
    numbers = [0] if sample else list(range(document["variants"]))
    ticks = 0
    for travel in document["travels"]:
        if sample and not travel["sample"]:
            continue
        watched, found = hidden_gestures(document, travel, numbers)
        if found:
            raise AssertionError("travel/%s: the libraries find clean gestures in ordinary travel: %r" % (travel["id"], found[:5]))
        ticks += watched
    return {"pointerTicks": ticks, "petTicks": ticks * len(document["bodies"]), "cues": []}


# endregion 🔖️Judgement


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def close(produced, expected):
    """🔍️ Deep equality with floats compared within 1e-12."""
    if isinstance(produced, bool) or isinstance(expected, bool):
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return math.isclose(produced, expected, rel_tol=1e-12, abs_tol=1e-12)
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    return produced == expected


def agree(scenario, produced, vectors):
    """🤝️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def constants(ctx):
    """🎚️ The thresholds the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


def presses(ctx):
    """🖲️ The signals of every committed press."""
    document = committed(ctx)
    return agree("presses", {vector["id"]: press_signals(vector, document["quantum"]) for vector in document["presses"]}, document["presses"])


def warmth(ctx):
    """🌡️ The answers to every committed history of attention."""
    vectors = committed(ctx)["warmths"]
    return agree("warmth", {vector["id"]: warmth_answers(vector["caresses"]) for vector in vectors}, vectors)


def circles(ctx):
    """⭕️ The cues of every committed circle."""
    document = committed(ctx)
    return Outcome({vector["id"]: judged_hover("circles", document, vector) for vector in document["circles"]})


def strokes(ctx):
    """🐈️ The cues of every committed petting."""
    document = committed(ctx)
    return Outcome({vector["id"]: judged_hover("strokes", document, vector) for vector in document["strokes"]})


def shakes(ctx):
    """🫨️ The cues of every committed held path."""
    document = committed(ctx)
    return Outcome(twins("shakes", {vector["id"]: judged_held(document, vector) for vector in document["shakes"]}, document["shakes"]))


def detection(ctx):
    """📈️ How many deliberate gestures are recognised, over their mirrored variants."""
    document = committed(ctx)
    return Outcome({kind: tally(document[kind], document["floors"][kind], kind) for kind in ("circles", "strokes", "shakes")})


def travel_sample(ctx):
    """🚶️ The sample of ordinary travel sets off nothing."""
    return Outcome(travelled(committed(ctx), True))


def travel_hours(ctx):
    """⌛️ All committed ordinary travel, in every variant, sets off nothing."""
    return Outcome(travelled(committed(ctx), False))


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧩️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("constants", constants).oracle("presses", presses).oracle("warmth", warmth).oracle("circles", circles).oracle("strokes", strokes).oracle("shakes", shakes).oracle("detection", detection).oracle("travel-sample", travel_sample).oracle("travel-hours", travel_hours)


# endregion 🔖️Registration
