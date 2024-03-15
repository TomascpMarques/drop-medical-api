turso-migrate-db:
  turso db shell peaceful-mauler < database/schemas/main.sql

turso-dump-db:
  turso db shell peaceful-mauler .dump > database/schemas/dump.sql

turso-load-dump: turso-dump-db
  turso db shell peaceful-mauler < database/schemas/dump.sql

run-prod:
  export APP_ENV=production && cargo r

run-local:
  export APP_ENV=local && cargo r
