/// Handle case matching. Specialized handlers such as pixiv and Newgrounds get ferried off.

pub(crate)

mod pixiv;
use warp;
pub const USER_AGENT: &str = "ReEmbeddy/0.0.1 (git repo here)";

#[derive(Default)]
pub struct SiteMetadata {
    width: u32,
    height: u32,

    url: String,
    title: String,
	description: String,
	image_url: String,
	video_url: String,
	oembed_url: String,
    poster: String,
    poster_img: String,

    tags: Vec<String>,
}

pub fn routeDiscordReq(agent: &String, api: &String) ->  warp::reply::Html<String>{
    warp::reply::html(format!("<html><head></head><body>Discord: {agent}</body></html>"))
}

pub fn routeTGReq(agent: &String, api: &String) ->  warp::reply::Html<String>{
    warp::reply::html(format!("<html><head></head><body>TG: {agent}</body></html>"))
}

pub fn routeElse(agent: &String, api: &String) -> warp::reply::Html<String>{
    let r = api.as_str();

    warp::reply::html(format!("<html><head></head><body><p>Normal: {agent}</p><br/><p>{r}</p></body></html>"))
}