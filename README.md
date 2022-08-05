# subrip

Library for manipulating .srt subtitle files.

## Get Started
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
