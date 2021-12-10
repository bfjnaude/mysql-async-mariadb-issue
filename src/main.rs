use mysql_async::prelude::Queryable;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args()
        .nth(1)
        .expect("no database url given as first argument");

    // let opts = mysql_async::Opts::from_url(&url).unwrap();
    // let conn_pool = mysql_async::Pool::new(opts);

    let conn_pool = mysql_async::Pool::from_url(url).unwrap();

    // let conn_pool = mysql_async::Pool::new(url);

    let mut conn = conn_pool.get_conn().await.expect("Could not connect to db");

    let version: Vec<String> = conn
        .exec("SELECT version()", ())
        .await
        .expect("Could not get version");

    for v in version.into_iter() {
        println!("Connected to database with version {}", v);
    }
    Ok(())
}
