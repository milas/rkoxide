# rkoxide

> ⚠️ Alpha - I have no idea what I'm doing in Rust

Rust wrappers for Rockchip C libraries

## Libraries
### rknn_api
See the `rknn_api-sys` crate generated using bindgen.

You'll need to run `git submodule update --init --recursive` or similar to get `./crates/rknn_api-sys/rknpu2/`.

## Demo
Copy a model to `./models`, e.g. `./models/yolov5s-640-640.rknn`.

Build & run with Docker Compose (on an RK3588 device):
```
MODEL=yolov5s-640-640.rknn docker compose run --build rknn-demo

... lots of build output ...

[2023-08-11T00:53:46Z INFO  rknn_demo] rknn_init: success
[2023-08-11T00:53:46Z INFO  rknn_demo] rknn_query versions: sdk[1.5.0 (e6fe0c678@2023-05-25T08:09:20)] driver[0.8.2]
[2023-08-11T00:53:46Z INFO  rknn_demo] rknn_query model size: input[1] output[3]
[2023-08-11T00:53:46Z INFO  rknn_demo] rknn_query attrs: model 640x640x3
```

All it does is load the model and output some basic info.
This is basically a Rust adaptation of the C code from [avafinger/ff-rknn](https://github.com/avafinger/ff-rknn/blob/c4fdcd6e8b4bb4a4007f901a85dd59892983d33f/ff-rknn.c#L627-L691).
(Except it doesn't actually do anything useful yet like run the model / decode video / render output.)
