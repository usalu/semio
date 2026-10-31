"""🌳 Exact IFC operand plans for first-party analytic stock differences; display tessellation is never an operand."""
import math
from fractions import Fraction


def interval(primitive):
    """📏 Retain actual finite stock elevation bounds for prisms, profiles and closed polyhedra."""
    return (min(point[2] for point in primitive['vertices']),max(point[2] for point in primitive['vertices'])) if primitive['kind']=='polyhedron' else (primitive['z0'],primitive['z1'])


def closed_polyhedron(item):
    """🔒 Require finite original vertices, planar nondegenerate faces and one consistently closed oriented shell."""
    raw=item['vertices'];faces=item['faces']
    if len(raw)<4 or len(faces)<4 or any(len(point)!=3 or any(type(value) not in (int,float) or not math.isfinite(value) for value in point) for point in raw):raise ValueError('Native polyhedra need finite original vertices and a complete closed shell')
    points=[tuple(Fraction(value) for value in point) for point in raw]
    if len(set(points))!=len(points):raise ValueError('Native polyhedron vertices must identify distinct original positions')
    edges={};neighbors={index:set() for index in range(len(faces))};volume=Fraction(0);origin=points[0]
    for index,face in enumerate(faces):
        if len(face)<3 or len(set(face))!=len(face) or any(type(vertex) is not int or not 0<=vertex<len(points) for vertex in face):raise ValueError('Native polyhedron faces need distinct valid vertex indices')
        anchor=points[face[0]];normal=None
        for a,b in zip(face[1:],face[2:]):
            u,v=([points[vertex][axis]-anchor[axis] for axis in range(3)] for vertex in (a,b));candidate=(u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0])
            if any(candidate):normal=candidate;break
        if normal is None or any(sum(normal[axis]*(points[vertex][axis]-anchor[axis]) for axis in range(3))!=0 for vertex in face):raise ValueError('Native polyhedron faces must be exactly planar and nondegenerate')
        for a,b in zip(face,face[1:]+face[:1]):edges.setdefault(tuple(sorted((a,b))),[]).append((index,a,b))
        for i in range(1,len(face)-1):
            a,b,c=([points[vertex][axis]-origin[axis] for axis in range(3)] for vertex in (face[0],face[i],face[i+1]));volume+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0])
    for rows in edges.values():
        if len(rows)!=2 or rows[0][1:]!=rows[1][1:][::-1]:raise ValueError('Native polyhedron edges must occur exactly twice in opposite directions')
        a,b=rows[0][0],rows[1][0];neighbors[a].add(b);neighbors[b].add(a)
    seen=set();pending=[0]
    while pending:
        node=pending.pop()
        if node in seen:continue
        seen.add(node);pending.extend(neighbors[node]-seen)
    if len(seen)!=len(faces) or volume<=0:raise ValueError('Native polyhedra need one outward-oriented positive closed shell')


def primitives(operand):
    """🧬 Traverse every authoritative primitive without replacing nested finite-height differences."""
    if operand.get('operator')=='difference':
        yield from primitives(operand['stock'])
        for cutter in operand['cutters']:yield from primitives(cutter['operand'])
    else:yield operand


def stock(operand):
    """🧱 Resolve the bounding nominal primitive while retaining the original tree separately."""
    return stock(operand['stock']) if operand.get('operator')=='difference' else operand


