FROM cffnpwr/git as git

FROM ubuntu:22.04

ARG UID=1000
ARG GID=1000
ARG USERNAME=developer

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

WORKDIR /workspace

RUN  apt-get update && \
  apt-get install -y software-properties-common && \
  add-apt-repository ppa:ubuntuhandbook1/ffmpeg6 && \
  apt-get update && \
  apt-get install -y --no-install-recommends sudo curl clang unzip build-essential pkg-config && \
  curl -L --output ~/wasi-sdk_21.0_amd64.deb https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-21/wasi-sdk_21.0_amd64.deb && \
  apt-get install -y ~/wasi-sdk_21.0_amd64.deb && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  groupadd --gid $GID $USERNAME && \
  useradd --uid $UID --gid $GID -m -s /bin/bash $USERNAME && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y && \
  rustup target add wasm32-wasi && \
  chmod -R 777 $RUSTUP_HOME $CARGO_HOME && \
  curl -fsSL https://deno.land/install.sh | DENO_INSTALL=/usr/local sh && \
  echo "${USERNAME}   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers 

COPY --from=git /usr/bin/git /usr/bin

USER ${USERNAME}