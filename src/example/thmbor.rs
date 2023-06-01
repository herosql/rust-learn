/*
一个图片服务器,通过url可以对图片进行动态转换

    需求分析:
        一个http服务
        提供图片动态转换的接口

    方案:
        java
            springboot
                5分钟项目初始化
            写接口
                支持功能 图片裁剪
                    operate 是否必填(是)
                        文本类型,枚举: 缩放(Scale),裁剪(Cropp),旋转(Rotate)
                    width 是否必填(否)
                        数字,不为负数(单位像素)
                    height 是否必填(否)
                        数字,不为负数(单位像素)
            找一个图片库
            对文件流进行操作
            估计1小时左右
            不到100行代码
        rust
            使用protobuf定义传输的数据


 */
pub use super::abi::*;
use axum::{extract::Path, handler::get, http::StatusCode, Router};
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use percent_encoding::percent_decode_str;
use photon_rs::transform::SamplingFilter;
use prost::Message;
use serde::Deserialize;
use std::convert::TryFrom;
use std::convert::TryInto;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让ImageSpec 生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}
// 让 ImageSpec 可以通过一个字符串创建.
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photon_rs 相应的方法里需要字符串
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

// 在我们定义的SampleFilter 和 photon_rs的SamplingFilter间转化
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

// 提供一些辅助函数，让创建一个 spec 的过程简单一些
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }
    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }
    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}
//-------------------------------------
//http服务器
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new().route("/image/:spec/:url", get(generate));

    // 运行web服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
use anyhow::Result;
use axum::{
    extract::Extension,
    http::{HeaderMap, HeaderValue},
    AddExtensionLayer,
};
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{info, instrument};
// -------------------获取源图并缓存

// 测试模块
#[cfg(test)]
mod tests {
    // 引入外部函数
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;
    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }

    #[test]
    fn test_web() {
        // main();
    }
}
