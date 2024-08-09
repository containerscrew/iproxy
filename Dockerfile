FROM docker.io/rust:1.80-bullseye as build-env

WORKDIR /app
COPY . /app

RUN cargo build --release

#FROM gcr.io/distroless/cc
#WORKDIR /app
#COPY --from=build-env /app/target/release/iproxy /app/iproxy
#EXPOSE 8080
#CMD ["./app/iproxy"]
