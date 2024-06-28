-- 2024-06-19-add-function.sql

CREATE OR REPLACE FUNCTION find_or_insert_today_with_balance(p_balance FLOAT)
    RETURNS SETOF history AS $$
BEGIN
    -- 오늘 날짜의 레코드를 찾습니다.
    IF NOT EXISTS (SELECT 1 FROM history WHERE date = CURRENT_DATE) THEN
        -- 없다면, 새로운 레코드를 balance와 함께 추가합니다.
        INSERT INTO history (date, balance) VALUES (CURRENT_DATE, p_balance);
    ELSE
        -- 이미 있는 경우 balance 값을 업데이트합니다.
        UPDATE history
        SET balance = p_balance
        WHERE date = CURRENT_DATE;
    END IF;

    -- 모든 레코드를 반환합니다.
    RETURN QUERY SELECT * FROM history;
END;
$$ LANGUAGE plpgsql;