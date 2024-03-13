----------------------------------------------------------------
-- Description: Migration that initializes the basic user schema.
--
-- Version: 0.0.1
--
-- @author: Stavros Grigoriou <unix121@protonmail.com>
----------------------------------------------------------------

--------------
-- Table: role
--------------
CREATE TABLE role
(
    role        VARCHAR(30) NOT NULL PRIMARY KEY,
    description TEXT        NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE role IS 'Roles of the application and registered applications';

-- Insert the two basic roles of the application
INSERT INTO role (role, description)
VALUES ('AUTHGATE_ADMIN', 'Global administrator of the application'),
       ('AUTHGATE_USER', 'Authgate registered user');

---------------
-- Table: user
---------------
CREATE TABLE "user"
(
    id         UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    username   VARCHAR(20) UNIQUE,
    email      VARCHAR(255) UNIQUE,
    password   VARCHAR(100) NOT NULL,
    locked     BOOLEAN      NOT NULL DEFAULT TRUE,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE "user" IS 'List of registered users of the application';

-------------------
-- Table: user_role
-------------------
CREATE TABLE user_role
(
    user_id    UUID,
    role       VARCHAR(30),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT user_role_pk PRIMARY KEY (user_id, role),
    CONSTRAINT user_role_user_id_fk FOREIGN KEY (user_id) REFERENCES "user" (id),
    CONSTRAINT user_role_role_fk FOREIGN KEY (role) REFERENCES role (role)
);

