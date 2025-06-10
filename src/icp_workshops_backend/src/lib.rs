use ic_cdk::api::management_canister::http_request::HttpHeader;
use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::HttpMethod;

#[ic_cdk::update]
async fn translate(text: String) -> String {
    let token = "";
    
// pub struct CanisterHttpRequestArgument {
//     pub url: String,
//     pub max_response_bytes: Option<u64>,
//     pub method: HttpMethod,
//     pub headers: Vec<HttpHeader>,
//     pub body: Option<Vec<u8>>,
//     pub transform: Option<TransformContext>,
// }
   let (res,) = http_request(
        CanisterHttpRequestArgument {
            url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers: vec![
                      HttpHeader {
                    name: "Authorization".to_string(),
                    value: format!("Bearer {}", token),
                },
                             HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                }
            ],
            body: Some(format!(r#"{{"inputs": "{}"}}"#, text).into()),
            transform: None,
        }, (20_849_972_800u128 + text.len() as u128 * 5_200u128)
    ).await.unwrap();

    

    let body = String::from_utf8(res.body).unwrap();
   ic_cdk::println!("{:?}", body);
    format!("Hello, {}!", text)
}

