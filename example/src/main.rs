extern crate reqwest;

use hyperx::http_client;
use reqwest::{Response, Result};

#[http_client(OpenWeatherMapsApiClient)]
trait OpenWeatherMapsApi {
    #[get(path = "forecast?id={id}&appid={key}")]
    fn get_forecast(&self, id: i32, key: String) -> Result<Response>;
}

#[http_client(JsonPlaceholderApiClient)]
trait JsonPlaceholderApi {
    #[get(path = "/comments?postId={post_id}")]
    fn get_comments(&self, post_id: i32) -> Result<Response>;

    #[post(path = "/posts", data = "{data}")]
    fn create_post(&self, data: String) -> Res;
}

struct Res {
    code: u16,
    body: String,
}

impl From<Result<reqwest::Response>> for Res {
    fn from(res: Result<reqwest::Response>) -> Res {
        let mut oof = res.unwrap();
        Res {
            code: oof.status().as_u16(),
            body: oof.text().unwrap_or_default(),
        }
    }
}

fn main() {
    // let client = reqwest::Client::new();
    // let api = OpenWeatherMapsApiClient::new("https://samples.openweathermap.org/data/2.5", client);
    // let _ = api.get_forecast(524901, "b1b15e88fa797225412429c1c50c122a1".to_string());

    let client = reqwest::Client::new();
    let api = JsonPlaceholderApiClient::new("https://jsonplaceholder.typicode.com", client);
    let mut response = api.get_comments(1).expect("Unsuccessful Api call");
    println!("{}", response.text().unwrap());

    let post_response = api.create_post(
        r#"
    {
      title: 'foo',
      body: 'bar',
      userId: 1
    }
    "#
        .to_string(),
    );
    println!("Status code: {}", post_response.code);
    println!("Response Body: \n{}", post_response.body);
}
