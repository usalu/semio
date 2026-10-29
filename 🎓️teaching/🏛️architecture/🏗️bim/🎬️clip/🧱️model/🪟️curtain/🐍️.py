"""🪟 Source-authored curtain stations, planar panes and true native portal cuts."""
from copy import deepcopy
import math


def point_at(path,station):
    """📏 Evaluate cumulative distance without deriving member locations from curve facets."""
    if station< -1e-9:
        raise ValueError("Curtain station precedes its path")
    offset=0.
    for a,b in zip(path,path[1:]):
        length=math.dist(a,b)
        if station<=offset+length+1e-9:
            return [a[i]+(b[i]-a[i])*min(1.,max(0.,(station-offset)/length)) for i in (0,1)]
        offset+=length
    if abs(station-offset)<=1e-8:
        return list(path[-1])
    raise ValueError("Curtain station exceeds its path")


def path_slice(path,start,end):
    """✂️ Retain every real join between two finite cumulative stations."""
    points=[point_at(path,start)]
    offset=0.
    for a,b in zip(path,path[1:]):
        offset+=math.dist(a,b)
        if start+1e-9<offset<end-1e-9:
            points.append(list(b))
    points.append(point_at(path,end))
    return points


def resolve_paths(core,model,curtain):
    """🧭 Resolve explicit member stations separately from the source reference path."""
    result=[]
    seen=set()
    for record in curtain["geometry"]["glazing_paths"]:
        if record["tag"] in seen:
            raise ValueError("Curtain path tags must be unique")
        seen.add(record["tag"])
        path=core._expand(model,record["path"])
        length=sum(math.dist(a,b) for a,b in zip(path,path[1:]))
        stations=[s["position"]["value"] for s in record["stations"]]
        if len(stations)<2 or abs(stations[0])>1e-9 or abs(stations[-1]-length)>1e-8 or any(b<=a for a,b in zip(stations,stations[1:])):
            raise ValueError("Curtain member stations must partition their complete finite path")
        result.append({"tag":record["tag"],"reference_path":path,"path":[point_at(path,s) for s in stations],"stations":deepcopy(record["stations"]),"length":length})
    return result


def physical_geometry(core,model,curtain):
    """🏗️ Build disjoint native metal members and planar panes, subtracting actual hosted apertures."""
    paths=resolve_paths(core,model,curtain)
    spec=core._find(model,curtain["type"])["filling"]
    z0,z1=core._z(model,curtain["geometry"]["base_level"]),core._top(model,curtain["geometry"]["top_level"])
    width,depth,glass=[spec[k]["value"] for k in ("frame_width","frame_depth","panel_thickness")]
    if width<=0 or z1-z0<=2*width:
        raise ValueError("Curtain construction requires finite positive members and clear glazing height")
    rails=[(z0,z0+width,"sill"),(z1-width,z1,"head")]
    for transom in spec["transoms"]:
        height=z0+transom["height"]["value"]
        rails.append((height-width/2,height+width/2,"transom"))
    rails.sort()
    if any(a<z0-1e-9 or b>z1+1e-9 for a,b,k in rails) or any(b>c+1e-9 for (a,b,k),(c,d,j) in zip(rails,rails[1:])):
        raise ValueError("Curtain rails must be disjoint inside their host height")
    clear=[(b,c) for (a,b,k),(c,d,j) in zip(rails,rails[1:]) if c>b+1e-9]
    parts=[]
    for record in paths:
        path=record["path"]
        lengths=[math.dist(a,b) for a,b in zip(path,path[1:])]
        offsets=[0.]
        for length in lengths:
            offsets.append(offsets[-1]+length)
        if min(lengths)<=width:
            raise ValueError("Curtain members consume an entire pane bay")
        for lo,hi,kind in rails:
            parts.append({"outline":core._path_band(path,depth),"z0":lo,"z1":hi,"material":spec["frame_material"],"role":"frame","member_kind":kind,"curtain_path":record["tag"]})
        for i,s in enumerate(offsets):
            start,end=(0.,width) if i==0 else (s-width,s) if i==len(offsets)-1 else (s-width/2,s+width/2)
            outline=core._path_band(path_slice(path,start,end),depth)
            for lo,hi in clear:
                parts.append({"outline":outline,"z0":lo,"z1":hi,"material":spec["frame_material"],"role":"frame","member_kind":"jamb" if i in (0,len(offsets)-1) else "mullion","station":record["stations"][i]["tag"],"curtain_path":record["tag"]})
        for i,(a,b,length) in enumerate(zip(path,path[1:],lengths)):
            left=width if i==0 else width/2
            right=width if i==len(lengths)-1 else width/2
            start=[a[k]+(b[k]-a[k])*left/length for k in (0,1)]
            end=[b[k]-(b[k]-a[k])*right/length for k in (0,1)]
            for lo,hi in clear:
                parts.append({"outline":core._path_band([start,end],glass),"z0":lo,"z1":hi,"material":spec["panel_material"],"role":"glass","panel_index":i,"curtain_path":record["tag"]})
    openings=[e for e in model["elements"] if e["class"]=="IfcOpeningElement" and e["geometry"].get("curtain_wall")==curtain["tag"]]
    cutters=[p for opening in openings for p in core.opening_geometry(opening["tag"],model)["parts"]]
    owned=core._subtract_prisms(parts,cutters)
    opening_tags={e['tag'] for e in openings}
    fillings=[core.filling_geometry(e['tag'],model) for e in model['elements'] if e['class'] in ('IfcDoor','IfcWindow') and e['geometry'].get('opening') in opening_tags]
    before=sum(core._area(p['outline'])*(p['z1']-p['z0']) for p in owned)
    owned=core._subtract_prisms(owned,[p for filling in fillings for p in filling['parts']])
    displacement=before-sum(core._area(p['outline'])*(p['z1']-p['z0']) for p in owned)
    return {"tag":curtain["tag"],"guid":curtain["guid"],"class":curtain["class"],"storey":curtain["storey"],"paths":paths,"parts":owned,"base":z0,"top":z1,"opening_tags":[e["tag"] for e in openings],"hosted_filling_tags":[f['tag'] for f in fillings],"hosted_filling_displacement_volume":displacement}
