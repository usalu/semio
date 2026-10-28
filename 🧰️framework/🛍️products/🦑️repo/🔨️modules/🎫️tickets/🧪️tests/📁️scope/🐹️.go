package tickets

import (
 "encoding/json"
 "os"
 "reflect"
 "testing"
 "unicode/utf16"

 sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

// 🛡️TestTicketScopePortableSegments shares the language-neutral path vocabulary.
func TestTicketScopePortableSegments(t *testing.T) {
 data,err:=os.ReadFile(sandbox.OriginPath("..","..","..","🔗️graphql","🧪️tests","🌿️tickets","📁️scope","🔣️.json"));if err!=nil {t.Fatal(err)}
 var vectors struct {InvalidSegments,ValidSegments []string};if err=json.Unmarshal(data,&vectors);err!=nil {t.Fatal(err)}
 for _,value:=range vectors.InvalidSegments {if portableDraftSegment(value) {t.Errorf("invalid segment accepted: %q",value)}}
 for _,value:=range vectors.ValidSegments {if !portableDraftSegment(value) {t.Errorf("valid segment refused: %q",value)}}
}

// 🧭️TestTicketScopeInvalidIDsHaveNoEffects refuses escaped paths before a store effect.
func TestTicketScopeInvalidIDsHaveNoEffects(t *testing.T) {
 store:=NewMemoryTicketStore();store.SeedFile("/repo/outside.txt","keep")
 service:=NewTicketService(NewTicketLayout("/repo/.🧬semio/🦑️repo"),store,NullIssueTracker{},NewFixedTicketClock(26,10,21,"2026-10-21 00:00:00"),NewRecordingEventSink())
 before:=store.Snapshot()
 for _,slug:=range []string{"","..","a/../b","a//b","a\\b","CON"} {
  if err:=service.Delete(NewTicketID(26,10,21,slug),true);err==nil {t.Errorf("invalid delete accepted: %q",slug)}
  if !reflect.DeepEqual(before,store.Snapshot()) {t.Fatal("invalid identity changed the store")}
 }
}

// 🔗️TestTicketScopeDraftSymlinksHaveNoEffects exercises genuine store metadata.
func TestTicketScopeDraftSymlinksHaveNoEffects(t *testing.T) {
 store:=NewMemoryTicketStore();root:="/repo/.🧬semio/🦑️repo";store.SeedDirectory(JoinPath(root,"📝️drafts/seed"));store.SeedSymlink(JoinPath(root,"📝️drafts/seed/link"))
 service:=NewTicketService(NewTicketLayout(root),store,NullIssueTracker{},NewFixedTicketClock(26,10,21,"2026-10-21 00:00:00"),NewRecordingEventSink())
 before:=store.Snapshot();paths:=store.Paths()
 _,err:=service.Open(TicketOpenRequest{Emoji:"🎫️",Title:"Scoped Ticket",Prompt:"Prompt",Client:"codex",Goal:"SCOPED",Draft:"seed",NoManagement:true})
 if err==nil {t.Fatal("symlink draft accepted")}
 if !reflect.DeepEqual(before,store.Snapshot())||!reflect.DeepEqual(paths,store.Paths()) {t.Fatal("draft preflight changed the store")}
}

// 📏️ticketScopeLongComponent reads the shared projected-path counterexample.
func ticketScopeLongComponent(t *testing.T) string {
 t.Helper();data,err:=os.ReadFile(sandbox.OriginPath("..","..","..","🔗️graphql","🧪️tests","🌿️tickets","📁️scope","🔣️.json"));if err!=nil {t.Fatal(err)}
 var vectors struct{ProjectedPathSegment string};if err=json.Unmarshal(data,&vectors);err!=nil {t.Fatal(err)};if vectors.ProjectedPathSegment=="" {t.Fatal("shared projected path vector required")};return vectors.ProjectedPathSegment
}

// 📦️TestTicketScopeProjectedDraftRefusesBeforeCreate checks every moved descendant.
func TestTicketScopeProjectedDraftRefusesBeforeCreate(t *testing.T) {
 store:=NewMemoryTicketStore();layout:=NewTicketLayout("/r/m");component:=ticketScopeLongComponent(t)
 source:=JoinPath(JoinPath(layout.RepoMetaDir(),"📝️drafts/seed/tree"),component);store.SeedFile(source,"keep")
 destination:=JoinPath(JoinPath(layout.TicketDir(NewTicketID(26,10,21,"SCOPED-TICKET")),"tree"),component)
 if len(utf16.Encode([]rune(source)))>256||len(utf16.Encode([]rune(destination)))<=256 {t.Fatal("projected extent fixture is not discriminating")}
 service:=NewTicketService(layout,store,NullIssueTracker{},NewFixedTicketClock(26,10,21,"2026-10-21 00:00:00"),NewRecordingEventSink())
 before,paths:=store.Snapshot(),store.Paths()
 if _,err:=service.Open(TicketOpenRequest{Emoji:"🎫️",Title:"Scoped Ticket",Prompt:"Prompt",Client:"codex",Goal:"SCOPED",Draft:"seed",NoManagement:true});err==nil {t.Fatal("projected overflow accepted")}
 if !reflect.DeepEqual(before,store.Snapshot())||!reflect.DeepEqual(paths,store.Paths()) {t.Fatal("projected refusal changed the store")}
}

// 🌳️TestTicketScopeProjectedRetitleRefusesBeforeRename keeps the existing tree intact.
func TestTicketScopeProjectedRetitleRefusesBeforeRename(t *testing.T) {
 store:=NewMemoryTicketStore();layout:=NewTicketLayout("/r/m");service:=NewTicketService(layout,store,NullIssueTracker{},NewFixedTicketClock(26,10,21,"2026-10-21 00:00:00"),NewRecordingEventSink())
 opened,err:=service.Open(TicketOpenRequest{Emoji:"🎫️",Title:"Short",Prompt:"Prompt",Client:"codex",Goal:"SCOPED",NoManagement:true});if err!=nil {t.Fatal(err)}
 id,err:=ParseTicketID(opened.ID);if err!=nil {t.Fatal(err)};component:=ticketScopeLongComponent(t)
 source:=JoinPath(JoinPath(layout.TicketDir(id),"tree"),component);store.SeedFile(source,"keep")
 next:=NewTicketID(26,10,21,"A-LONGER-RENAMED-TICKET");destination:=JoinPath(JoinPath(layout.TicketDir(next),"tree"),component)
 if len(utf16.Encode([]rune(source)))>256||len(utf16.Encode([]rune(destination)))<=256||len(utf16.Encode([]rune(layout.ImportantPath(next))))>256 {t.Fatal("retitle extent fixture is not discriminating")}
 before,paths:=store.Snapshot(),store.Paths();title:="A Longer Renamed Ticket"
 if _,err=service.Change(TicketChangeRequest{ID:opened.ID,Title:&title,NoManagement:true});err==nil {t.Fatal("projected retitle overflow accepted")}
 if !reflect.DeepEqual(before,store.Snapshot())||!reflect.DeepEqual(paths,store.Paths()) {t.Fatal("retitle refusal changed the store")}
}

// 💽️TestTicketScopeNamespaceSeparators preserves drive and UNC root identity.
func TestTicketScopeNamespaceSeparators(t *testing.T) {
 data,err:=os.ReadFile(sandbox.OriginPath("..","..","..","🔗️graphql","🧪️tests","🌿️tickets","📁️scope","🔣️.json"));if err!=nil {t.Fatal(err)}
 var vectors struct{SeparatorCases []struct{Input,Normalized,Parent string}};if err=json.Unmarshal(data,&vectors);err!=nil {t.Fatal(err)}
 for _,value:=range vectors.SeparatorCases {if actual:=NormalizeSeparators(value.Input);actual!=value.Normalized {t.Errorf("normalize %q=%q want %q",value.Input,actual,value.Normalized)};if actual:=DirName(value.Input);actual!=value.Parent {t.Errorf("parent %q=%q want %q",value.Input,actual,value.Parent)}}
}

// 🧾️TestTicketDocumentPreservesAllDeclaredState shares the durable Go/Rust document vector.
func TestTicketDocumentPreservesAllDeclaredState(t *testing.T) {
 data,err:=os.ReadFile(sandbox.OriginPath("..","..","🧪️tests","🧫️fixtures","📄️ticket-document-codec","📄️documents.json"));if err!=nil {t.Fatal(err)}
 var vectors struct{Documents []struct{Source,Text string}};if err=json.Unmarshal(data,&vectors);err!=nil {t.Fatal(err)}
 var source string;for _,entry:=range vectors.Documents {if entry.Source=="DURABLE-TICKET-STATE" {source=entry.Text}}
 if source=="" {t.Fatal("durable ticket vector required")}
 ticket,err:=DecodeTicketDocument(source);if err!=nil {t.Fatal(err)}
 first:=EncodeTicketDocument(ticket)
 var expected,actual interface{};if err=json.Unmarshal([]byte(source),&expected);err!=nil {t.Fatal(err)};if err=json.Unmarshal([]byte(first),&actual);err!=nil {t.Fatal(err)}
 if !reflect.DeepEqual(actual,expected) {t.Fatalf("durable state lost: %s",first)}
 reference,err:=json.MarshalIndent(ticket,"","  ");if err!=nil {t.Fatal(err)};if first!=string(reference) {t.Fatalf("canonical encoder differs from model JSON: %s",reference)}
 again,err:=DecodeTicketDocument(first);if err!=nil {t.Fatal(err)};if second:=EncodeTicketDocument(again);first!=second {t.Fatal("document encoding is not idempotent")}
}
