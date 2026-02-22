SELECT
    'CREATE DATABASE newsletter'
WHERE
    NOT EXISTS(
        SELECT
            pg_database.datname
        FROM
            pg_database
        WHERE
            datname = 'newsletter'
    ) \gexec
