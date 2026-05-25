FROM ubuntu:22.04

# 1. OSのアップデートと、自作OSに必要な基本ツール（QEMUなど）のインストール
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    qemu-system-x86 \
    && rm -rf /var/lib/apt/lists/*

# 2. Rust (Nightly) のインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

# 3. 自作OS用のコンポーネントとbootimageツールを追加
RUN rustup component add rust-src llvm-tools-preview \
    && cargo install bootimage

WORKDIR /workspace