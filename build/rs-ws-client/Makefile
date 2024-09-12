####################################################################################
service=airenas/rs-ws-client
version?=dev
########### DOCKER ##################################################################
tag=$(service):$(version)

dbuild:
	cd ../.. && docker buildx build -t $(tag) --build-arg BUILD_VERSION=$(version) -f build/rs-ws-client/Dockerfile .

dpush: dbuild
	docker push $(tag)

dscan: dbuild
	docker scan --accept-license $(tag)	
#####################################################################################
.PHONY: dbuild dpush
