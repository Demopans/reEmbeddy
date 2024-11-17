/// Contains main service loop, queueing up requests

use serde::Deserialize;

mod handlers;

const DISCORD: &str = "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)";
const CONFIG: &str = "config.json";
type Table = std::collections::HashMap<String, String>; // encodes url params correctly

#[derive(Deserialize)]
struct Config{
    host: [u8;4],
    port: u16
}

fn readConfig() ->Config{
    let file = std::fs::File::open(CONFIG).expect("config.json does not exist");
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
            let subdom: String = pat.as_str().split('.').next().unwrap_or("").to_string();
            match agent.as_str() {
                DISCORD     => handlers::routeDiscordReq(&agent, &subdom, &pat),
                _           => handlers::routeElse(&agent, &subdom, &pat),
            }

    });
    //unroll
    let hostip: [u8;4] = [config.host[0], config.host[1], config.host[2], config.host[3]];
    // run server
    warp::serve(decode).run((hostip, config.port)).await;
}