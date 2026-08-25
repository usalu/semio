// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package humanize renders stable, concise relative timestamps.

// #endregion 🧲️Header

package humanize

import (
	"fmt"
	"time"
)

// #region ⏳️RelativeTime

func Time(value time.Time) string {
	delta := time.Since(value)
	future := delta < 0
	if future {
		delta = -delta
	}
	amount, unit := 0, ""
	switch {
	case delta < time.Minute:
		amount, unit = max(1, int(delta/time.Second)), "second"
	case delta < time.Hour:
		amount, unit = int(delta/time.Minute), "minute"
	case delta < 24*time.Hour:
		amount, unit = int(delta/time.Hour), "hour"
	case delta < 30*24*time.Hour:
		amount, unit = int(delta/(24*time.Hour)), "day"
	case delta < 365*24*time.Hour:
		amount, unit = int(delta/(30*24*time.Hour)), "month"
	default:
		amount, unit = int(delta/(365*24*time.Hour)), "year"
	}
	if amount != 1 {
		unit += "s"
	}
	if future {
		return fmt.Sprintf("%d %s from now", amount, unit)
	}
	return fmt.Sprintf("%d %s ago", amount, unit)
}

// #endregion ⏳️RelativeTime
