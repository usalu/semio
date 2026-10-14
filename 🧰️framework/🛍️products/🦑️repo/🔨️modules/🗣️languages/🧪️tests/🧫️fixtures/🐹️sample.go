package sample

// #region 🔖️Alpha
type Shape struct {
	Sides int
}

func (s Shape) Perimeter(length int) int {
	return s.Sides * length
}
// #endregion 🔖️Alpha

// #region 🔖️Beta
const Answer = 42
// #endregion 🔖️Beta
