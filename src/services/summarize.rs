use crate::{models::models, services};
use chrono::{Datelike, NaiveDate};
use std::collections::{BTreeMap, BTreeSet};

pub fn run(file_path: &str) {
    println!("家計簿の集計を行います");
    let data = services::io::read_data_or_panic(file_path);

    // 全て集計対象月の1日に変換して合計する
    let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

    for date in target_dates {
        let filterd_data = get_filterd_date(&data, date);
        let sum = summarize_data(&filterd_data);
        result_table.insert(date, sum);
    }
    print_table(result_table);
}

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| item.get_first_day()).collect();
    target_dates
}

fn get_filterd_date(data: &Vec<models::Item>, filter_date: NaiveDate) -> Vec<models::Item> {
    let filter_data: Vec<models::Item> =
        data.iter().filter(|item| (item.get_year() == filter_date.year()) && (item.get_month() == filter_date.month())).cloned().collect();
    filter_data
}

fn summarize_data(data: &Vec<models::Item>) -> i32 {
    let mut sum: i32 = 0;

    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date: String = format_date(result.0);
        let price: String = format_price(result.1);
        println!("{}の収支は{}円でした", date, price)
    }
}
