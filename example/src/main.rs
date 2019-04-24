use hyperx::http_client;
use reqwest::{Response, Result};


#[http_client]
trait OpenWeatherMapsApi {
    #[get(path = "forecast?id={id}&appid={key}")]
    fn get_forecast(&self, id: i32, key: String) -> Result<Response>;
}

#[http_client]
trait JsonPlaceholderApi {
    #[get(path = "/comments?postId={post_id}")]
    fn get_comments(&self, post_id: i32) -> Result<Response>;

    #[post(path = "/posts", data = "{data}")]
    fn create_post(&self, data: String) -> Result<Response>;
}

fn main() {
    // let client = reqwest::Client::new();
    // let api = OpenWeatherMapsApi::new("https://samples.openweathermap.org/data/2.5", client);
    // let _ = api.get_forecast(524901, "b1b15e88fa797225412429c1c50c122a1".to_string());

    let client = reqwest::Client::new();
    let api = JsonPlaceholderApi::new("https://jsonplaceholder.typicode.com", client);
    let mut response = api.get_comments(1).expect("Unsuccessful Api call");
    println!("{}", response.text().unwrap());


    let mut post_response = api.create_post(r#"
    {
      title: 'foo',
      body: 'bar',
      userId: 1
    }
    "#.to_string()).expect("Unsuccessful Api call");
    println!("{}", post_response.text().unwrap());
}
