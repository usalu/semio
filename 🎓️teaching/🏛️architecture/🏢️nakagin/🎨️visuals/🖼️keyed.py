"""🖼️ Stable keyed camera images for immutable drawing presentations."""
import importlib
import numpy as np
from PIL import Image

_engine = importlib.import_module("manim")
_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
_sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")


def _capture(drawing, camera):
    """📸 Composite ordered native vector runs and owned image sprites exactly."""
    members = camera.get_mobjects_to_display([drawing])
    if not any(isinstance(item, _engine.ImageMobject) for item in members):
        return _snapshot.capture(drawing, camera)
    frame = Image.new("RGBA", (camera.pixel_width, camera.pixel_height))
    vectors = []

    def flush():
        if not vectors:
            return
        target = _engine.Camera(frame_center=np.array(camera.frame_center, copy=True), pixel_width=camera.pixel_width, pixel_height=camera.pixel_height,
                                frame_width=camera.frame_width, frame_height=camera.frame_height, frame_rate=camera.frame_rate,
                                background_opacity=0, cairo_line_width_multiple=camera.cairo_line_width_multiple, use_z_index=camera.use_z_index)
        target.capture_mobjects(vectors, include_submobjects=False)
        rgba = np.array(target.get_image())
        alpha = rgba[:, :, 3].astype(float)
        rgba[:, :, :3] = np.where(alpha[:, :, None] > 0, np.minimum(255, np.rint(rgba[:, :, :3].astype(float) * 255 / np.maximum(alpha[:, :, None], 1))), 0).astype(np.uint8)
        frame.alpha_composite(Image.fromarray(rgba, mode="RGBA"))
        vectors.clear()

    for item in members:
        if not isinstance(item, _engine.ImageMobject):
            vectors.append(item)
            continue
        flush()
        if isinstance(item, _sprites.Sprite):
            frame.alpha_composite(_sprites.full_frame(item, camera))
        elif isinstance(item, _snapshot.Snapshot):
            if not np.allclose(item.points, item.snapshot_points) or not np.allclose(camera.frame_center, item.snapshot_center) or (camera.frame_width, camera.frame_height) != item.snapshot_frame:
                raise ValueError("A drawing snapshot requires its original fixed camera and frame alignment")
            frame.alpha_composite(Image.fromarray(item.get_pixel_array(), mode="RGBA"))
        else:
            raise TypeError("Only owned fixed-frame snapshots and sprites can be captured")
    flush()
    _snapshot.capture(_engine.VGroup(), camera)
    image = _snapshot.Snapshot(np.asarray(frame))
    image.stretch_to_fit_width(camera.frame_width).stretch_to_fit_height(camera.frame_height).move_to(camera.frame_center)
    image.snapshot_points = image.points.copy()
    image.snapshot_center = np.array(camera.frame_center, copy=True)
    image.snapshot_frame = (camera.frame_width, camera.frame_height)
    return image


class DrawingRef:
    """🧭 Retain a semantic drawing across copies of its presentation."""

    def __init__(self, value):
        self.value = value

    def __deepcopy__(self, memo):
        return self


def create(builder, key, camera, *, initial=None, retain=False, cache=False):
    """🖼️ Update one stable image from keyed native drawings without stale Cairo family members."""
    images = {}

    def obtain(value, supplied=None):
        if cache and value in images:
            return images[value]
        drawing = builder() if supplied is None else supplied
        result = (_capture(drawing, camera), drawing if retain else None)
        if cache:
            images[value] = result
        return result

    current = key()
    image, drawing = obtain(current, initial)
    group = _engine.Group(image.copy() if cache else image)
    group._snapshot_key = current
    if retain:
        group._drawing_ref = DrawingRef(drawing)

    def refresh(mob):
        value = key()
        if value != mob._snapshot_key:
            image, drawing = obtain(value)
            mob.submobjects[0].pixel_array = image.pixel_array.copy()
            if retain:
                mob._drawing_ref.value = drawing
            mob._snapshot_key = value

    group.add_updater(refresh)
    return group
