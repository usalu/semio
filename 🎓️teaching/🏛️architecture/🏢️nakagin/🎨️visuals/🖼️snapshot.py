"""🖼️ Immutable screen snapshots of vector drawings for whole-object presentation."""
import types
import numpy as np
from PIL import Image
from manim import Camera, ImageMobject, VMobject

class Snapshot(ImageMobject):
    """🖼️ A camera-aligned immutable raster presentation of vector geometry."""

MITER_REACH = 5.0


def region(drawing, camera):
    """📐 Integer pixel box ``(x0, y0, x1, y1)`` that provably holds every painted pixel of vector geometry: its projected points padded by the widest stroke times the miter reach; ``None`` for nothing paintable, the whole frame for anything unmeasurable."""
    low, high, widest = np.full(2, np.inf), np.full(2, -np.inf), 0.0
    for member in drawing.get_family():
        points = getattr(member, "points", None)
        if points is None or len(points) == 0:
            continue
        if not isinstance(member, VMobject):
            return 0, 0, camera.pixel_width, camera.pixel_height
        pixels = camera.points_to_subpixel_coords(member, points)
        if not np.all(np.isfinite(pixels)):
            return 0, 0, camera.pixel_width, camera.pixel_height
        low, high = np.minimum(low, pixels.min(axis=0)), np.maximum(high, pixels.max(axis=0))
        widest = max(widest, member.get_stroke_width(), member.get_stroke_width(background=True))
    if not np.all(np.isfinite(low)):
        return None
    pad = int(np.ceil(widest * camera.cairo_line_width_multiple * camera.pixel_width / camera.frame_width * MITER_REACH)) + 2
    x0, y0 = np.maximum(0, np.floor(low).astype(int) - pad)
    x1, y1 = np.minimum((camera.pixel_width, camera.pixel_height), np.ceil(high).astype(int) + pad)
    return (int(x0), int(y0), int(x1), int(y1)) if x1 > x0 and y1 > y0 else None


def render(drawing, camera, box):
    """🪟 Un-premultiplied RGBA pixels of ``drawing`` inside the integer pixel ``box`` of ``camera``, rendered by a camera that covers only that box with the same pixel grid."""
    x0, y0, x1, y1 = box
    width, height = x1 - x0, y1 - y0
    scale = camera.pixel_width / camera.frame_width, camera.pixel_height / camera.frame_height
    centre = np.array(camera.frame_center, dtype=float, copy=True)
    centre[0] += (x0 + width / 2 - camera.pixel_width / 2) / scale[0]
    centre[1] += (camera.pixel_height / 2 - y0 - height / 2) / scale[1]
    target = Camera(frame_center=centre, pixel_width=width, pixel_height=height, frame_width=width / scale[0], frame_height=height / scale[1], frame_rate=camera.frame_rate,
                    background_opacity=0, cairo_line_width_multiple=camera.cairo_line_width_multiple, use_z_index=camera.use_z_index)
    target.capture_mobjects([drawing])
    rgba = np.array(target.get_image())
    alpha = rgba[:, :, 3].astype(float)
    rgba[:, :, :3] = np.where(alpha[:, :, None] > 0, np.minimum(255, np.rint(rgba[:, :, :3].astype(float) * 255 / np.maximum(alpha[:, :, None], 1))), 0).astype(np.uint8)
    return rgba


def install(camera):
    """🔌 Teach a camera to draw fixed-frame snapshots, once."""
    if not hasattr(camera, "_snapshot_display"):
        original = camera.display_image_mobject
        def display(self, item, pixels):
            if not isinstance(item, Snapshot):
                return original(item, pixels)
            if item.pixel_array.shape[:2] != pixels.shape[:2] or not np.allclose(item.points, item.snapshot_points) or not np.allclose(self.frame_center, item.snapshot_center) or (self.frame_width, self.frame_height) != item.snapshot_frame:
                raise ValueError("A drawing snapshot requires its original fixed camera and frame alignment")
            self.overlay_PIL_image(pixels, Image.fromarray(item.get_pixel_array(), mode="RGBA"))
        camera._snapshot_display = original
        camera.display_image_mobject = types.MethodType(display, camera)


def capture(drawing, camera):
    """📸 Capture painter order at the camera's pixel and frame dimensions; only the pixels the drawing can touch are rendered."""
    rgba = np.zeros((camera.pixel_height, camera.pixel_width, 4), np.uint8)
    box = region(drawing, camera)
    if box is not None:
        x0, y0, x1, y1 = box
        rgba[y0:y1, x0:x1] = render(drawing, camera, box)
    image = Snapshot(rgba)
    image.stretch_to_fit_width(camera.frame_width).stretch_to_fit_height(camera.frame_height).move_to(camera.frame_center)
    image.snapshot_points = image.points.copy()
    image.snapshot_center = np.array(camera.frame_center, copy=True)
    image.snapshot_frame = (camera.frame_width, camera.frame_height)
    install(camera)
    return image
