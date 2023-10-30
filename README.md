# 🔎economic indicator finder

A finder for extracting economic indicators from paragraphs

## Usage

```rust
use economic_indicator_finder::{Finder, Indicator, Speech};
use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let speechs = Speech::from_json_file("speech/speech.json")?;
    let finder = Finder::new(Indicator::from_json_file("asset/indicators.json")?)?;

    for speech in speechs {
        let speaker = &speech.speaker;
        let speech = read_to_string(speech.file)?;
        let matches: Vec<_> = finder.matches(speech);
        println!("From the speech of {}: {:#?}", speaker, matches);
    }
    Ok(())
}
```

## Indicators

## References

- [Economic Indicators @GovInfo](https://www.govinfo.gov/app/collection/econi/2023/01/7)
- [Main Economic Indicators, Volume 2023 Issue 10 @OECD-ilibrary](https://www.oecd-ilibrary.org/economics/main-economic-indicators/volume-2023/issue-10_bbbe4c08-en)
- [Economic Indicators @TradingEconomics](https://tradingeconomics.com/indicators)
- [매크로(거시경제) 용어정리 @개발자윌리](https://blog.naver.com/PostView.naver?blogId=techref&logNo=222500924666&from=search&redirect=Log&widgetTypeCall=true&directAccess=false)
- [경제 지표 @Wikipedia](https://ko.wikipedia.org/wiki/%EA%B2%BD%EC%A0%9C_%EC%A7%80%ED%91%9C)