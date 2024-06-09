CREATE TABLE stocks (
    id INT PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    owned INT NOT NULL,
    cost DECIMAL(10,3) NOT NULL,
    update_at DATE NOT NULL,
    company_name VARCHAR(50) NOT NULL,
    close_price NUMERIC(10,2) NOT NULL,
    highest_price NUMERIC(10,2) NOT NULL,
    open_price NUMERIC(10,2) NOT NULL,
    lowest_price NUMERIC(10,2) NOT NULL,
    yesterday_price NUMERIC(10,2) NOT NULL
);

