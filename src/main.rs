use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // register the table
  let ctx = SessionContext::new();
  ctx.register_csv("example", "tests/example.csv", CsvReadOptions::new()).await?;

  // create a plan to run a SQL q&uery
  let query = String::from("SELECT \"Column A\", MIN(\"Column B\") FROM example GROUP BY \"Column A\" LIMIT 100");
  let df = ctx.sql(&query).await?;

  // execute and print results
  df.show().await?;
  Ok(())
}
