#!/bin/bash
set -e

# Extract version from version.rb
VERSION=$(ruby -r ./lib/ruby_llm/turbovec/version.rb -e "puts RubyLLM::Turbovec::VERSION")

echo "Building gem version ${VERSION}..."
gem build ruby_llm-turbovec.gemspec

echo "Pushing to RubyGems..."
gem push ruby_llm-turbovec-${VERSION}.gem

echo "Done!"
