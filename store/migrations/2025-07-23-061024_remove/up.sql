-- Your SQL goes here
-- regions table
ALTER TABLE regions
    ALTER COLUMN name TYPE text,
    ALTER COLUMN name SET NOT NULL;

-- users table
ALTER TABLE users
    ALTER COLUMN username TYPE text,
    ALTER COLUMN username SET NOT NULL,
    ALTER COLUMN password TYPE text,
    ALTER COLUMN password SET NOT NULL;

-- website_ticks table
ALTER TABLE website_ticks
    ALTER COLUMN status TYPE text,
    ALTER COLUMN status SET NOT NULL,
    ALTER COLUMN region_id TYPE text,
    ALTER COLUMN region_id SET NOT NULL;

-- websites table
ALTER TABLE websites
    ALTER COLUMN url TYPE text,
    ALTER COLUMN url SET NOT NULL,
    ALTER COLUMN user_id TYPE text,
    ALTER COLUMN user_id SET NOT NULL;
