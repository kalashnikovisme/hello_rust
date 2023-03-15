run:
	bundle exec rake compile && bundle exec ruby -rhello_rust -e 'puts ParseJson.parse_json("test-1GB.json")'
