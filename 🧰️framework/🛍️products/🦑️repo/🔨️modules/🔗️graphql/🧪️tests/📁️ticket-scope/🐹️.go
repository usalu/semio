package graphql

import (
 "encoding/json"
 "os"
 "path/filepath"
 "reflect"
 "strings"
 "testing"
 "unicode/utf16"

 model "github.com/usalu/semio/repo/model"
 tickets "github.com/usalu/semio/repo/tickets"
 workspace "github.com/usalu/semio/repo/workspace"
)

type ticketScopeVectors struct {
 Clock struct {Year,Month,Day int;Stamp string}
 Roots []struct {Name,Draft string}
 Open model.TicketOpenInput
 Change struct {Title,Description,LLM,Effort,Client string;NoManagement bool}
 Close struct {Summary string;Files []string;NoManagement bool}
 Reopen model.TicketReopenInput
 Expected struct {Slug,Status,Prompt,OriginalSlug,OtherStatus,OtherPrompt string;Kinds,OtherKinds,DraftFiles []string}
}

// 🧭️TestTicketContextsKeepIndependentLifecycles exercises actual rooted GraphQL mutations.
func TestTicketContextsKeepIndependentLifecycles(t *testing.T) {
 data,err:=os.ReadFile(filepath.FromSlash("../../🧪️tests/🌿️tickets/📁️scope/🔣️.json"));if err!=nil {t.Fatal(err)}
 var vectors ticketScopeVectors;if err=json.Unmarshal(data,&vectors);err!=nil {t.Fatal(err)}
 parent:=os.Getenv("SEMIO_TEST_ARTIFACT_DIR")
 if !filepath.IsAbs(parent)||len(utf16.Encode([]rune(filepath.Join(parent,"tkt-"+strings.Repeat("x",32),"a/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/RENAMED-TICKET/📌️important/📝️.md"))))>256 {t.Fatal("bounded owned ticket artifact directory required")}
 base,err:=os.MkdirTemp(parent,"tkt-");if err!=nil {t.Fatal(err)};defer os.RemoveAll(base)
 ambient:=workspace.GetRootDir();cwd,err:=os.Getwd();if err!=nil {t.Fatal(err)}
 contexts:=make([]*RepoContext,len(vectors.Roots));opened:=make([]*model.Ticket,len(vectors.Roots))
 for i,root:=range vectors.Roots {
  path:=filepath.Join(base,root.Name);if err=os.MkdirAll(filepath.Join(path,".🧬semio","🦑️repo","📝️drafts","seed"),0755);err!=nil {t.Fatal(err)}
  if err=os.WriteFile(filepath.Join(path,".🧬semio","🦑️repo","📝️drafts","seed","note.md"),[]byte(root.Draft),0644);err!=nil {t.Fatal(err)}
  contexts[i]=NewRepoContext(path);contexts[i].tickets.Clock=tickets.NewFixedTicketClock(vectors.Clock.Year,vectors.Clock.Month,vectors.Clock.Day,vectors.Clock.Stamp);contexts[i].tickets.Events=tickets.NewRecordingEventSink()
 }
 for i,c:=range contexts {opened[i],err=c.TicketOpen(vectors.Open);if err!=nil {t.Fatal(err)}}
 otherBefore,err:=os.ReadFile(opened[1].JsonPath);if err!=nil {t.Fatal(err)}
 first:=opened[0];change:=vectors.Change
 changed,err:=contexts[0].TicketChange(model.TicketChangeInput{Year:first.Year,Month:first.Month,Day:first.Day,Slug:first.Slug,Title:&change.Title,Prompt:&change.Description,LLM:&change.LLM,Effort:&change.Effort,Client:&change.Client,NoManagement:true});if err!=nil {t.Fatal(err)}
 if changed.Slug!=vectors.Expected.Slug||changed.Description!=vectors.Expected.Prompt {t.Fatalf("changed=%+v",changed)}
 badClient,badTitle:="not-a-client","Forbidden Rename";before,err:=os.ReadFile(changed.JsonPath);if err!=nil {t.Fatal(err)}
 if _,err=contexts[0].TicketChange(model.TicketChangeInput{Year:changed.Year,Month:changed.Month,Day:changed.Day,Slug:changed.Slug,Title:&badTitle,Client:&badClient,NoManagement:true});err==nil {t.Fatal("invalid client accepted")}
 after,err:=os.ReadFile(changed.JsonPath);if err!=nil||!reflect.DeepEqual(before,after) {t.Fatalf("invalid request changed original: %v",err)}
 closed,err:=contexts[0].TicketClose(model.TicketCloseInput{Year:changed.Year,Month:changed.Month,Day:changed.Day,Slug:changed.Slug,Summary:vectors.Close.Summary,Files:vectors.Close.Files,NoManagement:true});if err!=nil {t.Fatal(err)}
 again:=filepath.Join(base,"a",".🧬semio","🦑️repo","📝️drafts","again");if err=os.MkdirAll(again,0755);err!=nil {t.Fatal(err)}
 if err=os.WriteFile(filepath.Join(again,"note.md"),[]byte("second\n"),0644);err!=nil {t.Fatal(err)}
 reopen:=vectors.Reopen;reopen.Year=closed.Year;reopen.Month=closed.Month;reopen.Day=closed.Day;reopen.Slug=closed.Slug
 reopened,err:=contexts[0].TicketReopen(reopen);if err!=nil {t.Fatal(err)}
 kinds:=make([]string,0,len(reopened.Interactions));for _,interaction:=range reopened.Interactions {kinds=append(kinds,interaction.Kind)}
 if string(reopened.Status)!=vectors.Expected.Status||!reflect.DeepEqual(kinds,vectors.Expected.Kinds) {t.Fatalf("reopened=%+v kinds=%v",reopened,kinds)}
 for i,name:=range vectors.Expected.DraftFiles {body,err:=os.ReadFile(filepath.Join(reopened.FolderPath,name));if err!=nil {t.Fatal(err)};want:=vectors.Roots[0].Draft;if i==1 {want="second\n"};if string(body)!=want {t.Fatalf("draft %s=%q",name,body)}}
 otherAfter,err:=os.ReadFile(opened[1].JsonPath);if err!=nil||!reflect.DeepEqual(otherBefore,otherAfter) {t.Fatalf("sibling changed: %v",err)}
 if ok,err:=contexts[0].TicketDelete(model.TicketDeleteInput{Year:reopened.Year,Month:reopened.Month,Day:reopened.Day,Slug:reopened.Slug,NoManagement:true});err!=nil||!ok {t.Fatalf("delete=%v %v",ok,err)}
 if _,err=os.Stat(reopened.FolderPath);!os.IsNotExist(err) {t.Fatalf("deleted ticket remains: %v",err)}
 if _,err=os.Stat(opened[1].JsonPath);err!=nil {t.Fatal(err)}
 current,_:=os.Getwd();if current!=cwd||workspace.GetRootDir()!=ambient {t.Fatal("repository context changed ambient root/cwd")}
}
