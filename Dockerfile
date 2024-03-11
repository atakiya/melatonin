FROM rust AS build

WORKDIR /tmp/build

COPY . .

RUN cargo install --path .

FROM ubuntu:latest

COPY --from=build /usr/local/cargo/bin/* /usr/local/bin/
