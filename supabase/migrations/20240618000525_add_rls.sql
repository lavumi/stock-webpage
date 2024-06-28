ALTER TABLE public.history ENABLE ROW LEVEL SECURITY;


CREATE POLICY anon_select_policy
    ON public.history
    AS PERMISSIVE
    FOR ALL
    TO public
    USING (auth.role() = 'anon');