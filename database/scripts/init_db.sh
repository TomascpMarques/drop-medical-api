#!/usr/bin/env bash

set -x
set -eo pipefail

# Check if psql is installed, if not abort
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed"
    exit 1
fi

# Check if sqlx is installed, if not abort
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed"
    echo >&2 "Use: "
    echo >&2 " cargo install --version='~0.7' sqlx-cli \
--no-default-features --features rustls,postgres"
    echo >&2 "to install it."
    exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=main}"
# Check if a custom port has been set, otherwise default to '2345'
DB_PORT="${POSTGRES_PORT:=2345}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]];
then

    docker run \
        --name dropmedical_pg \
        -e POSTGRES_HOST="${DB_HOST}" \
        -e POSTGRES_USER="${DB_USER}" \
        -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
        -e POSTGRES_DB="${DB_NAME}" \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
fi

# Keep pinging Postgres until it's ready for connection
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping..."
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# Database URL and export env variable
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
echo "Postgres url: ${DATABASE_URL}"

# SQLX create data base
sqlx database create

# SQLX run maigrations
sqlx migrate run --source database/migrations
>&2 echo "Postgres migrations were done, ready to go!"

# SQLX prepare offline query syntax
#cargo sqlx prepare -- --tests
# >&2 echo "Run sqlx prepare"
