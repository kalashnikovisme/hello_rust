start_time = Time.now
puts ParseJson.parse_json("test.json")["arr"]
ending_time = Time.now
puts "Rust time #{ending_time - start_time} sec"
