# frozen_string_literal: true

require_relative "lib/ruby_llm/turbovec/version"

Gem::Specification.new do |spec|
  # rubocop:disable Style/Dir
  root = File.expand_path(File.dirname(__FILE__))
  # rubocop:enable Style/Dir

  spec.name = "ruby_llm-turbovec"
  spec.version = RubyLLM::Turbovec::VERSION
  spec.authors = ["Sal Scotto"]
  spec.email = ["sal.scotto@gmail.com"]

  spec.summary = "Native Ruby bindings for the Turbovec Rust library"
  spec.description = "A Ruby gem that compiles a native Rust extension with magnus and rb-sys to expose Turbovec APIs."
  spec.homepage = "https://github.com/washu/ruby_llm-turbovec"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = "https://github.com/washu/ruby_llm-turbovec/blob/main/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: root, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git appveyor Gemfile])
    end
  end
  spec.files += Dir.glob("ext/ruby_llm/turbovec/**/*", base: root).select do |f|
    File.file?(File.join(root, f)) &&
      !f.start_with?("ext/ruby_llm/turbovec/target/") &&
      File.extname(f) != ".bundle" &&
      !f.include?(".dSYM/")
  end
  spec.extensions = ["ext/ruby_llm/turbovec/extconf.rb"]
  spec.bindir = "exe"
  spec.executables = spec.files.select { |f| f.start_with?("exe/") }.map { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_dependency "rb_sys", "~> 0.9"

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
