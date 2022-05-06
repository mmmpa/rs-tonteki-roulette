use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use lambda_runtime::{service_fn, LambdaEvent};
use rand::prelude::*;
use rand::SeedableRng;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::Digest;
use sha2::Sha256;

pub type Error = Box<dyn std::error::Error + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct StaticUrlEvent {
    #[serde(rename = "requestContext")]
    request_context: RequestContext,
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    body: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct RequestContext {
    http: RequestContextHttp,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct RequestContextHttp {
    method: String,
    path: String,
    protocol: String,
    #[serde(rename = "sourceIp")]
    source_ip: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
}

#[derive(Deserialize, Debug)]
struct Params {
    name: Option<String>,
}

const TONTEKI_SRC: [char; 4] = ['と', 'ん', 'て', 'き'];
const INDEX_HTML: &str = include_str!("../html/index.html");
const RESULT_HTML: &str = include_str!("../html/result.html");

#[tokio::main]
async fn main() -> std::result::Result<(), lambda_runtime::Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<StaticUrlEvent>) -> Result<Value> {
    let (
        StaticUrlEvent {
            body,
            is_base64_encoded,
            ..
        },
        _context,
    ) = event.into_parts();

    match body.and_then(|body| parse(&body, is_base64_encoded).ok()) {
        Some(Params { name: Some(name) }) if !name.is_empty() => response(&tonteki_html(&name)),
        _ => response(INDEX_HTML),
    }
}

fn response(body: &str) -> Result<Value> {
    Ok(json!({
        "statusCode": 200,
        "headers": {
            "content-type": "text/html",
        },
        "body": body
    }))
}

fn parse(body: &str, base64_encoded: bool) -> Result<Params> {
    let body = if base64_encoded {
        serde_urlencoded::from_bytes(&base64::decode(body)?)?
    } else {
        serde_urlencoded::from_bytes(body.as_bytes())?
    };

    Ok(body)
}

fn today() -> String {
    Utc::now()
        .with_timezone(&Tokyo)
        .format("%Y-%m-%d")
        .to_string()
}

fn tonteki_html(name: &str) -> String {
    RESULT_HTML.replace("{{tonteki}}", &tonteki(name, &today()))
}

fn tonteki(name: &str, today: &str) -> String {
    let seed = generate_random_seed(name, today);
    let mut rng = rand::rngs::StdRng::from_seed(seed);

    (0..4)
        .into_iter()
        .map(|_| *TONTEKI_SRC.choose(&mut rng).unwrap())
        .collect()
}

fn generate_random_seed(name: &str, today: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(name);
    hasher.update(today);
    hasher
        .finalize()
        .iter()
        .enumerate()
        .fold([0u8; 32], |mut a, (i, b)| {
            a[i] = *b;
            a
        })
}

#[cfg(test)]
mod tests {
    use crate::{parse, tonteki, StaticUrlEvent};
    use chrono::Utc;
    use chrono_tz::Asia::Tokyo;

    #[test]
    fn test_rand() {
        let today = Utc::now()
            .with_timezone(&Tokyo)
            .format("2022-01-01")
            .to_string();

        assert_eq!(tonteki("ふとし", &today), "てとんん");
    }

    #[test]
    fn test_event() {
        const JSON: &str = r#"
            {
              "version": "2.0",
              "routeKey": "$default",
              "rawPath": "/",
              "rawQueryString": "",
              "headers": {
                "sec-fetch-mode": "navigate",
                "referer": "https://dummy.lambda-url.ap-northeast-1.on.aws/",
                "sec-fetch-site": "same-origin",
                "accept-language": "ja-JP,ja;q=0.9,en-US;q=0.8,en;q=0.7",
                "x-forwarded-proto": "https",
                "origin": "https://dummy.lambda-url.ap-northeast-1.on.aws",
                "x-forwarded-port": "443",
                "x-forwarded-for": "",
                "sec-fetch-user": "?1",
                "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
                "sec-ch-ua": "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"99\", \"Google Chrome\";v=\"99\"",
                "sec-ch-ua-mobile": "?0",
                "x-amzn-trace-id": "",
                "sec-ch-ua-platform": "\"Linux\"",
                "host": "dummy.lambda-url.ap-northeast-1.on.aws",
                "upgrade-insecure-requests": "1",
                "content-type": "application/x-www-form-urlencoded",
                "cache-control": "max-age=0",
                "accept-encoding": "gzip, deflate, br",
                "sec-fetch-dest": "document",
                "user-agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.84 Safari/537.36"
              },
              "requestContext": {
                "accountId": "anonymous",
                "apiId": "dummy",
                "domainName": "dummy.lambda-url.ap-northeast-1.on.aws",
                "domainPrefix": "dummy",
                "http": {
                  "method": "POST",
                  "path": "/",
                  "protocol": "HTTP/1.1",
                  "sourceIp": "",
                  "userAgent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.84 Safari/537.36"
                },
                "requestId": "",
                "routeKey": "$default",
                "stage": "$default",
                "time": "05/May/2022:21:41:22 +0000",
                "timeEpoch": 1651786882006
              },
              "body": "bmFtZT0lRTMlODElQjUlRTMlODElQTglRTMlODElOTc=",
              "isBase64Encoded": true
            }
        "#;

        let event: StaticUrlEvent = serde_json::from_str(JSON).unwrap();
        let req = parse(&event.body.unwrap(), event.is_base64_encoded).unwrap();

        assert_eq!(req.name.unwrap(), "ふとし")
    }
}
