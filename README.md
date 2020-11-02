# RustPhxSample
Rustler/Elxiir/Phoenixを使用してRustでWebアプリケーションを開発するサンプルリポジトリ

ErlangVMの[NIFs](http://erlang.org/doc/tutorial/nif.html)を利用して、Elixir/PhoenixからRustの関数を呼び出している.

Phoenixではルーティング、レスポンスの構築のみを担当し、実際のアプリケーションの動作の本質であるDBへのWriteをRustが行う。ということをしている

詳しい解説は以下のブログ記事を参照

[RustでErlangVM上で動作するWebアプリケーションを開発する](https://sanposhiho.com/posts/rust-to-elixir-phx/)

---

To start your Phoenix server:

  * Install dependencies with `mix deps.get`
  * Install Node.js dependencies with `npm install` inside the `assets` directory
  * Start Phoenix endpoint with `mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://hexdocs.pm/phoenix/deployment.html).

## Learn more

  * Official website: https://www.phoenixframework.org/
  * Guides: https://hexdocs.pm/phoenix/overview.html
  * Docs: https://hexdocs.pm/phoenix
  * Forum: https://elixirforum.com/c/phoenix-forum
  * Source: https://github.com/phoenixframework/phoenix
