CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    amount REAL NOT NULL,
    category TEXT NOT NULL,
    date TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert some sample data
INSERT INTO expenses (description, amount, category) VALUES 
    ('Coffee', 45.0, 'food'),
    ('Bus ticket', 35.0, 'transport'),
    ('Lunch', 120.0, 'food');
