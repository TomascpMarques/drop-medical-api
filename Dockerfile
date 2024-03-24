# Containers OS image
FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 AS chef 
# Folder for the application
WORKDIR /app
# Required dependencies
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy all application files to the Docker image
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build our project
COPY . .
RUN cargo build --release --bin dropmedical 
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dropmedical dropmedical 
COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./dropmedical"]
