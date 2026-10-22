package graphql

import (
 "strings"
 languages "github.com/usalu/semio/repo/languages"
 model "github.com/usalu/semio/repo/model"
 providers "github.com/usalu/semio/repo/providers"
 tickets "github.com/usalu/semio/repo/tickets"
 workspace "github.com/usalu/semio/repo/workspace"
)

// 🎫️ticketService retains request metadata without mutating the server's shared service.
func (c *RepoContext) ticketService(client string) (*tickets.TicketService,error) {
 service:=*c.tickets
 service.InteractionMetadata=tickets.TicketInteractionMetadata{Author:workspace.GetGitAuthor(c.rootDir),System:languages.GetSystem(),Checkpoint:workspace.GetGitCheckpoint(c.rootDir)}
 if client!="" {resolved,err:=model.ResolveAllowedClient(client);if err!=nil {return nil,err};service.McpClient=providers.McpKindFromResolvedClient(resolved)}
 return &service,nil
}

// 🧾️ticketResult resolves the canonical lifecycle output through its same bound store.
func ticketResult(service *tickets.TicketService,outcome tickets.TicketOutcome,err error) (*model.Ticket,error) {
 if err!=nil {return nil,err}
 for _,warning:=range outcome.Warnings {workspace.WriteWarningf("%s",warning)}
 id,err:=tickets.ParseTicketID(outcome.ID);if err!=nil {return nil,err};return service.Read(id)
}

// ♻️TicketChange applies validated changes through the selected repository service.
func (c *RepoContext) TicketChange(input model.TicketChangeInput) (*model.Ticket,error) {
 service,err:=c.ticketService("");if err!=nil {return nil,err}
 outcome,err:=service.Change(tickets.TicketChangeRequest{ID:tickets.NewTicketID(input.Year,input.Month,input.Day,input.Slug).ID(),Title:input.Title,Description:input.Prompt,LLM:input.LLM,Effort:input.Effort,Client:input.Client,Goal:input.Goal,Parent:input.Parent,NoManagement:input.NoManagement})
 return ticketResult(service,outcome,err)
}

// 🗑️TicketDelete deletes only a ticket resolved by the selected repository service.
func (c *RepoContext) TicketDelete(input model.TicketDeleteInput) (bool,error) {
 service,err:=c.ticketService("");if err!=nil {return false,err}
 id,err:=tickets.ParseTicketID(tickets.NewTicketID(input.Year,input.Month,input.Day,input.Slug).ID());if err!=nil {return false,err}
 if err=service.Delete(id,input.NoManagement);err!=nil {return false,err};return true,nil
}

// 📬️TicketOpen opens or continues a ticket within the selected repository.
func (c *RepoContext) TicketOpen(input model.TicketOpenInput) (*model.Ticket,error) {
 service,err:=c.ticketService(input.Client);if err!=nil {return nil,err}
 if input.Prompt=="" {input.Prompt=input.Title}
 prompt:=strings.ToUpper(input.Prompt)
 if strings.Contains(prompt,"NOTICKET") {return nil,nil}
 if strings.Contains(prompt,"CONTINUE") {
  latest,err:=service.Latest();if err!=nil {return nil,err}
  if latest.Status==model.TicketStatusOpen {return latest,nil}
  outcome,err:=service.Reopen(tickets.TicketReopenRequest{ID:tickets.NewTicketID(latest.Year,latest.Month,latest.Day,latest.Slug).ID(),Prompt:input.Prompt,LLM:input.LLM,Effort:input.Effort,Client:input.Client,Draft:input.Draft,Goal:input.Goal,Parent:input.Parent,NoManagement:input.NoManagement,Session:c.ticketSession,PlanID:input.PlanID,SpecID:input.SpecID})
  return ticketResult(service,outcome,err)
 }
 outcome,err:=service.Open(tickets.TicketOpenRequest{Emoji:input.Emoji,Title:input.Title,Prompt:input.Prompt,LLM:input.LLM,Effort:input.Effort,Client:input.Client,Draft:input.Draft,Goal:input.Goal,Parent:input.Parent,NoIssue:input.NoIssue,NoManagement:input.NoManagement,Issue:input.Issue,Session:c.ticketSession,PlanID:input.PlanID,SpecID:input.SpecID})
 return ticketResult(service,outcome,err)
}

// 📪️TicketClose settles one ticket or every open ticket within the selected repository.
func (c *RepoContext) TicketClose(input model.TicketCloseInput) (*model.Ticket,error) {
 service,err:=c.ticketService("");if err!=nil {return nil,err}
 if input.All {
  all,err:=service.List(nil,nil,nil);if err!=nil {return nil,err};var last *model.Ticket
  for _,ticket:=range all {
   if ticket.Status!=model.TicketStatusOpen {continue}
   outcome,err:=service.Close(tickets.TicketCloseRequest{ID:tickets.NewTicketID(ticket.Year,ticket.Month,ticket.Day,ticket.Slug).ID(),Summary:"Bulk close",Bulk:true,NoManagement:input.NoManagement})
   if err!=nil {workspace.WriteWarningf("Failed to close ticket %s: %v",ticket.Slug,err);continue}
   last,err=ticketResult(service,outcome,nil);if err!=nil {return nil,err}
  }
  if !input.NoManagement {
   urls,err:=c.managementProvider.ListOpenIssuesWithLabel("ticket");if err!=nil {workspace.WriteWarningf("Failed to list ticket issues: %v",err)}else{for _,url:=range urls {if err=c.managementProvider.CloseIssue(url);err!=nil {workspace.WriteWarningf("Failed to close ticket issue %s: %v",url,err)}}}
  }
  return last,nil
 }
 if err:=tickets.ValidateTicketCloseRequest(tickets.TicketCloseRequest{Summary:input.Summary,Files:input.Files});err!=nil {return nil,err}
 id:=tickets.NewTicketID(input.Year,input.Month,input.Day,input.Slug).ID()
 if input.Title!=nil&&*input.Title!="" {outcome,err:=service.Change(tickets.TicketChangeRequest{ID:id,Title:input.Title,NoManagement:input.NoManagement});if err!=nil {return nil,err};id=outcome.ID}
 outcome,err:=service.Close(tickets.TicketCloseRequest{ID:id,Summary:input.Summary,Files:input.Files,NoManagement:input.NoManagement})
 return ticketResult(service,outcome,err)
}

// 🔓️TicketReopen resumes a ticket through its selected repository service.
func (c *RepoContext) TicketReopen(input model.TicketReopenInput) (*model.Ticket,error) {
 if _,_,_,err:=tickets.ResolveTicketInteraction(input.LLM,input.Effort,input.Client);err!=nil {return nil,err}
 service,err:=c.ticketService(input.Client);if err!=nil {return nil,err}
 id:=tickets.NewTicketID(input.Year,input.Month,input.Day,input.Slug).ID()
 if input.Title!=nil&&*input.Title!="" {outcome,err:=service.Change(tickets.TicketChangeRequest{ID:id,Title:input.Title,NoManagement:input.NoManagement});if err!=nil {return nil,err};id=outcome.ID}
 outcome,err:=service.Reopen(tickets.TicketReopenRequest{ID:id,Prompt:input.Prompt,LLM:input.LLM,Effort:input.Effort,Client:input.Client,Draft:input.Draft,Goal:input.Goal,Parent:input.Parent,NoManagement:input.NoManagement,Session:c.ticketSession,PlanID:input.PlanID,SpecID:input.SpecID})
 return ticketResult(service,outcome,err)
}
