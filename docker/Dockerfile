ARG RUST_BUILDER
ARG PROD_IMAGE_TAG
ARG S6_OVERLAY_VERSION
FROM ${RUST_BUILDER} as builder
RUN apt-get update && apt-get -y install cmake libssl-dev
WORKDIR build
COPY Cargo.toml .
COPY src/ ./src
RUN cargo build --release --bin pep_bot

FROM ubuntu:${PROD_IMAGE_TAG}
ARG S6_OVERLAY_VERSION
ADD https://github.com/just-containers/s6-overlay/releases/download/${S6_OVERLAY_VERSION}/s6-overlay-amd64-installer /tmp/
RUN chmod +x /tmp/s6-overlay-amd64-installer && /tmp/s6-overlay-amd64-installer /
RUN useradd -ms /bin/bash pepuser
RUN apt-get update && apt-get -y install ca-certificates dnsutils iproute2
COPY docker/root/ /
WORKDIR pep_bot
VOLUME /pep_bot/session
VOLUME /pep_bot/store
COPY --from=builder /build/target/release/pep_bot /usr/local/bin
ENV RUST_LOG=INFO
ENTRYPOINT ["/init"]
