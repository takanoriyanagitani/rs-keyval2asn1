#!/bin/sh

printf 'wrld' |
	wazero \
		run \
		-env ENV_KEY=helo \
		./rs-keyval2asn1.wasm |
	fq \
		-d asn1_ber \
		'.constructed[].value'
