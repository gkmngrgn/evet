# EVET

A CLI app for generating event announcement message including multiple
timezones.

```shell
evet --message="Rust Berlin Talks" \
     --date="2021-11-23 23:00"     \
     --timezone="Europe/Istanbul"  \
     --timezone="Europe/Berlin"    \
     --timezone="Japan"            \
     --local="Japan"

---
Rust Berlin Talks

Istanbul: 2021-11-23 17:00
Berlin: 2021-11-23 15:00
Japan: 2021-11-23 23:00
---
```
