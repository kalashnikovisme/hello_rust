require 'json'
require 'bundler/inline'

gemfile(true, quiet: true) do
  source "https://rubygems.org"

  gem 'rapidjson'
  gem 'oj'
end

data = File.readlines(ARGV[0]).join

start = Time.now
JSON.parse(data)["arr"][0]
ending = Time.now
puts "Ruby parse 1GB JSON.parse operation: #{ending - start} sec"

require 'rapidjson'

start = Time.now
RapidJSON.parse(data)["arr"][0]
ending = Time.now
puts "Ruby parse 1GB RapidJSON.parse operation: #{ending - start} sec"

require 'oj'

start = Time.now
Oj.load(data)["arr"][0]
ending = Time.now
puts "Ruby parse 1GB Oj.load operation: #{ending - start} sec"
