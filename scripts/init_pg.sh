#!/usr/bin/env bash
set -x
set -eo pipefail

PG_USER=${POSTGRES_USER:=postgres}
PG_PASSWORD="${POSTGRES_PASSWORD:=password}"
PG_DB="${POSTGRES_DB:=app-itest}"
PG_PORT="${POSTGRES_PORT:=5432}"

docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${PG_DB} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
