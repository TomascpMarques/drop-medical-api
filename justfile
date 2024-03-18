init-db:
  bash ./database/scripts/init_db.sh

clean-db:
  docker stop dropmedical_pg; \
  docker rm dropmedical_pg;

migrate-db:
  sqlx migrate run --source database/migration  

run-prod:
  export APP_ENV=production && cargo r

run-local:
  export APP_ENV=local && cargo r
