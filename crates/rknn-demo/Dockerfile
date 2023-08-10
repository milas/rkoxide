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
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/code/target \
    cargo build --color=always --release \
    && mkdir /out \
    && cp ./target/release/rknn-demo /out/ \
    ;

FROM debian:bookworm

COPY ./models/*.rknn /

COPY --link --from=build /code/crates/rknn_api-sys/rknpu2/runtime/RK3588/Linux/librknn_api/aarch64/*.so /usr/local/lib/
RUN ldconfig

COPY --link --from=build /out/rknn-demo /usr/local/bin/
#RUN ["rknn-demo"]
