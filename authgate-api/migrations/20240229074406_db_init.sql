----------------------------------------------------------------
-- Description: Migration that initializes the basic user schema.
--
-- Version: 0.0.1
--
-- @author: Stavros Grigoriou <unix121@protonmail.com>
----------------------------------------------------------------

---------------
-- Table: users
---------------
CREATE TABLE users
(
    id         UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    username   VARCHAR(20) UNIQUE,
    email      VARCHAR(255) UNIQUE,
    password   VARCHAR(100) NOT NULL,
    locked     BOOLEAN      NOT NULL DEFAULT TRUE,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE users IS 'List of registered users of the application';