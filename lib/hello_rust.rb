# frozen_string_literal: true

require_relative "hello_rust/version"
require_relative "hello_rust/hello_rust"

module HelloRust
  class Error < StandardError; end

  class << self
    def parse_json_with_rust(filename)
      ParseJson.parse_json(filename)
    end
  end
end
