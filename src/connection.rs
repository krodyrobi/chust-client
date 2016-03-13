
#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ClientRequest {
    Auth(String, String),
    Reg(String, String),
    Send(String)
}

#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Response {
    Ok,
    Err(u8, String)
}
