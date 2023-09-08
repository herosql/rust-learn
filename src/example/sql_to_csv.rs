/*
   通过sql查询csv文件
*/
use anyhow::{Ok, Result};
use polars::prelude::*;
use sqlparser::dialect::Dialect;
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::io::Cursor;

#[derive(Debug, Default)]
pub struct TyrDialect;

// 创建自己的 sql 方言.TyrDialect 支持 identifier 可以是简单的 url
impl Dialect for TyrDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        // todo!()
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    // identifier 可以有 ':', '/', '?', '&', '='
    fn is_identifier_part(&self, ch: char) -> bool {
        // todo!()
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
     FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );
    sql
}

fn parse_sql() {
    // tracing_subscriber::fmt::init();

    let sql = "SELECT a a1, b, 123, myfunc(b), * \
     FROM data_source \
      WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 \
       ORDER BY a DESC, b \
        LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}

fn show_table() -> Result<()> {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::blocking::get(url)?.text()?;

    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;

    let filtered = df.filter(&df["new_deaths"].gt(10))?;
    println!(
        "{:?}",
        filtered.select((
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_sql() {
        parse_sql()
    }

    #[test]
    fn test_example_sql() {
        let example = "SELECT location name, total_cases, new_cases, total_deaths, new_deaths 
     FROM https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5";
        let sql = example_sql();
        assert_eq!(example, sql);

        assert!(Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok());
    }

    #[test]
    fn test_show_table() {
        show_table();
    }
}
