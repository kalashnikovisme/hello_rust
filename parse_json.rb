require 'json'
require 'bundler/inline'
require 'benchmark'

gemfile(true, quiet: true) do
  source "https://rubygems.org"

  gem 'rapidjson'
  gem 'oj'
end


file_path = ARGV[0]

time = Benchmark.measure do
  data = File.readlines(file_path).join
  JSON.parse(data)["key1"]
end

puts "Ruby parse #{file_path} JSON.parse operation: #{time.real} sec"


time = Benchmark.measure do
  require 'rapidjson'

  data = File.readlines(ARGV[0]).join
  RapidJSON.parse(data)["key1"]
end

puts "Ruby parse #{file_path} RapidJSON.parse operation: #{time.real} sec"

time = Benchmark.measure do
  require 'oj'

  data = File.readlines(ARGV[0]).join
  Oj.load(data)["key1"]
end

puts "Ruby parse #{file_path} Oj.load operation: #{time.real} sec"
