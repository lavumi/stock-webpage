ALTER TABLE public.stocks ENABLE ROW LEVEL SECURITY;


CREATE POLICY anon_select_policy
ON public.stocks
AS PERMISSIVE
FOR ALL
TO public
USING (auth.role() = 'anon');