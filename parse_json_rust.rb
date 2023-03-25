require 'benchmark'

time = Benchmark.measure do
  result = UltimateJSON.parse_json(ARGV[0])
  puts "Nil is #{result['null']}"
  puts "Bool is #{result['bool']}"
  puts "Number is #{result['number']}"
  puts "String is #{result['string']}"
  puts "Array is #{result['array']}"
  puts "Object is #{result['object']}"
  puts "Object to hash #{result['object'].to_h}"
end

puts "Rust time #{time.real} sec"
