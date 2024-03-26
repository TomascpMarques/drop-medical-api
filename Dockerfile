FROM rust:1-buster as builder
WORKDIR /usr/src/dropmedical
COPY . .
RUN apt update && apt install lld clang -y
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo install --profile release --path .

FROM debian:bookworm-slim AS runtime
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/dropmedical /usr/local/bin/dropmedical 
COPY configuration configuration

ENV APP_ENVIRONMENT production
EXPOSE 8080

COPY . .
RUN rm Cargo.*
CMD ["dropmedical"]
