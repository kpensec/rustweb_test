# -*- mode: ruby -*-
# vi: ft=ruby

Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"
  config.vm.synced_folder "./app", "/app"
  config.vm.provision :shell, inline: 'pacman -Sy --noconfirm python2'
  config.vm.provision :ansible do |ansible|
    ansible.playbook = "./playbook.yml"
  end
  config.vm.network "forwarded_port", guest: 6379, host: 6379
end
