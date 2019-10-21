FROM ubuntu:18.04
WORKDIR /repo
ENV PATH="${PATH}:/root/.cargo/bin"
RUN apt-get update \
  && apt-get install -y curl build-essential \
  && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=none

COPY rust-toolchain .

# fe9baed is head at time of writing, no other reason.
RUN cargo install --git "https://github.com/leeola/cargo-build-deps.git" --rev "fe9baed"

RUN apt-get install -y libssl-dev
RUN apt-get install -y pkg-config

# build the deps, to ease the burden of Rust (hah).
COPY Cargo.lock .
COPY Cargo.toml .
COPY cli/Cargo.toml cli/Cargo.toml
COPY gui/Cargo.toml gui/Cargo.toml
COPY mnemosyne/Cargo.toml mnemosyne/Cargo.toml
COPY server/Cargo.toml server/Cargo.toml
# `build -p foo` seems to have trouble with building packages from the workspace. Not sure why.
RUN cd server && cargo build-deps \
  --ignore-pkg "mnemosyne" \
  --ignore-pkg "mnemosyne-gui" \
  --ignore-pkg "mnemosyne-server"

COPY . /repo
# temporarily using debug, non-release build
RUN cargo build --bin "nem-server" \
  && mv target/debug/nem-server /build
