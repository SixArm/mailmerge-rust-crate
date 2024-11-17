# MailMerge Rust crate

WORK IN PROGRESS. NOT USABLE YET.

Syntax:

```sh
mailmerge [options]
```

Options:

* -u, --username 

* -p, --password

* --host

* --port

* --security

* --ratelimit

* --test - test mode, which prints message to console

* -v, --verbose - verbose mode error, warn, info, debug, trace

Config file example using TOML format:

```toml
username = "alice@example.com"
password = "secret"
host = "smtp.example.com"
port = 587
security = "tls"
ratelimit = 0
```
