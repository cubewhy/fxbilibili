use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, Scope, get, http::header::HeaderValue, web};

use crate::{models::video::VideoId, services::bilibili::BilibiliServiceTrait};

#[get("/BV{bv}")]
async fn bv_handler(
    request: HttpRequest,
    path: web::Path<(String,)>,
    bili_service: web::Data<Arc<dyn BilibiliServiceTrait>>,
) -> impl Responder {
    let (bv_id,) = path.into_inner();
    let bili_service = bili_service.into_inner();

    let video_id = VideoId::Bv(bv_id);

    if is_browser(request.headers().get("User-Agent")) {
        // do redirect directly
        return HttpResponse::PermanentRedirect()
            .insert_header(("Location", video_id.url().as_str()))
            .body(format!("Redirect to {}", video_id.url()));
    }

    let og = match bili_service.video_info_og(&video_id).await {
        Ok(v) => v,
        Err(err) => return HttpResponse::Ok().body(err.to_string()), // TODO: use json
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(og.into_html())
}

#[get("/av{aid}")]
async fn av_handler(
    request: HttpRequest,
    path: web::Path<(String,)>,
    bili_service: web::Data<Arc<dyn BilibiliServiceTrait>>,
) -> impl Responder {
    let (av_id,) = path.into_inner();
    let bili_service = bili_service.into_inner();

    let Ok(av_id) = av_id.parse() else {
        return HttpResponse::BadRequest().body("Bad avid"); // TODO: use json
    };

    let video_id = VideoId::Av(av_id);

    if is_browser(request.headers().get("User-Agent")) {
        // do redirect directly
        return HttpResponse::PermanentRedirect()
            .insert_header(("Location", video_id.url().as_str()))
            .body(format!("Redirect to {}", video_id.url()));
    }

    let og = match bili_service.video_info_og(&video_id).await {
        Ok(v) => v,
        Err(err) => return HttpResponse::Ok().body(err.to_string()), // TODO: use json
    };

    if is_browser(request.headers().get("User-Agent")) {
        // do redirect directly
        return HttpResponse::PermanentRedirect()
            .insert_header(("Location", og.url.as_str()))
            .body(og.url);
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(og.into_html())
}

fn is_browser(ua: Option<&HeaderValue>) -> bool {
    let Some(ua) = ua else {
        return false;
    };

    let Ok(ua) = ua.to_str() else {
        return false;
    };
    ua.contains("Mozilla")
}

pub fn scope() -> Scope {
    web::scope("/video").service(bv_handler).service(av_handler)
}
