FROM docker.io/rust:1.80-bullseye as build

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=build /app/target/release/iproxy /app/iproxy

EXPOSE 8000
CMD ["./iproxy"]
