FROM rust as builder

WORKDIR /opt/fika

# Cache build dependencies
ADD .gitignore Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() {println!(\"If you see this, your docker build failed\");}" >> src/main.rs
RUN cargo build --release

# Remove temporary main and actually build our code
RUN rm -rf ./target/release/.fingerprint/fikabot*
ADD src src
RUN cargo build --release

FROM rust

WORKDIR /opt/

COPY --from=builder /opt/fika/target/release/fikabot .

RUN chmod +x /opt/fikabot

ENTRYPOINT [ "/opt/fikabot" ]