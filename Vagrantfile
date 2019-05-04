# vi: ft=ruby

Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"
  config.vm.synced_folder "./app", "/app"
  config.vm.provision :shell, path: "setup.sh"
end
