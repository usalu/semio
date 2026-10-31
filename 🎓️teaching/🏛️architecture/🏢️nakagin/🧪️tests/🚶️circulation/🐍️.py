"""🚶 Physical support connectivity from authored solids, independently checked with NumPy and NetworkX."""
from collections import deque
import importlib.util
import json
import math
from pathlib import Path
import sys
import unittest

import networkx as nx
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
CASES = json.loads((ROOT/'🧪️cases/🚶️circulation/🔣️.json').read_text(encoding='utf8'))
EPS = CASES['tolerance']
spec = importlib.util.spec_from_file_location('nakagin_circulation', ROOT/'🐍️.py')
MODEL = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = MODEL
spec.loader.exec_module(MODEL)


def adjacent(a,b):
    """🤝 Actual convex support contact requires minimum passage width and a normal riser."""
    ax0,ax1,ay0,ay1 = a['rect']
    bx0,bx1,by0,by1 = b['rect']
    dx,dy = min(ax1,bx1)-max(ax0,bx0),min(ay1,by1)-max(ay0,by0)
    broad = abs(a['z']-b['z']) <= CASES['max_riser'] and min(dx,dy)>=-EPS
    if not broad:
        return False
    return _shared_width(a,b)>=CASES['min_passage_width']-EPS if 'polygon' in a or 'polygon' in b else max(dx,dy)>=CASES['min_passage_width']-EPS


def support_graph(nodes):
    """🕸 Build a stdlib support graph without any centre-distance or semantic-key shortcuts."""
    graph = {key:set() for key in nodes}
    rows = list(nodes.items())
    for i,(ka,a) in enumerate(rows):
        for kb,b in rows[i+1:]:
            if adjacent(a,b):
                graph[ka].add(kb)
                graph[kb].add(ka)
    return graph


def reachable(graph,seeds):
    """🧭 Traverse only the geometrically supported walking edges."""
    seen,queue = set(seeds),deque(seeds)
    while queue:
        for neighbor in graph[queue.popleft()]-seen:
            seen.add(neighbor)
            queue.append(neighbor)
    return seen


def oracle_graph(nodes):
    """🔬 Independent NumPy clipping and NetworkX validate every convex support edge."""
    keys = list(nodes)
    data = np.asarray([(*nodes[key]['rect'],nodes[key]['z']) for key in keys])
    ox = np.minimum(data[:,None,1],data[None,:,1])-np.maximum(data[:,None,0],data[None,:,0])
    oy = np.minimum(data[:,None,3],data[None,:,3])-np.maximum(data[:,None,2],data[None,:,2])
    dz = np.abs(data[:,None,4]-data[None,:,4])
    links = (np.minimum(ox,oy)>=-EPS)&(np.maximum(ox,oy)>=CASES['min_passage_width']-EPS)&(dz<=CASES['max_riser'])
    polygon_ids = [i for i,key in enumerate(keys) if 'polygon' in nodes[key]]
    for i in polygon_ids:
        candidates = np.flatnonzero((ox[i]>=-EPS)&(oy[i]>=-EPS)&(dz[i]<=CASES['max_riser']))
        for j in candidates:
            links[i,j] = links[j,i] = _numpy_shared_width(nodes[keys[i]],nodes[keys[j]])>=CASES['min_passage_width']-EPS
    graph = nx.Graph()
    graph.add_nodes_from(keys)
    graph.add_edges_from((keys[i],keys[j]) for i,j in zip(*np.where(np.triu(links,1))))
    return graph