def conditioning(operand):
    """🗺️ Choose an invertible power-of-two numeric frame from source parameters, never a sampled material boundary."""
    original=stock(operand);points=[];features=[]
    for item in primitives(operand):
        z0,z1=interval(item)
        if not z1>z0:raise ValueError('Native analytic extrusion heights must increase')
        features.append(z1-z0)
        if item['kind']=='polyhedron':
            closed_polyhedron(item);points.extend(item['vertices']);features.extend(math.dist(item['vertices'][a],item['vertices'][b]) for face in item['faces'] for a,b in zip(face,face[1:]+face[:1]))
        elif item['kind']=='prism':
            for ring in (item['outline'],*item['holes']):
                points.extend([*point,z] for point in ring for z in (z0,z1))
                features.extend(math.dist(a,b) for a,b in zip(ring,ring[1:]+ring[:1]) if a!=b)
        elif item['kind']=='profile_extrusion':
            points.extend([*item['axis'],z] for z in (z0,z1));profile=item['profile'];shape=profile['shape']
            if shape in ('circle','ellipse'):features.extend([profile['radius']] if shape=='circle' else profile['radii'])
            elif shape=='rectangle':features.extend((profile['width'],profile['depth']))
            elif shape=='polygon':features.extend(math.dist(a,b) for a,b in zip(profile['outline'],profile['outline'][1:]+profile['outline'][:1]) if a!=b)
            else:raise ValueError('Unknown original analytic stock profile')
            for hole in profile['voids']:
                features.append(hole['radius'])
                if profile['shape']=='circle':features.append(profile['radius']-math.hypot(*hole['offset'])-hole['radius'])
        else:raise ValueError('Unknown authoritative native analytic primitive')
    if any(not math.isfinite(value) for point in points for value in point) or any(not math.isfinite(value) or value<=0 for value in features):raise ValueError('Native analytic operands require finite positive dimensions')
    feature=min(features);conditioned=feature*feature<1e-7;exponent=max(0,math.ceil(-math.log2(feature))) if conditioned else 0;scale=math.ldexp(1.,exponent)
    anchor=original['vertices'][0] if original['kind']=='polyhedron' else [*(original['outline'][0] if original['kind']=='prism' else original['axis']),original['z0']];origin=list(anchor) if conditioned else [0.,0.,0.]
    for axis in range(3):
        if any((point[axis]-origin[axis])+origin[axis]!=point[axis] for point in points):origin[axis]=0.
    for point in points:
        for axis,value in enumerate(point):
            local=(value-origin[axis])*scale
            if not math.isfinite(local) or origin[axis]+local/scale!=value:raise ValueError('Native analytic conditioning must preserve each original binary64 coordinate')
    return {'conditioned':conditioned,'origin':origin,'scale':scale,'minimum_source_feature':feature}


