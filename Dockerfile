FROM rust as builder

WORKDIR /opt/fika

ADD .gitignore Cargo.lock Cargo.toml ./
ADD src src

RUN cargo build --release

FROM rust

WORKDIR /opt/

COPY --from=builder /opt/fika/target/release/fika .

RUN chmod +x /opt/fika

ENTRYPOINT [ "/opt/fika" ]