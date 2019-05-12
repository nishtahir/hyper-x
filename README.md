# Hyper-X

A proof of concept `proc_macro` based HTTP client.

Hyper-X allows you to define your HTTP client as a `trait`

``` rust
#[http_client(JsonPlaceholderApiClient)]
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
struct JsonPlaceholderApiClient {
    root: String,
    client: reqwest::Client,
}
impl JsonPlaceholderApiClient {
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

Your trait must be declared with a `http_client(...)` attribute

```
#[http_client(JsonPlaceholderApiClient)]
trait JsonPlaceholderApi
```

Every request method must have an http attribute that contains the method
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

# License

```
   Copyright 2019 Nish Tahir

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```