FROM docker.io/rust:1.80-bullseye as build

WORKDIR /app
COPY .. /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=build /app/target/release/iproxy /app/iproxy
COPY ../config.toml /app/config.toml
EXPOSE 8000
CMD ["./iproxy"]
