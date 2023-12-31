# syntax=docker/dockerfile:1
FROM scratch AS git-rknpu2

ADD https://github.com/rockchip-linux/rknpu2.git#4e2101af40a6cbe253eb680673846b7ec832a896 /

FROM milas/rock5-sdk AS build

# deps
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      libclang-dev \
    && rm -rf /var/lib/apt/lists/* \
    ;

# install rust
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

WORKDIR /code

COPY --link --from=git-rknpu2 / /code/crates/rknn_api-sys/rknpu2/

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/code/target \
    cargo build --color=always --release \
    && mkdir /out \
    && cp ./target/release/rknn-demo /out/ \
    ;

FROM debian:bullseye

COPY --link --from=git-rknpu2 /runtime/RK3588/Linux/librknn_api/aarch64/*.so /usr/local/lib/
RUN ldconfig

COPY ./models/*.rknn /

COPY --link --from=build /out/rknn-demo /usr/local/bin/
