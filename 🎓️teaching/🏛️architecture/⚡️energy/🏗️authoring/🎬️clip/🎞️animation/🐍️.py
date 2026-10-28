"""🎞️ Own independent native drawable geometry at committed animation boundaries."""


def freeze_drawables(mobject):
    """📸 Preserve paint order and root identity while freezing unique drawable point families."""
    parts = []
    for member in mobject.family_members_with_points():
        part = member.copy().clear_updaters()
        part.submobjects = []
        parts.append(part)
    mobject.clear_updaters()
    mobject.clear_points()
    mobject.submobjects = parts
    return mobject
