package goals

import (
	fmt "fmt"
	strings "strings"

	events "github.com/usalu/semio/repo/events"
	identity "github.com/usalu/semio/repo/identity"
	model "github.com/usalu/semio/repo/model"
)

// #region 🔓️Lifecycle

// 🎯️Goals is the goal aggregate bound to its ports.
type Goals struct {
	store      GoalStore
	management ManagementPort
	emitter    events.Emitter
	author     string
}

// 🆕️NewGoals binds the aggregate to a store, a management provider, an emitter and an author.
func NewGoals(store GoalStore, management ManagementPort, emitter events.Emitter, author string) *Goals {
	return &Goals{store: store, management: management, emitter: emitter, author: author}
}

// 📋️List returns every goal, identifier ascending.
func (aggregate *Goals) List() ([]*model.Goal, error) {
	ids := aggregate.store.IDs()
	found := make([]*model.Goal, 0, len(ids))
	for _, id := range ids {
		goal, err := aggregate.Read(id)
		if err != nil {
			return nil, err
		}
		found = append(found, goal)
	}
	return found, nil
}

// 📖️Read returns one goal by identifier, accepting the compose form.
func (aggregate *Goals) Read(id string) (*model.Goal, error) {
	id = GoalIDForFilesystem(id)
	document, err := aggregate.store.Read(id)
	if err != nil {
		return nil, err
	}
	return DecodeGoal(id, document)
}

// 🔎️Search returns every goal whose identifier, title, description or status contains the term.
func (aggregate *Goals) Search(term string) ([]*model.Goal, error) {
	all, err := aggregate.List()
	if err != nil {
		return nil, err
	}
	needle := strings.ToLower(term)
	found := make([]*model.Goal, 0, len(all))
	for _, goal := range all {
		haystack := strings.ToLower(goal.ID + " " + goal.Title + " " + goal.Description + " " + goal.Status)
		if needle == "" || strings.Contains(haystack, needle) {
			found = append(found, goal)
		}
	}
	return found, nil
}

