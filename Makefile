format:
	cargo fmt --all -- --check

watch:
	cargo wwatch -q -c -w rest_api/src -x 'run -q'

# Avoid Over engineering by applying this
# Make it work, make it right, make it fast