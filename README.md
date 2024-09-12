# Ref.Finance Stableswap helper

## Test
```bash
# rust tests
wasm-pack test --node

# js tests
bun install
bun test
```

## Release

```bash
make
# Ensure correctness of `pkg/package.json` and then publish:
yarn publish
```
