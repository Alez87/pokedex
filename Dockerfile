FROM debian:stable-slim

WORKDIR /pokedex
ENV HOME /pokedex
ENV PATH="${HOME}:${HOME}/.cargo/bin:${HOME}/pokedex/target/release:${PATH}"

RUN apt-get update \
    && apt-get install -y curl build-essential git pkg-config libssl-dev \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rustup self update \
    && rustup toolchain install nightly-2021-05-31 --force \
    && rustup default nightly \
    && git clone https://github.com/Alez87/pokedex \
    && cd pokedex \
    && cargo build --release

EXPOSE 5000

WORKDIR ${HOME}/pokedex

CMD ["cargo", "run", "--release"]
