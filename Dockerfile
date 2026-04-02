FROM rust:1.94.1-slim-trixie AS base
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"
WORKDIR /app

RUN \
    apt update && apt upgrade -y && \
    apt install -y --no-install-suggests --no-install-recommends ca-certificates git git-lfs

RUN \
    git clone --depth 1 https://huggingface.co/onnx-community/gliner_small-v2.1 model && \
    git -C ./model lfs pull

FROM rust:1.94.1-slim-trixie AS builder
WORKDIR /app

RUN \
    apt update && apt upgrade -y && \
    apt install -y --no-install-suggests --no-install-recommends build-essential

COPY . /app
RUN cargo build --release --features cpu

FROM gcr.io/distroless/cc-debian13:nonroot
LABEL maintainer="Antoine Popineau <antoine.popineau@checkmarble.com>"

WORKDIR /app
USER nonroot

COPY --from=builder /app/target/release/ner /ner
COPY --from=base /app/model /tmp/model

ENV MODEL_PATH=/tmp/model
ENTRYPOINT ["/ner"]
