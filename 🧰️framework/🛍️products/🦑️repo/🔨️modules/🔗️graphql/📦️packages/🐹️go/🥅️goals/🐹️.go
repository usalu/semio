package graphql

import (
 "fmt"
 "strings"
 goals "github.com/usalu/semio/repo/goals"
 model "github.com/usalu/semio/repo/model"
 providers "github.com/usalu/semio/repo/providers"
)

// 🎯️GoalCreate creates a goal through the selected aggregate.
func (c *RepoContext) GoalCreate(input model.GoalCreateInput) (*model.Goal,error) {return c.goals.Create(input)}

// ♻️GoalChange changes a goal through the selected aggregate.
func (c *RepoContext) GoalChange(input model.GoalChangeInput) (*model.Goal,error) {return c.goals.Change(input)}

// 📪️GoalClose closes a goal through the selected aggregate.
func (c *RepoContext) GoalClose(input model.GoalCloseInput) (*model.Goal,error) {return c.goals.Close(input)}

// 🔓️GoalReopen reopens a goal through the selected aggregate.
func (c *RepoContext) GoalReopen(input model.GoalReopenInput) (*model.Goal,error) {return c.goals.Reopen(input)}

// 🗑️GoalDelete deletes a goal through the selected aggregate.
func (c *RepoContext) GoalDelete(input model.GoalDeleteInput) (bool,error) {return c.goals.Delete(input)}

// 🧾️saveManagementGoal persists a management link in the selected repository's real goal store.
func (c *RepoContext) saveManagementGoal(goal *model.Goal) error {
 document,err:=goals.EncodeGoal(goal);if err!=nil {return err};return goals.NewFsGoalStore(c.rootDir).Write(goal.ID,document)
}

// 🎯️rootGoalMilestone reads a milestone through the selected goal aggregate.
func (c *RepoContext) rootGoalMilestone(id string) (*int,error) {
 goal,err:=c.goals.Read(goals.GetRootGoalID(id));if err!=nil {return nil,err}
 if goal.Management==nil||goal.Management.Milestone=="" {return nil,nil}
 number,err:=model.ParseMilestoneNumber(goal.Management.Milestone);if err!=nil {return nil,err};return &number,nil
}

// 🐙️updateGoalMilestone records an actual provider milestone under the captured repository origin.
func (c *RepoContext) updateGoalMilestone(goal *model.Goal,number int) error {
 if c.repositoryURL=="" {return fmt.Errorf("repository origin URL is required for a goal milestone")}
 if goal.Management==nil {goal.Management=&model.GoalManagementData{}}
 url:=fmt.Sprintf("%s/milestone/%d",c.repositoryURL,number)
 if goal.Management.Milestone==url {return nil};goal.Management.Milestone=url;return c.saveManagementGoal(goal)
}

// 🧭️ensureGoalMilestone binds root-goal synchronization to the captured provider and store.
func (c *RepoContext) ensureGoalMilestone(goal *model.Goal) (*providers.ManagementMilestone,error) {
 if goal==nil||strings.TrimSpace(goal.Title)==""||!goals.IsRootGoal(goal.ID) {return nil,nil}
 if c.repositoryURL=="" {return nil,fmt.Errorf("repository origin URL is required for a goal milestone")}
 if goal.Management!=nil&&goal.Management.Milestone!="" {if number,err:=model.ParseMilestoneNumber(goal.Management.Milestone);err==nil&&number>0 {if milestone,err:=c.managementProvider.GetMilestone(number);err==nil {return milestone,nil}}}
 found,err:=c.managementProvider.FindMilestoneByTitle(goal.Title);if err!=nil {return nil,err}
 if found!=nil {if err=c.updateGoalMilestone(goal,found.Number);err!=nil {return nil,err};return found,nil}
 number,err:=c.managementProvider.CreateMilestone(goal.Title,goal.Description);if err!=nil {return nil,err}
 if goal.Dates.Due!="" {_=c.managementProvider.UpdateMilestone(number,"","","",goal.Dates.Due)}
 if err=c.updateGoalMilestone(goal,number);err!=nil {return nil,err}
 milestone,err:=c.managementProvider.GetMilestone(number);if err!=nil {return &providers.ManagementMilestone{Number:number,Title:goal.Title,Description:goal.Description},nil};return milestone,nil
}
