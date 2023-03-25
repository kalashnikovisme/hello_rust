start_time = Time.now
result = UltimateJSON.parse_json("test-100.json")
puts result['null']
puts result['bool']
puts result['number']
puts result['string']
puts result['array']
puts result['object']
puts result['object'].to_h
ending_time = Time.now
puts "Rust time #{ending_time - start_time} sec"
