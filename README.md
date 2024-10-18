# Reveal Leptos

## Developing your Reveal Leptos project

To develop your Reveal Leptos project, running

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.

### Adding Wasm
You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```


## Deploying your Reveal Leptos project

To build a Reveal Leptos app for release, use the command

```sh
trunk build --release --dist release
```

This will output the files necessary to run your app into the `release` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].


[Leptos]: https://github.com/leptos-rs/leptos

[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/

[deploy-csr]: https://book.leptos.dev/deployment/csr.html