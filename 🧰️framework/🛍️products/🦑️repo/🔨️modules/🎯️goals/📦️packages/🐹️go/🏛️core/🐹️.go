package goals

import (
	errors "errors"

	model "github.com/usalu/semio/repo/model"
)

// #region 🧲️Goal Contracts

// 🎯️Goal is the goal document shape of 📐️model, reexported so a client needs one import.
type Goal = model.Goal

// 🟠️GoalDates is the dated members of a goal.
type GoalDates = model.GoalDates

// 🟡️GoalManagementData is the milestone and issue a goal owns.
type GoalManagementData = model.GoalManagementData

// 🆕️GoalCreateInput is the input an open takes.
type GoalCreateInput = model.GoalCreateInput

// ♻️GoalChangeInput is the input a change takes.
type GoalChangeInput = model.GoalChangeInput

// 📪️GoalCloseInput is the input a close takes.
type GoalCloseInput = model.GoalCloseInput

// 🔓️GoalReopenInput is the input a reopen takes.
type GoalReopenInput = model.GoalReopenInput

// 🗑️GoalDeleteInput is the input a delete takes.
type GoalDeleteInput = model.GoalDeleteInput

// 💿️GoalNode is a goal node of the goal and ticket tree.
type GoalNode = model.GoalNode

// 💿️TicketNode is a ticket node of the goal and ticket tree.
type TicketNode = model.TicketNode

// ❗️GoalError is everything the goals domain can refuse, as a class rather than a rendered string.
type GoalError struct {
	Kind    string
	Message string
}

// 🔤️Error renders the refusal.
func (err GoalError) Error() string { return err.Message }

// 🏷️Class returns the stable class of the refusal.
func (err GoalError) Class() string { return err.Kind }

// 🏷️ErrorClass returns the stable class of any refusal this domain produced.
func ErrorClass(err error) string {
	if err == nil {
		return ""
	}
	var goalError GoalError
	if errors.As(err, &goalError) {
		return goalError.Kind
	}
	return "port"
}

// 🕳️errMissing refuses a blank required create input.
func errMissing(field string) error {
	return GoalError{Kind: "missing", Message: "missing " + field}
}

// 🚫️errNotAllowed refuses a slug outside the allowed vocabulary.
func errNotAllowed(message string) error { return GoalError{Kind: "not-allowed", Message: message} }

// 📛️errAlreadyExists refuses a duplicate identifier.
func errAlreadyExists(id string) error {
	return GoalError{Kind: "already-exists", Message: "goal with id " + id + " already exists"}
}

// 🔍️errNotFound refuses an identifier that carries no document.
func errNotFound(id string) error {
	return GoalError{Kind: "not-found", Message: "goal file not found: " + id}
}

// 🔁️errAlreadyInState refuses a lifecycle transition that is a no-operation.
func errAlreadyInState(state string) error {
	return GoalError{Kind: "already-in-state", Message: "goal is already " + state}
}

// 🔤️errTitle refuses a blank title or a slug used as a title.
func errTitle(message string) error { return GoalError{Kind: "title", Message: message} }

// 🔢️errUnparsable refuses a reference that carries no number.
func errUnparsable(value string) error {
	return GoalError{Kind: "unparsable", Message: "could not parse number from " + value}
}

// 📄️errDocument refuses a document this codec does not accept.
func errDocument(message string) error { return GoalError{Kind: "document", Message: message} }

// 🗄️errPort refuses when the store or the management provider refused.
func errPort(message string) error { return GoalError{Kind: "port", Message: message} }

// 🚦️GoalStatus is the closed set of states a goal can be in.
type GoalStatus string

// 🟢️GoalStatusOpen is a goal that is still being worked on.
const GoalStatusOpen GoalStatus = "open"

// 🔴️GoalStatusClosed is a goal that has been finished.
const GoalStatusClosed GoalStatus = "closed"

// 📋️AllGoalStatuses is the whole vocabulary, in declaration order.
func AllGoalStatuses() []GoalStatus { return []GoalStatus{GoalStatusOpen, GoalStatusClosed} }

// 📥️ParseGoalStatus accepts only a member of the closed vocabulary.
func ParseGoalStatus(value string) (GoalStatus, error) {
	for _, status := range AllGoalStatuses() {
		if string(status) == value {
			return status, nil
		}
	}
	return "", errDocument("unknown variant `" + value + "`, expected `open` or `closed`")
}

// #endregion 🧲️Goal Contracts