def top_rectangles(parts):
    """📐 Extract actual upward convex faces, preserving clipped corners rather than filling bounding boxes."""
    nodes = {}
    for part in parts:
        if part.role not in CASES['support_roles']:
            continue
        vertices = np.asarray(part.solid.vertices)
        z = vertices[:,2].max()
        for index,face in enumerate(part.solid.faces):
            points = vertices[list(face)]
            if len(face)<3 or not np.allclose(points[:,2],z,atol=EPS,rtol=0):
                continue
            xy = points[:,:2]
            area = np.sum(xy[:,0]*np.roll(xy[:,1],-1)-xy[:,1]*np.roll(xy[:,0],-1))
            if area<=EPS:
                continue
            edges = np.roll(xy,-1,axis=0)-xy
            turns = edges[:,0]*np.roll(edges[:,1],-1)-edges[:,1]*np.roll(edges[:,0],-1)
            if any(turn < -EPS for turn in turns):
                raise ValueError('Walking support requires convex decomposition: '+part.key)
            x0,y0 = xy.min(axis=0)
            x1,y1 = xy.max(axis=0)
            corners = {(round(float(x),6),round(float(y),6)) for x,y in xy}
            expected = {(round(float(x),6),round(float(y),6)) for x in (x0,x1) for y in (y0,y1)}
            key = part.key if part.key not in nodes else f'{part.key}#{index}'
            node = {'rect':(float(x0),float(x1),float(y0),float(y1)),'z':float(z),'role':part.role}
            if corners!=expected:
                node['polygon'] = tuple(tuple(map(float,p)) for p in xy)
            route = part.solid.meta.get('walking_route')
            if route:
                node['walking_route'] = route
            axis = part.solid.meta.get('walking_axis')
            if axis in ('x','y'):
                node['walking_axis'] = axis
            nodes[key] = node
    return nodes


def intersects(a,b):
    """🧊 Conservative positive-volume overlap between a walking prism and an actual-solid bounding box."""
    return all(min(a[1][k],b[1][k])-max(a[0][k],b[0][k])>EPS for k in range(3))


def sub(a,b):
    return tuple(a[k]-b[k] for k in range(3))


def dot(a,b):
    return sum(x*y for x,y in zip(a,b))


def cross(a,b):
    return (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])


