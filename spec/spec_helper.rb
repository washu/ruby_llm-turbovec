# frozen_string_literal: true

require "simplecov"

SimpleCov.start do
  add_filter do |source_file|
    source_file.filename.to_s.match?(%r{/spec/})
  end
  minimum_coverage 90
end

require_relative "../lib/ruby_llm/turbovec"

RSpec.configure do |config|
  # Enable flags like --only-failures and --next-failure
  config.example_status_persistence_file_path = ".rspec_status"

  # Disable RSpec exposing methods globally on `Module` and `main`
  config.disable_monkey_patching!

  config.expect_with :rspec do |c|
    c.syntax = :expect
  end
end
