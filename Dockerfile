FROM ubuntu:22.04

RUN apt update && apt install -y \
    git \
    build-essential \
    wget \
    gcc \
    automake autoconf autotools-dev libtool cmake \
    curl ca-certificates

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

EXPOSE 80
EXPOSE 443