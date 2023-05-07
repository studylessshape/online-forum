use actix_web::HttpRequest;
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

pub mod email;
pub mod error;

/// Steal the ownership of `t`.
///
/// **Attention**: Don't use `t` after stealing.
pub fn into_mem<T>(t: &T) -> T {
    unsafe {
        return (t as *const T as *mut T).read();
    }
}

/// Extract value from request's cookies.
///
/// When don't have this cookie or ouccr error when cookie from `str` to `T`, it returns `None`. Or returns Some(T).
pub fn extract_cookie<T>(req: &HttpRequest, cookie_name: &str) -> Option<T>
where
    T: FromStr,
{
    match req.cookie(cookie_name) {
        Some(val) => match T::from_str(val.value()) {
            Ok(res) => Some(res),
            Err(_) => None,
        },
        None => None,
    }
}

/// Get local address method from [@egmkang](https://github.com/egmkang/local_ipaddress)
pub fn get_local_addr() -> std::io::Result<SocketAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    socket.connect("8.8.8.8:80")?;

    socket.local_addr()
}
