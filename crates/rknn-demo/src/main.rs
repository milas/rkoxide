use anyhow::anyhow;
use clap::Parser;
use log;
use rknn_api_sys::{
    _rknn_query_cmd_RKNN_QUERY_INPUT_ATTR, _rknn_query_cmd_RKNN_QUERY_IN_OUT_NUM,
    _rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR, _rknn_query_cmd_RKNN_QUERY_SDK_VERSION,
    _rknn_tensor_format_RKNN_TENSOR_NCHW, rknn_context, rknn_init, rknn_input_output_num,
    rknn_query, rknn_sdk_version, rknn_tensor_attr,
};
use std::ffi::CStr;
use std::fs;
use std::mem::{size_of, MaybeUninit};
use std::os::raw::c_void;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    model_path: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let mut ctx: rknn_context = 0;
    let mut model_data = fs::read(args.model_path)?;
    unsafe {
        let ret = rknn_init(
            &mut ctx,
            model_data.as_mut_ptr() as *mut c_void,
            model_data.len() as u32,
            0,
            std::ptr::null_mut(),
        );
        if ret < 0 {
            return Err(anyhow!("rknn_init: {}", ret));
        }
    }

    log::info!("rknn_init: success");

    unsafe {
        let mut version = MaybeUninit::<rknn_sdk_version>::uninit();
        let ret = rknn_query(
            ctx,
            _rknn_query_cmd_RKNN_QUERY_SDK_VERSION,
            version.as_mut_ptr() as *mut c_void,
            size_of::<rknn_sdk_version>() as u32,
        );
        if ret < 0 {
            return Err(anyhow!("rknn_query versions: {}", ret));
        }
        let version = version.assume_init();
        log::info!(
            "rknn_query versions: sdk[{}] driver[{}]",
            CStr::from_ptr(version.api_version.as_ptr())
                .to_str()
                .unwrap_or_default(),
            CStr::from_ptr(version.drv_version.as_ptr())
                .to_str()
                .unwrap_or_default(),
        )
    }

    unsafe {
        let mut ionum = MaybeUninit::<rknn_input_output_num>::uninit();
        let ret = rknn_query(
            ctx,
            _rknn_query_cmd_RKNN_QUERY_IN_OUT_NUM,
            ionum.as_mut_ptr() as *mut c_void,
            size_of::<rknn_input_output_num>() as u32,
        );
        if ret < 0 {
            return Err(anyhow!("rknn_query model size: {}", ret));
        }
        let ionum = ionum.assume_init();
        log::info!(
            "rknn_query model size: input[{}] output[{}]",
            ionum.n_input,
            ionum.n_output
        );

        let mut input_attrs: Vec<rknn_tensor_attr> =
            Vec::<rknn_tensor_attr>::with_capacity((ionum.n_input + 1) as usize);
        for i in 0..ionum.n_input {
            let mut attr = MaybeUninit::<rknn_tensor_attr>::zeroed().assume_init();
            attr.index = i;
            let ret = rknn_query(
                ctx,
                _rknn_query_cmd_RKNN_QUERY_INPUT_ATTR,
                &mut attr as *mut _ as *mut c_void,
                size_of::<rknn_tensor_attr>() as u32,
            );
            if ret < 0 {
                return Err(anyhow!("rknn_query input attr: {}", ret));
            }
            input_attrs.insert(i as usize, attr);
        }

        let mut output_attrs: Vec<rknn_tensor_attr> =
            Vec::<rknn_tensor_attr>::with_capacity(ionum.n_output as usize);
        for i in 0..ionum.n_output {
            let mut attr = MaybeUninit::<rknn_tensor_attr>::zeroed().assume_init();
            attr.index = i;
            let ret = rknn_query(
                ctx,
                _rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR,
                &mut attr as *mut _ as *mut c_void,
                size_of::<rknn_tensor_attr>() as u32,
            );
            if ret < 0 {
                return Err(anyhow!("rknn_query output attr: {}", ret));
            }
            output_attrs.insert(i as usize, attr);
        }

        let channel: u32;
        let width: u32;
        let height: u32;
        if input_attrs[0].fmt == _rknn_tensor_format_RKNN_TENSOR_NCHW {
            let attr = input_attrs[0];
            channel = attr.dims[1];
            width = attr.dims[2];
            height = attr.dims[3];
        } else {
            let attr = input_attrs[0];
            width = attr.dims[1];
            height = attr.dims[2];
            channel = attr.dims[3];
        }
        log::info!("rknn_query attrs: model {}x{}x{}", width, height, channel);
    }
    Ok(())
}
