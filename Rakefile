# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

EXT_DIR = File.expand_path("ext/ruby_llm/turbovec", __dir__)

desc "Compile the native Rust extension"
task :compile do
  Dir.chdir(EXT_DIR) do
    sh RbConfig.ruby, "extconf.rb"
    sh "make"
  end
end

RSpec::Core::RakeTask.new(:spec)
task spec: :compile

require "rubocop/rake_task"

RuboCop::RakeTask.new

task default: %i[spec rubocop]
