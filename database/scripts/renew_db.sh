#!/usr/bin/env bash

set -eo pipefail

# Check if container is running
if [[ $(docker ps -l -q -f name="dropmedical_pg") ]]; then
    echo "OK - Cleaning docker container..."
    docker stop dropmedical_pg
    docker rm dropmedical_pg
else
    echo "OK - No docker container running"
fi

echo "SQL - Starting db"
bash ./database/scripts/init_db.sh
echo "SQL - Running migrations"
cargo sqlx migrate run --source database/migrations
echo "SQL - Migrations Done!!!"
