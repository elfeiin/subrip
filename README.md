# subrip

Library for manipulating .srt subtitle files.

## Get Started

Add `subrip = { git = "https://github.com/elein727/subrip.git" }` to your dependencies in `Cargo.toml`.

```rs
use subrip::Subtitles;

let input = "

1
01:23:45,678 --> 01:23:45,678
Oh.

2
01:23:45,678 --> 01:23:45,678
Is that how it is?

3
01:23:45,678 --> 01:23:45,678
I see now.
";

let mut subs = Subtitles::from_str(input).unwrap();

assert_eq![subs.entries.len(), 3];
```
