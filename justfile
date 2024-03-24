# Starts the dev loop
dev: 
  bash ./database/scripts/renew_db.sh; \
  export APP_ENV=local && \
  cargo watch -x check \
    -x clippy \
    -x "test -- --nocapture" \
    -x "test -- --ignored" \

# Start the Postgre docker container
init-db:
  bash ./database/scripts/init_db.sh

# Stop and remove the Postgre docker container
clean-db:
  docker stop dropmedical_pg; \
  docker rm dropmedical_pg;

# Resume (must exist) the Postgre container
resume-db:
  docker start dropmedical_pg

# Connect to the development database 
psql:
  psql -h localhost -U postgres -p 2345 -d main

# Run migrations using sqlx (local)
migrate-db:
  cargo sqlx migrate run --source database/migrations

# Create a new migration with the given name
migrate-new MIGRATION_NAME:
  cargo sqlx migrate add --source ./database/migrations {{MIGRATION_NAME}}

# Prepare the off-line support for sqlx queries 
sqlx-prepare:
  cargo sqlx prepare -- --workspace
  cargo sqlx prepare -- --tests --workspace

# Run the web-app in local mode
run-prod:
  export APP_ENV=production && cargo r

# Run the web-app in production mode
run-local:
  export APP_ENV=local && cargo r