def face_triangles(vertices,face):
    """✂️ Ear-clip a simple planar face so concave outlines are never replaced by a filled bounding polygon."""
    ids = list(face)
    normal = None
    for i in range(1,len(ids)-1):
        candidate = cross(sub(vertices[ids[i]],vertices[ids[0]]),sub(vertices[ids[i+1]],vertices[ids[0]]))
        if dot(candidate,candidate)>1e-18:
            normal = candidate
            break
    if normal is None:
        return []
    drop = max(range(3),key=lambda k:abs(normal[k]))
    xy = {i:tuple(vertices[i][k] for k in range(3) if k!=drop) for i in ids}
    turn = lambda a,b,c:(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
    area = sum(xy[a][0]*xy[b][1]-xy[b][0]*xy[a][1] for a,b in zip(ids,ids[1:]+ids[:1]))
    sign = 1 if area>0 else -1
    triangles = []
    while len(ids)>3:
        for n,b in enumerate(ids):
            a,c = ids[n-1],ids[(n+1)%len(ids)]
            if sign*turn(xy[a],xy[b],xy[c])<=1e-12:
                continue
            if any(all(sign*turn(xy[u],xy[v],xy[p])>=-1e-12 for u,v in ((a,b),(b,c),(c,a))) for p in ids if p not in (a,b,c)):
                continue
            triangles.append(tuple(vertices[k] for k in (a,b,c)))
            del ids[n]
            break
        else:
            raise ValueError('Non-simple or degenerate physical face')
    triangles.append(tuple(vertices[k] for k in ids))
    return triangles


def triangles_of(solid):
    return tuple(triangle for face in solid.faces for triangle in face_triangles(solid.vertices,face))


def triangle_hits_box(triangle,box):
    """🔺 Triangle/prism SAT rejects broad-phase boxes that merely enclose empty sloping or concave space."""
    basis = ((1,0,0),(0,1,0),(0,0,1))
    edges = [sub(triangle[(i+1)%3],triangle[i]) for i in range(3)]
    axes = list(basis)+[cross(edges[0],edges[1])]+[cross(edge,axis) for edge in edges for axis in basis]
    corners = [(x,y,z) for x in (box[0][0],box[1][0]) for y in (box[0][1],box[1][1]) for z in (box[0][2],box[1][2])]
    for axis in axes:
        length = math.sqrt(dot(axis,axis))
        if length<1e-12:
            continue
        direction = tuple(v/length for v in axis)
        a,b = [dot(p,direction) for p in triangle],[dot(p,direction) for p in corners]
        if max(a)<=min(b)+EPS or max(b)<=min(a)+EPS:
            return False
    return True


def inside_mesh(point,triangles):
    """🎯 A non-axis-aligned ray classifies containment when a walking prism never crosses a mesh surface."""
    direction = (1,.371119,.529733)
    hits = set()
    for a,b,c in triangles:
        e1,e2 = sub(b,a),sub(c,a)
        h = cross(direction,e2)
        determinant = dot(e1,h)
        if abs(determinant)<1e-12:
            continue
        q = sub(point,a)
        u = dot(q,h)/determinant
        v = dot(direction,cross(q,e1))/determinant
        distance = dot(e2,cross(q,e1))/determinant
        if u>=-1e-10 and v>=-1e-10 and u+v<=1+1e-10 and distance>EPS:
            hits.add(round(distance,7))
    return len(hits)%2==1


def mesh_hits_box(triangles,box):
    if any(triangle_hits_box(triangle,box) for triangle in triangles):
        return True
    return inside_mesh(tuple((a+b)/2 for a,b in zip(*box)),triangles)


def numpy_mesh_hits_box(triangles,box):
    """🧮 Independent NumPy polygon clipping and oriented solid angles validate the mesh/prism oracle."""
    lower,upper = np.asarray(box[0])+EPS,np.asarray(box[1])-EPS
    for triangle in triangles:
        polygon = [np.asarray(p,dtype=float) for p in triangle]
        for axis in range(3):
            for boundary,side in ((lower[axis],1),(upper[axis],-1)):
                clipped = []
                for a,b in zip(polygon,polygon[1:]+polygon[:1]):
                    da,db = side*(a[axis]-boundary),side*(b[axis]-boundary)
                    if da>=0:
                        clipped.append(a)
                    if (da>=0)!=(db>=0):
                        clipped.append(a+(b-a)*da/(da-db))
                polygon = clipped
        if len(polygon)>=3 and np.linalg.norm(sum((np.cross(polygon[i]-polygon[0],polygon[i+1]-polygon[0]) for i in range(1,len(polygon)-1)),np.zeros(3)))>1e-10:
            return True
    p = np.asarray(triangles)-np.mean(box,axis=0)
    a,b,c = p[:,0],p[:,1],p[:,2]
    la,lb,lc = (np.linalg.norm(v,axis=1) for v in (a,b,c))
    determinant = np.einsum('ij,ij->i',a,np.cross(b,c))
    denominator = la*lb*lc+np.einsum('ij,ij->i',a,b)*lc+np.einsum('ij,ij->i',b,c)*la+np.einsum('ij,ij->i',c,a)*lb
    return bool(abs(np.sum(2*np.arctan2(determinant,denominator)))>2*np.pi)


def walking_strips(node,capsule=False):
    """📏 Continuous centre strips cover flat connectors and both directions through square turning corners."""
    x0,x1,y0,y1 = node['rect']
    z = node['z']
    radius = CASES['min_passage_width']/2
    height = CASES['capsule_door_headroom'] if capsule else CASES['interior_headroom']
    if node.get('walking_route'):
        return [oriented_strip(a,b,z,radius,height) for a,b in zip(node['walking_route'],node['walking_route'][1:])]
    strips = []
    axis = node.get('walking_axis')
    if axis=='x' or axis is None and x1-x0>=y1-y0-EPS:
        middle = (y0+y1)/2
        strips.append(((x0,middle-radius,z+CASES['surface_tolerance']),(x1,middle+radius,z+height-EPS)))
    if axis=='y' or axis is None and y1-y0>=x1-x0-EPS:
        middle = (x0+x1)/2
        strips.append(((middle-radius,y0,z+CASES['surface_tolerance']),(middle+radius,y1,z+height-EPS)))
    return strips


def stair_center_paths(node):
    """🪜 Continuous narrow centreline tubes follow every actual tread run through its required headroom."""
    x0,x1,y0,y1 = node['rect']
    z,radius = node['z'],CASES['stair_center_tube']/2
    low,high = z+CASES['surface_tolerance'],z+CASES['interior_headroom']-EPS
    if x1-x0<y1-y0:
        middle = (y0+y1)/2
        return [((x0,middle-radius,low),(x1,middle+radius,high))]
    middle = (x0+x1)/2
    return [((middle-radius,y0,low),(middle+radius,y1,high))]


def physical_barriers(parts):
    """🏗️ Collect every physical structural mesh and conservative capsule envelope without a role allowlist."""
    barriers = [(part.key,part.role,tuple(tuple(fn(p[k] for p in part.solid.vertices) for k in range(3)) for fn in (min,max)),triangles_of(part.solid)) for part in parts]
    for slot in MODEL.slots():
        solid = MODEL.slot_solid(slot)
        bounds = tuple(tuple(fn(p[k] for p in solid.vertices) for k in range(3)) for fn in (min,max))
        barriers.append(('capsule-envelope-'+str((*slot.path,slot.item)),'capsule_envelope',bounds,triangles_of(solid)))
    return barriers


def headroom_conflicts(parts,nodes,mode="all",barriers=None):
    """🚧 Test the actual-mesh clearance of flat body strips and stair centreline paths."""
    barriers = physical_barriers(parts) if barriers is None else barriers
    found = []
    for key,node in nodes.items():
        if node['role']=='stair_tread' and mode!='flat':
            paths = stair_center_paths(node)
        elif node['role'] in ('landing','bridge_approach','ground_threshold','office_threshold','lift_approach','bridge','bridge_return') and mode!='stairs':
            paths = walking_strips(node,'/capsule-landing-' in key)
        else:
            continue
        for prism in paths:
            world_prism = prism['bounds'] if isinstance(prism,dict) else prism
            for other,role,bounds,triangles in barriers:
                if role=="stair_tread" and other in nodes and adjacent(node,nodes[other]) and 0<nodes[other]["z"]-node["z"]<=CASES["max_riser"]:
                    continue
                if not intersects(world_prism,bounds):
                    continue
                local = tuple(tuple(local_point(p,prism) for p in triangle) for triangle in triangles) if isinstance(prism,dict) else triangles
                query = prism['box'] if isinstance(prism,dict) else prism
                if mesh_hits_box(local,query):
                    found.append({'support':key,'obstruction':other,'obstruction_role':role,'walking_prism':prism,'obstacle_bounds':bounds})
    return found


def snapshot(parts=None):
    """🏢 Build one fresh full structural assembly so geometry edits cannot be hidden by a hand-authored graph."""
    parts = MODEL.structure_assembly('full').parts if parts is None else parts
    nodes = top_rectangles(parts)
    graph = support_graph(nodes)
    seeds = {key for key,node in nodes.items() if node['role']=='ground_threshold' and abs(node['z'])<=EPS}
    return nodes,graph,seeds


def oriented_strip(a,b,z,radius,height):
    """🧭 A continuous oriented prism follows a diagonal route without filling its exterior bounding corners."""
    length = math.dist(a,b)
    u = tuple((b[k]-a[k])/length for k in range(2))
    v = (-u[1],u[0])
    box = ((0,-radius,z+CASES['surface_tolerance']),(length,radius,z+height-EPS))
    corners = [(a[0]+u[0]*x+v[0]*y,a[1]+u[1]*x+v[1]*y,h) for x in (0,length) for y in (-radius,radius) for h in (box[0][2],box[1][2])]
    bounds = tuple(tuple(fn(p[k] for p in corners) for k in range(3)) for fn in (min,max))
    return {'box':box,'origin':a,'axes':(u,v),'bounds':bounds}


def local_point(point,prism):
    """🔄 Rigidly transform an obstacle into the walking prism frame for exact triangle SAT."""
    offset = tuple(point[k]-prism['origin'][k] for k in range(2))
    return (*(sum(offset[k]*axis[k] for k in range(2)) for axis in prism['axes']),point[2])


def polygon_area(points):
    """📊 Signed-edge polygon area measures residual unsupported walking footprints."""
    return abs(sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(points,points[1:]+points[:1])))/2


