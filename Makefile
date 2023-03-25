run:
	RUST_BACKTRACE=1 bundle exec rake compile
	RUST_BACKTRACE=1 bundle exec ruby -rhello_rust parse_json_rust.rb

benchmark:
	RUST_BACKTRACE=1 bundle exec rake compile
	RUST_BACKTRACE=1 bundle exec ruby -rhello_rust parse_json_rust.rb
	bundle exec ruby parse_json.rb test-300MB.json

another:
	# RB_SYS_CARGO_COMPILE=release RB_SYS_CARGO_PROFILE=release bundle exec rake compile && bundle exec ruby -rhello_rust parse_json_rust.rb
	# bundle exec ruby parse_json.rb test-300MB.json
	# RB_SYS_CARGO_COMPILE=release bundle exec rake compile && bundle exec ruby -rhello_rust -e 'puts ParseJson.parse_json("test-1GB.json")'
