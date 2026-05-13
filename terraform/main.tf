// Configure the Google Cloud provider
provider "google" {
  project = var.gcp_project
  region  = var.gcp_region
  zone    = var.gcp_zone
}

resource "google_compute_instance" "gcp-free" {
  name         = "gcp-free"
  hostname     = var.hostname
  machine_type = var.instance_type
  project      = var.gcp_project
  zone         = var.gcp_zone

  scheduling {
    automatic_restart   = true
    on_host_maintenance = "MIGRATE"
    preemptible         = false
    provisioning_model  = "STANDARD"
  }

  boot_disk {
    initialize_params {
      image = var.instance_image
      size  = 30
      type  = "pd-standard"
    }
  }
  network_interface {
    network = "default"
    access_config {
      // Include this section to give the VM an external ip address
    }
  }
}

output "ip" {
  value = google_compute_instance.gcp-free.network_interface.0.network_ip
}