-- Drop FK constraint
ALTER TABLE website_ticks
    DROP CONSTRAINT website_ticks_website_id_fkey;

-- Change websites.id to text
ALTER TABLE websites
    ALTER COLUMN id TYPE text;

-- Change website_ticks.website_id to text
ALTER TABLE website_ticks
    ALTER COLUMN website_id TYPE text;

-- Re-add FK constraint
ALTER TABLE website_ticks
    ADD CONSTRAINT website_ticks_website_id_fkey
    FOREIGN KEY (website_id) REFERENCES websites(id);
