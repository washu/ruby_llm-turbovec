# Release process

This gem uses a native Rust extension, so releases should always be verified before publishing.

## What the automation does

- `CI` runs the Ruby wrapper tests, RuboCop, and the Rust crate tests.
- The release workflow builds the `.gem` file and attaches it to a GitHub release when a tag like `v0.1.1` is pushed.
- The crate version watcher opens an issue when crates.io publishes a newer `turbovec` release.

## Publish steps

1. Update the Ruby version in `lib/ruby_llm/turbovec/version.rb`.
2. Update any native dependency changes under `ext/ruby_llm/turbovec/`.
3. Run:

   ```bash
   bundle exec rake spec
   bundle exec rubocop
   bundle exec rake build
   ```

4. Create and push a tag, for example:

   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```

5. The release workflow will build the gem and attach it to the GitHub release.
6. Download the built `.gem` artifact and publish it to RubyGems manually.

## RubyGems MFA

Publishing to RubyGems requires MFA, so the final `gem push` step is intentionally manual.

Use the built artifact from `pkg/` and publish it from a trusted environment where you can complete the MFA/OTP challenge.

