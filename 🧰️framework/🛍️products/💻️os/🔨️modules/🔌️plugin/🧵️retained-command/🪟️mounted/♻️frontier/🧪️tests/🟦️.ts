import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
test("mounted publications keep original payload and allocation through a genuine caller grant",()=>{
 const root=join(import.meta.dir,"../../../..");
 const host=readFileSync(join(root,"🦀️.rs"),"utf8"),frontier=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");
 const start=host.indexOf("fn retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize)",host.indexOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation"));
 const body=host.slice(start,host.indexOf("fn terminal_is_empty",start));
 expect(body).not.toContain("artifact_mutations.pop()");expect(body).not.toContain("drop(self.completion.take())");expect(body).not.toContain("drop(chunk)");
 expect(frontier).toContain("A::admit_completion_retirement(&mut self.publication,grant)");
 expect(frontier).toContain("self.completion=Some(original)");
 expect(frontier).toContain("release_bytes:owner.frame_release_bytes()");
 expect(frontier).toContain("if owner.terminal_is_empty(){let bytes=owner.frame_release_bytes()");
 for(const axis of ["copy_bytes","capacity_bytes","release_bytes","depth"]){expect(frontier).toContain("grant.maximum_"+axis+"<demand."+axis);}
 expect(host).toContain("self.window_config_store.close_direct_ingress(grant)");expect(host).toContain("self.granted_mounted_retirement_step(grant)");
 const publicationStart=host.indexOf("impl<A: ArtifactApp> PendingArtifactStorePublication<A>");
 const publication=host.slice(publicationStart,host.indexOf("enum PendingArtifactStorePublicationRetirement",publicationStart));
 expect(publication).toContain("fn close_step(&mut self, grant:RetainedCloneGrant)");
 expect(publication).not.toContain("maximum_bytes");
});
