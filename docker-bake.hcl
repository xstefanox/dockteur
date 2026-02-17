variable "BASE_IMAGE_NAME" {
  default = ""
}

variable "DOCKER_HOST" {
  default = ""
}

target "default" {
  target = "default"
  network = "host"
  entitlements = ["network.host"]
  args = {
    DOCKER_HOST = "${DOCKER_HOST}"
  }
  tags = [
    "${BASE_IMAGE_NAME}-default",
  ]
  cache-from = [
    "type=gha",
  ]
  cache-to = [
    "type=gha,mode=max",
  ]
}

target "alpine" {
  target = "alpine"
  network = "host"
  entitlements = ["network.host"]
  args = {
    DOCKER_HOST = "${DOCKER_HOST}"
  }
  tags = [
    "${BASE_IMAGE_NAME}-alpine",
  ]
  cache-from = [
    "type=gha",
  ]
  cache-to = [
    "type=gha,mode=max",
  ]
}
