use dotenv::dotenv;

pub fn fetch_input(year: u16, day: u8) -> anyhow::Result<String> {
    dotenv().ok();
    let session = std::env::var("SESSION")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = format!("session={}", session);
    let res = minreq::get(url).with_header("Cookie", cookie).send()?;
    let str = res.as_str()?;

    Ok(str.to_string())
}
