COPY public.stocks(id,symbol,owned,cost,update_at,company_name,close_price,highest_price,open_price,lowest_price,yesterday_price)
FROM './stocks.csv'
DELIMITER ','
CSV HEADER;