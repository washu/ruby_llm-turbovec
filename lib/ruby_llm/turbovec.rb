# frozen_string_literal: true

require_relative "turbovec/version"

begin
  require "ruby_llm/turbovec/ruby_llm_turbovec"
rescue LoadError
  require_relative "../../ext/ruby_llm/turbovec/ruby_llm_turbovec"
end

module RubyLLM
  module Turbovec
    class Error < StandardError; end
  end
end
