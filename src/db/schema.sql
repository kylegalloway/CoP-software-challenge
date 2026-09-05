CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY,
    month INTEGER NOT NULL CHECK (month BETWEEN 1 AND 12),
    day INTEGER NOT NULL CHECK (day BETWEEN 1 AND 31),
    title TEXT NOT NULL,
    quote TEXT NOT NULL,
    source_url TEXT NOT NULL,
    secondary_source_url TEXT,
    needs_secondary_source INTEGER NOT NULL DEFAULT 0 CHECK (needs_secondary_source IN (0,1)),
    image_url TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','confirmed','rejected')),
    submitted_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_entries_month_day ON entries (month, day);

CREATE TABLE IF NOT EXISTS votes (
    id INTEGER PRIMARY KEY,
    entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
    direction INTEGER NOT NULL CHECK (direction IN (-1, 1)),
    voter_fingerprint TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (entry_id, voter_fingerprint)
);