class Plan:
    """📋 Allocate one dependency-ordered native graph through the existing first-party STEP writer."""
    def __init__(self,helpers,operand,key):
        self.helpers=helpers;self.prefix=key+'_analytic_';self.nodes=[];self.operations=[];self.frame=conditioning(operand)
        result=self.operand(operand)
        if operand.get('operator')=='difference':self.root=self.add('IFCCSGSOLID',result,key=key)['ref']
        else:
            previous=result['ref'];self.root=key;self.nodes=[(key if name==previous else name,entity,attributes) for name,entity,attributes in self.nodes]
        self.keys=[name for name,_,_ in self.nodes]

    def add(self,entity,*attributes,key=None):
        """🔢 Reserve one first-party entity identifier without an external SDK object type."""
        name=key or self.prefix+str(len(self.nodes));self.nodes.append((name,entity,attributes));return {'ref':name}

    def point(self,coordinates):
        """📍 Preserve finite local binary64 coordinates in native Cartesian point records."""
        return self.add('IFCCARTESIANPOINT',self.helpers._coords(coordinates))

    def polyline(self,ring,local=False):
        """🔺 Retain original polygon loops and explicit closure without a hull replacement."""
        references=[self.point([(value-(0 if local else self.frame['origin'][axis]))*self.frame['scale'] for axis,value in enumerate(point)]) for point in ring]
        return self.add('IFCPOLYLINE',self.helpers._tuple(*(references+references[:1])))

    def primitive(self,item):
        """🧿 Emit exact circular or elliptical profiles and source polygon holes in their declared extrusion pose."""
        h=self.helpers;scale=self.frame['scale'];origin=self.frame['origin'];z0,z1=interval(item)
        if item['kind']=='polyhedron':
            points=[self.point([(value-origin[axis])*scale for axis,value in enumerate(point)]) for point in item['vertices']];faces=[]
            for face in item['faces']:
                loop=self.add('IFCPOLYLOOP',h._tuple(*[points[index] for index in face]));bound=self.add('IFCFACEOUTERBOUND',loop,h._enum('T'));faces.append(self.add('IFCFACE',h._tuple(bound)))
            shell=self.add('IFCCLOSEDSHELL',h._tuple(*faces));return self.add('IFCFACETEDBREP',shell)
        if item['kind']=='prism':
            outer=self.polyline(item['outline']);holes=[self.polyline(ring) for ring in item['holes']]
            profile=self.add('IFCARBITRARYPROFILEDEFWITHVOIDS' if holes else 'IFCARBITRARYCLOSEDPROFILEDEF',h._enum('AREA'),'$',outer,*([h._tuple(*holes)] if holes else []));location=[0.,0.,(z0-origin[2])*scale];direction='$'
        elif item['kind']=='profile_extrusion':
            source=item['profile'];shape=source['shape'];zero=self.point([0.,0.]);profile_pose=self.add('IFCAXIS2PLACEMENT2D',zero,'$');name=h._str('source_shape:'+shape)
            radii=[source['radius']] if shape=='circle' else source['radii'] if shape=='ellipse' else []
            ring=[[-source['width']/2,-source['depth']/2],[source['width']/2,-source['depth']/2],[source['width']/2,source['depth']/2],[-source['width']/2,source['depth']/2]] if shape=='rectangle' else source.get('outline')
            if source['voids']:
                outer=self.add('IFCCIRCLE' if shape=='circle' else 'IFCELLIPSE',profile_pose,*[h._real(value*scale) for value in radii]) if shape in ('circle','ellipse') else self.polyline(ring,local=True);holes=[]
                for hole in source['voids']:
                    point=self.point([value*scale for value in hole['offset']]);pose=self.add('IFCAXIS2PLACEMENT2D',point,'$');holes.append(self.add('IFCCIRCLE',pose,h._real(hole['radius']*scale)))
                profile=self.add('IFCARBITRARYPROFILEDEFWITHVOIDS',h._enum('AREA'),name,outer,h._tuple(*holes))
            elif shape=='polygon':profile=self.add('IFCARBITRARYCLOSEDPROFILEDEF',h._enum('AREA'),name,self.polyline(ring,local=True))
            else:profile=self.add('IFCCIRCLEPROFILEDEF' if shape=='circle' else 'IFCELLIPSEPROFILEDEF' if shape=='ellipse' else 'IFCRECTANGLEPROFILEDEF',h._enum('AREA'),name,profile_pose,*[h._real(value*scale) for value in (radii if radii else (source['width'],source['depth']))])
            angle=math.radians(item['rotation']);direction=self.add('IFCDIRECTION',h._coords([math.cos(angle),math.sin(angle),0.]));location=[(item['axis'][axis]-origin[axis])*scale for axis in (0,1)]+[(z0-origin[2])*scale]
        else:raise ValueError('Unknown authoritative native analytic primitive')
        point=self.point(location);pose=self.add('IFCAXIS2PLACEMENT3D',point,'$',direction);extrusion=self.add('IFCDIRECTION',h._coords([0.,0.,1.]));return self.add('IFCEXTRUDEDAREASOLID',profile,pose,extrusion,h._real((z1-z0)*scale))

    def operand(self,operand):
        """➖ Preserve each explicit stock/cutter ordering and owner role through nested regularized differences."""
        if operand.get('operator')!='difference':return self.primitive(operand)
        result=self.operand(operand['stock'])
        for cutter in operand['cutters']:
            other=self.operand(cutter['operand']);result=self.add('IFCBOOLEANRESULT',self.helpers._enum('DIFFERENCE'),result,other);self.operations.append({'node':result['ref'],**{name:cutter[name] for name in ('owner','owner_guid','role') if name in cutter}})
        return result

    def write(self,writer):
        """📄 Resolve only first-party references after the complete native graph has identifiers."""
        def resolved(value):
            """🪢 Bind explicit graph references without interpreting text as code."""
            if isinstance(value,dict):return writer.ref(value['ref'])
            if isinstance(value,list):return [resolved(item) for item in value]
            return value
        for key,entity,attributes in self.nodes:writer.add(key,entity,*[resolved(value) for value in attributes])
