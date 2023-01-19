# rust-graphql

GraphQLサーバーのベース

## Crate

- Webフレームワーク: [salvo](https://salvo.rs/)
- GraphQL: [Async-graphql](https://github.com/async-graphql/async-graphql)
- ORM: [SeaORM](https://www.sea-ql.org/SeaORM/)
- 非同期ランタイム: [tokio](https://tokio.rs/)
- ログ構造化フレームワーク: [tracing](https://github.com/tokio-rs/tracing)
- ENVファイルローダー: [dotenvy](https://github.com/allan2/dotenvy)

## 実行

```bash
docker compose up -d
cargo run
```
