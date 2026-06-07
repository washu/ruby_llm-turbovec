#!/bin/bash
set -e

# Extract version from version.rb
VERSION=$(ruby -r ./lib/rspec/llm/version.rb -e "puts RSpec::LLM::VERSION")

echo "Building gem version ${VERSION}..."
gem build rspec-llm.gemspec

echo "Pushing to RubyGems..."
gem push rspec-llm-${VERSION}.gem

echo "Done!"
