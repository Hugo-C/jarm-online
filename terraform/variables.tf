variable "gcp_project" {
  type    = string
  default = "jarm-online"
}
variable "gcp_zone" {
  type    = string
  default = "us-east1-d"
}

variable "gcp_region" {
  type    = string
  default = "us-east1"
}

variable "instance_type" {
  type    = string
  default = "e2-micro"
}
variable "instance_image" {
  type    = string
  default = "debian-cloud/debian-13"
}
variable "hostname" {
  type    = string
  default = "jarm.chas.tel"
}
