# RubyLLM::Turbovec

[![CI](https://github.com/washu/ruby_llm-turbovec/actions/workflows/ci.yml/badge.svg)](https://github.com/washu/ruby_llm-turbovec/actions/workflows/ci.yml)
[![Release](https://github.com/washu/ruby_llm-turbovec/actions/workflows/release.yml/badge.svg)](https://github.com/washu/ruby_llm-turbovec/actions/workflows/release.yml)

`RubyLLM::Turbovec` is a Ruby gem that ships a native Rust extension built with `magnus` and `rb-sys`.

The extension now wraps the real [`turbovec`](https://crates.io/crates/turbovec) Rust crate and exposes Ruby-friendly APIs for both positional vector search and stable ID-based search.

## Installation

This gem requires Ruby and Rust so the native extension can compile during installation.

Install it from a local checkout with:

```bash
bundle install
```

## Usage

### Positional index

```ruby
require "ruby_llm/turbovec"

index = RubyLLM::Turbovec::TurboQuantIndex.new(8, 4)
index.add([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])

results = index.search([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], 1)
results.indices_for_query(0)
# => [0]

index.write("index.tv")
loaded = RubyLLM::Turbovec::TurboQuantIndex.load("index.tv")
```

### Stable ID index

```ruby
id_index = RubyLLM::Turbovec::IdMapIndex.new(8, 4)
id_index.add_with_ids(
  [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
  [42]
)

scores, ids = id_index.search([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], 1)
ids
# => [42]

id_index.write("index.tvim")
reloaded = RubyLLM::Turbovec::IdMapIndex.load("index.tvim")
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `bundle exec rake spec` to build the native extension and run the tests.

The CI workflow also runs `cargo test --locked` inside `ext/ruby_llm/turbovec` so the native Turbovec crate is tested directly, not just through the Ruby wrapper.

The native binding uses a read/write lock around the underlying Rust indexes so read-heavy search workloads can proceed without the single global mutex bottleneck.

For a coverage matrix of what the Ruby gem wraps from the Rust crate, see `docs/api-coverage-matrix.md`.

For release steps, including the manual RubyGems MFA publish flow, see `docs/release-process.md`.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/washu/ruby_llm-turbovec. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/washu/ruby_llm-turbovec/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the RubyLLM::Turbovec project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/washu/ruby_llm-turbovec/blob/main/CODE_OF_CONDUCT.md).
