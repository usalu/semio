// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// The breach listing handler.

// #endregion 🧲️Header

package main

import (
	"net/http"
)

// #region 🚨️Breaches

// 🔷️handleBreachs returns breachs optionally filtered by ticket ID.
func (s *Server) handleBreachs(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	breachs, err := s.db.projectBreachs(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, breachs)
}

// #endregion 🚨️Breaches
