"""🧸 Cropped camera sprites for moving actual projected capsule geometry."""
import importlib
import types
import numpy as np
from PIL import Image
from manim import ImageMobject

_snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")

class Sprite(ImageMobject):
    """🧸 A camera-aligned projected capsule that can translate and scale."""

def full_frame(sprite, camera):
    """🖼️ Place an axis-aligned sprite into the camera's pixel frame."""
    corners = camera.points_to_subpixel_coords(sprite, sprite.points)
    if not np.allclose(corners[:, 0], [corners[0, 0], corners[1, 0], corners[0, 0], corners[1, 0]], atol=1e-5) or not np.allclose(corners[:, 1], [corners[0, 1], corners[0, 1], corners[2, 1], corners[2, 1]], atol=1e-5):
        raise ValueError("Capsule sprites support translation and axis-aligned scale only")
    x0, y0 = np.rint(corners[0]).astype(int)
    x1, y1 = np.rint(corners[3]).astype(int)
    frame = Image.new("RGBA", (camera.pixel_width, camera.pixel_height))
    if x1 > x0 and y1 > y0:
        part = Image.fromarray(sprite.get_pixel_array(), mode="RGBA")
        if part.size != (x1 - x0, y1 - y0):
            part = part.resize((x1 - x0, y1 - y0), Image.Resampling.BICUBIC)
        frame.paste(part, (int(x0), int(y0)))
    return frame

def capture(drawing, camera):
    """📸 Capture actual capsule vectors at their current camera pixel bounds."""
    _snapshot.install(camera)
    box = _snapshot.region(drawing, camera)
    full = np.zeros((camera.pixel_height, camera.pixel_width, 4), np.uint8)
    if box is not None:
        full[box[1]:box[3], box[0]:box[2]] = _snapshot.render(drawing, camera, box)
    rows, cols = np.nonzero(full[:, :, 3])
    if len(rows):
        x0, x1, y0, y1 = int(cols.min()), int(cols.max()) + 1, int(rows.min()), int(rows.max()) + 1
    else:
        x0 = y0 = 0
        x1 = y1 = 1
    image = Sprite(full[y0:y1, x0:x1].copy())
    image.stretch_to_fit_width((x1 - x0) * camera.frame_width / camera.pixel_width)
    image.stretch_to_fit_height((y1 - y0) * camera.frame_height / camera.pixel_height)
    centre = np.array([camera.frame_center[0] + ((x0 + x1) / (2 * camera.pixel_width) - .5) * camera.frame_width,
                       camera.frame_center[1] + (.5 - (y0 + y1) / (2 * camera.pixel_height)) * camera.frame_height,
                       camera.frame_center[2]])
    image.move_to(centre if len(rows) else drawing.get_center())
    if not hasattr(camera, "_sprite_display"):
        original = camera.display_image_mobject
        def display(self, item, pixels):
            if isinstance(item, Sprite):
                self.overlay_PIL_image(pixels, full_frame(item, self))
            else:
                original(item, pixels)
        camera._sprite_display = original
        camera.display_image_mobject = types.MethodType(display, camera)
    return image
