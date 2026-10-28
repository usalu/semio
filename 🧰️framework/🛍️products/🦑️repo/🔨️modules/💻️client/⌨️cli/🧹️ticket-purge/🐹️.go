// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hands the client's ticket folders to the single generated-output purge of the tickets module.

// #endregion 🧲️Header

package client

import (
	ticketspkg "github.com/usalu/semio/repo/tickets"
)

// 🧹️purgeTicketGeneratedOutput plans, and with apply performs, the oversized generated-output purge of the given ticket folders, or of every closed ticket when none is given.
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json (PurgePolicy)
func purgeTicketGeneratedOutput(apply bool, ticketDirs ...string) (ticketspkg.PurgePlan, error) {
	options := ticketspkg.PurgeOptions{Root: GetRootDir(), TicketsDir: GetTicketsDir(), Selection: ticketspkg.PurgeOversized, Apply: apply}
	if len(ticketDirs) == 0 {
		return ticketspkg.PurgeClosedTicketGeneratedOutput(options)
	}
	return ticketspkg.PurgeTicketGeneratedOutput(options, ticketDirs)
}
