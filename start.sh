#!/bin/bash

changeDirToPostgresSeed="cd postgres-seed/ &"
changeDirToRoot="cd .. &"
container_name="rust-travel-app-postgres-db-1"
start_ssr_app="cd rust-ssr-app && cargo run"

handle_error () {
    echo "An error occurred on line $1"
    exit 1
}
trap 'handle_error $LINENO' ERR

# NEED TO FIGURE OUT HOW TO RUN THIS AFTER INIT
run_app () {
  echo "Running tailwind" &
  npm run tailwind &
  wait
  echo "Running webpack" &
  npm run client &
  echo "Running rust app" &
  $start_ssr_app
}

handle_initial_run () {
  echo "Running docker-system prune --all ..." &
  docker system prune --all &
  wait
  echo "Running docker compose up..." &
  COMPOSE_HTTP_TIMEOUT=500 docker-compose up &
    echo "Cd into postgres-seed and build app..." &
    $changeDirToPostgresSeed
    cargo run build &
    wait
    $changeDirToRoot
    wait
    if test -d /postgres-seed/target/debug; then
      echo "Directory /postgres-seed/target/debug exists executing binary..." &
      cargo run /posrgres-seed/target/debug/postgres-seed &
    fi
}

handle_start () {
  echo "Do you want to prune all images/containers?: (y/n)"

  read ynPrune

  if [ $ynPrune = y ]; then
    echo "Running docker-system prune --all ..." &
    docker system prune --all &
    wait
  else
    echo "Running docker-compose down -v..." &
    docker-compose down -v &
    wait
  fi

  echo "Sleeping for 5 seconds..." &

  sleep 5 &

  wait
  echo "Did you install/update dependencies?: (y/n)"

  read yn

  if [ $yn = y ]; then
    echo "Creating build of postgres-seed..." &
    COMPOSE_HTTP_TIMEOUT=500 docker-compose up --build -V &
    # if [ "$( docker container inspect -f '{{.State.Status}}' $container_name )" = "running" ]; then
    echo "Cd into postgres-seed and run app..." &
    $changeDirToPostgresSeed
    cargo run &
    wait
    $changeDirToRoot
    wait
    # if [ test -d /postgres-seed/target/debug ]; then
    #   echo "Directory /postgres-seed/target/debug exists executing binary..." &
    #   cargo run /posrgres-seed/target/debug/postgres-seed &
    # fi
    # fi
    # UNTIL I CAN FIGURE OUT WHY RUST BILD COMMAND FAILS ON DOKCERFILE
    # docker stop postgres-seed &
    # docker-compose rm -f postgres-seed &
  else
    echo "Running docker-compose.yaml file and starting services..." &
    COMPOSE_HTTP_TIMEOUT=500 docker-compose up &
    # if [ "$( docker container inspect -f '{{.State.Status}}' $container_name )" = "running" ]; then
    echo "Cd into postgres-seed and run app..." &
    $changeDirToPostgresSeed
    cargo run &
    wait
    $changeDirToRoot
    wait
    # if [ test -d /postgres-seed/target/debug ]; then
    #   echo "Directory /postgres-seed/target/debug exists executing binary..." &
    #   cargo run /posrgres-seed/target/debug/postgres-seed &
    # fi
  # fi
  # UNTIL I CAN FIGURE OUT WHY RUST BILD COMMAND FAILS ON DOKCERFILE
  # docker stop postgres-seed &
  # docker-compose rm -f postgres-seed &
  fi
}

echo "Is this your first time running the app?: (y/n)"

read ynInitialRun

if [ $ynInitialRun = y ]; then
  handle_initial_run
else
  handle_start
fi
