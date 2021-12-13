# EVET

A CLI app for generating event announcement message including multiple
timezones.

    evet --message="Rust Berlin Talks" \
         --date="2021-11-23 23:00" \
         --timezone="Europe/Istanbul" \
         --timezone="Europe/Berlin" \
         --timezone="Japan"

    ---
    Rust Berlin Talks

    Istanbul: 2021-11-24 01:00
    Berlin: 2021-11-23 23:00
    Japan: 2021-11-24 07:00
    ---

Also it is possible to specify the local timezone for the given date.

    evet --message="Rust Berlin Talks" \
         --date="2021-11-23 23:00" \
         --timezone="Europe/Istanbul" \
         --timezone="Europe/Berlin" \
         --timezone="Japan" \
         --local="Japan"

    ---
    Rust Berlin Talks

    Istanbul: 2021-11-23 17:00
    Berlin: 2021-11-23 15:00
    Japan: 2021-11-23 23:00
    ---