def clip_halfplane(points,a,b,inside=True):
    """✂️ Split a convex footprint on an actual support edge without expanding its geometry."""
    result = []
    sign = 1 if inside else -1
    distance = lambda p:sign*((b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0]))
    for p,q in zip(points,points[1:]+points[:1]):
        dp,dq = distance(p),distance(q)
        if dp>=-1e-10:
            result.append(p)
        if (dp>=0)!=(dq>=0):
            result.append(tuple(p[k]+(q[k]-p[k])*dp/(dp-dq) for k in range(2)))
    return result


def unsupported_area(query,supports):
    """🧩 Subtract real convex supports from a walking footprint, retaining every uncovered fragment."""
    remaining = [list(query)]
    for support in supports:
        outside = []
        for piece in remaining:
            inside = piece
            for a,b in zip(support,support[1:]+support[:1]):
                fragment = clip_halfplane(inside,a,b,False)
                if polygon_area(fragment)>1e-12:
                    outside.append(fragment)
                inside = clip_halfplane(inside,a,b)
                if not inside:
                    break
        remaining = outside
    return sum(polygon_area(piece) for piece in remaining)


def numpy_supported_area(query,supports):
    """🔬 Independent NumPy clipping and vertical interval integration measure the support union."""
    polygons = []
    boundary = np.asarray(query,dtype=float)
    for support in supports:
        rows = np.asarray(support,dtype=float)
        for start,end in zip(boundary,np.roll(boundary,-1,axis=0)):
            if not len(rows):
                break
            direction = end-start
            offsets = rows-start
            distances = direction[0]*offsets[:,1]-direction[1]*offsets[:,0]
            following = np.roll(rows,-1,axis=0)
            next_distances = np.roll(distances,-1)
            clipped = []
            for i in range(len(rows)):
                if distances[i]>=-1e-10:
                    clipped.append(rows[i])
                if (distances[i]>=0)!=(next_distances[i]>=0):
                    clipped.append(rows[i]+(following[i]-rows[i])*distances[i]/(distances[i]-next_distances[i]))
            rows = np.asarray(clipped)
        if len(rows)>=3:
            polygons.append(rows)
    if not polygons:
        return 0
    starts = np.concatenate(polygons)
    ends = np.concatenate([np.roll(poly,-1,axis=0) for poly in polygons])
    directions = ends-starts
    r,s = directions[:,None,:],directions[None,:,:]
    offsets = starts[None,:,:]-starts[:,None,:]
    determinant = r[:,:,0]*s[:,:,1]-r[:,:,1]*s[:,:,0]
    safe = np.where(abs(determinant)>1e-12,determinant,1)
    t = (offsets[:,:,0]*s[:,:,1]-offsets[:,:,1]*s[:,:,0])/safe
    u = (offsets[:,:,0]*r[:,:,1]-offsets[:,:,1]*r[:,:,0])/safe
    valid = (abs(determinant)>1e-12)&(t>=0)&(t<=1)&(u>=0)&(u<=1)
    crossings = (starts[:,None,0]+t*directions[:,None,0])[valid]
    cuts = sorted(set(map(float,np.concatenate((starts[:,0],crossings)))))
    area = 0
    for left,right in zip(cuts,cuts[1:]):
        if right-left<1e-12:
            continue
        x = (left+right)/2
        bands = []
        for polygon in polygons:
            end = np.roll(polygon,-1,axis=0)
            selected = (np.minimum(polygon[:,0],end[:,0])<x)&(np.maximum(polygon[:,0],end[:,0])>x)
            a,b = polygon[selected],end[selected]
            if len(a)>=2:
                ys = a[:,1]+(b[:,1]-a[:,1])*(x-a[:,0])/(b[:,0]-a[:,0])
                bands.append((float(ys.min()),float(ys.max())))
        covered,low,high = 0,None,None
        for a,b in sorted(bands):
            if low is None:
                low,high = a,b
            elif a<=high+1e-10:
                high = max(high,b)
            else:
                covered += high-low
                low,high = a,b
        if low is not None:
            covered += high-low
        area += (right-left)*covered
    return area


