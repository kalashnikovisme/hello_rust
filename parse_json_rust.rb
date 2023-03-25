start_time = Time.now
result = ParseJson.parse_json("test.json")
puts result['null']
puts result['bool']
puts result['number']
puts result['string']
puts result['array']
puts result['object']
ending_time = Time.now
puts "Rust time #{ending_time - start_time} sec"
