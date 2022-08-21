use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn construct_headers_experiments() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
                HeaderName::from_static("accept"),
                HeaderValue::from_static("*/*"),
        );
        headers.insert(
                HeaderName::from_static("accept-language"),
                HeaderValue::from_static("en-US,en;q=0.5"),
        );
        headers.insert(HeaderName::from_static("x-super-properties"), HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiRmlyZWZveCIsImRldmljZSI6IiIsInN5c3RlbV9sb2NhbGUiOiJlbi1VUyIsImJyb3dzZXJfdXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEwMy4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEwMy4wIiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTAzLjAiLCJvc192ZXJzaW9uIjoiMTAiLCJyZWZlcnJlciI6IiIsInJlZmVycmluZ19kb21haW4iOiIiLCJyZWZlcnJlcl9jdXJyZW50IjoiIiwicmVmZXJyaW5nX2RvbWFpbl9jdXJyZW50IjoiIiwicmVsZWFzZV9jaGFubmVsIjoic3RhYmxlIiwiY2xpZW50X2J1aWxkX251bWJlciI6MTMzODUyLCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsfQ=="));
        headers.insert(
                HeaderName::from_static("x-contect-properties"),
                HeaderValue::from_static("eyJsb2NhdGlvbiI6IkFjY2VwdCBJbnZpdGUgUGFnZSJ9"),
        );
        headers.insert(
                HeaderName::from_static("x-discord-locale"),
                HeaderValue::from_static("en-US"),
        );
        headers.insert(
                HeaderName::from_static("referer"),
                HeaderValue::from_static("https://discord.com"),
        );
        headers.insert(
                HeaderName::from_static("x-debug-options"),
                HeaderValue::from_static("bugReporterEnabled"),
        );
        headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:103.0) Gecko/20100101 Firefox/103.0"));
        headers.insert(
                HeaderName::from_static("connection"),
                HeaderValue::from_static("keep-alive"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-dest"),
                HeaderValue::from_static("empty"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-mode"),
                HeaderValue::from_static("cors"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-site"),
                HeaderValue::from_static("same-origin"),
        );
        headers.insert(
                HeaderName::from_static("te"),
                HeaderValue::from_static("trailers"),
        );
        headers
}
pub fn construct_headers_register(fingerprint: &'static str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
                HeaderName::from_static("accept"),
                HeaderValue::from_static("*/*"),
        );
        headers.insert(
                HeaderName::from_static("accept-language"),
                HeaderValue::from_static("en-US,en;q=0.5"),
        );
        headers.insert(HeaderName::from_static("x-super-properties"), HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiRmlyZWZveCIsImRldmljZSI6IiIsInN5c3RlbV9sb2NhbGUiOiJlbi1VUyIsImJyb3dzZXJfdXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEwMy4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEwMy4wIiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTAzLjAiLCJvc192ZXJzaW9uIjoiMTAiLCJyZWZlcnJlciI6IiIsInJlZmVycmluZ19kb21haW4iOiIiLCJyZWZlcnJlcl9jdXJyZW50IjoiIiwicmVmZXJyaW5nX2RvbWFpbl9jdXJyZW50IjoiIiwicmVsZWFzZV9jaGFubmVsIjoic3RhYmxlIiwiY2xpZW50X2J1aWxkX251bWJlciI6MTMzODUyLCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsfQ=="));
        headers.insert(
                HeaderName::from_static("x-discord-locale"),
                HeaderValue::from_static("en-US"),
        );
        headers.insert(
                HeaderName::from_static("origin"),
                HeaderValue::from_static("https://discord.com"),
        );
        headers.insert(
                HeaderName::from_static("referer"),
                HeaderValue::from_static("https://discord.com/channels/@me"),
        );
        headers.insert(
                HeaderName::from_static("x-fingerprint"),
                HeaderValue::from_static(fingerprint),
        );
        headers.insert(
                HeaderName::from_static("x-debug-options"),
                HeaderValue::from_static("bugReporterEnabled"),
        );
        headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:103.0) Gecko/20100101 Firefox/103.0"));
        headers.insert(
                HeaderName::from_static("connection"),
                HeaderValue::from_static("keep-alive"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-dest"),
                HeaderValue::from_static("empty"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-mode"),
                HeaderValue::from_static("cors"),
        );
        headers.insert(
                HeaderName::from_static("sec-fetch-site"),
                HeaderValue::from_static("same-origin"),
        );
        headers.insert(
                HeaderName::from_static("te"),
                HeaderValue::from_static("trailers"),
        );
        headers
}
