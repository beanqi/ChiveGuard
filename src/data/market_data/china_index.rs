

// TODO: Provide methods to fetch China market indices

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, BufRead};
    use crate::data::MarketData;

    #[test]
    fn test_fetch_china_index() {
        fn read_china_index_from_csv(file_path: &str) -> io::Result<Vec<MarketData>> {
            let mut market_data = Vec::new();
            let file = File::open(file_path)?;
            let reader = io::BufReader::new(file);
            let mut lines = reader.lines();
            lines.next(); // Skip the first line
            for line in lines {
                let line = line?;
                let fields: Vec<&str> = line.split(',').collect();
                let data = MarketData {
                    symbol: String::from("HS300"),
                    timestamp: fields[0].parse().unwrap_or(0),
                    open: fields[1].parse().unwrap_or(0.0),
                    high: fields[2].parse().unwrap_or(0.0),
                    low: fields[3].parse().unwrap_or(0.0),
                    close: fields[4].parse().unwrap_or(0.0),
                    volume: fields[5].parse().unwrap_or(0.0),
                 };
                    market_data.push(data);
            }

            Ok(market_data)
        }
            let data = read_china_index_from_csv(r"C:\Users\ynlbq\code\rust\ChiveGuard\src\data\market_data\hs300.csv").expect("Failed to read CSV file");
            assert!(!data.is_empty());
    }
}