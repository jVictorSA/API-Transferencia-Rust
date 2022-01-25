pub async fn hello(_req: Request<State>) -> tide::Result {
    return Ok("Hello, world!".into()) //into() transforma a str em tide::Result
}