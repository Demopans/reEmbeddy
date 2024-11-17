/// Handle case matching. Specialized handlers such as pixiv and Newgrounds get ferried off.

pub(crate)

mod pixiv;
use warp;
pub const USER_AGENT: &str = "ReEmbeddy/0.0.1 (git repo here)";
struct SiteField {
    api: &'static str
}
impl SiteField {
    fn api(&self) -> &str {
        self.api
    }
}

#[derive(Default)]
struct SiteMetadata {
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

static SITES: phf::Map<&'static str, SiteField> = phf::phf_map!{
    "deviantart"     => SiteField {api: "https://backend.deviantart.com/oembed?url={id}"},
    "pixiv"          => SiteField {api: "https://www.pixiv.net/ajax/illust/{id}?lang={lang}"},                                 // requires auth. Handle as a special case
    "furaffinity"    => SiteField {api: "https://faexport.spangle.org.uk/submission/{id}.json"},                               // uses 3rd party bindings. Mark for future replacement
    "x"              => SiteField {api: "https://api.x.com/2/tweets/{id}"},                                                    // requires oauth
    "twitter"        => SiteField {api: "https://api.twitter.com/2/tweets/{id}"},                                              // requires oauth
    "instagram"      => SiteField {api: "https://graph.facebook.com/v20.0/instagram_oembed?url={url}&access_token={key}"},     // requires auth. Handle as a special case
    "weasyl"         => SiteField {api: "https://www.weasyl.com/api/submissions/{id}/view"},
    "hentai-foundry" => SiteField {api: "https://thumbs.hentai-foundry.com/thumb.php?pid={id}"},
    "e621"           => SiteField {api: "https://e621.net/posts/{id}.json"},
    "newgrounds"     => SiteField {api: "https://art.ngfiles.com/images/"},                                                    // reverse engineer url to request body
    "bsky"           => SiteField {api: "https://api.bsky.app/xrpc/app.bsky.feed.getPostThread?uri=at://{author}/app.bsky.feed.post/{id}&depth=0"} // this is awful. Also best use oauth
};


pub fn routeDiscordReq(agent: &String, sub: &String, path: &warp::path::FullPath) ->  warp::reply::Html<String>{
    warp::reply::html(format!("<html><head></head><body>Discord: {agent}</body></html>"))
}

pub fn routeTGReq(agent: &String, sub: &String, path: &warp::path::FullPath) ->  warp::reply::Html<String>{
    warp::reply::html(format!("<html><head></head><body>TG: {agent}</body></html>"))
}

pub fn routeElse(agent: &String, sub: &String, path: &warp::path::FullPath) -> warp::reply::Html<String>{
    // route known sites




    let r = path.as_str();

    warp::reply::html(format!("<html><head></head><body><p>Normal: {agent}</p><br/><p>{r}</p></body></html>"))
}