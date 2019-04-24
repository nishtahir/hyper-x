# Hyper-X

A proof of concept `proc_macro` based HTTP client.

Hyper-X allows you to define your HTTP client as a `trait`

``` rust
#[http_client]
trait JsonPlaceholderApi {
    #[get(path = "/comments?postId={post_id}")]
    fn get_comments(&self, post_id: i32) -> Result<Response>;

    #[post(path = "/posts", data = "{data}")]
    fn create_post(&self, data: String) -> Result<Response>;
}
```

Hyper-X generates a struct and implementation with common boiler plate generated
for you behind the scenes.

``` rust
struct JsonPlaceholderApi {
    root: String,
    client: reqwest::Client,
}
impl JsonPlaceholderApi {
    fn new<S: Into<String>>(root: S, client: reqwest::Client) -> Self {
        JsonPlaceholderApi {
            root: root.into(),
            client: client,
        }
    }
    fn get_comments(&self, post_id: i32) -> Result<Response> {
        let path_segments = format!("/comments?postId={post_id}", post_id = post_id);
        let mut url = String::new();
        url.push_str(&self.root);
        url.push_str(&path_segments);
        self.client.get(&url).send().into()
    }
    fn create_post(&self, data: String) -> Result<Response> {
        let path_segments = format!("/posts",);
        let mut url = String::new();
        url.push_str(&self.root);
        url.push_str(&path_segments);
        self.client.post(&url).body(data).send().into()
    }
}
```

## Api declaration

Every request method must have an http annotation that contains the method
and path.

``` rust
#[get(path = "/comments/1")]
```

Parameters can be interpolated inline using the familiar string interpolation  
syntax `{param}`. Parameters declared map to parameters in the anotated functions

``` rust
#[get(path = "/comments?postId={post_id}")]
fn get_comments(&self, post_id: i32) -> Result<Response>;
```

Request body is declared using the `data` attribute

``` rust
#[post(path = "/posts", data = "{data}")]
fn create_post(&self, data: String) -> Result<Response>;
```

