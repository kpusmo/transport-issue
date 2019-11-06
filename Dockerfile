FROM rust:1.37

WORKDIR /usr/src/app
RUN cargo install systemfd cargo-watch

#install dependencies
RUN mkdir src && echo "//aaa" > src/lib.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build
RUN rm -rf src

#build
COPY src src
RUN cargo build
ENV RUST_BACKTRACE=1