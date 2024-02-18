FROM rust:latest as builder
WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./
# Assuming you have a src directory
COPY ./src ./src
RUN cargo build --release

FROM rust:slim
RUN apt-get update && apt-get install libpq5 -y
COPY --from=builder /app/target/release/rusty .
COPY ./static ./static 
COPY ./templates ./templates 
ENTRYPOINT ["./rusty"]
