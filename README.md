# proxycurl-linkedin-rs

rust crate for interacting with the proxycurl LinkedIn API, not feature-complete

<https://nubela.co/proxycurl/docs?shell#overview>

currently only supports the endpoints we actually use at Jara, sorry for the inconvenience.

all endpoints will be supported by semvar 1.0.0, the last value will count endpoints in the API being added, the middle value will count the API itself.

## set-up

store your proxycurl API key in the `PROXYCURL_API_KEY` environment variable.

## usage

add as a dependency

```Cargo.toml
proxycurl-linkedin-rs = "0"
```

initialize the client with your api key

```rs

```

construct a request

send it