def strip_footprint(prism):
    """👣 Recover the exact horizontal body footprint of an oriented continuous walking prism."""
    lower,upper = prism['box']
    a,(u,v) = prism['origin'],prism['axes']
    return [tuple(a[k]+u[k]*x+v[k]*y for k in range(2)) for x,y in ((lower[0],lower[1]),(upper[0],lower[1]),(upper[0],upper[1]),(lower[0],upper[1]))]


def _polygon(node):
    x0,x1,y0,y1 = node['rect']
    return node.get('polygon',((x0,y0),(x1,y0),(x1,y1),(x0,y1)))


def _shared_width(a,b):
    points = list(_polygon(a))
    clip = _polygon(b)
    for p,q in zip(clip,clip[1:]+clip[:1]):
        clipped = []
        distance = lambda v:(q[0]-p[0])*(v[1]-p[1])-(q[1]-p[1])*(v[0]-p[0])
        for u,v in zip(points,points[1:]+points[:1]):
            du,dv = distance(u),distance(v)
            if du>=-EPS:
                clipped.append(u)
            if (du>=0)!=(dv>=0):
                clipped.append(tuple(u[k]+(v[k]-u[k])*du/(du-dv) for k in range(2)))
        points = clipped
    return max((math.dist(p,q) for p in points for q in points),default=0)


