FROM rust:slim

ENV USER=${USER}

# Node.js version
ENV NODE_VERSION=14

# Executables versions
ENV CARGO_GENERATE_VERSION=v0.5.1
ENV NVM_VERSION=v0.37.2
ENV WRANGLER_VERSION=v1.13.0

# Install curl to download install scripts below
RUN apt-get update \
  && apt-get install -y --no-install-recommends curl

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install cargo-generate
RUN curl -fsSL https://github.com/cargo-generate/cargo-generate/releases/download/${CARGO_GENERATE_VERSION}/cargo-generate-${CARGO_GENERATE_VERSION}-x86_64-unknown-linux-musl.tar.gz | tar zx --strip-components 1 -C /usr/local/cargo/bin

# Install wrangler
RUN curl -fsSL https://github.com/cloudflare/wrangler/releases/download/${WRANGLER_VERSION}/wrangler-${WRANGLER_VERSION}-x86_64-unknown-linux-musl.tar.gz | tar zx --strip-components 1 -C /usr/local/cargo/bin 

# Install nvm, will auto-install Node.js version defined in NODE_VERSION
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/${NVM_VERSION}/install.sh | bash

# Uninstall curl again, as it's not needed anymore
RUN apt-get remove -y --autoremove curl