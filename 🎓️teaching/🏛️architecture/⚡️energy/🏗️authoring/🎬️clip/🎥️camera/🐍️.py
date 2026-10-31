"""🎥 Cache exact native two-dimensional Cairo paths behind the scene camera interface."""
from collections import OrderedDict
import weakref

from manim.camera.camera import Camera as _NativeCamera
from manim.mobject.types.vectorized_mobject import VMobject as _NativePath


def _method_key(method):
    """🔑 Compare the actual path-policy function without retaining its owning scene object."""
    function = getattr(method, '__func__', method)
    try:
        return weakref.ref(function)
    except TypeError:
        return None


_NATIVE_POLICY = tuple(weakref.ref(method) for method in (
    _NativePath.get_subpath_split_indices_from_points, _NativePath.consider_points_equals_2d,
    _NativeCamera.transform_points_pre_display))


class _CairoPathCamera(_NativeCamera):
    """🖌️ Reuse bounded weak-owned paths while preserving native fill/stroke order and alpha."""

    def __init__(self, *args, path_cache_entries=131072, path_cache_bytes=536870912, **kwargs):
        self._path_cache = OrderedDict()
        self._path_cache_size = 0
        self._path_cache_entries = max(0, int(path_cache_entries))
        self._path_cache_bytes = max(0, int(path_cache_bytes))
        self._path_cache_hits = self._path_cache_misses = 0
        super().__init__(*args, **kwargs)

    def _drop_path(self, reference):
        """🗑️ Release an evicted or collected object's accounted key and native path."""
        entry = self._path_cache.pop(reference, None)
        if entry is not None:
            self._path_cache_size -= entry[2]

    def set_cairo_context_path(self, context, mobject):
        """📐 Append only paths with byte-identical points, transforms and native path semantics."""
        points = mobject.points
        methods = tuple(_method_key(getattr(mobject, name)) for name in
                        ('get_subpath_split_indices_from_points', 'consider_points_equals_2d')) + (_method_key(self.transform_points_pre_display),)
        if not len(points) or not self._path_cache_entries or not self._path_cache_bytes or methods != _NATIVE_POLICY:
            self._drop_path(weakref.ref(mobject))
            return super().set_cairo_context_path(context, mobject)
        reference = weakref.ref(mobject)
        signature = (points.dtype.str, points.shape, points.tobytes(), tuple(context.get_matrix()),
                     mobject.n_points_per_cubic_curve, mobject.tolerance_for_point_equality, methods)
        entry = self._path_cache.get(reference)
        if entry is not None and entry[0] == signature:
            self._path_cache.move_to_end(reference)
            self._path_cache_hits += 1
            context.new_path()
            context.append_path(entry[1])
            return self
        self._path_cache_misses += 1
        self._drop_path(reference)
        super().set_cairo_context_path(context, mobject)
        path = context.copy_path()
        size = len(signature[2]) + 2048 + sum(64 + 8*len(coordinates) for _, coordinates in path)
        if size <= self._path_cache_bytes:
            owner = weakref.ref(self)
            def collected(reference):
                """🧹 Remove an owned path without retaining the camera through the callback."""
                camera = owner()
                if camera is not None:
                    camera._drop_path(reference)
            reference = weakref.ref(mobject, collected)
            self._path_cache[reference] = (signature, path, size)
            self._path_cache_size += size
            while len(self._path_cache) > self._path_cache_entries or self._path_cache_size > self._path_cache_bytes:
                self._drop_path(next(iter(self._path_cache)))
        return self
