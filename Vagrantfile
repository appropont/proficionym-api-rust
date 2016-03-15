# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|

  config.vm.box = "ubuntu/trusty64"
  config.vm.hostname = "api.proficionym.dev"
  
  config.vm.network "forwarded_port", guest: 3000, host: 80
  config.vm.network :private_network, :auto_network => true
  
  config.vm.synced_folder "./", "/home/vagrant/Code"
  
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y git curl
    curl -sSf https://static.rust-lang.org/rustup.sh | sh
  SHELL
end
