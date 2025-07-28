use actix_web::{
    Scope, get,
    web::{self, Redirect},
};

#[get("/")]
async fn index() -> Redirect {
    Redirect::to("https://github.com/cubewhy/fxbilibili").permanent()
}

pub fn scope() -> Scope {
    web::scope("").service(index)
}
