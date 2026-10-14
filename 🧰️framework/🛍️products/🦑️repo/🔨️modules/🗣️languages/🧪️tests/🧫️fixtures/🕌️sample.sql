-- #region 🔖️Alpha
CREATE TABLE IF NOT EXISTS public.shape (
  sides INT NOT NULL
);

CREATE OR REPLACE VIEW public.wide_shape AS
SELECT sides FROM public.shape;
-- #endregion 🔖️Alpha
