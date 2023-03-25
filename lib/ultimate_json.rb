# frozen_string_literal: true

require_relative "ultimate_json/version"
require_relative "ultimate_json/ultimate_json"

module UltimateJson
  class Error < StandardError; end

  class << self
    def parse_json_with_rust(filename)
      UltimateJSON.parse_json(filename)
    end
  end
end
