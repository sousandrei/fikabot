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

FROM ubuntu

ENV SLACK_TOKEN ""
ENV MONGO_URL ""
ENV PORT 8080

RUN apt update && \
    apt upgrade -y && \
    apt install -y libssl-dev

WORKDIR /opt/

# Add Tini
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]

COPY --from=builder /opt/fika/target/release/fikabot /opt/fikabot

RUN chmod +x /opt/fikabot

CMD [ "/opt/fikabot" ]