// 🆕️Create opens a goal, asks the management provider for the milestone or issue the depth calls
// for, persists the document and emits the open event.
func (aggregate *Goals) Create(input model.GoalCreateInput) (*model.Goal, error) {
	if input.Title == "" {
		return nil, errMissing("title")
	}
	if input.Description == "" {
		return nil, errMissing("description")
	}
	if input.Prompt == "" {
		return nil, errMissing("prompt")
	}
	if input.DueDate == "" {
		return nil, errMissing("due date")
	}
	if input.LLM == "" {
		return nil, errMissing("llm")
	}
	if input.Client == "" {
		return nil, errMissing("client")
	}
	llm, err := model.ResolveAllowedLLM(input.LLM)
	if err != nil {
		return nil, errNotAllowed(err.Error())
	}
	effort := ""
	if input.Effort != "" {
		effort, err = model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
	}
	client, err := model.ResolveAllowedClient(input.Client)
	if err != nil {
		return nil, errNotAllowed(err.Error())
	}
	id := ComposeGoalID(input.Parent, input.Title)
	if aggregate.store.Exists(id) {
		return nil, errAlreadyExists(id)
	}
	goal := &model.Goal{
		Title:       input.Title,
		Description: input.Description,
		Prompt:      input.Prompt,
		Status:      string(GoalStatusOpen),
		Dates:       model.GoalDates{Due: input.DueDate},
		Client:      client,
		LLM:         llm,
		Effort:      effort,
		Parent:      input.Parent,
		ID:          id,
	}
	if !input.NoManagement {
		management, err := aggregate.openManagement(id, input)
		if err != nil {
			return nil, err
		}
		goal.Management = management
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(id, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalOpenEnded, events.GoalOpenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       goal.Title,
		Description: goal.Description,
		LLM:         llm,
		Effort:      effort,
		Parent:      goal.Parent,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🧩️openManagement asks the provider for the milestone or issue an identifier's depth calls for.
func (aggregate *Goals) openManagement(id string, input model.GoalCreateInput) (*model.GoalManagementData, error) {
	if IsRootGoal(id) {
		if input.Milestone != "" {
			return &model.GoalManagementData{Milestone: input.Milestone}, nil
		}
		number, err := aggregate.management.CreateMilestone(input.Title, input.Description)
		if err != nil {
			return nil, err
		}
		if input.DueDate != "" {
			_ = aggregate.management.UpdateMilestone(number, "", "", "", input.DueDate)
		}
		return &model.GoalManagementData{Milestone: fmt.Sprintf("%s/milestone/%d", aggregate.management.RepoURL(), number)}, nil
	}
	if IsFirstGenGoal(id) {
		var milestone *int64
		if root, err := aggregate.Read(RootGoalID(id)); err == nil && root.Management != nil && root.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(root.Management.Milestone); err == nil {
				milestone = &number
			}
		}
		issue, err := aggregate.management.CreateGoalIssue(input.Title, input.Description, milestone)
		if err != nil {
			return nil, err
		}
		return &model.GoalManagementData{Issue: issue}, nil
	}
	issue, err := aggregate.management.CreateGoalIssue(input.Title, input.Description, nil)
	if err != nil {
		return nil, err
	}
	if parent, err := aggregate.Read(ParentGoalID(id)); err == nil && parent.Management != nil && parent.Management.Issue != "" {
		_ = aggregate.management.AddSubIssue(parent.Management.Issue, issue)
	}
	return &model.GoalManagementData{Issue: issue}, nil
}

// ♻️Change changes a goal: retitling moves the document, re-parenting moves it too, and the
// management provider is told about the new title, body, state and due date.
func (aggregate *Goals) Change(input model.GoalChangeInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if input.Title != nil {
		if err := aggregate.retitle(goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.LLM != nil {
		llm, err := model.ResolveAllowedLLM(*input.LLM)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.LLM = llm
	}
	if input.Effort != nil {
		effort, err := model.ResolveAllowedEffort(*input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.Effort = effort
	}
	if input.Parent != nil {
		slug := goal.ID
		if index := strings.LastIndex(goal.ID, "/"); index != -1 {
			slug = goal.ID[index+1:]
		}
		target := slug
		if *input.Parent != "" {
			target = *input.Parent + "/" + slug
		}
		if err := aggregate.relocate(goal, target); err != nil {
			return nil, err
		}
		goal.Parent = *input.Parent
	}
	if !input.NoManagement {
		if err := aggregate.syncManagement(goal, goal.Status, goal.Dates.Due); err != nil {
			return nil, err
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalChangeEnded, events.GoalChangePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Title:       input.Title,
		Description: input.Description,
		LLM:         input.LLM,
		Effort:      input.Effort,
		Parent:      input.Parent,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 📪️Close closes a goal, refusing a goal that is already closed.
func (aggregate *Goals) Close(input model.GoalCloseInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if goal.Status == string(GoalStatusClosed) {
		return nil, errAlreadyInState("closed")
	}
	goal.Status = string(GoalStatusClosed)
	goal.Summary = input.Summary
	if !input.NoManagement && goal.Management != nil {
		if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
				if err := aggregate.management.UpdateMilestone(number, goal.Title, goal.Description, "closed", ""); err != nil {
					return nil, err
				}
			}
		} else if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			if err := aggregate.management.CloseIssue(goal.Management.Issue); err != nil {
				return nil, err
			}
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalCloseEnded, events.GoalClosePayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Summary:     goal.Summary,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🔓️Reopen reopens a goal, refusing a goal that is already open, and replaces the interaction
// members with the ones the reopen carries.
func (aggregate *Goals) Reopen(input model.GoalReopenInput) (*model.Goal, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return nil, err
	}
	if goal.Status == string(GoalStatusOpen) {
		return nil, errAlreadyInState("open")
	}
	goal.Status = string(GoalStatusOpen)
	if input.Title != nil {
		if err := aggregate.retitle(goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}
	goal.Prompt = input.Prompt
	goal.LLM = input.LLM
	goal.Client = input.Client
	if input.Effort != "" {
		effort, err := model.ResolveAllowedEffort(input.Effort)
		if err != nil {
			return nil, errNotAllowed(err.Error())
		}
		goal.Effort = effort
	}
	if !input.NoManagement {
		if err := aggregate.syncManagement(goal, string(GoalStatusOpen), goal.Dates.Due); err != nil {
			return nil, err
		}
	}
	document, err := EncodeGoal(goal)
	if err != nil {
		return nil, err
	}
	if err := aggregate.store.Write(goal.ID, document); err != nil {
		return nil, err
	}
	aggregate.emit(events.EventGoalReopenEnded, events.GoalReopenPayload{
		GoalPayload: events.GoalPayload{ID: goal.ID},
		Prompt:      input.Prompt,
		Client:      input.Client,
		LLM:         input.LLM,
		Effort:      goal.Effort,
		Author:      aggregate.author,
	})
	return goal, nil
}

// 🗑️Delete deletes a goal and everything below it, after retiring its milestone or issue.
func (aggregate *Goals) Delete(input model.GoalDeleteInput) (bool, error) {
	goal, err := aggregate.Read(input.ID)
	if err != nil {
		return false, err
	}
	if !input.NoManagement && goal.Management != nil {
		if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
			if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
				if err := aggregate.management.DeleteMilestone(number); err != nil {
					return false, err
				}
			}
		} else if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
			parts := strings.Split(goal.Management.Issue, "/")
			if err := aggregate.management.DeleteIssue(parts[len(parts)-1]); err != nil {
				return false, err
			}
		}
	}
	if err := aggregate.store.Remove(goal.ID); err != nil {
		return false, err
	}
	return true, nil
}

// 🔁️syncManagement tells the provider about a goal's current title, body, state and due date.
func (aggregate *Goals) syncManagement(goal *model.Goal, state, due string) error {
	if goal.Management == nil {
		return nil
	}
	if IsRootGoal(goal.ID) && goal.Management.Milestone != "" {
		if number, err := ParseMilestoneNumber(goal.Management.Milestone); err == nil {
			return aggregate.management.UpdateMilestone(number, goal.Title, goal.Description, state, due)
		}
		return nil
	}
	if !IsRootGoal(goal.ID) && goal.Management.Issue != "" {
		return aggregate.management.UpdateGoalIssue(goal.Management.Issue, goal.Title, goal.Description)
	}
	return nil
}

// 🔤️retitle refuses a blank title or a slug used as a title and moves the document.
func (aggregate *Goals) retitle(goal *model.Goal, title string) error {
	title = strings.TrimSpace(title)
	if title == "" {
		return errTitle("goal title is required")
	}
	slug := identity.Slugify(title)
	if title == slug {
		return errTitle("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(slug) {
		return errTitle("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}
	target := ComposeGoalID(goal.Parent, title)
	if err := aggregate.relocate(goal, target); err != nil {
		return err
	}
	goal.Title = title
	return nil
}

// 🚚️relocate moves a goal to a target identifier, refusing a target that is taken.
func (aggregate *Goals) relocate(goal *model.Goal, target string) error {
	if target == goal.ID {
		return nil
	}
	if aggregate.store.Exists(target) {
		return errAlreadyExists(target)
	}
	if err := aggregate.store.Rename(goal.ID, target); err != nil {
		return err
	}
	goal.ID = target
	return nil
}

// 📤️emit sends one envelope to the bound emitter.
func (aggregate *Goals) emit(kind events.EventKind, payload interface{}) {
	if aggregate.emitter == nil {
		return
	}
	aggregate.emitter.Emit(kind, "repo-cli", payload)
}

// #endregion 🔓️Lifecycle
