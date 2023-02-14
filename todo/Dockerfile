FROM rust:1.67 AS builder

WORKDIR /todo

COPY Cargo.toml Cargo.toml

# 依存関係だけ先にビルド
# 依存関係が変わらない場合キャッシュが使われる
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates

# ビルドしたやつのうち、アプリのものを削除
RUN rm -f target/release/deps/todo*

RUN cargo build --release

# -----
# リリース用
FROM debian:latest
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]


