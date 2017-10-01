# owo.rs tests

To run tests, you need to set an environment variable named `"OWO_TOKEN"` with
your token.

These tests are ignored by default, so you need to enable them with the
`--enabled` flag when testing.

### Sample

```sh
$ OWO_TOKEN=mytokenhere cargo test -- --ignored
```
