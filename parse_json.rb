require 'json'
require 'bundler/inline'

gemfile(true, quiet: true) do
  source "https://rubygems.org"

  gem 'rapidjson'
  gem 'oj'
end


file_path = ARGV[0]
start = Time.now
data = File.readlines(file_path).join
JSON.parse(data)["arr2"]
ending = Time.now
puts "Ruby parse #{file_path} JSON.parse operation: #{ending - start} sec"

require 'rapidjson'

start = Time.now
data = File.readlines(ARGV[0]).join
RapidJSON.parse(data)["arr"][0]
ending = Time.now
puts "Ruby parse #{file_path} RapidJSON.parse operation: #{ending - start} sec"

require 'oj'

start = Time.now
data = File.readlines(ARGV[0]).join
Oj.load(data)["arr"][0]
ending = Time.now
puts "Ruby parse #{file_path} Oj.load operation: #{ending - start} sec"
