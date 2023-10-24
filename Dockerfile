FROM docker.io/rust:1.73-slim-buster as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build-env /app/target/release/ipfinder /app/ipfinder
EXPOSE 8080
CMD ["./app/ipfinder"]