def _numpy_shared_width(a,b):
    rows = np.asarray(_polygon(a),dtype=float)
    boundary = np.asarray(_polygon(b),dtype=float)
    for start,end in zip(boundary,np.roll(boundary,-1,axis=0)):
        if not len(rows):
            return 0
        direction = end-start
        offsets = rows-start
        distance = direction[0]*offsets[:,1]-direction[1]*offsets[:,0]
        following = np.roll(rows,-1,axis=0)
        next_distance = np.roll(distance,-1)
        result = []
        for index in range(len(rows)):
            if distance[index]>=-EPS:
                result.append(rows[index])
            if (distance[index]>=0)!=(next_distance[index]>=0):
                result.append(rows[index]+(following[index]-rows[index])*distance[index]/(distance[index]-next_distance[index]))
        rows = np.asarray(result)
    return float(np.linalg.norm(rows[:,None]-rows[None,:],axis=2).max()) if len(rows) else 0


class CirculationContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        print('[DEBUG] Building circulation support graph',flush=True)
        cls.parts = MODEL.structure_assembly("full").parts
        cls.nodes,cls.graph,cls.seeds = snapshot(cls.parts)
        cls.reached = reachable(cls.graph,cls.seeds)
        cls.barriers = physical_barriers(cls.parts)

    def test_portable_support_cases(self):
        for case in CASES['cases']:
            with self.subTest(case=case['id']):
                nodes = {key:{'rect':rect,'z':z} for key,rect,z in case['nodes']}
                graph = support_graph(nodes)
                expected = set(case['reachable'])
                self.assertEqual(reachable(graph,case['ground']),expected)
                oracle = oracle_graph(nodes)
                self.assertEqual(set().union(*(nx.node_connected_component(oracle,key) for key in case['ground'])),expected)

    def test_portable_convex_support_contact_matches_numpy(self):
        for case in CASES['convex_support_cases']:
            with self.subTest(case=case['id']):
                self.assertEqual(adjacent(case['a'],case['b']),case['adjacent'])
                self.assertEqual(_numpy_shared_width(case['a'],case['b'])>=CASES['min_passage_width']-EPS,case['adjacent'])

    def test_portable_oriented_clearance_matches_numpy(self):
        for case in CASES['oriented_clearance_cases']:
            with self.subTest(case=case['id']):
                prism = oriented_strip(case['start'],case['end'],0,.3,2.1)
                solid = MODEL._box(*case['obstacle'])
                local = tuple(tuple(local_point(p,prism) for p in triangle) for triangle in triangles_of(solid))
                matrix = np.asarray(prism['axes'])
                vertices = np.asarray(solid.vertices)
                oracle_xy = (vertices[:,:2]-np.asarray(prism['origin']))@matrix.T
                self.assertTrue(np.allclose([local_point(p,prism)[:2] for p in vertices],oracle_xy))
                self.assertEqual(mesh_hits_box(local,prism['box']),case['blocked'])
                self.assertEqual(numpy_mesh_hits_box(local,prism['box']),case['blocked'])

    def test_portable_support_union_coverage_matches_numpy(self):
        for case in CASES['support_coverage_cases']:
            with self.subTest(case=case['id']):
                uncovered = unsupported_area(case['query'],case['supports'])
                self.assertAlmostEqual(uncovered,case['unsupported_area'])
                self.assertAlmostEqual(polygon_area(case['query'])-numpy_supported_area(case['query'],case['supports']),case['unsupported_area'])

    def test_every_oriented_walking_footprint_has_actual_continuous_support(self):
        for key,node in self.nodes.items():
            if not node.get('walking_route'):
                continue
            supports = [_polygon(other) for other in self.nodes.values() if abs(other['z']-node['z'])<EPS]
            for prism in walking_strips(node):
                footprint = strip_footprint(prism)
                self.assertLess(unsupported_area(footprint,supports),1e-7,key)
                self.assertAlmostEqual(numpy_supported_area(footprint,supports),polygon_area(footprint),places=7,msg=key)

    def test_office_thresholds_receive_actual_mesh_headroom_checks(self):
        for case in CASES['office_headroom_cases']:
            with self.subTest(case=case['id']):
                solid = MODEL.Solid(tuple(tuple(p) for p in case['vertices']),tuple(tuple(f) for f in case['faces']),())
                triangles = triangles_of(solid)
                bounds = tuple(tuple(fn(p[k] for p in solid.vertices) for k in range(3)) for fn in (min,max))
                barriers = [('header','test_header',bounds,triangles)]
                found = headroom_conflicts([],{'office':case['node']},mode='flat',barriers=barriers)
                self.assertEqual(bool(found),case['blocked'])
                self.assertEqual(any(numpy_mesh_hits_box(triangles,query) for query in walking_strips(case['node'])),case['blocked'])

    def test_portable_mesh_clearance_matches_numpy(self):
        for case in CASES['mesh_clearance_cases']:
            with self.subTest(case=case['id']):
                solid = MODEL.Solid(tuple(tuple(p) for p in case['vertices']),tuple(tuple(f) for f in case['faces']),())
                triangles = triangles_of(solid)
                self.assertEqual(mesh_hits_box(triangles,case['query']),case['blocked'])
                self.assertEqual(numpy_mesh_hits_box(triangles,case['query']),case['blocked'])

    def test_portable_walking_strip_directions(self):
        for case in CASES['walking_strip_cases']:
            with self.subTest(case=case['id']):
                self.assertTrue(np.allclose(walking_strips(case['node']),case['expected'],atol=EPS,rtol=0))

    def test_portable_stair_centre_paths(self):
        for case in CASES['stair_center_path_cases']:
            with self.subTest(case=case['id']):
                self.assertTrue(np.allclose(stair_center_paths(case['node']),case['expected'],atol=EPS,rtol=0))

    def test_portable_headroom_boxes_match_numpy(self):
        for case in CASES['clearance_cases']:
            with self.subTest(case=case['id']):
                expected = case['blocked']
                self.assertEqual([key for key,lo,hi in case['boxes'] if intersects(case['query'],(lo,hi))],expected)
                query = np.asarray(case['query'])
                self.assertEqual([key for key,lo,hi in case['boxes'] if np.all(np.minimum(query[1],hi)-np.maximum(query[0],lo)>EPS)],expected)

    def test_ground_seeds_are_the_two_authored_exterior_thresholds(self):
        self.assertEqual(len(self.seeds),CASES['terminals']['ground_threshold'])
        self.assertTrue(all(self.nodes[key]['role']=='ground_threshold' and abs(self.nodes[key]['z'])<=EPS for key in self.seeds))

    def test_every_authored_terminal_is_ground_reachable(self):
        for role,count in CASES['terminals'].items():
            targets = {key for key,node in self.nodes.items() if node['role']==role}
            self.assertEqual(len(targets),count,role)
            self.assertEqual(sorted(targets-self.reached),[],role)

    def test_new_flat_connectors_have_continuous_walking_headroom(self):
        failures = headroom_conflicts(self.parts,self.nodes,"flat",self.barriers)
        self.assertEqual(failures,[],json.dumps(failures[:12],ensure_ascii=False))

    def test_every_stair_centre_path_has_all_part_mesh_headroom(self):
        failures = headroom_conflicts(self.parts,self.nodes,"stairs",self.barriers)
        self.assertEqual(failures,[],json.dumps(failures[:12],ensure_ascii=False))

    def test_independent_oracle_matches_every_actual_support_edge(self):
        oracle = oracle_graph(self.nodes)
        self.assertEqual({frozenset((a,b)) for a,neighbors in self.graph.items() for b in neighbors},{frozenset(edge) for edge in oracle.edges})
        self.assertEqual(self.reached,set().union(*(nx.node_connected_component(oracle,key) for key in self.seeds)))

    def test_every_capsule_landing_is_reachable_from_ground(self):
        targets = {key for key in self.nodes if '/capsule-landing-' in key}
        self.assertEqual(len(targets),CASES['expected_capsule_landings'])
        missing = sorted(targets-self.reached)
        self.assertEqual(missing,[],json.dumps([{'node':key,**self.nodes[key]} for key in missing],ensure_ascii=False))

    def test_both_service_roof_returns_are_ground_reachable(self):
        targets = {key for key in self.nodes if '/roof-return-landing-' in key}
        self.assertEqual(len(targets),2)
        self.assertEqual(targets-self.reached,set())
        self.assertEqual(sorted(round(self.nodes[key]['z'],6) for key in targets),sorted(round(t.height,6) for t in MODEL.towers()))

    def test_both_bridge_approaches_are_supported_and_reachable(self):
        bridges = {key for key,node in self.nodes.items() if node['role']=='bridge'}
        self.assertEqual(len(bridges),CASES['expected_bridges'])
        approaches = []
        for key in sorted(bridges):
            bridge = self.nodes[key]
            self.assertIn(key,self.reached)
            for tower,edge in (('A',bridge['rect'][0]),('B',bridge['rect'][1])):
                neighbors = {other for other in self.graph[key] if self.nodes[other]['rect'][0]-EPS<=edge<=self.nodes[other]['rect'][1]+EPS and (self.nodes[other]['rect'][0]<edge-EPS if tower=='A' else self.nodes[other]['rect'][1]>edge+EPS)}
                approaches.append((key,tower))
                self.assertTrue(neighbors,f'{key} {tower} approach has no actual support across the minimum walking width')
                self.assertTrue(neighbors&self.reached,f'{key} {tower} approach is disconnected from ground')
        self.assertEqual(len(approaches),CASES['expected_bridge_approaches'])


if __name__=='__main__':
    unittest.main(verbosity=1)
