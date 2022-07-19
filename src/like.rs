pub type Likes = Response<Like>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String, 
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now()
        }
    }
}

// list last 50 like from a tweet given its id
#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    // TODO: find a list of likes by tweet id and return them
    let likes = Likes { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

// add one like to a tweet
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>) -> HttpResponse { 
    // TODO: add a like to a tweet
    let like = Like::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>) -> HttpResponse {
    // remove one like from a tweet
    HttpResponse::NoContext()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}