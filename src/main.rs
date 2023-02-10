use actix_web::{middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


//create a function that accepts a string as input and returns a sha256 hash of the string
pub fn sha(text: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    format!("{:x}", result)
}

//create a function that accepts text as input and returns a hash of the text
//use the DefaultHasher to create a hash of the text
//use the hasher.finish() method to get the hash value
pub fn hash(text: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

//create a struct that will be used to deserialize the JSON payload
#[derive(Deserialize)]
struct Text {
    text: String,
}


#[post("/token")]
async fn tokenize(text: web::Json<Text>) -> impl Responder {
    //use the hash function to create a hash of the text and then return the hash
    //as JSON
    let hash = sha(&text.text);
    //use HttpResponse to return the hash as JSON
    HttpResponse::Ok().body(hash.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // run the server on port 8001 and use the default logger as middleware
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(tokenize)
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}