/// Contains main service loop, queueing up requests
mod handlers; mod model;

use {crate::model::Config, crate::model::SiteField};

fn readConfig() -> Config{
    let file = std::fs::File::open(crate::model::CONFIG).expect("config.json does not exist");
    let buff = std::io::BufReader::new(file);
    let json: Config = serde_json::from_reader(buff).expect("JSON was not well-formatted");
    json
}

fn main() {
    let cfg: Config = readConfig();

    run(&cfg); // reference only. Also looks like I can just do that
}

/// Routing happens here I guess
#[tokio::main]
async fn run(config: &Config){
    use warp::Filter;

    // HOST/?u={link}
    let decode =
        warp::header("user-agent")
        .and(warp::header("host")) // be sure to configure nginx with a *.example.com rerouter!
        .and(warp::path::full())
        .map(|agent: String,h: String, pat: warp::path::FullPath|{
            println!("{h} with ");

            // split for subdomain
            let subdom: &str = pat.as_str().split('.').next().unwrap_or("");

            let mut matchSubdomain = |sub| -> &str {
                match sub {
                    "deviantart"     => "https://backend.deviantart.com/oembed?url={id}",
                    "pixiv"          => "https://www.pixiv.net/ajax/illust/{id}?lang={lang}",                                 // requires auth. Handle as a special case
                    "furaffinity"    => "https://faexport.spangle.org.uk/submission/{id}.json",                               // uses 3rd party bindings. Mark for future replacement
                    "x"              => "https://api.x.com/2/tweets/{id}",                                                    // requires oauth
                    "twitter"        => "https://api.twitter.com/2/tweets/{id}",                                              // requires oauth
                    "instagram"      => "https://graph.facebook.com/v20.0/instagram_oembed?url={url}&access_token={key}",     // requires auth. Handle as a special case
                    "weasyl"         => "https://www.weasyl.com/api/submissions/{id}/view",
                    "hentai-foundry" => "https://thumbs.hentai-foundry.com/thumb.php?pid={id}",
                    "e621"           => "https://e621.net/posts/{id}.json",
                    "newgrounds"     => "https://art.ngfiles.com/images/",                                                    // reverse engineer url to request body
                    "bsky"           => "https://api.bsky.app/xrpc/app.bsky.feed.getPostThread?uri=at://{author}/app.bsky.feed.post/{id}&depth=0", // this is awful. Also best use oauth
                    _                => "https://example.com"
                }
            };

            // path to api params
            let apiPayload = matchSubdomain(subdom).to_string();

            match agent.as_str() {
                DISCORD     => handlers::routeDiscordReq(&agent, &apiPayload),
                _           => {

                    handlers::routeElse(&agent, &apiPayload)
                },
            }

    });
    //unroll
    let hostip: [u8;4] = [config.host[0], config.host[1], config.host[2], config.host[3]];
    // run server
    warp::serve(decode).run((hostip, config.port)).await;
}